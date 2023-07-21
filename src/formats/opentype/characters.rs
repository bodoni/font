use std::collections::HashMap;

use opentype::truetype::character_mapping::{CharacterMapping, Encoding};
use opentype::truetype::GlyphID;

use crate::Result;

pub struct Characters(HashMap<u32, GlyphID>);

impl Characters {
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
