#[macro_use]
mod support;

mod noto_naskh_arabic {
    use font::opentype::truetype::Tag;

    use crate::support::{setup, Fixture};

    #[test]
    fn axes() {
        let mut file = setup(Fixture::NotoNaskhArabic);
        let values = ok!(file[0].axes());
        assert!(values.values().all(|value| value.range.is_none()));
    }

    #[test]
    fn metrics() {
        let mut file = setup(Fixture::NotoNaskhArabic);
        let values = ok!(file[0].metrics());
        assert_eq!(values.granularity, 2048.0);
    }

    #[test]
    fn tables() {
        let mut file = setup(Fixture::NotoNaskhArabic);
        let values = ok!(file[0].tables());
        assert_eq!(
            values,
            vec![
                Tag(*b"GDEF"),
                Tag(*b"GPOS"),
                Tag(*b"GSUB"),
                Tag(*b"OS/2"),
                Tag(*b"cmap"),
                Tag(*b"cvt "),
                Tag(*b"fpgm"),
                Tag(*b"gasp"),
                Tag(*b"glyf"),
                Tag(*b"loca"),
                Tag(*b"head"),
                Tag(*b"hhea"),
                Tag(*b"hmtx"),
                Tag(*b"maxp"),
                Tag(*b"name"),
                Tag(*b"post"),
                Tag(*b"prep"),
            ],
        );
    }
}
