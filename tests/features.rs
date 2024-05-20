#[macro_use]
mod support;

use std::ops::RangeInclusive;

use font::features::{Language, Script, Type as Feature};
use font::Font;

use crate::support::{setup, Fixture};

#[test]
fn crimson_text() {
    let mut file = setup(Fixture::CrimsonText);
    let entries = extract(&mut file[0])
        .into_iter()
        .map(|(feature, script, language, _)| (feature, script, language))
        .collect::<Vec<_>>();
    #[rustfmt::skip]
    assert_eq!(
        entries,
        [
            (Feature::CaseSensitiveForms, Script::Default, Language::Default),
            (Feature::CaseSensitiveForms, Script::Latin, Language::Azerbaijani),
            (Feature::CaseSensitiveForms, Script::Latin, Language::Catalan),
            (Feature::CaseSensitiveForms, Script::Latin, Language::CrimeanTatar),
            (Feature::CaseSensitiveForms, Script::Latin, Language::Default),
            (Feature::CaseSensitiveForms, Script::Latin, Language::Kazakh),
            (Feature::CaseSensitiveForms, Script::Latin, Language::Moldavian),
            (Feature::CaseSensitiveForms, Script::Latin, Language::Romanian),
            (Feature::CaseSensitiveForms, Script::Latin, Language::Tatar),
            (Feature::CaseSensitiveForms, Script::Latin, Language::Turkish),
            (Feature::GlyphCompositionDecomposition, Script::Default, Language::Default),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Azerbaijani),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Catalan),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::CrimeanTatar),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Default),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Kazakh),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Moldavian),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Romanian),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Tatar),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Turkish),
            (Feature::DiscretionaryLigatures, Script::Default, Language::Default),
            (Feature::DiscretionaryLigatures, Script::Latin, Language::Azerbaijani),
            (Feature::DiscretionaryLigatures, Script::Latin, Language::Catalan),
            (Feature::DiscretionaryLigatures, Script::Latin, Language::CrimeanTatar),
            (Feature::DiscretionaryLigatures, Script::Latin, Language::Default),
            (Feature::DiscretionaryLigatures, Script::Latin, Language::Kazakh),
            (Feature::DiscretionaryLigatures, Script::Latin, Language::Moldavian),
            (Feature::DiscretionaryLigatures, Script::Latin, Language::Romanian),
            (Feature::DiscretionaryLigatures, Script::Latin, Language::Tatar),
            (Feature::DiscretionaryLigatures, Script::Latin, Language::Turkish),
            (Feature::Denominators, Script::Default, Language::Default),
            (Feature::Denominators, Script::Latin, Language::Azerbaijani),
            (Feature::Denominators, Script::Latin, Language::Catalan),
            (Feature::Denominators, Script::Latin, Language::CrimeanTatar),
            (Feature::Denominators, Script::Latin, Language::Default),
            (Feature::Denominators, Script::Latin, Language::Kazakh),
            (Feature::Denominators, Script::Latin, Language::Moldavian),
            (Feature::Denominators, Script::Latin, Language::Romanian),
            (Feature::Denominators, Script::Latin, Language::Tatar),
            (Feature::Denominators, Script::Latin, Language::Turkish),
            (Feature::Fractions, Script::Default, Language::Default),
            (Feature::Fractions, Script::Latin, Language::Azerbaijani),
            (Feature::Fractions, Script::Latin, Language::Catalan),
            (Feature::Fractions, Script::Latin, Language::CrimeanTatar),
            (Feature::Fractions, Script::Latin, Language::Default),
            (Feature::Fractions, Script::Latin, Language::Kazakh),
            (Feature::Fractions, Script::Latin, Language::Moldavian),
            (Feature::Fractions, Script::Latin, Language::Romanian),
            (Feature::Fractions, Script::Latin, Language::Tatar),
            (Feature::Fractions, Script::Latin, Language::Turkish),
            (Feature::Kerning, Script::Default, Language::Default),
            (Feature::Kerning, Script::Latin, Language::Default),
            (Feature::StandardLigatures, Script::Default, Language::Default),
            (Feature::StandardLigatures, Script::Latin, Language::Azerbaijani),
            (Feature::StandardLigatures, Script::Latin, Language::Catalan),
            (Feature::StandardLigatures, Script::Latin, Language::CrimeanTatar),
            (Feature::StandardLigatures, Script::Latin, Language::Default),
            (Feature::StandardLigatures, Script::Latin, Language::Kazakh),
            (Feature::StandardLigatures, Script::Latin, Language::Moldavian),
            (Feature::StandardLigatures, Script::Latin, Language::Romanian),
            (Feature::StandardLigatures, Script::Latin, Language::Tatar),
            (Feature::StandardLigatures, Script::Latin, Language::Turkish),
            (Feature::LocalizedForms, Script::Latin, Language::Azerbaijani),
            (Feature::LocalizedForms, Script::Latin, Language::Catalan),
            (Feature::LocalizedForms, Script::Latin, Language::CrimeanTatar),
            (Feature::LocalizedForms, Script::Latin, Language::Kazakh),
            (Feature::LocalizedForms, Script::Latin, Language::Moldavian),
            (Feature::LocalizedForms, Script::Latin, Language::Romanian),
            (Feature::LocalizedForms, Script::Latin, Language::Tatar),
            (Feature::LocalizedForms, Script::Latin, Language::Turkish),
            (Feature::MarkPositioning, Script::Default, Language::Default),
            (Feature::MarkPositioning, Script::Latin, Language::Default),
            (Feature::MarkToMarkPositioning, Script::Default, Language::Default),
            (Feature::MarkToMarkPositioning, Script::Latin, Language::Default),
            (Feature::Numerators, Script::Default, Language::Default),
            (Feature::Numerators, Script::Latin, Language::Azerbaijani),
            (Feature::Numerators, Script::Latin, Language::Catalan),
            (Feature::Numerators, Script::Latin, Language::CrimeanTatar),
            (Feature::Numerators, Script::Latin, Language::Default),
            (Feature::Numerators, Script::Latin, Language::Kazakh),
            (Feature::Numerators, Script::Latin, Language::Moldavian),
            (Feature::Numerators, Script::Latin, Language::Romanian),
            (Feature::Numerators, Script::Latin, Language::Tatar),
            (Feature::Numerators, Script::Latin, Language::Turkish),
            (Feature::ScientificInferiors, Script::Default, Language::Default),
            (Feature::ScientificInferiors, Script::Latin, Language::Azerbaijani),
            (Feature::ScientificInferiors, Script::Latin, Language::Catalan),
            (Feature::ScientificInferiors, Script::Latin, Language::CrimeanTatar),
            (Feature::ScientificInferiors, Script::Latin, Language::Default),
            (Feature::ScientificInferiors, Script::Latin, Language::Kazakh),
            (Feature::ScientificInferiors, Script::Latin, Language::Moldavian),
            (Feature::ScientificInferiors, Script::Latin, Language::Romanian),
            (Feature::ScientificInferiors, Script::Latin, Language::Tatar),
            (Feature::ScientificInferiors, Script::Latin, Language::Turkish),
            (Feature::Subscript, Script::Default, Language::Default),
            (Feature::Subscript, Script::Latin, Language::Azerbaijani),
            (Feature::Subscript, Script::Latin, Language::Catalan),
            (Feature::Subscript, Script::Latin, Language::CrimeanTatar),
            (Feature::Subscript, Script::Latin, Language::Default),
            (Feature::Subscript, Script::Latin, Language::Kazakh),
            (Feature::Subscript, Script::Latin, Language::Moldavian),
            (Feature::Subscript, Script::Latin, Language::Romanian),
            (Feature::Subscript, Script::Latin, Language::Tatar),
            (Feature::Subscript, Script::Latin, Language::Turkish),
            (Feature::Superscript, Script::Default, Language::Default),
            (Feature::Superscript, Script::Latin, Language::Azerbaijani),
            (Feature::Superscript, Script::Latin, Language::Catalan),
            (Feature::Superscript, Script::Latin, Language::CrimeanTatar),
            (Feature::Superscript, Script::Latin, Language::Default),
            (Feature::Superscript, Script::Latin, Language::Kazakh),
            (Feature::Superscript, Script::Latin, Language::Moldavian),
            (Feature::Superscript, Script::Latin, Language::Romanian),
            (Feature::Superscript, Script::Latin, Language::Tatar),
            (Feature::Superscript, Script::Latin, Language::Turkish),
            (Feature::SlashedZero, Script::Default, Language::Default),
            (Feature::SlashedZero, Script::Latin, Language::Azerbaijani),
            (Feature::SlashedZero, Script::Latin, Language::Catalan),
            (Feature::SlashedZero, Script::Latin, Language::CrimeanTatar),
            (Feature::SlashedZero, Script::Latin, Language::Default),
            (Feature::SlashedZero, Script::Latin, Language::Kazakh),
            (Feature::SlashedZero, Script::Latin, Language::Moldavian),
            (Feature::SlashedZero, Script::Latin, Language::Romanian),
            (Feature::SlashedZero, Script::Latin, Language::Tatar),
            (Feature::SlashedZero, Script::Latin, Language::Turkish),
        ],
    );
}

#[test]
fn noto_serif() {
    let mut file = setup(Fixture::NotoSerifThai);
    let entries = extract(&mut file[0])
        .into_iter()
        .map(|(feature, script, language, _)| (feature, script, language))
        .collect::<Vec<_>>();
    #[rustfmt::skip]
    assert_eq!(
        entries,
        [
            (Feature::AccessAllAlternates, Script::Cyrillic, Language::Default),
            (Feature::AccessAllAlternates, Script::Cyrillic, Language::Macedonian),
            (Feature::AccessAllAlternates, Script::Cyrillic, Language::Serbian),
            (Feature::AccessAllAlternates, Script::Default, Language::Default),
            (Feature::AccessAllAlternates, Script::Greek, Language::AmericanistPhoneticNotation),
            (Feature::AccessAllAlternates, Script::Greek, Language::Default),
            (Feature::AccessAllAlternates, Script::Greek, Language::InternationalPhoneticAlphabet),
            (Feature::AccessAllAlternates, Script::Latin, Language::AmericanistPhoneticNotation),
            (Feature::AccessAllAlternates, Script::Latin, Language::Catalan),
            (Feature::AccessAllAlternates, Script::Latin, Language::Default),
            (Feature::AccessAllAlternates, Script::Latin, Language::InternationalPhoneticAlphabet),
            (Feature::AccessAllAlternates, Script::Latin, Language::Marshallese),
            (Feature::AccessAllAlternates, Script::Latin, Language::Moldavian),
            (Feature::AccessAllAlternates, Script::Latin, Language::Navajo),
            (Feature::AccessAllAlternates, Script::Latin, Language::Romanian),
            (Feature::AccessAllAlternates, Script::Thai, Language::Default),
            (Feature::GlyphCompositionDecomposition, Script::Cyrillic, Language::Default),
            (Feature::GlyphCompositionDecomposition, Script::Cyrillic, Language::Macedonian),
            (Feature::GlyphCompositionDecomposition, Script::Cyrillic, Language::Serbian),
            (Feature::GlyphCompositionDecomposition, Script::Default, Language::Default),
            (Feature::GlyphCompositionDecomposition, Script::Greek, Language::AmericanistPhoneticNotation),
            (Feature::GlyphCompositionDecomposition, Script::Greek, Language::Default),
            (Feature::GlyphCompositionDecomposition, Script::Greek, Language::InternationalPhoneticAlphabet),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::AmericanistPhoneticNotation),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Catalan),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Default),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::InternationalPhoneticAlphabet),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Marshallese),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Moldavian),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Navajo),
            (Feature::GlyphCompositionDecomposition, Script::Latin, Language::Romanian),
            (Feature::GlyphCompositionDecomposition, Script::Thai, Language::Default),
            (Feature::Distances, Script::Cyrillic, Language::Default),
            (Feature::Distances, Script::Cyrillic, Language::Macedonian),
            (Feature::Distances, Script::Cyrillic, Language::Serbian),
            (Feature::Distances, Script::Default, Language::Default),
            (Feature::Distances, Script::Greek, Language::AmericanistPhoneticNotation),
            (Feature::Distances, Script::Greek, Language::Default),
            (Feature::Distances, Script::Greek, Language::InternationalPhoneticAlphabet),
            (Feature::Distances, Script::Latin, Language::AmericanistPhoneticNotation),
            (Feature::Distances, Script::Latin, Language::Catalan),
            (Feature::Distances, Script::Latin, Language::Default),
            (Feature::Distances, Script::Latin, Language::InternationalPhoneticAlphabet),
            (Feature::Distances, Script::Latin, Language::Marshallese),
            (Feature::Distances, Script::Latin, Language::Moldavian),
            (Feature::Distances, Script::Latin, Language::Navajo),
            (Feature::Distances, Script::Latin, Language::Romanian),
            (Feature::Distances, Script::Thai, Language::Default),
            (Feature::Kerning, Script::Default, Language::Default),
            (Feature::Kerning, Script::Latin, Language::AmericanistPhoneticNotation),
            (Feature::Kerning, Script::Latin, Language::Catalan),
            (Feature::Kerning, Script::Latin, Language::Default),
            (Feature::Kerning, Script::Latin, Language::InternationalPhoneticAlphabet),
            (Feature::Kerning, Script::Latin, Language::Marshallese),
            (Feature::Kerning, Script::Latin, Language::Moldavian),
            (Feature::Kerning, Script::Latin, Language::Navajo),
            (Feature::Kerning, Script::Latin, Language::Romanian),
            (Feature::Kerning, Script::Thai, Language::Default),
            (Feature::StandardLigatures, Script::Cyrillic, Language::Default),
            (Feature::StandardLigatures, Script::Cyrillic, Language::Macedonian),
            (Feature::StandardLigatures, Script::Cyrillic, Language::Serbian),
            (Feature::StandardLigatures, Script::Default, Language::Default),
            (Feature::StandardLigatures, Script::Greek, Language::AmericanistPhoneticNotation),
            (Feature::StandardLigatures, Script::Greek, Language::Default),
            (Feature::StandardLigatures, Script::Greek, Language::InternationalPhoneticAlphabet),
            (Feature::StandardLigatures, Script::Latin, Language::AmericanistPhoneticNotation),
            (Feature::StandardLigatures, Script::Latin, Language::Catalan),
            (Feature::StandardLigatures, Script::Latin, Language::Default),
            (Feature::StandardLigatures, Script::Latin, Language::InternationalPhoneticAlphabet),
            (Feature::StandardLigatures, Script::Latin, Language::Marshallese),
            (Feature::StandardLigatures, Script::Latin, Language::Moldavian),
            (Feature::StandardLigatures, Script::Latin, Language::Navajo),
            (Feature::StandardLigatures, Script::Latin, Language::Romanian),
            (Feature::StandardLigatures, Script::Thai, Language::Default),
            (Feature::LocalizedForms, Script::Latin, Language::Moldavian),
            (Feature::LocalizedForms, Script::Latin, Language::Romanian),
            (Feature::MarkPositioning, Script::Cyrillic, Language::Default),
            (Feature::MarkPositioning, Script::Cyrillic, Language::Macedonian),
            (Feature::MarkPositioning, Script::Cyrillic, Language::Serbian),
            (Feature::MarkPositioning, Script::Default, Language::Default),
            (Feature::MarkPositioning, Script::Greek, Language::AmericanistPhoneticNotation),
            (Feature::MarkPositioning, Script::Greek, Language::Default),
            (Feature::MarkPositioning, Script::Greek, Language::InternationalPhoneticAlphabet),
            (Feature::MarkPositioning, Script::Latin, Language::AmericanistPhoneticNotation),
            (Feature::MarkPositioning, Script::Latin, Language::Catalan),
            (Feature::MarkPositioning, Script::Latin, Language::Default),
            (Feature::MarkPositioning, Script::Latin, Language::InternationalPhoneticAlphabet),
            (Feature::MarkPositioning, Script::Latin, Language::Marshallese),
            (Feature::MarkPositioning, Script::Latin, Language::Moldavian),
            (Feature::MarkPositioning, Script::Latin, Language::Navajo),
            (Feature::MarkPositioning, Script::Latin, Language::Romanian),
            (Feature::MarkPositioning, Script::Thai, Language::Default),
            (Feature::MarkToMarkPositioning, Script::Cyrillic, Language::Default),
            (Feature::MarkToMarkPositioning, Script::Cyrillic, Language::Macedonian),
            (Feature::MarkToMarkPositioning, Script::Cyrillic, Language::Serbian),
            (Feature::MarkToMarkPositioning, Script::Default, Language::Default),
            (Feature::MarkToMarkPositioning, Script::Greek, Language::AmericanistPhoneticNotation),
            (Feature::MarkToMarkPositioning, Script::Greek, Language::Default),
            (Feature::MarkToMarkPositioning, Script::Greek, Language::InternationalPhoneticAlphabet),
            (Feature::MarkToMarkPositioning, Script::Latin, Language::AmericanistPhoneticNotation),
            (Feature::MarkToMarkPositioning, Script::Latin, Language::Catalan),
            (Feature::MarkToMarkPositioning, Script::Latin, Language::Default),
            (Feature::MarkToMarkPositioning, Script::Latin, Language::InternationalPhoneticAlphabet),
            (Feature::MarkToMarkPositioning, Script::Latin, Language::Marshallese),
            (Feature::MarkToMarkPositioning, Script::Latin, Language::Moldavian),
            (Feature::MarkToMarkPositioning, Script::Latin, Language::Navajo),
            (Feature::MarkToMarkPositioning, Script::Latin, Language::Romanian),
            (Feature::MarkToMarkPositioning, Script::Thai, Language::Default),
            (Feature::Ordinals, Script::Cyrillic, Language::Default),
            (Feature::Ordinals, Script::Cyrillic, Language::Macedonian),
            (Feature::Ordinals, Script::Cyrillic, Language::Serbian),
            (Feature::Ordinals, Script::Default, Language::Default),
            (Feature::Ordinals, Script::Greek, Language::AmericanistPhoneticNotation),
            (Feature::Ordinals, Script::Greek, Language::Default),
            (Feature::Ordinals, Script::Greek, Language::InternationalPhoneticAlphabet),
            (Feature::Ordinals, Script::Latin, Language::AmericanistPhoneticNotation),
            (Feature::Ordinals, Script::Latin, Language::Catalan),
            (Feature::Ordinals, Script::Latin, Language::Default),
            (Feature::Ordinals, Script::Latin, Language::InternationalPhoneticAlphabet),
            (Feature::Ordinals, Script::Latin, Language::Marshallese),
            (Feature::Ordinals, Script::Latin, Language::Moldavian),
            (Feature::Ordinals, Script::Latin, Language::Navajo),
            (Feature::Ordinals, Script::Latin, Language::Romanian),
            (Feature::Ordinals, Script::Thai, Language::Default),
            (Feature::StylisticSet1, Script::Cyrillic, Language::Default),
            (Feature::StylisticSet1, Script::Cyrillic, Language::Macedonian),
            (Feature::StylisticSet1, Script::Cyrillic, Language::Serbian),
            (Feature::StylisticSet1, Script::Default, Language::Default),
            (Feature::StylisticSet1, Script::Greek, Language::AmericanistPhoneticNotation),
            (Feature::StylisticSet1, Script::Greek, Language::Default),
            (Feature::StylisticSet1, Script::Greek, Language::InternationalPhoneticAlphabet),
            (Feature::StylisticSet1, Script::Latin, Language::AmericanistPhoneticNotation),
            (Feature::StylisticSet1, Script::Latin, Language::Catalan),
            (Feature::StylisticSet1, Script::Latin, Language::Default),
            (Feature::StylisticSet1, Script::Latin, Language::InternationalPhoneticAlphabet),
            (Feature::StylisticSet1, Script::Latin, Language::Marshallese),
            (Feature::StylisticSet1, Script::Latin, Language::Moldavian),
            (Feature::StylisticSet1, Script::Latin, Language::Navajo),
            (Feature::StylisticSet1, Script::Latin, Language::Romanian),
            (Feature::StylisticSet1, Script::Thai, Language::Default),
        ]
    );
}

#[test]
fn qahiri() {
    let mut file = setup(Fixture::Qahiri);
    let entries = extract(&mut file[0])
        .into_iter()
        .map(|(feature, script, language, _)| (feature, script, language))
        .collect::<Vec<_>>();
    #[rustfmt::skip]
    assert_eq!(
        entries,
        [
            (Feature::ContextualAlternates, Script::Arabic, Language::Default),
            (Feature::ContextualAlternates, Script::Default, Language::Default),
            (Feature::GlyphCompositionDecomposition, Script::Arabic, Language::Default),
            (Feature::GlyphCompositionDecomposition, Script::Default, Language::Default),
            (Feature::ContextualLigatures, Script::Arabic, Language::Default),
            (Feature::ContextualLigatures, Script::Default, Language::Default),
            (Feature::CursivePositioning, Script::Arabic, Language::Default),
            (Feature::CursivePositioning, Script::Default, Language::Default),
            (Feature::Denominators, Script::Arabic, Language::Default),
            (Feature::Denominators, Script::Default, Language::Default),
            (Feature::TerminalForms1, Script::Arabic, Language::Default),
            (Feature::TerminalForms1, Script::Default, Language::Default),
            (Feature::InitialForms, Script::Arabic, Language::Default),
            (Feature::InitialForms, Script::Default, Language::Default),
            (Feature::IsolatedForms, Script::Arabic, Language::Default),
            (Feature::IsolatedForms, Script::Default, Language::Default),
            (Feature::Kerning, Script::Arabic, Language::Default),
            (Feature::Kerning, Script::Default, Language::Default),
            (Feature::LocalizedForms, Script::Latin, Language::Default),
            (Feature::MarkPositioning, Script::Arabic, Language::Default),
            (Feature::MarkPositioning, Script::Default, Language::Default),
            (Feature::MedialForms1, Script::Arabic, Language::Default),
            (Feature::MedialForms1, Script::Default, Language::Default),
            (Feature::Numerators, Script::Arabic, Language::Default),
            (Feature::Numerators, Script::Default, Language::Default),
            (Feature::OldstyleFigures, Script::Arabic, Language::Default),
            (Feature::OldstyleFigures, Script::Default, Language::Default),
            (Feature::RequiredContextualAlternates, Script::Arabic, Language::Default),
            (Feature::RequiredContextualAlternates, Script::Default, Language::Default),
            (Feature::StylisticAlternates, Script::Arabic, Language::Default),
            (Feature::StylisticAlternates, Script::Default, Language::Default),
            (Feature::StylisticAlternates, Script::Latin, Language::Default),
            (Feature::StylisticSet1, Script::Arabic, Language::Default),
            (Feature::StylisticSet1, Script::Default, Language::Default),
            (Feature::StylisticSet2, Script::Arabic, Language::Default),
            (Feature::StylisticSet2, Script::Default, Language::Default)
        ]
    );
}

fn extract<T>(
    font: &mut Font<T>,
) -> Vec<(Feature, Script, Language, Vec<Vec<RangeInclusive<char>>>)>
where
    T: font::Read,
{
    ok!(font.features())
        .into_iter()
        .flat_map(|(feature, value)| {
            value.into_iter().flat_map(move |(script, value)| {
                value.into_iter().map(move |(language, characters)| {
                    (feature.clone(), script.clone(), language, characters)
                })
            })
        })
        .collect()
}
