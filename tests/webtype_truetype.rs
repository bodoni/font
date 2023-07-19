#[macro_use]
mod support;

mod noto_naskh_arabic {
    use crate::support::{setup, Fixture};

    #[test]
    fn axes() {
        let mut file = setup(Fixture::NotoNaskhArabic);
        let axes = ok!(file[0].axes());
        assert!(axes.values().all(|value| value.range.is_none()));
    }

    #[test]
    fn metrics() {
        let mut file = setup(Fixture::NotoNaskhArabic);
        let metrics = ok!(file[0].metrics());
        assert_eq!(metrics.units_per_em, 2048.0);
    }
}
