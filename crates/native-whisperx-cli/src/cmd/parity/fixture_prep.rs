//! Fixture preparation for CLI runs and ONNX Runtime dylib discovery.

use super::*;

pub(super) fn prepare_fixture_for_cli_run(
    fixture: &mut ParityFixtureCase,
    root: &Path,
    whisperx_command: Option<&PathBuf>,
    model_dir: &Path,
    model_cache_only: bool,
) {
    fixture.input = resolve_cli_path_with_root(fixture.input.clone(), root);
    if let Some(command) = whisperx_command {
        fixture.whisperx.command = command.clone();
    }
    fixture.whisperx.command_wrapper = fixture
        .whisperx
        .command_wrapper
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.native_asr.whisper_bundle = fixture
        .native_asr
        .whisper_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.alignment.model_bundle = fixture
        .alignment
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.translation.model_bundle = fixture
        .translation
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    apply_fixture_model_options(fixture, Some(model_dir), model_cache_only);
    fixture.vad.model_bundle = fixture
        .vad
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.diarization.model_bundle = fixture
        .diarization
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.diarization.speaker_embedding_model_bundle = fixture
        .diarization
        .speaker_embedding_model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    if fixture.output.output_dir.is_none() {
        fixture.output.output_dir = Some(root.join("out").join("parity-bench").join(&fixture.name));
    }
}

pub(super) fn apply_fixture_model_options(
    fixture: &mut ParityFixtureCase,
    model_dir: Option<&Path>,
    model_cache_only: bool,
) {
    if let Some(model_dir) = model_dir {
        fixture.native_asr.model_dir = Some(model_dir.to_path_buf());
        fixture.alignment.model_dir = Some(model_dir.to_path_buf());
        fixture.translation.model_dir = Some(model_dir.to_path_buf());
    }
    if model_cache_only {
        fixture.native_asr.model_cache_only = true;
        fixture.alignment.model_cache_only = true;
        fixture.translation.model_cache_only = true;
    }
}

pub(super) fn prepare_multi_input_fixture_for_cli_run(
    fixture: &mut ParityMultiInputFixtureCase,
    root: &Path,
    whisperx_command: Option<&PathBuf>,
    model_dir: &Path,
    model_cache_only: bool,
) -> anyhow::Result<()> {
    if fixture.inputs.is_empty() {
        anyhow::bail!(
            "parity benchmark multi-input case `{}` must contain at least one input",
            fixture.name
        );
    }
    if fixture.output.basename.is_some() {
        anyhow::bail!(
            "parity benchmark multi-input case `{}` cannot set output.basename",
            fixture.name
        );
    }
    fixture.inputs = fixture
        .inputs
        .iter()
        .cloned()
        .map(|input| resolve_cli_path_with_root(input, root))
        .collect();
    if let Some(command) = whisperx_command {
        fixture.whisperx.command = command.clone();
    }
    fixture.whisperx.command_wrapper = fixture
        .whisperx
        .command_wrapper
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.native_asr.whisper_bundle = fixture
        .native_asr
        .whisper_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.native_asr.model_dir = Some(model_dir.to_path_buf());
    fixture.alignment.model_bundle = fixture
        .alignment
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.alignment.model_dir = Some(model_dir.to_path_buf());
    fixture.translation.model_bundle = fixture
        .translation
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.translation.model_dir = Some(model_dir.to_path_buf());
    fixture.vad.model_bundle = fixture
        .vad
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.diarization.model_bundle = fixture
        .diarization
        .model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    fixture.diarization.speaker_embedding_model_bundle = fixture
        .diarization
        .speaker_embedding_model_bundle
        .take()
        .map(|path| resolve_cli_path_with_root(path, root));
    if model_cache_only {
        fixture.native_asr.model_cache_only = true;
        fixture.alignment.model_cache_only = true;
        fixture.translation.model_cache_only = true;
    }
    if fixture.output.output_dir.is_none() {
        fixture.output.output_dir = Some(root.join("out").join("parity-bench").join(&fixture.name));
    }
    Ok(())
}

pub(super) fn set_ort_dylib_path_from_fixture_if_missing(fixture: &ParityFixtureCase) {
    if std::env::var_os("ORT_DYLIB_PATH").is_some() {
        return;
    }
    let Some(path) = inferred_ort_dylib_path(fixture) else {
        return;
    };
    std::env::set_var("ORT_DYLIB_PATH", path);
}

pub(super) fn set_ort_dylib_path_from_multi_input_fixture_if_missing(
    fixture: &ParityMultiInputFixtureCase,
) {
    if std::env::var_os("ORT_DYLIB_PATH").is_some() {
        return;
    }
    let Some(path) =
        inferred_ort_dylib_path_from_parts(&fixture.vad, &fixture.diarization, &fixture.whisperx)
    else {
        return;
    };
    std::env::set_var("ORT_DYLIB_PATH", path);
}

pub(super) fn inferred_ort_dylib_path(fixture: &ParityFixtureCase) -> Option<PathBuf> {
    inferred_ort_dylib_path_with_env(fixture, std::env::var_os("ORT_DYLIB_PATH"))
}

pub(super) fn inferred_ort_dylib_path_with_env(
    fixture: &ParityFixtureCase,
    ort_dylib_path: Option<OsString>,
) -> Option<PathBuf> {
    if ort_dylib_path.is_some() {
        return None;
    }
    inferred_ort_dylib_path_from_parts(&fixture.vad, &fixture.diarization, &fixture.whisperx)
}

pub(super) fn inferred_ort_dylib_path_from_parts(
    vad: &VadConfig,
    diarization: &DiarizationConfig,
    whisperx: &WhisperxOracleConfig,
) -> Option<PathBuf> {
    let uses_native_onnx_vad = matches!(vad.method, VadMethod::Silero | VadMethod::Pyannote)
        || (vad.selection.is_automatic() && diarization.enabled);
    if !uses_native_onnx_vad {
        return None;
    }
    let env_root = whisperx.command.parent()?.parent()?;
    find_onnxruntime_dylib(env_root)
}

pub(super) fn find_onnxruntime_dylib(env_root: &Path) -> Option<PathBuf> {
    let lib_dir = env_root.join("lib");
    let mut candidates = Vec::new();
    for python_dir in fs::read_dir(&lib_dir).ok()?.filter_map(Result::ok) {
        let file_name = python_dir.file_name();
        if !file_name.to_string_lossy().starts_with("python") {
            continue;
        }
        let capi_dir = python_dir
            .path()
            .join("site-packages")
            .join("onnxruntime")
            .join("capi");
        let Ok(entries) = fs::read_dir(capi_dir) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if name.starts_with("libonnxruntime.so") || name.starts_with("libonnxruntime.dylib") {
                candidates.push(path);
            }
        }
    }
    candidates.sort();
    candidates.into_iter().next()
}
