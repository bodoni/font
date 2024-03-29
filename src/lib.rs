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
pub extern crate webtype;

#[macro_use(dereference, error, raise)]
extern crate typeface;

pub mod formats;
pub mod glyph;

mod file;
mod font;
mod metrics;
mod offset;

pub use self::font::{Case, Font};
pub use file::File;
pub use formats::opentype::axes::{self, Axes};
pub use formats::opentype::characters::Characters;
pub use formats::opentype::features::{self, Features};
pub use formats::opentype::names::Names;
pub use formats::opentype::palettes::Palettes;
pub use formats::opentype::tables::Tables;
pub use glyph::Glyph;
pub use metrics::Metrics;
pub use offset::Offset;

/// A number.
pub type Number = f32;
