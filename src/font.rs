use std::ops::Deref;

use Number;
use case::Case;

/// A font.
pub struct Font {
    /// The number of units per em.
    pub units_per_em: usize,
    /// The ascender line relative to the base line.
    pub ascender: Number,
    /// The descender line relative to the base line.
    pub descender: Number,
    /// The collection of glyphs.
    pub case: Box<Case>,
}

impl Deref for Font {
    type Target = Box<Case>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.case
    }
}
