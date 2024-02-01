#[macro_use]
mod support;

use std::collections::{BTreeMap, BTreeSet};

use crate::support::{setup, Fixture};

#[test]
fn crimson_text() {
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
                    .map(|(script, languages)| (script, languages.into_iter().collect::<Vec<_>>()))
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
