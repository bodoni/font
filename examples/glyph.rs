extern crate font;
extern crate svg;

use font::Glyph;
use svg::node::element::Group;

const OTF: &'static str = "tests/fixtures/SourceSerifPro-Regular.otf";
const SVG: &'static str = "examples/glyph.svg";

fn main() {
    use font::File;
    use svg::Document;
    use svg::node::element::Style;

    let File { fonts, .. } = File::open(OTF).unwrap();
    let glyph = fonts[0].draw('&').unwrap().unwrap();
    let (width, height) = (glyph.advance_width(), glyph.height() + 2.0 * 50.0);
    let transform = format!("translate(0, {}) scale(1, -1)", glyph.bounding_box.3 + 50.0);
    let glyph = draw(&glyph).set("transform", transform);
    let style = Style::new("path { fill: none; stroke: black; stroke-width: 3; }");
    let document = Document::new().set("width", width).set("height", height).add(style).add(glyph);
    svg::save(SVG, &document).unwrap();
}

fn draw(glyph: &Glyph) -> Group {
    use font::{Offset, Segment};
    use svg::node::Node;
    use svg::node::element::Path;
    use svg::node::element::path::Data;

    let mut group = Group::new();
    let mut a = Offset::default();
    for contour in glyph.iter() {
        a += contour.offset;
        let mut data = Data::new().move_to(vec![a.0, a.1]);
        for segment in contour.iter() {
            match segment {
                &Segment::Linear(b) => {
                    a += b;
                    data = data.line_by(vec![b.0, b.1]);
                },
                &Segment::Cubic(b, mut c, mut d) => {
                    c += b;
                    d += c;
                    a += d;
                    data = data.cubic_curve_by(vec![b.0, b.1, c.0, c.1, d.0, d.1]);
                },
                _ => unreachable!(),
            }
        }
        data = data.close();
        group.append(Path::new().set("d", data));
    }
    group
}
