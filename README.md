# Font [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a font toolbox.

## [Documentation][doc]

## Example

```rust
use font::{File, Segment};

let path = "SourceSerifPro-Regular.otf";
let File { fonts, .. } = File::open(path).unwrap();
let glyph = fonts[0].draw('&').unwrap().unwrap();

for contour in glyph.iter() {
    for segment in contour.iter() {
        match segment {
            &Segment::Linear(..) => {},
            &Segment::Quadratic(..) => {},
            &Segment::Cubic(..) => {},
        }
    }
}
```

<div align="center">
  <a href="https://github.com/bodoni/font/blob/master/examples/glyph.svg">
    <img src="https://cdn.rawgit.com/bodoni/font/master/examples/glyph.svg">
  </a>
</div>

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[doc]: https://bodoni.github.io/font
[status-img]: https://travis-ci.org/bodoni/font.svg?branch=master
[status-url]: https://travis-ci.org/bodoni/font
[version-img]: https://img.shields.io/crates/v/font.svg
[version-url]: https://crates.io/crates/font
