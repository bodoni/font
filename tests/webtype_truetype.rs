#[macro_use]
mod support;

mod noto_naskh_arabic {
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
}
