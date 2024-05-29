//! Layout features.

mod graph;
mod sample;
mod transform;

pub use opentype::layout::{Language, Script};

pub use sample::{Component, Sample};

use std::collections::{BTreeMap, BTreeSet};
use std::io::Result;

use opentype::layout::{Directory, Feature};

use crate::formats::opentype::cache::Cache;
use crate::formats::opentype::features::graph::{Graph, Table};
use crate::formats::opentype::features::transform::Transform;

/// Layout features.
pub type Features = (BTreeMap<Type, Value>, Vec<Option<Vec<BTreeSet<Sample>>>>);

/// A type.
pub type Type = Feature;

/// A value.
pub type Value = BTreeMap<Script, BTreeMap<Language, Vec<usize>>>;

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Features> {
    let mapping = cache.reverse_mapping()?.clone();
    let mut values = Default::default();
    let mut samples: Vec<_> = Default::default();
    if let Some(table) = cache.try_glyph_positioning()? {
        let directory = table.borrow();
        map(&mut values, &directory, samples.len());
        let graphs = list(&directory);
        samples.extend(
            graphs
                .iter()
                .map(|value| value.transform(&mapping, &graphs)),
        );
    }
    if let Some(table) = cache.try_glyph_substitution()? {
        let directory = table.borrow();
        map(&mut values, &directory, samples.len());
        let graphs = list(&directory);
        samples.extend(
            graphs
                .iter()
                .map(|value| value.transform(&mapping, &graphs)),
        );
    }
    Ok((values, samples))
}

fn list<T>(directory: &Directory<T>) -> Vec<Vec<Graph>>
where
    T: Table,
{
    directory
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
        .collect()
}

fn map<T>(
    values: &mut BTreeMap<Feature, BTreeMap<Script, BTreeMap<Language, Vec<usize>>>>,
    directory: &Directory<T>,
    offset: usize,
) where
    T: Table,
{
    for (i, header) in directory.scripts.headers.iter().enumerate() {
        let script = Script::from_tag(&header.tag);
        if let Some(record) = directory.scripts.records[i].default_language.as_ref() {
            for index in record.feature_indices.iter().cloned().map(usize::from) {
                if let (Some(header), Some(record)) = (
                    directory.features.headers.get(index),
                    directory.features.records.get(index),
                ) {
                    let feature = Feature::from_tag(&header.tag);
                    let indices = record
                        .lookup_indices
                        .iter()
                        .cloned()
                        .map(usize::from)
                        .map(|value| offset + value)
                        .collect();
                    values
                        .entry(feature)
                        .or_default()
                        .entry(script)
                        .or_default()
                        .insert(Language::Default, indices);
                }
            }
        }
        for (j, header) in directory.scripts.records[i]
            .language_headers
            .iter()
            .enumerate()
        {
            let language = Language::from_tag(&header.tag);
            let record = &directory.scripts.records[i].language_records[j];
            for index in record.feature_indices.iter().cloned().map(usize::from) {
                if let (Some(header), Some(record)) = (
                    directory.features.headers.get(index),
                    directory.features.records.get(index),
                ) {
                    let feature = Feature::from_tag(&header.tag);
                    let indices = record
                        .lookup_indices
                        .iter()
                        .cloned()
                        .map(usize::from)
                        .map(|value| offset + value)
                        .collect();
                    values
                        .entry(feature)
                        .or_default()
                        .entry(script)
                        .or_default()
                        .insert(language, indices);
                }
            }
        }
    }
}
