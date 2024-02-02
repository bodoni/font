//! Multilingual strings.

use std::io::Result;
use std::rc::Rc;

use crate::formats::opentype::cache::Cache;

/// Multilingual strings.
pub type Names = Rc<opentype::truetype::tables::Names>;

pub(crate) fn read<T: typeface::tape::Read>(cache: &mut Cache<T>) -> Result<Names> {
    Ok(cache.naming_table()?.clone())
}
