use std::collections::{BTreeSet, HashMap};
use std::io::Result;

use opentype::truetype::tables::character_mapping::{CharacterMapping, Encoding};
use opentype::truetype::GlyphID;

pub struct Forward(HashMap<u32, GlyphID>);

pub struct Reverse(HashMap<GlyphID, BTreeSet<u32>>);

impl Forward {
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

impl Reverse {
    pub fn new(mapping: &Forward) -> Self {
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
