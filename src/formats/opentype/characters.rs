use std::collections::HashMap;

use opentype::truetype::character_mapping::{CharacterMapping, Encoding};
use opentype::truetype::GlyphID;

use crate::Result;

pub struct Characters(pub Vec<(u32, u32)>);

pub struct Mapping(HashMap<u32, GlyphID>);

impl Characters {
    pub fn new(character_mapping: &CharacterMapping) -> Result<Self> {
        for encoding in character_mapping.encodings.iter() {
            let ranges = match encoding {
                Encoding::Format0(encoding) => encoding.characters(),
                Encoding::Format4(encoding) => encoding.characters(),
                Encoding::Format6(encoding) => encoding.characters(),
                Encoding::Format12(encoding) => encoding.characters(),
                _ => continue,
            };
            return Ok(Self(compress(ranges)));
        }
        raise!("found no known character-to-glyph encoding")
    }
}

impl From<Characters> for Vec<(u32, u32)> {
    #[inline]
    fn from(characters: Characters) -> Self {
        characters.0
    }
}

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
    pub fn find(&self, character: char) -> Option<GlyphID> {
        self.0.get(&(character as u32)).copied()
    }
}

fn compress(ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)>{
    let mut result: Vec<(u32, u32)> = Vec::with_capacity(ranges.len());
    for range in ranges {
        if let Some(last) = result.last_mut() {
            if last.1 + 1 == range.0 {
                last.1 = range.1;
                continue;
            }
        }
        result.push(range);
    }
    result
}
