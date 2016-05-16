# Font [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a font toolbox.

## [Documentation][doc]

## Example

```rust
use font::File;
use font::Operation::*;

let path = "SourceSerifPro-Regular.otf";
let file = File::open(path).unwrap();
let glyph = file[0].draw('&').unwrap().unwrap();

for operation in glyph.iter() {
    match operation {
        &CurveTo(..) => println!("Curve!"),
        &LineTo(..) => println!("Line!"),
        &MoveTo(..) => println!("Move!"),
    }
}
```

## Contribution

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[version-img]: https://img.shields.io/crates/v/font.svg
[version-url]: https://crates.io/crates/font
[status-img]: https://travis-ci.org/bodoni/font.svg?branch=master
[status-url]: https://travis-ci.org/bodoni/font
[doc]: https://bodoni.github.io/font
