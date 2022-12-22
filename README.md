# Font [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides a font toolbox.

## [Example]

```rust
use font::{Font, Segment};

let path = "OpenSans-Italic.ttf";
let font = Font::open(path).unwrap();
let glyph = font.draw('&').unwrap().unwrap();
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
  <a href="https://github.com/bodoni/font/blob/main/src/bin/draw.rs">
    <img src="https://cdn.rawgit.com/bodoni/font/main/assets/draw/OpenSans-Italic.svg">
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

[example]: src/bin/draw.rs
