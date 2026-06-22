//! Native transcript import and optional segment translation support.

#[cfg(not(feature = "translation"))]
use std::time::Instant;
#[cfg(feature = "translation")]
use std::{fs, path::Path, path::PathBuf, time::Instant};

use audio_analysis_transcription::{TranscriptionPipelineRequest, TranscriptionPipelineResponse};

use crate::config::{
    DevicePreference, NativeWhisperxConfig, NativeWhisperxError, TranslationConfig,
};
use crate::workflow::run_with_phase_observer;

#[cfg(feature = "translation")]
use candle_core::IndexOp;

pub fn import_whisperx_json(
    bytes: &[u8],
) -> Result<text_transcripts::TranscriptionContract, NativeWhisperxError> {
    text_transcripts::parse_whisperx_json(bytes)
        .map_err(|error| NativeWhisperxError::Import(error.to_string()))
}

#[cfg(feature = "translation")]
pub(crate) fn run_native_with_translation(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    let mut response = run_with_phase_observer(request, config)?;
    let translation_started = Instant::now();
    let mut translator = MarianSegmentTranslator::from_config(config)?;
    translate_response_segments(&mut response, config, &mut translator)?;
    response.diagnostics.push(format!(
        "phaseTranslationSeconds={:.6}",
        translation_started.elapsed().as_secs_f64()
    ));
    Ok(response)
}

#[cfg(not(feature = "translation"))]
pub(crate) fn run_native_with_translation(
    request: TranscriptionPipelineRequest,
    config: &NativeWhisperxConfig,
) -> Result<TranscriptionPipelineResponse, NativeWhisperxError> {
    let _ = (request, config);
    Err(NativeWhisperxError::InvalidConfig(
        "native post-ASR translation requires the `translation` feature".to_string(),
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TranslationRunOptions {
    pub(crate) source_language: Option<String>,
    pub(crate) target_language: String,
    pub(crate) max_new_tokens: usize,
}

pub(crate) trait SegmentTranslator {
    fn model_id(&self) -> &str;
    fn model_source(&self) -> &'static str;
    fn translate_segment(
        &mut self,
        text: &str,
        options: &TranslationRunOptions,
    ) -> Result<String, NativeWhisperxError>;
}

pub(crate) fn translate_response_segments(
    response: &mut TranscriptionPipelineResponse,
    config: &NativeWhisperxConfig,
    translator: &mut dyn SegmentTranslator,
) -> Result<(), NativeWhisperxError> {
    let options = translation_run_options(config)?;
    for segment in &mut response.transcript.segments {
        let source_text = segment.text.trim();
        if source_text.is_empty() {
            continue;
        }
        segment.text = translator.translate_segment(source_text, &options)?;
        segment.language = Some(options.target_language.clone());
        segment.words.clear();
        segment.chars.clear();
    }
    response.transcript.language = Some(options.target_language.clone());
    response.transcript.text = Some(response.transcript.joined_text());
    response
        .diagnostics
        .push(format!("translationModelId={}", translator.model_id()));
    response.diagnostics.push(format!(
        "translationModelSource={}",
        translator.model_source()
    ));
    if let Some(source_language) = &options.source_language {
        response
            .diagnostics
            .push(format!("translationSourceLanguage={source_language}"));
    }
    response.diagnostics.push(format!(
        "translationTargetLanguage={}",
        options.target_language
    ));
    response.diagnostics.push(format!(
        "translationMaxNewTokens={}",
        options.max_new_tokens
    ));
    Ok(())
}

fn translation_run_options(
    config: &NativeWhisperxConfig,
) -> Result<TranslationRunOptions, NativeWhisperxError> {
    let model_pair = config
        .translation
        .model_id
        .as_deref()
        .and_then(opus_mt_language_pair);
    let source_language = config
        .translation
        .source_language
        .clone()
        .or_else(|| config.asr.language.clone())
        .or_else(|| model_pair.as_ref().map(|(source, _)| (*source).to_string()));
    let target_language = config
        .translation
        .target_language
        .clone()
        .or_else(|| model_pair.as_ref().map(|(_, target)| (*target).to_string()))
        .unwrap_or_else(|| "en".to_string());

    if let (Some((expected_source, expected_target)), Some(source_language)) =
        (model_pair, source_language.as_deref())
    {
        if source_language != expected_source || target_language != expected_target {
            return Err(NativeWhisperxError::InvalidConfig(format!(
                "translation model expects {expected_source}->{expected_target}, got {source_language}->{target_language}"
            )));
        }
    }

    Ok(TranslationRunOptions {
        source_language,
        target_language,
        max_new_tokens: config.translation.max_new_tokens,
    })
}

fn opus_mt_language_pair(model_id: &str) -> Option<(&'static str, &'static str)> {
    let suffix = model_id.rsplit('/').next().unwrap_or(model_id);
    match suffix {
        "opus-mt-de-en" => Some(("de", "en")),
        _ => None,
    }
}

#[cfg(feature = "translation")]
pub(crate) const REQUIRED_TRANSLATION_FILES: &[&str] = &[
    "config.json",
    "generation_config.json",
    "source.spm",
    "target.spm",
    "vocab.json",
];

#[cfg(feature = "translation")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TranslationWeightFormat {
    Safetensors,
    Pytorch,
}

#[cfg(feature = "translation")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TranslationBundlePaths {
    pub(crate) root: PathBuf,
    pub(crate) config_json: PathBuf,
    pub(crate) generation_config_json: PathBuf,
    pub(crate) source_spm: PathBuf,
    pub(crate) target_spm: PathBuf,
    pub(crate) vocab_json: PathBuf,
    pub(crate) model_weights: PathBuf,
    pub(crate) weight_format: TranslationWeightFormat,
    pub(crate) source: &'static str,
}

#[cfg(feature = "translation")]
struct MarianSegmentTranslator {
    model_id: String,
    model_source: &'static str,
    source_tokenizer: sentencepiece_rs::SentencePieceProcessor,
    target_tokenizer: sentencepiece_rs::SentencePieceProcessor,
    config: candle_transformers::models::marian::Config,
    model: candle_transformers::models::marian::MTModel,
    device: candle_core::Device,
}

#[cfg(feature = "translation")]
impl MarianSegmentTranslator {
    fn from_config(config: &NativeWhisperxConfig) -> Result<Self, NativeWhisperxError> {
        let model_id = config
            .translation
            .model_id
            .clone()
            .unwrap_or_else(|| "Helsinki-NLP/opus-mt-de-en".to_string());
        let bundle = resolve_translation_bundle(&config.translation)?;
        let device = translation_device(config.asr.device)?;
        let marian_config: candle_transformers::models::marian::Config =
            read_json_file(&bundle.config_json)?;
        let _generation_config: serde_json::Value = read_json_file(&bundle.generation_config_json)?;
        let _vocab: serde_json::Value = read_json_file(&bundle.vocab_json)?;
        let source_tokenizer = sentencepiece_rs::SentencePieceProcessor::open(&bundle.source_spm)
            .map_err(|error| {
            NativeWhisperxError::Transcription(format!(
                "failed to load source SentencePiece model `{}`: {error}",
                bundle.source_spm.display()
            ))
        })?;
        let target_tokenizer = sentencepiece_rs::SentencePieceProcessor::open(&bundle.target_spm)
            .map_err(|error| {
            NativeWhisperxError::Transcription(format!(
                "failed to load target SentencePiece model `{}`: {error}",
                bundle.target_spm.display()
            ))
        })?;
        let vb = match bundle.weight_format {
            TranslationWeightFormat::Safetensors => unsafe {
                candle_nn::VarBuilder::from_mmaped_safetensors(
                    &[bundle.model_weights.as_path()],
                    candle_core::DType::F32,
                    &device,
                )
            },
            TranslationWeightFormat::Pytorch => candle_nn::VarBuilder::from_pth(
                &bundle.model_weights,
                candle_core::DType::F32,
                &device,
            ),
        }
        .map_err(|error| {
            NativeWhisperxError::Transcription(format!(
                "failed to load Marian weights `{}`: {error}",
                bundle.model_weights.display()
            ))
        })?;
        let model = candle_transformers::models::marian::MTModel::new(&marian_config, vb).map_err(
            |error| {
                NativeWhisperxError::Transcription(format!(
                    "failed to construct Marian translation model from `{}`: {error}",
                    bundle.root.display()
                ))
            },
        )?;
        Ok(Self {
            model_id,
            model_source: bundle.source,
            source_tokenizer,
            target_tokenizer,
            config: marian_config,
            model,
            device,
        })
    }
}

#[cfg(feature = "translation")]
impl SegmentTranslator for MarianSegmentTranslator {
    fn model_id(&self) -> &str {
        &self.model_id
    }

    fn model_source(&self) -> &'static str {
        self.model_source
    }

    fn translate_segment(
        &mut self,
        text: &str,
        options: &TranslationRunOptions,
    ) -> Result<String, NativeWhisperxError> {
        let mut input_ids: Vec<u32> = self
            .source_tokenizer
            .encode_to_ids(text)
            .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))?
            .into_iter()
            .map(|id| id as u32)
            .collect();
        if input_ids.last().copied() != Some(self.config.eos_token_id) {
            input_ids.push(self.config.eos_token_id);
        }
        input_ids.truncate(self.config.max_position_embeddings);
        let input = candle_core::Tensor::new(input_ids.as_slice(), &self.device)
            .and_then(|tensor| tensor.unsqueeze(0))
            .map_err(candle_translation_error)?;
        self.model.reset_kv_cache();
        let encoder_xs = self
            .model
            .encoder()
            .forward(&input, 0)
            .map_err(candle_translation_error)?;
        let mut generated = vec![self.config.decoder_start_token_id];
        for _ in 0..options.max_new_tokens {
            self.model.reset_kv_cache();
            let decoder_input = candle_core::Tensor::new(generated.as_slice(), &self.device)
                .and_then(|tensor| tensor.unsqueeze(0))
                .map_err(candle_translation_error)?;
            let logits = self
                .model
                .decode(&decoder_input, &encoder_xs, 0)
                .map_err(candle_translation_error)?;
            let next = logits
                .i((0, generated.len() - 1))
                .and_then(|logits| logits.argmax(candle_core::D::Minus1))
                .and_then(|token| token.to_scalar::<u32>())
                .map_err(candle_translation_error)?;
            if next == self.config.eos_token_id || next == self.config.forced_eos_token_id {
                break;
            }
            generated.push(next);
        }
        let decoded_ids: Vec<usize> = generated
            .into_iter()
            .skip(1)
            .map(|id| id as usize)
            .collect();
        self.target_tokenizer
            .decode_ids(&decoded_ids)
            .map(|text| text.trim().to_string())
            .map_err(|error| NativeWhisperxError::Transcription(error.to_string()))
    }
}

#[cfg(feature = "translation")]
fn candle_translation_error(error: candle_core::Error) -> NativeWhisperxError {
    NativeWhisperxError::Transcription(format!("Marian translation failed: {error}"))
}

#[cfg(not(feature = "translation"))]
struct MarianSegmentTranslator;

#[cfg(not(feature = "translation"))]
impl MarianSegmentTranslator {
    fn from_config(_config: &NativeWhisperxConfig) -> Result<Self, NativeWhisperxError> {
        Err(NativeWhisperxError::InvalidConfig(
            "native post-ASR translation requires the `translation` feature".to_string(),
        ))
    }
}

#[cfg(feature = "translation")]
fn translation_device(
    preference: DevicePreference,
) -> Result<candle_core::Device, NativeWhisperxError> {
    match preference {
        DevicePreference::Auto | DevicePreference::Cpu => Ok(candle_core::Device::Cpu),
        DevicePreference::Cuda => {
            #[cfg(feature = "cuda")]
            {
                candle_core::Device::new_cuda(0).map_err(|error| {
                    NativeWhisperxError::Transcription(format!(
                        "failed to initialize Candle CUDA device 0 for translation: {error}"
                    ))
                })
            }
            #[cfg(not(feature = "cuda"))]
            {
                Err(NativeWhisperxError::InvalidConfig(
                    "translation requested CUDA but this binary was built without the `cuda` feature".to_string(),
                ))
            }
        }
    }
}

#[cfg(feature = "translation")]
pub(crate) fn resolve_translation_bundle(
    translation: &TranslationConfig,
) -> Result<TranslationBundlePaths, NativeWhisperxError> {
    if let Some(bundle) = &translation.model_bundle {
        return resolve_translation_bundle_paths(bundle, "explicit-bundle");
    }

    let model_id = translation
        .model_id
        .as_deref()
        .unwrap_or("Helsinki-NLP/opus-mt-de-en");
    if translation.model_cache_only {
        return resolve_cached_translation_model(model_id, translation.model_dir.as_deref())
            .ok_or_else(|| missing_translation_model_error(model_id, translation));
    }

    let mut downloader = model_runtime::HuggingFaceDownloader::new().progress(false);
    if let Some(model_dir) = &translation.model_dir {
        downloader = downloader.cache_dir(model_dir.clone());
    }
    let downloaded = downloader
        .download(&translation_model_spec(model_id))
        .map_err(|error| {
            NativeWhisperxError::Transcription(format!(
                "failed to download translation model `{model_id}`: {error}"
            ))
        })?;
    let model_dir = downloaded.model_dir().ok_or_else(|| {
        NativeWhisperxError::Transcription(format!(
            "translation model `{model_id}` resolved without a local model directory"
        ))
    })?;
    resolve_translation_bundle_paths(model_dir, "hugging-face-cache")
}

#[cfg(feature = "translation")]
fn resolve_cached_translation_model(
    model_id: &str,
    model_dir: Option<&Path>,
) -> Option<TranslationBundlePaths> {
    for root in hugging_face_cache_roots(model_dir) {
        for candidate in hf_cache_candidates(&root, model_id) {
            if let Ok(paths) = resolve_translation_bundle_paths(&candidate, "hugging-face-cache") {
                return Some(paths);
            }
        }
    }
    None
}

#[cfg(feature = "translation")]
fn hugging_face_cache_roots(model_dir: Option<&Path>) -> Vec<PathBuf> {
    if let Some(model_dir) = model_dir {
        return vec![model_dir.to_path_buf()];
    }
    if let Some(home) = std::env::var_os("HF_HOME") {
        return vec![PathBuf::from(home).join("hub")];
    }
    std::env::var_os("HOME")
        .map(|home| vec![PathBuf::from(home).join(".cache/huggingface/hub")])
        .unwrap_or_default()
}

#[cfg(feature = "translation")]
fn hf_cache_candidates(root: &Path, model_id: &str) -> Vec<PathBuf> {
    let mut candidates = vec![root.to_path_buf(), root.join(model_id.replace('/', "--"))];
    let hf_repo_dir = root.join(format!("models--{}", model_id.replace('/', "--")));
    if let Ok(snapshot) = fs::read_to_string(hf_repo_dir.join("refs/main")) {
        candidates.push(hf_repo_dir.join("snapshots").join(snapshot.trim()));
    }
    if let Ok(entries) = fs::read_dir(hf_repo_dir.join("snapshots")) {
        for entry in entries.flatten() {
            candidates.push(entry.path());
        }
    }
    candidates
}

#[cfg(feature = "translation")]
fn resolve_translation_bundle_paths(
    root: &Path,
    source: &'static str,
) -> Result<TranslationBundlePaths, NativeWhisperxError> {
    let (model_weights, weight_format) = resolve_translation_weights(root)?;
    Ok(TranslationBundlePaths {
        root: root.to_path_buf(),
        config_json: resolve_translation_file(root, "config.json")?,
        generation_config_json: resolve_translation_file(root, "generation_config.json")?,
        source_spm: resolve_translation_file(root, "source.spm")?,
        target_spm: resolve_translation_file(root, "target.spm")?,
        vocab_json: resolve_translation_file(root, "vocab.json")?,
        model_weights,
        weight_format,
        source,
    })
}

#[cfg(feature = "translation")]
fn resolve_translation_file(root: &Path, file: &str) -> Result<PathBuf, NativeWhisperxError> {
    if let Ok(bundle) = model_runtime::ModelBundle::load(root) {
        if let Some(path) = bundle.file_path(file).filter(|path| path.exists()) {
            return Ok(path);
        }
    }
    let direct = root.join(file);
    if direct.exists() {
        return Ok(direct);
    }
    Err(NativeWhisperxError::Transcription(format!(
        "translation bundle `{}` is missing required file `{file}`",
        root.display()
    )))
}

#[cfg(feature = "translation")]
fn resolve_translation_weights(
    root: &Path,
) -> Result<(PathBuf, TranslationWeightFormat), NativeWhisperxError> {
    if let Ok(path) = resolve_translation_file(root, "model.safetensors") {
        return Ok((path, TranslationWeightFormat::Safetensors));
    }
    if let Some(path) = first_file_with_extension(root, "safetensors") {
        return Ok((path, TranslationWeightFormat::Safetensors));
    }
    if let Ok(path) = resolve_translation_file(root, "pytorch_model.bin") {
        return Ok((path, TranslationWeightFormat::Pytorch));
    }
    Err(NativeWhisperxError::Transcription(format!(
        "translation bundle `{}` is missing supported Marian weights: model.safetensors, *.safetensors, or pytorch_model.bin",
        root.display()
    )))
}

#[cfg(feature = "translation")]
fn first_file_with_extension(root: &Path, extension: &str) -> Option<PathBuf> {
    fs::read_dir(root)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .find(|path| {
            path.extension()
                .and_then(|value| value.to_str())
                .is_some_and(|value| value == extension)
        })
}

#[cfg(feature = "translation")]
fn translation_model_spec(model_id: &str) -> model_runtime::HuggingFaceModelSpec {
    let mut spec = model_runtime::HuggingFaceModelSpec::new(
        model_id.to_string(),
        model_runtime::ModelTask::Custom("translation".to_string()),
    );
    spec.files = REQUIRED_TRANSLATION_FILES
        .iter()
        .copied()
        .map(model_runtime::ModelFileRequest::required)
        .chain(
            ["model.safetensors", "pytorch_model.bin"]
                .into_iter()
                .map(model_runtime::ModelFileRequest::optional),
        )
        .collect();
    spec
}

#[cfg(feature = "translation")]
fn missing_translation_model_error(
    model_id: &str,
    translation: &TranslationConfig,
) -> NativeWhisperxError {
    NativeWhisperxError::Transcription(format!(
        "failed to resolve translation model `{model_id}`; required files: {}; --model-dir={}; cache-only={}",
        REQUIRED_TRANSLATION_FILES.join(", "),
        translation
            .model_dir
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<default huggingface cache>".to_string()),
        translation.model_cache_only
    ))
}

#[cfg(feature = "translation")]
fn read_json_file<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, NativeWhisperxError> {
    let bytes = fs::read(path)?;
    serde_json::from_slice(&bytes).map_err(NativeWhisperxError::Json)
}
