# Font [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a font toolbox.

## [Documentation][doc]

## Example

```rust
use font::{File, Segment};

let path = "SourceSerifPro-Regular.otf";
let file = File::open(path).unwrap();
let font = &file[0];

for contour in font.draw('&').unwrap().unwrap().iter() {
    for segment in contour.iter() {
        match segment {
            &Segment::Linear(..) => println!("Line!"),
            &Segment::Quadratic(..) => println!("Curve!"),
            &Segment::Cubic(..) => println!("Curve!"),
        }
    }
}
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[doc]: https://bodoni.github.io/font
[status-img]: https://travis-ci.org/bodoni/font.svg?branch=master
[status-url]: https://travis-ci.org/bodoni/font
[version-img]: https://img.shields.io/crates/v/font.svg
[version-url]: https://crates.io/crates/font
