use {Fixture, setup, trace};

#[test]
fn draw_from_a_to_z() {
    let font = &setup(Fixture::CFF)[0];
    for code in b'a'..(b'z' + 1) {
        font.case.draw(code as char).unwrap().unwrap();
    }
}

#[test]
fn draw_o() {
    let font = &setup(Fixture::CFF)[0];
    let glyph = font.case.draw('o').unwrap().unwrap();
    assert_eq!(glyph.len(), 2);
    assert_eq!(&trace(&glyph), &vec![
        (274.0, 445.0),
        (409.0, 236.0),
        (274.0,  30.0),
        (140.0, 236.0),
        (274.0, 445.0),
        (274.0, 491.0),
        ( 45.0, 237.0),
        (274.0, -15.0),
        (504.0, 237.0),
        (274.0, 491.0),
    ]);
}

#[test]
fn draw_r() {
    let font = &setup(Fixture::CFF)[0];
    let glyph = font.case.draw('r').unwrap().unwrap();
    assert_eq!(glyph.bounding_box, (34.0, 0.0, 412.0, 491.0));
    assert_eq!(glyph.side_bearings, (34.0, 11.0));
}

#[test]
fn open() {
    let file = setup(Fixture::CFF);
    let font = &file[0];
    assert_eq!(font.units_per_em, 1000);
    assert_eq!(font.ascender, 918);
    assert_eq!(font.descender, -335);
}
