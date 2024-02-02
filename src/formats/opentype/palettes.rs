//! Color palettes.

use std::io::Result;
use std::rc::Rc;

use crate::formats::opentype::cache::Cache;

/// Color palettes.
pub type Palettes = Option<Rc<opentype::tables::ColorPalettes>>;

pub(crate) fn read<T: typeface::tape::Read>(cache: &mut Cache<T>) -> Result<Palettes> {
    Ok(cache.try_color_palettes()?.cloned())
}
