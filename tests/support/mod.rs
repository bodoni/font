#![allow(dead_code)]

macro_rules! ok(($result:expr) => ($result.unwrap()));

use font::{File, Glyph};

pub enum Fixture {
    AdobeBlank,
    CrimsonText,
    Numans,
    OpenSans,
    SourceSerif,
    VesperLibre,
}

pub fn setup(fixture: Fixture) -> File<::std::fs::File> {
    ok!(File::open(match fixture {
        Fixture::AdobeBlank => "tests/fixtures/AdobeBlank-Regular.ttf",
        Fixture::CrimsonText => "tests/fixtures/CrimsonText-Regular.ttf",
        Fixture::Numans => "tests/fixtures/Numans-Regular.ttf",
        Fixture::OpenSans => "tests/fixtures/OpenSans-Italic.ttf",
        Fixture::SourceSerif => "tests/fixtures/SourceSerifPro-Regular.otf",
        Fixture::VesperLibre => "tests/fixtures/VesperLibre-Regular.ttf",
    }))
}

pub fn trace(glyph: &Glyph) -> Vec<(f32, f32)> {
    use font::Offset;
    use font::Segment::*;

    let mut points = vec![];
    let mut offset = Offset::default();
    for contour in glyph.iter() {
        offset += contour.offset;
        points.push(offset.into());
        for segment in contour.iter() {
            match segment {
                &Linear(a) => {
                    offset += a;
                }
                &Quadratic(a, b) => {
                    offset += a;
                    offset += b;
                }
                &Cubic(a, b, c) => {
                    offset += a;
                    offset += b;
                    offset += c;
                }
            }
            points.push(offset.into());
        }
    }
    points
}
