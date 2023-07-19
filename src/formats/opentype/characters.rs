use std::collections::HashMap;

use opentype::truetype::character_mapping::{CharacterMapping, Encoding};
use opentype::truetype::GlyphID;

use crate::Result;

pub struct Characters(HashMap<u32, GlyphID>);

macro_rules! remap(
    ($source:expr) => ({
        let source = $source;
        let mut destination = HashMap::with_capacity(source.len());
        for (&key, &value) in source.iter() {
            destination.insert(key as u32, value);
        }
        destination
    })
);

impl Characters {
    pub fn new(character_mapping: &CharacterMapping) -> Result<Self> {
        if character_mapping.encodings.is_empty() {
            raise!("found no character-to-glyph encoding");
        }
        Ok(Self(match &character_mapping.encodings[0] {
            Encoding::Format0(encoding) => remap!(encoding.mapping()),
            Encoding::Format4(encoding) => remap!(encoding.mapping()),
            Encoding::Format6(encoding) => remap!(encoding.mapping()),
            Encoding::Format12(encoding) => encoding.mapping(),
            Encoding::Format14(encoding) => encoding.mapping(),
            _ => raise!("found an unknown character-to-glyph encoding"),
        }))
    }

    #[inline]
    pub fn find(&self, character: char) -> Option<GlyphID> {
        self.0.get(&(character as u32)).copied()
    }
}
