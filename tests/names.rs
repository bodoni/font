#[macro_use]
mod support;

use crate::support::{setup, Fixture};

#[test]
fn noto_color_emoji() {
    let font = &mut setup(Fixture::NotoColorEmoji)[0];
    let table = ok!(font.names());
    let names = table.borrow().iter().collect::<Vec<_>>();
    let values = names
        .iter()
        .map(|(_, value)| ok!(value.as_deref()))
        .collect::<Vec<_>>();
    assert_eq!(
        values,
        &[
            "Copyright 2022 Google Inc.",
            "Noto Color Emoji",
            "Regular",
            "Noto Color Emoji",
            "Noto Color Emoji",
            "Version 2.042;GOOG;noto-emoji:20231129:7f49a00d523ae5f94e52fd9f9a39bac9cf65f958",
            "NotoColorEmoji",
            "Noto is a trademark of Google Inc.",
            "Google, Inc.",
            "Google, Inc.",
            "Color emoji font using COLRv1.",
            "https://github.com/googlefonts/noto-emoji",
            "https://github.com/googlefonts/noto-emoji",
            concat!(
                "This Font Software is licensed under the SIL Open Font License, Version 1.1. ",
                r#"This Font Software is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR "#,
                "CONDITIONS OF ANY KIND, either express or implied. See the SIL Open Font License ",
                "for the specific language, permissions and limitations governing your use of ",
                "this Font Software.",
            ),
            "http://scripts.sil.org/OFL",
        ],
    );
}
