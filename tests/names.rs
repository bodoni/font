#[macro_use]
mod support;

mod noto_color_emoji {
    use font::opentype::truetype::tables::names::Names;

    use crate::support::{setup, Fixture};

    #[test]
    fn read() {
        let font = &mut setup(Fixture::NotoColorEmoji)[0];
        let table = ok!(font.names());
        test(&table.borrow());
    }

    #[test]
    fn write() {
        use std::fs::File;
        use std::io::Cursor;

        use font::formats::opentype::{read, write, Disposition};
        use font::Case;

        let path = crate::support::path(Fixture::NotoColorEmoji);
        let file = ok!(File::open(path));
        let mut font = ok!(ok!(read(file)).pop());

        let table = ok!(font.names());
        let other = {
            let table = table.borrow();
            let records = table.iter().map(|(id, value)| (id, ok!(value)));
            let language_tags = table.language_tags().map(Option::unwrap);
            ok!(Names::from_iter(
                records,
                language_tags,
                &mut Default::default(),
            ))
        };
        *table.borrow_mut() = other;

        let mut cursor: Cursor<Vec<u8>> = Cursor::new(vec![]);
        ok!(write(font, &mut cursor, |tag| {
            if tag != b"name" {
                Disposition::Retain
            } else {
                Disposition::Update
            }
        }));

        let cursor = Cursor::new(cursor.into_inner());
        let mut font = ok!(ok!(read(cursor)).pop());
        let table = ok!(font.names());
        test(&table.borrow());
    }

    fn test(table: &Names) {
        let records = table.iter().collect::<Vec<_>>();
        let values = records
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
}
