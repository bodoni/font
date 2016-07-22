use library::Glyph;

use {Fixture, setup};

#[test]
fn draw_cff_letter() {
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
    assert_eq!(&trace(&glyph), &vec![
        ( 639.0, 1116.0),
        ( 803.0, 1066.5),
        ( 915.0,  924.0),
        ( 926.0,  924.0),
        ( 993.0, 1096.0),
        (1120.0, 1096.0),
        ( 887.0,    0.0),
        ( 754.0,    0.0),
        ( 780.0,  209.0),
        ( 772.0,  209.0),
        ( 395.0,  -20.0),
        ( 177.0,   79.0),
        (  98.0,  350.0),
        ( 169.0,  736.0),
        ( 365.0, 1015.0),
        ( 639.0, 1116.0),
        ( 449.0,  119.0),
        ( 642.0,  211.5),
        ( 798.0,  452.5),
        ( 858.0,  750.0),
        ( 802.0,  914.0),
        ( 655.0,  975.0),
        ( 461.5,  889.0),
        ( 321.0,  656.0),
        ( 270.0,  346.0),
        ( 317.0,  175.5),
        ( 449.0,  119.0),
        ( 989.0, 1456.0),
        ( 929.0, 1299.0),
        ( 770.0, 1241.0),
        ( 610.0, 1298.5),
        ( 551.0, 1454.0),
        ( 613.0, 1606.5),
        ( 770.0, 1665.0),
        ( 930.0, 1608.0),
        ( 989.0, 1456.0),
        ( 885.0, 1454.0),
        ( 853.0, 1538.0),
        ( 770.0, 1569.0),
        ( 688.0, 1538.0),
        ( 655.0, 1454.0),
        ( 684.5, 1369.5),
        ( 770.0, 1339.0),
        ( 853.0, 1369.5),
        ( 885.0, 1454.0),
    ]);
}

#[test]
fn draw_ttf_letter() {
    let font = &setup(Fixture::TTF)[0];
    let glyph = font.case.draw('o').unwrap().unwrap();
    assert_eq!(glyph.len(), 2);
    assert_eq!(&trace(&glyph), &vec![
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
    let glyph = font.case.draw('/').unwrap().unwrap();
    assert_eq!(glyph.len(), 1);
    assert_eq!(&trace(&glyph), &vec![
        ( 893.0,  1462.0),
        (  80.0,     0.0),
        ( -94.0,     0.0),
        ( 719.0,  1462.0),
        ( 893.0,  1462.0),
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
    let mut offset = Offset::zero();
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
