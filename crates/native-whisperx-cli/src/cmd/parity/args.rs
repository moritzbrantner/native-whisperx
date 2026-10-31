//! Clap argument structs for the parity commands.

use super::*;
use clap::Args;

#[derive(Debug, Args)]
pub(crate) struct ParityArgs {
    pub(crate) input: PathBuf,
    #[arg(long, visible_alias = "whisperx_command")]
    pub(crate) whisperx_command: Option<PathBuf>,
    #[arg(long, visible_alias = "whisperx_model", default_value = "small")]
    pub(crate) whisperx_model: String,
    #[arg(long, visible_alias = "expected_json")]
    pub(crate) expected_json: Option<PathBuf>,
    #[arg(long, visible_alias = "whisper_bundle")]
    pub(crate) whisper_bundle: Option<PathBuf>,
    #[arg(long, default_value = "small")]
    pub(crate) model: String,
    #[arg(long, value_enum, default_value_t = CliDevicePreference::Auto)]
    pub(crate) device: CliDevicePreference,
    #[arg(long = "no-align", visible_alias = "no_align")]
    pub(crate) no_align: bool,
    #[arg(long, visible_alias = "alignment_bundle")]
    pub(crate) alignment_bundle: Option<PathBuf>,
    #[arg(
        long = "align-model",
        visible_alias = "align_model",
        default_value = "facebook/wav2vec2-base-960h"
    )]
    pub(crate) alignment_model: String,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    pub(crate) model_cache_only: bool,
    #[arg(long = "interpolate-method", visible_alias = "interpolate_method", value_enum, default_value_t = CliAlignmentInterpolationMethod::Nearest)]
    pub(crate) interpolate_method: CliAlignmentInterpolationMethod,
    #[arg(
        long = "return-char-alignments",
        visible_alias = "return_char_alignments"
    )]
    pub(crate) return_char_alignments: bool,
    #[arg(long, visible_alias = "speaker_embedding_bundle")]
    pub(crate) speaker_embedding_bundle: Option<PathBuf>,
    #[arg(long, visible_alias = "min_speakers")]
    pub(crate) min_speakers: Option<usize>,
    #[arg(long, visible_alias = "max_speakers")]
    pub(crate) max_speakers: Option<usize>,
    #[arg(long)]
    pub(crate) language: Option<String>,
    #[arg(long, visible_alias = "output_dir")]
    pub(crate) output_dir: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct ParityFixturesArgs {
    pub(crate) manifest: PathBuf,
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,
    #[arg(long, visible_alias = "whisperx_command")]
    pub(crate) whisperx_command: Option<PathBuf>,
    #[arg(long = "output-dir", visible_alias = "output_dir")]
    pub(crate) output_dir: Option<PathBuf>,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    pub(crate) model_cache_only: bool,
    #[arg(long = "case")]
    pub(crate) cases: Vec<String>,
    #[arg(long = "case-timeout-seconds", visible_alias = "case_timeout_seconds")]
    pub(crate) case_timeout_seconds: Option<u64>,
    #[arg(
        long = "require-non-gating-passed",
        visible_alias = "require_non_gating_passed"
    )]
    pub(crate) require_non_gating_passed: bool,
}

#[derive(Debug, Args)]
pub(crate) struct ParityBenchArgs {
    pub(crate) manifest: PathBuf,
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,
    #[arg(long, visible_alias = "whisperx_command")]
    pub(crate) whisperx_command: Option<PathBuf>,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    pub(crate) model_cache_only: bool,
    #[arg(long = "model-revision", visible_alias = "model_revision")]
    pub(crate) model_revision: Option<String>,
    #[arg(long = "iterations", default_value_t = 3)]
    pub(crate) iterations: usize,
    #[arg(long = "warmups", default_value_t = 1)]
    pub(crate) warmups: usize,
    #[arg(long = "case")]
    pub(crate) cases: Vec<String>,
    #[arg(long = "case-timeout-seconds", visible_alias = "case_timeout_seconds")]
    pub(crate) case_timeout_seconds: Option<u64>,
    #[arg(long = "native-only", visible_alias = "native_only")]
    pub(crate) native_only: bool,
    #[arg(long = "report-only", visible_alias = "report_only")]
    pub(crate) report_only: bool,
    #[arg(long)]
    pub(crate) json: bool,
}

#[derive(Debug, Args)]
pub(crate) struct ParitySummaryArgs {
    pub(crate) report: PathBuf,
    #[arg(long = "preflight-report", visible_alias = "preflight_report")]
    pub(crate) preflight_report: Option<PathBuf>,
    #[arg(long = "allow-missing-report", visible_alias = "allow_missing_report")]
    pub(crate) allow_missing_report: bool,
    #[arg(long)]
    pub(crate) suite: Option<String>,
    #[arg(long)]
    pub(crate) features: Option<String>,
    #[arg(long)]
    pub(crate) runner: Option<String>,
    #[arg(long)]
    pub(crate) manifest: Option<PathBuf>,
    #[arg(long = "output-dir", visible_alias = "output_dir")]
    pub(crate) output_dir: Option<PathBuf>,
    #[arg(long = "smoke-root", visible_alias = "smoke_root")]
    pub(crate) smoke_root: Option<PathBuf>,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "whisperx-command", visible_alias = "whisperx_command")]
    pub(crate) whisperx_command: Option<PathBuf>,
    #[arg(long = "progress-log", visible_alias = "progress_log")]
    pub(crate) progress_log: Option<PathBuf>,
    #[arg(long = "ort-dylib-path", visible_alias = "ort_dylib_path")]
    pub(crate) ort_dylib_path: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub(crate) struct ParityBenchSummaryArgs {
    pub(crate) report: PathBuf,
}

#[derive(Debug, Args)]
pub(crate) struct ParityFixtureCaseArgs {
    #[arg(long)]
    pub(crate) fixture: PathBuf,
    #[arg(long)]
    pub(crate) root: PathBuf,
    #[arg(long)]
    pub(crate) report: PathBuf,
}

#[derive(Debug, Args)]
pub(crate) struct ParityBenchCaseArgs {
    #[arg(long)]
    pub(crate) fixture: PathBuf,
    #[arg(long)]
    pub(crate) report: PathBuf,
    #[arg(long)]
    pub(crate) iterations: usize,
    #[arg(long)]
    pub(crate) warmups: usize,
    #[arg(long = "native-only", visible_alias = "native_only")]
    pub(crate) native_only: bool,
}

#[derive(Debug, Args)]
pub(crate) struct ParityBenchMultiInputCaseArgs {
    #[arg(long)]
    pub(crate) fixture: PathBuf,
    #[arg(long)]
    pub(crate) report: PathBuf,
    #[arg(long)]
    pub(crate) iterations: usize,
    #[arg(long)]
    pub(crate) warmups: usize,
    #[arg(long = "native-only", visible_alias = "native_only")]
    pub(crate) native_only: bool,
}

#[derive(Debug, Args)]
pub(crate) struct ParityGoldensArgs {
    pub(crate) manifest: PathBuf,
    #[arg(long)]
    pub(crate) root: Option<PathBuf>,
    #[arg(
        long,
        visible_alias = "whisperx_command",
        default_value = ".audio-tools/whisperx-venv/bin/whisperx"
    )]
    pub(crate) whisperx_command: PathBuf,
    #[arg(long = "model-dir", visible_alias = "model_dir")]
    pub(crate) model_dir: Option<PathBuf>,
    #[arg(long = "model-cache-only", visible_alias = "model_cache_only")]
    pub(crate) model_cache_only: bool,
    #[arg(long = "case")]
    pub(crate) cases: Vec<String>,
    #[arg(long = "include-non-gating", visible_alias = "include_non_gating")]
    pub(crate) include_non_gating: bool,
    #[arg(long)]
    pub(crate) overwrite: bool,
    #[arg(long = "dry-run", visible_alias = "dry_run")]
    pub(crate) dry_run: bool,
}
