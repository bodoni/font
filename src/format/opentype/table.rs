use std::io::Result;

use opentype::truetype::Tag;
use typeface::Tape;

use crate::format::opentype::cache::Cache;

/// Table tags.
pub type Tables = Vec<Tag>;

pub(crate) fn read<T: Tape>(cache: &mut Cache<T>) -> Result<Tables> {
    Ok(cache
        .backend
        .offset_table
        .records
        .iter()
        .map(|record| record.tag)
        .collect())
}
