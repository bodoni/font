use {Fixture, setup};

#[test]
fn draw_cff_letter() {
    use library::Segment::*;

    let font = &setup(Fixture::CFF)[0];
    let glyph = font.case.draw('o').unwrap().unwrap();
    assert_eq!(glyph.len(), 2);

    assert_eq!(glyph[0].offset, (274.0, 445.0));
    assert_eq!(&*glyph[0], &[
        Cubic(( 87.0,    0.0), ( 48.0, -74.0), (  0.0, -135.0)),
        Cubic((  0.0, -134.0), (-48.0, -72.0), (-87.0,    0.0)),
        Cubic((-86.0,    0.0), (-48.0,  72.0), (  0.0,  134.0)),
        Cubic((  0.0,  135.0), ( 48.0,  74.0), ( 86.0,    0.0)),
    ]);

    assert_eq!(glyph[1].offset, (274.0, 491.0));
    assert_eq!(&*glyph[1], &[
        Cubic((-115.0,    0.0), (-114.0, -89.0), (   0.0, -165.0)),
        Cubic((   0.0, -166.0), ( 113.0, -86.0), ( 116.0,    0.0)),
        Cubic(( 117.0,    0.0), ( 113.0,  86.0), (   0.0,  166.0)),
        Cubic((   0.0,  165.0), (-114.0,  89.0), (-116.0,    0.0)),
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
fn draw_ttf_compound_glyph() {
    use library::Segment::*;

    let font = &setup(Fixture::TTF)[0];
    let glyph = font.case.draw('å').unwrap().unwrap();
    assert_eq!(glyph.len(), 4);

    assert_eq!(glyph[0].offset, (639.0, 1116.0));
    assert_eq!(&*glyph[0], &[
        Quadratic((  92.0,     0.0), (  72.0, -49.5)),
        Quadratic((  72.0,   -49.5), (  40.0, -93.0)),
        Linear   ((  11.0,     0.0)),
        Linear   ((  67.0,   172.0)),
        Linear   (( 127.0,     0.0)),
        Linear   ((-233.0, -1096.0)),
        Linear   ((-133.0,     0.0)),
        Linear   ((  26.0,   209.0)),
        Linear   ((  -8.0,     0.0)),
        Quadratic((-179.0,  -229.0), (-198.0,   0.0)),
        Quadratic((-139.0,     0.0), ( -79.0,  99.0)),
        Quadratic(( -79.0,    99.0), (   0.0, 172.0)),
        Quadratic((   0.0,   208.0), (  71.0, 178.0)),
        Quadratic((  71.0,   178.0), ( 125.0, 101.0)),
        Quadratic(( 125.0,   101.0), ( 149.0,   0.0)),
    ]);

    assert_eq!(glyph[1].offset, (598.0, 119.0));
    assert_eq!(glyph[2].offset, (1587.0, 1575.0));
    assert_eq!(glyph[3].offset, (1483.0, 1478.0));
}

#[test]
fn draw_ttf_letter() {
    use library::Segment::*;

    let font = &setup(Fixture::TTF)[0];
    let glyph = font.case.draw('o').unwrap().unwrap();
    assert_eq!(glyph.len(), 2);

    assert_eq!(glyph[0].offset, (643.0, 1110.0));
    assert_eq!(&*glyph[0], &[
        Quadratic(( 190.0,    0.0), ( 110.0, -112.5)),
        Quadratic(( 110.0, -112.5), (   0.0, -197.0)),
        Quadratic((   0.0, -188.0), ( -72.0, -167.0)),
        Quadratic(( -72.0, -167.0), (-123.0,  -91.0)),
        Quadratic((-123.0,  -91.0), (-155.0,    0.0)),
        Quadratic((-192.0,    0.0), (-109.0,  113.0)),
        Quadratic((-109.0,  113.0), (   0.0,  196.0)),
        Quadratic((   0.0,  190.0), (  73.0,  167.5)),
        Quadratic((  73.0,  167.5), ( 124.0,   89.5)),
        Quadratic(( 124.0,   89.5), ( 151.0,    0.0)),
    ]);

    assert_eq!(glyph[1].offset, (1030.0, 711.0));
    assert_eq!(&*glyph[1], &[
        Quadratic((   0.0,  115.0), ( -62.5,   71.5)),
        Quadratic(( -62.5,   71.5), (-107.0,    0.0)),
        Quadratic((-109.0,    0.0), ( -86.5,  -74.0)),
        Quadratic(( -86.5,  -74.0), ( -47.5, -131.5)),
        Quadratic(( -47.5, -131.5), (   0.0, -161.0)),
        Quadratic((   0.0, -131.0), (  63.5,  -71.5)),
        Quadratic((  63.5,  -71.5), ( 119.0,    0.0)),
        Quadratic(( 104.0,    0.0), (  83.0,   73.0)),
        Quadratic((  83.0,   73.0), (  46.5,  134.5)),
        Quadratic((  46.5,  134.5), (   0.0,  173.0)),
    ]);
}

#[test]
fn draw_ttf_lowercase_letters() {
    let font = &setup(Fixture::TTF)[0];
    for code in b'a'..(b'z' + 1) {
        font.case.draw(code as char).unwrap().unwrap();
    }
}
