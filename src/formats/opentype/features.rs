//! Layout features.

pub use opentype::layout::{Class, Coverage, Language, Script};
pub use opentype::truetype::GlyphID;

use std::collections::{BTreeMap, BTreeSet};
use std::io::Result;

use opentype::layout::{Directory, Feature};

use crate::formats::opentype::cache::Cache;
use crate::formats::opentype::characters::{Character, ReverseMapping};

/// Layout features.
pub type Features = BTreeMap<Type, Value>;

/// A type.
pub type Type = Feature;

/// A value.
pub type Value = BTreeMap<Script, BTreeMap<Language, BTreeSet<Vec<Character>>>>;

trait Characters {
    #[inline]
    fn characters(&self, _: &ReverseMapping) -> BTreeSet<Vec<Character>> {
        Default::default()
    }
}

trait Compress {
    fn compress(self, _: &ReverseMapping) -> Option<Character>;
}

impl Characters for opentype::tables::glyph_positioning::Type {}

impl Characters for opentype::tables::glyph_substitution::Type {
    fn characters(&self, mapping: &ReverseMapping) -> BTreeSet<Vec<Character>> {
        use opentype::layout::{ChainedContext, Context};
        use opentype::tables::glyph_substitution::{SingleSubstitution, Type};

        let map = |glyph_id| mapping.get(glyph_id);
        let mut values = BTreeSet::default();
        match self {
            Type::SingleSubstitution(SingleSubstitution::Format1(value)) => {
                values.extend(
                    uncover(&value.coverage)
                        .filter_map(map)
                        .map(Character::Scalar)
                        .map(vector),
                );
            }
            Type::SingleSubstitution(SingleSubstitution::Format2(value)) => {
                values.extend(
                    uncover(&value.coverage)
                        .filter_map(map)
                        .map(Character::Scalar)
                        .map(vector),
                );
            }
            Type::MultipleSubstitution(value) => {
                values.extend(
                    uncover(&value.coverage)
                        .filter_map(map)
                        .map(Character::Scalar)
                        .map(vector),
                );
            }
            Type::AlternateSubstitution(value) => {
                values.extend(
                    uncover(&value.coverage)
                        .filter_map(map)
                        .map(Character::Scalar)
                        .map(vector),
                );
            }
            Type::LigatureSubstitution(value) => {
                values.extend(uncover(&value.coverage).zip(&value.records).flat_map(
                    |(glyph_id, record)| {
                        record.records.iter().filter_map(move |record| {
                            let mut value = Vec::with_capacity(record.glyph_count as usize);
                            value.push(Character::Scalar(mapping.get(glyph_id)?));
                            for glyph_id in &record.glyph_ids {
                                value.push(Character::Scalar(mapping.get(*glyph_id)?));
                            }
                            Some(value)
                        })
                    },
                ));
            }
            Type::ContextualSubstitution(Context::Format1(value)) => {
                values.extend(uncover(&value.coverage).zip(&value.records).flat_map(
                    |(glyph_id, record)| {
                        record.records.iter().filter_map(move |record| {
                            let mut value = Vec::with_capacity(record.glyph_count as usize);
                            value.push(Character::Scalar(mapping.get(glyph_id)?));
                            for glyph_id in &record.glyph_ids {
                                value.push(Character::Scalar(mapping.get(*glyph_id)?));
                            }
                            Some(value)
                        })
                    },
                ));
            }
            Type::ContextualSubstitution(Context::Format2(value)) => {
                let (classes, mapping) = unclass(&value.class, mapping);
                values.extend(
                    uncover(&value.coverage)
                        .filter_map(|glyph_id| mapping.get(&glyph_id))
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .filter_map(|index| {
                            value
                                .records
                                .get(*index as usize)
                                .and_then(|record| record.as_ref().map(|record| (index, record)))
                        })
                        .flat_map(|(index, record)| {
                            record.records.iter().filter_map(|record| {
                                let mut value = Vec::with_capacity(record.glyph_count as usize);
                                value.push(classes.get(index)?.clone());
                                for index in &record.indices {
                                    value.push(classes.get(index)?.clone());
                                }
                                Some(value)
                            })
                        }),
                );
            }
            Type::ContextualSubstitution(Context::Format3(value)) => {
                if let Some(value) = value
                    .coverages
                    .iter()
                    .map(|coverage| coverage.compress(mapping))
                    .collect::<Option<Vec<_>>>()
                {
                    values.insert(value);
                }
            }
            Type::ChainedContextualSubstitution(ChainedContext::Format1(value)) => {
                values.extend(uncover(&value.coverage).zip(&value.records).flat_map(
                    |(glyph_id, record)| {
                        record.records.iter().filter_map(move |record| {
                            let mut value = Vec::with_capacity(
                                record.backward_glyph_count as usize
                                    + record.glyph_count as usize
                                    + record.forward_glyph_count as usize,
                            );
                            for glyph_id in record.backward_glyph_ids.iter().rev() {
                                value.push(Character::Scalar(mapping.get(*glyph_id)?));
                            }
                            value.push(Character::Scalar(mapping.get(glyph_id)?));
                            for glyph_id in &record.glyph_ids {
                                value.push(Character::Scalar(mapping.get(*glyph_id)?));
                            }
                            for glyph_id in &record.forward_glyph_ids {
                                value.push(Character::Scalar(mapping.get(*glyph_id)?));
                            }
                            Some(value)
                        })
                    },
                ));
            }
            Type::ChainedContextualSubstitution(ChainedContext::Format2(value)) => {
                let (backward_classes, _) = unclass(&value.backward_class, mapping);
                let (forward_classes, _) = unclass(&value.forward_class, mapping);
                let (classes, mapping) = unclass(&value.class, mapping);
                values.extend(
                    uncover(&value.coverage)
                        .filter_map(|glyph_id| mapping.get(&glyph_id))
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .filter_map(|index| {
                            value
                                .records
                                .get(*index as usize)
                                .and_then(|record| record.as_ref().map(|record| (index, record)))
                        })
                        .flat_map(|(index, record)| {
                            record.records.iter().filter_map(|record| {
                                let mut value = Vec::with_capacity(
                                    record.backward_glyph_count as usize
                                        + record.glyph_count as usize
                                        + record.forward_glyph_count as usize,
                                );
                                for index in record.backward_indices.iter().rev() {
                                    value.push(backward_classes.get(index)?.clone());
                                }
                                value.push(classes.get(index)?.clone());
                                for index in &record.indices {
                                    value.push(classes.get(index)?.clone());
                                }
                                for index in &record.forward_indices {
                                    value.push(forward_classes.get(index)?.clone());
                                }
                                Some(value)
                            })
                        }),
                );
            }
            Type::ChainedContextualSubstitution(ChainedContext::Format3(value)) => {
                if let (Some(mut backward_value), Some(value), Some(forward_value)) = (
                    value
                        .backward_coverages
                        .iter()
                        .rev()
                        .map(|coverage| coverage.compress(mapping))
                        .collect::<Option<Vec<_>>>(),
                    value
                        .coverages
                        .iter()
                        .map(|coverage| coverage.compress(mapping))
                        .collect::<Option<Vec<_>>>(),
                    value
                        .forward_coverages
                        .iter()
                        .map(|coverage| coverage.compress(mapping))
                        .collect::<Option<Vec<_>>>(),
                ) {
                    backward_value.extend(value);
                    backward_value.extend(forward_value);
                    values.insert(backward_value);
                }
            }
            Type::ReverseChainedContextualSubstibution(_) => {}
            _ => {}
        }
        values
    }
}

impl Compress for BTreeSet<GlyphID> {
    fn compress(self, mapping: &ReverseMapping) -> Option<Character> {
        self.into_iter()
            .filter_map(|glyph_id| mapping.get(glyph_id))
            .collect::<Vec<_>>()
            .compress(mapping)
    }
}

impl Compress for &Coverage {
    fn compress(self, mapping: &ReverseMapping) -> Option<Character> {
        match self {
            Coverage::Format1(value) => value.glyph_ids.compress(mapping),
            Coverage::Format2(value) => value
                .records
                .iter()
                .filter_map(|record| {
                    Some((
                        mapping.get(record.start_glyph_id)?,
                        mapping.get(record.end_glyph_id)?,
                    ))
                })
                .collect::<Vec<_>>()
                .compress(mapping),
        }
    }
}

impl Compress for &[GlyphID] {
    fn compress(self, mapping: &ReverseMapping) -> Option<Character> {
        match self.len() {
            0 => None,
            1 => Some(Character::Scalar(mapping.get(self[0])?)),
            _ => self
                .iter()
                .filter_map(|glyph_id| mapping.get(*glyph_id))
                .collect::<Vec<_>>()
                .compress(mapping),
        }
    }
}

impl Compress for Vec<char> {
    fn compress(mut self, _: &ReverseMapping) -> Option<Character> {
        self.sort();
        match self.len() {
            0 => None,
            1 => Some(Character::Scalar(self[0])),
            _ => Some(Character::List(self)),
        }
    }
}

impl Compress for Vec<(char, char)> {
    fn compress(mut self, mapping: &ReverseMapping) -> Option<Character> {
        self.sort();
        match self.len() {
            0 => None,
            1 => self[0].compress(mapping),
            _ => Some(Character::Ranges(self)),
        }
    }
}

impl Compress for (char, char) {
    fn compress(self, _: &ReverseMapping) -> Option<Character> {
        if self.0 == self.1 {
            Some(Character::Scalar(self.0))
        } else {
            Some(Character::Range(self.0, self.1))
        }
    }
}

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Features> {
    let mut values = Features::default();
    let mapping = cache.reverse_mapping()?.clone();
    if let Some(table) = cache.try_glyph_positioning()? {
        populate(&mut values, &table.borrow(), &mapping);
    }
    if let Some(table) = cache.try_glyph_substitution()? {
        populate(&mut values, &table.borrow(), &mapping);
    }
    Ok(values)
}

fn populate<T>(values: &mut Features, table: &Directory<T>, mapping: &ReverseMapping)
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
                    let characters = record
                        .lookup_indices
                        .iter()
                        .filter_map(|index| table.lookups.records.get(*index as usize))
                        .flat_map(|record| {
                            record
                                .tables
                                .iter()
                                .flat_map(|table| table.characters(mapping))
                        })
                        .collect::<BTreeSet<_>>();
                    values
                        .entry(feature)
                        .or_default()
                        .entry(script)
                        .or_default()
                        .insert(Language::Default, characters);
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
                    let characters = record
                        .lookup_indices
                        .iter()
                        .filter_map(|index| table.lookups.records.get(*index as usize))
                        .flat_map(|record| {
                            record
                                .tables
                                .iter()
                                .flat_map(|table| table.characters(mapping))
                        })
                        .collect::<BTreeSet<_>>();
                    values
                        .entry(feature)
                        .or_default()
                        .entry(script)
                        .or_default()
                        .insert(language, characters);
                }
            }
        }
    }
}

#[inline]
fn vector<T>(value: T) -> Vec<T> {
    vec![value]
}

fn unclass(
    value: &Class,
    mapping: &ReverseMapping,
) -> (BTreeMap<u16, Character>, BTreeMap<GlyphID, u16>) {
    let mut forward = BTreeMap::<_, BTreeSet<_>>::default();
    let mut reverse = BTreeMap::default();
    match value {
        Class::Format1(value) => {
            let range = value.start_glyph_id..value.start_glyph_id + value.glyph_count;
            for (glyph_id, index) in range.zip(value.indices.iter().cloned()) {
                forward.entry(index).or_default().insert(glyph_id);
                reverse.insert(glyph_id, index);
            }
        }
        Class::Format2(value) => {
            for record in &value.records {
                for glyph_id in record.start_glyph_id..=record.end_glyph_id {
                    forward.entry(record.index).or_default().insert(glyph_id);
                    reverse.insert(glyph_id, record.index);
                }
            }
        }
    }
    (
        forward
            .into_iter()
            .filter_map(|(index, glyph_ids)| {
                glyph_ids
                    .compress(mapping)
                    .map(|characters| (index, characters))
            })
            .collect(),
        reverse,
    )
}

fn uncover(value: &Coverage) -> Box<dyn Iterator<Item = GlyphID> + '_> {
    match value {
        Coverage::Format1(value) => Box::new(value.glyph_ids.iter().cloned()),
        Coverage::Format2(value) => Box::new(
            value
                .records
                .iter()
                .flat_map(|record| record.start_glyph_id..=record.end_glyph_id),
        ),
    }
}
