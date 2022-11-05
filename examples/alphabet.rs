extern crate arguments;
extern crate font;

use font::Font;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let arguments = std::env::args();
    let arguments = arguments::parse(arguments).unwrap();
    let path = match arguments.get::<String>("path") {
        Some(path) => path,
        _ => {
            println!("Error: --path should be given.");
            return;
        }
    };
    list(Path::new(&path), process).unwrap();
}

fn list<F>(path: &Path, callback: F) -> io::Result<()>
where
    F: Fn(&Path) -> io::Result<()> + Copy,
{
    if !path.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            list(&path, callback)?;
            continue;
        }
        match path.extension().and_then(|extension| extension.to_str()) {
            Some("otf") => callback(&path)?,
            Some("ttf") => callback(&path)?,
            _ => {}
        }
    }
    Ok(())
}

fn process(path: &Path) -> io::Result<()> {
    println!("Processing {:?}...", path);
    Font::open(path)?;
    Ok(())
}
