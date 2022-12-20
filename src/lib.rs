//! Font toolbox.
//!
//! # Example
//!
//! ```
//! use font::{Font, Segment};
//!
//! let path = "OpenSans-Italic.otf";
//! let font = Font::open(path).unwrap();
//! let glyph = font.draw('&').unwrap().unwrap();
//! for contour in glyph.iter() {
//!     for segment in contour.iter() {
//!         match segment {
//!             &Segment::Linear(..) => { /* … */ },
//!             &Segment::Quadratic(..) => { /* … */ },
//!             &Segment::Cubic(..) => { /* … */ },
//!         }
//!     }
//! }
//! ```

extern crate opentype;
extern crate postscript;
extern crate truetype;
#[macro_use(dereference, raise)]
extern crate typeface;

mod builder;
mod case;
mod file;
mod font;
mod format;
mod glyph;
mod offset;

pub use typeface::{Error, Result};

pub use self::font::Font;
pub use case::Case;
pub use file::File;
pub use glyph::{Contour, Glyph, Segment};
pub use offset::Offset;

/// A number.
pub type Number = f32;
