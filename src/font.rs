use std::io::Result;

use crate::metrics::Metrics;
use crate::{Axes, Characters, Features, Glyph, Names, Palettes, Tables};

/// A font.
pub struct Font {
    case: Box<dyn Case>,
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

        impl Font {
            $(
                $(#[$attribute])*
                #[inline]
                pub fn $function(&mut self $(, $argument_name: $argument_type)*) -> Result<$type> {
                    self.case.$function($($argument_name),*)
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
    /// Draw a character.
    fn draw(character: char) -> Option<Glyph>;
}

pub fn read<T: typeface::tape::Read + 'static>(mut tape: T) -> Result<Vec<Font>> {
    use opentype::truetype::Tag;

    let tag = tape.peek::<Tag>()?;
    if opentype::accept(&tag) {
        Ok(crate::formats::opentype::read(tape)?
            .into_iter()
            .map(|font| Font {
                case: Box::new(font),
            })
            .collect())
    } else if webtype::accept(&tag) {
        Ok(crate::formats::webtype::read(tape)?
            .into_iter()
            .map(|font| Font {
                case: Box::new(font),
            })
            .collect())
    } else {
        error!("found an unknown file format")
    }
}
