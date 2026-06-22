//! Optional native translation configuration applied after ASR.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranslationConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub model_id: Option<String>,
    #[serde(default)]
    pub model_bundle: Option<PathBuf>,
    #[serde(default)]
    pub model_dir: Option<PathBuf>,
    #[serde(default)]
    pub model_cache_only: bool,
    #[serde(default)]
    pub source_language: Option<String>,
    #[serde(default)]
    pub target_language: Option<String>,
    #[serde(default = "default_translation_max_new_tokens")]
    pub max_new_tokens: usize,
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
