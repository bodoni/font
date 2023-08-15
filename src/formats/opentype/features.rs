//! Features.

pub use opentype::layout::feature::Feature as Type;

use std::collections::HashSet;
use std::io::Result;

use typeface::Tape;

use crate::formats::opentype::cache::Cache;

/// Features.
pub type Features = HashSet<Type>;

pub(crate) fn read<T: Tape>(cache: &mut Cache<T>) -> Result<crate::Features> {
    let mut features = Features::default();
    if let Some(table) = cache.try_glyph_positioning()? {
        for header in table.features.headers.iter() {
            if let Some(r#type) = Type::from_tag(&header.tag) {
                features.insert(r#type);
            }
        }
    }
    if let Some(table) = cache.try_glyph_substitution()? {
        for header in table.features.headers.iter() {
            if let Some(r#type) = Type::from_tag(&header.tag) {
                features.insert(r#type);
            }
        }
    }
    Ok(features)
}
