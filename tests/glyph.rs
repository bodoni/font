use library::Glyph;

use {Fixture, setup};

#[test]
fn draw_cff_letter() {
    let font = &setup(Fixture::CFF)[0];
    let glyph = font.case.draw('o').unwrap().unwrap();
    assert_eq!(glyph.len(), 2);
    assert_eq!(&trace(&glyph), &[
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
fn draw_cff_lowercase_letters() {
    let font = &setup(Fixture::CFF)[0];
    for code in b'a'..(b'z' + 1) {
        font.case.draw(code as char).unwrap().unwrap();
    }
}

#[test]
fn draw_ttf_compound_glyph() {
    let font = &setup(Fixture::TTF)[0];
    let glyph = font.case.draw('å').unwrap().unwrap();
    assert_eq!(glyph.len(), 4);
}

#[test]
fn draw_ttf_letter() {
    let font = &setup(Fixture::TTF)[0];
    let glyph = font.case.draw('o').unwrap().unwrap();
    assert_eq!(glyph.len(), 2);
    assert_eq!(&trace(&glyph), &[
        ( 643.0, 1110.0),
        ( 943.0,  997.5),
        (1053.0,  688.0),
        ( 981.0,  333.0),
        ( 786.0,   75.0),
        ( 508.0,  -16.0),
        ( 207.0,   97.0),
        (  98.0,  406.0),
        ( 171.0,  763.5),
        ( 368.0, 1020.5),
        ( 643.0, 1110.0),
        ( 879.0,  711.0),
        ( 816.5,  897.5),
        ( 647.0,  969.0),
        ( 451.5,  895.0),
        ( 317.5,  689.5),
        ( 270.0,  397.0),
        ( 333.5,  194.5),
        ( 516.0,  123.0),
        ( 703.0,  196.0),
        ( 832.5,  403.5),
        ( 879.0,  711.0),
    ]);
}

#[test]
fn draw_ttf_lowercase_letters() {
    let font = &setup(Fixture::TTF)[0];
    for code in b'a'..(b'z' + 1) {
        font.case.draw(code as char).unwrap().unwrap();
    }
}

fn trace(glyph: &Glyph) -> Vec<(f32, f32)> {
    use library::Offset;
    use library::Segment::*;

    let mut points = vec![];
    let mut offset = Offset::from(0.0);
    for contour in glyph.iter() {
        offset += contour.offset;
        points.push(offset.into());
        for segment in contour.iter() {
            match segment {
                &Linear(a) => {
                    offset += a;
                },
                &Quadratic(a, b) => {
                    offset += a;
                    offset += b;
                },
                &Cubic(a, b, c) => {
                    offset += a;
                    offset += b;
                    offset += c;
                },
            }
            points.push(offset.into());
        }
    }
    points
}
