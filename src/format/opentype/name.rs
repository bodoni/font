use std::io::Result;
use std::rc::Rc;

use opentype::truetype::NamingTable;
use typeface::Tape;

use crate::format::opentype::cache::Cache;

/// Multilingual strings.
pub type Names = Rc<NamingTable>;

pub(crate) fn read<T: Tape>(cache: &mut Cache<T>) -> Result<Names> {
    Ok(cache.naming_table()?.clone())
}
