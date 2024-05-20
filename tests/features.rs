#[macro_use]
mod support;

use std::collections::{BTreeMap, BTreeSet};

use font::features::{Language, Script, Type as Feature};
use font::Font;

use crate::support::{setup, Fixture};

#[test]
fn crimson_text() {
    let mut file = setup(Fixture::CrimsonText);
    assert_eq!(
        extract(&mut file[0]),
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
fn noto_serif() {
    let mut file = setup(Fixture::NotoSerifThai);
    assert_eq!(
        extract(&mut file[0]),
        [
            (
                vec![
                    Feature::AccessAllAlternates,
                    Feature::GlyphCompositionDecomposition,
                    Feature::Distances,
                    Feature::StandardLigatures,
                    Feature::MarkPositioning,
                    Feature::MarkToMarkPositioning,
                    Feature::Ordinals,
                    Feature::StylisticSet1
                ],
                vec![
                    (
                        Script::Cyrillic,
                        vec![None, Some(Language::Macedonian), Some(Language::Serbian)]
                    ),
                    (Script::Default, vec![None]),
                    (
                        Script::Greek,
                        vec![
                            None,
                            Some(Language::AmericanistPhoneticNotation),
                            Some(Language::InternationalPhoneticAlphabet)
                        ]
                    ),
                    (
                        Script::Latin,
                        vec![
                            None,
                            Some(Language::AmericanistPhoneticNotation),
                            Some(Language::Catalan),
                            Some(Language::InternationalPhoneticAlphabet),
                            Some(Language::Marshallese),
                            Some(Language::Moldavian),
                            Some(Language::Navajo),
                            Some(Language::Romanian)
                        ]
                    ),
                    (Script::Thai, vec![None])
                ]
            ),
            (
                vec![Feature::Kerning],
                vec![
                    (Script::Default, vec![None]),
                    (
                        Script::Latin,
                        vec![
                            None,
                            Some(Language::AmericanistPhoneticNotation),
                            Some(Language::Catalan),
                            Some(Language::InternationalPhoneticAlphabet),
                            Some(Language::Marshallese),
                            Some(Language::Moldavian),
                            Some(Language::Navajo),
                            Some(Language::Romanian)
                        ]
                    ),
                    (Script::Thai, vec![None])
                ]
            ),
            (
                vec![Feature::LocalizedForms],
                vec![(
                    Script::Latin,
                    vec![Some(Language::Moldavian), Some(Language::Romanian)]
                )]
            ),
        ],
    );
}

#[test]
fn qahiri() {
    let mut file = setup(Fixture::Qahiri);
    assert_eq!(
        extract(&mut file[0]),
        [
            (
                vec![
                    Feature::ContextualAlternates,
                    Feature::GlyphCompositionDecomposition,
                    Feature::ContextualLigatures,
                    Feature::CursivePositioning,
                    Feature::Denominators,
                    Feature::TerminalForms1,
                    Feature::InitialForms,
                    Feature::IsolatedForms,
                    Feature::Kerning,
                    Feature::MarkPositioning,
                    Feature::MedialForms1,
                    Feature::Numerators,
                    Feature::OldstyleFigures,
                    Feature::RequiredContextualAlternates,
                    Feature::StylisticSet1,
                    Feature::StylisticSet2
                ],
                vec![(Script::Arabic, vec![None]), (Script::Default, vec![None])]
            ),
            (
                vec![Feature::StylisticAlternates],
                vec![
                    (Script::Arabic, vec![None]),
                    (Script::Default, vec![None]),
                    (Script::Latin, vec![None])
                ]
            ),
            (
                vec![Feature::LocalizedForms],
                vec![(Script::Latin, vec![None])]
            ),
        ],
    );
}

fn extract<T>(font: &mut Font<T>) -> Vec<(Vec<Feature>, Vec<(Script, Vec<Option<Language>>)>)>
where
    T: font::Read,
{
    let mut values: BTreeMap<_, BTreeSet<Feature>> = Default::default();
    for (feature, value) in ok!(font.features()).into_iter() {
        let scripts = value
            .into_iter()
            .map(|(script, languages)| (script, languages.into_keys().collect::<BTreeSet<_>>()))
            .collect::<BTreeMap<_, BTreeSet<_>>>();
        values.entry(scripts).or_default().insert(feature);
    }
    values
        .into_iter()
        .map(|(scripts, features)| {
            (
                features.into_iter().collect::<Vec<_>>(),
                scripts
                    .into_iter()
                    .map(|(script, languages)| (script, languages.into_iter().collect::<Vec<_>>()))
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}
