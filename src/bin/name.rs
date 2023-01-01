extern crate arguments;
extern crate font;

use font::File;

fn main() {
    let arguments = arguments::parse(std::env::args()).unwrap();
    let font = match arguments.get::<String>("font") {
        Some(font) => font,
        _ => {
            println!("Error: --font should be given.");
            return;
        }
    };
    let File { mut fonts } = File::open(font).unwrap();
    for ((name_id, language_tag), value) in fonts[0].names().unwrap().iter() {
        let name_id = format!("{:?}", name_id);
        let language_tag = language_tag.as_deref().unwrap_or("--");
        let value = truncate(value.as_deref().unwrap_or("--"));
        println!("{: <25} {: <5} {}", name_id, language_tag, value);
    }
}

fn truncate(string: &str) -> String {
    const MAX: usize = 50;
    let count = string.chars().count();
    let mut string = match string.char_indices().nth(MAX) {
        None => string.to_owned(),
        Some((index, _)) => string[..index].to_owned(),
    };
    if count > MAX {
        string.push_str("…");
    }
    string
}
