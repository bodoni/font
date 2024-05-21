//! Builder and parser for fonts.
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
//! let glyph = ok!(ok!(fonts[0].glyph('a')));
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

#[cfg(feature = "webtype")]
pub extern crate webtype;

#[macro_use(dereference, error, raise)]
extern crate typeface;

pub mod formats;
pub mod glyph;

mod file;
mod font;
mod metrics;
mod offset;

pub use typeface::tape::{Read, Write};

pub use self::file::File;
pub use self::font::{Case, Font};
pub use self::formats::opentype::axes::{self, Axes};
pub use self::formats::opentype::characters::{Character, Characters};
pub use self::formats::opentype::features::{self, Features};
pub use self::formats::opentype::names::Names;
pub use self::formats::opentype::palettes::Palettes;
pub use self::formats::opentype::tables::Tables;
pub use self::glyph::Glyph;
pub use self::metrics::Metrics;
pub use self::offset::Offset;

/// A number.
pub type Number = f32;
