#![allow(dead_code)]

macro_rules! ok(($result:expr) => ($result.unwrap()));

use std::path::PathBuf;

use font::{File, Glyph};

pub enum Fixture {
    AdobeBlank,
    AdobeVFPrototype,
    CrimsonText,
    MonteCarlo,
    NotoColorEmoji,
    NotoNaskhArabic,
    NotoSerifThai,
    Numans,
    OpenSans,
    Qahiri,
    SourceSerif,
    VesperLibre,
    ZenLoop,
}

pub fn setup(fixture: Fixture) -> File<std::fs::File> {
    ok!(File::open(path(fixture)))
}

pub fn path(fixture: Fixture) -> PathBuf {
    let file_name = match fixture {
        Fixture::AdobeBlank => "AdobeBlank-Regular.ttf",
        Fixture::AdobeVFPrototype => "AdobeVFPrototype.ttf",
        Fixture::CrimsonText => "CrimsonText-Regular.ttf",
        Fixture::MonteCarlo => "MonteCarlo-Regular.ttf",
        Fixture::NotoColorEmoji => "NotoColorEmoji-Regular.ttf",
        Fixture::NotoNaskhArabic => "NotoNaskhArabic-Regular.woff2",
        Fixture::NotoSerifThai => "NotoSerifThai[wdth,wght].ttf",
        Fixture::Numans => "Numans-Regular.ttf",
        Fixture::OpenSans => "OpenSans-Italic.ttf",
        Fixture::Qahiri => "Qahiri-Regular.ttf",
        Fixture::SourceSerif => "SourceSerifPro-Regular.otf",
        Fixture::VesperLibre => "VesperLibre-Regular.ttf",
        Fixture::ZenLoop => "ZenLoop-Regular.ttf",
    };
    PathBuf::from("tests").join("fixtures").join(file_name)
}

pub fn trace(glyph: &Glyph) -> Vec<(f32, f32)> {
    use font::glyph::Segment::*;
    use font::Offset;

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
