extern crate font;

use font::File;

mod glyph;

fn setup() -> File {
    File::open("tests/fixtures/SourceSerifPro-Regular.otf").unwrap()
}
