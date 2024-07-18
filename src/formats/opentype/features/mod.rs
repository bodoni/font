//! Layout features.

mod rules;
mod sample;
mod transform;

pub use opentype::layout::{Feature, Language, Script};

pub use sample::{Component, Sample};

use std::collections::{BTreeSet, HashMap};
use std::io::Result;

use opentype::layout;

use crate::formats::opentype::cache::Cache;
use crate::formats::opentype::features::rules::{Rules, Table};
use crate::formats::opentype::features::transform::Transform;
use crate::formats::opentype::mapping::Reverse as Mapping;

/// A directory.
#[derive(Clone, Debug, Default)]
pub struct Directory {
    /// Scripts to languages.
    pub scripts: Vec<(Script, Vec<usize>)>,
    /// Languages to features.
    pub languages: Vec<(Language, Vec<usize>)>,
    /// Features to samples.
    pub features: Vec<(Feature, Vec<usize>)>,
    /// Samples.
    pub samples: Vec<Vec<Option<BTreeSet<Sample>>>>,
}

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Directory> {
    let mapping = cache.reverse_mapping()?.clone();

    let mut scripts = HashMap::default();
    let mut languages = (Vec::default(), HashMap::default());
    let mut features = (Vec::default(), HashMap::default());
    let mut samples = (Vec::default(), HashMap::default());

    if let Some(table) = cache.try_glyph_positioning()? {
        let _ = process_table(
            &table.borrow(),
            &mapping,
            &mut scripts,
            &mut languages,
            &mut features,
            &mut samples,
        );
    }

    if let Some(table) = cache.try_glyph_substitution()? {
        let _ = process_table(
            &table.borrow(),
            &mapping,
            &mut scripts,
            &mut languages,
            &mut features,
            &mut samples,
        );
    }

    let mut scripts = scripts.into_iter().collect::<Vec<_>>();
    let mut languages = languages.0;
    let mut features = features.0;
    let mut samples = samples.0;

    sort(&mut samples, &mut features);
    sort(&mut features, &mut languages);
    sort(&mut languages, &mut scripts);

    scripts.sort();
    for (_, indices) in scripts.iter_mut() {
        indices.sort();
    }

    Ok(Directory {
        scripts,
        languages,
        features,
        samples,
    })
}

#[allow(clippy::type_complexity)]
fn process_table<T>(
    directory: &layout::Directory<T>,
    mapping: &Mapping,
    scripts: &mut HashMap<Script, Vec<usize>>,
    languages: &mut (
        Vec<(Language, Vec<usize>)>,
        HashMap<(Language, Vec<usize>), usize>,
    ),
    features: &mut (
        Vec<(Feature, Vec<usize>)>,
        HashMap<(Feature, Vec<usize>), usize>,
    ),
    samples: &mut (
        Vec<Vec<Option<BTreeSet<Sample>>>>,
        HashMap<Vec<Option<BTreeSet<Sample>>>, usize>,
    ),
) -> Option<()>
where
    T: Table,
{
    let graphs = process_graphs(directory, mapping, samples);
    for (i, header) in directory.scripts.headers.iter().enumerate() {
        scripts
            .entry(Script::from_tag(&header.tag))
            .or_default()
            .extend(process_script(
                directory,
                &directory.scripts.records[i],
                languages,
                features,
                &graphs,
            )?);
    }
    Some(())
}

#[allow(clippy::type_complexity)]
fn process_graphs<T>(
    directory: &layout::Directory<T>,
    mapping: &Mapping,
    samples: &mut (
        Vec<Vec<Option<BTreeSet<Sample>>>>,
        HashMap<Vec<Option<BTreeSet<Sample>>>, usize>,
    ),
) -> Vec<usize>
where
    T: Table,
{
    let graphs: Vec<Vec<Option<Rules>>> = directory
        .lookups
        .records
        .iter()
        .map(|record| {
            record
                .tables
                .iter()
                .map(|table| table.extract(directory))
                .collect()
        })
        .collect();
    graphs
        .iter()
        .map(|values| append(samples, values.transform(mapping, &graphs)))
        .collect()
}

#[allow(clippy::type_complexity)]
fn process_script<T>(
    directory: &layout::Directory<T>,
    record: &layout::script::Record,
    languages: &mut (
        Vec<(Language, Vec<usize>)>,
        HashMap<(Language, Vec<usize>), usize>,
    ),
    features: &mut (
        Vec<(Feature, Vec<usize>)>,
        HashMap<(Feature, Vec<usize>), usize>,
    ),
    graphs: &[usize],
) -> Option<Vec<usize>> {
    let mut indices = Vec::with_capacity(
        if record.default_language.is_some() {
            1
        } else {
            0
        } + record.language_headers.len(),
    );
    if let Some(record) = record.default_language.as_ref() {
        indices.push(append(
            languages,
            (
                Language::Default,
                process_language(directory, record, features, graphs)?,
            ),
        ));
    }
    for (j, header) in record.language_headers.iter().enumerate() {
        indices.push(append(
            languages,
            (
                Language::from_tag(&header.tag),
                process_language(directory, &record.language_records[j], features, graphs)?,
            ),
        ))
    }
    Some(indices)
}

#[allow(clippy::type_complexity)]
fn process_language<T>(
    directory: &layout::Directory<T>,
    record: &layout::language::Record,
    features: &mut (
        Vec<(Feature, Vec<usize>)>,
        HashMap<(Feature, Vec<usize>), usize>,
    ),
    graphs: &[usize],
) -> Option<Vec<usize>> {
    record
        .feature_indices
        .iter()
        .cloned()
        .map(usize::from)
        .filter_map(|index| {
            if let (Some(header), Some(record)) = (
                directory.features.headers.get(index),
                directory.features.records.get(index),
            ) {
                Some((header, record))
            } else {
                None
            }
        })
        .map(|(header, record)| {
            append(
                features,
                (
                    Feature::from_tag(&header.tag),
                    process_feature(record, graphs)?,
                ),
            )
            .into()
        })
        .collect()
}

fn process_feature(record: &layout::feature::Record, graphs: &[usize]) -> Option<Vec<usize>> {
    record
        .lookup_indices
        .iter()
        .cloned()
        .map(usize::from)
        .map(|index| graphs.get(index).cloned())
        .collect()
}

fn append<T>(values: &mut (Vec<T>, HashMap<T, usize>), value: T) -> usize
where
    T: Clone + std::hash::Hash + std::cmp::Eq,
{
    if let Some(index) = values.1.get(&value).cloned() {
        index
    } else {
        let index = values.0.len();
        values.0.push(value.clone());
        values.1.insert(value, index);
        index
    }
}

fn sort<T, U>(values: &mut [T], other: &mut [(U, Vec<usize>)])
where
    T: std::cmp::Ord,
{
    let mapping = {
        let mut indices = (0..values.len()).collect::<Vec<_>>();
        indices.sort_by(|one, other| values[*one].cmp(&values[*other]));
        indices
            .into_iter()
            .enumerate()
            .map(|(new, old)| (old, new))
            .collect::<HashMap<_, _>>()
    };
    values.sort();
    for other in other.iter_mut() {
        for other in other.1.iter_mut() {
            *other = mapping[other];
        }
    }
}
