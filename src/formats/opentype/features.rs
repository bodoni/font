//! Layout features.

pub use opentype::layout::{Coverage, Language, Script};

use std::collections::BTreeMap;
use std::io::Result;

use opentype::layout::{Directory, Feature};

use crate::formats::opentype::cache::Cache;
use crate::Character;

/// Layout features.
pub type Features = BTreeMap<Type, Value>;

/// A type.
pub type Type = Feature;

/// A value.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Value {
    /// The scripts and languages.
    pub scripts: BTreeMap<Script, BTreeMap<Option<Language>, Vec<Vec<Character>>>>,
}

trait Characters {
    #[inline]
    fn characters(&self) -> Vec<Vec<Character>> {
        Default::default()
    }
}

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Features> {
    let mut values = Features::default();
    if let Some(table) = cache.try_glyph_positioning()? {
        populate(&mut values, &table.borrow());
    }
    if let Some(table) = cache.try_glyph_substitution()? {
        populate(&mut values, &table.borrow());
    }
    Ok(values)
}

fn populate<T>(values: &mut Features, table: &Directory<T>)
where
    T: Characters,
{
    for (i, header) in table.scripts.headers.iter().enumerate() {
        let script = Script::from_tag(&header.tag);
        if let Some(record) = table.scripts.records[i].default_language.as_ref() {
            for index in record.feature_indices.iter() {
                if let (Some(header), Some(record)) = (
                    table.features.headers.get(*index as usize),
                    table.features.records.get(*index as usize),
                ) {
                    let feature = Feature::from_tag(&header.tag);
                    let mut characters = record
                        .lookup_indices
                        .iter()
                        .filter_map(|index| table.lookups.records.get(*index as usize))
                        .flat_map(|record| record.tables.iter().flat_map(Characters::characters))
                        .collect::<Vec<_>>();
                    characters.sort();
                    values
                        .entry(feature)
                        .or_default()
                        .scripts
                        .entry(script)
                        .or_default()
                        .insert(None, characters);
                }
            }
        }
        for (j, header) in table.scripts.records[i].language_headers.iter().enumerate() {
            let language = Language::from_tag(&header.tag);
            let record = &table.scripts.records[i].language_records[j];
            for index in record.feature_indices.iter() {
                if let (Some(header), Some(record)) = (
                    table.features.headers.get(*index as usize),
                    table.features.records.get(*index as usize),
                ) {
                    let feature = Feature::from_tag(&header.tag);
                    let mut characters = record
                        .lookup_indices
                        .iter()
                        .filter_map(|index| table.lookups.records.get(*index as usize))
                        .flat_map(|record| record.tables.iter().flat_map(Characters::characters))
                        .collect::<Vec<_>>();
                    characters.sort();
                    values
                        .entry(feature)
                        .or_default()
                        .scripts
                        .entry(script)
                        .or_default()
                        .insert(Some(language), characters);
                }
            }
        }
    }
}

impl Characters for opentype::tables::glyph_positioning::Type {}

impl Characters for opentype::tables::glyph_substitution::Type {
    fn characters(&self) -> Vec<Vec<Character>> {
        use opentype::tables::glyph_substitution::SingleSubstitution;
        use opentype::tables::glyph_substitution::Type;

        let mut values = Vec::default();
        match self {
            Type::SingleSubstitution(SingleSubstitution::Format1(value)) => {
                values.extend(expand(&value.coverage));
            }
            Type::SingleSubstitution(SingleSubstitution::Format2(value)) => {
                values.extend(expand(&value.coverage));
            }
            _ => {}
        }
        values
    }
}

fn expand(_: &Coverage) -> impl IntoIterator<Item = Vec<Character>> {
    vec![]
}
