extern crate font as library;

use library::File;

mod font;
mod glyph;

fn setup() -> File {
    File::open("tests/fixtures/SourceSerifPro-Regular.otf").unwrap()
}
