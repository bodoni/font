use std::collections::{BTreeMap, BTreeSet};

use opentype::layout::context::Action;
use opentype::layout::{Class, Coverage, Directory};
use opentype::truetype::GlyphID;

pub type Rules = BTreeSet<Rule>;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Rule {
    Simple((Vec<Glyph>, Vec<Glyph>)),
    Alternate((GlyphID, Vec<GlyphID>)),
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Glyph {
    Scalar(GlyphID),
    Range((GlyphID, GlyphID)),
    Ranges(Vec<(GlyphID, GlyphID)>),
    List(Vec<GlyphID>),
}

pub trait Table: Sized {
    #[inline]
    fn extract(&self, _: &Directory<Self>) -> Option<Rules>
    where
        Self: Sized,
    {
        None
    }

    #[inline]
    fn ascend(value: Rule, _: &Directory<Self>) -> Rule {
        value
    }

    #[inline]
    fn descend(_: &[Action], _: &Directory<Self>) -> Vec<Glyph> {
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
        Self::Range(value)
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

impl Table for opentype::tables::glyph_positioning::Type {}

impl Table for opentype::tables::glyph_substitution::Type {
    fn extract(&self, directory: &Directory<Self>) -> Option<Rules> {
        use opentype::layout::{ChainedContext, Context};
        use opentype::tables::glyph_substitution::{SingleSubstitution, Type};

        let mut values = Rules::default();
        match self {
            Type::SingleSubstitution(SingleSubstitution::Format1(table)) => {
                values.extend(uncover(&table.coverage).filter_map(|glyph_id| {
                    let other_id = glyph_id as isize + table.delta_glyph_id as isize;
                    GlyphID::try_from(other_id).ok().map(|other_id| {
                        Self::ascend(
                            Rule::Simple((vec![glyph_id.into()], vec![other_id.into()])),
                            directory,
                        )
                    })
                }));
            }
            Type::SingleSubstitution(SingleSubstitution::Format2(table)) => {
                values.extend(
                    uncover(&table.coverage)
                        .zip(table.glyph_ids.iter().cloned())
                        .map(|(glyph_id, other_id)| {
                            Self::ascend(
                                Rule::Simple((vec![glyph_id.into()], vec![other_id.into()])),
                                directory,
                            )
                        }),
                );
            }
            Type::MultipleSubstitution(table) => {
                values.extend(uncover(&table.coverage).zip(&table.records).map(
                    |(glyph_id, record)| {
                        Self::ascend(
                            Rule::Simple((
                                vec![glyph_id.into()],
                                record.glyph_ids.iter().cloned().map(Into::into).collect(),
                            )),
                            directory,
                        )
                    },
                ));
            }
            Type::AlternateSubstitution(table) => {
                values.extend(uncover(&table.coverage).zip(&table.records).map(
                    |(glyph_id, record)| {
                        Self::ascend(
                            Rule::Alternate((glyph_id, record.glyph_ids.to_vec())),
                            directory,
                        )
                    },
                ));
            }
            Type::LigatureSubstitution(table) => {
                values.extend(uncover(&table.coverage).zip(&table.records).flat_map(
                    |(glyph_id, record)| {
                        record.records.iter().map(move |record| {
                            let mut value = Vec::with_capacity(record.glyph_count as usize);
                            value.push(glyph_id.into());
                            value.extend(record.glyph_ids.iter().cloned().map(Into::into));
                            Self::ascend(
                                Rule::Simple((value, vec![record.glyph_id.into()])),
                                directory,
                            )
                        })
                    },
                ));
            }
            Type::ContextualSubstitution(Context::Format1(table)) => {
                values.extend(uncover(&table.coverage).zip(&table.records).flat_map(
                    |(glyph_id, record)| {
                        record
                            .records
                            .iter()
                            .filter(|record| record.action_count > 0)
                            .map(move |record| {
                                let mut value = Vec::with_capacity(record.glyph_count as usize);
                                value.push(glyph_id.into());
                                value.extend(record.glyph_ids.iter().cloned().map(Into::into));
                                Self::ascend(
                                    Rule::Simple((
                                        value,
                                        Self::descend(&record.actions, directory),
                                    )),
                                    directory,
                                )
                            })
                    },
                ));
            }
            Type::ContextualSubstitution(Context::Format2(table)) => {
                let (classes, mapping) = unclass(&table.class);
                let classes = &classes;
                values.extend(
                    deduplicate(
                        uncover(&table.coverage)
                            .filter_map(|glyph_id| mapping.get(&glyph_id).cloned()),
                    )
                    .filter_map(|class_index| {
                        table
                            .records
                            .get(class_index as usize)
                            .and_then(|record| record.as_ref().map(|record| (class_index, record)))
                    })
                    .flat_map(|(class_index, record)| {
                        record
                            .records
                            .iter()
                            .filter(|record| record.action_count > 0)
                            .map(move |record| {
                                let mut value = Vec::with_capacity(record.glyph_count as usize);
                                value.push(classes.get(&class_index)?.clone());
                                for class_index in &record.indices {
                                    value.push(classes.get(class_index)?.clone());
                                }
                                Some(Self::ascend(
                                    Rule::Simple((
                                        value,
                                        Self::descend(&record.actions, directory),
                                    )),
                                    directory,
                                ))
                            })
                    })
                    .collect::<Option<Vec<_>>>()?,
                );
            }
            Type::ContextualSubstitution(Context::Format3(table)) => {
                if table.action_count > 0 {
                    let value = table.coverages.iter().cloned().map(Glyph::from).collect();
                    values.insert(Self::ascend(
                        Rule::Simple((value, Self::descend(&table.actions, directory))),
                        directory,
                    ));
                }
            }
            Type::ChainedContextualSubstitution(ChainedContext::Format1(table)) => {
                values.extend(uncover(&table.coverage).zip(&table.records).flat_map(
                    |(glyph_id, record)| {
                        record
                            .records
                            .iter()
                            .filter(|record| record.action_count > 0)
                            .map(move |record| {
                                let mut value = Vec::with_capacity(
                                    record.backward_glyph_count as usize
                                        + record.glyph_count as usize
                                        + record.forward_glyph_count as usize,
                                );
                                value.extend(
                                    record
                                        .backward_glyph_ids
                                        .iter()
                                        .rev()
                                        .cloned()
                                        .map(Into::into),
                                );
                                value.push(glyph_id.into());
                                value.extend(record.glyph_ids.iter().cloned().map(Into::into));
                                value.extend(
                                    record.forward_glyph_ids.iter().cloned().map(Into::into),
                                );
                                Self::ascend(
                                    Rule::Simple((
                                        value,
                                        Self::descend(&record.actions, directory),
                                    )),
                                    directory,
                                )
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
                    deduplicate(
                        uncover(&table.coverage)
                            .filter_map(|glyph_id| mapping.get(&glyph_id).cloned()),
                    )
                    .filter_map(|class_index| {
                        table
                            .records
                            .get(class_index as usize)
                            .and_then(|record| record.as_ref().map(|record| (class_index, record)))
                    })
                    .flat_map(|(class_index, record)| {
                        record
                            .records
                            .iter()
                            .filter(|record| record.action_count > 0)
                            .map(move |record| {
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
                                Some(Self::ascend(
                                    Rule::Simple((
                                        value,
                                        Self::descend(&record.actions, directory),
                                    )),
                                    directory,
                                ))
                            })
                    })
                    .collect::<Option<Vec<_>>>()?,
                );
            }
            Type::ChainedContextualSubstitution(ChainedContext::Format3(table)) => {
                if table.action_count > 0 {
                    let mut value = table
                        .backward_coverages
                        .iter()
                        .cloned()
                        .rev()
                        .map(Glyph::from)
                        .collect::<Vec<_>>();
                    value.extend(table.coverages.iter().cloned().map(Glyph::from));
                    value.extend(table.forward_coverages.iter().cloned().map(Glyph::from));
                    values.insert(Self::ascend(
                        Rule::Simple((value, Self::descend(&table.actions, directory))),
                        directory,
                    ));
                }
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
                values.insert(Self::ascend(
                    Rule::Simple((
                        value,
                        table.glyph_ids.iter().cloned().map(Into::into).collect(),
                    )),
                    directory,
                ));
            }
            _ => {
                return None;
            }
        }
        Some(values)
    }
}

fn deduplicate<T, U>(values: T) -> impl Iterator<Item = U>
where
    T: Iterator<Item = U>,
    U: std::cmp::Ord + Clone,
{
    let mut seen = BTreeSet::new();
    values.filter(move |value| seen.insert(value.clone()))
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
