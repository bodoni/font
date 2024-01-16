use std::io::Result;
use std::rc::Rc;

use typeface::Tape;

use crate::formats::opentype::cache::Cache;

/// Color palettes.
pub type Palettes = Option<Rc<opentype::tables::ColorPalettes>>;

pub(crate) fn read<T: Tape>(cache: &mut Cache<T>) -> Result<Palettes> {
    Ok(cache.try_color_palettes()?.cloned())
}
