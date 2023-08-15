//! Parser for fonts.
//!
//! # Example
//!
//! ```
//! use font::glyph::Segment;
//! use font::File;
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
pub extern crate webtype;

#[macro_use(dereference, error, raise)]
extern crate typeface;

pub mod glyph;

mod file;
mod font;
mod formats;
mod metrics;
mod offset;

pub use self::font::Font;
pub use file::File;
pub use formats::opentype::axes::{self, Axes};
pub use formats::opentype::features::{self, Features};
pub use formats::opentype::names::Names;
pub use glyph::Glyph;
pub use metrics::Metrics;
pub use offset::Offset;

/// Characters.
pub type Characters = Vec<std::ops::RangeInclusive<u32>>;

/// A number.
pub type Number = f32;
