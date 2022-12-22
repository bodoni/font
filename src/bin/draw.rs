extern crate arguments;
extern crate font;
extern crate svg;

mod common;

use font::Font;
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
    let font = Font::open(font).unwrap();
    let glyph = font.draw(character).unwrap().unwrap();
    let (width, height) = (
        glyph.width() + 2.0 * glyph.side_bearings.0,
        font.ascender - font.descender,
    );
    let background = element::Rectangle::new()
        .set("width", width)
        .set("height", height)
        .set("fill", "#eee");
    let transform = format!("translate(0, {}) scale(1, -1)", font.ascender);
    let glyph = common::drawing::draw(&glyph).set("transform", transform);
    let style = element::Style::new("path { fill: black; fill-rule: nonzero }");
    let document = Document::new()
        .set("width", width)
        .set("height", height)
        .add(style)
        .add(background)
        .add(glyph);
    print!("{}", document);
}
