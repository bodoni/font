use std::io::Result;

use crate::formats::opentype::cache::Cache;
use crate::Timestamps;

pub(crate) fn read<T: crate::Read>(cache: &mut Cache<T>) -> Result<Timestamps> {
    let font_header = cache.font_header()?.clone();
    let font_header = font_header.borrow();
    Ok(Timestamps {
        creation: font_header.created,
        modification: font_header.modified,
    })
}
