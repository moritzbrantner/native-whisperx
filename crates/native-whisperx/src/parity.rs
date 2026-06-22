//! Parity harness execution and structured comparison against Python WhisperX.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use audio_analysis_transcription::SpeechActivitySegment;
use text_transcripts::TranscriptionContract;

use crate::config::{
    default_whisperx_command, AlignmentConfig, AsrConfig, AsrProvider, DiarizationConfig,
    ExpectedOutputComparison, ExpectedTranscriptTarget, ExternalWhisperxConfig, InputSource,
    NativeWhisperxConfig, NativeWhisperxError, OutputConfig, ParityComparison,
    ParityComparisonConfig, ParityConfig, ParityFixtureCase, ParityFixtureCaseReport,
    ParityFixtureSuite, ParityFixtureSuiteReport, ParityPreflightCaseReport, ParityPreflightReport,
    ParityReport, ParityTolerance, TranslationConfig, VadConfig, VadMethod,
};
use crate::output::{compare_expected_outputs, normalize_space};
use crate::{import_whisperx_json, run};

pub fn compare_with_whisperx(config: ParityConfig) -> Result<ParityReport, NativeWhisperxError> {
    let mut native_asr = config.native_asr;
    native_asr.provider = AsrProvider::Native;
    native_asr.language = config.language.clone();
    let external_task = native_asr.task;
    let translation = config.translation;
    let alignment = config.alignment;
    let vad = config.vad;
    let diarization = config.diarization;
    let whisperx_diarization = config
        .whisperx_diarization
        .unwrap_or_else(|| diarization.clone());

    let native_report = run(NativeWhisperxConfig {
        input: InputSource::Path {
            path: config.input.clone(),
        },
        asr: native_asr,
        translation,
        vad: vad.clone(),
        alignment: alignment.clone(),
        diarization: diarization.clone(),
        output: config.output.clone(),
    })?;

    let whisperx_report = run(NativeWhisperxConfig {
        input: InputSource::Path { path: config.input },
        asr: AsrConfig {
            provider: AsrProvider::ExternalWhisperX,
            task: external_task,
            language: config.language,
            external_whisperx: config.whisperx,
            ..AsrConfig::default()
        },
        translation: TranslationConfig::default(),
        vad,
        alignment,
        diarization: whisperx_diarization,
        output: config.output,
    })?;

    let expected = config
        .expected_json
        .map(|path| fs::read(path).map_err(NativeWhisperxError::Io))
        .transpose()?
        .map(|bytes| import_whisperx_json(&bytes))
        .transpose()?;

    let mut comparison = compare_transcripts(
        &native_report.response.transcript,
        &whisperx_report.response.transcript,
        ParityTolerance::default(),
        &config.comparison,
    );
    comparison.diagnostic_differences = compare_diagnostics(
        &native_report.response.diagnostics,
        &whisperx_report.response.diagnostics,
    );
    compare_vad_segments(
        &native_report.response.vad_segments,
        &whisperx_report.response.vad_segments,
        ParityTolerance::default(),
        &config.comparison,
        &mut comparison,
    );

    let (expected_segment_count_matches, expected_text_matches) = expected_transcript_matches(
        expected.as_ref(),
        config.expected_target,
        &native_report.response.transcript,
        &whisperx_report.response.transcript,
    );

    Ok(ParityReport {
        native_report,
        whisperx_report,
        expected,
        expected_target: config.expected_target,
        comparison,
        expected_segment_count_matches,
        expected_text_matches,
    })
}

pub(crate) fn expected_transcript_matches(
    expected: Option<&TranscriptionContract>,
    expected_target: ExpectedTranscriptTarget,
    native_transcript: &TranscriptionContract,
    whisperx_transcript: &TranscriptionContract,
) -> (Option<bool>, Option<bool>) {
    let Some(expected) = expected else {
        return (None, None);
    };
    let comparison_transcript = match expected_target {
        ExpectedTranscriptTarget::Native => native_transcript,
        ExpectedTranscriptTarget::Whisperx => whisperx_transcript,
    };
    (
        Some(expected.segments.len() == comparison_transcript.segments.len()),
        Some(
            normalize_space(&expected.text_or_joined())
                == normalize_space(&comparison_transcript.text_or_joined()),
        ),
    )
}

pub fn run_parity_fixture_suite(
    suite: ParityFixtureSuite,
    root: Option<&Path>,
) -> Result<ParityFixtureSuiteReport, NativeWhisperxError> {
    run_parity_fixture_suite_with_runner(suite, root, compare_with_whisperx)
}

pub(crate) fn run_parity_fixture_suite_with_runner<F>(
    suite: ParityFixtureSuite,
    root: Option<&Path>,
    mut runner: F,
) -> Result<ParityFixtureSuiteReport, NativeWhisperxError>
where
    F: FnMut(ParityConfig) -> Result<ParityReport, NativeWhisperxError>,
{
    let mut cases = Vec::with_capacity(suite.fixtures.len());

    for fixture in suite.fixtures {
        let fixture = resolve_fixture_case_paths(fixture, root);
        let name = fixture.name;
        let gating = fixture.gating;
        let required_diagnostics = fixture.required_diagnostics;
        let expected_outputs = fixture.expected_outputs;
        let case_result = runner(ParityConfig {
            input: fixture.input,
            expected_json: fixture.expected_json,
            expected_target: fixture.expected_target,
            comparison: fixture.comparison,
            native_asr: fixture.native_asr,
            translation: fixture.translation,
            vad: fixture.vad,
            alignment: fixture.alignment,
            diarization: fixture.diarization,
            whisperx_diarization: fixture.whisperx_diarization,
            whisperx: fixture.whisperx,
            language: fixture.language,
            output: fixture.output,
        })
        .and_then(|report| {
            let missing_required_diagnostics =
                missing_required_diagnostics(&report, &required_diagnostics);
            let expected_output_matches =
                compare_expected_outputs(&report.native_report.output_files, &expected_outputs)?;
            let passed = parity_fixture_case_passed(
                &report,
                &missing_required_diagnostics,
                &expected_output_matches,
            );
            let failure_summary = parity_fixture_failure_summary(
                Some(&report),
                &missing_required_diagnostics,
                &expected_output_matches,
                None,
            );
            Ok(ParityFixtureCaseReport {
                name: name.clone(),
                gating,
                passed,
                started_at: None,
                elapsed_seconds: None,
                timed_out: false,
                report: Some(report),
                missing_required_diagnostics,
                expected_output_matches,
                error: None,
                failure_summary,
            })
        });

        match case_result {
            Ok(case) => cases.push(case),
            Err(error) => {
                let error = error.to_string();
                cases.push(ParityFixtureCaseReport {
                    name,
                    gating,
                    passed: false,
                    started_at: None,
                    elapsed_seconds: None,
                    timed_out: false,
                    report: None,
                    missing_required_diagnostics: Vec::new(),
                    expected_output_matches: Vec::new(),
                    failure_summary: parity_fixture_failure_summary(None, &[], &[], Some(&error)),
                    error: Some(error),
                });
            }
        }
    }

    let passed = cases
        .iter()
        .filter(|case| case.gating)
        .all(|case| case.passed);
    Ok(ParityFixtureSuiteReport { passed, cases })
}

pub(crate) fn parity_fixture_failure_summary(
    report: Option<&ParityReport>,
    missing_required_diagnostics: &[String],
    expected_output_matches: &[ExpectedOutputComparison],
    error: Option<&str>,
) -> Vec<String> {
    let mut summary = Vec::new();
    if let Some(report) = report {
        summary.extend(report.comparison.differences.iter().cloned());
        summary.extend(report.comparison.diagnostic_differences.iter().cloned());
        if report.expected_text_matches == Some(false) {
            summary.push("expected transcript text differs".to_string());
        }
        if report.expected_segment_count_matches == Some(false) {
            summary.push("expected transcript segment count differs".to_string());
        }
    }
    summary.extend(
        missing_required_diagnostics
            .iter()
            .map(|diagnostic| format!("missing required diagnostic: {diagnostic}")),
    );
    summary.extend(
        expected_output_matches
            .iter()
            .filter(|output| !output.passed)
            .filter_map(|output| {
                output
                    .difference
                    .as_ref()
                    .map(|difference| format!("{:?} output: {difference}", output.format))
            }),
    );
    if let Some(error) = error {
        summary.push(error.to_string());
    }
    summary
}

pub fn run_parity_preflight(
    suite: ParityFixtureSuite,
    manifest: PathBuf,
    root: PathBuf,
    whisperx_command: PathBuf,
    model_dir: PathBuf,
    require_expected: bool,
    include_non_gating: bool,
) -> ParityPreflightReport {
    let source_checkout_tag = whisperx_source_checkout_tag();
    let source_checkout_ok = source_checkout_tag.as_deref() == Some("v3.8.6");
    let whisperx_version_result = check_whisperx_version(&whisperx_command);
    let model_dir_ok = model_dir.exists();

    let mut cases = Vec::with_capacity(suite.fixtures.len());
    for fixture in suite.fixtures {
        let fixture = resolve_fixture_case_paths(fixture, Some(&root));
        let enforce = fixture.gating || include_non_gating;
        let mut missing = Vec::new();
        let mut warnings = Vec::new();

        push_preflight_check(
            enforce,
            &mut missing,
            &mut warnings,
            source_checkout_ok,
            || match source_checkout_tag.as_deref() {
                Some(tag) => {
                    format!(".audio-tools/whisperx-src is not exact tag v3.8.6 (found {tag})")
                }
                None => ".audio-tools/whisperx-src is missing or not at an exact tag".to_string(),
            },
        );
        push_preflight_check(
            enforce,
            &mut missing,
            &mut warnings,
            whisperx_version_result.is_ok(),
            || {
                whisperx_version_result
                    .as_ref()
                    .err()
                    .cloned()
                    .unwrap_or_else(|| "whisperx command failed --version".to_string())
            },
        );
        push_preflight_check(enforce, &mut missing, &mut warnings, model_dir_ok, || {
            format!("model directory {} does not exist", model_dir.display())
        });
        push_preflight_check(
            enforce,
            &mut missing,
            &mut warnings,
            fixture.input.exists(),
            || format!("input {} does not exist", fixture.input.display()),
        );

        if require_expected {
            if let Some(expected_json) = &fixture.expected_json {
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    expected_json.exists(),
                    || format!("expected JSON {} does not exist", expected_json.display()),
                );
            }
            for expected_output in &fixture.expected_outputs {
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    expected_output.path.exists(),
                    || {
                        format!(
                            "expected {:?} output {} does not exist",
                            expected_output.format,
                            expected_output.path.display()
                        )
                    },
                );
            }
        }

        for env_name in preflight_required_hf_token_envs(&fixture) {
            push_preflight_check(
                enforce,
                &mut missing,
                &mut warnings,
                std::env::var_os(env_name).is_some(),
                || format!("environment variable {env_name} is not set"),
            );
        }

        if fixture.translation.enabled {
            if let Some(model_bundle) = &fixture.translation.model_bundle {
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    model_bundle.exists(),
                    || {
                        format!(
                            "translation bundle {} does not exist",
                            model_bundle.display()
                        )
                    },
                );
            }
        }

        if preflight_case_needs_onnx_runtime(&fixture) {
            push_preflight_check(
                enforce,
                &mut missing,
                &mut warnings,
                env_path_exists("ORT_DYLIB_PATH"),
                || "ORT_DYLIB_PATH is not set to an existing file".to_string(),
            );
        }

        if fixture.vad.enabled
            && matches!(fixture.vad.method, VadMethod::Silero | VadMethod::Pyannote)
        {
            if let Some(model_bundle) = &fixture.vad.model_bundle {
                let vad_label = match fixture.vad.method {
                    VadMethod::Silero => "Silero",
                    VadMethod::Pyannote => "pyannote",
                    VadMethod::Energy => "energy",
                };
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    model_bundle.exists(),
                    || {
                        format!(
                            "{vad_label} VAD bundle {} does not exist",
                            model_bundle.display()
                        )
                    },
                );
                let model_file =
                    fixture
                        .vad
                        .model_file
                        .as_deref()
                        .unwrap_or(match fixture.vad.method {
                            VadMethod::Silero => "silero_vad.onnx",
                            VadMethod::Pyannote => "segmentation.onnx",
                            VadMethod::Energy => "",
                        });
                if model_bundle.is_dir() || fixture.vad.model_file.is_some() {
                    let model_path = model_bundle.join(model_file);
                    push_preflight_check(
                        enforce,
                        &mut missing,
                        &mut warnings,
                        model_path.exists(),
                        || {
                            format!(
                                "{vad_label} VAD model {} does not exist",
                                model_path.display()
                            )
                        },
                    );
                }
            } else {
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    false,
                    || match fixture.vad.method {
                        VadMethod::Silero => "Silero VAD modelBundle is not set".to_string(),
                        VadMethod::Pyannote => "pyannote VAD modelBundle is not set".to_string(),
                        VadMethod::Energy => "energy VAD modelBundle is not set".to_string(),
                    },
                );
            }
        }

        if let Some(model_bundle) = &fixture.diarization.speaker_embedding_model_bundle {
            push_preflight_check(
                enforce,
                &mut missing,
                &mut warnings,
                model_bundle.exists(),
                || {
                    format!(
                        "speaker embedding bundle {} does not exist",
                        model_bundle.display()
                    )
                },
            );
            if let Some(model_file) = &fixture.diarization.speaker_embedding_model_file {
                let model_path = model_bundle.join(model_file);
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    model_path.exists(),
                    || {
                        format!(
                            "speaker embedding model {} does not exist",
                            model_path.display()
                        )
                    },
                );
            }
        }
        if let Some(model_bundle) = &fixture.diarization.model_bundle {
            push_preflight_check(
                enforce,
                &mut missing,
                &mut warnings,
                model_bundle.exists(),
                || {
                    format!(
                        "diarization model bundle {} does not exist",
                        model_bundle.display()
                    )
                },
            );
            for (label, file) in [
                (
                    "diarization manifest",
                    fixture
                        .diarization
                        .manifest_file
                        .as_deref()
                        .unwrap_or("pyannote_diarization_manifest.json"),
                ),
                (
                    "diarization segmentation model",
                    fixture
                        .diarization
                        .segmentation_model_file
                        .as_deref()
                        .unwrap_or("segmentation.onnx"),
                ),
                (
                    "diarization embedding model",
                    fixture
                        .diarization
                        .embedding_model_file
                        .as_deref()
                        .unwrap_or("embedding.onnx"),
                ),
                (
                    "diarization PLDA transform",
                    fixture
                        .diarization
                        .plda_transform_file
                        .as_deref()
                        .unwrap_or("plda_transform.json"),
                ),
                (
                    "diarization PLDA model",
                    fixture
                        .diarization
                        .plda_model_file
                        .as_deref()
                        .unwrap_or("plda_model.json"),
                ),
                (
                    "diarization clustering config",
                    fixture
                        .diarization
                        .clustering_config_file
                        .as_deref()
                        .unwrap_or("clustering.json"),
                ),
            ] {
                let artifact = model_bundle.join(file);
                push_preflight_check(
                    enforce,
                    &mut missing,
                    &mut warnings,
                    artifact.exists(),
                    || format!("{label} {} does not exist", artifact.display()),
                );
            }
        }

        cases.push(ParityPreflightCaseReport {
            name: fixture.name,
            gating: fixture.gating,
            passed: missing.is_empty(),
            missing,
            warnings,
        });
    }

    let passed = cases.iter().all(|case| case.passed);
    ParityPreflightReport {
        passed,
        manifest,
        root,
        whisperx_command,
        model_dir,
        source_checkout_tag,
        cases,
    }
}

fn preflight_required_hf_token_envs(fixture: &ParityFixtureCase) -> Vec<&str> {
    let mut envs = Vec::new();
    push_diarization_hf_token_env(&mut envs, &fixture.diarization);
    if let Some(diarization) = &fixture.whisperx_diarization {
        push_diarization_hf_token_env(&mut envs, diarization);
        if diarization.enabled && diarization.hf_token.is_none() {
            if let Some(env) = fixture.whisperx.hf_token_env.as_deref() {
                envs.push(env);
            }
        }
    } else if fixture.whisperx.diarize {
        if let Some(env) = fixture.whisperx.hf_token_env.as_deref() {
            envs.push(env);
        }
    }
    envs.sort_unstable();
    envs.dedup();
    envs
}

fn push_diarization_hf_token_env<'a>(envs: &mut Vec<&'a str>, diarization: &'a DiarizationConfig) {
    if diarization.enabled && diarization.hf_token.is_none() {
        if let Some(env) = diarization.hf_token_env.as_deref() {
            envs.push(env);
        }
    }
}

fn preflight_case_needs_onnx_runtime(fixture: &ParityFixtureCase) -> bool {
    let vad_uses_onnx = fixture.vad.enabled
        && matches!(fixture.vad.method, VadMethod::Silero | VadMethod::Pyannote);
    let diarization_uses_onnx = fixture.diarization.enabled
        && (fixture.diarization.model_bundle.is_some()
            || fixture.diarization.speaker_embedding_model_bundle.is_some());
    vad_uses_onnx || diarization_uses_onnx
}

fn push_preflight_check<F>(
    enforce: bool,
    missing: &mut Vec<String>,
    warnings: &mut Vec<String>,
    passed: bool,
    message: F,
) where
    F: FnOnce() -> String,
{
    if passed {
        return;
    }
    if enforce {
        missing.push(message());
    } else {
        warnings.push(message());
    }
}

fn whisperx_source_checkout_tag() -> Option<String> {
    let output = Command::new("git")
        .args([
            "-C",
            ".audio-tools/whisperx-src",
            "describe",
            "--tags",
            "--exact-match",
            "HEAD",
        ])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn check_whisperx_version(command: &Path) -> Result<(), String> {
    if !command.exists() {
        return Err(format!(
            "whisperx command {} does not exist",
            command.display()
        ));
    }
    let output = Command::new(command)
        .arg("--version")
        .output()
        .map_err(|error| format!("failed to run {} --version: {error}", command.display()))?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(format!(
            "{} --version exited with status {}{}",
            command.display(),
            output.status,
            if stderr.is_empty() {
                String::new()
            } else {
                format!(": {stderr}")
            }
        ))
    }
}

fn env_path_exists(name: &str) -> bool {
    std::env::var_os(name)
        .map(PathBuf::from)
        .is_some_and(|path| path.exists())
}

pub(crate) fn parity_fixture_case_passed(
    report: &ParityReport,
    missing_required_diagnostics: &[String],
    expected_output_matches: &[ExpectedOutputComparison],
) -> bool {
    report.comparison.passed
        && report.expected_text_matches != Some(false)
        && report.expected_segment_count_matches != Some(false)
        && missing_required_diagnostics.is_empty()
        && expected_output_matches
            .iter()
            .filter(|output| output.gating)
            .all(|output| output.passed)
}

pub(crate) fn missing_required_diagnostics(
    report: &ParityReport,
    required: &[String],
) -> Vec<String> {
    required
        .iter()
        .filter(|required| {
            !report
                .native_report
                .response
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic == *required)
        })
        .cloned()
        .collect()
}

pub(crate) fn resolve_fixture_case_paths(
    mut fixture: ParityFixtureCase,
    root: Option<&Path>,
) -> ParityFixtureCase {
    fixture.input = resolve_path_with_root(fixture.input, root);
    fixture.expected_json = resolve_optional_path_with_root(fixture.expected_json, root);
    for expected_output in &mut fixture.expected_outputs {
        expected_output.path = resolve_path_with_root(expected_output.path.clone(), root);
    }
    resolve_asr_paths(&mut fixture.native_asr, root);
    resolve_translation_paths(&mut fixture.translation, root);
    resolve_vad_paths(&mut fixture.vad, root);
    resolve_alignment_paths(&mut fixture.alignment, root);
    resolve_diarization_paths(&mut fixture.diarization, root);
    if let Some(diarization) = &mut fixture.whisperx_diarization {
        resolve_diarization_paths(diarization, root);
    }
    resolve_external_whisperx_paths(&mut fixture.whisperx, root);
    resolve_output_paths(&mut fixture.output, root);
    fixture
}

fn resolve_asr_paths(asr: &mut AsrConfig, root: Option<&Path>) {
    asr.whisper_bundle = resolve_optional_path_with_root(asr.whisper_bundle.take(), root);
    asr.model_dir = resolve_optional_path_with_root(asr.model_dir.take(), root);
    resolve_external_whisperx_paths(&mut asr.external_whisperx, root);
}

fn resolve_translation_paths(translation: &mut TranslationConfig, root: Option<&Path>) {
    translation.model_bundle =
        resolve_optional_path_with_root(translation.model_bundle.take(), root);
    translation.model_dir = resolve_optional_path_with_root(translation.model_dir.take(), root);
}

fn resolve_vad_paths(vad: &mut VadConfig, root: Option<&Path>) {
    vad.model_bundle = resolve_optional_path_with_root(vad.model_bundle.take(), root);
}

fn resolve_alignment_paths(alignment: &mut AlignmentConfig, root: Option<&Path>) {
    alignment.model_bundle = resolve_optional_path_with_root(alignment.model_bundle.take(), root);
    alignment.model_dir = resolve_optional_path_with_root(alignment.model_dir.take(), root);
}

fn resolve_diarization_paths(diarization: &mut DiarizationConfig, root: Option<&Path>) {
    diarization.model_bundle =
        resolve_optional_path_with_root(diarization.model_bundle.take(), root);
    diarization.speaker_embedding_model_bundle =
        resolve_optional_path_with_root(diarization.speaker_embedding_model_bundle.take(), root);
}

fn resolve_external_whisperx_paths(whisperx: &mut ExternalWhisperxConfig, root: Option<&Path>) {
    if whisperx.command != default_whisperx_command() {
        whisperx.command = resolve_path_with_root(whisperx.command.clone(), root);
    }
    whisperx.output_dir = resolve_optional_path_with_root(whisperx.output_dir.take(), root);
}

fn resolve_output_paths(output: &mut OutputConfig, root: Option<&Path>) {
    output.output_dir = resolve_optional_path_with_root(output.output_dir.take(), root);
}

fn resolve_optional_path_with_root(path: Option<PathBuf>, root: Option<&Path>) -> Option<PathBuf> {
    path.map(|path| resolve_path_with_root(path, root))
}

fn resolve_path_with_root(path: PathBuf, root: Option<&Path>) -> PathBuf {
    match root {
        Some(root) if path.is_relative() => root.join(path),
        _ => path,
    }
}

pub(crate) fn compare_transcripts(
    native: &TranscriptionContract,
    whisperx: &TranscriptionContract,
    tolerance: ParityTolerance,
    config: &ParityComparisonConfig,
) -> ParityComparison {
    let mut differences = Vec::new();
    let text_matches =
        normalize_space(&native.text_or_joined()) == normalize_space(&whisperx.text_or_joined());
    if !text_matches {
        push_comparison_difference(
            &mut differences,
            config.text,
            "normalized transcript text differs".to_string(),
        );
    }

    let language_matches = native.language == whisperx.language;
    if !language_matches {
        push_comparison_difference(
            &mut differences,
            config.language,
            format!(
                "language differs: native={:?} reference={:?}",
                native.language, whisperx.language
            ),
        );
    }

    let native_segment_text = segment_text_signature(native);
    let reference_segment_text = segment_text_signature(whisperx);
    let segment_text_matches = native_segment_text == reference_segment_text;
    if !segment_text_matches {
        push_comparison_difference(
            &mut differences,
            config.segment_text,
            format!(
                "segment text sequence differs: native={native_segment_text:?} reference={reference_segment_text:?}"
            ),
        );
    }

    let native_word_text = word_text_signature(native);
    let reference_word_text = word_text_signature(whisperx);
    let word_text_matches = native_word_text == reference_word_text;
    if !word_text_matches {
        push_comparison_difference(
            &mut differences,
            config.word_text,
            format!(
                "word text sequence differs: native={native_word_text:?} reference={reference_word_text:?}"
            ),
        );
    }

    let native_char_count = char_count(native);
    let whisperx_char_count = char_count(whisperx);
    let char_count_matches = native_char_count == whisperx_char_count;
    if !char_count_matches {
        push_comparison_difference(
            &mut differences,
            config.char_count,
            format!(
                "char alignment count differs: native={native_char_count} reference={whisperx_char_count}"
            ),
        );
    }

    let native_char_content = char_content_signature(native);
    let whisperx_char_content = char_content_signature(whisperx);
    let char_content_matches = native_char_content == whisperx_char_content;
    if !char_content_matches {
        push_comparison_difference(
            &mut differences,
            config.char_content,
            char_content_difference(&native_char_content, &whisperx_char_content),
        );
    }

    let segment_count_matches = native.segments.len() == whisperx.segments.len();
    if !segment_count_matches {
        push_comparison_difference(
            &mut differences,
            config.segment_count,
            format!(
                "segment count differs: native={} reference={}",
                native.segments.len(),
                whisperx.segments.len()
            ),
        );
    }

    let native_word_count = word_count(native);
    let whisperx_word_count = word_count(whisperx);
    let word_count_matches = native_word_count == whisperx_word_count;
    if !word_count_matches {
        push_comparison_difference(
            &mut differences,
            config.word_count,
            format!(
                "word count differs: native={native_word_count} reference={whisperx_word_count}"
            ),
        );
    }

    let segment_timing_matches = timings_match(
        native.segments.iter().map(|segment| {
            (
                segment.start_seconds,
                segment.end_seconds,
                format!("segment {}", segment.index),
            )
        }),
        whisperx.segments.iter().map(|segment| {
            (
                segment.start_seconds,
                segment.end_seconds,
                format!("segment {}", segment.index),
            )
        }),
        tolerance.segment_seconds,
        "segment",
        config.segment_timing,
        &mut differences,
    );

    let native_words = native
        .segments
        .iter()
        .flat_map(|segment| segment.words.iter())
        .collect::<Vec<_>>();
    let whisperx_words = whisperx
        .segments
        .iter()
        .flat_map(|segment| segment.words.iter())
        .collect::<Vec<_>>();
    let word_timing_matches = timings_match(
        native_words.iter().enumerate().map(|(index, word)| {
            (
                word.start_seconds,
                word.end_seconds,
                format!("word {index}"),
            )
        }),
        whisperx_words.iter().enumerate().map(|(index, word)| {
            (
                word.start_seconds,
                word.end_seconds,
                format!("word {index}"),
            )
        }),
        tolerance.word_seconds,
        "word",
        config.word_timing,
        &mut differences,
    );

    let speaker_turns_match = speaker_turn_signature(native) == speaker_turn_signature(whisperx);
    if !speaker_turns_match {
        push_comparison_difference(
            &mut differences,
            config.speaker_turns,
            "speaker turn structure differs".to_string(),
        );
    }

    let passed = comparison_field_passed(config.text, text_matches)
        && comparison_field_passed(config.language, language_matches)
        && comparison_field_passed(config.segment_text, segment_text_matches)
        && comparison_field_passed(config.word_text, word_text_matches)
        && comparison_field_passed(config.char_count, char_count_matches)
        && comparison_field_passed(config.char_content, char_content_matches)
        && comparison_field_passed(config.segment_count, segment_count_matches)
        && comparison_field_passed(config.word_count, word_count_matches)
        && comparison_field_passed(config.segment_timing, segment_timing_matches)
        && comparison_field_passed(config.word_timing, word_timing_matches)
        && comparison_field_passed(config.speaker_turns, speaker_turns_match);

    ParityComparison {
        text_matches,
        language_matches: Some(language_matches),
        segment_text_matches: Some(segment_text_matches),
        word_text_matches: Some(word_text_matches),
        char_count_matches: Some(char_count_matches),
        char_content_matches: Some(char_content_matches),
        segment_count_matches,
        word_count_matches,
        segment_timing_matches,
        word_timing_matches,
        speaker_turns_match,
        vad_segment_count_matches: None,
        vad_segment_timing_matches: None,
        confidence_compared: true,
        passed,
        tolerance,
        differences,
        diagnostic_differences: Vec::new(),
    }
}

fn comparison_field_passed(enabled: bool, matches: bool) -> bool {
    !enabled || matches
}

fn push_comparison_difference(differences: &mut Vec<String>, enabled: bool, difference: String) {
    if enabled {
        differences.push(difference);
    } else {
        differences.push(format!("report-only: {difference}"));
    }
}

fn word_count(transcript: &TranscriptionContract) -> usize {
    transcript
        .segments
        .iter()
        .map(|segment| segment.words.len())
        .sum()
}

fn char_count(transcript: &TranscriptionContract) -> usize {
    transcript
        .segments
        .iter()
        .map(|segment| segment.chars.len())
        .sum()
}

fn char_content_signature(transcript: &TranscriptionContract) -> Vec<String> {
    transcript
        .segments
        .iter()
        .flat_map(|segment| segment.chars.iter())
        .map(|character| character.character.clone())
        .collect()
}

fn char_content_difference(native: &[String], whisperx: &[String]) -> String {
    let mismatch = native
        .iter()
        .zip(whisperx.iter())
        .enumerate()
        .find(|(_, (native, whisperx))| native != whisperx);
    if let Some((index, (native, whisperx))) = mismatch {
        return format!(
            "char alignment content differs at char {index}: native={native:?} reference={whisperx:?}"
        );
    }
    format!(
        "char alignment content differs: native_count={} reference_count={}",
        native.len(),
        whisperx.len()
    )
}

fn segment_text_signature(transcript: &TranscriptionContract) -> Vec<String> {
    transcript
        .segments
        .iter()
        .map(|segment| normalize_space(&segment.text))
        .collect()
}

fn word_text_signature(transcript: &TranscriptionContract) -> Vec<String> {
    transcript
        .segments
        .iter()
        .flat_map(|segment| segment.words.iter())
        .map(|word| normalize_space(&word.text))
        .collect()
}

pub(crate) fn compare_diagnostics(native: &[String], whisperx: &[String]) -> Vec<String> {
    let native_set = native.iter().collect::<std::collections::BTreeSet<_>>();
    let whisperx_set = whisperx.iter().collect::<std::collections::BTreeSet<_>>();
    let mut differences = Vec::new();

    for diagnostic in native_set.difference(&whisperx_set) {
        differences.push(format!("native diagnostic only: {diagnostic}"));
    }
    for diagnostic in whisperx_set.difference(&native_set) {
        differences.push(format!("whisperx diagnostic only: {diagnostic}"));
    }

    differences
}

pub(crate) fn compare_vad_segments(
    native: &[SpeechActivitySegment],
    whisperx: &[SpeechActivitySegment],
    tolerance: ParityTolerance,
    config: &ParityComparisonConfig,
    comparison: &mut ParityComparison,
) {
    if !config.vad_segments {
        comparison.vad_segment_count_matches = None;
        comparison.vad_segment_timing_matches = None;
        return;
    }

    let count_matches = native.len() == whisperx.len();
    if !count_matches {
        push_comparison_difference(
            &mut comparison.differences,
            config.vad_segment_count,
            format!(
                "VAD segment count differs: native={} reference={}",
                native.len(),
                whisperx.len()
            ),
        );
    }

    let timing_matches = timings_match(
        native.iter().enumerate().map(|(index, segment)| {
            (
                Some(segment.start_seconds),
                Some(segment.end_seconds),
                format!("VAD segment {index}"),
            )
        }),
        whisperx.iter().enumerate().map(|(index, segment)| {
            (
                Some(segment.start_seconds),
                Some(segment.end_seconds),
                format!("VAD segment {index}"),
            )
        }),
        tolerance.segment_seconds,
        "VAD segment",
        config.vad_segment_timing,
        &mut comparison.differences,
    );

    comparison.vad_segment_count_matches = Some(count_matches);
    comparison.vad_segment_timing_matches = Some(timing_matches);
    comparison.passed = comparison.passed
        && comparison_field_passed(config.vad_segment_count, count_matches)
        && comparison_field_passed(config.vad_segment_timing, timing_matches);
}

fn timings_match<N, W>(
    native: N,
    whisperx: W,
    tolerance: f64,
    label: &str,
    enabled: bool,
    differences: &mut Vec<String>,
) -> bool
where
    N: Iterator<Item = (Option<f64>, Option<f64>, String)>,
    W: Iterator<Item = (Option<f64>, Option<f64>, String)>,
{
    let native = native.collect::<Vec<_>>();
    let whisperx = whisperx.collect::<Vec<_>>();
    if native.len() != whisperx.len() {
        return false;
    }

    let mut matches = true;
    for ((native_start, native_end, name), (whisperx_start, whisperx_end, _)) in
        native.into_iter().zip(whisperx)
    {
        if !optional_seconds_match(native_start, whisperx_start, tolerance)
            || !optional_seconds_match(native_end, whisperx_end, tolerance)
        {
            push_comparison_difference(
                differences,
                enabled,
                format_timing_difference(
                    label,
                    &name,
                    native_start,
                    native_end,
                    whisperx_start,
                    whisperx_end,
                    tolerance,
                ),
            );
            matches = false;
        }
    }
    matches
}

fn format_timing_difference(
    label: &str,
    name: &str,
    native_start: Option<f64>,
    native_end: Option<f64>,
    whisperx_start: Option<f64>,
    whisperx_end: Option<f64>,
    tolerance: f64,
) -> String {
    format!(
        "{label} timing differs at {name}: native start={} native end={}, reference start={} reference end={}, start_delta={} end_delta={} tolerance={:.3}s",
        format_optional_seconds(native_start),
        format_optional_seconds(native_end),
        format_optional_seconds(whisperx_start),
        format_optional_seconds(whisperx_end),
        format_optional_delta(native_start, whisperx_start),
        format_optional_delta(native_end, whisperx_end),
        tolerance,
    )
}

fn format_optional_seconds(value: Option<f64>) -> String {
    value
        .map(|value| format!("{value:.3}s"))
        .unwrap_or_else(|| "missing".to_string())
}

fn format_optional_delta(left: Option<f64>, right: Option<f64>) -> String {
    match (left, right) {
        (Some(left), Some(right)) => format!("{:.3}s", (left - right).abs()),
        _ => "missing".to_string(),
    }
}

fn optional_seconds_match(left: Option<f64>, right: Option<f64>, tolerance: f64) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => (left - right).abs() <= tolerance,
        (None, None) => true,
        _ => false,
    }
}

fn speaker_turn_signature(transcript: &TranscriptionContract) -> Vec<Option<usize>> {
    let mut speakers = Vec::<String>::new();
    transcript
        .segments
        .iter()
        .map(|segment| {
            segment.speaker.as_ref().map(|speaker| {
                speakers
                    .iter()
                    .position(|known| known == speaker)
                    .unwrap_or_else(|| {
                        speakers.push(speaker.clone());
                        speakers.len() - 1
                    })
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use audio_analysis_transcription::TranscriptionPipelineResponse;

    use crate::config::{
        ExpectedOutputFile, NativeWhisperxReport, OutputComparisonMode, OutputFormat,
    };
    use crate::import_whisperx_json;

    const WHISPERX_SAMPLE: &[u8] =
        include_bytes!("../../../tests/fixtures/whisperx-parity-sample.json");

    #[test]
    fn parity_comparison_accepts_permutation_equivalent_speaker_turns() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.segments[0].speaker = Some("reference-speaker-b".to_string());
        whisperx.segments[1].speaker = Some("reference-speaker-a".to_string());

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig {
                text: false,
                language: false,
                segment_text: false,
                word_text: false,
                char_count: false,
                char_content: false,
                segment_count: false,
                word_count: false,
                segment_timing: false,
                word_timing: false,
                speaker_turns: true,
                vad_segments: false,
                vad_segment_timing: false,
                vad_segment_count: false,
            },
        );

        assert!(comparison.speaker_turns_match);
        assert!(comparison.passed);
    }

    #[test]
    fn parity_comparison_reports_text_language_word_and_char_categories() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.language = Some("de".to_string());
        whisperx.segments[0].text = "hello changed".to_string();
        whisperx.segments[0].words[0].text = "changed".to_string();
        whisperx.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig::default(),
        );

        assert_eq!(comparison.language_matches, Some(false));
        assert_eq!(comparison.segment_text_matches, Some(false));
        assert_eq!(comparison.word_text_matches, Some(false));
        assert_eq!(comparison.char_count_matches, Some(false));
        assert_eq!(comparison.char_content_matches, Some(false));
        assert!(!comparison.passed);
        for expected in [
            "language differs: native=Some(\"en\") reference=Some(\"de\")",
            "segment text sequence differs: native=[\"hello world\", \"second speaker\"] reference=[\"hello changed\", \"second speaker\"]",
            "word text sequence differs: native=[\"hello\", \"world\", \"second\", \"speaker\"] reference=[\"changed\", \"world\", \"second\", \"speaker\"]",
            "char alignment count differs",
            "char alignment content differs",
        ] {
            assert!(
                comparison
                    .differences
                    .iter()
                    .any(|difference| difference.contains(expected)),
                "comparison should report `{expected}`: {:?}",
                comparison.differences
            );
        }
    }

    #[test]
    fn parity_comparison_fails_character_content_mismatches() {
        let mut native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        native.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });
        let mut whisperx = native.clone();
        whisperx.segments[0].chars[0].character = "x".to_string();

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig::default(),
        );

        assert_eq!(comparison.char_count_matches, Some(true));
        assert_eq!(comparison.char_content_matches, Some(false));
        assert!(!comparison.passed);
        assert!(comparison
            .differences
            .iter()
            .any(|difference| { difference.contains("char alignment content differs at char 0") }));
    }

    #[test]
    fn parity_comparison_config_defaults_to_strict() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav"
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");
        let parity_config: ParityConfig =
            serde_json::from_str(r#"{"input":"audio/input.wav"}"#).expect("config should parse");

        assert_eq!(
            fixture_suite.fixtures[0].comparison,
            ParityComparisonConfig::default()
        );
        assert_eq!(parity_config.comparison, ParityComparisonConfig::default());
    }

    #[test]
    fn parity_comparison_config_can_make_timing_report_only() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.segments[0].start_seconds = Some(4.0);
        whisperx.segments[0].words[0].start_seconds = Some(4.0);
        let config = ParityComparisonConfig {
            segment_timing: false,
            word_timing: false,
            ..ParityComparisonConfig::default()
        };

        let comparison =
            compare_transcripts(&native, &whisperx, ParityTolerance::default(), &config);

        assert!(!comparison.segment_timing_matches);
        assert!(!comparison.word_timing_matches);
        assert!(comparison.passed);
        let segment_difference = comparison
            .differences
            .iter()
            .find(|difference| {
                difference.starts_with("report-only: segment timing differs at segment 0")
            })
            .expect("segment timing difference should be reported");
        assert!(segment_difference.contains("native start="));
        assert!(segment_difference.contains("reference start=4.000s"));
        assert!(segment_difference.contains("start_delta="));
        assert!(segment_difference.contains("tolerance=0.100s"));

        let word_difference = comparison
            .differences
            .iter()
            .find(|difference| difference.starts_with("report-only: word timing differs at word 0"))
            .expect("word timing difference should be reported");
        assert!(word_difference.contains("native start="));
        assert!(word_difference.contains("reference start=4.000s"));
        assert!(word_difference.contains("start_delta="));
        assert!(word_difference.contains("tolerance=0.050s"));
    }

    #[test]
    fn parity_comparison_strict_timing_differences_fail_with_numeric_deltas() {
        let native = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let mut whisperx = native.clone();
        whisperx.segments[0].start_seconds = Some(4.0);
        whisperx.segments[0].end_seconds = Some(5.0);
        whisperx.segments[0].words[0].start_seconds = Some(4.0);
        whisperx.segments[0].words[0].end_seconds = Some(4.5);

        let comparison = compare_transcripts(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &ParityComparisonConfig::default(),
        );

        assert!(!comparison.segment_timing_matches);
        assert!(!comparison.word_timing_matches);
        assert!(!comparison.passed);
        let segment_difference = comparison
            .differences
            .iter()
            .find(|difference| difference.starts_with("segment timing differs at segment 0"))
            .expect("segment timing difference should be reported");
        assert!(segment_difference.contains("native start="));
        assert!(segment_difference.contains("native end="));
        assert!(segment_difference.contains("reference start=4.000s"));
        assert!(segment_difference.contains("reference end=5.000s"));
        assert!(segment_difference.contains("start_delta="));
        assert!(segment_difference.contains("end_delta="));
        assert!(segment_difference.contains("tolerance=0.100s"));

        let word_difference = comparison
            .differences
            .iter()
            .find(|difference| difference.starts_with("word timing differs at word 0"))
            .expect("word timing difference should be reported");
        assert!(word_difference.contains("native start="));
        assert!(word_difference.contains("native end="));
        assert!(word_difference.contains("reference start=4.000s"));
        assert!(word_difference.contains("reference end=4.500s"));
        assert!(word_difference.contains("start_delta="));
        assert!(word_difference.contains("end_delta="));
        assert!(word_difference.contains("tolerance=0.050s"));
    }

    #[test]
    fn fixture_suite_keeps_report_only_differences_visible() {
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", true, "audio/input.wav")],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |_| {
            let mut report = fixture_parity_report();
            report.comparison.segment_timing_matches = false;
            report.comparison.differences =
                vec!["report-only: segment timing differs at segment 0".to_string()];
            Ok(report)
        })
        .expect("suite should run");

        assert!(report.passed);
        assert!(report.cases[0].passed);
        assert!(report.cases[0]
            .failure_summary
            .iter()
            .any(|difference| difference == "report-only: segment timing differs at segment 0"));
    }

    #[test]
    fn parity_fixture_manifest_accepts_comparison_config() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "comparison": {
                    "segmentTiming": false,
                    "charContent": false
                  }
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        assert!(!fixture_suite.fixtures[0].comparison.segment_timing);
        assert!(!fixture_suite.fixtures[0].comparison.char_content);
        assert!(fixture_suite.fixtures[0].comparison.word_timing);
        assert!(fixture_suite.fixtures[0].comparison.char_count);
    }

    #[test]
    fn parity_fixture_manifest_accepts_expected_target() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "expectedTarget": "whisperx"
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        assert_eq!(
            fixture_suite.fixtures[0].expected_target,
            ExpectedTranscriptTarget::Whisperx
        );
    }

    #[test]
    fn legacy_fixture_expected_target_defaults_to_native() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav"
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        assert_eq!(
            fixture_suite.fixtures[0].expected_target,
            ExpectedTranscriptTarget::Native
        );
    }

    #[test]
    fn compare_with_whisperx_expected_target_uses_whisperx_transcript() {
        let expected = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let whisperx = expected.clone();
        let mut native = expected.clone();
        native.segments[0].text = "native transcript mismatch".to_string();
        native.segments.pop();

        let (expected_segment_count_matches, expected_text_matches) = expected_transcript_matches(
            Some(&expected),
            ExpectedTranscriptTarget::Whisperx,
            &native,
            &whisperx,
        );

        assert_eq!(expected_text_matches, Some(true));
        assert_eq!(expected_segment_count_matches, Some(true));

        let mut report = fixture_parity_report();
        report.expected_target = ExpectedTranscriptTarget::Whisperx;
        report.expected_text_matches = expected_text_matches;
        report.expected_segment_count_matches = expected_segment_count_matches;
        report.comparison.differences =
            vec!["report-only: native transcript differs from WhisperX transcript".to_string()];

        assert!(parity_fixture_case_passed(&report, &[], &[]));
    }

    #[test]
    fn parity_fixture_manifest_accepts_whisperx_diarization_config() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "diarization": {
                    "enabled": true,
                    "modelId": "native-spectral-speaker-baseline"
                  },
                  "whisperxDiarization": {
                    "enabled": true,
                    "modelId": "pyannote/speaker-diarization-community-1",
                    "hfTokenEnv": "HF_TOKEN",
                    "returnSpeakerEmbeddings": true
                  }
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        let fixture = &fixture_suite.fixtures[0];
        assert_eq!(
            fixture.diarization.model_id,
            "native-spectral-speaker-baseline"
        );
        let whisperx_diarization = fixture
            .whisperx_diarization
            .as_ref()
            .expect("whisperx diarization config");
        assert_eq!(
            whisperx_diarization.model_id,
            "pyannote/speaker-diarization-community-1"
        );
        assert_eq!(
            whisperx_diarization.hf_token_env.as_deref(),
            Some("HF_TOKEN")
        );
        assert!(whisperx_diarization.return_speaker_embeddings);
    }

    #[test]
    fn parity_fixture_manifest_without_whisperx_diarization_keeps_shared_behavior() {
        let fixture_suite: ParityFixtureSuite = serde_json::from_str(
            r#"{
              "fixtures": [
                {
                  "name": "case",
                  "input": "audio/input.wav",
                  "diarization": {
                    "enabled": true,
                    "modelId": "legacy-shared-model"
                  }
                }
              ]
            }"#,
        )
        .expect("fixture suite should parse");

        let fixture = &fixture_suite.fixtures[0];
        assert_eq!(fixture.diarization.model_id, "legacy-shared-model");
        assert!(fixture.whisperx_diarization.is_none());
    }

    #[test]
    fn diagnostic_comparison_reports_provider_specific_entries() {
        let differences = compare_diagnostics(
            &["shared".to_string(), "native-only".to_string()],
            &["shared".to_string(), "whisperx-only".to_string()],
        );

        assert_eq!(
            differences,
            vec![
                "native diagnostic only: native-only".to_string(),
                "whisperx diagnostic only: whisperx-only".to_string()
            ]
        );
    }

    #[test]
    fn parity_fixture_fails_failed_output_comparison() {
        let report = fixture_parity_report();
        let failed_outputs = vec![ExpectedOutputComparison {
            format: OutputFormat::Txt,
            comparison: OutputComparisonMode::Exact,
            gating: true,
            expected_path: PathBuf::from("expected.txt"),
            actual_path: Some(PathBuf::from("actual.txt")),
            passed: false,
            difference: Some("line 1 differs".to_string()),
        }];

        assert!(!parity_fixture_case_passed(&report, &[], &failed_outputs));
    }

    #[test]
    fn preflight_resolves_relative_manifest_paths_under_root() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path();
        fs::create_dir_all(root.join("audio")).expect("audio");
        fs::create_dir_all(root.join("models")).expect("models");
        fs::write(root.join("audio/input.wav"), b"audio").expect("input");

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![minimal_fixture("case", true, "audio/input.wav")],
            },
            root.join("fixtures.json"),
            root.to_path_buf(),
            PathBuf::from("/bin/true"),
            root.join("models"),
            false,
            false,
        );

        assert!(!report.cases[0]
            .missing
            .iter()
            .any(|missing| missing.contains("input")));
    }

    #[test]
    fn preflight_reports_missing_input() {
        let temp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(temp.path().join("models")).expect("models");

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![minimal_fixture("case", true, "audio/missing.wav")],
            },
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            false,
            false,
        );

        assert!(!report.cases[0].passed);
        assert!(report.cases[0]
            .missing
            .iter()
            .any(|missing| missing.contains("audio/missing.wav")));
    }

    #[test]
    fn preflight_reports_missing_expected_output_when_required() {
        let temp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(temp.path().join("audio")).expect("audio");
        fs::create_dir_all(temp.path().join("models")).expect("models");
        fs::write(temp.path().join("audio/input.wav"), b"audio").expect("input");
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.expected_outputs.push(ExpectedOutputFile {
            format: OutputFormat::Srt,
            path: PathBuf::from("expected/missing.srt"),
            comparison: OutputComparisonMode::Exact,
            gating: true,
        });

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![fixture],
            },
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            true,
            false,
        );

        assert!(report.cases[0]
            .missing
            .iter()
            .any(|missing| missing.contains("expected/missing.srt")));
    }

    #[test]
    fn preflight_ignores_missing_non_gating_resources_unless_included() {
        let temp = tempfile::tempdir().expect("tempdir");
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", false, "audio/missing.wav")],
        };

        let ignored = run_parity_preflight(
            suite.clone(),
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            false,
            false,
        );
        assert!(ignored.passed);
        assert!(ignored.cases[0].missing.is_empty());
        assert!(!ignored.cases[0].warnings.is_empty());

        let included = run_parity_preflight(
            suite,
            temp.path().join("fixtures.json"),
            temp.path().to_path_buf(),
            PathBuf::from("/bin/true"),
            temp.path().join("models"),
            false,
            true,
        );
        assert!(!included.passed);
        assert!(!included.cases[0].missing.is_empty());
    }

    #[test]
    fn preflight_reports_missing_onnx_runtime_for_onnx_diarization() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path();
        fs::create_dir_all(root.join("audio")).expect("audio");
        fs::create_dir_all(root.join("models/diarization")).expect("diarization model bundle");
        fs::create_dir_all(root.join("models")).expect("models");
        fs::write(root.join("audio/input.wav"), b"audio").expect("input");
        for file in [
            "pyannote_diarization_manifest.json",
            "segmentation.onnx",
            "embedding.onnx",
            "plda_transform.json",
            "plda_model.json",
            "clustering.json",
        ] {
            fs::write(root.join("models/diarization").join(file), b"model").expect(file);
        }
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.diarization = DiarizationConfig {
            enabled: true,
            model_bundle: Some(PathBuf::from("models/diarization")),
            ..DiarizationConfig::default()
        };
        let previous_ort = std::env::var_os("ORT_DYLIB_PATH");
        std::env::set_var("ORT_DYLIB_PATH", root.join("missing-onnxruntime.so"));

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![fixture],
            },
            root.join("fixtures.json"),
            root.to_path_buf(),
            PathBuf::from("/bin/true"),
            root.join("models"),
            false,
            false,
        );

        if let Some(previous_ort) = previous_ort {
            std::env::set_var("ORT_DYLIB_PATH", previous_ort);
        } else {
            std::env::remove_var("ORT_DYLIB_PATH");
        }
        assert!(!report.cases[0].passed);
        assert!(report.cases[0]
            .missing
            .iter()
            .any(|missing| missing == "ORT_DYLIB_PATH is not set to an existing file"));
    }

    #[test]
    fn preflight_skips_hf_token_env_when_diarization_is_disabled() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path();
        fs::create_dir_all(root.join("audio")).expect("audio");
        fs::create_dir_all(root.join("models")).expect("models");
        fs::write(root.join("audio/input.wav"), b"audio").expect("input");
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.diarization = DiarizationConfig {
            enabled: false,
            model_id: "pyannote/speaker-diarization-community-1".to_string(),
            hf_token_env: Some("NATIVE_WHISPERX_TEST_MISSING_HF_TOKEN".to_string()),
            ..DiarizationConfig::default()
        };
        std::env::remove_var("NATIVE_WHISPERX_TEST_MISSING_HF_TOKEN");

        let report = run_parity_preflight(
            ParityFixtureSuite {
                fixtures: vec![fixture],
            },
            root.join("fixtures.json"),
            root.to_path_buf(),
            PathBuf::from("/bin/true"),
            root.join("models"),
            false,
            false,
        );

        assert!(!report.cases[0]
            .missing
            .iter()
            .any(|missing| { missing.contains("NATIVE_WHISPERX_TEST_MISSING_HF_TOKEN") }));
    }

    #[test]
    fn fixture_suite_records_gating_case_error_and_fails_suite() {
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", true, "audio/input.wav")],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |_| {
            Err(NativeWhisperxError::InvalidConfig(
                "setup failed".to_string(),
            ))
        })
        .expect("suite should not abort");

        assert!(!report.passed);
        assert!(!report.cases[0].passed);
        assert_eq!(
            report.cases[0].error.as_deref(),
            Some("invalid configuration: setup failed")
        );
    }

    #[test]
    fn fixture_suite_passes_separate_whisperx_diarization_config() {
        let mut fixture = minimal_fixture("case", true, "audio/input.wav");
        fixture.diarization = DiarizationConfig {
            enabled: true,
            model_id: "native-spectral-speaker-baseline".to_string(),
            min_speakers: Some(2),
            max_speakers: Some(2),
            ..DiarizationConfig::default()
        };
        fixture.whisperx_diarization = Some(DiarizationConfig {
            enabled: true,
            model_id: "pyannote/speaker-diarization-community-1".to_string(),
            hf_token_env: Some("HF_TOKEN".to_string()),
            return_speaker_embeddings: true,
            min_speakers: Some(2),
            max_speakers: Some(2),
            ..DiarizationConfig::default()
        });
        let suite = ParityFixtureSuite {
            fixtures: vec![fixture],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |config| {
            assert_eq!(
                config.diarization.model_id,
                "native-spectral-speaker-baseline"
            );
            let whisperx_diarization = config
                .whisperx_diarization
                .expect("whisperx diarization config");
            assert_eq!(
                whisperx_diarization.model_id,
                "pyannote/speaker-diarization-community-1"
            );
            assert!(whisperx_diarization.return_speaker_embeddings);
            Ok(fixture_parity_report())
        })
        .expect("suite should run");

        assert!(report.passed);
    }

    #[test]
    fn fixture_suite_records_non_gating_case_error_and_keeps_suite_passed() {
        let suite = ParityFixtureSuite {
            fixtures: vec![minimal_fixture("case", false, "audio/input.wav")],
        };

        let report = run_parity_fixture_suite_with_runner(suite, None, |_| {
            Err(NativeWhisperxError::InvalidConfig(
                "setup failed".to_string(),
            ))
        })
        .expect("suite should not abort");

        assert!(report.passed);
        assert!(!report.cases[0].passed);
        assert!(report.cases[0].error.is_some());
    }

    #[test]
    fn failure_summary_includes_output_diff_and_missing_diagnostics() {
        let report = fixture_parity_report();
        let summary = parity_fixture_failure_summary(
            Some(&report),
            &["asrModelSource=hugging-face-cache".to_string()],
            &[ExpectedOutputComparison {
                format: OutputFormat::Txt,
                comparison: OutputComparisonMode::Exact,
                gating: true,
                expected_path: PathBuf::from("expected.txt"),
                actual_path: Some(PathBuf::from("actual.txt")),
                passed: false,
                difference: Some("line 1 differs: expected \"a\", actual \"b\"".to_string()),
            }],
            None,
        );

        assert!(summary
            .iter()
            .any(|line| line.contains("missing required diagnostic")));
        assert!(summary.iter().any(|line| line.contains("line 1 differs")));
    }

    #[test]
    fn parity_fixture_resolves_relative_paths_against_root() {
        let fixture = resolve_fixture_case_paths(
            ParityFixtureCase {
                name: "case".to_string(),
                gating: true,
                input: PathBuf::from("audio/input.wav"),
                clip_seconds: None,
                timeout_seconds: None,
                expected_json: Some(PathBuf::from("expected/input.json")),
                expected_target: ExpectedTranscriptTarget::Native,
                comparison: ParityComparisonConfig::default(),
                expected_outputs: vec![ExpectedOutputFile {
                    format: OutputFormat::Srt,
                    path: PathBuf::from("expected/input.srt"),
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                }],
                native_asr: AsrConfig {
                    whisper_bundle: Some(PathBuf::from("models/whisper")),
                    model_dir: Some(PathBuf::from("models")),
                    external_whisperx: ExternalWhisperxConfig {
                        command: PathBuf::from("bin/whisperx"),
                        output_dir: Some(PathBuf::from("external-out")),
                        ..ExternalWhisperxConfig::default()
                    },
                    ..AsrConfig::default()
                },
                translation: TranslationConfig {
                    model_bundle: Some(PathBuf::from("models/translation")),
                    model_dir: Some(PathBuf::from("models")),
                    ..TranslationConfig::default()
                },
                vad: VadConfig {
                    model_bundle: Some(PathBuf::from("models/silero")),
                    ..VadConfig::default()
                },
                alignment: AlignmentConfig {
                    model_bundle: Some(PathBuf::from("models/wav2vec2")),
                    model_dir: Some(PathBuf::from("models")),
                    ..AlignmentConfig::default()
                },
                diarization: DiarizationConfig {
                    speaker_embedding_model_bundle: Some(PathBuf::from("models/speakers")),
                    ..DiarizationConfig::default()
                },
                whisperx_diarization: None,
                whisperx: ExternalWhisperxConfig {
                    command: PathBuf::from("bin/whisperx"),
                    output_dir: Some(PathBuf::from("whisperx-out")),
                    ..ExternalWhisperxConfig::default()
                },
                language: Some("en".to_string()),
                output: OutputConfig {
                    output_dir: Some(PathBuf::from("out")),
                    ..OutputConfig::default()
                },
                required_diagnostics: Vec::new(),
            },
            Some(Path::new("/smoke")),
        );

        assert_eq!(fixture.input, PathBuf::from("/smoke/audio/input.wav"));
        assert_eq!(
            fixture.expected_json,
            Some(PathBuf::from("/smoke/expected/input.json"))
        );
        assert_eq!(
            fixture.expected_outputs[0].path,
            PathBuf::from("/smoke/expected/input.srt")
        );
        assert_eq!(
            fixture.native_asr.whisper_bundle,
            Some(PathBuf::from("/smoke/models/whisper"))
        );
        assert_eq!(
            fixture.native_asr.external_whisperx.command,
            PathBuf::from("/smoke/bin/whisperx")
        );
        assert_eq!(
            fixture.translation.model_bundle,
            Some(PathBuf::from("/smoke/models/translation"))
        );
        assert_eq!(
            fixture.translation.model_dir,
            Some(PathBuf::from("/smoke/models"))
        );
        assert_eq!(
            fixture.vad.model_bundle,
            Some(PathBuf::from("/smoke/models/silero"))
        );
        assert_eq!(
            fixture.alignment.model_bundle,
            Some(PathBuf::from("/smoke/models/wav2vec2"))
        );
        assert_eq!(
            fixture.diarization.speaker_embedding_model_bundle,
            Some(PathBuf::from("/smoke/models/speakers"))
        );
        assert_eq!(
            fixture.whisperx.command,
            PathBuf::from("/smoke/bin/whisperx")
        );
        assert_eq!(fixture.output.output_dir, Some(PathBuf::from("/smoke/out")));
    }

    #[test]
    fn parity_fixture_reports_required_diagnostics() {
        let mut report = fixture_parity_report();
        report
            .native_report
            .response
            .diagnostics
            .push("asrModelSource=hugging-face-cache".to_string());

        let missing = missing_required_diagnostics(
            &report,
            &[
                "asrModelSource=hugging-face-cache".to_string(),
                "asrModelId=openai/whisper-tiny.en".to_string(),
            ],
        );

        assert_eq!(
            missing,
            vec!["asrModelId=openai/whisper-tiny.en".to_string()]
        );
        assert!(!parity_fixture_case_passed(&report, &missing, &[]));
    }

    #[test]
    fn parity_fixture_passes_when_comparison_expected_and_diagnostics_pass() {
        let mut report = fixture_parity_report();
        report.expected_text_matches = Some(true);
        report.expected_segment_count_matches = Some(true);
        report
            .native_report
            .response
            .diagnostics
            .push("asrModelSource=hugging-face-cache".to_string());

        let missing = missing_required_diagnostics(
            &report,
            &["asrModelSource=hugging-face-cache".to_string()],
        );

        assert!(missing.is_empty());
        assert!(parity_fixture_case_passed(&report, &missing, &[]));
    }

    #[test]
    fn parity_fixture_fails_expected_json_mismatches() {
        let mut report = fixture_parity_report();
        report.expected_text_matches = Some(false);
        report.expected_segment_count_matches = Some(true);

        assert!(!parity_fixture_case_passed(&report, &[], &[]));

        report.expected_text_matches = Some(true);
        report.expected_segment_count_matches = Some(false);

        assert!(!parity_fixture_case_passed(&report, &[], &[]));
    }

    #[test]
    fn parity_fixture_fails_failed_comparison() {
        let mut report = fixture_parity_report();
        report.comparison.passed = false;

        assert!(!parity_fixture_case_passed(&report, &[], &[]));
    }

    #[test]
    fn vad_segment_comparison_fails_count_mismatch() {
        let transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let config = ParityComparisonConfig::default();
        let mut comparison = compare_transcripts(
            &transcript,
            &transcript,
            ParityTolerance::default(),
            &config,
        );
        let native = vec![
            SpeechActivitySegment::new(0.0, 1.0, 0.9).unwrap(),
            SpeechActivitySegment::new(2.0, 3.0, 0.8).unwrap(),
        ];
        let whisperx = vec![SpeechActivitySegment::new(0.0, 1.0, 0.7).unwrap()];

        compare_vad_segments(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &config,
            &mut comparison,
        );

        assert_eq!(comparison.vad_segment_count_matches, Some(false));
        assert_eq!(comparison.vad_segment_timing_matches, Some(false));
        assert!(!comparison.passed);
        assert!(comparison
            .differences
            .iter()
            .any(|difference| difference.contains("VAD segment count differs")));
    }

    #[test]
    fn vad_segment_timing_can_be_report_only() {
        let transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        let config = ParityComparisonConfig {
            vad_segment_timing: false,
            ..ParityComparisonConfig::default()
        };
        let mut comparison = compare_transcripts(
            &transcript,
            &transcript,
            ParityTolerance::default(),
            &config,
        );
        let native = vec![SpeechActivitySegment::new(0.0, 1.0, 0.9).unwrap()];
        let whisperx = vec![SpeechActivitySegment::new(0.25, 1.0, 0.7).unwrap()];

        compare_vad_segments(
            &native,
            &whisperx,
            ParityTolerance::default(),
            &config,
            &mut comparison,
        );

        assert_eq!(comparison.vad_segment_count_matches, Some(true));
        assert_eq!(comparison.vad_segment_timing_matches, Some(false));
        assert!(comparison.passed);
        assert!(comparison.differences.iter().any(|difference| {
            difference.starts_with("report-only: VAD segment timing differs")
        }));
    }

    fn minimal_fixture(name: &str, gating: bool, input: &str) -> ParityFixtureCase {
        ParityFixtureCase {
            name: name.to_string(),
            gating,
            input: PathBuf::from(input),
            clip_seconds: None,
            timeout_seconds: None,
            expected_json: None,
            expected_target: ExpectedTranscriptTarget::Native,
            comparison: ParityComparisonConfig::default(),
            expected_outputs: Vec::new(),
            native_asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            whisperx_diarization: None,
            whisperx: ExternalWhisperxConfig::default(),
            language: None,
            output: OutputConfig::default(),
            required_diagnostics: Vec::new(),
        }
    }

    fn fixture_parity_report() -> ParityReport {
        let native_report = NativeWhisperxReport {
            response: fixture_response_with_chars(),
            output_files: Vec::new(),
        };
        let whisperx_report = native_report.clone();
        ParityReport {
            native_report,
            whisperx_report,
            expected: None,
            expected_target: ExpectedTranscriptTarget::Native,
            comparison: ParityComparison {
                text_matches: true,
                language_matches: Some(true),
                segment_text_matches: Some(true),
                word_text_matches: Some(true),
                char_count_matches: Some(true),
                char_content_matches: Some(true),
                segment_count_matches: true,
                word_count_matches: true,
                segment_timing_matches: true,
                word_timing_matches: true,
                speaker_turns_match: true,
                vad_segment_count_matches: None,
                vad_segment_timing_matches: None,
                confidence_compared: true,
                passed: true,
                tolerance: ParityTolerance::default(),
                differences: Vec::new(),
                diagnostic_differences: Vec::new(),
            },
            expected_segment_count_matches: None,
            expected_text_matches: None,
        }
    }

    fn fixture_response_with_chars() -> TranscriptionPipelineResponse {
        let mut transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        transcript.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });
        TranscriptionPipelineResponse {
            accepted: true,
            operation: "audio.transcription.transcribe".to_string(),
            provider: "fixture".to_string(),
            model_id: "fixture".to_string(),
            transcript,
            vad_segments: Vec::new(),
            alignment: None,
            diarization: None,
            artifacts: Vec::new(),
            diagnostics: Vec::new(),
        }
    }
}
