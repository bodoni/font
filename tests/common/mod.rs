#![allow(dead_code)]

use library::{File, Glyph};

pub enum Fixture {
    CFF,
    TTF,
}

pub fn setup(fixture: Fixture) -> File {
    match fixture {
        Fixture::CFF => File::open("tests/fixtures/SourceSerifPro-Regular.otf").unwrap(),
        Fixture::TTF => File::open("tests/fixtures/OpenSans-Italic.ttf").unwrap(),
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
