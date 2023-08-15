//! Features.

pub use opentype::layout::{Feature, Language, Script};

use std::io::Result;

use opentype::layout::Directory;
use typeface::Tape;

use crate::formats::opentype::cache::Cache;

/// Features.
pub type Features = Vec<(Script, Vec<(Option<Language>, Vec<(Feature, Vec<char>)>)>)>;

pub(crate) fn read<T: Tape>(cache: &mut Cache<T>) -> Result<Features> {
    let mut scripts = vec![];
    if let Some(table) = cache.try_glyph_positioning()? {
        scripts.extend(parse(table));
    }
    if let Some(table) = cache.try_glyph_substitution()? {
        scripts.extend(parse(table));
    }
    Ok(scripts)
}

fn parse<T>(table: &Directory<T>) -> Features {
    let mut scripts = Vec::with_capacity(table.scripts.count as usize);
    for (i, header) in table.scripts.headers.iter().enumerate() {
        let mut languages =
            Vec::with_capacity(table.scripts.records[i].language_count as usize + 1);
        let script = match Script::from_tag(&header.tag) {
            Some(value) => value,
            _ => continue,
        };
        if let Some(record) = table.scripts.records[i].default_language.as_ref() {
            let mut features = Vec::with_capacity(record.feature_index_count as usize);
            for index in record.feature_indices.iter() {
                if let Some(header) = table.features.headers.get(*index as usize) {
                    if let Some(feature) = Feature::from_tag(&header.tag) {
                        features.push((feature, Default::default()));
                    }
                }
            }
            languages.push((None, features));
        }
        for (j, header) in table.scripts.records[i].language_headers.iter().enumerate() {
            let language = match Language::from_tag(&header.tag) {
                Some(value) => value,
                _ => continue,
            };
            let record = &table.scripts.records[i].language_records[j];
            let mut features = Vec::with_capacity(record.feature_index_count as usize);
            for index in record.feature_indices.iter() {
                if let Some(header) = table.features.headers.get(*index as usize) {
                    if let Some(feature) = Feature::from_tag(&header.tag) {
                        features.push((feature, Default::default()));
                    }
                }
            }
            languages.push((Some(language), features));
        }
        scripts.push((script, languages));
    }
    scripts
}
