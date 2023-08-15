//! Features.

pub use opentype::layout::{Feature, Language, Script};

use std::collections::BTreeMap;
use std::io::Result;

use opentype::layout::Directory;
use typeface::Tape;

use crate::formats::opentype::cache::Cache;

/// Features.
pub type Features = BTreeMap<Script, BTreeMap<Option<Language>, BTreeMap<Feature, Vec<char>>>>;

pub(crate) fn read<T: Tape>(cache: &mut Cache<T>) -> Result<Features> {
    let mut values = Features::default();
    if let Some(table) = cache.try_glyph_positioning()? {
        populate(&mut values, table);
    }
    if let Some(table) = cache.try_glyph_substitution()? {
        populate(&mut values, table);
    }
    Ok(values)
}

fn populate<T>(values: &mut Features, table: &Directory<T>) {
    for (i, header) in table.scripts.headers.iter().enumerate() {
        let script = match Script::from_tag(&header.tag) {
            Some(value) => value,
            _ => continue,
        };
        if let Some(record) = table.scripts.records[i].default_language.as_ref() {
            for index in record.feature_indices.iter() {
                if let Some(header) = table.features.headers.get(*index as usize) {
                    if let Some(feature) = Feature::from_tag(&header.tag) {
                        let _ = values
                            .entry(script)
                            .or_default()
                            .entry(None)
                            .or_default()
                            .entry(feature)
                            .or_default();
                    }
                }
            }
        }
        for (j, header) in table.scripts.records[i].language_headers.iter().enumerate() {
            let language = match Language::from_tag(&header.tag) {
                Some(value) => value,
                _ => continue,
            };
            let record = &table.scripts.records[i].language_records[j];
            for index in record.feature_indices.iter() {
                if let Some(header) = table.features.headers.get(*index as usize) {
                    if let Some(feature) = Feature::from_tag(&header.tag) {
                        let _ = values
                            .entry(script)
                            .or_default()
                            .entry(Some(language))
                            .or_default()
                            .entry(feature)
                            .or_default();
                    }
                }
            }
        }
    }
}
