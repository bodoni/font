#[macro_use]
mod support;

mod noto_naskh_arabic {
    use crate::support::{setup, Fixture};

    #[test]
    fn metrics() {
        let mut file = setup(Fixture::NotoNaskhArabic);
        let metrics = ok!(file[0].metrics());
        assert_eq!(metrics.units_per_em, 2048.0);
    }

    #[test]
    fn properties() {
        let mut file = setup(Fixture::NotoNaskhArabic);
        let properties = ok!(file[0].properties());
        assert_eq!(properties.vendor_id, "GOOG");
    }
}
