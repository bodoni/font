#[macro_use]
mod support;

mod adobe_blank {
    use crate::support::{setup, Fixture};

    #[test]
    fn characters() {
        let mut file = setup(Fixture::CrimsonText);
        let values = ok!(file[0].characters());
        assert_eq!(
            values,
            [
                13..=13,
                32..=126,
                160..=172,
                174..=328,
                330..=382,
                393..=393,
                416..=417,
                431..=432,
                461..=468,
                470..=476,
                482..=483,
                507..=511,
                536..=539,
                562..=563,
                567..=567,
                598..=598,
                700..=700,
                710..=711,
                728..=733,
                768..=780,
                786..=786,
                803..=803,
                806..=808,
                956..=956,
                7680..=7699,
                7704..=7707,
                7710..=7723,
                7725..=7755,
                7757..=7759,
                7764..=7771,
                7773..=7780,
                7782..=7782,
                7784..=7799,
                7801..=7801,
                7803..=7833,
                7840..=7929,
                8208..=8213,
                8216..=8218,
                8220..=8222,
                8224..=8226,
                8228..=8228,
                8230..=8230,
                8240..=8240,
                8242..=8243,
                8248..=8250,
                8254..=8254,
                8260..=8260,
                8304..=8304,
                8308..=8308,
                8320..=8324,
                8364..=8364,
                8482..=8482,
                8592..=8595,
                8706..=8706,
                8721..=8723,
                8725..=8725,
                8730..=8730,
                8733..=8734,
                8800..=8800,
                8804..=8805,
                9753..=9753,
                10087..=10087,
                63632..=63640,
                63642..=63647,
                64257..=64258,
            ]
        );
    }

    #[test]
    fn draw_a() {
        let font = &mut setup(Fixture::AdobeBlank)[0];
        let glyph = ok!(ok!(font.draw('a')));
        assert_eq!(glyph.len(), 0);
    }
}

mod adobe_vf_prototype {
    use std::collections::HashMap;

    use font::axes::Type;
    use font::opentype::truetype::Tag;

    use crate::support::{setup, Fixture};

    #[test]
    fn axes() {
        let mut file = setup(Fixture::AdobeVFPrototype);

        let values = ok!(file[0].axes());
        assert_eq!(values.len(), 5);
        assert!(values[&Type::Italic].range.is_none());
        assert!(values[&Type::Slant].range.is_none());
        assert_eq!(ok!(values[&Type::Weight].range), (200.0, 900.0));
        assert_eq!(values[&Type::Weight].default.round(), 389.0);
        assert!(values[&Type::Width].range.is_none());
        assert_eq!(values[&Type::Width].default, 100.0);

        let value = values[&Type::Other(Tag(*b"CNTR"))];
        let values: HashMap<_, _> = ok!(file[0].names())
            .iter()
            .map(|((name_id, _), value)| (name_id, value.unwrap()))
            .collect();
        assert_eq!(values[&value.name_id], "Contrast");
    }
}

mod crimson_text {
    use std::collections::{BTreeMap, BTreeSet};

    use font::axes::Type;

    use crate::support::{setup, Fixture};

    #[test]
    fn axes() {
        let mut file = setup(Fixture::CrimsonText);
        let values = ok!(file[0].axes());
        assert_eq!(values[&Type::Italic].default, 0.0);
    }

    #[test]
    fn features() {
        use font::features::{Language, Script, Type as Feature};

        let mut file = setup(Fixture::CrimsonText);
        let mut values: BTreeMap<_, BTreeSet<Feature>> = Default::default();
        for (feature, value) in ok!(file[0].features()).into_iter() {
            values.entry(value.scripts).or_default().insert(feature);
        }
        let values = values
            .into_iter()
            .map(|(scripts, features)| {
                (
                    features.into_iter().collect::<Vec<_>>(),
                    scripts
                        .into_iter()
                        .map(|(script, languages)| {
                            (script, languages.into_iter().collect::<Vec<_>>())
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            values,
            [
                (
                    vec![
                        Feature::Kerning,
                        Feature::MarkPositioning,
                        Feature::MarkToMarkPositioning
                    ],
                    vec![(Script::Default, vec![None]), (Script::Latin, vec![None])],
                ),
                (
                    vec![
                        Feature::CaseSensitiveForms,
                        Feature::GlyphCompositionDecomposition,
                        Feature::DiscretionaryLigatures,
                        Feature::Denominators,
                        Feature::Fractions,
                        Feature::StandardLigatures,
                        Feature::Numerators,
                        Feature::ScientificInferiors,
                        Feature::Subscript,
                        Feature::Superscript,
                        Feature::SlashedZero
                    ],
                    vec![
                        (Script::Default, vec![None]),
                        (
                            Script::Latin,
                            vec![
                                None,
                                Some(Language::Azerbaijani),
                                Some(Language::Catalan),
                                Some(Language::CrimeanTatar),
                                Some(Language::Kazakh),
                                Some(Language::Moldavian),
                                Some(Language::Romanian),
                                Some(Language::Tatar),
                                Some(Language::Turkish)
                            ]
                        )
                    ],
                ),
                (
                    vec![Feature::LocalizedForms],
                    vec![(
                        Script::Latin,
                        vec![
                            Some(Language::Azerbaijani),
                            Some(Language::Catalan),
                            Some(Language::CrimeanTatar),
                            Some(Language::Kazakh),
                            Some(Language::Moldavian),
                            Some(Language::Romanian),
                            Some(Language::Tatar),
                            Some(Language::Turkish)
                        ]
                    )],
                )
            ]
        );
    }

    #[test]
    fn metrics() {
        let mut file = setup(Fixture::CrimsonText);
        let metrics = ok!(file[0].metrics());
        assert_eq!(metrics.granularity, 1024.0);
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
    fn draw_i() {
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

mod noto_color_emoji {
    use crate::support::{setup, Fixture};

    #[test]
    fn palettes() {
        let font = &mut setup(Fixture::NotoColorEmoji)[0];
        let table = ok!(ok!(font.palettes()));
        let values = table
            .iter()
            .map(|palette| {
                palette
                    .map(|color| {
                        format!(
                            "#{:02x}{:02x}{:02x}{:02x}",
                            color.red, color.green, color.blue, color.alpha,
                        )
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].len(), 5895);
        assert_eq!(
            &values[0][..10],
            &[
                "#000000ff",
                "#00000dff",
                "#000066ff",
                "#000088ff",
                "#00008bff",
                "#000095ff",
                "#0000ffff",
                "#000101ff",
                "#000200ff",
                "#000202ff",
            ],
        );
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
    use font::axes::Type;

    use crate::support::{setup, trace, Fixture};

    #[test]
    fn axes() {
        let mut file = setup(Fixture::OpenSans);
        let values = ok!(file[0].axes());
        assert_eq!(values[&Type::Slant].default, -12.0);
    }

    #[test]
    fn metrics() {
        let mut file = setup(Fixture::OpenSans);
        let values = ok!(file[0].metrics());
        assert_eq!(values.granularity, 2048.0);
        assert_eq!(values.clipping_ascender, 2189.0);
        assert_eq!(values.ascender, 1567.0);
        assert_eq!(values.cap_height, 1462.0);
        assert_eq!(values.x_height, 1096.0);
        assert_eq!(values.baseline, 0.0);
        assert_eq!(values.descender, -492.0);
        assert_eq!(values.clipping_descender, -600.0);
        assert_eq!(values.line_gap, 132.0);
    }

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

mod zen_loop {
    use crate::support::{setup, trace, Fixture};

    #[test]
    fn draw_d() {
        let font = &mut setup(Fixture::ZenLoop)[0];
        let glyph = ok!(ok!(font.draw('d')));
        assert_eq!(glyph.len(), 2);
        #[rustfmt::skip]
        assert_eq!(&trace(&glyph), &vec![
            (228.0,   0.0),
            (238.0,  10.0),
            (238.0, 703.0),
            (228.0, 713.0),
            (217.0, 703.0),
            (217.0, 438.0),
            (145.0, 482.0),
            ( 86.5, 450.5),
            ( 51.5, 364.5),
            ( 40.0, 239.0),
            ( 51.5, 113.0),
            ( 86.5,  26.5),
            (145.0,  -5.0),
            (217.0,  39.0),
            (217.0,  10.0),
            (228.0,   0.0),

            (145.0, 461.0),
            (184.0, 446.0),
            (217.0, 404.0),
            (217.0,  74.0),
            (184.0,  31.0),
            (145.0,  16.0),
            ( 98.5,  45.0),
            ( 71.0, 124.5),
            ( 62.0, 239.0),
            ( 71.0, 353.0),
            ( 98.5, 432.0),
            (145.0, 461.0),
        ]);
    }
}
