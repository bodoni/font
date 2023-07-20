//! Axes.

use std::collections::HashMap;

use opentype::truetype::q32;

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
    /// Create an instance from a weight class.
    #[inline]
    pub fn from_weight_class(value: u16) -> Self {
        (value as Number).into()
    }

    /// Create an instance from a width class.
    #[inline]
    pub fn from_width_class(value: u16) -> Self {
        match value {
            1 => 50.0,
            2 => 62.5,
            3 => 75.0,
            4 => 87.5,
            5 => 100.0,
            6 => 112.5,
            7 => 125.0,
            8 => 150.0,
            9 => 200.0,
            _ => Number::NAN,
        }.into()
    }

    /// Create an instance from an italic angle.
    pub fn from_italic_angle(value: q32) -> Self {
        Number::from(value).into()
    }

    /// Create an instance from a flag for italic.
    pub fn from_italic_flag(value: bool) -> Self {
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
