//! Features.

pub use opentype::layout::feature::Feature as Type;

use std::collections::HashSet;

/// Features.
pub type Features = HashSet<Type>;
