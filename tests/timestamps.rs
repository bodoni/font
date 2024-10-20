#[macro_use]
mod support;

use font::Timestamps;

use crate::support::{setup, Fixture};

#[test]
fn noto_naskh_arabic() {
    let mut file = setup(Fixture::NotoNaskhArabic);
    let values = ok!(file[0].timestamps());
    assert_eq!(
        values,
        Timestamps {
            creation: 3442496726,
            modification: 3525888992,
        }
    );
}
