extern crate font;
extern crate svg;

use font::{File, Offset, Segment};
use svg::Document;
use svg::node::Node;
use svg::node::element::{Group, Path};
use svg::node::element::path::Data;

fn main() {
    let glyph = '&';
    let font_path = "tests/fixtures/SourceSerifPro-Regular.otf";
    let image_path = "examples/glyph.svg";
    let file = File::open(font_path).unwrap();
    let font = &file[0];
    let glyph = font.draw(glyph).unwrap().unwrap();
    let mut group = Group::new();
    let mut a = Offset::default();
    for contour in glyph.iter() {
        a += contour.offset;
        let mut data = Data::new().move_to(vec![a.0, a.1]);
        for segment in contour.iter() {
            match segment {
                &Segment::Linear(b) => {
                    let b = a + b;
                    data = data.line_to(vec![b.0, b.1]);
                    a = b;
                },
                &Segment::Cubic(b, c, d) => {
                    let b = a + b;
                    let c = b + c;
                    let d = c + d;
                    data = data.cubic_curve_to(vec![b.0, b.1, c.0, c.1, d.0, d.1]);
                    a = d;
                },
                _ => unreachable!(),
            }
        }
        data = data.close();
        group.append(Path::new().set("fill", "none").set("stroke", "black").set("d", data));
    }
    let ((left, bottom, right, top), (lsb, rsb)) = (glyph.bounding_box, glyph.side_bearings);
    let width = (right + rsb) - (left - lsb);
    let height = top - bottom + 2.0 * 50.0;
    group = group.set("transform", format!("translate(0, {}) scale(1, -1)", top + 50.0));
    let image = Document::new().set("width", width).set("height", height).add(group);
    svg::save(image_path, &image).unwrap();
}
