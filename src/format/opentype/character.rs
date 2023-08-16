use std::collections::HashMap;
use std::io::Result;
use std::ops::RangeInclusive;

use opentype::truetype::character_mapping::{CharacterMapping, Encoding};
use opentype::truetype::GlyphID;
use typeface::Tape;

use crate::format::opentype::cache::Cache;

/// Unicode code points.
pub type Characters = Vec<RangeInclusive<u32>>;

pub struct Mapping(HashMap<u32, GlyphID>);

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

pub(crate) fn read<T: Tape>(cache: &mut Cache<T>) -> Result<Characters> {
    for encoding in cache.character_mapping()?.encodings.iter() {
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

fn compress(ranges: Vec<(u32, u32)>) -> Vec<RangeInclusive<u32>> {
    let mut result: Vec<RangeInclusive<u32>> = Vec::with_capacity(ranges.len());
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
