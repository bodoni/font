extern crate arguments;
extern crate font;
extern crate svg;

mod support;

use font::File;
use svg::node::element;
use svg::Document;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let font = match arguments.get::<String>("font") {
        Some(font) => font,
        _ => {
            println!("Error: --font should be given.");
            return;
        }
    };
    let character = match arguments.get::<String>("character") {
        Some(character) => character.chars().next().unwrap(),
        _ => {
            println!("Error: --character should be given.");
            return;
        }
    };
    let File { mut fonts } = File::open(font).unwrap();
    let metrics = fonts[0].metrics().unwrap();
    let glyph = fonts[0].draw(character).unwrap().unwrap();
    let (width, height) = (
        glyph.width() + 2.0 * glyph.side_bearings.0,
        metrics.ascender - metrics.descender,
    );
    let transform = format!("translate(0, {}) scale(1, -1)", metrics.ascender);
    let glyph = support::drawing::draw(&glyph).set("transform", transform);
    let style = element::Style::new("path { fill: black; fill-rule: nonzero }");
    let document = Document::new()
        .set("width", width)
        .set("height", height)
        .add(style)
        .add(glyph);
    print!("{}", document);
}
