//! Design axes.

use std::collections::BTreeMap;
use std::io::Result;

use opentype::truetype::{q32, Tag};
use typeface::Tape;

use crate::formats::opentype::cache::Cache;
use crate::Number;

/// Design axes.
pub type Axes = BTreeMap<Type, Value>;

macro_rules! implement(
    ($($tag:literal => $variant:ident,)*) => (
        /// A type.
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum Type {
            $($variant,)*
            Custom(Tag),
        }

        impl Type {
            /// Create an instance from a tag.
            pub fn from_tag(tag: &Tag) -> Self {
                match &**tag {
                    $($tag => Self::$variant,)*
                    _ => Self::Custom(tag.clone()),
                }
            }
        }

        impl From<Type> for Tag {
            fn from(value: Type) -> Self {
                match value {
                    $(Type::$variant => Tag(*$tag),)*
                    Type::Custom(tag) => tag,
                }
            }
        }
    );
);

implement!(
    b"ital" => Italic,
    b"opsz" => OpticalSize,
    b"slnt" => Slant,
    b"wdth" => Width,
    b"wght" => Weight,
);

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
        }
        .into()
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

pub(crate) fn read<T: Tape>(cache: &mut Cache<T>) -> Result<Axes> {
    use opentype::truetype::{PostScript, WindowsMetrics};

    let font_header = cache.font_header()?.clone();
    let machintosh_flags = font_header.macintosh_flags;
    let windows_metrics = cache.windows_metrics()?.clone();
    macro_rules! get(
        ($($version:ident),+) => (
            match &*windows_metrics {
                $(WindowsMetrics::$version(ref table) => (
                    table.weight_class,
                    table.width_class,
                    table.selection_flags,
                ),)*
            }
        );
    );
    let (weight_class, width_class, windows_flags) =
        get!(Version0, Version1, Version2, Version3, Version4, Version5);
    let italic_flag = machintosh_flags.is_italic() || windows_flags.is_italic();

    let postscript = cache.postscript()?.clone();
    macro_rules! get(
        ($($version:ident),+) => (
            match &*postscript {
                $(PostScript::$version(ref table) => table.italic_angle,)*
            }
        );
    );
    let italic_angle = get!(Version1, Version2, Version3);

    let mut axes = Axes::new();
    axes.insert(Type::Italic, Value::from_italic_flag(italic_flag));
    axes.insert(Type::Slant, Value::from_italic_angle(italic_angle));
    axes.insert(Type::Weight, Value::from_weight_class(weight_class));
    axes.insert(Type::Width, Value::from_width_class(width_class));
    if let Some(table) = cache.try_font_variations()? {
        for record in table.axis_records.iter() {
            if record.flags.is_hidden() {
                continue;
            }
            axes.insert(
                Type::from_tag(&record.tag),
                Value {
                    default: record.default_value.into(),
                    range: Some((record.min_value.into(), record.max_value.into())),
                },
            );
        }
    }
    Ok(axes)
}
