//! Parity commands for comparisons, fixture suites, goldens, and benchmarks.

use super::*;

mod args;
mod bench;
mod bench_contract;
mod bench_report;
mod bench_runner;
mod fixture_prep;
mod fixtures;
mod goldens;
mod summary;

pub(crate) use args::*;
pub(crate) use bench::*;
pub(crate) use bench_contract::*;
use bench_report::*;
pub(crate) use bench_runner::*;
use fixture_prep::*;
pub(crate) use fixtures::*;
pub(crate) use goldens::*;
pub(crate) use summary::*;

pub(crate) fn parity_command(args: ParityArgs) -> anyhow::Result<()> {
    ensure_whisperx_compat_enabled("Python WhisperX parity oracle")?;
    let report = compare_with_whisperx(ParityConfig {
        input: args.input,
        expected_json: args.expected_json,
        expected_target: ExpectedTranscriptTarget::Native,
        comparison: ParityComparisonConfig::default(),
        native_asr: AsrConfig {
            provider: AsrProvider::Native,
            model_id: args.model,
            whisper_bundle: args.whisper_bundle,
            model_dir: args.model_dir.clone(),
            model_cache_only: args.model_cache_only,
            device: args.device.into(),
            ..AsrConfig::default()
        },
        translation: TranslationConfig::default(),
        vad: VadConfig::default(),
        alignment: alignment_config(
            args.no_align,
            args.alignment_model,
            args.alignment_bundle,
            args.model_dir,
            args.model_cache_only,
            args.interpolate_method,
            args.return_char_alignments,
        ),
        diarization: DiarizationConfig {
            enabled: args.speaker_embedding_bundle.is_some()
                || args.min_speakers.is_some()
                || args.max_speakers.is_some(),
            speaker_embedding_model_bundle: args.speaker_embedding_bundle,
            min_speakers: args.min_speakers,
            max_speakers: args.max_speakers,
            ..DiarizationConfig::default()
        },
        whisperx_diarization: None,
        whisperx: WhisperxOracleConfig {
            command: args
                .whisperx_command
                .unwrap_or_else(|| PathBuf::from("whisperx")),
            model: args.whisperx_model,
            output_dir: args.output_dir.clone(),
            ..WhisperxOracleConfig::default()
        },
        language: args.language,
        output: OutputConfig {
            output_dir: args.output_dir,
            formats: vec![OutputFormat::Json],
            basename: Some("whisperx-parity".to_string()),
            pretty_json: true,
            subtitles: SubtitleConfig::default(),
        },
    })?;

    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn is_timeout_error(error: &str) -> bool {
    error.contains("exceeded timeout")
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let millis = duration.subsec_millis();
    if seconds == 0 {
        format!("{millis}ms")
    } else if millis == 0 {
        format!("{seconds}s")
    } else {
        format!("{seconds}.{millis:03}s")
    }
}

fn duration_seconds(duration: Duration) -> f64 {
    duration.as_secs_f64()
}

fn unix_timestamp_string(time: SystemTime) -> String {
    match time.duration_since(UNIX_EPOCH) {
        Ok(duration) => format!("{}.{}", duration.as_secs(), duration.subsec_millis()),
        Err(_) => "0.000".to_string(),
    }
}

pub(crate) fn smoke_root_or_arg(root: Option<PathBuf>, command: &str) -> anyhow::Result<PathBuf> {
    let root = root
        .or_else(smoke_root_from_env_or_dotenv)
        .with_context(|| {
            format!("{command} requires --root, SMOKE_ROOT, or SMOKE_ROOT in .env for local audio, expected JSON, and model cache paths")
        })?;
    absolute_from_cwd(root)
}

fn smoke_root_from_env_or_dotenv() -> Option<PathBuf> {
    std::env::var_os("SMOKE_ROOT")
        .map(PathBuf::from)
        .or_else(|| dotenv_value("SMOKE_ROOT").map(PathBuf::from))
}

fn dotenv_value(key: &str) -> Option<String> {
    let contents = fs::read_to_string(".env").ok()?;
    for line in contents.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let trimmed = trimmed.strip_prefix("export ").unwrap_or(trimmed);
        let Some((candidate, value)) = trimmed.split_once('=') else {
            continue;
        };
        if candidate.trim() != key {
            continue;
        }
        let value = value.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .or_else(|| {
                value
                    .strip_prefix('\'')
                    .and_then(|value| value.strip_suffix('\''))
            })
            .unwrap_or(value);
        if value.is_empty() {
            return None;
        }
        return Some(value.to_string());
    }
    None
}

fn resolve_cli_path_with_root(path: PathBuf, root: &Path) -> PathBuf {
    if path.is_relative() {
        root.join(path)
    } else {
        path
    }
}

#[cfg(test)]
#[path = "parity_tests.rs"]
mod tests;
