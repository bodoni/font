extern crate arguments;
extern crate font;
extern crate svg;
extern crate walkdir;

mod support;

use std::io::Result;
use std::path::{Path, PathBuf};

use font::Font;
use svg::node::element;
use svg::{Document, Node};

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let input: PathBuf = match arguments.get::<String>("input") {
        Some(input) => input.into(),
        _ => {
            println!("Error: --input should be given.");
            return;
        }
    };
    let output: PathBuf = match arguments.get::<String>("output") {
        Some(output) => output.into(),
        _ => {
            println!("Error: --output should be given.");
            return;
        }
    };
    let characters: Vec<_> = match arguments.get::<String>("characters") {
        Some(characters) => characters.chars().collect(),
        // Reference:
        // https://youtu.be/_zmodHxs1XY?t=1531
        _ => ['a', 'n', 'o', 'p'].into(),
    };
    let ignores = arguments.get_all::<String>("ignore").unwrap_or(vec![]);
    let workers = arguments.get::<usize>("workers").unwrap_or(1);
    let values = support::scanning::scan(&input, process, (characters, output), workers);
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

fn process(
    path: PathBuf,
    (characters, output): (Vec<char>, PathBuf),
) -> (PathBuf, Result<Option<()>>) {
    let result = match draw(&path, &characters) {
        Ok(Some(document)) => {
            let path = output.join(path.file_stem().unwrap()).with_extension("svg");
            match svg::save(&path, &document) {
                Ok(_) => {
                    println!("[success] {:?}", path);
                    Ok(Some(()))
                }
                Err(error) => {
                    println!("[failure] {:?} ({:?})", path, error);
                    Err(error)
                }
            }
        }
        Ok(None) => {
            println!("[success] {:?}", path);
            Ok(None)
        }
        Err(error) => {
            println!("[failure] {:?} ({:?})", path, error);
            Err(error)
        }
    };
    (path, result)
}

fn draw(path: &Path, characters: &[char]) -> Result<Option<Document>> {
    const SIZE: usize = 512;
    let style = element::Style::new("path { fill: black; fill-rule: nonzero }");
    let background = element::Rectangle::new()
        .set("width", SIZE)
        .set("height", SIZE)
        .set("fill", "#eee");
    let mut document = Document::new()
        .set("width", SIZE)
        .set("height", SIZE)
        .add(style)
        .add(background);
    let font = Font::open(path)?;
    let columns = (characters.len() as f32).sqrt().ceil() as usize;
    let offset = SIZE as f32 / columns as f32;
    let scale = SIZE as f32 / columns as f32 / font.units_per_em;
    for (index, character) in characters.iter().enumerate() {
        let glyph = match font.draw(*character)? {
            Some(glyph) => glyph,
            _ => return Ok(None),
        };
        let i = index % columns;
        let j = index / columns;
        let transform = format!(
            "translate({:.2}, {:.2}) scale({:.2}) translate({:.2}, {:.2}) scale(1, -1)",
            i as f32 * offset,
            j as f32 * offset,
            scale,
            (font.units_per_em - glyph.advance_width) / 2.0,
            font.ascender,
        );
        let mut glyph = support::drawing::draw(&glyph);
        glyph.assign("transform", transform);
        document.append(glyph);
    }
    Ok(Some(document))
}
