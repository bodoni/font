extern crate arguments;
extern crate font;
extern crate walkdir;

mod support;

use std::io::Result;
use std::path::PathBuf;

use font::Font;

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
    let (ignored, failed): (Vec<_>, Vec<_>) = other.into_iter().partition(|(path, _)| {
        let path = path.to_str().unwrap();
        ignores.iter().any(|name| path.contains(name))
    });
    println!("Succeeded: {}", succeeded.len());
    println!("Ignored: {}", ignored.len());
    for (path, result) in ignored.iter() {
        println!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    println!("Failed: {}", failed.len());
    for (path, result) in failed.iter() {
        println!("{:?}: {}", path, result.as_ref().err().unwrap());
    }
    assert_eq!(failed.len(), 0);
}

fn process(path: PathBuf) -> (PathBuf, Result<()>) {
    let result = match Font::open(&path) {
        Ok(_) => {
            println!("[success] {:?}", path);
            Ok(())
        }
        Err(error) => {
            println!("[failure] {:?} ({:?})", path, error);
            Err(error)
        }
    };
    (path, result)
}
