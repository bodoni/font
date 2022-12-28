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

pub fn setup(fixture: Fixture) -> File {
    File::open(match fixture {
        Fixture::AdobeBlank => "tests/fixtures/selected-fonts/AdobeBlank-Regular.ttf",
        Fixture::CrimsonText => "tests/fixtures/selected-fonts/CrimsonText-Regular.ttf",
        Fixture::Numans => "tests/fixtures/selected-fonts/Numans-Regular.ttf",
        Fixture::OpenSans => "tests/fixtures/selected-fonts/OpenSans-Italic.ttf",
        Fixture::SourceSerif => "tests/fixtures/selected-fonts/SourceSerifPro-Regular.otf",
        Fixture::VesperLibre => "tests/fixtures/selected-fonts/VesperLibre-Regular.ttf",
    })
    .unwrap()
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
