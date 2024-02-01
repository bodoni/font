#[macro_use]
mod support;

use crate::support::{setup, Fixture};

#[test]
fn noto_color_emoji() {
    let font = &mut setup(Fixture::NotoColorEmoji)[0];
    let table = ok!(ok!(font.palettes()));
    let values = table
        .iter()
        .map(|palette| {
            palette
                .map(|color| {
                    format!(
                        "#{:02x}{:02x}{:02x}{:02x}",
                        color.red, color.green, color.blue, color.alpha,
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    assert_eq!(values.len(), 1);
    assert_eq!(values[0].len(), 5895);
    assert_eq!(
        &values[0][..10],
        &[
            "#000000ff",
            "#00000dff",
            "#000066ff",
            "#000088ff",
            "#00008bff",
            "#000095ff",
            "#0000ffff",
            "#000101ff",
            "#000200ff",
            "#000202ff",
        ],
    );
}
