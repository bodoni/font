use std::io::Result;

use crate::{Axes, Characters, Features, Glyph, Metrics, Names, Palettes, Tables, Timestamps};

/// A font.
pub struct Font<T> {
    format: Format<T>,
}

enum Format<T> {
    OpenType(crate::formats::opentype::Font<T>),
    #[cfg(feature = "webtype")]
    WebType(crate::formats::webtype::Font<T>),
}

macro_rules! implement {
    (
        $(
            $(#[$attribute:meta])*
            fn $function:ident($($argument_name:ident: $argument_type:ty),*) -> $type:ty;
        )+
    ) => (
        /// A type that represents a font in a specific format.
        pub trait Case {
            $(
                $(#[$attribute])*
                fn $function(&mut self $(, $argument_name: $argument_type)*) -> Result<$type>;
            )+
        }

        impl<T> Font<T>
        where
            T: crate::Read,
        {
            $(
                $(#[$attribute])*
                #[inline]
                pub fn $function(&mut self $(, $argument_name: $argument_type)*) -> Result<$type> {
                    match self.format {
                        Format::OpenType(ref mut font) => font.$function($($argument_name),*),
                        #[cfg(feature = "webtype")]
                        Format::WebType(ref mut font) => font.$function($($argument_name),*),
                    }
                }
            )+
        }
    );
}

implement! {
    /// Return the axes.
    fn axes() -> Axes;
    /// Return the characters.
    fn characters() -> Characters;
    /// Return the features.
    fn features() -> Features;
    /// Return the metrics.
    fn metrics() -> Metrics;
    /// Return the names.
    fn names() -> Names;
    /// Return the palettes.
    fn palettes() -> Palettes;
    /// Return the tables.
    fn tables() -> Tables;
    /// Return the timestamps.
    fn timestamps() -> Timestamps;
    /// Return the glyph of a character.
    fn glyph(character: char) -> Option<Glyph>;
}

pub fn read<T: crate::Read>(mut tape: T) -> Result<Vec<Font<T>>> {
    use opentype::truetype::Tag;

    let tag = tape.peek::<Tag>()?;
    if opentype::accept(&tag) {
        return Ok(crate::formats::opentype::read(tape)?
            .into_iter()
            .map(|font| Font {
                format: Format::OpenType(font),
            })
            .collect());
    }
    #[cfg(feature = "webtype")]
    if webtype::accept(&tag) {
        return Ok(crate::formats::webtype::read(tape)?
            .into_iter()
            .map(|font| Font {
                format: Format::WebType(font),
            })
            .collect());
    }
    error!("found an unknown file format")
}
