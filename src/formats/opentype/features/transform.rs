use std::collections::{BTreeMap, BTreeSet};

use opentype::layout::{Feature, Language, Script};
use opentype::truetype::GlyphID;

use crate::formats::opentype::features;
use crate::formats::opentype::features::graph::{Glyph, Graph};
use crate::formats::opentype::features::sample::{Position, Sample};
use crate::formats::opentype::mapping::Reverse as Mapping;

pub trait Transform<'l> {
    type Target;
    type Parameter: 'l;

    fn transform(self, _: &Mapping, _: Self::Parameter) -> Self::Target;
}

type Features = BTreeMap<Feature, Value>;

type Value = BTreeMap<Script, BTreeMap<Language, Graph>>;

impl<'l> Transform<'l> for &Features {
    type Target = features::Features;
    type Parameter = ();

    fn transform(self, mapping: &Mapping, _: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|(key, value)| (*key, value.transform(mapping, self)))
            .collect()
    }
}

impl<'l> Transform<'l> for &Value {
    type Target = features::Value;
    type Parameter = &'l Features;

    fn transform(self, mapping: &Mapping, features: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|(key, value)| (*key, value.transform(mapping, features)))
            .collect()
    }
}

impl<'l> Transform<'l> for &BTreeMap<Language, Graph> {
    type Target = BTreeMap<Language, BTreeSet<Sample>>;
    type Parameter = &'l Features;

    fn transform(self, mapping: &Mapping, features: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|(key, value)| (*key, value.transform(mapping, features)))
            .collect()
    }
}

impl<'l> Transform<'l> for &Graph {
    type Target = BTreeSet<Sample>;
    type Parameter = &'l Features;

    fn transform(self, mapping: &Mapping, features: Self::Parameter) -> Self::Target {
        postcompress(
            self.iter()
                .filter_map(|(value, _)| value.transform(mapping, features)),
        )
    }
}

impl<'l> Transform<'l> for &[Glyph] {
    type Target = Option<Vec<Position>>;
    type Parameter = &'l Features;

    fn transform(self, mapping: &Mapping, features: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|value| value.transform(mapping, features))
            .collect()
    }
}

impl<'l> Transform<'l> for &Glyph {
    type Target = Option<Position>;
    type Parameter = &'l Features;

    fn transform(self, mapping: &Mapping, features: Self::Parameter) -> Self::Target {
        match self {
            Glyph::Single(value) => map(*value, mapping, features).map(Position::Single),
            Glyph::Range((start, end)) => precompress(*start..=*end, mapping, features),
            Glyph::Ranges(value) => precompress(
                value.iter().flat_map(|value| value.0..=value.1),
                mapping,
                features,
            ),
            Glyph::List(value) => precompress(value.iter().cloned(), mapping, features),
        }
    }
}

#[inline]
fn map(value: GlyphID, mapping: &Mapping, _: &Features) -> Option<char> {
    mapping.get(value)
}

fn precompress<T>(values: T, mapping: &Mapping, features: &Features) -> Option<Position>
where
    T: Iterator<Item = GlyphID>,
{
    let values = values
        .filter_map(|glyph_id| map(glyph_id, mapping, features))
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
                position(&mut values, (start, end));
                range = Some((next, next));
            }
            (Some((start, end)), None) => {
                position(&mut values, (start, end));
                break;
            }
            (None, None) => break,
        }
    }
    match values.len() {
        0 => None,
        1 => values.first().cloned(),
        _ => Some(Position::Set(values)),
    }
}

fn postcompress<T>(values: T) -> BTreeSet<Sample>
where
    T: Iterator<Item = Vec<Position>>,
{
    let values = values
        .filter_map(|mut values| match values.len() {
            0 => None,
            1 => values.pop().map(Sample::Simple),
            _ => Some(Sample::Single(values)),
        })
        .collect::<BTreeSet<_>>();
    let mut iterator = values.into_iter();
    let mut values = BTreeSet::new();
    let mut range = None;
    loop {
        match (range, iterator.next()) {
            (None, Some(Sample::Simple(Position::Single(next)))) => {
                range = Some((next, next));
            }
            (Some((start, end)), Some(Sample::Simple(Position::Single(next)))) => {
                if end as usize + 1 == next as usize {
                    range = Some((start, next));
                    continue;
                }
                sample(&mut values, (start, end));
                range = Some((next, next));
            }
            (None, Some(value)) => {
                values.insert(value);
            }
            (Some((start, end)), Some(value)) => {
                sample(&mut values, (start, end));
                values.insert(value);
                range = None;
            }
            (Some((start, end)), None) => {
                sample(&mut values, (start, end));
                break;
            }
            (None, None) => break,
        }
    }
    values
}

#[inline]
fn position(values: &mut BTreeSet<Position>, (start, end): (char, char)) {
    if start == end {
        values.insert(Position::Single(start));
    } else if start as usize + 1 == end as usize {
        values.insert(Position::Single(start));
        values.insert(Position::Single(end));
    } else {
        values.insert(Position::Range((start, end)));
    }
}

#[inline]
fn sample(values: &mut BTreeSet<Sample>, (start, end): (char, char)) {
    if start == end {
        values.insert(Sample::Simple(Position::Single(start)));
    } else if start as usize + 1 == end as usize {
        values.insert(Sample::Simple(Position::Single(start)));
        values.insert(Sample::Simple(Position::Single(end)));
    } else {
        values.insert(Sample::Range((start, end)));
    }
}
