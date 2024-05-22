//! Layout features.

mod characters;
mod glyphs;

pub use opentype::layout::{Language, Script};

use std::collections::{BTreeMap, BTreeSet};
use std::io::Result;

use opentype::layout::{Directory, Feature};

use crate::formats::opentype::cache::Cache;
use crate::formats::opentype::characters::Character;
use crate::formats::opentype::features::characters::Characters;
use crate::formats::opentype::features::glyphs::{Glyph, Glyphs};

/// Layout features.
pub type Features = BTreeMap<Type, Value>;

/// A type.
pub type Type = Feature;

/// A value.
pub type Value = BTreeMap<Script, BTreeMap<Language, BTreeSet<Vec<Character>>>>;

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Features> {
    let mut values = Default::default();
    if let Some(table) = cache.try_glyph_positioning()? {
        populate(&mut values, &table.borrow());
    }
    if let Some(table) = cache.try_glyph_substitution()? {
        populate(&mut values, &table.borrow());
    }
    let mapping = cache.reverse_mapping()?.clone();
    Ok(values.characters(&mapping, ()))
}

#[allow(clippy::type_complexity)]
fn populate<T>(
    values: &mut BTreeMap<Feature, BTreeMap<Script, BTreeMap<Language, BTreeSet<Vec<Glyph>>>>>,
    table: &Directory<T>,
) where
    T: Glyphs,
{
    for (i, header) in table.scripts.headers.iter().enumerate() {
        let script = Script::from_tag(&header.tag);
        if let Some(record) = table.scripts.records[i].default_language.as_ref() {
            for index in record.feature_indices.iter().cloned().map(usize::from) {
                if let (Some(header), Some(record)) = (
                    table.features.headers.get(index),
                    table.features.records.get(index),
                ) {
                    let feature = Feature::from_tag(&header.tag);
                    let glyphs = record
                        .lookup_indices
                        .iter()
                        .cloned()
                        .filter_map(|index| table.lookups.records.get(index as usize))
                        .flat_map(|record| record.tables.iter().flat_map(|table| table.glyphs()))
                        .collect::<BTreeSet<_>>();
                    values
                        .entry(feature)
                        .or_default()
                        .entry(script)
                        .or_default()
                        .insert(Language::Default, glyphs);
                }
            }
        }
        for (j, header) in table.scripts.records[i].language_headers.iter().enumerate() {
            let language = Language::from_tag(&header.tag);
            let record = &table.scripts.records[i].language_records[j];
            for index in record.feature_indices.iter().cloned().map(usize::from) {
                if let (Some(header), Some(record)) = (
                    table.features.headers.get(index),
                    table.features.records.get(index),
                ) {
                    let feature = Feature::from_tag(&header.tag);
                    let glyphs = record
                        .lookup_indices
                        .iter()
                        .cloned()
                        .filter_map(|index| table.lookups.records.get(index as usize))
                        .flat_map(|record| record.tables.iter().flat_map(|table| table.glyphs()))
                        .collect::<BTreeSet<_>>();
                    values
                        .entry(feature)
                        .or_default()
                        .entry(script)
                        .or_default()
                        .insert(language, glyphs);
                }
            }
        }
    }
}
