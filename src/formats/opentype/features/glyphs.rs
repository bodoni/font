use std::collections::{BTreeMap, BTreeSet};

use opentype::layout::{Class, Coverage};
use opentype::truetype::GlyphID;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Glyph {
    Scalar(GlyphID),
    Range(GlyphID, GlyphID),
    Ranges(Vec<(GlyphID, GlyphID)>),
    List(Vec<GlyphID>),
}

pub trait Glyphs {
    #[inline]
    fn glyphs(&self) -> BTreeSet<Vec<Glyph>> {
        Default::default()
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

impl Glyphs for opentype::tables::glyph_positioning::Type {}

impl Glyphs for opentype::tables::glyph_substitution::Type {
    fn glyphs(&self) -> BTreeSet<Vec<Glyph>> {
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
