use font::{Glyph, Offset, Segment};
use svg::node::element;
use svg::node::Node;

pub fn draw(glyph: &Glyph) -> element::Group {
    let mut group = element::Group::new();
    let mut data = element::path::Data::new();
    let mut a = Offset::default();
    for contour in glyph.iter() {
        a += contour.offset;
        data = data.move_to(vec![a.0, a.1]);
        for segment in contour.iter() {
            match segment {
                &Segment::Linear(b) => {
                    a += b;
                    data = data.line_by(vec![b.0, b.1]);
                }
                &Segment::Quadratic(b, mut c) => {
                    c += b;
                    a += c;
                    data = data.quadratic_curve_by(vec![b.0, b.1, c.0, c.1]);
                }
                &Segment::Cubic(b, mut c, mut d) => {
                    c += b;
                    d += c;
                    a += d;
                    data = data.cubic_curve_by(vec![b.0, b.1, c.0, c.1, d.0, d.1]);
                }
            }
        }
        data = data.close();
    }
    if !data.is_empty() {
        group.append(element::Path::new().set("d", data));
    }
    group
}
