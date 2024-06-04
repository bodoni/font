use std::collections::BTreeSet;

use opentype::truetype::GlyphID;

use crate::formats::opentype::features::rules::{Glyph, Rule, Rules};
use crate::formats::opentype::features::sample::{Component, Sample};
use crate::formats::opentype::mapping::Reverse as Mapping;

pub trait Transform<'l> {
    type Target;
    type Parameter: 'l;

    fn transform(self, _: &Mapping, _: Self::Parameter) -> Self::Target;
}

impl<'l> Transform<'l> for &[Option<Rules>] {
    type Target = Option<Vec<BTreeSet<Sample>>>;
    type Parameter = &'l [Vec<Option<Rules>>];

    fn transform(self, mapping: &Mapping, rules: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|value| {
                value
                    .as_ref()
                    .and_then(|value| value.transform(mapping, rules))
            })
            .collect()
    }
}

impl<'l> Transform<'l> for &Rules {
    type Target = Option<BTreeSet<Sample>>;
    type Parameter = &'l [Vec<Option<Rules>>];

    fn transform(self, mapping: &Mapping, rules: Self::Parameter) -> Self::Target {
        postcompress(self.iter().map(|value| value.transform(mapping, rules)))
    }
}

impl<'l> Transform<'l> for &Rule {
    type Target = Option<Sample>;
    type Parameter = &'l [Vec<Option<Rules>>];

    fn transform(self, mapping: &Mapping, rules: Self::Parameter) -> Self::Target {
        match self {
            Rule::Simple((source, _)) => source.transform(mapping, rules).map(Sample::Composite),
            Rule::Alternate((source, target)) if target.len() > 1 => source
                .transform(mapping, rules)
                .map(|source| Sample::Alternate((source, target.len()))),
            Rule::Alternate((source, _)) => [Glyph::from(*source)]
                .transform(mapping, rules)
                .map(Sample::Composite),
        }
    }
}

impl<'l> Transform<'l> for &[Glyph] {
    type Target = Option<Vec<BTreeSet<Component>>>;
    type Parameter = &'l [Vec<Option<Rules>>];

    fn transform(self, mapping: &Mapping, rules: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|value| value.transform(mapping, rules))
            .collect()
    }
}

impl<'l> Transform<'l> for &Glyph {
    type Target = Option<BTreeSet<Component>>;
    type Parameter = &'l [Vec<Option<Rules>>];

    fn transform(self, mapping: &Mapping, rules: Self::Parameter) -> Self::Target {
        let value = match self {
            Glyph::Scalar(value) => precompress(
                (*value..=*value).filter_map(|glyph_id| glyph_id.transform(mapping, rules)),
            ),
            Glyph::Range((start, end)) => precompress(
                (*start..=*end).filter_map(|glyph_id| glyph_id.transform(mapping, rules)),
            ),
            Glyph::Ranges(value) => precompress(
                value
                    .iter()
                    .flat_map(|value| value.0..=value.1)
                    .filter_map(|glyph_id| glyph_id.transform(mapping, rules)),
            ),
            Glyph::List(value) => precompress(
                value
                    .iter()
                    .cloned()
                    .filter_map(|glyph_id| glyph_id.transform(mapping, rules)),
            ),
        };
        if !value.is_empty() {
            Some(value)
        } else {
            None
        }
    }
}

impl<'l> Transform<'l> for GlyphID {
    type Target = Option<char>;
    type Parameter = &'l [Vec<Option<Rules>>];

    #[inline]
    fn transform(self, mapping: &Mapping, _: Self::Parameter) -> Self::Target {
        mapping.get(self)
    }
}

fn precompress<T>(values: T) -> BTreeSet<Component>
where
    T: Iterator<Item = char>,
{
    let values = values.collect::<BTreeSet<_>>();
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
    T: Iterator<Item = Option<Sample>>,
{
    let values = values.collect::<Option<BTreeSet<_>>>()?;
    let mut iterator = values.into_iter();
    let mut values = BTreeSet::new();
    let mut range: Option<(char, char)> = None;
    loop {
        match (range, iterator.next()) {
            (None, Some(Sample::Alternate(value))) => {
                values.insert(Sample::Alternate(value));
            }
            (None, Some(Sample::Composite(value))) => {
                if value.len() == 1 && value[0].len() == 1 {
                    if let Some(Component::Scalar(next)) = value[0].first() {
                        range = Some((*next, *next));
                        continue;
                    }
                }
                values.insert(Sample::Composite(value));
            }
            (Some((start, end)), Some(Sample::Alternate(value))) => {
                sample(&mut values, (start, end));
                values.insert(Sample::Alternate(value));
                range = None;
            }
            (Some((start, end)), Some(Sample::Composite(value))) => {
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
                values.insert(Sample::Composite(value));
                range = None;
            }
            (Some((start, end)), None) => {
                sample(&mut values, (start, end));
                break;
            }
            _ => {
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
    } else {
        values.insert(Component::Range((start, end)));
    }
}

#[inline]
fn sample(values: &mut BTreeSet<Sample>, (start, end): (char, char)) {
    if start == end {
        values.insert(Sample::Simple(Component::Scalar(start)));
    } else {
        values.insert(Sample::Simple(Component::Range((start, end))));
    }
}
