use {Fixture, setup};

#[test]
fn open_cff() {
    let file = setup(Fixture::CFF);
    let font = &file[0];
    assert_eq!(font.units_per_em, 1000);
    assert_eq!(font.ascender, 918);
    assert_eq!(font.descender, -335);
}

#[test]
fn open_ttf() {
    let file = setup(Fixture::TTF);
    let font = &file[0];
    assert_eq!(font.units_per_em, 2048);
    assert_eq!(font.ascender, 2189);
    assert_eq!(font.descender, -600);
}
