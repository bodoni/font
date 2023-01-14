extern crate font;

#[macro_use]
mod support;

mod adobe_blank {
    use crate::support::{setup, Fixture};

    #[test]
    fn draw_a() {
        let font = &mut setup(Fixture::AdobeBlank)[0];
        let glyph = ok!(ok!(font.draw('a')));
        assert_eq!(glyph.len(), 0);
    }
}

mod crimson_text {
    use crate::support::{setup, Fixture};

    #[test]
    fn open() {
        let mut file = setup(Fixture::CrimsonText);
        let metrics = ok!(file[0].metrics());
        assert_eq!(metrics.units_per_em, 1024.0);
        assert_eq!(metrics.clipping_ascender, 1106.0);
        assert_eq!(metrics.ascender, 972.0);
        assert_eq!(metrics.cap_height, 656.0);
        assert_eq!(metrics.x_height, 430.0);
        assert_eq!(metrics.baseline, 0.0);
        assert_eq!(metrics.descender, -359.0);
        assert_eq!(metrics.clipping_descender, -297.0);
        assert_eq!(metrics.line_gap, 0.0);
    }
}

mod monte_carlo {
    use crate::support::{setup, trace, Fixture};

    #[test]
    fn draw_letters() {
        let font = &mut setup(Fixture::MonteCarlo)[0];
        let glyph = ok!(ok!(font.draw('i')));
        assert_eq!(glyph.len(), 2);
        #[rustfmt::skip]
        assert_eq!(&trace(&glyph), &vec![
            (102.0,  -6.0),
            ( 61.0,  12.5),
            ( 48.0,  58.0),
            ( 49.5,  74.5),
            ( 53.0,  92.0),
            ( 87.5, 181.5),
            (133.0, 270.0),
            (157.0, 301.5),
            (181.0, 315.5),
            (212.0, 326.0),
            (222.0, 329.0),
            (233.0, 331.0),
            (192.5, 283.0),
            (148.0, 206.0),
            (122.5, 148.0),
            (101.0,  88.0),
            ( 91.0,  44.0),
            ( 96.0,  17.0),
            (112.0,   9.0),
            (144.5,  21.0),
            (172.0,  46.0),
            (213.5, 101.0),
            (252.0, 165.0),
            (257.0, 169.0),
            (261.0, 163.0),
            (259.0, 155.0),
            (221.0,  89.0),
            (172.0,  27.0),
            (141.5,   4.0),
            (102.0,  -6.0),

            (224.2497 , 387.49524),
            (205.34991, 395.37015),
            (198.0    , 413.74493),
            (209.54987, 439.46964),
            (234.74957, 451.5445 ),
            (253.64935, 444.19458),
            (260.99927, 425.2948 ),
            (249.9744 , 399.5701 ),
            (224.2497 , 387.49524),
        ]);
    }
}

mod numans {
    use crate::support::{setup, trace, Fixture};

    #[test]
    fn draw_a() {
        let font = &mut setup(Fixture::Numans)[0];
        let glyph = ok!(ok!(font.draw('a')));
        assert_eq!(glyph.len(), 2);
        #[rustfmt::skip]
        assert_eq!(&trace(&glyph), &vec![
            ( 238.5,   62.5),
            ( 123.0,  317.5),
            ( 241.5,  572.5),
            ( 573.0,  666.0),
            ( 922.0,  666.0),
            ( 922.0,  676.0),
            ( 614.0,  983.0),
            ( 450.5,  937.0),
            ( 358.0,  819.0),
            ( 334.0,  799.0),
            ( 205.0,  799.0),
            ( 184.0,  819.0),
            ( 316.0, 1042.0),
            ( 614.0, 1126.0),
            ( 979.0, 1008.5),
            (1106.0,  676.0),
            (1106.0,   20.0),
            (1085.0,    0.0),
            ( 973.0,    0.0),
            ( 950.0,   20.0),
            ( 932.0,  133.0),
            ( 879.0,   80.5),
            ( 797.0,   26.0),
            ( 553.0,  -31.0),
            ( 238.5,   62.5),

            ( 307.0,  317.0),
            ( 373.5,  165.0),
            ( 563.0,  113.0),
            ( 690.5,  135.5),
            ( 791.5,  188.5),
            ( 862.0,  251.0),
            ( 907.5,  304.5),
            ( 922.0,  328.0),
            ( 922.0,  522.0),
            ( 573.0,  522.0),
            ( 307.0,  317.0),
        ]);
    }
}

mod open_sans {
    use crate::support::{setup, trace, Fixture};

    #[test]
    fn draw_a_ring() {
        let font = &mut setup(Fixture::OpenSans)[0];
        let glyph = ok!(ok!(font.draw('å')));
        assert_eq!(glyph.len(), 4);
        #[rustfmt::skip]
        assert_eq!(&trace(&glyph), &vec![
            ( 639.0, 1116.0),
            ( 803.0, 1066.5),
            ( 915.0,  924.0),
            ( 926.0,  924.0),
            ( 993.0, 1096.0),
            (1120.0, 1096.0),
            ( 887.0,    0.0),
            ( 754.0,    0.0),
            ( 780.0,  209.0),
            ( 772.0,  209.0),
            ( 395.0,  -20.0),
            ( 177.0,   79.0),
            (  98.0,  350.0),
            ( 169.0,  736.0),
            ( 365.0, 1015.0),
            ( 639.0, 1116.0),
            ( 449.0,  119.0),
            ( 642.0,  211.5),
            ( 798.0,  452.5),
            ( 858.0,  750.0),
            ( 802.0,  914.0),
            ( 655.0,  975.0),
            ( 461.5,  889.0),
            ( 321.0,  656.0),
            ( 270.0,  346.0),
            ( 317.0,  175.5),
            ( 449.0,  119.0),

            ( 951.0, 1456.0),
            ( 891.0, 1299.0),
            ( 732.0, 1241.0),
            ( 572.0, 1298.5),
            ( 513.0, 1454.0),
            ( 575.0, 1606.5),
            ( 732.0, 1665.0),
            ( 892.0, 1608.0),
            ( 951.0, 1456.0),
            ( 847.0, 1454.0),
            ( 815.0, 1538.0),
            ( 732.0, 1569.0),
            ( 650.0, 1538.0),
            ( 617.0, 1454.0),
            ( 646.5, 1369.5),
            ( 732.0, 1339.0),
            ( 815.0, 1369.5),
            ( 847.0, 1454.0),
        ]);
    }

    #[test]
    fn draw_copyright() {
        let font = &mut setup(Fixture::OpenSans)[0];
        let glyph = ok!(ok!(font.draw('©')));
        assert_eq!(glyph.bounding_box, (139.0, -20.0, 1642.0, 1483.0));
        assert_eq!(glyph.side_bearings, (139.0, 62.0));
    }

    #[test]
    fn draw_from_a_to_z() {
        let font = &mut setup(Fixture::OpenSans)[0];
        for code in b'a'..(b'z' + 1) {
            ok!(ok!(font.draw(code as char)));
        }
    }

    #[test]
    fn draw_o() {
        let font = &mut setup(Fixture::OpenSans)[0];
        let glyph = ok!(ok!(font.draw('o')));
        assert_eq!(glyph.len(), 2);
        #[rustfmt::skip]
        assert_eq!(&trace(&glyph), &vec![
            ( 643.0, 1110.0),
            ( 943.0,  997.5),
            (1053.0,  688.0),
            ( 981.0,  333.0),
            ( 786.0,   75.0),
            ( 508.0,  -16.0),
            ( 207.0,   97.0),
            (  98.0,  406.0),
            ( 171.0,  763.5),
            ( 368.0, 1020.5),
            ( 643.0, 1110.0),

            ( 879.0,  711.0),
            ( 816.5,  897.5),
            ( 647.0,  969.0),
            ( 451.5,  895.0),
            ( 317.5,  689.5),
            ( 270.0,  397.0),
            ( 333.5,  194.5),
            ( 516.0,  123.0),
            ( 703.0,  196.0),
            ( 832.5,  403.5),
            ( 879.0,  711.0),
        ]);
    }

    #[test]
    fn draw_slash() {
        let font = &mut setup(Fixture::OpenSans)[0];
        let glyph = ok!(ok!(font.draw('/')));
        assert_eq!(glyph.len(), 1);
        #[rustfmt::skip]
        assert_eq!(&trace(&glyph), &vec![
            (893.0, 1462.0),
            ( 80.0,    0.0),
            (-94.0,    0.0),
            (719.0, 1462.0),
            (893.0, 1462.0),
        ]);
    }

    #[test]
    fn open() {
        let mut file = setup(Fixture::OpenSans);
        let metrics = ok!(file[0].metrics());
        assert_eq!(metrics.units_per_em, 2048.0);
        assert_eq!(metrics.clipping_ascender, 2189.0);
        assert_eq!(metrics.ascender, 1567.0);
        assert_eq!(metrics.cap_height, 1462.0);
        assert_eq!(metrics.x_height, 1096.0);
        assert_eq!(metrics.baseline, 0.0);
        assert_eq!(metrics.descender, -492.0);
        assert_eq!(metrics.clipping_descender, -600.0);
        assert_eq!(metrics.line_gap, 132.0);
    }
}

mod vesper_libre {
    use crate::support::{setup, trace, Fixture};

    #[test]
    fn draw_a() {
        let font = &mut setup(Fixture::VesperLibre)[0];
        let glyph = ok!(ok!(font.draw('a')));
        assert_eq!(glyph.len(), 2);
        #[rustfmt::skip]
        assert_eq!(&trace(&glyph), &vec![
            ( 68.0, 241.0),
            (102.0, 389.0),
            (218.0, 475.0),
            (364.5, 512.0),
            (531.0, 524.0),
            (611.0, 521.0),
            (611.0, 575.0),
            (590.5, 742.5),
            (510.0, 851.0),
            (454.5, 869.0),
            (382.0, 875.0),
            (315.0, 868.0),
            (281.0, 686.0),
            (220.0, 671.0),
            (166.5, 686.0),
            (121.0, 726.0),
            (103.0, 779.0),
            (187.0, 879.0),
            (305.5, 928.0),
            (436.5, 962.5),
            (530.0, 975.0),
            (729.0, 894.0),
            (790.0, 674.0),
            (790.0, 192.0),
            (869.0, 129.5),
            (963.0,  93.0),
            (951.0,  35.0),
            (835.5,  -6.5),
            (723.0, -25.0),
            (687.5,  59.0),
            (638.0, 132.0),
            (562.0,  62.5),
            (443.5,   0.0),
            (326.0, -25.0),
            (180.0,  15.5),
            ( 95.0, 117.0),
            ( 68.0, 241.0),

            (356.0, 112.0),
            (390.0, 106.0),
            (501.5, 124.5),
            (611.0, 165.0),
            (611.0, 458.0),
            (440.0, 433.0),
            (327.0, 394.5),
            (271.0, 357.0),
            (252.0, 291.0),
            (266.5, 219.0),
            (305.0, 151.5),
            (356.0, 112.0),
        ]);
    }

    #[test]
    fn draw_ellipsis() {
        let font = &mut setup(Fixture::VesperLibre)[0];
        let glyph = ok!(ok!(font.draw('…')));
        assert_eq!(glyph.len(), 3);
        #[rustfmt::skip]
        assert_eq!(&trace(&glyph), &vec![
            (358.0,   1.0),
            (298.0, -10.0),
            (216.0,  -0.5),
            (163.0,  25.0),
            (150.0,  85.0),
            (161.0, 170.0),
            (187.0, 223.0),
            (246.0, 235.0),
            (330.0, 224.0),
            (384.0, 198.0),
            (396.0, 143.0),
            (384.5,  56.5),
            (358.0,   1.0),

            (806.0,   1.0),
            (746.0, -10.0),
            (664.0,  -0.5),
            (611.0,  25.0),
            (598.0,  85.0),
            (609.0, 170.0),
            (635.0, 223.0),
            (694.0, 235.0),
            (778.0, 224.0),
            (832.0, 198.0),
            (844.0, 143.0),
            (832.5,  56.5),
            (806.0,   1.0),

            (1255.0,   1.0),
            (1195.0, -10.0),
            (1113.0,  -0.5),
            (1060.0,  25.0),
            (1047.0,  85.0),
            (1058.0, 170.0),
            (1084.0, 223.0),
            (1143.0, 235.0),
            (1227.0, 224.0),
            (1281.0, 198.0),
            (1293.0, 143.0),
            (1281.5,  56.5),
            (1255.0,   1.0),
        ]);
    }
}
