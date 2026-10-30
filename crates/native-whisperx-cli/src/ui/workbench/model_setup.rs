//! Session-owned transport for browser-prepared model resources. Model semantics
//! and all integrity/provenance checks remain with the existing providers.

use std::collections::BTreeSet;
use std::io::{Cursor, Read};
use std::path::Path;

use anyhow::{bail, Context};
use serde_json::{json, Value};

const FILES: [&str; 8] = [
    "segmentation.onnx",
    "embedding.onnx",
    "plda_transform.json",
    "plda_model.json",
    "clustering.json",
    "MODEL_PROVENANCE.md",
    "LICENSE.md",
    "pyannote_diarization_manifest.json",
];

pub(super) fn install(session: &super::WorkbenchSession, body: &[u8]) -> anyhow::Result<Value> {
    session.ensure_idle()?;
    if !cfg!(all(
        feature = "pyannote-vad",
        feature = "pyannote-diarization"
    )) {
        bail!("pyannote model setup is unavailable in this build");
    }
    let mut root = session.root().to_path_buf();
    for component in [".native-whisperx", "model-bundles"] {
        root = session.authorized_new_path(&root.join(component))?;
        std::fs::create_dir_all(&root)?;
        root = session.authorized_path(&root)?;
    }
    let staging = tempfile::Builder::new()
        .prefix(".community-")
        .tempdir_in(&root)?;
    let mut archive =
        zip::ZipArchive::new(Cursor::new(body)).context("invalid model bundle archive")?;
    if archive.len() != FILES.len() {
        bail!("model bundle must contain exactly eight approved files");
    }
    let mut seen = BTreeSet::new();
    let mut total = 0_u64;
    for index in 0..archive.len() {
        let entry = archive.by_index(index)?;
        let name = entry.name()?.into_owned();
        if !FILES.contains(&name.as_str()) || !seen.insert(name.clone()) {
            bail!("unexpected model bundle file");
        }
        if entry.compression() != zip::CompressionMethod::Stored || entry.size() > 32 * 1024 * 1024
        {
            bail!("unsupported model bundle entry size or encoding");
        }
        total = total
            .checked_add(entry.size())
            .context("model bundle is too large")?;
        if total > 40 * 1024 * 1024 {
            bail!("model bundle is too large");
        }
        let expected = entry.size();
        let mut output = std::fs::File::create(staging.path().join(&name))?;
        if std::io::copy(&mut entry.take(expected + 1), &mut output)? != expected {
            bail!("incomplete model bundle entry");
        }
    }
    let report = native_whisperx::verify_pyannote_diarization_bundle(staging.path())?;
    native_whisperx::verify_pyannote_vad_bundle(staging.path())?;
    let destination = root.join(format!("sha256-{}", report.artifact_set_sha256));
    if let Ok(metadata) = std::fs::symlink_metadata(&destination) {
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            bail!("model cache destination must be a directory");
        }
        let existing = native_whisperx::verify_pyannote_diarization_bundle(&destination)
            .and_then(|_| native_whisperx::verify_pyannote_vad_bundle(&destination));
        if existing.is_err() {
            let quarantine = root.join(format!(
                "invalid-{}",
                staging
                    .path()
                    .file_name()
                    .and_then(|name| name.to_str())
                    .context("invalid cache staging path")?
            ));
            std::fs::rename(&destination, quarantine)?;
            std::fs::rename(staging.path(), &destination)?;
        }
    } else {
        std::fs::rename(staging.path(), &destination)?;
    }
    ready(&destination)
}

fn ready(path: &Path) -> anyhow::Result<Value> {
    let report = native_whisperx::verify_pyannote_diarization_bundle(path)?;
    native_whisperx::verify_pyannote_vad_bundle(path)?;
    Ok(
        json!({"bundlePath":path,"vadBundlePath":path,"modelId":report.model_id,
        "sourceRevision":report.source_revision,"artifactSetSha256":report.artifact_set_sha256}),
    )
}

#[cfg(all(test, feature = "pyannote-vad", feature = "pyannote-diarization"))]
mod tests {
    use super::*;
    use std::io::{Cursor, Write};

    fn archive(first: &str) -> Vec<u8> {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for name in [
            first,
            "embedding.onnx",
            "plda_transform.json",
            "plda_model.json",
            "clustering.json",
            "MODEL_PROVENANCE.md",
            "LICENSE.md",
            "pyannote_diarization_manifest.json",
        ] {
            writer
                .start_file(
                    name,
                    zip::write::SimpleFileOptions::default()
                        .compression_method(zip::CompressionMethod::Stored),
                )
                .unwrap();
            writer.write_all(b"invalid model payload").unwrap();
        }
        writer.finish().unwrap().into_inner()
    }

    #[test]
    fn upload_rejects_paths_and_does_not_persist_unverified_models() {
        let root = tempfile::tempdir().unwrap();
        let session =
            super::super::WorkbenchSession::new(root.path().into(), root.path().join("speakers"))
                .unwrap();
        for name in ["../escape.onnx", "/absolute.onnx", "unexpected.onnx"] {
            let error = install(&session, &archive(name)).unwrap_err();
            assert!(
                error.to_string().contains("unexpected model bundle file"),
                "{error}"
            );
        }
        assert!(!root.path().join("escape.onnx").exists());
        assert_eq!(
            std::fs::read_dir(root.path().join(".native-whisperx/model-bundles"))
                .unwrap()
                .count(),
            0
        );
    }

    #[test]
    fn missing_model_provenance_fails_instead_of_becoming_ready() {
        let root = tempfile::tempdir().unwrap();
        let session =
            super::super::WorkbenchSession::new(root.path().into(), root.path().join("speakers"))
                .unwrap();
        assert!(install(&session, &archive("segmentation.onnx")).is_err());
        assert_eq!(
            std::fs::read_dir(root.path().join(".native-whisperx/model-bundles"))
                .unwrap()
                .count(),
            0
        );
    }
}
