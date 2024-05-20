//! Unicode code points.

use std::collections::HashMap;
use std::io::Result;

use opentype::truetype::tables::character_mapping::{CharacterMapping, Encoding};
use opentype::truetype::GlyphID;

use crate::formats::opentype::cache::Cache;

/// Ranges of Unicode code points.
pub type Characters = Vec<CharacterRange>;

pub(crate) type CharacterRange = (char, char);

pub(crate) struct Mapping(HashMap<u32, GlyphID>);

pub(crate) struct ReverseMapping(HashMap<GlyphID, u32>);

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
        Self(
            mapping
                .0
                .iter()
                .map(|(character_id, glyph_id)| (*glyph_id, *character_id))
                .collect(),
        )
    }

    #[inline]
    pub fn get(&self, glyph_id: GlyphID) -> Option<char> {
        self.0.get(&glyph_id).cloned().and_then(char::from_u32)
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
        return compress(ranges);
    }
    raise!("found no known character-to-glyph encoding")
}

fn compress(ranges: Vec<(u32, u32)>) -> Result<Vec<CharacterRange>> {
    let mut result: Vec<CharacterRange> = Vec::with_capacity(ranges.len());
    for range in ranges {
        if let (Some(start), Some(end)) = (char::from_u32(range.0), char::from_u32(range.1)) {
            if let Some(last) = result.last_mut() {
                if last.1 as usize + 1 == start as usize {
                    *last = (last.0, end);
                    continue;
                }
            }
            result.push((start, end));
        }
    }
    Ok(result)
}
