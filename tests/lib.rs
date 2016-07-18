extern crate font as library;

use library::File;

mod font;
mod glyph;

enum Fixture {
    CFF,
    #[allow(dead_code)]
    TTF,
}

fn setup(fixture: Option<Fixture>) -> File {
    match fixture.unwrap_or(Fixture::CFF) {
        Fixture::CFF => File::open("tests/fixtures/SourceSerifPro-Regular.otf").unwrap(),
        Fixture::TTF => File::open("tests/fixtures/OpenSans-Italic.ttf").unwrap(),
    }
}
