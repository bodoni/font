extern crate font;

mod support;

mod source_serif {
    use crate::support::{setup, trace, Fixture};

    #[test]
    fn draw_from_a_to_z() {
        let font = &setup(Fixture::SourceSerif)[0];
        for code in b'a'..=b'z' {
            font.case.draw(code as char).unwrap().unwrap();
        }
    }

    #[test]
    fn draw_o() {
        let font = &setup(Fixture::SourceSerif)[0];
        let glyph = font.case.draw('o').unwrap().unwrap();
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
        let font = &setup(Fixture::SourceSerif)[0];
        let glyph = font.case.draw('r').unwrap().unwrap();
        assert_eq!(glyph.bounding_box, (34.0, 0.0, 412.0, 491.0));
        assert_eq!(glyph.side_bearings, (34.0, 11.0));
    }

    #[test]
    fn open() {
        let file = setup(Fixture::SourceSerif);
        let font = &file[0];
        assert_eq!(font.metrics.units_per_em, 1000.0);
        assert_eq!(font.metrics.clipping_ascender, 918.0);
        assert_eq!(font.metrics.ascender, 730.0);
        assert_eq!(font.metrics.cap_height, 670.0);
        assert_eq!(font.metrics.x_height, 475.0);
        assert_eq!(font.metrics.baseline, 0.0);
        assert_eq!(font.metrics.descender, -270.0);
        assert_eq!(font.metrics.clipping_descender, -335.0);
        assert_eq!(font.metrics.line_gap, 0.0);
    }
}
