//! Optional native translation configuration applied after ASR.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Library-owned controls for post-ASR translation model resolution.
///
/// These controls intentionally do not depend on CLI argument types. Embedding
/// applications can select an explicit bundle, an application model directory,
/// or cache-only resolution without constructing a command-line configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationConfig {
    /// Enables post-ASR translation for finite Workflow Composition.
    #[serde(default)]
    pub enabled: bool,
    /// Hugging Face model ID used by the legacy single-model translation path.
    #[serde(default)]
    pub model_id: Option<String>,
    /// Explicit local Marian model bundle, preferred over cache resolution.
    #[serde(default)]
    pub model_bundle: Option<PathBuf>,
    /// Application-selected model/cache root used for translation resolution.
    #[serde(default)]
    pub model_dir: Option<PathBuf>,
    /// Forbids translation model downloads and fails when local assets are absent.
    #[serde(default)]
    pub model_cache_only: bool,
    /// ISO 639-1 source language code for the configured model.
    #[serde(default)]
    pub source_language: Option<String>,
    /// ISO 639-1 target language code for the configured model.
    #[serde(default)]
    pub target_language: Option<String>,
    /// Maximum number of tokens produced for each translated segment.
    #[serde(default = "default_translation_max_new_tokens")]
    pub max_new_tokens: usize,
}

/// Configuration for the public native Marian/OPUS-MT plan executor.
///
/// Unlike [`TranslationConfig`], this type is not tied to the legacy
/// single-model post-ASR workflow. Each leg's canonical model ID comes from a
/// public [`crate::TranslationPlan`]. Models are resolved lazily and reused by
/// the provider for later segments or plans.
#[cfg(feature = "translation")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeOpusMtTranslationProviderConfig {
    /// Application-selected Hugging Face model/cache root.
    #[serde(default)]
    pub model_dir: Option<PathBuf>,
    /// Forbids downloads and fails when a planned model is absent locally.
    #[serde(default)]
    pub model_cache_only: bool,
    /// Candle device preference used for every model loaded by the provider.
    #[serde(default)]
    pub device: super::DevicePreference,
    /// Maximum number of tokens produced for each translated segment.
    #[serde(default = "default_translation_max_new_tokens")]
    pub max_new_tokens: usize,
}

#[cfg(feature = "translation")]
impl Default for NativeOpusMtTranslationProviderConfig {
    fn default() -> Self {
        Self {
            model_dir: None,
            model_cache_only: false,
            device: super::DevicePreference::Auto,
            max_new_tokens: default_translation_max_new_tokens(),
        }
    }
}

impl Default for TranslationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            model_id: None,
            model_bundle: None,
            model_dir: None,
            model_cache_only: false,
            source_language: None,
            target_language: None,
            max_new_tokens: default_translation_max_new_tokens(),
        }
    }
}

fn default_translation_max_new_tokens() -> usize {
    256
}
