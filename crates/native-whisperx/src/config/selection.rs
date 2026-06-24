//! Shared configuration selection metadata for automatic Workflow Composition.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigSelection {
    Automatic,
    #[default]
    Explicit,
}

impl ConfigSelection {
    pub fn is_explicit(&self) -> bool {
        matches!(self, Self::Explicit)
    }

    pub fn is_automatic(&self) -> bool {
        matches!(self, Self::Automatic)
    }
}
