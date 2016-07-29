//! Font toolbox.
//!
//! # Example
//!
//! ```
//! extern crate font;
//!
//! use font::{File, Segment};
//!
//! # fn main() {
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let File { fonts, .. } = File::open(path).unwrap();
//! let glyph = fonts[0].draw('&').unwrap().unwrap();
//!
//! for contour in glyph.iter() {
//!     for segment in contour.iter() {
//!         match segment {
//!             &Segment::Linear(..) => { /* … */ },
//!             &Segment::Quadratic(..) => { /* … */ },
//!             &Segment::Cubic(..) => { /* … */ },
//!         }
//!     }
//! }
//! # }
//! ```

extern crate opentype;
extern crate postscript;
extern crate truetype;

/// An error.
pub type Error = std::io::Error;

/// A number.
pub type Number = f32;

/// A result.
pub type Result<T> = std::io::Result<T>;

macro_rules! deref {
    ($struct_name:ident::$field_name:ident => $target_name:ty) => (
        impl ::std::ops::Deref for $struct_name {
            type Target = $target_name;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field_name
            }
        }

        impl ::std::ops::DerefMut for $struct_name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field_name
            }
        }
    );
}

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

macro_rules! some(
    ($option:expr, $message:expr) => (
        match $option {
            Some(value) => value,
            _ => raise!($message),
        }
    );
);

mod builder;
mod case;
mod file;
mod font;
mod format;
mod glyph;
mod offset;

pub use case::Case;
pub use file::File;
pub use font::Font;
pub use glyph::{Contour, Glyph, Segment};
pub use offset::Offset;
