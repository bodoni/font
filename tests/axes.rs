#[macro_use]
mod support;

use std::collections::HashMap;

use font::axes::Type;
use font::opentype::truetype::Tag;

use crate::support::{setup, Fixture};

#[test]
fn adobe_vf_prototype() {
    let mut file = setup(Fixture::AdobeVFPrototype);

    let values = ok!(file[0].axes());
    assert_eq!(values.len(), 5);
    assert!(values[&Type::Italic].range.is_none());
    assert!(values[&Type::Slant].range.is_none());
    assert_eq!(ok!(values[&Type::Weight].range), (200.0, 900.0));
    assert_eq!(values[&Type::Weight].default.round(), 389.0);
    assert!(values[&Type::Width].range.is_none());
    assert_eq!(values[&Type::Width].default, 100.0);

    let value = values[&Type::Other(Tag(*b"CNTR"))];
    let values: HashMap<_, _> = ok!(file[0].names())
        .borrow()
        .iter()
        .map(|((_, _, _, name_id), value)| (name_id, value.unwrap()))
        .collect();
    assert_eq!(values[&value.name_id], "Contrast");
}

#[test]
fn crimson_text() {
    let mut file = setup(Fixture::CrimsonText);
    let values = ok!(file[0].axes());
    assert_eq!(values[&Type::Italic].default, 0.0);
}

#[test]
fn noto_naskh_arabic() {
    let mut file = setup(Fixture::NotoNaskhArabic);
    let values = ok!(file[0].axes());
    assert!(values.values().all(|value| value.range.is_none()));
}

#[test]
fn open_sans() {
    let mut file = setup(Fixture::OpenSans);
    let values = ok!(file[0].axes());
    assert_eq!(values[&Type::Slant].default, -12.0);
}
