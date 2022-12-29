extern crate arguments;
extern crate font;
extern crate svg;
extern crate walkdir;

mod support;

use std::io::Result;
use std::path::{Path, PathBuf};

use font::File;
use svg::node::{element, Node};

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let input: PathBuf = match arguments.get::<String>("input") {
        Some(input) => input.into(),
        _ => {
            println!("Error: --input should be given.");
            return;
        }
    };
    let output: Option<PathBuf> = match arguments.get::<String>("output") {
        Some(output) => Some(output.into()),
        _ => None,
    };
    let characters: Vec<_> = match arguments.get::<String>("characters") {
        Some(characters) => characters.chars().collect(),
        _ => {
            println!("Error: --characters should be given.");
            return;
        }
    };
    let ignores = arguments.get_all::<String>("ignore").unwrap_or(vec![]);
    let workers = arguments.get::<usize>("workers").unwrap_or(1);
    let values = support::scanning::scan(&input, process, (characters, output), workers);
    let (positives, negatives): (Vec<_>, Vec<_>) =
        values.into_iter().partition(|(_, result)| result.is_ok());
    let (successes, missing): (Vec<_>, Vec<_>) = positives
        .into_iter()
        .partition(|(_, result)| result.as_ref().unwrap().is_some());
    let (ignored, failures): (Vec<_>, Vec<_>) = negatives.into_iter().partition(|(path, _)| {
        let path = path.to_str().unwrap();
        ignores.iter().any(|name| path.contains(name))
    });
    println!("Successes: {}", successes.len());
    println!("Missing: {}", missing.len());
    for (path, _) in missing.iter() {
        println!("{:?}", path);
    }
    println!("Ignored: {}", ignored.len());
    for (path, result) in ignored.iter() {
        println!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    println!("Failures: {}", failures.len());
    for (path, result) in failures.iter() {
        println!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    assert_eq!(failures.len(), 0);
}

fn process(
    path: PathBuf,
    (characters, output): (Vec<char>, Option<PathBuf>),
) -> (PathBuf, Result<Option<()>>) {
    const DOCUMENT_SIZE: f32 = 512.0;
    let group = match draw(&path, &characters, DOCUMENT_SIZE) {
        Ok(None) => {
            println!("[missing] {:?}", path);
            return (path, Ok(None));
        }
        Err(error) => {
            println!("[failure] {:?} ({:?})", path, error);
            return (path, Err(error));
        }
        Ok(Some(group)) => group,
    };
    let style = element::Style::new("path { fill: black; fill-rule: nonzero }");
    let document = element::SVG::new()
        .set("width", DOCUMENT_SIZE)
        .set("height", DOCUMENT_SIZE)
        .add(style)
        .add(group);
    let output = match output {
        None => {
            println!("[success] {:?}", path);
            return (path, Ok(Some(())));
        }
        Some(output) => output,
    };
    let output = output.join(path.file_stem().unwrap()).with_extension("svg");
    match svg::save(&output, &document) {
        Ok(_) => {
            println!("[success] {:?}", path);
            return (path, Ok(Some(())));
        }
        Err(error) => {
            println!("[failure] {:?} ({:?})", path, error);
            return (path, Err(error));
        }
    }
}

fn draw(path: &Path, characters: &[char], document_size: f32) -> Result<Option<element::Group>> {
    let mut group = element::Group::new();
    let File { mut fonts } = File::open(path)?;
    let metrics = fonts[0].metrics()?;
    let glyph_size = metrics.ascender - metrics.descender;
    let columns = (characters.len() as f32).sqrt().ceil() as usize;
    let offset = document_size / columns as f32;
    let scale = document_size / columns as f32 / glyph_size;
    for (index, character) in characters.iter().enumerate() {
        let glyph = match fonts[0].draw(*character)? {
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
            (glyph_size - glyph.advance_width) / 2.0,
            metrics.ascender,
        );
        let mut glyph = support::drawing::draw(&glyph);
        glyph.assign("transform", transform);
        group.append(glyph);
    }
    Ok(Some(group))
}
