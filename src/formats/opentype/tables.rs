use std::io::Result;

use opentype::truetype::Tag;

use crate::formats::opentype::cache::Cache;

/// Tables.
pub type Tables = Vec<Tag>;

pub(crate) fn read<T: typeface::tape::Read>(cache: &mut Cache<T>) -> Result<Tables> {
    Ok(cache
        .backend
        .offsets
        .records
        .iter()
        .map(|record| record.tag)
        .collect())
}
