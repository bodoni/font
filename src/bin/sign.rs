extern crate arguments;
extern crate font;
extern crate svg;
extern crate walkdir;

mod support;

use std::io::Result;
use std::path::{Path, PathBuf};

use font::Font;
use svg::node::element;
use svg::Document;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let path: PathBuf = match arguments.get::<String>("path") {
        Some(path) => path.into(),
        _ => {
            println!("Error: --path should be given.");
            return;
        }
    };
    let ignores = arguments.get_all::<String>("ignore").unwrap_or(vec![]);
    let workers = arguments.get::<usize>("workers").unwrap_or(1);
    let values = support::scanning::scan(&path, process, workers);
    let (succeeded, other): (Vec<_>, Vec<_>) =
        values.into_iter().partition(|(_, result)| result.is_ok());
    let (found, missing): (Vec<_>, Vec<_>) = succeeded
        .into_iter()
        .partition(|(_, result)| result.as_ref().unwrap().is_some());
    let (ignored, failed): (Vec<_>, Vec<_>) = other.into_iter().partition(|(path, _)| {
        let path = path.to_str().unwrap();
        ignores.iter().any(|name| path.contains(name))
    });
    println!("Found: {}", found.len());
    println!("Missing: {}", missing.len());
    println!("Failed: {}", failed.len());
    for (path, result) in failed.iter() {
        println!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    println!("Ignored: {}", ignored.len());
    for (path, result) in ignored.iter() {
        println!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    assert_eq!(failed.len(), 0);
}

fn process(path: PathBuf) -> (PathBuf, Result<Option<()>>) {
    let result = match draw(&path) {
        Ok(option) => {
            println!("[success] {:?}", path);
            Ok(option)
        }
        Err(error) => {
            println!("[failure] {:?} ({:?})", path, error);
            Err(error)
        }
    };
    (path, result)
}

fn draw(path: &Path) -> Result<Option<()>> {
    let font = Font::open(path)?;
    let glyph = match font.draw('a')? {
        Some(glyph) => glyph,
        _ => return Ok(None),
    };
    let (width, height) = (glyph.advance_width, font.ascender - font.descender);
    let background = element::Rectangle::new()
        .set("width", width)
        .set("height", height)
        .set("fill", "#eee");
    let transform = format!("translate(0, {}) scale(1, -1)", font.ascender);
    let glyph = support::drawing::draw(&glyph).set("transform", transform);
    let style = element::Style::new("path { fill: black; fill-rule: nonzero }");
    let _ = Document::new()
        .set("width", width)
        .set("height", height)
        .add(style)
        .add(background)
        .add(glyph);
    Ok(Some(()))
}
