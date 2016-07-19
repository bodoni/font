use {Fixture, setup};

#[test]
fn draw_cff_letter() {
    use library::Curve::*;
    use library::Segment::*;

    let font = &setup(Fixture::CFF)[0];
    let glyph = font.case.draw('o').unwrap().unwrap();
    assert_eq!(glyph.len(), 2);
    assert_eq!(glyph[0].offset, (274.0, 445.0));
    assert_eq!(&*glyph[0], &[
        Curve(Cubic(( 87.0,    0.0), ( 48.0, -74.0), (  0.0, -135.0))),
        Curve(Cubic((  0.0, -134.0), (-48.0, -72.0), (-87.0,    0.0))),
        Curve(Cubic((-86.0,    0.0), (-48.0,  72.0), (  0.0,  134.0))),
        Curve(Cubic((  0.0,  135.0), ( 48.0,  74.0), ( 86.0,    0.0))),
    ]);
    assert_eq!(glyph[1].offset, (274.0, 491.0));
    assert_eq!(&*glyph[1], &[
        Curve(Cubic((-115.0,    0.0), (-114.0, -89.0), (   0.0, -165.0))),
        Curve(Cubic((   0.0, -166.0), ( 113.0, -86.0), ( 116.0,    0.0))),
        Curve(Cubic(( 117.0,    0.0), ( 113.0,  86.0), (   0.0,  166.0))),
        Curve(Cubic((   0.0,  165.0), (-114.0,  89.0), (-116.0,    0.0))),
    ]);
}

#[test]
fn draw_cff_lowercase_letters() {
    let font = &setup(Fixture::CFF)[0];
    for code in b'a'..(b'z' + 1) {
        font.case.draw(code as char).unwrap().unwrap();
    }
}

#[test]
fn draw_ttf_letter() {
    let font = &setup(Fixture::TTF)[0];
    let glyph = font.case.draw('o').unwrap().unwrap();
    assert_eq!(glyph.len(), 2);
    assert_eq!(glyph[0].offset, (643.0, 1110.0));
    assert_eq!(glyph[1].offset, (1030.0, 711.0));
}
