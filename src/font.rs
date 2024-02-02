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
            fn $function:ident(&mut self $(, $argument_name:ident: $argument_type:ty)*) -> $type:ty;
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
    fn axes(&mut self) -> Axes;
    /// Return the characters.
    fn characters(&mut self) -> Characters;
    /// Return the features.
    fn features(&mut self) -> Features;
    /// Return the metrics.
    fn metrics(&mut self) -> Metrics;
    /// Return the names.
    fn names(&mut self) -> Names;
    /// Return the palettes.
    fn palettes(&mut self) -> Palettes;
    /// Return the tables.
    fn tables(&mut self) -> Tables;
    /// Draw a character.
    fn draw(&mut self, character: char) -> Option<Glyph>;
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
