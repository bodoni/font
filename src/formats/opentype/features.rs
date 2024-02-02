//! Layout features.

pub use opentype::layout::{Language, Script};

use std::collections::{BTreeMap, BTreeSet};
use std::io::Result;

use opentype::layout::{Directory, Feature};

use crate::formats::opentype::cache::Cache;

/// Layout features.
pub type Features = BTreeMap<Type, Value>;

/// A type.
pub type Type = Feature;

/// A value.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Value {
    /// The scripts and languages.
    pub scripts: BTreeMap<Script, BTreeSet<Option<Language>>>,
}

pub(crate) fn read<T: typeface::tape::Read>(cache: &mut Cache<T>) -> Result<Features> {
    let mut values = Features::default();
    if let Some(table) = cache.try_glyph_positioning()? {
        populate(&mut values, &table.borrow());
    }
    if let Some(table) = cache.try_glyph_substitution()? {
        populate(&mut values, &table.borrow());
    }
    Ok(values)
}

fn populate<T>(values: &mut Features, table: &Directory<T>) {
    for (i, header) in table.scripts.headers.iter().enumerate() {
        let script = Script::from_tag(&header.tag);
        if let Some(record) = table.scripts.records[i].default_language.as_ref() {
            for index in record.feature_indices.iter() {
                if let Some(header) = table.features.headers.get(*index as usize) {
                    let feature = Feature::from_tag(&header.tag);
                    let value = values.entry(feature).or_default();
                    value.scripts.entry(script).or_default().insert(None);
                }
            }
        }
        for (j, header) in table.scripts.records[i].language_headers.iter().enumerate() {
            let language = Language::from_tag(&header.tag);
            let record = &table.scripts.records[i].language_records[j];
            for index in record.feature_indices.iter() {
                if let Some(header) = table.features.headers.get(*index as usize) {
                    let feature = Feature::from_tag(&header.tag);
                    let value = values.entry(feature).or_default();
                    value
                        .scripts
                        .entry(script)
                        .or_default()
                        .insert(Some(language));
                }
            }
        }
    }
}
