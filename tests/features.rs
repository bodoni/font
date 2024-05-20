#[macro_use]
mod support;

use std::collections::BTreeSet;

use font::opentype::truetype::Tag;
use font::Font;

use crate::support::{setup, Fixture};

#[test]
fn crimson_text() {
    let mut file = setup(Fixture::CrimsonText);
    let entries = extract(&mut file[0]);
    let entries = entries
        .iter()
        .map(|(feature, script, language, characters)| {
            (&**feature, &**script, &**language, &**characters)
        })
        .collect::<Vec<_>>();
    #[rustfmt::skip]
    assert_eq!(
        entries,
        [
            ("case", "DFLT", "DFLT", "\u{300}, \u{301}, \u{302}, \u{303}, \u{304}, \u{306}, \u{307}, \u{308}, \u{309}, \u{30a}, \u{30b}, \u{30c}, \u{323}, \u{327}"),
            ("case", "latn", "AZE ", "\u{300}, \u{301}, \u{302}, \u{303}, \u{304}, \u{306}, \u{307}, \u{308}, \u{309}, \u{30a}, \u{30b}, \u{30c}, \u{323}, \u{327}"),
            ("case", "latn", "CAT ", "\u{300}, \u{301}, \u{302}, \u{303}, \u{304}, \u{306}, \u{307}, \u{308}, \u{309}, \u{30a}, \u{30b}, \u{30c}, \u{323}, \u{327}"),
            ("case", "latn", "CRT ", "\u{300}, \u{301}, \u{302}, \u{303}, \u{304}, \u{306}, \u{307}, \u{308}, \u{309}, \u{30a}, \u{30b}, \u{30c}, \u{323}, \u{327}"),
            ("case", "latn", "DFLT", "\u{300}, \u{301}, \u{302}, \u{303}, \u{304}, \u{306}, \u{307}, \u{308}, \u{309}, \u{30a}, \u{30b}, \u{30c}, \u{323}, \u{327}"),
            ("case", "latn", "KAZ ", "\u{300}, \u{301}, \u{302}, \u{303}, \u{304}, \u{306}, \u{307}, \u{308}, \u{309}, \u{30a}, \u{30b}, \u{30c}, \u{323}, \u{327}"),
            ("case", "latn", "MOL ", "\u{300}, \u{301}, \u{302}, \u{303}, \u{304}, \u{306}, \u{307}, \u{308}, \u{309}, \u{30a}, \u{30b}, \u{30c}, \u{323}, \u{327}"),
            ("case", "latn", "ROM ", "\u{300}, \u{301}, \u{302}, \u{303}, \u{304}, \u{306}, \u{307}, \u{308}, \u{309}, \u{30a}, \u{30b}, \u{30c}, \u{323}, \u{327}"),
            ("case", "latn", "TAT ", "\u{300}, \u{301}, \u{302}, \u{303}, \u{304}, \u{306}, \u{307}, \u{308}, \u{309}, \u{30a}, \u{30b}, \u{30c}, \u{323}, \u{327}"),
            ("case", "latn", "TRK ", "\u{300}, \u{301}, \u{302}, \u{303}, \u{304}, \u{306}, \u{307}, \u{308}, \u{309}, \u{30a}, \u{30b}, \u{30c}, \u{323}, \u{327}"),
            ("ccmp", "DFLT", "DFLT", "Ŀ, \u{302}\u{300}, \u{302}\u{301}, \u{302}\u{303}, \u{302}\u{309}, \u{306}\u{300}, \u{306}\u{301}, \u{306}\u{303}, \u{306}\u{309}, ﬁ, ﬂ"),
            ("ccmp", "latn", "AZE ", "Ŀ, \u{302}\u{300}, \u{302}\u{301}, \u{302}\u{303}, \u{302}\u{309}, \u{306}\u{300}, \u{306}\u{301}, \u{306}\u{303}, \u{306}\u{309}, ﬁ, ﬂ"),
            ("ccmp", "latn", "CAT ", "Ŀ, \u{302}\u{300}, \u{302}\u{301}, \u{302}\u{303}, \u{302}\u{309}, \u{306}\u{300}, \u{306}\u{301}, \u{306}\u{303}, \u{306}\u{309}, ﬁ, ﬂ"),
            ("ccmp", "latn", "CRT ", "Ŀ, \u{302}\u{300}, \u{302}\u{301}, \u{302}\u{303}, \u{302}\u{309}, \u{306}\u{300}, \u{306}\u{301}, \u{306}\u{303}, \u{306}\u{309}, ﬁ, ﬂ"),
            ("ccmp", "latn", "DFLT", "Ŀ, \u{302}\u{300}, \u{302}\u{301}, \u{302}\u{303}, \u{302}\u{309}, \u{306}\u{300}, \u{306}\u{301}, \u{306}\u{303}, \u{306}\u{309}, ﬁ, ﬂ"),
            ("ccmp", "latn", "KAZ ", "Ŀ, \u{302}\u{300}, \u{302}\u{301}, \u{302}\u{303}, \u{302}\u{309}, \u{306}\u{300}, \u{306}\u{301}, \u{306}\u{303}, \u{306}\u{309}, ﬁ, ﬂ"),
            ("ccmp", "latn", "MOL ", "Ŀ, \u{302}\u{300}, \u{302}\u{301}, \u{302}\u{303}, \u{302}\u{309}, \u{306}\u{300}, \u{306}\u{301}, \u{306}\u{303}, \u{306}\u{309}, ﬁ, ﬂ"),
            ("ccmp", "latn", "ROM ", "Ŀ, \u{302}\u{300}, \u{302}\u{301}, \u{302}\u{303}, \u{302}\u{309}, \u{306}\u{300}, \u{306}\u{301}, \u{306}\u{303}, \u{306}\u{309}, ﬁ, ﬂ"),
            ("ccmp", "latn", "TAT ", "Ŀ, \u{302}\u{300}, \u{302}\u{301}, \u{302}\u{303}, \u{302}\u{309}, \u{306}\u{300}, \u{306}\u{301}, \u{306}\u{303}, \u{306}\u{309}, ﬁ, ﬂ"),
            ("ccmp", "latn", "TRK ", "Ŀ, \u{302}\u{300}, \u{302}\u{301}, \u{302}\u{303}, \u{302}\u{309}, \u{306}\u{300}, \u{306}\u{301}, \u{306}\u{303}, \u{306}\u{309}, ﬁ, ﬂ"),
            ("dlig", "DFLT", "DFLT", "Th, fb, ffb, ffh, ffj, ffk, fh, fj, fk"),
            ("dlig", "latn", "AZE ", "Th, fb, ffb, ffh, ffj, ffk, fh, fj, fk"),
            ("dlig", "latn", "CAT ", "Th, fb, ffb, ffh, ffj, ffk, fh, fj, fk"),
            ("dlig", "latn", "CRT ", "Th, fb, ffb, ffh, ffj, ffk, fh, fj, fk"),
            ("dlig", "latn", "DFLT", "Th, fb, ffb, ffh, ffj, ffk, fh, fj, fk"),
            ("dlig", "latn", "KAZ ", "Th, fb, ffb, ffh, ffj, ffk, fh, fj, fk"),
            ("dlig", "latn", "MOL ", "Th, fb, ffb, ffh, ffj, ffk, fh, fj, fk"),
            ("dlig", "latn", "ROM ", "Th, fb, ffb, ffh, ffj, ffk, fh, fj, fk"),
            ("dlig", "latn", "TAT ", "Th, fb, ffb, ffh, ffj, ffk, fh, fj, fk"),
            ("dlig", "latn", "TRK ", "Th, fb, ffb, ffh, ffj, ffk, fh, fj, fk"),
            ("dnom", "DFLT", "DFLT", "0, 1, 2, 3, 4"),
            ("dnom", "latn", "AZE ", "0, 1, 2, 3, 4"),
            ("dnom", "latn", "CAT ", "0, 1, 2, 3, 4"),
            ("dnom", "latn", "CRT ", "0, 1, 2, 3, 4"),
            ("dnom", "latn", "DFLT", "0, 1, 2, 3, 4"),
            ("dnom", "latn", "KAZ ", "0, 1, 2, 3, 4"),
            ("dnom", "latn", "MOL ", "0, 1, 2, 3, 4"),
            ("dnom", "latn", "ROM ", "0, 1, 2, 3, 4"),
            ("dnom", "latn", "TAT ", "0, 1, 2, 3, 4"),
            ("dnom", "latn", "TRK ", "0, 1, 2, 3, 4"),
            ("frac", "DFLT", "DFLT", "1/2, 1/4, 3/4"),
            ("frac", "latn", "AZE ", "1/2, 1/4, 3/4"),
            ("frac", "latn", "CAT ", "1/2, 1/4, 3/4"),
            ("frac", "latn", "CRT ", "1/2, 1/4, 3/4"),
            ("frac", "latn", "DFLT", "1/2, 1/4, 3/4"),
            ("frac", "latn", "KAZ ", "1/2, 1/4, 3/4"),
            ("frac", "latn", "MOL ", "1/2, 1/4, 3/4"),
            ("frac", "latn", "ROM ", "1/2, 1/4, 3/4"),
            ("frac", "latn", "TAT ", "1/2, 1/4, 3/4"),
            ("frac", "latn", "TRK ", "1/2, 1/4, 3/4"),
            ("kern", "DFLT", "DFLT", ""),
            ("kern", "latn", "DFLT", ""),
            ("liga", "DFLT", "DFLT", "ff, ffi, ffl, fi, fl"),
            ("liga", "latn", "AZE ", "ff, ffi, ffl, fi, fl"),
            ("liga", "latn", "CAT ", "ff, ffi, ffl, fi, fl"),
            ("liga", "latn", "CRT ", "ff, ffi, ffl, fi, fl"),
            ("liga", "latn", "DFLT", "ff, ffi, ffl, fi, fl"),
            ("liga", "latn", "KAZ ", "ff, ffi, ffl, fi, fl"),
            ("liga", "latn", "MOL ", "ff, ffi, ffl, fi, fl"),
            ("liga", "latn", "ROM ", "ff, ffi, ffl, fi, fl"),
            ("liga", "latn", "TAT ", "ff, ffi, ffl, fi, fl"),
            ("liga", "latn", "TRK ", "ff, ffi, ffl, fi, fl"),
            ("locl", "latn", "AZE ", "i"),
            ("locl", "latn", "CAT ", ""),
            ("locl", "latn", "CRT ", "i"),
            ("locl", "latn", "KAZ ", "i"),
            ("locl", "latn", "MOL ", "Ş, ş, Ţ, ţ"),
            ("locl", "latn", "ROM ", "Ş, ş, Ţ, ţ"),
            ("locl", "latn", "TAT ", "i"),
            ("locl", "latn", "TRK ", "i"),
            ("mark", "DFLT", "DFLT", ""),
            ("mark", "latn", "DFLT", ""),
            ("mkmk", "DFLT", "DFLT", ""),
            ("mkmk", "latn", "DFLT", ""),
            ("numr", "DFLT", "DFLT", "0, 1, 2, 3, 4"),
            ("numr", "latn", "AZE ", "0, 1, 2, 3, 4"),
            ("numr", "latn", "CAT ", "0, 1, 2, 3, 4"),
            ("numr", "latn", "CRT ", "0, 1, 2, 3, 4"),
            ("numr", "latn", "DFLT", "0, 1, 2, 3, 4"),
            ("numr", "latn", "KAZ ", "0, 1, 2, 3, 4"),
            ("numr", "latn", "MOL ", "0, 1, 2, 3, 4"),
            ("numr", "latn", "ROM ", "0, 1, 2, 3, 4"),
            ("numr", "latn", "TAT ", "0, 1, 2, 3, 4"),
            ("numr", "latn", "TRK ", "0, 1, 2, 3, 4"),
            ("sinf", "DFLT", "DFLT", "0, 1, 2, 3, 4"),
            ("sinf", "latn", "AZE ", "0, 1, 2, 3, 4"),
            ("sinf", "latn", "CAT ", "0, 1, 2, 3, 4"),
            ("sinf", "latn", "CRT ", "0, 1, 2, 3, 4"),
            ("sinf", "latn", "DFLT", "0, 1, 2, 3, 4"),
            ("sinf", "latn", "KAZ ", "0, 1, 2, 3, 4"),
            ("sinf", "latn", "MOL ", "0, 1, 2, 3, 4"),
            ("sinf", "latn", "ROM ", "0, 1, 2, 3, 4"),
            ("sinf", "latn", "TAT ", "0, 1, 2, 3, 4"),
            ("sinf", "latn", "TRK ", "0, 1, 2, 3, 4"),
            ("subs", "DFLT", "DFLT", "0, 1, 2, 3, 4"),
            ("subs", "latn", "AZE ", "0, 1, 2, 3, 4"),
            ("subs", "latn", "CAT ", "0, 1, 2, 3, 4"),
            ("subs", "latn", "CRT ", "0, 1, 2, 3, 4"),
            ("subs", "latn", "DFLT", "0, 1, 2, 3, 4"),
            ("subs", "latn", "KAZ ", "0, 1, 2, 3, 4"),
            ("subs", "latn", "MOL ", "0, 1, 2, 3, 4"),
            ("subs", "latn", "ROM ", "0, 1, 2, 3, 4"),
            ("subs", "latn", "TAT ", "0, 1, 2, 3, 4"),
            ("subs", "latn", "TRK ", "0, 1, 2, 3, 4"),
            ("sups", "DFLT", "DFLT", "0, 1, 2, 3, 4"),
            ("sups", "latn", "AZE ", "0, 1, 2, 3, 4"),
            ("sups", "latn", "CAT ", "0, 1, 2, 3, 4"),
            ("sups", "latn", "CRT ", "0, 1, 2, 3, 4"),
            ("sups", "latn", "DFLT", "0, 1, 2, 3, 4"),
            ("sups", "latn", "KAZ ", "0, 1, 2, 3, 4"),
            ("sups", "latn", "MOL ", "0, 1, 2, 3, 4"),
            ("sups", "latn", "ROM ", "0, 1, 2, 3, 4"),
            ("sups", "latn", "TAT ", "0, 1, 2, 3, 4"),
            ("sups", "latn", "TRK ", "0, 1, 2, 3, 4"),
            ("zero", "DFLT", "DFLT", "0"),
            ("zero", "latn", "AZE ", "0"),
            ("zero", "latn", "CAT ", "0"),
            ("zero", "latn", "CRT ", "0"),
            ("zero", "latn", "DFLT", "0"),
            ("zero", "latn", "KAZ ", "0"),
            ("zero", "latn", "MOL ", "0"),
            ("zero", "latn", "ROM ", "0"),
            ("zero", "latn", "TAT ", "0"),
            ("zero", "latn", "TRK ", "0"),
        ],
    );
}

#[test]
fn noto_serif() {
    let mut file = setup(Fixture::NotoSerifThai);
    let entries = extract(&mut file[0]);
    let entries = entries
        .iter()
        .map(|(feature, script, language, _)| (&**feature, &**script, &**language))
        .collect::<Vec<_>>();
    assert_eq!(
        entries,
        [
            ("aalt", "cyrl", "DFLT"),
            ("aalt", "cyrl", "MKD "),
            ("aalt", "cyrl", "SRB "),
            ("aalt", "DFLT", "DFLT"),
            ("aalt", "grek", "APPH"),
            ("aalt", "grek", "DFLT"),
            ("aalt", "grek", "IPPH"),
            ("aalt", "latn", "APPH"),
            ("aalt", "latn", "CAT "),
            ("aalt", "latn", "DFLT"),
            ("aalt", "latn", "IPPH"),
            ("aalt", "latn", "MAH "),
            ("aalt", "latn", "MOL "),
            ("aalt", "latn", "NAV "),
            ("aalt", "latn", "ROM "),
            ("aalt", "thai", "DFLT"),
            ("ccmp", "cyrl", "DFLT"),
            ("ccmp", "cyrl", "MKD "),
            ("ccmp", "cyrl", "SRB "),
            ("ccmp", "DFLT", "DFLT"),
            ("ccmp", "grek", "APPH"),
            ("ccmp", "grek", "DFLT"),
            ("ccmp", "grek", "IPPH"),
            ("ccmp", "latn", "APPH"),
            ("ccmp", "latn", "CAT "),
            ("ccmp", "latn", "DFLT"),
            ("ccmp", "latn", "IPPH"),
            ("ccmp", "latn", "MAH "),
            ("ccmp", "latn", "MOL "),
            ("ccmp", "latn", "NAV "),
            ("ccmp", "latn", "ROM "),
            ("ccmp", "thai", "DFLT"),
            ("dist", "cyrl", "DFLT"),
            ("dist", "cyrl", "MKD "),
            ("dist", "cyrl", "SRB "),
            ("dist", "DFLT", "DFLT"),
            ("dist", "grek", "APPH"),
            ("dist", "grek", "DFLT"),
            ("dist", "grek", "IPPH"),
            ("dist", "latn", "APPH"),
            ("dist", "latn", "CAT "),
            ("dist", "latn", "DFLT"),
            ("dist", "latn", "IPPH"),
            ("dist", "latn", "MAH "),
            ("dist", "latn", "MOL "),
            ("dist", "latn", "NAV "),
            ("dist", "latn", "ROM "),
            ("dist", "thai", "DFLT"),
            ("kern", "DFLT", "DFLT"),
            ("kern", "latn", "APPH"),
            ("kern", "latn", "CAT "),
            ("kern", "latn", "DFLT"),
            ("kern", "latn", "IPPH"),
            ("kern", "latn", "MAH "),
            ("kern", "latn", "MOL "),
            ("kern", "latn", "NAV "),
            ("kern", "latn", "ROM "),
            ("kern", "thai", "DFLT"),
            ("liga", "cyrl", "DFLT"),
            ("liga", "cyrl", "MKD "),
            ("liga", "cyrl", "SRB "),
            ("liga", "DFLT", "DFLT"),
            ("liga", "grek", "APPH"),
            ("liga", "grek", "DFLT"),
            ("liga", "grek", "IPPH"),
            ("liga", "latn", "APPH"),
            ("liga", "latn", "CAT "),
            ("liga", "latn", "DFLT"),
            ("liga", "latn", "IPPH"),
            ("liga", "latn", "MAH "),
            ("liga", "latn", "MOL "),
            ("liga", "latn", "NAV "),
            ("liga", "latn", "ROM "),
            ("liga", "thai", "DFLT"),
            ("locl", "latn", "MOL "),
            ("locl", "latn", "ROM "),
            ("mark", "cyrl", "DFLT"),
            ("mark", "cyrl", "MKD "),
            ("mark", "cyrl", "SRB "),
            ("mark", "DFLT", "DFLT"),
            ("mark", "grek", "APPH"),
            ("mark", "grek", "DFLT"),
            ("mark", "grek", "IPPH"),
            ("mark", "latn", "APPH"),
            ("mark", "latn", "CAT "),
            ("mark", "latn", "DFLT"),
            ("mark", "latn", "IPPH"),
            ("mark", "latn", "MAH "),
            ("mark", "latn", "MOL "),
            ("mark", "latn", "NAV "),
            ("mark", "latn", "ROM "),
            ("mark", "thai", "DFLT"),
            ("mkmk", "cyrl", "DFLT"),
            ("mkmk", "cyrl", "MKD "),
            ("mkmk", "cyrl", "SRB "),
            ("mkmk", "DFLT", "DFLT"),
            ("mkmk", "grek", "APPH"),
            ("mkmk", "grek", "DFLT"),
            ("mkmk", "grek", "IPPH"),
            ("mkmk", "latn", "APPH"),
            ("mkmk", "latn", "CAT "),
            ("mkmk", "latn", "DFLT"),
            ("mkmk", "latn", "IPPH"),
            ("mkmk", "latn", "MAH "),
            ("mkmk", "latn", "MOL "),
            ("mkmk", "latn", "NAV "),
            ("mkmk", "latn", "ROM "),
            ("mkmk", "thai", "DFLT"),
            ("ordn", "cyrl", "DFLT"),
            ("ordn", "cyrl", "MKD "),
            ("ordn", "cyrl", "SRB "),
            ("ordn", "DFLT", "DFLT"),
            ("ordn", "grek", "APPH"),
            ("ordn", "grek", "DFLT"),
            ("ordn", "grek", "IPPH"),
            ("ordn", "latn", "APPH"),
            ("ordn", "latn", "CAT "),
            ("ordn", "latn", "DFLT"),
            ("ordn", "latn", "IPPH"),
            ("ordn", "latn", "MAH "),
            ("ordn", "latn", "MOL "),
            ("ordn", "latn", "NAV "),
            ("ordn", "latn", "ROM "),
            ("ordn", "thai", "DFLT"),
            ("ss01", "cyrl", "DFLT"),
            ("ss01", "cyrl", "MKD "),
            ("ss01", "cyrl", "SRB "),
            ("ss01", "DFLT", "DFLT"),
            ("ss01", "grek", "APPH"),
            ("ss01", "grek", "DFLT"),
            ("ss01", "grek", "IPPH"),
            ("ss01", "latn", "APPH"),
            ("ss01", "latn", "CAT "),
            ("ss01", "latn", "DFLT"),
            ("ss01", "latn", "IPPH"),
            ("ss01", "latn", "MAH "),
            ("ss01", "latn", "MOL "),
            ("ss01", "latn", "NAV "),
            ("ss01", "latn", "ROM "),
            ("ss01", "thai", "DFLT"),
        ]
    );
}

#[test]
fn qahiri() {
    let mut file = setup(Fixture::Qahiri);
    let entries = extract(&mut file[0]);
    let entries = entries
        .iter()
        .map(|(feature, script, language, _)| (&**feature, &**script, &**language))
        .collect::<Vec<_>>();
    assert_eq!(
        entries,
        [
            ("calt", "arab", "DFLT"),
            ("calt", "DFLT", "DFLT"),
            ("ccmp", "arab", "DFLT"),
            ("ccmp", "DFLT", "DFLT"),
            ("clig", "arab", "DFLT"),
            ("clig", "DFLT", "DFLT"),
            ("curs", "arab", "DFLT"),
            ("curs", "DFLT", "DFLT"),
            ("dnom", "arab", "DFLT"),
            ("dnom", "DFLT", "DFLT"),
            ("fina", "arab", "DFLT"),
            ("fina", "DFLT", "DFLT"),
            ("init", "arab", "DFLT"),
            ("init", "DFLT", "DFLT"),
            ("isol", "arab", "DFLT"),
            ("isol", "DFLT", "DFLT"),
            ("kern", "arab", "DFLT"),
            ("kern", "DFLT", "DFLT"),
            ("locl", "latn", "DFLT"),
            ("mark", "arab", "DFLT"),
            ("mark", "DFLT", "DFLT"),
            ("medi", "arab", "DFLT"),
            ("medi", "DFLT", "DFLT"),
            ("numr", "arab", "DFLT"),
            ("numr", "DFLT", "DFLT"),
            ("onum", "arab", "DFLT"),
            ("onum", "DFLT", "DFLT"),
            ("rclt", "arab", "DFLT"),
            ("rclt", "DFLT", "DFLT"),
            ("salt", "arab", "DFLT"),
            ("salt", "DFLT", "DFLT"),
            ("salt", "latn", "DFLT"),
            ("ss01", "arab", "DFLT"),
            ("ss01", "DFLT", "DFLT"),
            ("ss02", "arab", "DFLT"),
            ("ss02", "DFLT", "DFLT"),
        ]
    );
}

fn extract<T>(font: &mut Font<T>) -> Vec<(String, String, String, String)>
where
    T: font::Read,
{
    ok!(font.features())
        .into_iter()
        .flat_map(|(feature, value)| {
            value.into_iter().flat_map(move |(script, value)| {
                value.into_iter().map(move |(language, characters)| {
                    (
                        ok!(Tag::from(feature.clone()).as_str()).to_string(),
                        ok!(Tag::from(script.clone()).as_str()).to_string(),
                        ok!(Tag::from(language).as_str()).to_string(),
                        flatten(&characters),
                    )
                })
            })
        })
        .collect()
}

fn flatten(entries: &BTreeSet<Vec<(char, char)>>) -> String {
    let mut value = String::new();
    for (index, entry) in entries.iter().enumerate() {
        for (start, end) in entry {
            if start == end {
                value.push(*start);
            } else {
                value.push('[');
                value.push(*start);
                value.push('-');
                value.push(*end);
                value.push(']');
            }
        }
        if index + 1 < entries.len() {
            value.push_str(", ");
        }
    }
    value
}
