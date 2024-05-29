use std::collections::BTreeSet;

use opentype::truetype::GlyphID;

use crate::formats::opentype::features::graph::{Glyph, Graph};
use crate::formats::opentype::features::sample::{Component, Sample};
use crate::formats::opentype::mapping::Reverse as Mapping;

pub trait Transform<'l> {
    type Target;
    type Parameter: 'l;

    fn transform(self, _: &Mapping, _: Self::Parameter) -> Self::Target;
}

impl<'l> Transform<'l> for &[Graph] {
    type Target = Option<Vec<BTreeSet<Sample>>>;
    type Parameter = &'l [Vec<Graph>];

    fn transform(self, mapping: &Mapping, graphs: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|value| value.transform(mapping, graphs))
            .collect()
    }
}

impl<'l> Transform<'l> for &Graph {
    type Target = Option<BTreeSet<Sample>>;
    type Parameter = &'l [Vec<Graph>];

    fn transform(self, mapping: &Mapping, graphs: Self::Parameter) -> Self::Target {
        postcompress(
            self.iter()
                .map(|(value, _)| value.transform(mapping, graphs)),
        )
    }
}

impl<'l> Transform<'l> for &[Glyph] {
    type Target = Option<Vec<BTreeSet<Component>>>;
    type Parameter = &'l [Vec<Graph>];

    fn transform(self, mapping: &Mapping, graphs: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|value| value.transform(mapping, graphs))
            .collect()
    }
}

impl<'l> Transform<'l> for &Glyph {
    type Target = Option<BTreeSet<Component>>;
    type Parameter = &'l [Vec<Graph>];

    fn transform(self, mapping: &Mapping, graphs: Self::Parameter) -> Self::Target {
        let value = match self {
            Glyph::Scalar(value) => precompress(*value..=*value, mapping, graphs),
            Glyph::Range((start, end)) => precompress(*start..=*end, mapping, graphs),
            Glyph::Ranges(value) => precompress(
                value.iter().flat_map(|value| value.0..=value.1),
                mapping,
                graphs,
            ),
            Glyph::List(value) => precompress(value.iter().cloned(), mapping, graphs),
        };
        if !value.is_empty() {
            Some(value)
        } else {
            None
        }
    }
}

#[inline]
fn map(value: GlyphID, mapping: &Mapping, _: &[Vec<Graph>]) -> Option<char> {
    mapping.get(value)
}

fn precompress<T>(values: T, mapping: &Mapping, graphs: &[Vec<Graph>]) -> BTreeSet<Component>
where
    T: Iterator<Item = GlyphID>,
{
    let values = values
        .filter_map(|glyph_id| map(glyph_id, mapping, graphs))
        .collect::<BTreeSet<_>>();
    let mut iterator = values.into_iter();
    let mut values = BTreeSet::new();
    let mut range = None;
    loop {
        match (range, iterator.next()) {
            (None, Some(next)) => {
                range = Some((next, next));
            }
            (Some((start, end)), Some(next)) => {
                if end as usize + 1 == next as usize {
                    range = Some((start, next));
                    continue;
                }
                component(&mut values, (start, end));
                range = Some((next, next));
            }
            (Some((start, end)), None) => {
                component(&mut values, (start, end));
                break;
            }
            (None, None) => {
                break;
            }
        }
    }
    values
}

fn postcompress<T>(values: T) -> Option<BTreeSet<Sample>>
where
    T: Iterator<Item = Option<Vec<BTreeSet<Component>>>>,
{
    let values = values.collect::<Option<BTreeSet<_>>>()?;
    let mut iterator = values.into_iter();
    let mut values = BTreeSet::new();
    let mut range: Option<(char, char)> = None;
    loop {
        match (range, iterator.next()) {
            (None, Some(value)) => {
                if value.len() == 1 && value[0].len() == 1 {
                    if let Some(Component::Scalar(next)) = value[0].first() {
                        range = Some((*next, *next));
                        continue;
                    }
                }
                values.insert(Sample::Compound(value));
            }
            (Some((start, end)), Some(value)) => {
                if value.len() == 1 && value[0].len() == 1 {
                    if let Some(Component::Scalar(next)) = value[0].first() {
                        if end as usize + 1 == *next as usize {
                            range = Some((start, *next));
                            continue;
                        }
                        sample(&mut values, (start, end));
                        range = Some((*next, *next));
                        continue;
                    }
                }
                sample(&mut values, (start, end));
                values.insert(Sample::Compound(value));
                range = None;
            }
            (Some((start, end)), None) => {
                sample(&mut values, (start, end));
                break;
            }
            (None, None) => {
                break;
            }
        }
    }
    Some(values)
}

#[inline]
fn component(values: &mut BTreeSet<Component>, (start, end): (char, char)) {
    if start == end {
        values.insert(Component::Scalar(start));
    } else if start as usize + 1 == end as usize {
        values.insert(Component::Scalar(start));
        values.insert(Component::Scalar(end));
    } else {
        values.insert(Component::Range((start, end)));
    }
}

#[inline]
fn sample(values: &mut BTreeSet<Sample>, (start, end): (char, char)) {
    if start == end {
        values.insert(Sample::Simple(Component::Scalar(start)));
    } else if start as usize + 1 == end as usize {
        values.insert(Sample::Simple(Component::Scalar(start)));
        values.insert(Sample::Simple(Component::Scalar(end)));
    } else {
        values.insert(Sample::Simple(Component::Range((start, end))));
    }
}
