//! A parser for fonts.
//!
//! # Example
//!
//! ```
//! use font::{File, Segment};
//!
//! macro_rules! ok(($result:expr) => ($result.unwrap()));
//!
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let File { mut fonts } = ok!(File::open(path));
//! let glyph = ok!(ok!(fonts[0].draw('a')));
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

pub extern crate opentype;

extern crate postscript;
extern crate truetype;

#[macro_use(dereference, raise)]
extern crate typeface;

mod file;
mod font;
mod format;
mod glyph;
mod metrics;
mod offset;

pub use typeface::{Error, Result};

pub use self::font::Font;
pub use file::File;
pub use glyph::{Contour, Glyph, Segment};
pub use metrics::Metrics;
pub use offset::Offset;

/// A number.
pub type Number = f32;
