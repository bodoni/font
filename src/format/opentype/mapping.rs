use std::collections::HashMap;
use super::truetype::CharMapping;

use Result;

pub struct Mapping(HashMap<u16, u16>);

impl Mapping {
    pub fn new(mut char_mapping: CharMapping) -> Result<Self> {
        if char_mapping.encodings.is_empty() {
            raise!("cannot find a char-to-glyph encoding");
        }
        Ok(Mapping(char_mapping.encodings.swap_remove(0).mapping()))
    }

    #[inline]
    pub fn find(&self, glyph: char) -> Option<usize> {
        self.0.get(&(glyph as u16)).map(|id| *id as usize)
    }
}
