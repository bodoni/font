#[test]
fn open() {
    let file = ::setup();
    let font = &file.fonts[0];

    assert_eq!(font.units_per_em, 1000);
    assert_eq!(font.ascender, 918);
    assert_eq!(font.descender, -335);
}
