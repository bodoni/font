#[macro_use]
mod support;

mod source_serif {
    use crate::support::{setup, trace, Fixture};

    #[test]
    fn draw_from_a_to_z() {
        let font = &mut setup(Fixture::SourceSerif)[0];
        for code in b'a'..=b'z' {
            ok!(ok!(font.draw(code as char)));
        }
    }

    #[test]
    fn draw_o() {
        let font = &mut setup(Fixture::SourceSerif)[0];
        let glyph = ok!(ok!(font.draw('o')));
        assert_eq!(glyph.len(), 2);
        #[rustfmt::skip]
        assert_eq!(&trace(&glyph), &vec![
            (274.0, 445.0),
            (409.0, 236.0),
            (274.0,  30.0),
            (140.0, 236.0),
            (274.0, 445.0),

            (274.0, 491.0),
            ( 45.0, 237.0),
            (274.0, -15.0),
            (504.0, 237.0),
            (274.0, 491.0),
        ]);
    }

    #[test]
    fn draw_r() {
        let font = &mut setup(Fixture::SourceSerif)[0];
        let glyph = ok!(ok!(font.draw('r')));
        assert_eq!(glyph.bounding_box, (34.0, 0.0, 412.0, 491.0));
        assert_eq!(glyph.side_bearings, (34.0, 11.0));
    }

    #[test]
    fn metrics() {
        let mut file = setup(Fixture::SourceSerif);
        let metrics = ok!(file[0].metrics());
        assert_eq!(metrics.units_per_em, 1000.0);
        assert_eq!(metrics.clipping_ascender, 918.0);
        assert_eq!(metrics.ascender, 730.0);
        assert_eq!(metrics.cap_height, 670.0);
        assert_eq!(metrics.x_height, 475.0);
        assert_eq!(metrics.baseline, 0.0);
        assert_eq!(metrics.descender, -270.0);
        assert_eq!(metrics.clipping_descender, -335.0);
        assert_eq!(metrics.line_gap, 0.0);
    }

    #[test]
    fn properties() {
        let mut file = setup(Fixture::SourceSerif);
        let properties = ok!(file[0].properties());
        assert!(properties.cubic);
    }
}
