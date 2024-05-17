//! Unicode code points.

use std::collections::HashMap;
use std::io::Result;
use std::ops::RangeInclusive;

use opentype::truetype::tables::character_mapping::{CharacterMapping, Encoding};
use opentype::truetype::GlyphID;

use crate::formats::opentype::cache::Cache;
use crate::CharacterID;

/// Ranges of Unicode code points.
pub type Characters = Vec<RangeInclusive<CharacterID>>;

pub(crate) struct Mapping(HashMap<CharacterID, GlyphID>);

pub(crate) struct ReverseMapping(HashMap<GlyphID, CharacterID>);

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
        self.0.get(&(character as CharacterID)).copied()
    }
}

impl ReverseMapping {
    pub fn new(mapping: &Mapping) -> Self {
        Self(
            mapping
                .0
                .iter()
                .map(|(character_id, glyph_id)| (*glyph_id, *character_id))
                .collect(),
        )
    }

    #[inline]
    pub fn get(&self, glyph_id: GlyphID) -> Option<CharacterID> {
        self.0.get(&glyph_id).cloned()
    }
}

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Characters> {
    for encoding in cache.character_mapping()?.borrow().encodings.iter() {
        let ranges = match encoding {
            Encoding::Format0(encoding) => encoding.characters(),
            Encoding::Format4(encoding) => encoding.characters(),
            Encoding::Format6(encoding) => encoding.characters(),
            Encoding::Format12(encoding) => encoding.characters(),
            _ => continue,
        };
        return Ok(compress(ranges));
    }
    raise!("found no known character-to-glyph encoding")
}

fn compress(ranges: Vec<(CharacterID, CharacterID)>) -> Vec<RangeInclusive<CharacterID>> {
    let mut result: Vec<RangeInclusive<CharacterID>> = Vec::with_capacity(ranges.len());
    for range in ranges {
        if let Some(last) = result.last_mut() {
            if last.end() + 1 == range.0 {
                *last = *last.start()..=range.1;
                continue;
            }
        }
        result.push(range.0..=range.1);
    }
    result
}
