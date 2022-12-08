#![allow(dead_code)]

use library::{File, Glyph};

pub enum Fixture {
    OpenSans,
    SourceSerif,
    VesperLibre,
}

pub fn setup(fixture: Fixture) -> File {
    match fixture {
        Fixture::OpenSans => File::open("tests/fixtures/OpenSans-Italic.ttf").unwrap(),
        Fixture::SourceSerif => File::open("tests/fixtures/SourceSerifPro-Regular.otf").unwrap(),
        Fixture::VesperLibre => File::open("tests/fixtures/VesperLibre-Regular.ttf").unwrap(),
    }
}

pub fn trace(glyph: &Glyph) -> Vec<(f32, f32)> {
    use library::Offset;
    use library::Segment::*;

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
