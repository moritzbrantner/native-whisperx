use native_whisperx::{
    CuratedLanguage, TranslationPlan, TranslationPlanError, TranslationPlanProvenance,
};

const EXPECTED_DIRECT_MODELS: &[((CuratedLanguage, CuratedLanguage), &str)] = &[
    (
        (CuratedLanguage::German, CuratedLanguage::English),
        "Helsinki-NLP/opus-mt-de-en",
    ),
    (
        (CuratedLanguage::German, CuratedLanguage::Spanish),
        "Helsinki-NLP/opus-mt-de-es",
    ),
    (
        (CuratedLanguage::German, CuratedLanguage::French),
        "Helsinki-NLP/opus-mt-de-fr",
    ),
    (
        (CuratedLanguage::German, CuratedLanguage::Italian),
        "Helsinki-NLP/opus-mt-de-it",
    ),
    (
        (CuratedLanguage::German, CuratedLanguage::Dutch),
        "Helsinki-NLP/opus-mt-de-nl",
    ),
    (
        (CuratedLanguage::German, CuratedLanguage::Polish),
        "Helsinki-NLP/opus-mt-de-pl",
    ),
    (
        (CuratedLanguage::English, CuratedLanguage::German),
        "Helsinki-NLP/opus-mt-en-de",
    ),
    (
        (CuratedLanguage::English, CuratedLanguage::Spanish),
        "Helsinki-NLP/opus-mt-en-es",
    ),
    (
        (CuratedLanguage::English, CuratedLanguage::French),
        "Helsinki-NLP/opus-mt-en-fr",
    ),
    (
        (CuratedLanguage::English, CuratedLanguage::Italian),
        "Helsinki-NLP/opus-mt-en-it",
    ),
    (
        (CuratedLanguage::English, CuratedLanguage::Portuguese),
        "Helsinki-NLP/opus-mt-tc-big-en-pt",
    ),
    (
        (CuratedLanguage::English, CuratedLanguage::Dutch),
        "Helsinki-NLP/opus-mt-en-nl",
    ),
    (
        (CuratedLanguage::English, CuratedLanguage::Polish),
        "Helsinki-NLP/opus-mt-en-zlw",
    ),
    (
        (CuratedLanguage::Spanish, CuratedLanguage::German),
        "Helsinki-NLP/opus-mt-es-de",
    ),
    (
        (CuratedLanguage::Spanish, CuratedLanguage::English),
        "Helsinki-NLP/opus-mt-es-en",
    ),
    (
        (CuratedLanguage::Spanish, CuratedLanguage::French),
        "Helsinki-NLP/opus-mt-es-fr",
    ),
    (
        (CuratedLanguage::Spanish, CuratedLanguage::Italian),
        "Helsinki-NLP/opus-mt-es-it",
    ),
    (
        (CuratedLanguage::Spanish, CuratedLanguage::Dutch),
        "Helsinki-NLP/opus-mt-es-nl",
    ),
    (
        (CuratedLanguage::Spanish, CuratedLanguage::Polish),
        "Helsinki-NLP/opus-mt-es-pl",
    ),
    (
        (CuratedLanguage::French, CuratedLanguage::German),
        "Helsinki-NLP/opus-mt-fr-de",
    ),
    (
        (CuratedLanguage::French, CuratedLanguage::English),
        "Helsinki-NLP/opus-mt-fr-en",
    ),
    (
        (CuratedLanguage::French, CuratedLanguage::Spanish),
        "Helsinki-NLP/opus-mt-fr-es",
    ),
    (
        (CuratedLanguage::French, CuratedLanguage::Polish),
        "Helsinki-NLP/opus-mt-fr-pl",
    ),
    (
        (CuratedLanguage::Italian, CuratedLanguage::German),
        "Helsinki-NLP/opus-mt-it-de",
    ),
    (
        (CuratedLanguage::Italian, CuratedLanguage::English),
        "Helsinki-NLP/opus-mt-it-en",
    ),
    (
        (CuratedLanguage::Italian, CuratedLanguage::Spanish),
        "Helsinki-NLP/opus-mt-it-es",
    ),
    (
        (CuratedLanguage::Italian, CuratedLanguage::French),
        "Helsinki-NLP/opus-mt-it-fr",
    ),
    (
        (CuratedLanguage::Portuguese, CuratedLanguage::English),
        "Helsinki-NLP/opus-mt-ROMANCE-en",
    ),
    (
        (CuratedLanguage::Dutch, CuratedLanguage::English),
        "Helsinki-NLP/opus-mt-nl-en",
    ),
    (
        (CuratedLanguage::Dutch, CuratedLanguage::Spanish),
        "Helsinki-NLP/opus-mt-nl-es",
    ),
    (
        (CuratedLanguage::Dutch, CuratedLanguage::French),
        "Helsinki-NLP/opus-mt-nl-fr",
    ),
    (
        (CuratedLanguage::Polish, CuratedLanguage::German),
        "Helsinki-NLP/opus-mt-pl-de",
    ),
    (
        (CuratedLanguage::Polish, CuratedLanguage::English),
        "Helsinki-NLP/opus-mt-pl-en",
    ),
    (
        (CuratedLanguage::Polish, CuratedLanguage::Spanish),
        "Helsinki-NLP/opus-mt-pl-es",
    ),
    (
        (CuratedLanguage::Polish, CuratedLanguage::French),
        "Helsinki-NLP/opus-mt-pl-fr",
    ),
];

#[test]
fn german_to_english_uses_the_compatible_direct_model() {
    let plan = TranslationPlan::new(CuratedLanguage::German, CuratedLanguage::English)
        .expect("German to English should be supported");

    assert_eq!(plan.source(), CuratedLanguage::German);
    assert_eq!(plan.target(), CuratedLanguage::English);
    assert_eq!(plan.provenance(), TranslationPlanProvenance::Direct);
    assert_eq!(plan.legs().len(), 1);
    assert_eq!(plan.legs()[0].source(), CuratedLanguage::German);
    assert_eq!(plan.legs()[0].target(), CuratedLanguage::English);
    assert_eq!(plan.legs()[0].model_id(), "Helsinki-NLP/opus-mt-de-en");
}

#[test]
fn portuguese_to_dutch_uses_an_ordered_english_pivot() {
    let plan = TranslationPlan::new(CuratedLanguage::Portuguese, CuratedLanguage::Dutch)
        .expect("Portuguese to Dutch should be supported through English");

    assert_eq!(plan.source(), CuratedLanguage::Portuguese);
    assert_eq!(plan.target(), CuratedLanguage::Dutch);
    assert_eq!(
        plan.provenance(),
        TranslationPlanProvenance::PivotTranslation {
            pivot: CuratedLanguage::English,
        }
    );
    assert_eq!(plan.legs().len(), 2);
    assert_eq!(plan.legs()[0].source(), CuratedLanguage::Portuguese);
    assert_eq!(plan.legs()[0].target(), CuratedLanguage::English);
    assert_eq!(plan.legs()[0].model_id(), "Helsinki-NLP/opus-mt-ROMANCE-en");
    assert_eq!(plan.legs()[1].source(), CuratedLanguage::English);
    assert_eq!(plan.legs()[1].target(), CuratedLanguage::Dutch);
    assert_eq!(plan.legs()[1].model_id(), "Helsinki-NLP/opus-mt-en-nl");
}

#[test]
fn every_distinct_curated_pair_has_a_repeatable_well_formed_plan() {
    let mut pair_count = 0;

    for source in CuratedLanguage::ALL {
        for target in CuratedLanguage::ALL {
            if source == target {
                continue;
            }
            pair_count += 1;

            let first = TranslationPlan::new(source, target)
                .expect("every distinct curated pair should have a plan");
            let repeated = TranslationPlan::new(source, target)
                .expect("selecting the same pair again should succeed");

            assert_eq!(first, repeated, "{source}->{target} changed plans");
            assert_eq!(first.source(), source);
            assert_eq!(first.target(), target);

            if let Some(model_id) = expected_direct_model(source, target) {
                assert_eq!(first.provenance(), TranslationPlanProvenance::Direct);
                assert_eq!(first.legs().len(), 1, "{source}->{target}");
                assert_eq!(first.legs()[0].source(), source);
                assert_eq!(first.legs()[0].target(), target);
                assert_eq!(first.legs()[0].model_id(), model_id);
            } else {
                let pivot = CuratedLanguage::English;
                assert_eq!(
                    first.provenance(),
                    TranslationPlanProvenance::PivotTranslation { pivot }
                );
                assert_eq!(first.legs().len(), 2, "{source}->{target}");
                assert_eq!(first.legs()[0].source(), source);
                assert_eq!(first.legs()[0].target(), pivot);
                assert_eq!(
                    first.legs()[0].model_id(),
                    expected_direct_model(source, pivot)
                        .expect("every curated source must have a golden English bridge")
                );
                assert_eq!(first.legs()[1].source(), pivot);
                assert_eq!(first.legs()[1].target(), target);
                assert_eq!(
                    first.legs()[1].model_id(),
                    expected_direct_model(pivot, target)
                        .expect("every curated target must have a golden English bridge")
                );
            }
        }
    }

    assert_eq!(pair_count, 56);
}

#[test]
fn same_and_unsupported_languages_have_distinct_typed_errors() {
    let same_language = TranslationPlan::new(CuratedLanguage::French, CuratedLanguage::French)
        .expect_err("same-language translation should be rejected");
    assert_eq!(
        same_language,
        TranslationPlanError::SameLanguage {
            language: CuratedLanguage::French,
        }
    );

    let unsupported = TranslationPlan::from_language_codes("ja", "en")
        .expect_err("languages outside the curated set should be rejected");
    assert_eq!(
        unsupported,
        TranslationPlanError::UnsupportedLanguage {
            language: "ja".to_string(),
        }
    );
}

fn expected_direct_model(source: CuratedLanguage, target: CuratedLanguage) -> Option<&'static str> {
    EXPECTED_DIRECT_MODELS
        .iter()
        .find_map(|((candidate_source, candidate_target), model_id)| {
            (*candidate_source == source && *candidate_target == target).then_some(*model_id)
        })
}
