#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::Cursor;
use std::path::PathBuf;

use font::formats::opentype::Disposition;
use font::opentype::truetype::tables::Names;
use font::Case;
use test::{black_box, Bencher};

macro_rules! ok(($result:expr) => ($result.unwrap()));

#[bench]
fn read(bencher: &mut Bencher) {
    use font::formats::opentype::read;

    let path = PathBuf::from("tests")
        .join("fixtures")
        .join("OpenSans-Italic.ttf");
    bencher.iter(|| {
        let file = ok!(File::open(&path));
        let mut font = ok!(ok!(read(file)).pop());
        black_box(ok!(font.names()));
    });
}

#[bench]
fn read_write(bencher: &mut Bencher) {
    use font::formats::opentype::{read, write};

    let path = PathBuf::from("tests")
        .join("fixtures")
        .join("OpenSans-Italic.ttf");
    bencher.iter(|| {
        let file = ok!(File::open(&path));
        let mut font = ok!(ok!(read(file)).pop());
        black_box(ok!(font.names()));
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);
        black_box(ok!(write(font, &mut cursor, |tag| {
            if tag != b"name" {
                Disposition::Retain
            } else {
                Disposition::Update
            }
        })));
    });
}

#[bench]
fn read_update_write(bencher: &mut Bencher) {
    use font::formats::opentype::{read, write};

    let path = PathBuf::from("tests")
        .join("fixtures")
        .join("OpenSans-Italic.ttf");
    bencher.iter(|| {
        let file = ok!(File::open(&path));
        let mut font = ok!(ok!(read(file)).pop());
        let table = ok!(font.names());
        let other = {
            let table = table.borrow();
            let records = table.iter().map(|(id, value)| (id, ok!(value)));
            let language_tags = table.language_tags().map(Option::unwrap);
            ok!(Names::from_iter(
                records,
                language_tags,
                &mut Default::default(),
            ))
        };
        *table.borrow_mut() = other;
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);
        black_box(ok!(write(font, &mut cursor, |tag| {
            if tag != b"name" {
                Disposition::Retain
            } else {
                Disposition::Update
            }
        })));
    });
}
