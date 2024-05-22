//! Unicode code points.

use std::collections::{BTreeSet, HashMap};
use std::io::Result;

use opentype::truetype::tables::character_mapping::{CharacterMapping, Encoding};
use opentype::truetype::GlyphID;

use crate::formats::opentype::cache::Cache;

/// A character.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Character {
    Scalar(char),
    Range(char, char),
    Ranges(Vec<(char, char)>),
    List(Vec<char>),
}

/// Characters.
pub type Characters = Vec<Character>;

pub(crate) struct Mapping(HashMap<u32, GlyphID>);

pub(crate) struct ReverseMapping(HashMap<GlyphID, BTreeSet<u32>>);

impl Mapping {
    pub fn new(character_mapping: &CharacterMapping) -> Result<Self> {
        for encoding in character_mapping.encodings.iter() {
            match encoding {
                Encoding::Format0(encoding) => return Ok(Self(encoding.mapping())),
                Encoding::Format4(encoding) => return Ok(Self(encoding.mapping())),
                Encoding::Format6(encoding) => return Ok(Self(encoding.mapping())),
                Encoding::Format12(encoding) => return Ok(Self(encoding.mapping())),
                _ => {}
            }
        }
        raise!("found no known character-to-glyph encoding")
    }

    #[inline]
    pub fn get(&self, character: char) -> Option<GlyphID> {
        self.0.get(&(character as u32)).copied()
    }
}

impl ReverseMapping {
    pub fn new(mapping: &Mapping) -> Self {
        let mut values = HashMap::<_, BTreeSet<_>>::default();
        for (character_id, glyph_id) in &mapping.0 {
            values.entry(*glyph_id).or_default().insert(*character_id);
        }
        Self(values)
    }

    #[inline]
    pub fn get(&self, glyph_id: GlyphID) -> Option<char> {
        self.0
            .get(&glyph_id)
            .and_then(BTreeSet::first)
            .cloned()
            .and_then(char::from_u32)
    }
}

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Vec<Character>> {
    for encoding in cache.character_mapping()?.borrow().encodings.iter() {
        let ranges = match encoding {
            Encoding::Format0(encoding) => encoding.characters(),
            Encoding::Format4(encoding) => encoding.characters(),
            Encoding::Format6(encoding) => encoding.characters(),
            Encoding::Format12(encoding) => encoding.characters(),
            _ => continue,
        };
        return compress(ranges);
    }
    raise!("found no known character-to-glyph encoding")
}

fn compress(ranges: Vec<(u32, u32)>) -> Result<Vec<Character>> {
    let mut values = Vec::with_capacity(ranges.len());
    for range in ranges {
        if let (Some(start), Some(end)) = (char::from_u32(range.0), char::from_u32(range.1)) {
            if let Some(value) = values.last_mut() {
                let (first, last) = match value {
                    Character::Scalar(first) => (*first, *first),
                    Character::Range(first, last) => (*first, *last),
                    _ => unreachable!(),
                };
                if last as usize + 1 == start as usize {
                    *value = Character::Range(first, end);
                    continue;
                }
            }
            if start == end {
                values.push(Character::Scalar(start));
            } else {
                values.push(Character::Range(start, end));
            }
        }
    }
    Ok(values)
}
