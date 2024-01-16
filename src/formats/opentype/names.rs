use std::io::Result;
use std::rc::Rc;

use typeface::Tape;

use crate::formats::opentype::cache::Cache;

/// Multilingual strings.
pub type Names = Rc<opentype::truetype::tables::Names>;

pub(crate) fn read<T: Tape>(cache: &mut Cache<T>) -> Result<Names> {
    Ok(cache.naming_table()?.clone())
}
