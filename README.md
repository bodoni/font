# Font [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a builder and parser for fonts.

## [Example]

```rust
use font::glyph::Segment;
use font::File;

macro_rules! ok(($result:expr) => ($result.unwrap()));

let path = "SourceSerifPro-Regular.otf";
let File { mut fonts } = ok!(File::open(path));
let glyph = ok!(ok!(fonts[0].glyph('a')));
for contour in glyph.iter() {
    for segment in contour.iter() {
        match segment {
            &Segment::Linear(..) => { /* … */ },
            &Segment::Quadratic(..) => { /* … */ },
            &Segment::Cubic(..) => { /* … */ },
        }
    }
}
```

<div align="center">
  <a href="https://github.com/bodoni/workbench/blob/main/founder/src/drawing.rs">
    <img src="https://raw.githubusercontent.com/bodoni/font/main/assets/SourceSerifPro-Regular.svg">
  </a>
</div>

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[build-img]: https://github.com/bodoni/font/workflows/build/badge.svg
[build-url]: https://github.com/bodoni/font/actions/workflows/build.yml
[documentation-img]: https://docs.rs/font/badge.svg
[documentation-url]: https://docs.rs/font
[package-img]: https://img.shields.io/crates/v/font.svg
[package-url]: https://crates.io/crates/font

[example]: https://github.com/bodoni/founder/blob/main/src/drawing.rs
