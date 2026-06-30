#[cfg(test)]
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use candle_core::{DType, Device, IndexOp, Tensor, D};
use candle_nn::{
    embedding, linear, linear_no_bias, Conv1d, Conv1dConfig, Embedding, LayerNorm, Linear, Module,
    VarBuilder,
};
use candle_transformers::models::whisper::{self};
use serde::Deserialize;
#[cfg(test)]
use text_transcripts::TranscriptWordContract;
use text_transcripts::{TranscriptSegmentContract, TranscriptionContract};
use tokenizers::Tokenizer;
use video_analysis_core::Result;

use crate::native_device::{resolve_native_device, ResolvedNativeDevice};
use crate::{
    candle_batch_count, invalid_request, model_output_mismatch, setup_error, validate_asr_request,
    AsrRequest, AsrResponse, CandleWhisperComputeType, CandleWhisperDecodeRuntime,
    CandleWhisperOptions, SpeechActivitySegment, TranscriptionTask,
};

const REQUIRED_WHISPER_FILES: &[&str] = &[
    "config.json",
    "generation_config.json",
    "tokenizer.json",
    "preprocessor_config.json",
    "model.safetensors",
];
const WHISPER_TIMESTAMP_SECONDS_PER_TOKEN: f64 = 0.02;
const WHISPER_TIMESTAMP_TOKEN_COUNT: u32 =
    (whisper::CHUNK_LENGTH as f64 / WHISPER_TIMESTAMP_SECONDS_PER_TOKEN) as u32 + 1;
const ASR_WINDOW_LEADING_CONTEXT_SECONDS: f64 = 0.25;
const ASR_WINDOW_TRAILING_CONTEXT_SECONDS: f64 = 0.04;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WhisperBundlePaths {
    pub root: PathBuf,
    pub config_json: PathBuf,
    pub generation_config_json: PathBuf,
    pub tokenizer_json: PathBuf,
    pub preprocessor_config_json: PathBuf,
    pub model_safetensors: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WhisperRunSetup {
    model_id: String,
    task: TranscriptionTask,
    language: Option<String>,
    bundle: WhisperBundlePaths,
    model_source: &'static str,
    resolved_device: ResolvedNativeDevice,
    requested_compute_type: CandleWhisperComputeType,
    resolved_compute_type: CandleWhisperComputeType,
    model_weight_dtype: DType,
}

#[derive(Debug, Clone)]
struct ResolvedWhisperModel {
    model_id: String,
    bundle: WhisperBundlePaths,
    source: &'static str,
}

#[derive(Debug, Clone, Deserialize)]
struct GenerationConfig {
    #[serde(default)]
    decoder_start_token_id: Option<u32>,
    #[serde(default)]
    eos_token_id: Option<u32>,
    #[serde(default)]
    forced_decoder_ids: Option<Vec<(usize, Option<u32>)>>,
    #[serde(default)]
    max_length: Option<usize>,
    #[serde(default)]
    lang_to_id: std::collections::BTreeMap<String, u32>,
    #[serde(default)]
    task_to_id: std::collections::BTreeMap<String, u32>,
    #[serde(default)]
    no_timestamps_token_id: Option<u32>,
    #[serde(default)]
    suppress_tokens: Vec<u32>,
    #[serde(default)]
    begin_suppress_tokens: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WhisperDecodeMode {
    WithoutTimestamps,
    TimestampTokens,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WhisperDecodeTimingMode {
    Auto,
    NoTimestamps,
    #[allow(dead_code)]
    TimestampTokensRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WhisperWindowTiming {
    ChunkWindow,
    WhisperTimestampTokens,
}

#[derive(Debug, Clone, PartialEq)]
struct WhisperTimedWindow {
    decoded: WhisperDecodedWindow,
    timing: WhisperWindowTiming,
    fallback_reason: Option<&'static str>,
    diagnostics: WhisperDecodeDiagnostics,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct WhisperDecodeDiagnostics {
    timestamp_tokens_requested: bool,
    timestamp_tokens_present: bool,
    decoded_token_ids: Vec<u32>,
    decoder_prompt_prefill_count: usize,
    decoder_cached_token_step_count: usize,
    decoder_input_token_count: usize,
    generated_token_count: usize,
    decoder_completed_row_count: usize,
    decoder_max_active_row_batch_size: usize,
    decoder_effective_active_batch_sizes: Vec<usize>,
    decoder_active_row_compaction_count: usize,
    decoder_self_attention_cache_reused: bool,
    decoder_cross_attention_cache_reused: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct WhisperTimestampSpec {
    begin_token_id: u32,
    end_token_id: u32,
    seconds_per_token: f64,
}

#[derive(Debug, Clone, PartialEq)]
struct WhisperDecodedWindow {
    text: String,
    segments: Vec<WhisperDecodedSegment>,
}

#[derive(Debug, Clone, PartialEq)]
struct WhisperDecodedSegment {
    text: String,
    start_seconds: f64,
    end_seconds: f64,
    token_ids: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WhisperDecoderInputKind {
    PromptPrefill,
    CachedTokenStep,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WhisperDecoderInput {
    token_ids: Vec<u32>,
    position_offset: usize,
    flush_cache: bool,
    kind: WhisperDecoderInputKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WhisperAutoregressiveRow {
    tokens: Vec<u32>,
    prompt_len: usize,
    cache_position: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ActiveWhisperDecodeRow {
    original_index: usize,
    row: WhisperAutoregressiveRow,
    stats: WhisperGenerationStats,
}

impl WhisperAutoregressiveRow {
    fn new(prompt_tokens: Vec<u32>) -> Self {
        Self {
            tokens: prompt_tokens,
            prompt_len: 0,
            cache_position: 0,
        }
        .with_prompt_len()
    }

    fn with_prompt_len(mut self) -> Self {
        self.prompt_len = self.tokens.len();
        self
    }

    fn next_decoder_input(&self) -> WhisperDecoderInput {
        if self.cache_position == 0 {
            return WhisperDecoderInput {
                token_ids: self.tokens.clone(),
                position_offset: 0,
                flush_cache: true,
                kind: WhisperDecoderInputKind::PromptPrefill,
            };
        }
        let last_token = self
            .tokens
            .last()
            .copied()
            .expect("autoregressive row must retain at least the prompt token");
        WhisperDecoderInput {
            token_ids: vec![last_token],
            position_offset: self.tokens.len() - 1,
            flush_cache: false,
            kind: WhisperDecoderInputKind::CachedTokenStep,
        }
    }

    fn mark_forwarded(&mut self) {
        self.cache_position = self.tokens.len();
    }

    fn generated_tokens(&self) -> &[u32] {
        &self.tokens[self.prompt_len..]
    }

    fn accept(&mut self, token: u32) {
        self.tokens.push(token);
    }

    fn into_generated_tokens(self) -> Vec<u32> {
        self.tokens.into_iter().skip(self.prompt_len).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct WhisperGenerationStats {
    prompt_prefill_count: usize,
    cached_token_step_count: usize,
    decoder_input_token_count: usize,
    generated_token_count: usize,
    completed_row_count: usize,
    max_active_row_batch_size: usize,
    effective_active_batch_sizes: Vec<usize>,
    active_row_compaction_count: usize,
    decoder_self_attention_cache_reused: bool,
    decoder_cross_attention_cache_reused: bool,
}

impl WhisperGenerationStats {
    fn record_input(&mut self, input: &WhisperDecoderInput) {
        match input.kind {
            WhisperDecoderInputKind::PromptPrefill => self.prompt_prefill_count += 1,
            WhisperDecoderInputKind::CachedTokenStep => self.cached_token_step_count += 1,
        }
        self.decoder_input_token_count += input.token_ids.len();
    }

    fn record_generated_token(&mut self) {
        self.generated_token_count += 1;
    }

    fn record_active_row_batch_size(&mut self, batch_size: usize) {
        self.max_active_row_batch_size = self.max_active_row_batch_size.max(batch_size);
        self.effective_active_batch_sizes.push(batch_size);
    }

    fn record_active_row_compaction(&mut self) {
        self.active_row_compaction_count += 1;
    }

    fn record_completed_row(&mut self) {
        self.completed_row_count = 1;
    }

    fn record_decoder_stats(&mut self, stats: CachedWhisperDecoderStats) {
        self.decoder_self_attention_cache_reused |= stats.self_attention_cache_reused;
        self.decoder_cross_attention_cache_reused |= stats.cross_attention_cache_reused;
    }

    fn extend(self, diagnostics: &mut WhisperDecodeDiagnostics) {
        diagnostics.decoder_prompt_prefill_count += self.prompt_prefill_count;
        diagnostics.decoder_cached_token_step_count += self.cached_token_step_count;
        diagnostics.decoder_input_token_count += self.decoder_input_token_count;
        diagnostics.generated_token_count += self.generated_token_count;
        diagnostics.decoder_completed_row_count += self.completed_row_count;
        diagnostics.decoder_max_active_row_batch_size = diagnostics
            .decoder_max_active_row_batch_size
            .max(self.max_active_row_batch_size);
        diagnostics
            .decoder_effective_active_batch_sizes
            .extend(self.effective_active_batch_sizes);
        diagnostics.decoder_active_row_compaction_count += self.active_row_compaction_count;
        diagnostics.decoder_self_attention_cache_reused |= self.decoder_self_attention_cache_reused;
        diagnostics.decoder_cross_attention_cache_reused |=
            self.decoder_cross_attention_cache_reused;
    }
}

impl WhisperDecodeDiagnostics {
    fn add_generation_counts_from(&mut self, other: &Self) {
        self.decoder_prompt_prefill_count += other.decoder_prompt_prefill_count;
        self.decoder_cached_token_step_count += other.decoder_cached_token_step_count;
        self.decoder_input_token_count += other.decoder_input_token_count;
        self.generated_token_count += other.generated_token_count;
        self.decoder_completed_row_count += other.decoder_completed_row_count;
        self.decoder_max_active_row_batch_size = self
            .decoder_max_active_row_batch_size
            .max(other.decoder_max_active_row_batch_size);
        self.decoder_effective_active_batch_sizes
            .extend(other.decoder_effective_active_batch_sizes.iter().copied());
        self.decoder_active_row_compaction_count += other.decoder_active_row_compaction_count;
        self.decoder_self_attention_cache_reused |= other.decoder_self_attention_cache_reused;
        self.decoder_cross_attention_cache_reused |= other.decoder_cross_attention_cache_reused;
    }
}

#[allow(dead_code)]
pub(crate) fn transcribe(
    options: &CandleWhisperOptions,
    request: AsrRequest,
) -> Result<AsrResponse> {
    transcribe_with_load_observer(options, request, |_| {})
}

pub(crate) fn transcribe_with_load_observer(
    options: &CandleWhisperOptions,
    request: AsrRequest,
    on_loaded: impl FnOnce(f64),
) -> Result<AsrResponse> {
    let setup = WhisperRunSetup::from_options_and_request(options, &request)?;
    let load_started = std::time::Instant::now();
    let mut session = CandleWhisperSession::load(setup)?;
    on_loaded(load_started.elapsed().as_secs_f64());
    session.transcribe_chunks(options, request)
}

pub(crate) enum ReusableCandleWhisperSessionEvent {
    LoadStart,
    LoadEnd { duration_seconds: f64 },
    Reuse,
}

pub(crate) struct ReusableCandleWhisperSession {
    session: CandleWhisperSession,
}

impl ReusableCandleWhisperSession {
    pub(crate) fn transcribe(
        current: &mut Option<Self>,
        options: &CandleWhisperOptions,
        request: AsrRequest,
        mut observe: impl FnMut(ReusableCandleWhisperSessionEvent),
    ) -> Result<AsrResponse> {
        let setup = WhisperRunSetup::from_options_and_request(options, &request)?;
        let session_reused = match current.as_ref() {
            Some(existing) if existing.session.setup == setup => true,
            Some(_) | None => {
                observe(ReusableCandleWhisperSessionEvent::LoadStart);
                let load_started = std::time::Instant::now();
                *current = Some(Self {
                    session: CandleWhisperSession::load(setup)?,
                });
                observe(ReusableCandleWhisperSessionEvent::LoadEnd {
                    duration_seconds: load_started.elapsed().as_secs_f64(),
                });
                false
            }
        };
        if session_reused {
            observe(ReusableCandleWhisperSessionEvent::Reuse);
        }
        let session = current
            .as_mut()
            .expect("reusable Candle Whisper session is loaded");
        let mut response = session.session.transcribe_chunks(options, request)?;
        response.diagnostics.push(if session_reused {
            "asrModelSession=reused".to_string()
        } else {
            "asrModelSession=loaded".to_string()
        });
        Ok(response)
    }
}

impl WhisperRunSetup {
    fn from_options_and_request(
        options: &CandleWhisperOptions,
        request: &AsrRequest,
    ) -> Result<Self> {
        validate_asr_request(request)?;
        let model = resolve_whisper_model(options, &request.model_id)?;
        let resolved_device = resolve_native_device(options.device)?;
        let resolved_compute_type = options
            .compute_type
            .resolve_for_device(resolved_device.cuda_active())?;
        let model_weight_dtype = candle_whisper_model_weight_dtype(resolved_compute_type);
        Ok(Self {
            model_id: model.model_id,
            task: request.task,
            language: request
                .language
                .clone()
                .or_else(|| options.language.clone()),
            bundle: model.bundle,
            model_source: model.source,
            resolved_device,
            requested_compute_type: options.compute_type,
            resolved_compute_type,
            model_weight_dtype,
        })
    }
}

fn candle_whisper_model_weight_dtype(compute_type: CandleWhisperComputeType) -> DType {
    match compute_type {
        CandleWhisperComputeType::Automatic => unreachable!("compute type must be resolved first"),
        CandleWhisperComputeType::Fp16 => DType::F16,
        CandleWhisperComputeType::Fp32 => DType::F32,
    }
}

fn candle_dtype_name(dtype: DType) -> &'static str {
    match dtype {
        DType::F16 => "f16",
        DType::F32 => "f32",
        _ => "other",
    }
}

fn resolve_whisper_model(
    options: &CandleWhisperOptions,
    requested_model_id: &str,
) -> Result<ResolvedWhisperModel> {
    let model_id = canonical_whisper_model_id(requested_model_id)?;
    if let Some(bundle) = &options.model_bundle {
        let bundle = resolve_whisper_bundle_paths(bundle)?;
        return Ok(ResolvedWhisperModel {
            model_id,
            bundle,
            source: "explicit-bundle",
        });
    }

    #[cfg(feature = "model-bundles")]
    {
        if options.model_cache_only {
            let bundle = resolve_cached_whisper_model(&model_id, options.model_dir.as_deref())
                .ok_or_else(|| missing_whisper_model_error(&model_id, options))?;
            return Ok(ResolvedWhisperModel {
                model_id,
                bundle,
                source: "hugging-face-cache",
            });
        }

        let mut downloader = model_runtime::HuggingFaceDownloader::new().progress(false);
        if let Some(model_dir) = &options.model_dir {
            downloader = downloader.cache_dir(model_dir.clone());
        }
        let downloaded = downloader
            .download(&whisper_model_spec(&model_id))
            .map_err(|error| missing_whisper_model_error_with_source(&model_id, options, error))?;
        let bundle = downloaded
            .model_dir()
            .ok_or_else(|| {
                setup_error(format!(
                    "native Candle Whisper model `{model_id}` resolved without a local model directory"
                ))
            })
            .and_then(resolve_whisper_bundle_paths)?;
        Ok(ResolvedWhisperModel {
            model_id,
            bundle,
            source: "hugging-face-cache",
        })
    }

    #[cfg(not(feature = "model-bundles"))]
    {
        Err(setup_error(format!(
            "native Candle Whisper model `{model_id}` requires --whisper-bundle or the model-bundles feature for Hugging Face resolution"
        )))
    }
}

fn canonical_whisper_model_id(value: &str) -> Result<String> {
    match value {
        "tiny" => Ok("openai/whisper-tiny".to_string()),
        "tiny.en" => Ok("openai/whisper-tiny.en".to_string()),
        "base" => Ok("openai/whisper-base".to_string()),
        "base.en" => Ok("openai/whisper-base.en".to_string()),
        "small" => Ok("openai/whisper-small".to_string()),
        "small.en" => Ok("openai/whisper-small.en".to_string()),
        "medium" => Ok("openai/whisper-medium".to_string()),
        "medium.en" => Ok("openai/whisper-medium.en".to_string()),
        "large" => Ok("openai/whisper-large-v3".to_string()),
        "large-v1" => Ok("openai/whisper-large-v1".to_string()),
        "large-v2" => Ok("openai/whisper-large-v2".to_string()),
        "large-v3" => Ok("openai/whisper-large-v3".to_string()),
        "large-v3-turbo" => Ok("openai/whisper-large-v3-turbo".to_string()),
        other if looks_like_hf_repo_id(other) => Ok(other.to_string()),
        other => Err(setup_error(format!(
            "unsupported native Candle Whisper model alias `{other}`; native Candle Whisper requires a supported Whisper alias, a Hugging Face repo ID with Candle-compatible files, or --whisper-bundle"
        ))),
    }
}

fn looks_like_hf_repo_id(value: &str) -> bool {
    let mut parts = value.split('/');
    matches!((parts.next(), parts.next(), parts.next()), (Some(owner), Some(repo), None) if !owner.is_empty() && !repo.is_empty())
}

#[cfg(feature = "model-bundles")]
fn resolve_cached_whisper_model(
    model_id: &str,
    model_dir: Option<&Path>,
) -> Option<WhisperBundlePaths> {
    let mut roots = Vec::new();
    if let Some(model_dir) = model_dir {
        roots.push(model_dir.to_path_buf());
    } else if let Some(home) = std::env::var_os("HF_HOME") {
        roots.push(PathBuf::from(home).join("hub"));
    } else if let Some(home) = std::env::var_os("HOME") {
        roots.push(PathBuf::from(home).join(".cache/huggingface/hub"));
    }
    for root in roots {
        for candidate in whisper_cache_candidates(&root, model_id) {
            if let Ok(paths) = resolve_whisper_bundle_paths(&candidate) {
                return Some(paths);
            }
        }
    }
    None
}

#[cfg(feature = "model-bundles")]
fn whisper_cache_candidates(root: &Path, model_id: &str) -> Vec<PathBuf> {
    let mut candidates = vec![root.to_path_buf(), root.join(model_id.replace('/', "--"))];
    let hf_repo_dir = root.join(format!("models--{}", model_id.replace('/', "--")));
    if let Ok(snapshot) = std::fs::read_to_string(hf_repo_dir.join("refs/main")) {
        candidates.push(hf_repo_dir.join("snapshots").join(snapshot.trim()));
    }
    if let Ok(entries) = std::fs::read_dir(hf_repo_dir.join("snapshots")) {
        for entry in entries.flatten() {
            candidates.push(entry.path());
        }
    }
    candidates
}

#[cfg(feature = "model-bundles")]
fn whisper_model_spec(model_id: &str) -> model_runtime::HuggingFaceModelSpec {
    let mut spec = model_runtime::HuggingFaceModelSpec::new(
        model_id.to_string(),
        model_runtime::ModelTask::SpeechRecognition,
    );
    spec.files = REQUIRED_WHISPER_FILES
        .iter()
        .copied()
        .map(model_runtime::ModelFileRequest::required)
        .collect();
    spec
}

fn missing_whisper_model_error(
    model_id: &str,
    options: &CandleWhisperOptions,
) -> video_analysis_core::DetectError {
    setup_error(format!(
        "failed to resolve native Candle Whisper model `{model_id}`; required files: {}; --model-dir={}; cache-only={}",
        REQUIRED_WHISPER_FILES.join(", "),
        options
            .model_dir
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<default huggingface cache>".to_string()),
        options.model_cache_only
    ))
}

fn missing_whisper_model_error_with_source(
    model_id: &str,
    options: &CandleWhisperOptions,
    source: impl std::fmt::Display,
) -> video_analysis_core::DetectError {
    setup_error(format!(
        "failed to resolve native Candle Whisper model `{model_id}`; required files: {}; --model-dir={}; cache-only={}: {source}",
        REQUIRED_WHISPER_FILES.join(", "),
        options
            .model_dir
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<default huggingface cache>".to_string()),
        options.model_cache_only
    ))
}

fn whisper_setup_diagnostics(setup: &WhisperRunSetup) -> Vec<String> {
    vec![
        format!("asrModelResolved={}", setup.bundle.root.display()),
        format!("asrModelSource={}", setup.model_source),
        format!("asrModelId={}", setup.model_id),
        format!(
            "requestedComputeType={}",
            setup.requested_compute_type.as_str()
        ),
        format!(
            "resolvedComputeType={}",
            setup.resolved_compute_type.as_str()
        ),
        format!(
            "modelWeightDtype={}",
            candle_dtype_name(setup.model_weight_dtype)
        ),
    ]
}

pub(crate) fn resolve_whisper_bundle_paths(bundle: &Path) -> Result<WhisperBundlePaths> {
    if !bundle.exists() {
        return Err(setup_error(format!(
            "required Candle Whisper model bundle `{}` is missing",
            bundle.display()
        )));
    }
    Ok(WhisperBundlePaths {
        root: bundle.to_path_buf(),
        config_json: crate::native_bundles::resolve_required_bundle_file(bundle, "config.json")?,
        generation_config_json: crate::native_bundles::resolve_required_bundle_file(
            bundle,
            "generation_config.json",
        )?,
        tokenizer_json: crate::native_bundles::resolve_required_bundle_file(
            bundle,
            "tokenizer.json",
        )?,
        preprocessor_config_json: crate::native_bundles::resolve_required_bundle_file(
            bundle,
            "preprocessor_config.json",
        )?,
        model_safetensors: crate::native_bundles::resolve_required_bundle_file(
            bundle,
            "model.safetensors",
        )?,
    })
}

#[derive(Debug, Clone)]
struct CachedWhisperAttention {
    query: Linear,
    key: Linear,
    value: Linear,
    out: Linear,
    n_head: usize,
    kv_cache: Option<(Tensor, Tensor)>,
}

impl CachedWhisperAttention {
    fn load(n_state: usize, n_head: usize, vb: VarBuilder) -> candle_core::Result<Self> {
        Ok(Self {
            query: linear(n_state, n_state, vb.pp("q_proj"))?,
            key: linear_no_bias(n_state, n_state, vb.pp("k_proj"))?,
            value: linear(n_state, n_state, vb.pp("v_proj"))?,
            out: linear(n_state, n_state, vb.pp("out_proj"))?,
            n_head,
            kv_cache: None,
        })
    }

    fn forward(
        &mut self,
        x: &Tensor,
        xa: Option<&Tensor>,
        mask: Option<&Tensor>,
        flush_cache: bool,
    ) -> candle_core::Result<(Tensor, bool)> {
        let q = self.query.forward(x)?;
        let (k, v, cache_reused) = match xa {
            None => {
                if flush_cache {
                    self.kv_cache = None;
                }
                let current_k = self.key.forward(x)?;
                let current_v = self.value.forward(x)?;
                if let Some((cached_k, cached_v)) = &self.kv_cache {
                    let k = Tensor::cat(&[cached_k, &current_k], 1)?;
                    let v = Tensor::cat(&[cached_v, &current_v], 1)?;
                    self.kv_cache = Some((k.clone(), v.clone()));
                    (k, v, true)
                } else {
                    self.kv_cache = Some((current_k.clone(), current_v.clone()));
                    (current_k, current_v, false)
                }
            }
            Some(x) => {
                if flush_cache {
                    self.kv_cache = None;
                }
                if let Some((k, v)) = &self.kv_cache {
                    (k.clone(), v.clone(), true)
                } else {
                    let k = self.key.forward(x)?;
                    let v = self.value.forward(x)?;
                    self.kv_cache = Some((k.clone(), v.clone()));
                    (k, v, false)
                }
            }
        };
        let wv = self.qkv_attention(&q, &k, &v, mask)?;
        Ok((self.out.forward(&wv)?, cache_reused))
    }

    fn reshape_head(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        let (n_batch, n_ctx, n_state) = x.dims3()?;
        let target_dims = &[n_batch, n_ctx, self.n_head, n_state / self.n_head];
        x.reshape(target_dims)?.transpose(1, 2)
    }

    fn qkv_attention(
        &self,
        q: &Tensor,
        k: &Tensor,
        v: &Tensor,
        mask: Option<&Tensor>,
    ) -> candle_core::Result<Tensor> {
        let (_, _, n_state) = q.dims3()?;
        let scale = ((n_state / self.n_head) as f64).powf(-0.25);
        let q = (self.reshape_head(q)? * scale)?;
        let k = (self.reshape_head(k)?.transpose(2, 3)? * scale)?;
        let v = self.reshape_head(v)?.contiguous()?;
        let mut qk = q.matmul(&k)?;
        if let Some(mask) = mask {
            qk = qk.broadcast_add(mask)?;
        }
        let w = candle_nn::ops::softmax_last_dim(&qk)?;
        w.matmul(&v)?.transpose(1, 2)?.flatten_from(2)
    }

    fn reset_kv_cache(&mut self) {
        self.kv_cache = None;
    }

    fn select_kv_cache_rows(&mut self, row_indices: &Tensor) -> candle_core::Result<()> {
        if let Some((cached_k, cached_v)) = &self.kv_cache {
            self.kv_cache = Some((
                cached_k.index_select(row_indices, 0)?,
                cached_v.index_select(row_indices, 0)?,
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct CachedWhisperBlockStats {
    self_cache_reused: bool,
    cross_cache_reused: bool,
}

#[derive(Debug, Clone)]
struct CachedWhisperBlock {
    attn: CachedWhisperAttention,
    attn_ln: LayerNorm,
    cross_attn: Option<(CachedWhisperAttention, LayerNorm)>,
    mlp_linear1: Linear,
    mlp_linear2: Linear,
    mlp_ln: LayerNorm,
}

impl CachedWhisperBlock {
    fn load(
        n_state: usize,
        n_head: usize,
        cross_attention: bool,
        vb: VarBuilder,
    ) -> candle_core::Result<Self> {
        let cross_attn = if cross_attention {
            Some((
                CachedWhisperAttention::load(n_state, n_head, vb.pp("encoder_attn"))?,
                layer_norm(n_state, vb.pp("encoder_attn_layer_norm"))?,
            ))
        } else {
            None
        };
        Ok(Self {
            attn: CachedWhisperAttention::load(n_state, n_head, vb.pp("self_attn"))?,
            attn_ln: layer_norm(n_state, vb.pp("self_attn_layer_norm"))?,
            cross_attn,
            mlp_linear1: linear(n_state, n_state * 4, vb.pp("fc1"))?,
            mlp_linear2: linear(n_state * 4, n_state, vb.pp("fc2"))?,
            mlp_ln: layer_norm(n_state, vb.pp("final_layer_norm"))?,
        })
    }

    fn forward(
        &mut self,
        x: &Tensor,
        xa: Option<&Tensor>,
        mask: Option<&Tensor>,
        flush_kv_cache: bool,
    ) -> candle_core::Result<(Tensor, CachedWhisperBlockStats)> {
        let (attn, self_cache_reused) =
            self.attn
                .forward(&self.attn_ln.forward(x)?, None, mask, flush_kv_cache)?;
        let mut x = (x + attn)?;
        let mut stats = CachedWhisperBlockStats {
            self_cache_reused,
            cross_cache_reused: false,
        };
        if let Some((attn, ln)) = &mut self.cross_attn {
            let (cross, cross_cache_reused) =
                attn.forward(&ln.forward(&x)?, xa, None, flush_kv_cache)?;
            x = (&x + cross)?;
            stats.cross_cache_reused = cross_cache_reused;
        }
        let mlp = self.mlp_linear2.forward(
            &self
                .mlp_linear1
                .forward(&self.mlp_ln.forward(&x)?)?
                .gelu()?,
        )?;
        Ok(((x + mlp)?, stats))
    }

    fn reset_kv_cache(&mut self) {
        self.attn.reset_kv_cache();
        if let Some((attn, _)) = &mut self.cross_attn {
            attn.reset_kv_cache();
        }
    }

    fn select_kv_cache_rows(&mut self, row_indices: &Tensor) -> candle_core::Result<()> {
        self.attn.select_kv_cache_rows(row_indices)?;
        if let Some((attn, _)) = &mut self.cross_attn {
            attn.select_kv_cache_rows(row_indices)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct CachedWhisperEncoder {
    conv1: Conv1d,
    conv2: Conv1d,
    positional_embedding: Tensor,
    blocks: Vec<CachedWhisperBlock>,
    ln_post: LayerNorm,
}

impl CachedWhisperEncoder {
    fn load(vb: VarBuilder, cfg: &whisper::Config) -> candle_core::Result<Self> {
        let cfg1 = Conv1dConfig {
            padding: 1,
            stride: 1,
            groups: 1,
            dilation: 1,
            cudnn_fwd_algo: None,
        };
        let cfg2 = Conv1dConfig {
            padding: 1,
            stride: 2,
            groups: 1,
            dilation: 1,
            cudnn_fwd_algo: None,
        };
        let n_state = cfg.d_model;
        let n_head = cfg.encoder_attention_heads;
        let conv1 = conv1d(cfg.num_mel_bins, n_state, 3, cfg1, vb.pp("conv1"))?;
        let conv2 = conv1d(n_state, n_state, 3, cfg2, vb.pp("conv2"))?;
        let positional_embedding = sinusoids(cfg.max_source_positions, n_state, vb.device())?;
        let blocks = (0..cfg.encoder_layers)
            .map(|index| {
                CachedWhisperBlock::load(n_state, n_head, false, vb.pp(format!("layers.{index}")))
            })
            .collect::<candle_core::Result<Vec<_>>>()?;
        Ok(Self {
            conv1,
            conv2,
            positional_embedding,
            blocks,
            ln_post: layer_norm(n_state, vb.pp("layer_norm"))?,
        })
    }

    fn forward(&mut self, x: &Tensor, flush_kv_cache: bool) -> candle_core::Result<Tensor> {
        let x = self.conv1.forward(x)?.gelu()?;
        let x = self.conv2.forward(&x)?.gelu()?;
        let x = x.transpose(1, 2)?;
        let (_, seq_len, _) = x.dims3()?;
        let positional_embedding = self.positional_embedding.narrow(0, 0, seq_len)?;
        let mut x = x.broadcast_add(&positional_embedding)?;
        for block in self.blocks.iter_mut() {
            x = block.forward(&x, None, None, flush_kv_cache)?.0;
        }
        self.ln_post.forward(&x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct CachedWhisperDecoderStats {
    self_attention_cache_reused: bool,
    cross_attention_cache_reused: bool,
}

impl CachedWhisperDecoderStats {
    fn merge_block(&mut self, block: CachedWhisperBlockStats) {
        self.self_attention_cache_reused |= block.self_cache_reused;
        self.cross_attention_cache_reused |= block.cross_cache_reused;
    }
}

#[derive(Debug, Clone)]
struct CachedWhisperDecoder {
    token_embedding: Embedding,
    positional_embedding: Tensor,
    blocks: Vec<CachedWhisperBlock>,
    ln: LayerNorm,
}

impl CachedWhisperDecoder {
    fn load(vb: VarBuilder, cfg: &whisper::Config) -> candle_core::Result<Self> {
        let n_state = cfg.d_model;
        let n_head = cfg.decoder_attention_heads;
        let token_embedding = embedding(cfg.vocab_size, n_state, vb.pp("embed_tokens"))?;
        let positional_embedding = vb.get(
            (cfg.max_target_positions, n_state),
            "embed_positions.weight",
        )?;
        let blocks = (0..cfg.decoder_layers)
            .map(|index| {
                CachedWhisperBlock::load(n_state, n_head, true, vb.pp(format!("layers.{index}")))
            })
            .collect::<candle_core::Result<Vec<_>>>()?;
        Ok(Self {
            token_embedding,
            positional_embedding,
            blocks,
            ln: layer_norm(n_state, vb.pp("layer_norm"))?,
        })
    }

    fn forward(
        &mut self,
        x: &Tensor,
        xa: &Tensor,
        position_offset: usize,
        flush_kv_cache: bool,
    ) -> candle_core::Result<(Tensor, CachedWhisperDecoderStats)> {
        let token_count = x.dim(D::Minus1)?;
        let token_embedding = self.token_embedding.forward(x)?;
        let positional_embedding =
            self.positional_embedding
                .narrow(0, position_offset, token_count)?;
        let mut x = token_embedding.broadcast_add(&positional_embedding)?;
        let mask = decoder_causal_mask(
            token_count,
            position_offset + token_count,
            position_offset,
            x.device(),
        )?;
        let mut stats = CachedWhisperDecoderStats::default();
        for block in self.blocks.iter_mut() {
            let (next, block_stats) = block.forward(&x, Some(xa), Some(&mask), flush_kv_cache)?;
            stats.merge_block(block_stats);
            x = next;
        }
        Ok((self.ln.forward(&x)?, stats))
    }

    fn final_linear(&self, x: &Tensor) -> candle_core::Result<Tensor> {
        let batch_size = x.dim(0)?;
        let weight = self
            .token_embedding
            .embeddings()
            .broadcast_left(batch_size)?;
        x.matmul(&weight.t()?)
    }

    fn reset_kv_cache(&mut self) {
        for block in self.blocks.iter_mut() {
            block.reset_kv_cache();
        }
    }

    fn select_kv_cache_rows(&mut self, row_indices: &Tensor) -> candle_core::Result<()> {
        for block in self.blocks.iter_mut() {
            block.select_kv_cache_rows(row_indices)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct CachedWhisper {
    encoder: CachedWhisperEncoder,
    decoder: CachedWhisperDecoder,
    config: whisper::Config,
}

impl CachedWhisper {
    fn load(vb: &VarBuilder, config: whisper::Config) -> candle_core::Result<Self> {
        Ok(Self {
            encoder: CachedWhisperEncoder::load(vb.pp("model.encoder"), &config)?,
            decoder: CachedWhisperDecoder::load(vb.pp("model.decoder"), &config)?,
            config,
        })
    }

    fn reset_kv_cache(&mut self) {
        for block in self.encoder.blocks.iter_mut() {
            block.reset_kv_cache();
        }
        self.decoder.reset_kv_cache();
    }
}

fn conv1d(
    in_channels: usize,
    out_channels: usize,
    kernel_size: usize,
    config: Conv1dConfig,
    vb: VarBuilder,
) -> candle_core::Result<Conv1d> {
    let weight = vb.get((out_channels, in_channels, kernel_size), "weight")?;
    let bias = vb.get(out_channels, "bias")?;
    Ok(Conv1d::new(weight, Some(bias), config))
}

fn layer_norm(size: usize, vb: VarBuilder) -> candle_core::Result<LayerNorm> {
    let weight = vb.get(size, "weight")?;
    let bias = vb.get(size, "bias")?;
    Ok(LayerNorm::new(weight, bias, 1e-5))
}

fn sinusoids(length: usize, channels: usize, device: &Device) -> candle_core::Result<Tensor> {
    let max_timescale = 10000f32;
    let log_timescale_increment = max_timescale.ln() / (channels / 2 - 1) as f32;
    let inv_timescales: Vec<_> = (0..channels / 2)
        .map(|i| (i as f32 * (-log_timescale_increment)).exp())
        .collect();
    let inv_timescales = Tensor::new(inv_timescales.as_slice(), device)?.unsqueeze(0)?;
    let arange = Tensor::arange(0, length as u32, device)?
        .to_dtype(DType::F32)?
        .unsqueeze(1)?;
    let shape = (length, channels / 2);
    let scaled_time = (arange.broadcast_as(shape)? * inv_timescales.broadcast_as(shape)?)?;
    Tensor::cat(&[scaled_time.sin()?, scaled_time.cos()?], 1)
}

fn decoder_causal_mask(
    query_len: usize,
    key_len: usize,
    position_offset: usize,
    device: &Device,
) -> candle_core::Result<Tensor> {
    let values = (0..query_len)
        .flat_map(|query_index| {
            let absolute_query = position_offset + query_index;
            (0..key_len).map(move |key_index| {
                if key_index > absolute_query {
                    f32::NEG_INFINITY
                } else {
                    0.0
                }
            })
        })
        .collect::<Vec<_>>();
    Tensor::from_vec(values, (query_len, key_len), device)
}

struct CandleWhisperSession {
    setup: WhisperRunSetup,
    device: Device,
    model: CachedWhisper,
    tokenizer: Tokenizer,
    generation: GenerationConfig,
    mel_filters: Vec<f32>,
}

impl CandleWhisperSession {
    fn load(setup: WhisperRunSetup) -> Result<Self> {
        let device = candle_device(&setup.resolved_device)?;
        let config: whisper::Config = read_json(&setup.bundle.config_json, "config.json")?;
        let generation: GenerationConfig = read_json(
            &setup.bundle.generation_config_json,
            "generation_config.json",
        )?;
        let _preprocessor: serde_json::Value = read_json(
            &setup.bundle.preprocessor_config_json,
            "preprocessor_config.json",
        )?;
        let tokenizer = Tokenizer::from_file(&setup.bundle.tokenizer_json).map_err(|error| {
            invalid_request(format!(
                "failed to load tokenizer `{}`: {error}",
                setup.bundle.tokenizer_json.display()
            ))
        })?;
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(
                &[setup.bundle.model_safetensors.as_path()],
                setup.model_weight_dtype,
                &device,
            )
        }
        .map_err(|error| {
            setup_error(format!(
                "failed to load Candle Whisper weights `{}`: {error}",
                setup.bundle.model_safetensors.display()
            ))
        })?;
        let model = CachedWhisper::load(&vb, config.clone()).map_err(|error| {
            setup_error(format!(
                "failed to construct Candle Whisper model from `{}`: {error}",
                setup.bundle.root.display()
            ))
        })?;
        let mel_filters =
            mel_filter_bank(config.num_mel_bins, whisper::N_FFT, whisper::SAMPLE_RATE);
        Ok(Self {
            setup,
            device,
            model,
            tokenizer,
            generation,
            mel_filters,
        })
    }

    fn transcribe_chunks(
        &mut self,
        options: &CandleWhisperOptions,
        request: AsrRequest,
    ) -> Result<AsrResponse> {
        let mut segments = Vec::new();
        let mut next_index = 0_u64;
        let mut used_timestamp_tokens = false;
        let mut used_timestamp_word_projection = false;
        let mut timestamp_tokens_requested = false;
        let mut timestamp_tokens_present = false;
        let mut rejected_timestamp_segments = false;
        let mut timing_fallbacks = Vec::new();
        let mut decoder_prompt_prefill_count = 0_usize;
        let mut decoder_cached_token_step_count = 0_usize;
        let mut decoder_input_token_count = 0_usize;
        let mut generated_token_count = 0_usize;
        let mut decoder_completed_row_count = 0_usize;
        let mut decoder_max_active_row_batch_size = 0_usize;
        let mut decoder_effective_active_batch_sizes = Vec::new();
        let mut decoder_active_row_compaction_count = 0_usize;
        let mut decoder_self_attention_cache_reused = false;
        let mut decoder_cross_attention_cache_reused = false;
        let batch_size = candle_batch_size(options, request.chunks.len());
        for batch in request.chunks.chunks(batch_size) {
            let windows =
                collect_chunk_windows(&request.audio.samples, request.audio.sample_rate, batch)?;
            let timed_windows = match options.decode_runtime {
                CandleWhisperDecodeRuntime::AutoregressiveKvCache => windows
                    .iter()
                    .map(|window| {
                        self.decode_window_with_timing_mode(
                            &window.samples,
                            WhisperDecodeTimingMode::Auto,
                        )
                    })
                    .collect::<Result<Vec<_>>>()?,
                CandleWhisperDecodeRuntime::ActiveRowTensorBatch => {
                    self.decode_windows_with_timing_mode(&windows, WhisperDecodeTimingMode::Auto)?
                }
            };
            for (window, timed) in windows.iter().zip(timed_windows) {
                debug_assert!(window.chunk_start_seconds <= window.global_start_seconds);
                debug_assert!(window.local_start_seconds <= window.local_end_seconds);
                timestamp_tokens_requested |= timed.diagnostics.timestamp_tokens_requested;
                timestamp_tokens_present |= timed.diagnostics.timestamp_tokens_present;
                decoder_prompt_prefill_count += timed.diagnostics.decoder_prompt_prefill_count;
                decoder_cached_token_step_count +=
                    timed.diagnostics.decoder_cached_token_step_count;
                decoder_input_token_count += timed.diagnostics.decoder_input_token_count;
                generated_token_count += timed.diagnostics.generated_token_count;
                decoder_completed_row_count += timed.diagnostics.decoder_completed_row_count;
                decoder_max_active_row_batch_size = decoder_max_active_row_batch_size
                    .max(timed.diagnostics.decoder_max_active_row_batch_size);
                decoder_effective_active_batch_sizes.extend(
                    timed
                        .diagnostics
                        .decoder_effective_active_batch_sizes
                        .iter()
                        .copied(),
                );
                decoder_active_row_compaction_count +=
                    timed.diagnostics.decoder_active_row_compaction_count;
                decoder_self_attention_cache_reused |=
                    timed.diagnostics.decoder_self_attention_cache_reused;
                decoder_cross_attention_cache_reused |=
                    timed.diagnostics.decoder_cross_attention_cache_reused;
                if let Some(reason) = timed.fallback_reason {
                    if !timing_fallbacks.contains(&reason) {
                        timing_fallbacks.push(reason);
                    }
                    rejected_timestamp_segments |= reason == "unstableTimestampSegments";
                }
                if timed.timing == WhisperWindowTiming::ChunkWindow {
                    if timed.decoded.text.trim().is_empty() {
                        continue;
                    }
                    segments.push(window_fallback_segment(
                        next_index,
                        timed.decoded.text,
                        window.global_start_seconds,
                        window.global_end_seconds,
                        self.setup.language.clone(),
                    ));
                    next_index += 1;
                } else {
                    used_timestamp_tokens = true;
                    let timestamp_segments = decoded_window_to_contract_segments(
                        timed.decoded,
                        &mut next_index,
                        window.global_start_seconds,
                        window.global_end_seconds,
                        self.setup.language.clone(),
                    );
                    if timestamp_segments
                        .iter()
                        .any(|segment| !segment.words.is_empty())
                    {
                        used_timestamp_word_projection = true;
                    }
                    segments.extend(timestamp_segments);
                }
            }
        }
        let transcript = TranscriptionContract::from_segments(
            request.audio.source,
            request.language.clone(),
            segments,
        )
        .map_err(|error| model_output_mismatch(error.to_string()))?;
        let device_label = device_label(&self.setup.resolved_device);
        let mut diagnostics = whisper_setup_diagnostics(&self.setup);
        diagnostics.extend([
            "provider=candle-whisper".to_string(),
            format!("device={device_label}"),
            format!("modelId={}", self.setup.model_id),
            format!("bundle={}", self.setup.bundle.root.display()),
            format!("cuda={}", device_is_cuda(&self.setup.resolved_device)),
            format!("asrTask={}", self.setup.task.as_whisper_task()),
            if used_timestamp_tokens {
                "timing=whisperTimestampTokens".to_string()
            } else {
                "timing=expandedVadWindow".to_string()
            },
        ]);
        if let Some(language) = &self.setup.language {
            diagnostics.push(format!("language={language}"));
        }
        let observed_batch_execution = observed_candle_batch_execution(
            options.decode_runtime,
            decoder_max_active_row_batch_size,
        );
        if used_timestamp_word_projection {
            diagnostics.push("wordTiming=whisperTimestampProjection".to_string());
        }
        diagnostics.push(format!(
            "timestampTokensRequested={timestamp_tokens_requested}"
        ));
        diagnostics.push(format!("timestampTokensPresent={timestamp_tokens_present}"));
        diagnostics.push(format!(
            "timestampSegmentsRejected={rejected_timestamp_segments}"
        ));
        diagnostics.extend([
            format!("batchExecution={observed_batch_execution}"),
            format!("generation={}", generation_label(observed_batch_execution)),
            format!("completedRowCount={decoder_completed_row_count}"),
            format!("effectiveActiveBatchSize={decoder_max_active_row_batch_size}"),
            format!(
                "effectiveActiveBatchSizes={}",
                format_effective_active_batch_sizes(&decoder_effective_active_batch_sizes)
            ),
            format!("effectiveMaxBatchSize={decoder_max_active_row_batch_size}"),
            format!(
                "activeRowCompaction={}",
                decoder_active_row_compaction_count > 0
            ),
            format!("activeRowCompactionCount={decoder_active_row_compaction_count}"),
            format!(
                "cacheReuse={}",
                format_cache_reuse(
                    decoder_self_attention_cache_reused,
                    decoder_cross_attention_cache_reused
                )
            ),
            format!("decoderPromptPrefillCount={decoder_prompt_prefill_count}"),
            format!("decoderCachedTokenStepCount={decoder_cached_token_step_count}"),
            format!("decoderInputTokenCount={decoder_input_token_count}"),
            format!("generatedTokenCount={generated_token_count}"),
            format!("decoderCompletedRowCount={decoder_completed_row_count}"),
            format!("decoderMaxActiveRowBatchSize={decoder_max_active_row_batch_size}"),
            format!(
                "decoderEffectiveActiveBatchSizes={}",
                format_effective_active_batch_sizes(&decoder_effective_active_batch_sizes)
            ),
            format!("decoderActiveRowCompactionCount={decoder_active_row_compaction_count}"),
            format!(
                "decoderActiveRowCompactionOccurred={}",
                decoder_active_row_compaction_count > 0
            ),
            format!(
                "decoderSelfAttentionCacheReused={}",
                decoder_self_attention_cache_reused
            ),
            format!(
                "decoderCrossAttentionCacheReused={}",
                decoder_cross_attention_cache_reused
            ),
        ]);
        diagnostics.extend(
            timing_fallbacks
                .into_iter()
                .map(|reason| format!("timingFallback={reason}")),
        );
        Ok(AsrResponse {
            model_id: request.model_id,
            language: self
                .setup
                .task
                .output_language_hint()
                .map(str::to_string)
                .or_else(|| self.setup.language.clone()),
            transcript,
            diagnostics,
        })
    }

    fn decode_window_with_timing_mode(
        &mut self,
        samples: &[f32],
        mode: WhisperDecodeTimingMode,
    ) -> Result<WhisperTimedWindow> {
        match mode {
            WhisperDecodeTimingMode::NoTimestamps => {
                let decoded = self.decode_window(samples, WhisperDecodeMode::WithoutTimestamps)?;
                Ok(WhisperTimedWindow {
                    decoded: decoded.window,
                    timing: WhisperWindowTiming::ChunkWindow,
                    fallback_reason: None,
                    diagnostics: decoded.diagnostics,
                })
            }
            WhisperDecodeTimingMode::Auto => {
                if timestamp_spec_for_timing_mode(&self.tokenizer, mode)?.is_some() {
                    let decoded =
                        self.decode_window(samples, WhisperDecodeMode::TimestampTokens)?;
                    let diagnostics = decoded.diagnostics.clone();
                    if has_stable_timestamp_segments(&decoded.window, samples) {
                        return Ok(WhisperTimedWindow {
                            decoded: decoded.window,
                            timing: WhisperWindowTiming::WhisperTimestampTokens,
                            fallback_reason: None,
                            diagnostics,
                        });
                    }
                    let mut fallback = self.decode_window_with_timing_mode(
                        samples,
                        WhisperDecodeTimingMode::NoTimestamps,
                    )?;
                    fallback.fallback_reason = Some("unstableTimestampSegments");
                    fallback
                        .diagnostics
                        .add_generation_counts_from(&diagnostics);
                    fallback.diagnostics.timestamp_tokens_requested =
                        diagnostics.timestamp_tokens_requested;
                    fallback.diagnostics.timestamp_tokens_present =
                        diagnostics.timestamp_tokens_present;
                    fallback.diagnostics.decoded_token_ids = diagnostics.decoded_token_ids;
                    return Ok(fallback);
                }
                let mut fallback = self.decode_window_with_timing_mode(
                    samples,
                    WhisperDecodeTimingMode::NoTimestamps,
                )?;
                fallback.fallback_reason = Some("missingTimestampMetadata");
                Ok(fallback)
            }
            WhisperDecodeTimingMode::TimestampTokensRequired => {
                timestamp_spec_for_timing_mode(&self.tokenizer, mode)?;
                let decoded = self.decode_window(samples, WhisperDecodeMode::TimestampTokens)?;
                let diagnostics = decoded.diagnostics.clone();
                if !has_stable_timestamp_segments(&decoded.window, samples) {
                    return Err(model_output_mismatch(
                        "Whisper timestamp-token decode produced no stable bounded text segments",
                    ));
                }
                Ok(WhisperTimedWindow {
                    decoded: decoded.window,
                    timing: WhisperWindowTiming::WhisperTimestampTokens,
                    fallback_reason: None,
                    diagnostics,
                })
            }
        }
    }

    fn decode_windows_with_timing_mode(
        &mut self,
        windows: &[ChunkWindow],
        mode: WhisperDecodeTimingMode,
    ) -> Result<Vec<WhisperTimedWindow>> {
        if windows.is_empty() {
            return Ok(Vec::new());
        }
        match mode {
            WhisperDecodeTimingMode::NoTimestamps => self
                .decode_window_batch(windows, WhisperDecodeMode::WithoutTimestamps)?
                .into_iter()
                .map(|decoded| {
                    Ok(WhisperTimedWindow {
                        decoded: decoded.window,
                        timing: WhisperWindowTiming::ChunkWindow,
                        fallback_reason: None,
                        diagnostics: decoded.diagnostics,
                    })
                })
                .collect(),
            WhisperDecodeTimingMode::Auto => {
                if timestamp_spec_for_timing_mode(&self.tokenizer, mode)?.is_none() {
                    let mut fallback = self.decode_windows_with_timing_mode(
                        windows,
                        WhisperDecodeTimingMode::NoTimestamps,
                    )?;
                    for timed in &mut fallback {
                        timed.fallback_reason = Some("missingTimestampMetadata");
                    }
                    return Ok(fallback);
                }
                let timestamp_decoded =
                    self.decode_window_batch(windows, WhisperDecodeMode::TimestampTokens)?;
                let mut results: Vec<Option<WhisperTimedWindow>> = vec![None; windows.len()];
                let mut fallback_indices = Vec::new();
                for (index, (window, decoded)) in windows.iter().zip(timestamp_decoded).enumerate()
                {
                    let diagnostics = decoded.diagnostics.clone();
                    if has_stable_timestamp_segments(&decoded.window, &window.samples) {
                        results[index] = Some(WhisperTimedWindow {
                            decoded: decoded.window,
                            timing: WhisperWindowTiming::WhisperTimestampTokens,
                            fallback_reason: None,
                            diagnostics,
                        });
                    } else {
                        fallback_indices.push((index, diagnostics));
                    }
                }
                if !fallback_indices.is_empty() {
                    let fallback_windows = fallback_indices
                        .iter()
                        .map(|(index, _)| windows[*index].clone())
                        .collect::<Vec<_>>();
                    let fallbacks = self.decode_windows_with_timing_mode(
                        &fallback_windows,
                        WhisperDecodeTimingMode::NoTimestamps,
                    )?;
                    for ((index, timestamp_diagnostics), mut fallback) in
                        fallback_indices.into_iter().zip(fallbacks)
                    {
                        fallback.fallback_reason = Some("unstableTimestampSegments");
                        fallback
                            .diagnostics
                            .add_generation_counts_from(&timestamp_diagnostics);
                        fallback.diagnostics.timestamp_tokens_requested =
                            timestamp_diagnostics.timestamp_tokens_requested;
                        fallback.diagnostics.timestamp_tokens_present =
                            timestamp_diagnostics.timestamp_tokens_present;
                        fallback.diagnostics.decoded_token_ids =
                            timestamp_diagnostics.decoded_token_ids;
                        results[index] = Some(fallback);
                    }
                }
                Ok(results
                    .into_iter()
                    .map(|result| result.expect("every batched Whisper window is decoded"))
                    .collect())
            }
            WhisperDecodeTimingMode::TimestampTokensRequired => {
                timestamp_spec_for_timing_mode(&self.tokenizer, mode)?;
                let decoded =
                    self.decode_window_batch(windows, WhisperDecodeMode::TimestampTokens)?;
                decoded
                    .into_iter()
                    .zip(windows)
                    .map(|(decoded, window)| {
                        let diagnostics = decoded.diagnostics.clone();
                        if !has_stable_timestamp_segments(&decoded.window, &window.samples) {
                            return Err(model_output_mismatch(
                                "Whisper timestamp-token decode produced no stable bounded text segments",
                            ));
                        }
                        Ok(WhisperTimedWindow {
                            decoded: decoded.window,
                            timing: WhisperWindowTiming::WhisperTimestampTokens,
                            fallback_reason: None,
                            diagnostics,
                        })
                    })
                    .collect()
            }
        }
    }

    fn decode_window(
        &mut self,
        samples: &[f32],
        mode: WhisperDecodeMode,
    ) -> Result<WhisperDecodeOutput> {
        self.decode_window_batch(
            &[ChunkWindow {
                samples: samples.to_vec(),
                chunk_start_seconds: 0.0,
                local_start_seconds: 0.0,
                local_end_seconds: samples.len() as f64 / whisper::SAMPLE_RATE as f64,
                global_start_seconds: 0.0,
                global_end_seconds: samples.len() as f64 / whisper::SAMPLE_RATE as f64,
            }],
            mode,
        )
        .map(|mut outputs| outputs.remove(0))
    }

    fn decode_window_batch(
        &mut self,
        windows: &[ChunkWindow],
        mode: WhisperDecodeMode,
    ) -> Result<Vec<WhisperDecodeOutput>> {
        let audio_features = self.encode_window_batch(windows)?;
        let token_outputs = self.decode_tokens_batch(&audio_features, windows.len(), mode)?;
        token_outputs
            .into_iter()
            .map(|(token_ids, generation_stats)| {
                self.tokens_to_decode_output(token_ids, generation_stats, mode)
            })
            .collect()
    }

    fn encode_window_batch(&mut self, windows: &[ChunkWindow]) -> Result<Tensor> {
        if should_microbatch_encoder(&self.setup.resolved_device, windows.len()) {
            return self.encode_windows_individually(windows);
        }
        let mel = self.mel_tensor_batch(windows)?;
        self.model
            .encoder
            .forward(&mel, true)
            .map_err(|error| model_output_mismatch(format!("Whisper encoder failed: {error}")))
    }

    fn encode_windows_individually(&mut self, windows: &[ChunkWindow]) -> Result<Tensor> {
        let mut encoded = Vec::with_capacity(windows.len());
        for window in windows {
            let mel = self.mel_tensor_batch(std::slice::from_ref(window))?;
            let features = self.model.encoder.forward(&mel, true).map_err(|error| {
                model_output_mismatch(format!("Whisper encoder failed: {error}"))
            })?;
            encoded.push(features);
        }
        let encoded = encoded.iter().collect::<Vec<_>>();
        Tensor::cat(&encoded, 0).map_err(|error| {
            model_output_mismatch(format!("failed to stack Whisper encoder features: {error}"))
        })
    }

    fn mel_tensor_batch(&self, windows: &[ChunkWindow]) -> Result<Tensor> {
        let n_mel = self.model.config.num_mel_bins;
        let mut features = Vec::with_capacity(windows.len() * n_mel * whisper::N_FRAMES);
        for window in windows {
            let mel =
                whisper::audio::pcm_to_mel(&self.model.config, &window.samples, &self.mel_filters);
            let mel_frames = mel.len() / n_mel;
            for mel_index in 0..n_mel {
                let row_start = mel_index * mel_frames;
                let available = mel_frames.min(whisper::N_FRAMES);
                features.extend_from_slice(&mel[row_start..row_start + available]);
                if available < whisper::N_FRAMES {
                    features.extend(std::iter::repeat_n(0.0, whisper::N_FRAMES - available));
                }
            }
        }
        Tensor::from_vec(
            features,
            (windows.len(), n_mel, whisper::N_FRAMES),
            &self.device,
        )
        .map_err(|error| model_output_mismatch(format!("failed to build mel tensor: {error}")))
    }

    fn tokens_to_decode_output(
        &self,
        token_ids: Vec<u32>,
        generation_stats: WhisperGenerationStats,
        mode: WhisperDecodeMode,
    ) -> Result<WhisperDecodeOutput> {
        let mut diagnostics = WhisperDecodeDiagnostics {
            timestamp_tokens_requested: mode == WhisperDecodeMode::TimestampTokens,
            timestamp_tokens_present: timestamp_spec_for_timing_mode(
                &self.tokenizer,
                WhisperDecodeTimingMode::Auto,
            )?
            .is_some_and(|spec| {
                token_ids
                    .iter()
                    .any(|token| timestamp_seconds(*token, &spec).is_some())
            }),
            decoded_token_ids: token_ids.clone(),
            ..WhisperDecodeDiagnostics::default()
        };
        generation_stats.extend(&mut diagnostics);
        match mode {
            WhisperDecodeMode::WithoutTimestamps => Ok(WhisperDecodeOutput {
                window: WhisperDecodedWindow {
                    text: decode_text_tokens(&self.tokenizer, &token_ids)?,
                    segments: Vec::new(),
                },
                diagnostics,
            }),
            WhisperDecodeMode::TimestampTokens => {
                decode_timestamp_window(&self.tokenizer, &token_ids)?
                    .map(|window| {
                        Ok(WhisperDecodeOutput {
                            window,
                            diagnostics: diagnostics.clone(),
                        })
                    })
                    .unwrap_or_else(|| {
                        Ok(WhisperDecodeOutput {
                            window: WhisperDecodedWindow {
                                text: decode_text_tokens(&self.tokenizer, &token_ids)?,
                                segments: Vec::new(),
                            },
                            diagnostics,
                        })
                    })
            }
        }
    }

    fn decode_tokens_batch(
        &mut self,
        audio_features: &Tensor,
        row_count: usize,
        mode: WhisperDecodeMode,
    ) -> Result<Vec<(Vec<u32>, WhisperGenerationStats)>> {
        self.model.reset_kv_cache();
        let eos = self.eos_token_id()?;
        let max_length = self
            .generation
            .max_length
            .unwrap_or(self.model.config.max_target_positions)
            .min(self.model.config.max_target_positions);
        let initial_tokens = self.initial_tokens(mode)?;
        let mut active_rows = (0..row_count)
            .map(|original_index| ActiveWhisperDecodeRow {
                original_index,
                row: WhisperAutoregressiveRow::new(initial_tokens.clone()),
                stats: WhisperGenerationStats::default(),
            })
            .collect::<Vec<_>>();
        let mut active_features = audio_features.clone();
        let mut completed: Vec<Option<(Vec<u32>, WhisperGenerationStats)>> = vec![None; row_count];

        while !active_rows.is_empty() && active_rows[0].row.tokens.len() < max_length {
            let active_len_before_step = active_rows.len();
            let input = active_rows[0].row.next_decoder_input();
            debug_assert!(active_rows.iter().all(|active| active
                .row
                .next_decoder_input()
                .token_ids
                .len()
                == input.token_ids.len()));
            for active in &mut active_rows {
                active.stats.record_input(&input);
                active
                    .stats
                    .record_active_row_batch_size(active_len_before_step);
            }
            let token_ids = active_rows
                .iter()
                .flat_map(|active| active.row.next_decoder_input().token_ids)
                .collect::<Vec<_>>();
            let token_tensor = Tensor::from_vec(
                token_ids,
                (active_rows.len(), input.token_ids.len()),
                &self.device,
            )
            .map_err(|error| {
                model_output_mismatch(format!("failed to build batched token tensor: {error}"))
            })?;
            let (decoded, decoder_stats) = self
                .model
                .decoder
                .forward(
                    &token_tensor,
                    &active_features,
                    input.position_offset,
                    input.flush_cache,
                )
                .map_err(|error| {
                    model_output_mismatch(format!("Whisper batched decoder failed: {error}"))
                })?;
            for active in &mut active_rows {
                active.stats.record_decoder_stats(decoder_stats);
                active.row.mark_forwarded();
            }
            let logits = self.model.decoder.final_linear(&decoded).map_err(|error| {
                model_output_mismatch(format!("Whisper batched logits projection failed: {error}"))
            })?;
            let seq_index = input.token_ids.len() - 1;
            let mut next_tokens = Vec::with_capacity(active_rows.len());
            for (active_index, active) in std::mem::take(&mut active_rows).into_iter().enumerate() {
                let mut next_logits = logits
                    .i((active_index, seq_index, ..))
                    .and_then(|logits| logits.to_dtype(DType::F32))
                    .and_then(|logits| logits.to_vec1::<f32>())
                    .map_err(|error| {
                        model_output_mismatch(format!(
                            "Whisper batched greedy decode failed: {error}"
                        ))
                    })?;
                self.apply_logit_filters(&mut next_logits, mode, active.row.generated_tokens())?;
                let next = argmax_finite(&next_logits).ok_or_else(|| {
                    model_output_mismatch(
                        "Whisper logits were fully suppressed during batched decode",
                    )
                })? as u32;
                next_tokens.push((active, next));
            }
            let (mut survivors, survivor_indices) =
                apply_active_row_decisions(next_tokens, eos, &mut completed)?;
            if survivors.is_empty() {
                break;
            }
            if survivors.len() < active_len_before_step {
                if let Some(survivor) = survivors.first_mut() {
                    survivor.stats.record_active_row_compaction();
                }
                let row_indices = Tensor::new(survivor_indices.as_slice(), &self.device)
                    .and_then(|indices| indices.to_dtype(DType::I64))
                    .map_err(|error| {
                        model_output_mismatch(format!(
                            "failed to build Whisper active-row index tensor: {error}"
                        ))
                    })?;
                active_features =
                    active_features
                        .index_select(&row_indices, 0)
                        .map_err(|error| {
                            model_output_mismatch(format!(
                                "failed to compact Whisper encoder features: {error}"
                            ))
                        })?;
                self.model
                    .decoder
                    .select_kv_cache_rows(&row_indices)
                    .map_err(|error| {
                        model_output_mismatch(format!(
                            "failed to compact Whisper decoder KV cache: {error}"
                        ))
                    })?;
            }
            active_rows = survivors;
        }

        for mut active in active_rows {
            active.stats.record_completed_row();
            completed[active.original_index] =
                Some((active.row.into_generated_tokens(), active.stats));
        }
        completed
            .into_iter()
            .map(|result| {
                result.ok_or_else(|| model_output_mismatch("missing Whisper batch row result"))
            })
            .collect()
    }

    fn apply_logit_filters(
        &self,
        logits: &mut [f32],
        mode: WhisperDecodeMode,
        generated: &[u32],
    ) -> Result<()> {
        for token in &self.generation.suppress_tokens {
            suppress_token(logits, *token);
        }
        if generated.is_empty() {
            for token in &self.generation.begin_suppress_tokens {
                suppress_token(logits, *token);
            }
        }
        let no_timestamps = self
            .generation
            .no_timestamps_token_id
            .or_else(|| token_id(&self.tokenizer, whisper::NO_TIMESTAMPS_TOKEN));
        if let Some(no_timestamps) = no_timestamps {
            suppress_token(logits, no_timestamps);
        }
        let Some(spec) =
            timestamp_spec_for_timing_mode(&self.tokenizer, WhisperDecodeTimingMode::Auto)?
        else {
            return Ok(());
        };
        match mode {
            WhisperDecodeMode::WithoutTimestamps => {
                suppress_range(logits, spec.begin_token_id, spec.end_token_id);
            }
            WhisperDecodeMode::TimestampTokens => {
                apply_timestamp_logit_rules(logits, generated, &spec, self.eos_token_id()?)?;
            }
        }
        Ok(())
    }

    fn initial_tokens(&self, mode: WhisperDecodeMode) -> Result<Vec<u32>> {
        Self::initial_prompt_tokens_for_mode(
            &self.generation,
            &self.tokenizer,
            self.setup.language.as_deref(),
            self.setup.task,
            mode,
        )
    }

    #[cfg(test)]
    fn initial_prompt_tokens(
        generation: &GenerationConfig,
        tokenizer: &Tokenizer,
        language: Option<&str>,
    ) -> Result<Vec<u32>> {
        Self::initial_prompt_tokens_for_mode(
            generation,
            tokenizer,
            language,
            TranscriptionTask::Transcribe,
            WhisperDecodeMode::WithoutTimestamps,
        )
    }

    #[cfg(test)]
    fn initial_prompt_tokens_for_task(
        generation: &GenerationConfig,
        tokenizer: &Tokenizer,
        language: Option<&str>,
        task: TranscriptionTask,
    ) -> Result<Vec<u32>> {
        Self::initial_prompt_tokens_for_mode(
            generation,
            tokenizer,
            language,
            task,
            WhisperDecodeMode::WithoutTimestamps,
        )
    }

    fn initial_prompt_tokens_for_mode(
        generation: &GenerationConfig,
        tokenizer: &Tokenizer,
        language: Option<&str>,
        task: TranscriptionTask,
        mode: WhisperDecodeMode,
    ) -> Result<Vec<u32>> {
        let decoder_start = Self::decoder_start_token_id(generation, tokenizer)?;
        let mut tokens = vec![decoder_start];
        if let Some(language) = language {
            let token = Self::language_token_id(generation, tokenizer, language).ok_or_else(|| {
                invalid_request(format!(
                    "Whisper generation config/tokenizer does not define language token `{language}`"
                ))
            })?;
            tokens.push(token);
        }
        let task_token = Self::task_token_id(generation, tokenizer, task.as_whisper_task())
            .ok_or_else(|| {
                invalid_request(format!(
                    "Whisper generation config/tokenizer is missing {} task token",
                    task.as_whisper_task()
                ))
            })?;
        tokens.push(task_token);
        let no_timestamps = token_id(tokenizer, whisper::NO_TIMESTAMPS_TOKEN);
        if mode == WhisperDecodeMode::WithoutTimestamps {
            tokens.push(no_timestamps.ok_or_else(|| {
                invalid_request("Whisper tokenizer is missing no-timestamps token")
            })?);
        }
        if let Some(forced) = &generation.forced_decoder_ids {
            for (position, token) in forced {
                let Some(token) = token else {
                    continue;
                };
                if mode == WhisperDecodeMode::TimestampTokens
                    && no_timestamps.is_some_and(|no_timestamps| no_timestamps == *token)
                {
                    continue;
                }
                if *position < tokens.len() {
                    tokens[*position] = *token;
                } else {
                    while tokens.len() < *position {
                        tokens.push(decoder_start);
                    }
                    tokens.push(*token);
                }
            }
        }
        Ok(tokens)
    }

    fn decoder_start_token_id(generation: &GenerationConfig, tokenizer: &Tokenizer) -> Result<u32> {
        generation
            .decoder_start_token_id
            .or_else(|| token_id(tokenizer, whisper::SOT_TOKEN))
            .ok_or_else(|| {
                invalid_request("Whisper generation config is missing decoder_start_token_id")
            })
    }

    fn eos_token_id(&self) -> Result<u32> {
        Self::resolve_eos_token_id(&self.generation, &self.tokenizer)
    }

    fn resolve_eos_token_id(generation: &GenerationConfig, tokenizer: &Tokenizer) -> Result<u32> {
        generation
            .eos_token_id
            .or_else(|| token_id(tokenizer, whisper::EOT_TOKEN))
            .ok_or_else(|| invalid_request("Whisper generation config is missing eos_token_id"))
    }

    fn language_token_id(
        generation: &GenerationConfig,
        tokenizer: &Tokenizer,
        language: &str,
    ) -> Option<u32> {
        let normalized = language.trim().to_lowercase();
        let wrapped = format!("<|{normalized}|>");
        generation
            .lang_to_id
            .get(&wrapped)
            .or_else(|| generation.lang_to_id.get(&normalized))
            .copied()
            .or_else(|| token_id(tokenizer, &wrapped))
    }

    fn task_token_id(
        generation: &GenerationConfig,
        tokenizer: &Tokenizer,
        task: &str,
    ) -> Option<u32> {
        let wrapped = format!("<|{task}|>");
        generation
            .task_to_id
            .get(&wrapped)
            .or_else(|| generation.task_to_id.get(task))
            .copied()
            .or_else(|| token_id(tokenizer, &wrapped))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct WhisperDecodeOutput {
    window: WhisperDecodedWindow,
    diagnostics: WhisperDecodeDiagnostics,
}

fn candle_batch_size(options: &CandleWhisperOptions, chunk_count: usize) -> usize {
    match options.decode_runtime {
        CandleWhisperDecodeRuntime::AutoregressiveKvCache => {}
        CandleWhisperDecodeRuntime::ActiveRowTensorBatch => {
            return options.max_batch_size.unwrap_or(chunk_count.max(1)).max(1);
        }
    }
    if !options.batch_chunks {
        return 1;
    }
    if candle_batch_count(options, chunk_count) <= 1 {
        return chunk_count.max(1);
    }
    options.max_batch_size.unwrap_or(chunk_count.max(1)).max(1)
}

fn generation_label(observed_batch_execution: &str) -> &'static str {
    match observed_batch_execution {
        crate::CANDLE_WHISPER_ACTIVE_ROW_TENSOR_BATCH_EXECUTION => "active-row-tensor-batch",
        _ => "autoregressive-kv-cache",
    }
}

fn observed_candle_batch_execution(
    runtime: CandleWhisperDecodeRuntime,
    decoder_max_active_row_batch_size: usize,
) -> &'static str {
    if runtime == CandleWhisperDecodeRuntime::ActiveRowTensorBatch
        && decoder_max_active_row_batch_size > 1
    {
        return crate::CANDLE_WHISPER_ACTIVE_ROW_TENSOR_BATCH_EXECUTION;
    }
    crate::CANDLE_WHISPER_AUTOREGRESSIVE_KV_CACHE_EXECUTION
}

fn format_effective_active_batch_sizes(sizes: &[usize]) -> String {
    if sizes.is_empty() {
        return "none".to_string();
    }
    let mut sizes = sizes.to_vec();
    sizes.sort_unstable();
    sizes.dedup();
    sizes
        .into_iter()
        .map(|size| size.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn format_cache_reuse(self_attention: bool, cross_attention: bool) -> &'static str {
    match (self_attention, cross_attention) {
        (true, true) => "self-and-cross-attention",
        (true, false) => "self-attention",
        (false, true) => "cross-attention",
        (false, false) => "none",
    }
}

fn collect_chunk_windows(
    samples: &[f32],
    sample_rate: u32,
    chunks: &[SpeechActivitySegment],
) -> Result<Vec<ChunkWindow>> {
    let mut windows = Vec::new();
    for chunk in chunks {
        windows.extend(chunk_windows(samples, sample_rate, chunk)?);
    }
    Ok(windows)
}

fn apply_active_row_decisions(
    next_tokens: Vec<(ActiveWhisperDecodeRow, u32)>,
    eos: u32,
    completed: &mut [Option<(Vec<u32>, WhisperGenerationStats)>],
) -> Result<(Vec<ActiveWhisperDecodeRow>, Vec<u32>)> {
    let mut survivors = Vec::new();
    let mut survivor_indices = Vec::new();
    for (active_index, (mut active, next)) in next_tokens.into_iter().enumerate() {
        if next == eos {
            let original_index = active.original_index;
            if completed.get(original_index).is_none() {
                return Err(model_output_mismatch(
                    "Whisper active row completed outside the result range",
                ));
            }
            active.stats.record_completed_row();
            completed[original_index] = Some((active.row.into_generated_tokens(), active.stats));
        } else {
            active.row.accept(next);
            active.stats.record_generated_token();
            survivor_indices.push(active_index as u32);
            survivors.push(active);
        }
    }
    Ok((survivors, survivor_indices))
}

fn decode_timestamp_window(
    tokenizer: &Tokenizer,
    token_ids: &[u32],
) -> Result<Option<WhisperDecodedWindow>> {
    let spec = whisper_timestamp_spec(tokenizer)?;
    decode_timestamp_window_with_spec(tokenizer, token_ids, &spec)
}

fn decode_timestamp_window_with_spec(
    tokenizer: &Tokenizer,
    token_ids: &[u32],
    spec: &WhisperTimestampSpec,
) -> Result<Option<WhisperDecodedWindow>> {
    let mut segments = Vec::new();
    let mut pending_text_tokens = Vec::new();
    let mut segment_start = None;
    let mut previous_timestamp = None;
    let mut saw_timestamp = false;

    for token_id in token_ids {
        if let Some(seconds) = timestamp_seconds(*token_id, spec) {
            saw_timestamp = true;
            if let Some(previous) = previous_timestamp {
                if seconds < previous {
                    return Err(model_output_mismatch(format!(
                        "Whisper timestamp tokens are not monotonic: {seconds:.2} after {previous:.2}"
                    )));
                }
            }
            if !pending_text_tokens.is_empty() {
                let start_seconds = segment_start.unwrap_or(seconds);
                if seconds < start_seconds {
                    return Err(model_output_mismatch(format!(
                        "Whisper timestamp segment ends before it starts: {seconds:.2} < {start_seconds:.2}"
                    )));
                }
                let text = decode_text_tokens(tokenizer, &pending_text_tokens)?;
                if !text.is_empty() {
                    segments.push(WhisperDecodedSegment {
                        text,
                        start_seconds,
                        end_seconds: seconds,
                        token_ids: std::mem::take(&mut pending_text_tokens),
                    });
                } else {
                    pending_text_tokens.clear();
                }
            }
            segment_start = Some(seconds);
            previous_timestamp = Some(seconds);
        } else {
            pending_text_tokens.push(*token_id);
        }
    }

    if !pending_text_tokens.is_empty() {
        return Ok(None);
    }
    if !saw_timestamp {
        return Ok(None);
    }

    let text = segments
        .iter()
        .map(|segment| segment.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    Ok(Some(WhisperDecodedWindow { text, segments }))
}

fn whisper_timestamp_spec(tokenizer: &Tokenizer) -> Result<WhisperTimestampSpec> {
    let begin_token_id = token_id(tokenizer, "<|0.00|>").ok_or_else(|| {
        invalid_request("Whisper tokenizer is missing timestamp token `<|0.00|>`")
    })?;
    let end_token_id = token_id(tokenizer, "<|30.00|>")
        .map(|token| token + 1)
        .unwrap_or(begin_token_id + WHISPER_TIMESTAMP_TOKEN_COUNT);
    if end_token_id <= begin_token_id {
        return Err(invalid_request(
            "Whisper timestamp token range is empty or malformed",
        ));
    }
    Ok(WhisperTimestampSpec {
        begin_token_id,
        end_token_id,
        seconds_per_token: WHISPER_TIMESTAMP_SECONDS_PER_TOKEN,
    })
}

fn optional_whisper_timestamp_spec(tokenizer: &Tokenizer) -> Result<Option<WhisperTimestampSpec>> {
    if token_id(tokenizer, "<|0.00|>").is_none() {
        return Ok(None);
    }
    whisper_timestamp_spec(tokenizer).map(Some)
}

fn timestamp_spec_for_timing_mode(
    tokenizer: &Tokenizer,
    mode: WhisperDecodeTimingMode,
) -> Result<Option<WhisperTimestampSpec>> {
    match mode {
        WhisperDecodeTimingMode::Auto => optional_whisper_timestamp_spec(tokenizer),
        WhisperDecodeTimingMode::NoTimestamps => Ok(None),
        WhisperDecodeTimingMode::TimestampTokensRequired => {
            whisper_timestamp_spec(tokenizer).map(Some)
        }
    }
}

fn timestamp_seconds(token_id: u32, spec: &WhisperTimestampSpec) -> Option<f64> {
    (spec.begin_token_id..spec.end_token_id)
        .contains(&token_id)
        .then(|| (token_id - spec.begin_token_id) as f64 * spec.seconds_per_token)
}

fn apply_timestamp_logit_rules(
    logits: &mut [f32],
    generated: &[u32],
    spec: &WhisperTimestampSpec,
    eos: u32,
) -> Result<()> {
    let begin = spec.begin_token_id;
    let end = spec.end_token_id;
    let last_was_timestamp = generated
        .last()
        .is_some_and(|token| timestamp_seconds(*token, spec).is_some());
    let penultimate_was_timestamp = generated
        .get(generated.len().saturating_sub(2))
        .is_none_or(|token| timestamp_seconds(*token, spec).is_some());
    if last_was_timestamp {
        if penultimate_was_timestamp {
            suppress_range(logits, begin, end);
        } else {
            suppress_range(logits, 0, eos);
        }
    }
    if generated.is_empty() {
        let max_initial_timestamp = begin + (1.0 / spec.seconds_per_token).round() as u32;
        suppress_range(logits, max_initial_timestamp + 1, end);
    }
    if let Some(last_timestamp) = generated
        .iter()
        .rev()
        .find(|token| timestamp_seconds(**token, spec).is_some())
    {
        suppress_range(logits, begin, *last_timestamp);
    }
    let timestamp_logprob = logsumexp_range(logits, begin, end);
    let max_text_logprob = max_finite_range(logits, 0, begin);
    if let (Some(timestamp_logprob), Some(max_text_logprob)) = (timestamp_logprob, max_text_logprob)
    {
        if timestamp_logprob > max_text_logprob {
            suppress_range(logits, 0, begin);
        }
    }
    Ok(())
}

fn suppress_token(logits: &mut [f32], token: u32) {
    if let Some(logit) = logits.get_mut(token as usize) {
        *logit = f32::NEG_INFINITY;
    }
}

fn suppress_range(logits: &mut [f32], start: u32, end: u32) {
    let start = start as usize;
    let end = (end as usize).min(logits.len());
    if start >= end {
        return;
    }
    for logit in &mut logits[start..end] {
        *logit = f32::NEG_INFINITY;
    }
}

fn argmax_finite(logits: &[f32]) -> Option<usize> {
    logits
        .iter()
        .enumerate()
        .filter(|(_, logit)| logit.is_finite())
        .max_by(|(_, left), (_, right)| left.total_cmp(right))
        .map(|(index, _)| index)
}

fn max_finite_range(logits: &[f32], start: u32, end: u32) -> Option<f32> {
    let start = start as usize;
    let end = (end as usize).min(logits.len());
    if start >= end {
        return None;
    }
    logits[start..end]
        .iter()
        .copied()
        .filter(|logit| logit.is_finite())
        .max_by(f32::total_cmp)
}

fn logsumexp_range(logits: &[f32], start: u32, end: u32) -> Option<f32> {
    let start = start as usize;
    let end = (end as usize).min(logits.len());
    if start >= end {
        return None;
    }
    let max = logits[start..end]
        .iter()
        .copied()
        .filter(|logit| logit.is_finite())
        .max_by(f32::total_cmp)?;
    let sum = logits[start..end]
        .iter()
        .copied()
        .filter(|logit| logit.is_finite())
        .map(|logit| (logit - max).exp())
        .sum::<f32>();
    (sum > 0.0).then(|| max + sum.ln())
}

fn has_stable_timestamp_segments(decoded: &WhisperDecodedWindow, samples: &[f32]) -> bool {
    let audio_duration = samples.len() as f64 / whisper::SAMPLE_RATE as f64;
    let joined_text = decoded
        .segments
        .iter()
        .map(|segment| segment.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    !joined_text.trim().is_empty()
        && joined_text
            .trim()
            .chars()
            .any(|character| character.is_alphanumeric())
        && decoded.segments.iter().all(|segment| {
            !segment.text.trim().is_empty()
                && segment.end_seconds > segment.start_seconds
                && segment.start_seconds >= 0.0
                && segment.end_seconds <= audio_duration + 0.5
        })
        && decoded
            .segments
            .last()
            .is_some_and(|segment| segment.end_seconds >= audio_duration * 0.85)
}

fn decode_text_tokens(tokenizer: &Tokenizer, token_ids: &[u32]) -> Result<String> {
    tokenizer
        .decode(token_ids, true)
        .map(clean_decoded_text)
        .map_err(|error| model_output_mismatch(format!("failed to decode Whisper tokens: {error}")))
}

fn clean_decoded_text(text: String) -> String {
    text.replace("  ", " ").trim().to_string()
}

fn window_fallback_segment(
    index: u64,
    text: String,
    start_seconds: f64,
    end_seconds: f64,
    language: Option<String>,
) -> TranscriptSegmentContract {
    let mut segment = TranscriptSegmentContract::new(index, text);
    segment.start_seconds = Some(start_seconds);
    segment.end_seconds = Some(end_seconds);
    segment.language = language;
    segment
        .attributes
        .insert("provider".to_string(), "candle-whisper".to_string());
    segment
        .attributes
        .insert("timing".to_string(), "global".to_string());
    segment
}

fn decoded_window_to_contract_segments(
    decoded: WhisperDecodedWindow,
    next_index: &mut u64,
    window_start_seconds: f64,
    window_end_seconds: f64,
    language: Option<String>,
) -> Vec<TranscriptSegmentContract> {
    decoded
        .segments
        .into_iter()
        .filter_map(|decoded_segment| {
            let text = decoded_segment.text.trim().to_string();
            if text.is_empty() {
                return None;
            }
            let mut segment = TranscriptSegmentContract::new(*next_index, text);
            *next_index += 1;
            let global_start = (window_start_seconds + decoded_segment.start_seconds)
                .clamp(window_start_seconds, window_end_seconds);
            let global_end = (window_start_seconds + decoded_segment.end_seconds)
                .clamp(window_start_seconds, window_end_seconds);
            segment.start_seconds = Some(global_start);
            segment.end_seconds = Some(global_end);
            segment.language = language.clone();
            segment
                .attributes
                .insert("provider".to_string(), "candle-whisper".to_string());
            segment
                .attributes
                .insert("timing".to_string(), "global".to_string());
            segment.attributes.insert(
                "timingSource".to_string(),
                "whisperTimestampTokens".to_string(),
            );
            Some(segment)
        })
        .collect()
}

#[cfg(test)]
fn project_words_from_timestamp_segment(
    segment: &WhisperDecodedSegment,
) -> Vec<TranscriptWordContract> {
    let text = segment.text.trim();
    let words = text.split_whitespace().collect::<Vec<_>>();
    if words.is_empty() {
        return Vec::new();
    }

    let word_count = words.len();
    let weights = words
        .iter()
        .map(|word| word.chars().count())
        .collect::<Vec<_>>();
    let total_chars = weights.iter().sum::<usize>();
    if total_chars == 0 {
        return Vec::new();
    }

    let start = segment.start_seconds;
    let end = segment.end_seconds;
    let duration = (end - start).max(0.0);
    let mut cursor = start;
    words
        .into_iter()
        .zip(weights)
        .enumerate()
        .map(|(index, (word, weight))| {
            let word_start = cursor.clamp(start, end);
            let projected_end = if index + 1 == word_count {
                end
            } else {
                cursor + duration * weight as f64 / total_chars as f64
            };
            let word_end = projected_end.clamp(word_start, end);
            cursor = word_end;
            TranscriptWordContract {
                text: word.to_string(),
                start_seconds: Some(word_start),
                end_seconds: Some(word_end),
                confidence: None,
                speaker: None,
                attributes: BTreeMap::from([(
                    "timing".to_string(),
                    "whisperTimestampProjection".to_string(),
                )]),
            }
        })
        .collect()
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path, label: &str) -> Result<T> {
    let bytes = std::fs::read(path).map_err(|error| {
        setup_error(format!(
            "failed to read {label} `{}`: {error}",
            path.display()
        ))
    })?;
    serde_json::from_slice(&bytes).map_err(|error| {
        invalid_request(format!(
            "failed to parse {label} `{}`: {error}",
            path.display()
        ))
    })
}

fn candle_device(resolved: &ResolvedNativeDevice) -> Result<Device> {
    match resolved {
        ResolvedNativeDevice::Cpu => Ok(Device::Cpu),
        #[cfg(feature = "cuda")]
        ResolvedNativeDevice::Cuda(index) => Device::new_cuda(*index)
            .map_err(|error| setup_error(format!("failed to create CUDA device {index}: {error}"))),
    }
}

fn device_label(resolved: &ResolvedNativeDevice) -> String {
    match resolved {
        ResolvedNativeDevice::Cpu => "cpu".to_string(),
        #[cfg(feature = "cuda")]
        ResolvedNativeDevice::Cuda(index) => format!("cuda:{index}"),
    }
}

fn device_is_cuda(resolved: &ResolvedNativeDevice) -> bool {
    match resolved {
        ResolvedNativeDevice::Cpu => false,
        #[cfg(feature = "cuda")]
        ResolvedNativeDevice::Cuda(_) => true,
    }
}

fn should_microbatch_encoder(resolved: &ResolvedNativeDevice, window_count: usize) -> bool {
    window_count > 1 && device_is_cuda(resolved)
}

#[derive(Debug, Clone)]
struct ChunkWindow {
    samples: Vec<f32>,
    chunk_start_seconds: f64,
    local_start_seconds: f64,
    local_end_seconds: f64,
    global_start_seconds: f64,
    global_end_seconds: f64,
}

fn chunk_windows(
    samples: &[f32],
    sample_rate: u32,
    chunk: &SpeechActivitySegment,
) -> Result<Vec<ChunkWindow>> {
    let duration = samples.len() as f64 / sample_rate as f64;
    let padded_start_seconds =
        (chunk.start_seconds - ASR_WINDOW_LEADING_CONTEXT_SECONDS).clamp(0.0, duration);
    let padded_end_seconds = (chunk.end_seconds + ASR_WINDOW_TRAILING_CONTEXT_SECONDS)
        .clamp(padded_start_seconds, duration);
    let start = seconds_to_index(padded_start_seconds, sample_rate, samples.len());
    let end = seconds_to_index(padded_end_seconds, sample_rate, samples.len()).max(start + 1);
    let max_window = whisper::N_SAMPLES;
    let mut windows = Vec::new();
    let mut cursor = start;
    while cursor < end {
        let window_end = (cursor + max_window).min(end);
        let local_start_seconds = (cursor - start) as f64 / sample_rate as f64;
        let local_end_seconds = (window_end - start) as f64 / sample_rate as f64;
        windows.push(ChunkWindow {
            samples: samples[cursor..window_end].to_vec(),
            chunk_start_seconds: padded_start_seconds,
            local_start_seconds,
            local_end_seconds,
            global_start_seconds: padded_start_seconds + local_start_seconds,
            global_end_seconds: padded_start_seconds + local_end_seconds,
        });
        cursor = window_end;
    }
    Ok(windows)
}

fn seconds_to_index(seconds: f64, sample_rate: u32, limit: usize) -> usize {
    (seconds * sample_rate as f64)
        .round()
        .clamp(0.0, limit as f64) as usize
}

fn token_id(tokenizer: &Tokenizer, token: &str) -> Option<u32> {
    tokenizer.token_to_id(token)
}

fn mel_filter_bank(n_mels: usize, n_fft: usize, sample_rate: usize) -> Vec<f32> {
    let n_freqs = n_fft / 2 + 1;
    let min_mel = hz_to_mel(0.0);
    let max_mel = hz_to_mel(sample_rate as f32 / 2.0);
    let mel_points = (0..n_mels + 2)
        .map(|index| min_mel + (max_mel - min_mel) * index as f32 / (n_mels + 1) as f32)
        .map(mel_to_hz)
        .collect::<Vec<_>>();
    let fft_freqs = (0..n_freqs)
        .map(|index| sample_rate as f32 * index as f32 / n_fft as f32)
        .collect::<Vec<_>>();
    let mut filters = vec![0.0; n_mels * n_freqs];
    for mel_index in 0..n_mels {
        let lower = mel_points[mel_index];
        let center = mel_points[mel_index + 1];
        let upper = mel_points[mel_index + 2];
        for (freq_index, freq) in fft_freqs.iter().enumerate() {
            let value = if *freq < lower || *freq > upper {
                0.0
            } else if *freq <= center {
                (*freq - lower) / (center - lower).max(f32::EPSILON)
            } else {
                (upper - *freq) / (upper - center).max(f32::EPSILON)
            };
            filters[mel_index * n_freqs + freq_index] = value.max(0.0);
        }
    }
    filters
}

fn hz_to_mel(hz: f32) -> f32 {
    2595.0 * (1.0 + hz / 700.0).log10()
}

fn mel_to_hz(mel: f32) -> f32 {
    700.0 * (10_f32.powf(mel / 2595.0) - 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokenizers::models::wordlevel::WordLevel;

    fn create_fake_whisper_bundle(root: &Path) {
        for file in REQUIRED_WHISPER_FILES {
            std::fs::write(root.join(file), "").unwrap();
        }
    }

    fn minimal_asr_request(model_id: &str) -> AsrRequest {
        AsrRequest {
            audio: crate::LoadedAudio {
                samples: vec![0.0; 16_000],
                sample_rate: 16_000,
                channels: 1,
                source: None,
            },
            chunks: vec![SpeechActivitySegment::new(0.0, 1.0, 0.5).unwrap()],
            task: TranscriptionTask::Transcribe,
            language: Some("en".to_string()),
            model_id: model_id.to_string(),
        }
    }

    fn test_generation() -> GenerationConfig {
        GenerationConfig {
            decoder_start_token_id: Some(1),
            eos_token_id: Some(2),
            forced_decoder_ids: None,
            max_length: Some(8),
            lang_to_id: [("<|en|>".to_string(), 3), ("<|de|>".to_string(), 4)]
                .into_iter()
                .collect(),
            task_to_id: [("transcribe".to_string(), 5), ("translate".to_string(), 6)]
                .into_iter()
                .collect(),
            no_timestamps_token_id: Some(7),
            suppress_tokens: Vec::new(),
            begin_suppress_tokens: Vec::new(),
        }
    }

    fn test_tokenizer() -> Tokenizer {
        let temp = tempfile::tempdir().unwrap();
        let vocab = temp.path().join("vocab.json");
        std::fs::write(
            &vocab,
            serde_json::json!({
                "<unk>": 0,
                whisper::SOT_TOKEN: 1,
                whisper::EOT_TOKEN: 2,
                "<|en|>": 3,
                "<|de|>": 4,
                "<|transcribe|>": 5,
                "<|translate|>": 6,
                whisper::NO_TIMESTAMPS_TOKEN: 7
            })
            .to_string(),
        )
        .unwrap();
        let model = WordLevel::from_file(vocab.to_str().unwrap(), "<unk>".to_string()).unwrap();
        Tokenizer::new(model)
    }

    fn timestamp_test_tokenizer() -> Tokenizer {
        let temp = tempfile::tempdir().unwrap();
        let vocab = temp.path().join("vocab.json");
        std::fs::write(
            &vocab,
            serde_json::json!({
                "<unk>": 0,
                "hello": 10,
                "world": 11,
                "again": 12,
                "<|0.00|>": 100,
                "<|1.00|>": 150,
                "<|2.00|>": 200,
                "<|3.00|>": 250,
                "<|30.00|>": 1600
            })
            .to_string(),
        )
        .unwrap();
        let model = WordLevel::from_file(vocab.to_str().unwrap(), "<unk>".to_string()).unwrap();
        Tokenizer::new(model)
    }

    fn decoded_segment(text: &str, start_seconds: f64, end_seconds: f64) -> WhisperDecodedSegment {
        WhisperDecodedSegment {
            text: text.to_string(),
            start_seconds,
            end_seconds,
            token_ids: Vec::new(),
        }
    }

    fn assert_approx_eq(actual: f64, expected: f64) {
        assert!(
            (actual - expected).abs() < 1e-9,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn autoregressive_row_prefills_prompt_then_steps_with_last_token_position() {
        let mut row = WhisperAutoregressiveRow::new(vec![1, 3, 5, 7]);

        let prefill = row.next_decoder_input();
        assert_eq!(prefill.token_ids, vec![1, 3, 5, 7]);
        assert_eq!(prefill.position_offset, 0);
        assert!(prefill.flush_cache);
        assert_eq!(prefill.kind, WhisperDecoderInputKind::PromptPrefill);

        row.mark_forwarded();
        row.accept(42);
        let step = row.next_decoder_input();

        assert_eq!(step.token_ids, vec![42]);
        assert_eq!(step.position_offset, 4);
        assert!(!step.flush_cache);
        assert_eq!(step.kind, WhisperDecoderInputKind::CachedTokenStep);
        assert_eq!(row.generated_tokens(), &[42]);
    }

    #[test]
    fn generation_stats_report_actual_decoder_cache_reuse() {
        let mut stats = WhisperGenerationStats::default();
        let prefill = WhisperDecoderInput {
            token_ids: vec![1, 3, 5, 7],
            position_offset: 0,
            flush_cache: true,
            kind: WhisperDecoderInputKind::PromptPrefill,
        };
        let step = WhisperDecoderInput {
            token_ids: vec![42],
            position_offset: 4,
            flush_cache: false,
            kind: WhisperDecoderInputKind::CachedTokenStep,
        };
        stats.record_input(&prefill);
        stats.record_input(&step);
        stats.record_generated_token();
        stats.record_active_row_batch_size(3);
        stats.record_active_row_compaction();
        stats.record_completed_row();
        stats.record_decoder_stats(CachedWhisperDecoderStats {
            self_attention_cache_reused: true,
            cross_attention_cache_reused: true,
        });

        let mut diagnostics = WhisperDecodeDiagnostics::default();
        stats.extend(&mut diagnostics);

        assert_eq!(diagnostics.decoder_prompt_prefill_count, 1);
        assert_eq!(diagnostics.decoder_cached_token_step_count, 1);
        assert_eq!(diagnostics.decoder_input_token_count, 5);
        assert_eq!(diagnostics.generated_token_count, 1);
        assert_eq!(diagnostics.decoder_completed_row_count, 1);
        assert_eq!(diagnostics.decoder_max_active_row_batch_size, 3);
        assert_eq!(diagnostics.decoder_effective_active_batch_sizes, vec![3]);
        assert_eq!(diagnostics.decoder_active_row_compaction_count, 1);
        assert!(diagnostics.decoder_self_attention_cache_reused);
        assert!(diagnostics.decoder_cross_attention_cache_reused);
    }

    #[test]
    fn observed_batch_execution_requires_real_multi_row_decoder_call() {
        assert_eq!(
            observed_candle_batch_execution(CandleWhisperDecodeRuntime::ActiveRowTensorBatch, 3),
            crate::CANDLE_WHISPER_ACTIVE_ROW_TENSOR_BATCH_EXECUTION
        );
        assert_eq!(
            observed_candle_batch_execution(CandleWhisperDecodeRuntime::ActiveRowTensorBatch, 1),
            crate::CANDLE_WHISPER_AUTOREGRESSIVE_KV_CACHE_EXECUTION
        );
        assert_eq!(
            observed_candle_batch_execution(CandleWhisperDecodeRuntime::AutoregressiveKvCache, 3),
            crate::CANDLE_WHISPER_AUTOREGRESSIVE_KV_CACHE_EXECUTION
        );
    }

    #[test]
    fn encoder_microbatching_is_cuda_only_for_multi_window_batches() {
        assert!(!should_microbatch_encoder(&ResolvedNativeDevice::Cpu, 1));
        assert!(!should_microbatch_encoder(&ResolvedNativeDevice::Cpu, 2));

        #[cfg(feature = "cuda")]
        {
            assert!(!should_microbatch_encoder(
                &ResolvedNativeDevice::Cuda(0),
                1
            ));
            assert!(should_microbatch_encoder(&ResolvedNativeDevice::Cuda(0), 2));
        }
    }

    #[test]
    fn effective_active_batch_sizes_are_sorted_and_deduplicated() {
        assert_eq!(format_effective_active_batch_sizes(&[]), "none");
        assert_eq!(
            format_effective_active_batch_sizes(&[3, 2, 3, 1, 2]),
            "1,2,3"
        );
    }

    #[test]
    fn cache_reuse_diagnostic_names_observed_cache_modes() {
        assert_eq!(format_cache_reuse(true, true), "self-and-cross-attention");
        assert_eq!(format_cache_reuse(true, false), "self-attention");
        assert_eq!(format_cache_reuse(false, true), "cross-attention");
        assert_eq!(format_cache_reuse(false, false), "none");
    }

    #[test]
    fn active_row_decisions_compact_finished_rows_and_preserve_original_order() {
        let eos = 2;
        let rows = (0..3)
            .map(|original_index| ActiveWhisperDecodeRow {
                original_index,
                row: WhisperAutoregressiveRow::new(vec![1]),
                stats: WhisperGenerationStats::default(),
            })
            .collect::<Vec<_>>();
        let mut completed = vec![None, None, None];

        let (survivors, survivor_indices) = apply_active_row_decisions(
            rows.into_iter().zip([10, eos, 12]).collect(),
            eos,
            &mut completed,
        )
        .unwrap();
        assert_eq!(survivor_indices, vec![0, 2]);
        assert_eq!(
            survivors
                .iter()
                .map(|row| row.original_index)
                .collect::<Vec<_>>(),
            vec![0, 2]
        );
        assert_eq!(completed[1].as_ref().unwrap().0, Vec::<u32>::new());

        let (survivors, survivor_indices) = apply_active_row_decisions(
            survivors.into_iter().zip([eos, 14]).collect(),
            eos,
            &mut completed,
        )
        .unwrap();
        assert_eq!(survivor_indices, vec![1]);
        assert_eq!(survivors[0].original_index, 2);
        assert_eq!(completed[0].as_ref().unwrap().0, vec![10]);

        let (survivors, survivor_indices) = apply_active_row_decisions(
            survivors.into_iter().zip([eos]).collect(),
            eos,
            &mut completed,
        )
        .unwrap();
        assert!(survivors.is_empty());
        assert!(survivor_indices.is_empty());

        let completed_tokens = completed
            .into_iter()
            .map(|row| row.unwrap().0)
            .collect::<Vec<_>>();
        assert_eq!(completed_tokens, vec![vec![10], vec![], vec![12, 14]]);
    }

    #[test]
    fn active_row_decisions_keep_per_row_generation_stats_isolated() {
        let eos = 2;
        let mut rows = (0..2)
            .map(|original_index| ActiveWhisperDecodeRow {
                original_index,
                row: WhisperAutoregressiveRow::new(vec![1]),
                stats: WhisperGenerationStats::default(),
            })
            .collect::<Vec<_>>();
        rows[0].stats.decoder_input_token_count = 3;
        rows[1].stats.decoder_input_token_count = 7;
        let mut completed = vec![None, None];

        let (survivors, _) = apply_active_row_decisions(
            rows.into_iter().zip([42, eos]).collect(),
            eos,
            &mut completed,
        )
        .unwrap();
        assert_eq!(survivors[0].stats.decoder_input_token_count, 3);
        assert_eq!(survivors[0].stats.generated_token_count, 1);
        assert_eq!(
            completed[1].as_ref().unwrap().1.decoder_input_token_count,
            7
        );
        assert_eq!(completed[1].as_ref().unwrap().1.generated_token_count, 0);
    }

    #[test]
    fn fallback_diagnostics_keep_timestamp_state_and_retry_generation_counts() {
        let mut fallback = WhisperDecodeDiagnostics {
            decoder_prompt_prefill_count: 1,
            decoder_cached_token_step_count: 2,
            decoder_input_token_count: 6,
            generated_token_count: 2,
            decoder_self_attention_cache_reused: true,
            decoder_cross_attention_cache_reused: true,
            ..WhisperDecodeDiagnostics::default()
        };
        let timestamp_attempt = WhisperDecodeDiagnostics {
            timestamp_tokens_requested: true,
            timestamp_tokens_present: true,
            decoded_token_ids: vec![100, 10, 150],
            decoder_prompt_prefill_count: 1,
            decoder_cached_token_step_count: 1,
            decoder_input_token_count: 5,
            generated_token_count: 1,
            decoder_completed_row_count: 0,
            decoder_max_active_row_batch_size: 2,
            decoder_effective_active_batch_sizes: vec![2],
            decoder_active_row_compaction_count: 0,
            decoder_self_attention_cache_reused: true,
            decoder_cross_attention_cache_reused: true,
        };

        fallback.add_generation_counts_from(&timestamp_attempt);
        fallback.timestamp_tokens_requested = timestamp_attempt.timestamp_tokens_requested;
        fallback.timestamp_tokens_present = timestamp_attempt.timestamp_tokens_present;
        fallback.decoded_token_ids = timestamp_attempt.decoded_token_ids;

        assert!(fallback.timestamp_tokens_requested);
        assert!(fallback.timestamp_tokens_present);
        assert_eq!(fallback.decoded_token_ids, vec![100, 10, 150]);
        assert_eq!(fallback.decoder_prompt_prefill_count, 2);
        assert_eq!(fallback.decoder_cached_token_step_count, 3);
        assert_eq!(fallback.decoder_input_token_count, 11);
        assert_eq!(fallback.generated_token_count, 3);
        assert_eq!(fallback.decoder_completed_row_count, 0);
        assert_eq!(fallback.decoder_max_active_row_batch_size, 2);
        assert_eq!(fallback.decoder_effective_active_batch_sizes, vec![2]);
        assert_eq!(fallback.decoder_active_row_compaction_count, 0);
        assert!(fallback.decoder_self_attention_cache_reused);
        assert!(fallback.decoder_cross_attention_cache_reused);
    }

    #[test]
    fn initial_prompt_uses_requested_language_and_transcribe_task() {
        let tokens = CandleWhisperSession::initial_prompt_tokens(
            &test_generation(),
            &test_tokenizer(),
            Some("en"),
        )
        .unwrap();
        assert_eq!(tokens, vec![1, 3, 5, 7]);
        assert!(!tokens.contains(&6));
    }

    #[test]
    fn initial_prompt_uses_requested_language_and_translate_task() {
        let tokens = CandleWhisperSession::initial_prompt_tokens_for_task(
            &test_generation(),
            &test_tokenizer(),
            Some("en"),
            TranscriptionTask::Translate,
        )
        .unwrap();
        assert_eq!(tokens, vec![1, 3, 6, 7]);
        assert!(!tokens.contains(&5));
    }

    #[test]
    fn initial_prompt_uses_option_language_when_request_language_absent() {
        let setup = WhisperRunSetup {
            model_id: "openai/whisper-tiny".to_string(),
            task: TranscriptionTask::Transcribe,
            language: Some("de".to_string()),
            bundle: WhisperBundlePaths {
                root: PathBuf::from("bundle"),
                config_json: PathBuf::from("config.json"),
                generation_config_json: PathBuf::from("generation_config.json"),
                tokenizer_json: PathBuf::from("tokenizer.json"),
                preprocessor_config_json: PathBuf::from("preprocessor_config.json"),
                model_safetensors: PathBuf::from("model.safetensors"),
            },
            model_source: "explicit-bundle",
            resolved_device: ResolvedNativeDevice::Cpu,
            requested_compute_type: CandleWhisperComputeType::Automatic,
            resolved_compute_type: CandleWhisperComputeType::Fp32,
            model_weight_dtype: DType::F32,
        };
        let tokens = CandleWhisperSession::initial_prompt_tokens(
            &test_generation(),
            &test_tokenizer(),
            setup.language.as_deref(),
        )
        .unwrap();
        assert_eq!(tokens[1], 4);
    }

    #[test]
    fn request_language_wins_before_prompt_construction() {
        let request = AsrRequest {
            audio: crate::LoadedAudio {
                samples: vec![0.0; 16_000],
                sample_rate: 16_000,
                channels: 1,
                source: None,
            },
            chunks: vec![SpeechActivitySegment::new(0.0, 1.0, 0.5).unwrap()],
            task: TranscriptionTask::Transcribe,
            language: Some("en".to_string()),
            model_id: "openai/whisper-tiny".to_string(),
        };
        let options = CandleWhisperOptions {
            language: Some("de".to_string()),
            model_bundle: Some(PathBuf::from("missing")),
            ..CandleWhisperOptions::default()
        };
        let language = request
            .language
            .clone()
            .or_else(|| options.language.clone())
            .unwrap();
        assert_eq!(language, "en");
    }

    #[test]
    fn whisper_aliases_canonicalize_to_hugging_face_ids() {
        assert_eq!(
            canonical_whisper_model_id("small").unwrap(),
            "openai/whisper-small"
        );
        assert_eq!(
            canonical_whisper_model_id("tiny.en").unwrap(),
            "openai/whisper-tiny.en"
        );
        assert_eq!(
            canonical_whisper_model_id("large").unwrap(),
            "openai/whisper-large-v3"
        );
        assert_eq!(
            canonical_whisper_model_id("openai/whisper-small").unwrap(),
            "openai/whisper-small"
        );
        let error = canonical_whisper_model_id("unknown")
            .unwrap_err()
            .to_string();
        assert!(error.contains("setup_error"));
        assert!(error.contains("unsupported native Candle Whisper model alias"));
    }

    #[test]
    fn whisper_bundle_priority_wins_over_model_dir() {
        let explicit = tempfile::tempdir().unwrap();
        create_fake_whisper_bundle(explicit.path());
        let cache = tempfile::tempdir().unwrap();
        let options = CandleWhisperOptions {
            model_id: "tiny.en".to_string(),
            model_bundle: Some(explicit.path().to_path_buf()),
            model_dir: Some(cache.path().to_path_buf()),
            model_cache_only: true,
            ..CandleWhisperOptions::default()
        };
        let resolved = resolve_whisper_model(&options, "tiny.en").unwrap();
        assert_eq!(resolved.source, "explicit-bundle");
        assert_eq!(resolved.model_id, "openai/whisper-tiny.en");
        assert_eq!(resolved.bundle.root, explicit.path());
    }

    #[cfg(feature = "model-bundles")]
    #[test]
    fn whisper_cache_only_resolves_fake_hf_snapshot() {
        let temp = tempfile::tempdir().unwrap();
        let snapshot = temp
            .path()
            .join("models--openai--whisper-tiny.en/snapshots/abc123");
        std::fs::create_dir_all(&snapshot).unwrap();
        create_fake_whisper_bundle(&snapshot);
        let options = CandleWhisperOptions {
            model_dir: Some(temp.path().to_path_buf()),
            model_cache_only: true,
            ..CandleWhisperOptions::default()
        };
        let resolved = resolve_whisper_model(&options, "tiny.en").unwrap();
        assert_eq!(resolved.source, "hugging-face-cache");
        assert_eq!(resolved.model_id, "openai/whisper-tiny.en");
        assert_eq!(resolved.bundle.root, snapshot);
    }

    #[cfg(feature = "model-bundles")]
    #[test]
    fn whisper_cache_only_missing_model_reports_required_files() {
        let temp = tempfile::tempdir().unwrap();
        let options = CandleWhisperOptions {
            model_dir: Some(temp.path().to_path_buf()),
            model_cache_only: true,
            ..CandleWhisperOptions::default()
        };
        let error = resolve_whisper_model(&options, "tiny.en")
            .unwrap_err()
            .to_string();
        assert!(error.contains("setup_error"));
        assert!(error.contains("openai/whisper-tiny.en"));
        assert!(error.contains("config.json"));
        assert!(error.contains("generation_config.json"));
        assert!(error.contains("tokenizer.json"));
        assert!(error.contains("preprocessor_config.json"));
        assert!(error.contains("model.safetensors"));
        assert!(error.contains("cache-only=true"));
    }

    #[cfg(feature = "model-bundles")]
    #[test]
    fn whisper_model_spec_requests_required_candle_files() {
        let spec = whisper_model_spec("openai/whisper-tiny.en");
        assert_eq!(spec.repo_id_value(), Some("openai/whisper-tiny.en"));
        let rendered = format!("{:?}", spec.files);
        for file in REQUIRED_WHISPER_FILES {
            assert!(rendered.contains(file));
        }
    }

    #[test]
    fn whisper_setup_reports_model_resolution_diagnostics() {
        let explicit = tempfile::tempdir().unwrap();
        create_fake_whisper_bundle(explicit.path());
        let options = CandleWhisperOptions {
            model_bundle: Some(explicit.path().to_path_buf()),
            ..CandleWhisperOptions::default()
        };
        let setup =
            WhisperRunSetup::from_options_and_request(&options, &minimal_asr_request("tiny.en"))
                .unwrap();
        let diagnostics = whisper_setup_diagnostics(&setup);
        assert!(diagnostics
            .iter()
            .any(|item| item == "asrModelSource=explicit-bundle"));
        assert!(diagnostics
            .iter()
            .any(|item| item == "asrModelId=openai/whisper-tiny.en"));
        assert!(diagnostics
            .iter()
            .any(|item| item.starts_with("asrModelResolved=")));
        assert!(diagnostics
            .iter()
            .any(|item| item == "requestedComputeType=automatic"));
        assert!(diagnostics
            .iter()
            .any(|item| item == "resolvedComputeType=fp32"));
        assert!(diagnostics
            .iter()
            .any(|item| item == "modelWeightDtype=f32"));
    }

    #[test]
    fn whisper_setup_resolves_cuda_automatic_to_fp16_weights_without_cuda_runtime() {
        assert_eq!(
            candle_whisper_model_weight_dtype(
                CandleWhisperComputeType::Automatic
                    .resolve_for_device(true)
                    .unwrap()
            ),
            DType::F16
        );
    }

    #[test]
    fn whisper_setup_resolves_explicit_fp32_to_fp32_weights_on_cuda() {
        assert_eq!(
            candle_whisper_model_weight_dtype(
                CandleWhisperComputeType::Fp32
                    .resolve_for_device(true)
                    .unwrap()
            ),
            DType::F32
        );
    }

    #[test]
    fn nullable_forced_decoder_ids_are_skipped() {
        let mut generation = test_generation();
        generation.forced_decoder_ids = Some(vec![(1, None), (2, Some(5)), (3, Some(7))]);
        let tokens =
            CandleWhisperSession::initial_prompt_tokens(&generation, &test_tokenizer(), Some("en"))
                .unwrap();
        assert_eq!(tokens, vec![1, 3, 5, 7]);
    }

    #[test]
    fn timestamp_prompt_omits_no_timestamps_token() {
        let mut generation = test_generation();
        generation.forced_decoder_ids = Some(vec![(3, Some(7))]);
        let tokens = CandleWhisperSession::initial_prompt_tokens_for_mode(
            &generation,
            &test_tokenizer(),
            Some("en"),
            TranscriptionTask::Transcribe,
            WhisperDecodeMode::TimestampTokens,
        )
        .unwrap();
        assert_eq!(tokens, vec![1, 3, 5]);
        assert!(!tokens.contains(&7));
    }

    #[test]
    fn no_timestamps_prompt_keeps_forced_no_timestamps_token() {
        let mut generation = test_generation();
        generation.forced_decoder_ids = Some(vec![(3, Some(7))]);
        let tokens = CandleWhisperSession::initial_prompt_tokens_for_mode(
            &generation,
            &test_tokenizer(),
            Some("en"),
            TranscriptionTask::Transcribe,
            WhisperDecodeMode::WithoutTimestamps,
        )
        .unwrap();
        assert_eq!(tokens, vec![1, 3, 5, 7]);
    }

    #[test]
    fn timestamp_token_detection_uses_whisper_zero_token() {
        let spec = whisper_timestamp_spec(&timestamp_test_tokenizer()).unwrap();
        assert_eq!(spec.begin_token_id, 100);
        assert_eq!(spec.end_token_id, 1601);
        assert_eq!(timestamp_seconds(150, &spec), Some(1.0));
        assert_eq!(timestamp_seconds(99, &spec), None);
    }

    #[test]
    fn missing_timestamp_metadata_returns_invalid_request() {
        let error = whisper_timestamp_spec(&test_tokenizer())
            .unwrap_err()
            .to_string();
        assert!(error.contains("invalid_request"));
        assert!(error.contains("<|0.00|>"));
    }

    #[test]
    fn auto_timing_allows_missing_timestamp_metadata() {
        let spec = timestamp_spec_for_timing_mode(&test_tokenizer(), WhisperDecodeTimingMode::Auto)
            .unwrap();
        assert!(spec.is_none());
    }

    #[test]
    fn required_timing_rejects_missing_timestamp_metadata() {
        let error = timestamp_spec_for_timing_mode(
            &test_tokenizer(),
            WhisperDecodeTimingMode::TimestampTokensRequired,
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("invalid_request"));
        assert!(error.contains("<|0.00|>"));
    }

    #[test]
    fn no_timestamps_timing_does_not_require_timestamp_metadata() {
        let spec = timestamp_spec_for_timing_mode(
            &test_tokenizer(),
            WhisperDecodeTimingMode::NoTimestamps,
        )
        .unwrap();
        assert!(spec.is_none());
    }

    #[test]
    fn timestamp_decode_reads_one_bounded_segment() {
        let tokenizer = timestamp_test_tokenizer();
        let decoded = decode_timestamp_window(&tokenizer, &[100, 10, 11, 150])
            .unwrap()
            .unwrap();
        assert_eq!(decoded.text, "hello world");
        assert_eq!(decoded.segments.len(), 1);
        assert_eq!(decoded.segments[0].text, "hello world");
        assert_eq!(decoded.segments[0].start_seconds, 0.0);
        assert_eq!(decoded.segments[0].end_seconds, 1.0);
        assert_eq!(decoded.segments[0].token_ids, vec![10, 11]);
    }

    #[test]
    fn timestamp_decode_reads_multiple_bounded_segments() {
        let tokenizer = timestamp_test_tokenizer();
        let decoded = decode_timestamp_window(&tokenizer, &[100, 10, 150, 150, 11, 200])
            .unwrap()
            .unwrap();
        assert_eq!(decoded.text, "hello world");
        assert_eq!(decoded.segments.len(), 2);
        assert_eq!(decoded.segments[0].text, "hello");
        assert_eq!(decoded.segments[0].start_seconds, 0.0);
        assert_eq!(decoded.segments[0].end_seconds, 1.0);
        assert_eq!(decoded.segments[1].text, "world");
        assert_eq!(decoded.segments[1].start_seconds, 1.0);
        assert_eq!(decoded.segments[1].end_seconds, 2.0);
    }

    #[test]
    fn timestamp_decode_missing_end_timestamp_falls_back() {
        let tokenizer = timestamp_test_tokenizer();
        let decoded = decode_timestamp_window(&tokenizer, &[100, 10, 11]).unwrap();
        assert!(decoded.is_none());
    }

    #[test]
    fn timestamp_decode_without_timestamp_tokens_falls_back() {
        let tokenizer = timestamp_test_tokenizer();
        let decoded = decode_timestamp_window(&tokenizer, &[10, 11]).unwrap();
        assert!(decoded.is_none());
    }

    #[test]
    fn timestamp_decode_rejects_non_monotonic_timestamps() {
        let tokenizer = timestamp_test_tokenizer();
        let error = decode_timestamp_window(&tokenizer, &[150, 10, 100])
            .unwrap_err()
            .to_string();
        assert!(error.contains("model_output_mismatch"));
        assert!(error.contains("not monotonic"));
    }

    #[test]
    fn timestamp_decode_uses_text_between_timestamp_pairs() {
        let tokenizer = timestamp_test_tokenizer();
        let decoded = decode_timestamp_window(&tokenizer, &[100, 150, 10, 200, 250])
            .unwrap()
            .unwrap();
        assert_eq!(decoded.segments.len(), 1);
        assert_eq!(decoded.segments[0].text, "hello");
        assert_eq!(decoded.segments[0].start_seconds, 1.0);
        assert_eq!(decoded.segments[0].end_seconds, 2.0);
    }

    #[test]
    fn timestamp_logit_rules_select_timestamp_when_timestamp_mass_wins() {
        let spec = WhisperTimestampSpec {
            begin_token_id: 10,
            end_token_id: 13,
            seconds_per_token: 0.02,
        };
        let mut logits = vec![0.0; 13];
        logits[3] = 2.0;
        logits[10] = 1.8;
        logits[11] = 1.8;
        apply_timestamp_logit_rules(&mut logits, &[], &spec, 2).unwrap();

        assert!(logits[3].is_infinite() && logits[3].is_sign_negative());
        let selected = argmax_finite(&logits).unwrap();
        assert!((10..13).contains(&selected));
    }

    #[test]
    fn projected_single_word_receives_full_segment_duration() {
        let words = project_words_from_timestamp_segment(&decoded_segment("hello", 10.0, 12.0));

        assert_eq!(words.len(), 1);
        assert_eq!(words[0].text, "hello");
        assert_eq!(words[0].start_seconds, Some(10.0));
        assert_eq!(words[0].end_seconds, Some(12.0));
        assert_eq!(
            words[0].attributes.get("timing").map(String::as_str),
            Some("whisperTimestampProjection")
        );
    }

    #[test]
    fn projected_words_split_by_character_weight() {
        let words =
            project_words_from_timestamp_segment(&decoded_segment("hello rustaceans", 0.0, 3.0));

        assert_eq!(words.len(), 2);
        assert_eq!(words[0].text, "hello");
        assert_eq!(words[1].text, "rustaceans");
        assert_approx_eq(words[0].start_seconds.unwrap(), 0.0);
        assert_approx_eq(words[0].end_seconds.unwrap(), 1.0);
        assert_approx_eq(words[1].start_seconds.unwrap(), 1.0);
        assert_approx_eq(words[1].end_seconds.unwrap(), 3.0);
    }

    #[test]
    fn projected_words_keep_punctuation_attached() {
        let words =
            project_words_from_timestamp_segment(&decoded_segment("hello, world!", 0.0, 2.0));

        assert_eq!(
            words
                .iter()
                .map(|word| word.text.as_str())
                .collect::<Vec<_>>(),
            vec!["hello,", "world!"]
        );
    }

    #[test]
    fn projected_words_ignore_empty_or_whitespace_text() {
        let words = project_words_from_timestamp_segment(&decoded_segment("   \n\t  ", 0.0, 2.0));

        assert!(words.is_empty());
    }

    #[test]
    fn projected_words_stay_inside_parent_segment() {
        let words =
            project_words_from_timestamp_segment(&decoded_segment("hello rust world", 5.0, 5.1));

        assert!(!words.is_empty());
        for word in words {
            let start = word.start_seconds.unwrap();
            let end = word.end_seconds.unwrap();
            assert!((5.0..=5.1).contains(&start));
            assert!((5.0..=5.1).contains(&end));
            assert!(end >= start);
        }
    }

    #[test]
    fn timestamp_decoded_segments_do_not_project_words() {
        let decoded = WhisperDecodedWindow {
            text: "hello world".to_string(),
            segments: vec![decoded_segment("hello world", 0.5, 1.5)],
        };
        let mut next_index = 0;
        let segments = decoded_window_to_contract_segments(
            decoded,
            &mut next_index,
            10.0,
            12.0,
            Some("en".to_string()),
        );

        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].start_seconds, Some(10.5));
        assert_eq!(segments[0].end_seconds, Some(11.5));
        assert!(segments[0].words.is_empty());
        assert!(!segments[0].attributes.contains_key("wordTiming"));
    }

    #[test]
    fn timestamp_decoded_multiple_segments_keep_word_timing_empty() {
        let decoded = WhisperDecodedWindow {
            text: "hello world rustaceans".to_string(),
            segments: vec![
                decoded_segment("hello world", 0.0, 1.0),
                decoded_segment("rustaceans unite", 1.0, 2.0),
            ],
        };
        let mut next_index = 0;
        let segments =
            decoded_window_to_contract_segments(decoded, &mut next_index, 10.0, 12.0, None);

        assert_eq!(segments.len(), 2);
        for segment in &segments {
            let segment_start = segment.start_seconds.unwrap();
            let segment_end = segment.end_seconds.unwrap();
            assert!(segment_start >= 10.0);
            assert!(segment_end <= 12.0);
            assert!(segment.words.is_empty());
        }
    }

    #[test]
    fn timestamp_decoded_segments_map_to_transcript_contracts() {
        let decoded = WhisperDecodedWindow {
            text: "hello world".to_string(),
            segments: vec![
                WhisperDecodedSegment {
                    text: "hello".to_string(),
                    start_seconds: 0.5,
                    end_seconds: 1.25,
                    token_ids: vec![10],
                },
                WhisperDecodedSegment {
                    text: "world".to_string(),
                    start_seconds: 1.25,
                    end_seconds: 1.75,
                    token_ids: vec![11],
                },
            ],
        };
        let mut next_index = 7;
        let segments = decoded_window_to_contract_segments(
            decoded,
            &mut next_index,
            10.0,
            12.0,
            Some("en".to_string()),
        );
        assert_eq!(next_index, 9);
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].index, 7);
        assert_eq!(segments[0].text, "hello");
        assert_eq!(segments[0].start_seconds, Some(10.5));
        assert_eq!(segments[0].end_seconds, Some(11.25));
        assert_eq!(segments[1].index, 8);
        assert_eq!(segments[1].text, "world");
        assert_eq!(segments[1].start_seconds, Some(11.25));
        assert_eq!(segments[1].end_seconds, Some(11.75));
        assert_eq!(segments[0].language.as_deref(), Some("en"));
        assert_eq!(
            segments[0].attributes.get("timing").map(String::as_str),
            Some("global")
        );
        assert_eq!(
            segments[0]
                .attributes
                .get("timingSource")
                .map(String::as_str),
            Some("whisperTimestampTokens")
        );
        TranscriptionContract::from_segments(None, Some("en".to_string()), segments).unwrap();
    }

    #[test]
    fn projected_timestamp_words_pass_strict_transcript_validation() {
        let decoded = WhisperDecodedWindow {
            text: "hello world".to_string(),
            segments: vec![decoded_segment("hello world", 0.25, 1.25)],
        };
        let mut next_index = 0;
        let segments = decoded_window_to_contract_segments(
            decoded,
            &mut next_index,
            3.0,
            5.0,
            Some("en".to_string()),
        );
        let transcript =
            TranscriptionContract::from_segments(None, Some("en".to_string()), segments).unwrap();

        transcript.validate_strict().unwrap();
    }

    #[test]
    fn window_fallback_segment_uses_global_timing() {
        let segment =
            window_fallback_segment(3, "hello".to_string(), 4.0, 5.5, Some("en".to_string()));
        assert_eq!(segment.start_seconds, Some(4.0));
        assert_eq!(segment.end_seconds, Some(5.5));
        assert_eq!(
            segment.attributes.get("provider").map(String::as_str),
            Some("candle-whisper")
        );
        assert_eq!(
            segment.attributes.get("timing").map(String::as_str),
            Some("global")
        );
    }

    #[test]
    fn fallback_chunk_window_segment_does_not_project_words() {
        let segment = window_fallback_segment(
            3,
            "hello world".to_string(),
            4.0,
            5.5,
            Some("en".to_string()),
        );

        assert!(segment.words.is_empty());
        assert!(!segment.attributes.contains_key("wordTiming"));
    }

    #[test]
    fn chunk_windows_carry_local_and_global_timing() {
        let chunk = SpeechActivitySegment::new(1.0, 2.0, 0.8).unwrap();
        let windows = chunk_windows(&vec![0.0; 48_000], 16_000, &chunk).unwrap();
        assert_eq!(windows.len(), 1);
        assert_eq!(windows[0].chunk_start_seconds, 0.75);
        assert_eq!(windows[0].local_start_seconds, 0.0);
        assert_eq!(windows[0].local_end_seconds, 1.29);
        assert_eq!(windows[0].global_start_seconds, 0.75);
        assert_eq!(windows[0].global_end_seconds, 2.04);
    }

    #[test]
    fn unknown_explicit_language_returns_invalid_request() {
        let error = CandleWhisperSession::initial_prompt_tokens(
            &test_generation(),
            &test_tokenizer(),
            Some("zz"),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("invalid_request"));
        assert!(error.contains("language token"));
    }

    #[test]
    fn missing_eos_returns_invalid_request() {
        let mut generation = test_generation();
        generation.eos_token_id = None;
        let tokenizer = test_tokenizer();
        assert!(CandleWhisperSession::resolve_eos_token_id(&generation, &tokenizer).is_ok());

        let temp = tempfile::tempdir().unwrap();
        let vocab = temp.path().join("vocab.json");
        std::fs::write(
            &vocab,
            serde_json::json!({
                "<unk>": 0,
                whisper::SOT_TOKEN: 1,
                "<|en|>": 3,
                "<|transcribe|>": 5,
                whisper::NO_TIMESTAMPS_TOKEN: 7
            })
            .to_string(),
        )
        .unwrap();
        let tokenizer = Tokenizer::new(
            WordLevel::from_file(vocab.to_str().unwrap(), "<unk>".to_string()).unwrap(),
        );
        let error = CandleWhisperSession::resolve_eos_token_id(&generation, &tokenizer)
            .unwrap_err()
            .to_string();
        assert!(error.contains("invalid_request"));
        assert!(error.contains("eos_token_id"));
    }

    #[test]
    fn whisper_bundle_resolution_accepts_direct_and_files_layouts() {
        for nested in [false, true] {
            let temp = tempfile::tempdir().unwrap();
            let root = temp.path();
            let file_root = if nested {
                std::fs::create_dir(root.join("files")).unwrap();
                root.join("files")
            } else {
                root.to_path_buf()
            };
            for file in [
                "config.json",
                "generation_config.json",
                "tokenizer.json",
                "preprocessor_config.json",
                "model.safetensors",
            ] {
                std::fs::write(file_root.join(file), "").unwrap();
            }
            let paths = resolve_whisper_bundle_paths(root).unwrap();
            assert!(paths.config_json.exists());
            assert!(paths.generation_config_json.exists());
            assert!(paths.tokenizer_json.exists());
            assert!(paths.preprocessor_config_json.exists());
            assert!(paths.model_safetensors.exists());
        }
    }

    #[cfg(feature = "model-bundles")]
    #[test]
    fn whisper_bundle_resolution_accepts_manifest_layout() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        std::fs::create_dir(root.join("snapshots")).unwrap();
        for file in [
            "config.json",
            "generation_config.json",
            "tokenizer.json",
            "preprocessor_config.json",
            "model.safetensors",
        ] {
            std::fs::write(root.join("snapshots").join(file), "").unwrap();
        }
        std::fs::write(
            root.join("manifest.json"),
            serde_json::json!({
                "schema_version": 1,
                "name": "whisper-test",
                "repo_id": "openai/whisper-tiny",
                "revision": "main",
                "task": "speech_recognition",
                "files": {
                    "config.json": {"remote_path": "config.json", "local_path": "snapshots/config.json", "size_bytes": 0},
                    "generation_config.json": {"remote_path": "generation_config.json", "local_path": "snapshots/generation_config.json", "size_bytes": 0},
                    "tokenizer.json": {"remote_path": "tokenizer.json", "local_path": "snapshots/tokenizer.json", "size_bytes": 0},
                    "preprocessor_config.json": {"remote_path": "preprocessor_config.json", "local_path": "snapshots/preprocessor_config.json", "size_bytes": 0},
                    "model.safetensors": {"remote_path": "model.safetensors", "local_path": "snapshots/model.safetensors", "size_bytes": 0}
                }
            })
            .to_string(),
        )
        .unwrap();
        let paths = resolve_whisper_bundle_paths(root).unwrap();
        assert_eq!(
            paths.model_safetensors,
            root.join("snapshots/model.safetensors")
        );
    }
}
