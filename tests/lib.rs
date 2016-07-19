extern crate font as library;

use library::File;

mod font;
mod glyph;

enum Fixture {
    CFF,
    TTF,
}

fn setup(fixture: Fixture) -> File {
    match fixture {
        Fixture::CFF => File::open("tests/fixtures/SourceSerifPro-Regular.otf").unwrap(),
        Fixture::TTF => File::open("tests/fixtures/OpenSans-Italic.ttf").unwrap(),
    }
}
