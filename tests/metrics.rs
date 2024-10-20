#[macro_use]
mod support;

use crate::support::{setup, Fixture};

#[test]
fn crimson_text() {
    let mut file = setup(Fixture::CrimsonText);
    let values = ok!(file[0].metrics());
    assert_eq!(values.granularity, 1024.0);
    assert_eq!(values.clipping_ascender, 1106.0);
    assert_eq!(values.ascender, 972.0);
    assert_eq!(values.cap_height, 656.0);
    assert_eq!(values.x_height, 430.0);
    assert_eq!(values.baseline, 0.0);
    assert_eq!(values.descender, -359.0);
    assert_eq!(values.clipping_descender, -297.0);
    assert_eq!(values.line_gap, 0.0);
}

#[test]
fn noto_naskh_arabic() {
    let mut file = setup(Fixture::NotoNaskhArabic);
    let values = ok!(file[0].metrics());
    assert_eq!(values.granularity, 2048.0);
}

#[test]
fn open_sans() {
    let mut file = setup(Fixture::OpenSans);
    let values = ok!(file[0].metrics());
    assert_eq!(values.granularity, 2048.0);
    assert_eq!(values.clipping_ascender, 2189.0);
    assert_eq!(values.ascender, 1567.0);
    assert_eq!(values.cap_height, 1462.0);
    assert_eq!(values.x_height, 1096.0);
    assert_eq!(values.baseline, 0.0);
    assert_eq!(values.descender, -492.0);
    assert_eq!(values.clipping_descender, -600.0);
    assert_eq!(values.line_gap, 132.0);
}

#[test]
fn source_serif() {
    let mut file = setup(Fixture::SourceSerif);
    let values = ok!(file[0].metrics());
    assert_eq!(values.granularity, 1000.0);
    assert_eq!(values.clipping_ascender, 918.0);
    assert_eq!(values.ascender, 730.0);
    assert_eq!(values.cap_height, 670.0);
    assert_eq!(values.x_height, 475.0);
    assert_eq!(values.baseline, 0.0);
    assert_eq!(values.descender, -270.0);
    assert_eq!(values.clipping_descender, -335.0);
    assert_eq!(values.line_gap, 0.0);
}
