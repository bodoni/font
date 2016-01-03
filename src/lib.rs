//! Font toolbox.
//!
//! # Example
//!
//! ```
//! extern crate font;
//!
//! use font::File;
//!
//! # fn main() {
//! let path = "SourceSerifPro-Regular.otf";
//! # let path = "tests/fixtures/SourceSerifPro-Regular.otf";
//! let file = File::open(path).unwrap();
//! # }
//! ```

extern crate opentype;

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

macro_rules! raise(
    ($message:expr) => (return Err(::Error::new(::std::io::ErrorKind::Other, $message)));
);

mod file;

pub use file::{File, OpenType};
