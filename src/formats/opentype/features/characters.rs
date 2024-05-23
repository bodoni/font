use std::collections::{BTreeMap, BTreeSet};

use opentype::layout::{Feature, Language, Script};
use opentype::truetype::GlyphID;

use super::{Features, Value};
use crate::formats::opentype::characters::{Character, ReverseMapping as Mapping};
use crate::formats::opentype::features::glyphs::Glyph;

pub trait Characters<'l> {
    type Target;
    type Parameter: 'l;

    fn characters(self, _: &Mapping, _: Self::Parameter) -> Self::Target;
}

type Glyphs = BTreeMap<Vec<Glyph>, Vec<Glyph>>;

impl<'l> Characters<'l> for &BTreeMap<Feature, BTreeMap<Script, BTreeMap<Language, Glyphs>>> {
    type Target = Features;
    type Parameter = ();

    fn characters(self, mapping: &Mapping, _: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|(key, value)| (*key, value.characters(mapping, ())))
            .collect()
    }
}

impl<'l> Characters<'l> for &BTreeMap<Script, BTreeMap<Language, Glyphs>> {
    type Target = Value;
    type Parameter = ();

    fn characters(self, mapping: &Mapping, _: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|(key, value)| (*key, value.characters(mapping, ())))
            .collect()
    }
}

impl<'l> Characters<'l> for &BTreeMap<Language, Glyphs> {
    type Target = BTreeMap<Language, Character>;
    type Parameter = ();

    fn characters(self, mapping: &Mapping, _: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|(key, value)| (*key, value.characters(mapping, ())))
            .collect()
    }
}

impl<'l> Characters<'l> for &Glyphs {
    type Target = Character;
    type Parameter = ();

    fn characters(self, mapping: &Mapping, _: Self::Parameter) -> Self::Target {
        postcompress(
            self.iter()
                .filter_map(|(value, _)| value.characters(mapping, self)),
        )
    }
}

impl<'l> Characters<'l> for &[Glyph] {
    type Target = Option<Vec<Character>>;
    type Parameter = &'l Glyphs;

    fn characters(self, mapping: &Mapping, glyphs: Self::Parameter) -> Self::Target {
        self.iter()
            .map(|value| value.characters(mapping, glyphs))
            .collect()
    }
}

impl<'l> Characters<'l> for &Glyph {
    type Target = Option<Character>;
    type Parameter = &'l Glyphs;

    fn characters(self, mapping: &Mapping, glyphs: Self::Parameter) -> Self::Target {
        match self {
            Glyph::Scalar(value) => map(*value, mapping, glyphs).map(Character::Scalar),
            Glyph::Range(start, end) => precompress(*start..=*end, mapping, glyphs),
            Glyph::Ranges(value) => precompress(
                value.iter().flat_map(|value| value.0..=value.1),
                mapping,
                glyphs,
            ),
            Glyph::List(value) => precompress(value.iter().cloned(), mapping, glyphs),
        }
    }
}

#[inline]
fn map(value: GlyphID, mapping: &Mapping, _: &Glyphs) -> Option<char> {
    mapping.get(value)
}

fn precompress<T>(values: T, mapping: &Mapping, glyphs: &Glyphs) -> Option<Character>
where
    T: Iterator<Item = GlyphID>,
{
    let values = values
        .filter_map(|glyph_id| map(glyph_id, mapping, glyphs))
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
                insert(&mut values, (start, end));
                range = Some((next, next));
            }
            (Some((start, end)), None) => {
                insert(&mut values, (start, end));
                break;
            }
            (None, None) => break,
        }
    }
    match values.len() {
        0 => None,
        1 => values.first().cloned(),
        _ => Some(Character::List(values.into_iter().collect())),
    }
}

fn postcompress<T>(values: T) -> Character
where
    T: Iterator<Item = Vec<Character>>,
{
    let values = values
        .filter_map(|mut values| match values.len() {
            0 => None,
            1 => values.pop(),
            _ => Some(Character::List(values)),
        })
        .collect::<BTreeSet<_>>();
    let mut iterator = values.into_iter();
    let mut values = BTreeSet::new();
    let mut range = None;
    loop {
        match (range, iterator.next()) {
            (None, Some(Character::Scalar(next))) => {
                range = Some((next, next));
            }
            (Some((start, end)), Some(Character::Scalar(next))) => {
                if end as usize + 1 == next as usize {
                    range = Some((start, next));
                    continue;
                }
                inline(&mut values, (start, end));
                range = Some((next, next));
            }
            (None, Some(value)) => {
                values.insert(value);
            }
            (Some((start, end)), Some(value)) => {
                inline(&mut values, (start, end));
                values.insert(value);
                range = None;
            }
            (Some((start, end)), None) => {
                inline(&mut values, (start, end));
                break;
            }
            (None, None) => break,
        }
    }
    Character::List(values.into_iter().collect())
}

#[inline]
fn inline(values: &mut BTreeSet<Character>, (start, end): (char, char)) {
    if start == end {
        values.insert(Character::Scalar(start));
    } else if start as usize + 1 == end as usize {
        values.insert(Character::Scalar(start));
        values.insert(Character::Scalar(end));
    } else {
        values.insert(Character::Inline(start, end));
    }
}

#[inline]
fn insert(values: &mut BTreeSet<Character>, (start, end): (char, char)) {
    if start == end {
        values.insert(Character::Scalar(start));
    } else if start as usize + 1 == end as usize {
        values.insert(Character::Scalar(start));
        values.insert(Character::Scalar(end));
    } else {
        values.insert(Character::Range(start, end));
    }
}
