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
//! let file = File::open(path).unwrap();
//! let font = &file[0];
//!
//! for contour in font.draw('&').unwrap().unwrap().iter() {
//!     for segment in contour.iter() {
//!         match segment {
//!             &Segment::Linear(..) => println!("Line!"),
//!             &Segment::Quadratic(..) => println!("Curve!"),
//!             &Segment::Cubic(..) => println!("Curve!"),
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

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

macro_rules! some(
    ($option:expr, $message:expr) => (match $option {
        Some(value) => value,
        _ => raise!($message),
    });
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
