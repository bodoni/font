//! Color palettes.

use std::io::Result;

use crate::formats::opentype::cache::{Cache, Reference};

/// Color palettes.
pub type Palettes = Option<Reference<opentype::tables::ColorPalettes>>;

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Palettes> {
    Ok(cache.try_color_palettes()?.cloned())
}
