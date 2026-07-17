//! Deterministic planning for the curated native translation surface.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

/// A language supported by the curated native translation registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CuratedLanguage {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "nl")]
    Dutch,
    #[serde(rename = "pl")]
    Polish,
}

impl CuratedLanguage {
    /// The complete curated language set in stable registry order.
    pub const ALL: [Self; 8] = [
        Self::English,
        Self::German,
        Self::French,
        Self::Spanish,
        Self::Italian,
        Self::Portuguese,
        Self::Dutch,
        Self::Polish,
    ];

    /// Returns the ISO 639-1 language code used by OPUS-MT plans.
    pub const fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::German => "de",
            Self::French => "fr",
            Self::Spanish => "es",
            Self::Italian => "it",
            Self::Portuguese => "pt",
            Self::Dutch => "nl",
            Self::Polish => "pl",
        }
    }
}

impl fmt::Display for CuratedLanguage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl FromStr for CuratedLanguage {
    type Err = TranslationPlanError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "en" => Ok(Self::English),
            "de" => Ok(Self::German),
            "fr" => Ok(Self::French),
            "es" => Ok(Self::Spanish),
            "it" => Ok(Self::Italian),
            "pt" => Ok(Self::Portuguese),
            "nl" => Ok(Self::Dutch),
            "pl" => Ok(Self::Polish),
            _ => Err(TranslationPlanError::UnsupportedLanguage {
                language: value.to_string(),
            }),
        }
    }
}

/// Whether a plan uses one direct model or composes two legs through English.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum TranslationPlanProvenance {
    Direct,
    PivotTranslation { pivot: CuratedLanguage },
}

/// One ordered OPUS-MT model invocation in a translation plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationLeg {
    source: CuratedLanguage,
    target: CuratedLanguage,
    model_id: String,
}

impl TranslationLeg {
    /// Returns the source language consumed by this leg.
    pub const fn source(&self) -> CuratedLanguage {
        self.source
    }

    /// Returns the target language produced by this leg.
    pub const fn target(&self) -> CuratedLanguage {
        self.target
    }

    /// Returns the canonical Hugging Face OPUS-MT model ID for this leg.
    pub fn model_id(&self) -> &str {
        &self.model_id
    }
}

/// A deterministic direct or Pivot Translation plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationPlan {
    source: CuratedLanguage,
    target: CuratedLanguage,
    provenance: TranslationPlanProvenance,
    legs: Vec<TranslationLeg>,
}

impl TranslationPlan {
    /// Selects the stable validated plan for a pair of curated languages.
    pub fn new(
        source: CuratedLanguage,
        target: CuratedLanguage,
    ) -> Result<Self, TranslationPlanError> {
        if source == target {
            return Err(TranslationPlanError::SameLanguage { language: source });
        }

        if let Some(model_id) = validated_direct_model_id(source, target) {
            return Ok(Self {
                source,
                target,
                provenance: TranslationPlanProvenance::Direct,
                legs: vec![TranslationLeg {
                    source,
                    target,
                    model_id: model_id.to_string(),
                }],
            });
        }

        let pivot = CuratedLanguage::English;
        let source_model = validated_direct_model_id(source, pivot)
            .expect("every curated source language has a validated English translation model");
        let target_model = validated_direct_model_id(pivot, target)
            .expect("every curated target language has a validated English translation model");
        Ok(Self {
            source,
            target,
            provenance: TranslationPlanProvenance::PivotTranslation { pivot },
            legs: vec![
                TranslationLeg {
                    source,
                    target: pivot,
                    model_id: source_model.to_string(),
                },
                TranslationLeg {
                    source: pivot,
                    target,
                    model_id: target_model.to_string(),
                },
            ],
        })
    }

    /// Parses ISO 639-1 codes and selects their stable validated plan.
    pub fn from_language_codes(source: &str, target: &str) -> Result<Self, TranslationPlanError> {
        Self::new(source.parse()?, target.parse()?)
    }

    /// Returns the requested source language.
    pub const fn source(&self) -> CuratedLanguage {
        self.source
    }

    /// Returns the requested target language.
    pub const fn target(&self) -> CuratedLanguage {
        self.target
    }

    /// Returns whether the plan is direct or a Pivot Translation.
    pub const fn provenance(&self) -> TranslationPlanProvenance {
        self.provenance
    }

    /// Returns the one or two model legs in execution order.
    pub fn legs(&self) -> &[TranslationLeg] {
        &self.legs
    }
}

// This conservative registry contains dedicated pair repositories whose Marian
// files were validated for the native resolver. Portuguese and Polish use
// publisher-documented multilingual models only for their required English
// bridge directions. Missing non-English pairs deliberately pivot via English.
fn validated_direct_model_id(
    source: CuratedLanguage,
    target: CuratedLanguage,
) -> Option<&'static str> {
    use CuratedLanguage::{Dutch, English, French, German, Italian, Polish, Portuguese, Spanish};

    match (source, target) {
        (German, English) => Some("Helsinki-NLP/opus-mt-de-en"),
        (German, Spanish) => Some("Helsinki-NLP/opus-mt-de-es"),
        (German, French) => Some("Helsinki-NLP/opus-mt-de-fr"),
        (German, Italian) => Some("Helsinki-NLP/opus-mt-de-it"),
        (German, Dutch) => Some("Helsinki-NLP/opus-mt-de-nl"),
        (German, Polish) => Some("Helsinki-NLP/opus-mt-de-pl"),
        (English, German) => Some("Helsinki-NLP/opus-mt-en-de"),
        (English, Spanish) => Some("Helsinki-NLP/opus-mt-en-es"),
        (English, French) => Some("Helsinki-NLP/opus-mt-en-fr"),
        (English, Italian) => Some("Helsinki-NLP/opus-mt-en-it"),
        (English, Portuguese) => Some("Helsinki-NLP/opus-mt-tc-big-en-pt"),
        (English, Dutch) => Some("Helsinki-NLP/opus-mt-en-nl"),
        (English, Polish) => Some("Helsinki-NLP/opus-mt-en-zlw"),
        (Spanish, German) => Some("Helsinki-NLP/opus-mt-es-de"),
        (Spanish, English) => Some("Helsinki-NLP/opus-mt-es-en"),
        (Spanish, French) => Some("Helsinki-NLP/opus-mt-es-fr"),
        (Spanish, Italian) => Some("Helsinki-NLP/opus-mt-es-it"),
        (Spanish, Dutch) => Some("Helsinki-NLP/opus-mt-es-nl"),
        (Spanish, Polish) => Some("Helsinki-NLP/opus-mt-es-pl"),
        (French, German) => Some("Helsinki-NLP/opus-mt-fr-de"),
        (French, English) => Some("Helsinki-NLP/opus-mt-fr-en"),
        (French, Spanish) => Some("Helsinki-NLP/opus-mt-fr-es"),
        (French, Polish) => Some("Helsinki-NLP/opus-mt-fr-pl"),
        (Italian, German) => Some("Helsinki-NLP/opus-mt-it-de"),
        (Italian, English) => Some("Helsinki-NLP/opus-mt-it-en"),
        (Italian, Spanish) => Some("Helsinki-NLP/opus-mt-it-es"),
        (Italian, French) => Some("Helsinki-NLP/opus-mt-it-fr"),
        (Portuguese, English) => Some("Helsinki-NLP/opus-mt-ROMANCE-en"),
        (Dutch, English) => Some("Helsinki-NLP/opus-mt-nl-en"),
        (Dutch, Spanish) => Some("Helsinki-NLP/opus-mt-nl-es"),
        (Dutch, French) => Some("Helsinki-NLP/opus-mt-nl-fr"),
        (Polish, German) => Some("Helsinki-NLP/opus-mt-pl-de"),
        (Polish, English) => Some("Helsinki-NLP/opus-mt-pl-en"),
        (Polish, Spanish) => Some("Helsinki-NLP/opus-mt-pl-es"),
        (Polish, French) => Some("Helsinki-NLP/opus-mt-pl-fr"),
        _ => None,
    }
}

/// Validation failures returned before any translation model is resolved.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TranslationPlanError {
    #[error("unsupported translation language `{language}`")]
    UnsupportedLanguage { language: String },
    #[error("source and target translation language are both `{language}`")]
    SameLanguage { language: CuratedLanguage },
}
