//! Multilingual strings.

use std::io::Result;

use crate::formats::opentype::cache::{Cache, Reference};

/// Multilingual strings.
pub type Names = Reference<opentype::truetype::tables::Names>;

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Names> {
    Ok(cache.names()?.clone())
}
