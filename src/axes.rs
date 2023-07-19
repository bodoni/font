//! Axes.

use std::collections::HashMap;

use crate::Number;

/// Axes.
pub type Axes = HashMap<Type, Value>;

/// A type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Italic,
    OpticalSize,
    Slant,
    Weight,
    Width,
}

/// A value.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Value {
    /// The default value.
    pub default: Number,
    /// The value range.
    pub range: Option<(Number, Number)>,
}

impl Value {
    /// Create an instance from a flag for bold.
    pub fn from_bold(value: bool) -> Self {
        if value {
            700.0.into()
        } else {
            400.0.into()
        }
    }

    /// Create an instance from a flag for italic.
    pub fn from_italic(value: bool) -> Self {
        if value {
            1.0.into()
        } else {
            0.0.into()
        }
    }
}

impl From<Number> for Value {
    #[inline]
    fn from(default: Number) -> Self {
        Self {
            default,
            ..Default::default()
        }
    }
}
