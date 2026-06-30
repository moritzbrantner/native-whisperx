use std::path::{Path, PathBuf};

use video_analysis_core::Result;

use crate::setup_error;

pub(crate) fn resolve_required_bundle_file(bundle: &Path, file: &str) -> Result<PathBuf> {
    if !bundle.exists() {
        return Err(setup_error(format!(
            "required model bundle `{}` is missing",
            bundle.display()
        )));
    }

    let direct = bundle.join(file);
    if direct.exists() {
        return Ok(direct);
    }

    let files_dir = bundle.join("files").join(file);
    if files_dir.exists() {
        return Ok(files_dir);
    }

    #[cfg(feature = "model-bundles")]
    {
        let manifest = bundle.join("manifest.json");
        if manifest.exists() {
            let loaded = model_runtime::ModelBundle::load(&manifest).map_err(|error| {
                crate::invalid_request(format!(
                    "failed to parse model bundle manifest `{}`: {error}",
                    manifest.display()
                ))
            })?;
            for model_file in loaded.manifest.files.values() {
                if model_file.remote_path == file || model_file.local_path.ends_with(file) {
                    if let Some(path) = loaded.file_path(&model_file.remote_path) {
                        if path.exists() {
                            return Ok(path);
                        }
                    }
                }
            }
        }
    }

    Err(setup_error(format!(
        "required model bundle file `{file}` is missing in `{}`",
        bundle.display()
    )))
}

#[allow(dead_code)]
pub(crate) fn resolve_optional_bundle_file(bundle: &Path, file: &str) -> Result<Option<PathBuf>> {
    match resolve_required_bundle_file(bundle, file) {
        Ok(path) => Ok(Some(path)),
        Err(error) if error.to_string().contains("required model bundle file") => Ok(None),
        Err(error) => Err(error),
    }
}

pub(crate) fn validate_required_bundle_files(bundle: &Path, files: &[&str]) -> Result<()> {
    for file in files {
        resolve_required_bundle_file(bundle, file)?;
    }
    Ok(())
}
