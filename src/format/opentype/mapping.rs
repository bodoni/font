use std::collections::HashMap;

use truetype::char_mapping::Encoding;
use truetype::{CharMapping, GlyphID};

use crate::Result;

pub struct Mapping(HashMap<u32, GlyphID>);

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

impl Mapping {
    pub fn new(char_mapping: CharMapping) -> Result<Self> {
        if char_mapping.encodings.is_empty() {
            raise!("found no char-to-glyph encoding");
        }
        Ok(Mapping(match &char_mapping.encodings[0] {
            Encoding::Format0(encoding) => remap!(encoding.mapping()),
            Encoding::Format4(encoding) => remap!(encoding.mapping()),
            Encoding::Format6(encoding) => remap!(encoding.mapping()),
            Encoding::Format12(encoding) => encoding.mapping(),
            Encoding::Format14(encoding) => encoding.mapping(),
            _ => raise!("found an unsupported char-to-glyph encoding"),
        }))
    }

    #[inline]
    pub fn find(&self, character: char) -> Option<GlyphID> {
        self.0.get(&(character as u32)).map(|glyph_id| *glyph_id)
    }
}
