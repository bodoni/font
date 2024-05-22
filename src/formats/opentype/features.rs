//! Layout features.

pub use opentype::layout::{Class, Coverage, Language, Script};
pub use opentype::truetype::GlyphID;

use std::collections::{BTreeMap, BTreeSet};
use std::io::Result;

use opentype::layout::{Directory, Feature};

use crate::formats::opentype::cache::Cache;
use crate::formats::opentype::characters::{Character, ReverseMapping as Mapping};

/// Layout features.
pub type Features = BTreeMap<Type, Value>;

/// A type.
pub type Type = Feature;

/// A value.
pub type Value = BTreeMap<Script, BTreeMap<Language, BTreeSet<Vec<Character>>>>;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
enum Glyph {
    Scalar(GlyphID),
    Range(GlyphID, GlyphID),
    Ranges(Vec<(GlyphID, GlyphID)>),
    List(Vec<GlyphID>),
}

type Glyphs = BTreeSet<Vec<Glyph>>;

trait ToCharacters<'l> {
    type Target;
    type Parameter: 'l;

    fn to_characters(self, _: &Mapping, _: Self::Parameter) -> Self::Target;
}

trait ToGlyphs {
    #[inline]
    fn to_glyphs(&self) -> Glyphs {
        Default::default()
    }
}

impl<'l> ToCharacters<'l> for &BTreeMap<Feature, BTreeMap<Script, BTreeMap<Language, Glyphs>>> {
    type Target = Features;
    type Parameter = ();

    fn to_characters(self, mapping: &Mapping, _: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|(key, value)| (*key, value.to_characters(mapping, ())))
            .collect()
    }
}

impl<'l> ToCharacters<'l> for &BTreeMap<Script, BTreeMap<Language, Glyphs>> {
    type Target = Value;
    type Parameter = ();

    fn to_characters(self, mapping: &Mapping, _: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|(key, value)| (*key, value.to_characters(mapping, ())))
            .collect()
    }
}

impl<'l> ToCharacters<'l> for &BTreeMap<Language, Glyphs> {
    type Target = BTreeMap<Language, BTreeSet<Vec<Character>>>;
    type Parameter = ();

    fn to_characters(self, mapping: &Mapping, _: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|(key, value)| (*key, value.to_characters(mapping, ())))
            .collect()
    }
}

impl<'l> ToCharacters<'l> for &Glyphs {
    type Target = BTreeSet<Vec<Character>>;
    type Parameter = ();

    fn to_characters(self, mapping: &Mapping, _: Self::Parameter) -> Self::Target {
        self.iter()
            .filter_map(|value| value.to_characters(mapping, self))
            .collect()
    }
}

impl<'l> ToCharacters<'l> for &[Glyph] {
    type Target = Option<Vec<Character>>;
    type Parameter = &'l Glyphs;

    fn to_characters(self, mapping: &Mapping, glyphs: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|value| value.to_characters(mapping, glyphs))
            .collect()
    }
}

impl<'l> ToCharacters<'l> for &Glyph {
    type Target = Option<Character>;
    type Parameter = &'l Glyphs;

    fn to_characters(self, mapping: &Mapping, glyphs: Self::Parameter) -> Self::Target {
        match self {
            Glyph::Scalar(value) => mapping.get(*value).map(Character::Scalar),
            Glyph::Range(start, end) => to_characters(*start..=*end, mapping, glyphs),
            Glyph::Ranges(value) => to_characters(
                value.iter().flat_map(|value| value.0..=value.1),
                mapping,
                glyphs,
            ),
            Glyph::List(value) => to_characters(value.iter().cloned(), mapping, glyphs),
        }
    }
}

impl ToGlyphs for opentype::tables::glyph_positioning::Type {}

impl ToGlyphs for opentype::tables::glyph_substitution::Type {
    fn to_glyphs(&self) -> Glyphs {
        use opentype::layout::{ChainedContext, Context};
        use opentype::tables::glyph_substitution::{SingleSubstitution, Type};

        let mut values = BTreeSet::default();
        match self {
            Type::SingleSubstitution(SingleSubstitution::Format1(table)) => {
                values.extend(uncover(&table.coverage).map(Glyph::Scalar).map(vector));
            }
            Type::SingleSubstitution(SingleSubstitution::Format2(table)) => {
                values.extend(uncover(&table.coverage).map(Glyph::Scalar).map(vector));
            }
            Type::MultipleSubstitution(table) => {
                values.extend(uncover(&table.coverage).map(Glyph::Scalar).map(vector));
            }
            Type::AlternateSubstitution(table) => {
                values.extend(uncover(&table.coverage).map(Glyph::Scalar).map(vector));
            }
            Type::LigatureSubstitution(table) => {
                values.extend(uncover(&table.coverage).zip(&table.records).flat_map(
                    |(glyph_id, record)| {
                        record.records.iter().map(move |record| {
                            let mut value = Vec::with_capacity(record.glyph_count as usize);
                            value.push(Glyph::Scalar(glyph_id));
                            for glyph_id in record.glyph_ids.iter().cloned() {
                                value.push(Glyph::Scalar(glyph_id));
                            }
                            value
                        })
                    },
                ));
            }
            Type::ContextualSubstitution(Context::Format1(table)) => {
                values.extend(uncover(&table.coverage).zip(&table.records).flat_map(
                    |(glyph_id, record)| {
                        record.records.iter().map(move |record| {
                            let mut value = Vec::with_capacity(record.glyph_count as usize);
                            value.push(Glyph::Scalar(glyph_id));
                            for glyph_id in record.glyph_ids.iter().cloned() {
                                value.push(Glyph::Scalar(glyph_id));
                            }
                            value
                        })
                    },
                ));
            }
            Type::ContextualSubstitution(Context::Format2(table)) => {
                let (classes, mapping) = unclass(&table.class);
                let classes = &classes;
                values.extend(
                    uncover(&table.coverage)
                        .filter_map(|glyph_id| mapping.get(&glyph_id).cloned())
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .filter_map(|class_index| {
                            table.records.get(class_index as usize).and_then(|record| {
                                record.as_ref().map(|record| (class_index, record))
                            })
                        })
                        .flat_map(|(class_index, record)| {
                            record.records.iter().filter_map(move |record| {
                                let mut value = Vec::with_capacity(record.glyph_count as usize);
                                value.push(classes.get(&class_index)?.clone());
                                for class_index in &record.indices {
                                    value.push(classes.get(class_index)?.clone());
                                }
                                Some(value)
                            })
                        }),
                );
            }
            Type::ContextualSubstitution(Context::Format3(table)) => {
                values.insert(table.coverages.iter().cloned().map(Glyph::from).collect());
            }
            Type::ChainedContextualSubstitution(ChainedContext::Format1(table)) => {
                values.extend(uncover(&table.coverage).zip(&table.records).flat_map(
                    |(glyph_id, record)| {
                        record.records.iter().map(move |record| {
                            let mut value = Vec::with_capacity(
                                record.backward_glyph_count as usize
                                    + record.glyph_count as usize
                                    + record.forward_glyph_count as usize,
                            );
                            for glyph_id in record.backward_glyph_ids.iter().rev().cloned() {
                                value.push(Glyph::Scalar(glyph_id));
                            }
                            value.push(Glyph::Scalar(glyph_id));
                            for glyph_id in record.glyph_ids.iter().cloned() {
                                value.push(Glyph::Scalar(glyph_id));
                            }
                            for glyph_id in record.forward_glyph_ids.iter().cloned() {
                                value.push(Glyph::Scalar(glyph_id));
                            }
                            value
                        })
                    },
                ));
            }
            Type::ChainedContextualSubstitution(ChainedContext::Format2(table)) => {
                let (backward_classes, _) = unclass(&table.backward_class);
                let backward_classes = &backward_classes;

                let (classes, mapping) = unclass(&table.class);
                let classes = &classes;

                let (forward_classes, _) = unclass(&table.forward_class);
                let forward_classes = &forward_classes;

                values.extend(
                    uncover(&table.coverage)
                        .filter_map(|glyph_id| mapping.get(&glyph_id).cloned())
                        .collect::<BTreeSet<_>>()
                        .into_iter()
                        .filter_map(|class_index| {
                            table.records.get(class_index as usize).and_then(|record| {
                                record.as_ref().map(|record| (class_index, record))
                            })
                        })
                        .flat_map(|(class_index, record)| {
                            record.records.iter().filter_map(move |record| {
                                let mut value = Vec::with_capacity(
                                    record.backward_glyph_count as usize
                                        + record.glyph_count as usize
                                        + record.forward_glyph_count as usize,
                                );
                                for class_index in record.backward_indices.iter().rev() {
                                    value.push(backward_classes.get(class_index)?.clone());
                                }
                                value.push(classes.get(&class_index)?.clone());
                                for class_index in &record.indices {
                                    value.push(classes.get(class_index)?.clone());
                                }
                                for class_index in &record.forward_indices {
                                    value.push(forward_classes.get(class_index)?.clone());
                                }
                                Some(value)
                            })
                        }),
                );
            }
            Type::ChainedContextualSubstitution(ChainedContext::Format3(table)) => {
                let mut value = table
                    .backward_coverages
                    .iter()
                    .cloned()
                    .rev()
                    .map(Glyph::from)
                    .collect::<Vec<_>>();
                value.extend(table.coverages.iter().cloned().map(Glyph::from));
                value.extend(table.forward_coverages.iter().cloned().map(Glyph::from));
                values.insert(value);
            }
            Type::ReverseChainedContextualSubstibution(table) => {
                let mut value = table
                    .backward_coverages
                    .iter()
                    .cloned()
                    .rev()
                    .map(Glyph::from)
                    .collect::<Vec<_>>();
                value.push(table.coverage.clone().into());
                value.extend(table.forward_coverages.iter().cloned().map(Glyph::from));
                values.insert(value);
            }
            _ => {}
        }
        values
    }
}

impl From<GlyphID> for Glyph {
    #[inline]
    fn from(value: GlyphID) -> Self {
        Self::Scalar(value)
    }
}

impl From<(GlyphID, GlyphID)> for Glyph {
    #[inline]
    fn from(value: (GlyphID, GlyphID)) -> Self {
        Self::Range(value.0, value.1)
    }
}

impl From<Vec<GlyphID>> for Glyph {
    #[inline]
    fn from(value: Vec<GlyphID>) -> Self {
        Self::List(value)
    }
}

impl From<Vec<(GlyphID, GlyphID)>> for Glyph {
    #[inline]
    fn from(value: Vec<(GlyphID, GlyphID)>) -> Self {
        Self::Ranges(value)
    }
}

impl From<Coverage> for Glyph {
    fn from(value: Coverage) -> Self {
        match value {
            Coverage::Format1(value) => value.glyph_ids.into(),
            Coverage::Format2(value) => value
                .records
                .into_iter()
                .map(|record| (record.start_glyph_id, record.end_glyph_id))
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Features> {
    let mut values = Default::default();
    if let Some(table) = cache.try_glyph_positioning()? {
        populate(&mut values, &table.borrow());
    }
    if let Some(table) = cache.try_glyph_substitution()? {
        populate(&mut values, &table.borrow());
    }
    let mapping = cache.reverse_mapping()?.clone();
    Ok(values.to_characters(&mapping, ()))
}

fn populate<T>(
    values: &mut BTreeMap<Feature, BTreeMap<Script, BTreeMap<Language, Glyphs>>>,
    table: &Directory<T>,
) where
    T: ToGlyphs,
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
                        .flat_map(|record| record.tables.iter().flat_map(|table| table.to_glyphs()))
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
                        .flat_map(|record| record.tables.iter().flat_map(|table| table.to_glyphs()))
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

fn to_characters<T>(values: T, mapping: &Mapping, _: &Glyphs) -> Option<Character>
where
    T: Iterator<Item = GlyphID>,
{
    let mut values = values
        .filter_map(|glyph_id| mapping.get(glyph_id))
        .collect::<Vec<_>>();
    values.sort();
    values.dedup();
    if values.is_empty() {
        return None;
    }
    if values.len() == 1 {
        return Some(Character::Scalar(values[0]));
    }
    let mut ranges = Vec::new();
    let (mut start, mut end) = (values[0], values[0]);
    let mut iterator = values.iter().skip(1).cloned();
    loop {
        match iterator.next() {
            Some(next) => {
                if end as usize + 1 == next as usize {
                    continue;
                }
                ranges.push((start, end));
                start = next;
                end = next;
            }
            _ => {
                ranges.push((start, end));
                break;
            }
        }
    }
    if ranges.len() == 1 {
        if ranges[0].0 == ranges[0].1 {
            return Some(Character::Scalar(ranges[0].0));
        }
        return Some(Character::Range(ranges[0].0, ranges[0].1));
    }
    if 2 * ranges.len() < values.len() {
        Some(Character::Ranges(ranges))
    } else {
        Some(Character::List(values))
    }
}

#[inline]
fn vector<T>(value: T) -> Vec<T> {
    vec![value]
}

fn unclass(value: &Class) -> (BTreeMap<u16, Glyph>, BTreeMap<GlyphID, u16>) {
    let mut forward = BTreeMap::<_, BTreeSet<_>>::default();
    let mut reverse = BTreeMap::default();
    match value {
        Class::Format1(value) => {
            let range = value.start_glyph_id..value.start_glyph_id + value.glyph_count;
            for (glyph_id, class_index) in range.zip(value.indices.iter().cloned()) {
                forward.entry(class_index).or_default().insert(glyph_id);
                reverse.insert(glyph_id, class_index);
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
            .map(|(class_index, glyph_ids)| {
                (
                    class_index,
                    glyph_ids.into_iter().collect::<Vec<_>>().into(),
                )
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
