use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use video_analysis_core::Result;

use crate::ctc_alignment::{
    backtrack_ctc, build_ctc_trellis, tokens_to_segment_words, CtcAlignedToken, CtcVocabulary,
};
use crate::native_device::ResolvedNativeDevice;
use crate::{
    invalid_request, model_output_mismatch, unsupported_runtime, AlignedChar, AlignedWord,
    AlignmentInterpolationMethod, AlignmentRequest,
};

#[derive(Debug, Clone)]
pub(crate) struct Wav2Vec2BundlePaths {
    pub config_json: PathBuf,
    pub tokenizer_vocab: PathBuf,
    pub preprocessor_config_json: PathBuf,
    pub model_safetensors: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Wav2Vec2LayoutReport {
    pub architecture: String,
    pub do_stable_layer_norm: bool,
    pub positional_conv_layout: String,
    pub feature_extractor_norm: String,
    pub encoder_layer_count: usize,
    pub missing_required_keys: Vec<String>,
    pub unsupported_reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct Wav2Vec2ConfigSummary {
    pub model_type: Option<String>,
    pub architectures: Vec<String>,
    pub vocab_size: Option<usize>,
    pub word_delimiter_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct Wav2Vec2Vocabulary {
    pub ctc: CtcVocabulary,
    pub word_delimiter_token: Option<String>,
}

pub(crate) enum Wav2Vec2ModelLoadEvent {
    Start,
    End { duration_seconds: f64 },
}

#[derive(Debug, Deserialize)]
struct RawWav2Vec2Config {
    #[serde(default)]
    model_type: Option<String>,
    #[serde(default)]
    architectures: Vec<String>,
    #[serde(default)]
    vocab_size: Option<usize>,
    #[serde(default)]
    word_delimiter_token: Option<String>,
    #[serde(default)]
    hidden_size: Option<usize>,
    #[serde(default)]
    num_hidden_layers: Option<usize>,
    #[serde(default)]
    num_attention_heads: Option<usize>,
    #[serde(default)]
    intermediate_size: Option<usize>,
    #[serde(default)]
    hidden_act: Option<String>,
    #[serde(default)]
    layer_norm_eps: Option<f64>,
    #[serde(default)]
    feat_extract_norm: Option<String>,
    #[serde(default)]
    feat_extract_activation: Option<String>,
    #[serde(default)]
    conv_dim: Vec<usize>,
    #[serde(default)]
    conv_stride: Vec<usize>,
    #[serde(default)]
    conv_kernel: Vec<usize>,
    #[serde(default)]
    conv_bias: Option<bool>,
    #[serde(default)]
    num_conv_pos_embeddings: Option<usize>,
    #[serde(default)]
    num_conv_pos_embedding_groups: Option<usize>,
    #[serde(default)]
    do_stable_layer_norm: Option<bool>,
    #[serde(default)]
    final_dropout: Option<f64>,
    #[serde(default)]
    hidden_dropout: Option<f64>,
    #[serde(default)]
    activation_dropout: Option<f64>,
    #[serde(default)]
    attention_dropout: Option<f64>,
    #[serde(default)]
    layerdrop: Option<f64>,
    #[serde(default)]
    pad_token_id: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct Wav2Vec2CtcConfig {
    pub model_type: Option<String>,
    pub architectures: Vec<String>,
    pub vocab_size: usize,
    pub word_delimiter_token: Option<String>,
    pub hidden_size: usize,
    pub num_hidden_layers: usize,
    pub num_attention_heads: usize,
    pub intermediate_size: usize,
    pub hidden_act: String,
    pub layer_norm_eps: f64,
    pub feat_extract_norm: Option<String>,
    pub feat_extract_activation: String,
    pub conv_dim: Vec<usize>,
    pub conv_stride: Vec<usize>,
    pub conv_kernel: Vec<usize>,
    pub conv_bias: bool,
    pub num_conv_pos_embeddings: usize,
    pub num_conv_pos_embedding_groups: usize,
    pub do_stable_layer_norm: bool,
    pub final_dropout: Option<f64>,
    pub hidden_dropout: Option<f64>,
    pub activation_dropout: Option<f64>,
    pub attention_dropout: Option<f64>,
    pub layerdrop: Option<f64>,
    pub pad_token_id: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct RawWav2Vec2PreprocessorConfig {
    #[serde(default)]
    sampling_rate: Option<u32>,
    #[serde(default)]
    do_normalize: Option<bool>,
    #[serde(default)]
    return_attention_mask: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct Wav2Vec2PreprocessorConfig {
    pub sampling_rate: Option<u32>,
    pub do_normalize: Option<bool>,
    pub return_attention_mask: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct Wav2Vec2CtcEmission {
    pub segment_index: u64,
    pub emissions: Vec<Vec<f32>>,
    pub token_ids: Vec<usize>,
    pub chars: Vec<AlignedInputChar>,
    pub blank_id: usize,
    pub transcript_words: Vec<String>,
    pub segment_start_seconds: f64,
    pub segment_end_seconds: f64,
    pub frame_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct AlignedInputChar {
    pub char_index: usize,
    pub character: String,
    pub is_word_delimiter: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NativeAlignmentResult {
    pub words: Vec<AlignedWord>,
    pub chars: Vec<AlignedChar>,
}

#[allow(dead_code)]
pub(crate) fn emit_wav2vec2_ctc(
    bundle: &Path,
    request: &AlignmentRequest,
) -> Result<Vec<Vec<f32>>> {
    let cpu = ResolvedNativeDevice::Cpu;
    let emissions = emit_wav2vec2_ctc_segments(bundle, request, &cpu)?;
    Ok(emissions
        .into_iter()
        .flat_map(|segment| segment.emissions)
        .collect())
}

#[allow(dead_code)]
pub(crate) fn align_wav2vec2_ctc(
    bundle: &Path,
    request: &AlignmentRequest,
    device: &ResolvedNativeDevice,
    interpolate_method: AlignmentInterpolationMethod,
    return_char_alignments: bool,
) -> Result<NativeAlignmentResult> {
    align_wav2vec2_ctc_with_load_observer(
        bundle,
        request,
        device,
        interpolate_method,
        return_char_alignments,
        |_| {},
    )
}

pub(crate) fn align_wav2vec2_ctc_with_load_observer(
    bundle: &Path,
    request: &AlignmentRequest,
    device: &ResolvedNativeDevice,
    interpolate_method: AlignmentInterpolationMethod,
    return_char_alignments: bool,
    observe_load: impl FnMut(Wav2Vec2ModelLoadEvent),
) -> Result<NativeAlignmentResult> {
    let emission_segments =
        emit_wav2vec2_ctc_segments_with_load_observer(bundle, request, device, observe_load)?;
    let mut aligned_words = Vec::new();
    let mut aligned_chars = Vec::new();
    for segment in emission_segments {
        let trellis = build_ctc_trellis(&segment.emissions, &segment.token_ids, segment.blank_id)?;
        let path = backtrack_ctc(
            &trellis,
            &segment.emissions,
            &segment.token_ids,
            segment.blank_id,
        )?;
        aligned_words.extend(tokens_to_segment_words(
            segment.segment_index,
            &path,
            &segment.transcript_words,
            segment.segment_start_seconds,
            segment.segment_end_seconds,
            segment.frame_seconds,
        )?);
        if return_char_alignments {
            let segment_chars = tokens_to_segment_chars(
                segment.segment_index,
                &path,
                &segment.chars,
                segment.segment_start_seconds,
                segment.segment_end_seconds,
                segment.frame_seconds,
                interpolate_method,
            )?;
            aligned_chars.extend(whisperx_compatible_segment_chars(
                segment.segment_index,
                segment_chars,
            ));
        }
    }
    Ok(NativeAlignmentResult {
        words: aligned_words,
        chars: aligned_chars,
    })
}

pub(crate) fn emit_wav2vec2_ctc_segments(
    bundle: &Path,
    request: &AlignmentRequest,
    device: &ResolvedNativeDevice,
) -> Result<Vec<Wav2Vec2CtcEmission>> {
    emit_wav2vec2_ctc_segments_with_load_observer(bundle, request, device, |_| {})
}

pub(crate) fn emit_wav2vec2_ctc_segments_with_load_observer(
    bundle: &Path,
    request: &AlignmentRequest,
    device: &ResolvedNativeDevice,
    mut observe_load: impl FnMut(Wav2Vec2ModelLoadEvent),
) -> Result<Vec<Wav2Vec2CtcEmission>> {
    let paths = resolve_wav2vec2_bundle_paths(bundle)?;
    let config = parse_wav2vec2_ctc_config(&paths.config_json)?;
    let preprocessor = parse_wav2vec2_preprocessor_config(&paths.preprocessor_config_json)?;
    let vocab = parse_ctc_vocabulary(&paths.tokenizer_vocab, config.pad_token_id)?;
    if vocab.tokens.len() > config.vocab_size {
        return Err(model_output_mismatch(format!(
            "wav2vec2 tokenizer vocab has {} entries but config vocab_size is {}",
            vocab.tokens.len(),
            config.vocab_size
        )));
    }
    observe_load(Wav2Vec2ModelLoadEvent::Start);
    let load_started = std::time::Instant::now();
    let model = crate::native_wav2vec2_model::Wav2Vec2ForCtc::load(
        &paths.model_safetensors,
        config,
        preprocessor,
        device.candle_device()?,
    )?;
    observe_load(Wav2Vec2ModelLoadEvent::End {
        duration_seconds: load_started.elapsed().as_secs_f64(),
    });
    let mut segments = Vec::new();
    let audio_duration = request.audio.duration_seconds();
    for segment in &request.transcript.segments {
        let segment_start = segment.start_seconds.unwrap_or(0.0);
        let segment_end = segment.end_seconds.unwrap_or(audio_duration);
        if !segment_start.is_finite()
            || !segment_end.is_finite()
            || segment_end <= segment_start
            || segment_end > audio_duration + 1e-6
        {
            return Err(invalid_request(
                "transcript segment timing is outside audio range",
            ));
        }
        let transcript_words = segment_words(segment);
        if transcript_words.is_empty() {
            continue;
        }
        let transcript_text = transcript_words.join(" ");
        let aligned_chars = normalize_text_to_aligned_chars(&transcript_text, &vocab)?;
        let token_ids = aligned_chars
            .iter()
            .map(|character| character.token_id)
            .collect::<Vec<_>>();
        let samples = slice_segment_samples(
            &request.audio.samples,
            request.audio.sample_rate,
            request.audio.channels,
            segment_start,
            segment_end,
        )?;
        let emissions = model.emit_log_probs(&samples)?;
        let frame_seconds = (segment_end - segment_start) / emissions.len() as f64;
        if !frame_seconds.is_finite() || frame_seconds <= 0.0 {
            return Err(model_output_mismatch(
                "wav2vec2 CTC frame timing is invalid",
            ));
        }
        segments.push(Wav2Vec2CtcEmission {
            segment_index: segment.index,
            emissions,
            token_ids,
            chars: aligned_chars
                .into_iter()
                .map(|character| AlignedInputChar {
                    char_index: character.char_index,
                    character: character.character,
                    is_word_delimiter: character.is_word_delimiter,
                })
                .collect(),
            blank_id: vocab.blank_id,
            transcript_words,
            segment_start_seconds: segment_start,
            segment_end_seconds: segment_end,
            frame_seconds,
        });
    }
    Ok(segments)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TokenizedAlignmentChar {
    char_index: usize,
    character: String,
    token_id: usize,
    is_word_delimiter: bool,
}

pub(crate) fn resolve_wav2vec2_bundle_paths(bundle: &Path) -> Result<Wav2Vec2BundlePaths> {
    let tokenizer_vocab = if let Some(tokenizer_json) =
        crate::native_bundles::resolve_optional_bundle_file(bundle, "tokenizer.json")?
    {
        tokenizer_json
    } else {
        crate::native_bundles::resolve_required_bundle_file(bundle, "vocab.json")?
    };
    Ok(Wav2Vec2BundlePaths {
        config_json: crate::native_bundles::resolve_required_bundle_file(bundle, "config.json")?,
        tokenizer_vocab,
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

pub(crate) fn inspect_wav2vec2_bundle_layout(bundle: &Path) -> Result<Wav2Vec2LayoutReport> {
    let paths = resolve_wav2vec2_bundle_paths(bundle)?;
    let raw = parse_raw_wav2vec2_config(&paths.config_json)?;
    let mut unsupported_reasons = Vec::new();
    if let Err(error) = validate_model_type_and_architecture(&raw) {
        unsupported_reasons.push(error.to_string());
    }
    if raw.do_stable_layer_norm == Some(true) {
        unsupported_reasons.push(
            "unsupported_runtime: unsupported wav2vec2 config do_stable_layer_norm=true; stable layer norm execution is not implemented"
                .to_string(),
        );
    }
    match raw.feat_extract_norm.as_deref() {
        Some("group") | Some("layer") | None => {}
        Some(other) => unsupported_reasons.push(format!(
            "unsupported_runtime: unsupported wav2vec2 feature extractor norm `{other}`"
        )),
    }
    let tensors =
        candle_core::safetensors::load(&paths.model_safetensors, &candle_core::Device::Cpu)
            .map_err(|error| {
                crate::setup_error(format!(
                    "failed to read wav2vec2 safetensors metadata `{}`: {error}",
                    paths.model_safetensors.display()
                ))
            })?;
    let mut missing_required_keys = missing_required_model_keys(&raw, &tensors);
    let positional_conv_layout = positional_conv_layout(&raw, &tensors, &mut missing_required_keys);
    if !missing_required_keys.is_empty() {
        unsupported_reasons.push(format!(
            "unsupported_runtime: wav2vec2 safetensors layout is missing {} required tensor key(s)",
            missing_required_keys.len()
        ));
    }
    Ok(Wav2Vec2LayoutReport {
        architecture: raw
            .architectures
            .first()
            .cloned()
            .unwrap_or_else(|| "unspecified".to_string()),
        do_stable_layer_norm: raw.do_stable_layer_norm.unwrap_or(false),
        positional_conv_layout,
        feature_extractor_norm: raw
            .feat_extract_norm
            .unwrap_or_else(|| "unspecified".to_string()),
        encoder_layer_count: raw.num_hidden_layers.unwrap_or(0),
        missing_required_keys,
        unsupported_reasons,
    })
}

#[allow(dead_code)]
pub(crate) fn parse_wav2vec2_config(path: &Path) -> Result<Wav2Vec2ConfigSummary> {
    let config = parse_raw_wav2vec2_config(path)?;
    validate_model_type_and_architecture(&config)?;
    Ok(Wav2Vec2ConfigSummary {
        model_type: config.model_type,
        architectures: config.architectures,
        vocab_size: config.vocab_size,
        word_delimiter_token: config.word_delimiter_token,
    })
}

pub(crate) fn parse_wav2vec2_ctc_config(path: &Path) -> Result<Wav2Vec2CtcConfig> {
    let raw = parse_raw_wav2vec2_config(path)?;
    validate_model_type_and_architecture(&raw)?;
    if raw.do_stable_layer_norm == Some(true) {
        return Err(unsupported_runtime(
            "unsupported wav2vec2 config do_stable_layer_norm=true; stable layer norm execution is not implemented",
        ));
    }
    let vocab_size = raw
        .vocab_size
        .ok_or_else(|| invalid_request("wav2vec2 config missing vocab_size"))?;
    let hidden_size = raw
        .hidden_size
        .ok_or_else(|| invalid_request("wav2vec2 config missing hidden_size"))?;
    let num_attention_heads = raw
        .num_attention_heads
        .ok_or_else(|| invalid_request("wav2vec2 config missing num_attention_heads"))?;
    if num_attention_heads == 0 || hidden_size % num_attention_heads != 0 {
        return Err(invalid_request(
            "wav2vec2 hidden_size must be divisible by num_attention_heads",
        ));
    }
    if raw.conv_dim.is_empty()
        || raw.conv_dim.len() != raw.conv_stride.len()
        || raw.conv_dim.len() != raw.conv_kernel.len()
    {
        return Err(invalid_request(
            "wav2vec2 conv_dim, conv_stride, and conv_kernel must be non-empty and have matching lengths",
        ));
    }
    if raw
        .conv_dim
        .iter()
        .chain(raw.conv_stride.iter())
        .chain(raw.conv_kernel.iter())
        .any(|value| *value == 0)
    {
        return Err(invalid_request(
            "wav2vec2 convolution dimensions, strides, and kernels must be positive",
        ));
    }
    Ok(Wav2Vec2CtcConfig {
        model_type: raw.model_type,
        architectures: raw.architectures,
        vocab_size,
        word_delimiter_token: raw.word_delimiter_token,
        hidden_size,
        num_hidden_layers: raw.num_hidden_layers.unwrap_or(0),
        num_attention_heads,
        intermediate_size: raw.intermediate_size.unwrap_or(hidden_size * 4),
        hidden_act: raw.hidden_act.unwrap_or_else(|| "gelu".to_string()),
        layer_norm_eps: raw.layer_norm_eps.unwrap_or(1e-5),
        feat_extract_norm: raw.feat_extract_norm,
        feat_extract_activation: raw
            .feat_extract_activation
            .unwrap_or_else(|| "gelu".to_string()),
        conv_dim: raw.conv_dim,
        conv_stride: raw.conv_stride,
        conv_kernel: raw.conv_kernel,
        conv_bias: raw.conv_bias.unwrap_or(false),
        num_conv_pos_embeddings: raw.num_conv_pos_embeddings.unwrap_or(0),
        num_conv_pos_embedding_groups: raw.num_conv_pos_embedding_groups.unwrap_or(1),
        do_stable_layer_norm: raw.do_stable_layer_norm.unwrap_or(false),
        final_dropout: raw.final_dropout,
        hidden_dropout: raw.hidden_dropout,
        activation_dropout: raw.activation_dropout,
        attention_dropout: raw.attention_dropout,
        layerdrop: raw.layerdrop,
        pad_token_id: raw.pad_token_id,
    })
}

pub(crate) fn parse_wav2vec2_preprocessor_config(
    path: &Path,
) -> Result<Wav2Vec2PreprocessorConfig> {
    let bytes = std::fs::read(path).map_err(|error| {
        crate::setup_error(format!(
            "failed to read wav2vec2 preprocessor config `{}`: {error}",
            path.display()
        ))
    })?;
    let raw: RawWav2Vec2PreprocessorConfig = serde_json::from_slice(&bytes).map_err(|error| {
        invalid_request(format!(
            "failed to parse wav2vec2 preprocessor config `{}`: {error}",
            path.display()
        ))
    })?;
    if let Some(sampling_rate) = raw.sampling_rate {
        if sampling_rate != 16_000 {
            return Err(invalid_request(format!(
                "wav2vec2 preprocessor sampling_rate must be 16000, got {sampling_rate}"
            )));
        }
    }
    Ok(Wav2Vec2PreprocessorConfig {
        sampling_rate: raw.sampling_rate,
        do_normalize: raw.do_normalize,
        return_attention_mask: raw.return_attention_mask,
    })
}

fn parse_raw_wav2vec2_config(path: &Path) -> Result<RawWav2Vec2Config> {
    let bytes = std::fs::read(path).map_err(|error| {
        crate::setup_error(format!(
            "failed to read wav2vec2 config `{}`: {error}",
            path.display()
        ))
    })?;
    serde_json::from_slice(&bytes).map_err(|error| {
        invalid_request(format!(
            "failed to parse wav2vec2 config `{}`: {error}",
            path.display()
        ))
    })
}

fn validate_model_type_and_architecture(raw: &RawWav2Vec2Config) -> Result<()> {
    if let Some(model_type) = raw.model_type.as_deref() {
        if model_type != "wav2vec2" {
            return Err(unsupported_runtime(format!(
                "unsupported CTC alignment model type `{model_type}`; expected wav2vec2"
            )));
        }
    }
    if !raw.architectures.is_empty()
        && !raw
            .architectures
            .iter()
            .any(|architecture| architecture == "Wav2Vec2ForCTC")
    {
        return Err(unsupported_runtime(format!(
            "unsupported wav2vec2 architecture `{}`; expected Wav2Vec2ForCTC",
            raw.architectures.join(",")
        )));
    }
    Ok(())
}

fn missing_required_model_keys(
    raw: &RawWav2Vec2Config,
    tensors: &std::collections::HashMap<String, candle_core::Tensor>,
) -> Vec<String> {
    let mut keys = Vec::new();
    for index in 0..raw.conv_dim.len() {
        let prefix = format!("wav2vec2.feature_extractor.conv_layers.{index}");
        keys.push(format!("{prefix}.conv.weight"));
        match raw.feat_extract_norm.as_deref() {
            Some("group") if index == 0 => {
                keys.push(format!("{prefix}.layer_norm.weight"));
                keys.push(format!("{prefix}.layer_norm.bias"));
            }
            Some("layer") => {
                keys.push(format!("{prefix}.layer_norm.weight"));
                keys.push(format!("{prefix}.layer_norm.bias"));
            }
            _ => {}
        }
    }
    keys.extend([
        "wav2vec2.feature_projection.layer_norm.weight".to_string(),
        "wav2vec2.feature_projection.layer_norm.bias".to_string(),
        "wav2vec2.feature_projection.projection.weight".to_string(),
        "lm_head.weight".to_string(),
    ]);
    for index in 0..raw.num_hidden_layers.unwrap_or(0) {
        let prefix = format!("wav2vec2.encoder.layers.{index}");
        keys.extend([
            format!("{prefix}.attention.q_proj.weight"),
            format!("{prefix}.attention.k_proj.weight"),
            format!("{prefix}.attention.v_proj.weight"),
            format!("{prefix}.attention.out_proj.weight"),
            format!("{prefix}.layer_norm.weight"),
            format!("{prefix}.layer_norm.bias"),
            format!("{prefix}.feed_forward.intermediate_dense.weight"),
            format!("{prefix}.feed_forward.output_dense.weight"),
            format!("{prefix}.final_layer_norm.weight"),
            format!("{prefix}.final_layer_norm.bias"),
        ]);
    }
    keys.into_iter()
        .filter(|key| !tensors.contains_key(key))
        .collect()
}

fn positional_conv_layout(
    raw: &RawWav2Vec2Config,
    tensors: &std::collections::HashMap<String, candle_core::Tensor>,
    missing_required_keys: &mut Vec<String>,
) -> String {
    if raw.num_conv_pos_embeddings.unwrap_or(0) == 0 {
        return "none".to_string();
    }
    let prefix = "wav2vec2.encoder.pos_conv_embed.conv";
    if tensors.contains_key(&format!("{prefix}.weight")) {
        return "plain".to_string();
    }
    let legacy_g = format!("{prefix}.weight_g");
    let legacy_v = format!("{prefix}.weight_v");
    if tensors.contains_key(&legacy_g) || tensors.contains_key(&legacy_v) {
        if !tensors.contains_key(&legacy_g) {
            missing_required_keys.push(legacy_g);
        }
        if !tensors.contains_key(&legacy_v) {
            missing_required_keys.push(legacy_v);
        }
        return "weight-norm".to_string();
    }
    let parametrized_g = format!("{prefix}.parametrizations.weight.original0");
    let parametrized_v = format!("{prefix}.parametrizations.weight.original1");
    if tensors.contains_key(&parametrized_g) || tensors.contains_key(&parametrized_v) {
        if !tensors.contains_key(&parametrized_g) {
            missing_required_keys.push(parametrized_g);
        }
        if !tensors.contains_key(&parametrized_v) {
            missing_required_keys.push(parametrized_v);
        }
        return "weight-norm".to_string();
    }
    missing_required_keys.push(format!("{prefix}.weight"));
    "missing".to_string()
}

pub(crate) fn parse_ctc_vocabulary(
    tokenizer_json: &Path,
    pad_token_id: Option<usize>,
) -> Result<CtcVocabulary> {
    let bytes = std::fs::read(tokenizer_json).map_err(|error| {
        crate::setup_error(format!(
            "failed to read wav2vec2 tokenizer `{}`: {error}",
            tokenizer_json.display()
        ))
    })?;
    let value: Value = serde_json::from_slice(&bytes).map_err(|error| {
        invalid_request(format!(
            "failed to parse wav2vec2 tokenizer `{}`: {error}",
            tokenizer_json.display()
        ))
    })?;
    let vocab = if let Some(vocab) = value.pointer("/model/vocab").and_then(Value::as_object) {
        vocab
    } else if let Some(vocab) = value.pointer("/vocab").and_then(Value::as_object) {
        vocab
    } else if let Some(vocab) = value.as_object().filter(|object| {
        !object.is_empty()
            && object
                .values()
                .all(|candidate| candidate.as_u64().is_some())
    }) {
        vocab
    } else {
        return Err(unsupported_runtime(
            "unsupported wav2vec2 tokenizer layout; expected flat model.vocab or vocab.json mapping",
        ));
    };
    let max_id = vocab
        .values()
        .filter_map(Value::as_u64)
        .max()
        .ok_or_else(|| unsupported_runtime("wav2vec2 tokenizer vocab is empty"))?
        as usize;
    let mut tokens = vec![None; max_id + 1];
    for (token, id) in vocab {
        let Some(id) = id.as_u64() else {
            return Err(unsupported_runtime(
                "unsupported wav2vec2 tokenizer layout; vocab ids must be integers",
            ));
        };
        let id = id as usize;
        if tokens[id].is_some() {
            return Err(unsupported_runtime(
                "unsupported wav2vec2 tokenizer layout; duplicate vocab id",
            ));
        }
        tokens[id] = Some(token.clone());
    }
    let tokens = tokens
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| {
            unsupported_runtime("unsupported wav2vec2 tokenizer layout; vocab ids must be dense")
        })?;
    let word_delimiter_token = value
        .pointer("/word_delimiter_token")
        .or_else(|| value.pointer("/model/word_delimiter_token"))
        .and_then(Value::as_str);
    let pad_token = value
        .pointer("/padding/pad_token")
        .or_else(|| value.pointer("/model/pad_token"))
        .and_then(Value::as_str);
    let blank_id = pad_token
        .and_then(|token| token_id(&tokens, token))
        .or(pad_token_id.filter(|id| *id < tokens.len()))
        .or_else(|| token_id(&tokens, "[PAD]"))
        .or_else(|| token_id(&tokens, "<pad>"))
        .or_else(|| {
            (word_delimiter_token == Some("|"))
                .then(|| token_id(&tokens, "|"))
                .flatten()
        })
        .ok_or_else(|| {
            unsupported_runtime(
                "unsupported wav2vec2 tokenizer layout; could not determine CTC blank token",
            )
        })?;
    Ok(CtcVocabulary { blank_id, tokens })
}

#[allow(dead_code)]
pub(crate) fn normalized_text_to_token_ids(
    text: &str,
    vocab: &CtcVocabulary,
) -> Result<Vec<usize>> {
    Ok(normalize_text_to_aligned_chars(text, vocab)?
        .into_iter()
        .map(|character| character.token_id)
        .collect())
}

fn normalize_text_to_aligned_chars(
    text: &str,
    vocab: &CtcVocabulary,
) -> Result<Vec<TokenizedAlignmentChar>> {
    let delimiter = token_id(&vocab.tokens, "|").map(|_| "|");
    let uppercase_hits = vocab
        .tokens
        .iter()
        .filter(|token| token.chars().any(|ch| ch.is_ascii_uppercase()))
        .count();
    let lowercase_hits = vocab
        .tokens
        .iter()
        .filter(|token| token.chars().any(|ch| ch.is_ascii_lowercase()))
        .count();
    let use_uppercase = uppercase_hits >= lowercase_hits;
    let mut chars = Vec::new();
    for (char_index, character) in text.trim().chars().enumerate() {
        if character.is_whitespace() {
            if let Some(delimiter) = delimiter {
                if let Some(id) = token_id(&vocab.tokens, delimiter) {
                    chars.push(TokenizedAlignmentChar {
                        char_index,
                        character: " ".to_string(),
                        token_id: id,
                        is_word_delimiter: true,
                    });
                }
            }
            continue;
        }
        let lowered = character.to_lowercase().collect::<String>();
        let token = if character.is_alphabetic() {
            if use_uppercase {
                character.to_uppercase().collect::<String>()
            } else {
                lowered.clone()
            }
        } else {
            character.to_string()
        };
        let token_id = token_id(&vocab.tokens, &token).unwrap_or(usize::MAX);
        chars.push(TokenizedAlignmentChar {
            char_index,
            character: lowered,
            token_id,
            is_word_delimiter: false,
        });
    }
    if chars.is_empty() {
        return Err(invalid_request(
            "transcript text does not contain any wav2vec2 CTC tokens",
        ));
    }
    Ok(chars)
}

fn tokens_to_segment_chars(
    segment_index: u64,
    tokens: &[CtcAlignedToken],
    chars: &[AlignedInputChar],
    segment_start: f64,
    segment_end: f64,
    frame_seconds: f64,
    interpolate_method: AlignmentInterpolationMethod,
) -> Result<Vec<AlignedChar>> {
    if chars.is_empty() {
        return Ok(Vec::new());
    }
    let mut projected = chars
        .iter()
        .enumerate()
        .map(|(position, character)| {
            let token = tokens.iter().find(|token| token.token_index == position);
            let (start_seconds, end_seconds, confidence) = token
                .map(|token| {
                    let start = segment_start + token.frame_index as f64 * frame_seconds;
                    let end = segment_start + (token.frame_index + 1) as f64 * frame_seconds;
                    (
                        Some(start.clamp(segment_start, segment_end)),
                        Some(end.clamp(segment_start, segment_end)),
                        Some(token.score),
                    )
                })
                .unwrap_or((None, None, None));
            AlignedChar {
                segment_index,
                char_index: character.char_index,
                character: character.character.clone(),
                start_seconds,
                end_seconds,
                confidence,
            }
        })
        .collect::<Vec<_>>();
    interpolate_missing_char_timings(
        &mut projected,
        segment_start,
        segment_end,
        interpolate_method,
    );
    Ok(projected)
}

fn whisperx_compatible_segment_chars(
    segment_index: u64,
    mut chars: Vec<AlignedChar>,
) -> Vec<AlignedChar> {
    if chars.is_empty()
        || chars
            .first()
            .is_some_and(|character| character.character == " ")
    {
        return chars;
    }
    for character in &mut chars {
        character.char_index += 1;
    }
    let mut projected = Vec::with_capacity(chars.len() + 1);
    projected.push(AlignedChar {
        segment_index,
        char_index: 0,
        character: " ".to_string(),
        start_seconds: None,
        end_seconds: None,
        confidence: None,
    });
    projected.extend(chars);
    projected
}

fn interpolate_missing_char_timings(
    chars: &mut [AlignedChar],
    segment_start: f64,
    segment_end: f64,
    method: AlignmentInterpolationMethod,
) {
    if method == AlignmentInterpolationMethod::Ignore {
        return;
    }
    for index in 0..chars.len() {
        if chars[index]
            .start_seconds
            .zip(chars[index].end_seconds)
            .is_some()
        {
            continue;
        }
        let previous = chars[..index]
            .iter()
            .rfind(|character| character.start_seconds.zip(character.end_seconds).is_some());
        let next = chars[index + 1..]
            .iter()
            .find(|character| character.start_seconds.zip(character.end_seconds).is_some());
        let midpoint = match method {
            AlignmentInterpolationMethod::Nearest => {
                nearest_midpoint(previous, next).unwrap_or((segment_start + segment_end) * 0.5)
            }
            AlignmentInterpolationMethod::Linear => linear_midpoint(index, previous, next, chars)
                .unwrap_or_else(|| {
                    nearest_midpoint(previous, next).unwrap_or((segment_start + segment_end) * 0.5)
                }),
            AlignmentInterpolationMethod::Ignore => continue,
        }
        .clamp(segment_start, segment_end);
        chars[index].start_seconds = Some(midpoint);
        chars[index].end_seconds = Some(midpoint);
    }
}

fn nearest_midpoint(previous: Option<&AlignedChar>, next: Option<&AlignedChar>) -> Option<f64> {
    match (previous, next) {
        (Some(previous), Some(next)) => {
            let previous_mid = char_midpoint(previous)?;
            let next_mid = char_midpoint(next)?;
            Some(if previous_mid.abs() <= next_mid.abs() {
                previous_mid
            } else {
                next_mid
            })
        }
        (Some(previous), None) => char_midpoint(previous),
        (None, Some(next)) => char_midpoint(next),
        (None, None) => None,
    }
}

fn linear_midpoint(
    index: usize,
    previous: Option<&AlignedChar>,
    next: Option<&AlignedChar>,
    chars: &[AlignedChar],
) -> Option<f64> {
    let previous = previous?;
    let next = next?;
    let previous_position = chars
        .iter()
        .position(|candidate| std::ptr::eq(candidate, previous))?;
    let next_position = chars
        .iter()
        .position(|candidate| std::ptr::eq(candidate, next))?;
    let denominator = (next_position - previous_position) as f64;
    if denominator <= 0.0 {
        return None;
    }
    let fraction = (index - previous_position) as f64 / denominator;
    Some(char_midpoint(previous)? + (char_midpoint(next)? - char_midpoint(previous)?) * fraction)
}

fn char_midpoint(character: &AlignedChar) -> Option<f64> {
    Some((character.start_seconds? + character.end_seconds?) * 0.5)
}

fn segment_words(segment: &text_transcripts::TranscriptSegmentContract) -> Vec<String> {
    if segment.words.is_empty() {
        segment
            .text
            .split_whitespace()
            .map(|word| word.trim().to_string())
            .filter(|word| !word.is_empty())
            .collect()
    } else {
        segment
            .words
            .iter()
            .map(|word| word.text.trim().to_string())
            .filter(|word| !word.is_empty())
            .collect()
    }
}

fn slice_segment_samples(
    samples: &[f32],
    sample_rate: u32,
    channels: u16,
    segment_start: f64,
    segment_end: f64,
) -> Result<Vec<f32>> {
    if sample_rate == 0 || channels == 0 {
        return Err(invalid_request(
            "audio sample rate and channels must be positive",
        ));
    }
    let channels = channels as usize;
    let frame_count = samples.len() / channels;
    let start_frame = (segment_start * sample_rate as f64)
        .round()
        .clamp(0.0, frame_count as f64) as usize;
    let end_frame = (segment_end * sample_rate as f64)
        .round()
        .clamp(start_frame as f64, frame_count as f64) as usize;
    if end_frame <= start_frame {
        return Err(invalid_request("alignment segment audio slice is empty"));
    }
    let mut mono = Vec::with_capacity(end_frame - start_frame);
    for frame in start_frame..end_frame {
        let offset = frame * channels;
        let value = if channels == 1 {
            samples[offset]
        } else {
            samples[offset..offset + channels]
                .iter()
                .copied()
                .sum::<f32>()
                / channels as f32
        };
        mono.push(value);
    }
    Ok(mono)
}

fn token_id(tokens: &[String], token: &str) -> Option<usize> {
    tokens.iter().position(|candidate| candidate == token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{Device, Tensor};
    use std::collections::HashMap;
    use text_transcripts::{TranscriptSegmentContract, TranscriptionContract};

    fn write_valid_bundle(root: &Path, nested: bool) {
        let file_root = if nested {
            std::fs::create_dir(root.join("files")).unwrap();
            root.join("files")
        } else {
            root.to_path_buf()
        };
        std::fs::write(
            file_root.join("config.json"),
            serde_json::json!({
                "model_type": "wav2vec2",
                "architectures": ["Wav2Vec2ForCTC"],
                "vocab_size": 10,
                "word_delimiter_token": "|",
                "hidden_size": 1,
                "num_hidden_layers": 0,
                "num_attention_heads": 1,
                "intermediate_size": 1,
                "hidden_act": "gelu",
                "layer_norm_eps": 1e-5,
                "feat_extract_activation": "gelu",
                "conv_dim": [1],
                "conv_stride": [1],
                "conv_kernel": [1],
                "conv_bias": false,
                "num_conv_pos_embeddings": 0,
                "num_conv_pos_embedding_groups": 1
            })
            .to_string(),
        )
        .unwrap();
        std::fs::write(file_root.join("tokenizer.json"), minimal_tokenizer()).unwrap();
        std::fs::write(file_root.join("preprocessor_config.json"), "{}").unwrap();
        std::fs::write(file_root.join("model.safetensors"), "").unwrap();
    }

    fn minimal_tokenizer() -> String {
        serde_json::json!({
            "version": "1.0",
            "word_delimiter_token": "|",
            "model": {
                "type": "WordLevel",
                "vocab": {
                    "[PAD]": 0,
                    "H": 1,
                    "E": 2,
                    "L": 3,
                    "O": 4,
                    "|": 5,
                    "W": 6,
                    "R": 7,
                    "D": 8,
                    "<unk>": 9
                },
                "unk_token": "<unk>"
            }
        })
        .to_string()
    }

    fn write_tiny_bundle(root: &Path) {
        write_valid_bundle(root, false);
        std::fs::write(
            root.join("preprocessor_config.json"),
            serde_json::json!({
                "sampling_rate": 16000,
                "do_normalize": false,
                "return_attention_mask": false
            })
            .to_string(),
        )
        .unwrap();
        let tensors = tiny_model_tensors();
        candle_core::safetensors::save(&tensors, root.join("model.safetensors")).unwrap();
    }

    fn write_tiny_feature_norm_bundle(root: &Path, norm: &str) {
        write_tiny_bundle(root);
        std::fs::write(
            root.join("config.json"),
            serde_json::json!({
                "model_type": "wav2vec2",
                "architectures": ["Wav2Vec2ForCTC"],
                "vocab_size": 10,
                "word_delimiter_token": "|",
                "hidden_size": 1,
                "num_hidden_layers": 0,
                "num_attention_heads": 1,
                "intermediate_size": 1,
                "hidden_act": "gelu",
                "layer_norm_eps": 1e-5,
                "feat_extract_norm": norm,
                "feat_extract_activation": "gelu",
                "conv_dim": [1],
                "conv_stride": [1],
                "conv_kernel": [1],
                "conv_bias": false,
                "num_conv_pos_embeddings": 0,
                "num_conv_pos_embedding_groups": 1
            })
            .to_string(),
        )
        .unwrap();
        let device = Device::Cpu;
        let mut tensors = tiny_model_tensors();
        tensors.insert(
            "wav2vec2.feature_extractor.conv_layers.0.layer_norm.weight".to_string(),
            Tensor::new(&[1.0f32], &device).unwrap(),
        );
        tensors.insert(
            "wav2vec2.feature_extractor.conv_layers.0.layer_norm.bias".to_string(),
            Tensor::new(&[0.0f32], &device).unwrap(),
        );
        candle_core::safetensors::save(&tensors, root.join("model.safetensors")).unwrap();
    }

    fn tiny_model_tensors() -> HashMap<String, Tensor> {
        let device = Device::Cpu;
        let mut tensors = HashMap::new();
        tensors.insert(
            "wav2vec2.feature_extractor.conv_layers.0.conv.weight".to_string(),
            Tensor::new(&[1.0f32], &device)
                .unwrap()
                .reshape((1, 1, 1))
                .unwrap(),
        );
        tensors.insert(
            "wav2vec2.feature_projection.layer_norm.weight".to_string(),
            Tensor::new(&[1.0f32], &device).unwrap(),
        );
        tensors.insert(
            "wav2vec2.feature_projection.layer_norm.bias".to_string(),
            Tensor::new(&[0.0f32], &device).unwrap(),
        );
        tensors.insert(
            "wav2vec2.feature_projection.projection.weight".to_string(),
            Tensor::new(&[1.0f32], &device)
                .unwrap()
                .reshape((1, 1))
                .unwrap(),
        );
        tensors.insert(
            "wav2vec2.feature_projection.projection.bias".to_string(),
            Tensor::new(&[0.0f32], &device).unwrap(),
        );
        tensors.insert(
            "lm_head.weight".to_string(),
            Tensor::new(&[0.0f32; 10], &device)
                .unwrap()
                .reshape((10, 1))
                .unwrap(),
        );
        tensors.insert(
            "lm_head.bias".to_string(),
            Tensor::new(&[0.0f32; 10], &device).unwrap(),
        );
        tensors
    }

    fn write_pos_conv_config(root: &Path) {
        std::fs::write(
            root.join("config.json"),
            serde_json::json!({
                "model_type": "wav2vec2",
                "architectures": ["Wav2Vec2ForCTC"],
                "vocab_size": 10,
                "word_delimiter_token": "|",
                "hidden_size": 1,
                "num_hidden_layers": 0,
                "num_attention_heads": 1,
                "intermediate_size": 1,
                "hidden_act": "gelu",
                "layer_norm_eps": 1e-5,
                "feat_extract_activation": "gelu",
                "conv_dim": [1],
                "conv_stride": [1],
                "conv_kernel": [1],
                "num_conv_pos_embeddings": 2,
                "num_conv_pos_embedding_groups": 1
            })
            .to_string(),
        )
        .unwrap();
    }

    fn write_tiny_weight_norm_bundle(root: &Path, parametrization_layout: bool, bad_g: bool) {
        write_tiny_bundle(root);
        write_pos_conv_config(root);
        let device = Device::Cpu;
        let mut tensors = tiny_model_tensors();
        let (g_name, v_name) = if parametrization_layout {
            (
                "wav2vec2.encoder.pos_conv_embed.conv.parametrizations.weight.original0",
                "wav2vec2.encoder.pos_conv_embed.conv.parametrizations.weight.original1",
            )
        } else {
            (
                "wav2vec2.encoder.pos_conv_embed.conv.weight_g",
                "wav2vec2.encoder.pos_conv_embed.conv.weight_v",
            )
        };
        let weight_g = if bad_g {
            Tensor::new(&[1.0f32, 2.0, 3.0], &device).unwrap()
        } else {
            Tensor::new(&[10.0f32], &device)
                .unwrap()
                .reshape((1, 1, 1))
                .unwrap()
        };
        tensors.insert(g_name.to_string(), weight_g);
        tensors.insert(
            v_name.to_string(),
            Tensor::new(&[3.0f32, 4.0], &device)
                .unwrap()
                .reshape((1, 1, 2))
                .unwrap(),
        );
        candle_core::safetensors::save(&tensors, root.join("model.safetensors")).unwrap();
    }

    fn write_tiny_kernel_weight_norm_bundle(root: &Path) {
        write_tiny_bundle(root);
        write_pos_conv_config(root);
        let device = Device::Cpu;
        let mut tensors = tiny_model_tensors();
        tensors.insert(
            "wav2vec2.encoder.pos_conv_embed.conv.weight_g".to_string(),
            Tensor::new(&[5.0f32, 10.0], &device)
                .unwrap()
                .reshape((1, 1, 2))
                .unwrap(),
        );
        tensors.insert(
            "wav2vec2.encoder.pos_conv_embed.conv.weight_v".to_string(),
            Tensor::new(&[3.0f32, 4.0], &device)
                .unwrap()
                .reshape((1, 1, 2))
                .unwrap(),
        );
        candle_core::safetensors::save(&tensors, root.join("model.safetensors")).unwrap();
    }

    fn write_tiny_plain_positional_conv_bundle(root: &Path) {
        write_tiny_bundle(root);
        write_pos_conv_config(root);
        let device = Device::Cpu;
        let mut tensors = tiny_model_tensors();
        tensors.insert(
            "wav2vec2.encoder.pos_conv_embed.conv.weight".to_string(),
            Tensor::new(&[0.5f32, 0.5], &device)
                .unwrap()
                .reshape((1, 1, 2))
                .unwrap(),
        );
        candle_core::safetensors::save(&tensors, root.join("model.safetensors")).unwrap();
    }

    fn alignment_request(text: &str) -> AlignmentRequest {
        let mut segment = TranscriptSegmentContract::new(7, text);
        segment.start_seconds = Some(0.0);
        segment.end_seconds = Some(1.0);
        let transcript =
            TranscriptionContract::from_segments(None, Some("en".to_string()), vec![segment])
                .unwrap();
        AlignmentRequest {
            audio: crate::LoadedAudio {
                samples: vec![0.0; 16_000],
                sample_rate: 16_000,
                channels: 1,
                source: None,
            },
            transcript,
            language: Some("en".to_string()),
            model_id: "facebook/wav2vec2-base-960h".to_string(),
        }
    }

    #[test]
    fn bundle_resolution_accepts_direct_and_files_layouts() {
        for nested in [false, true] {
            let temp = tempfile::tempdir().unwrap();
            write_valid_bundle(temp.path(), nested);
            let paths = resolve_wav2vec2_bundle_paths(temp.path()).unwrap();
            assert!(paths.config_json.exists());
            assert!(paths.tokenizer_vocab.exists());
            assert!(paths.preprocessor_config_json.exists());
            assert!(paths.model_safetensors.exists());
        }
    }

    #[cfg(feature = "model-bundles")]
    #[test]
    fn bundle_resolution_accepts_manifest_layout() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path();
        std::fs::create_dir(root.join("snapshots")).unwrap();
        write_valid_bundle(&root.join("snapshots"), false);
        std::fs::write(
            root.join("manifest.json"),
            serde_json::json!({
                "schema_version": 1,
                "name": "wav2vec2-test",
                "repo_id": "facebook/wav2vec2-base-960h",
                "revision": "main",
                "task": "speech_recognition",
                "files": {
                    "config.json": {"remote_path": "config.json", "local_path": "snapshots/config.json", "size_bytes": 0},
                    "tokenizer.json": {"remote_path": "tokenizer.json", "local_path": "snapshots/tokenizer.json", "size_bytes": 0},
                    "preprocessor_config.json": {"remote_path": "preprocessor_config.json", "local_path": "snapshots/preprocessor_config.json", "size_bytes": 0},
                    "model.safetensors": {"remote_path": "model.safetensors", "local_path": "snapshots/model.safetensors", "size_bytes": 0}
                }
            })
            .to_string(),
        )
        .unwrap();
        let paths = resolve_wav2vec2_bundle_paths(root).unwrap();
        assert_eq!(
            paths.model_safetensors,
            root.join("snapshots/model.safetensors")
        );
    }

    #[test]
    fn tokenizer_vocab_parser_accepts_minimal_wav2vec2_layout() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("tokenizer.json");
        std::fs::write(&path, minimal_tokenizer()).unwrap();
        let vocab = parse_ctc_vocabulary(&path, None).unwrap();
        assert_eq!(vocab.blank_id, 0);
        assert_eq!(vocab.tokens[5], "|");
    }

    #[test]
    fn vocab_json_layout_is_supported() {
        let temp = tempfile::tempdir().unwrap();
        write_valid_bundle(temp.path(), false);
        std::fs::remove_file(temp.path().join("tokenizer.json")).unwrap();
        std::fs::write(
            temp.path().join("vocab.json"),
            serde_json::json!({
                "[PAD]": 0,
                "H": 1,
                "E": 2,
                "L": 3,
                "O": 4,
                "|": 5,
                "W": 6,
                "R": 7,
                "D": 8,
                "<unk>": 9
            })
            .to_string(),
        )
        .unwrap();

        let paths = resolve_wav2vec2_bundle_paths(temp.path()).unwrap();
        assert_eq!(paths.tokenizer_vocab, temp.path().join("vocab.json"));
        let config = parse_wav2vec2_ctc_config(&paths.config_json).unwrap();
        let vocab = parse_ctc_vocabulary(&paths.tokenizer_vocab, config.pad_token_id).unwrap();

        assert_eq!(vocab.blank_id, 0);
        assert_eq!(vocab.tokens[5], "|");
    }

    #[test]
    fn pad_token_id_can_define_ctc_blank() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("vocab.json");
        std::fs::write(
            &path,
            serde_json::json!({
                "A": 0,
                "B": 1,
                "|": 2,
                "<unk>": 3
            })
            .to_string(),
        )
        .unwrap();

        let vocab = parse_ctc_vocabulary(&path, Some(3)).unwrap();

        assert_eq!(vocab.blank_id, 3);
    }

    #[test]
    fn text_normalization_maps_words_to_ctc_ids() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("tokenizer.json");
        std::fs::write(&path, minimal_tokenizer()).unwrap();
        let vocab = parse_ctc_vocabulary(&path, None).unwrap();
        let ids = normalized_text_to_token_ids("hello world!", &vocab).unwrap();
        assert_eq!(ids, vec![1, 2, 3, 3, 4, 5, 6, 4, 7, 3, 8, usize::MAX]);
    }

    #[test]
    fn unsupported_tokenizer_layout_returns_unsupported_runtime() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("tokenizer.json");
        std::fs::write(
            &path,
            serde_json::json!({"model": {"type": "BPE"}}).to_string(),
        )
        .unwrap();
        let error = parse_ctc_vocabulary(&path, None).unwrap_err().to_string();
        assert!(error.contains("unsupported_runtime"));
        assert!(error.contains("flat model.vocab"));
    }

    #[test]
    fn wav2vec2_config_accepts_minimal_ctc_config() {
        let temp = tempfile::tempdir().unwrap();
        write_valid_bundle(temp.path(), false);
        let config = parse_wav2vec2_ctc_config(&temp.path().join("config.json")).unwrap();
        assert_eq!(config.vocab_size, 10);
        assert_eq!(config.hidden_size, 1);
        assert_eq!(config.conv_dim, vec![1]);
    }

    #[test]
    fn wav2vec2_config_rejects_bad_conv_shapes() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(
            temp.path().join("config.json"),
            serde_json::json!({
                "model_type": "wav2vec2",
                "architectures": ["Wav2Vec2ForCTC"],
                "vocab_size": 10,
                "hidden_size": 4,
                "num_attention_heads": 2,
                "conv_dim": [4, 4],
                "conv_stride": [2],
                "conv_kernel": [3, 3]
            })
            .to_string(),
        )
        .unwrap();
        let error = parse_wav2vec2_ctc_config(&temp.path().join("config.json"))
            .unwrap_err()
            .to_string();
        assert!(error.contains("invalid_request"));
        assert!(error.contains("conv_dim"));
    }

    #[test]
    fn stable_layer_norm_config_reports_clear_unsupported_runtime_until_implemented() {
        let temp = tempfile::tempdir().unwrap();
        write_valid_bundle(temp.path(), false);
        std::fs::write(
            temp.path().join("config.json"),
            serde_json::json!({
                "model_type": "wav2vec2",
                "architectures": ["Wav2Vec2ForCTC"],
                "vocab_size": 10,
                "hidden_size": 1,
                "num_hidden_layers": 0,
                "num_attention_heads": 1,
                "conv_dim": [1],
                "conv_stride": [1],
                "conv_kernel": [1],
                "do_stable_layer_norm": true
            })
            .to_string(),
        )
        .unwrap();

        let error = parse_wav2vec2_ctc_config(&temp.path().join("config.json"))
            .unwrap_err()
            .to_string();

        assert!(error.contains("unsupported_runtime"));
        assert!(error.contains("do_stable_layer_norm"));
    }

    #[test]
    fn wav2vec2_preprocessor_accepts_16khz() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("preprocessor_config.json");
        std::fs::write(
            &path,
            serde_json::json!({"sampling_rate": 16000, "do_normalize": true}).to_string(),
        )
        .unwrap();
        let config = parse_wav2vec2_preprocessor_config(&path).unwrap();
        assert_eq!(config.sampling_rate, Some(16_000));
        assert_eq!(config.do_normalize, Some(true));
    }

    #[test]
    fn wav2vec2_preprocessor_rejects_non_16khz() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("preprocessor_config.json");
        std::fs::write(
            &path,
            serde_json::json!({"sampling_rate": 8000}).to_string(),
        )
        .unwrap();
        let error = parse_wav2vec2_preprocessor_config(&path)
            .unwrap_err()
            .to_string();
        assert!(error.contains("invalid_request"));
        assert!(error.contains("16000"));
    }

    #[test]
    fn unsupported_wav2vec2_layout_reports_missing_key() {
        let temp = tempfile::tempdir().unwrap();
        write_valid_bundle(temp.path(), false);
        let error = emit_wav2vec2_ctc(temp.path(), &alignment_request("hello"))
            .unwrap_err()
            .to_string();
        assert!(error.contains("unsupported_runtime"));
        assert!(error.contains("safetensors"));
    }

    #[test]
    fn wav2vec2_layout_report_identifies_plain_positional_conv() {
        let temp = tempfile::tempdir().unwrap();
        write_tiny_plain_positional_conv_bundle(temp.path());

        let report = inspect_wav2vec2_bundle_layout(temp.path()).unwrap();

        assert_eq!(report.architecture, "Wav2Vec2ForCTC");
        assert_eq!(report.positional_conv_layout, "plain");
        assert!(!report.do_stable_layer_norm);
        assert!(report.missing_required_keys.is_empty());
        assert!(report.unsupported_reasons.is_empty());
    }

    #[test]
    fn wav2vec2_layout_report_identifies_weight_norm_positional_conv() {
        let temp = tempfile::tempdir().unwrap();
        write_tiny_weight_norm_bundle(temp.path(), true, false);

        let report = inspect_wav2vec2_bundle_layout(temp.path()).unwrap();

        assert_eq!(report.positional_conv_layout, "weight-norm");
        assert!(report.missing_required_keys.is_empty());
        assert!(report.unsupported_reasons.is_empty());
    }

    #[test]
    fn wav2vec2_layout_report_flags_stable_layer_norm() {
        let temp = tempfile::tempdir().unwrap();
        write_tiny_bundle(temp.path());
        std::fs::write(
            temp.path().join("config.json"),
            serde_json::json!({
                "model_type": "wav2vec2",
                "architectures": ["Wav2Vec2ForCTC"],
                "vocab_size": 10,
                "hidden_size": 1,
                "num_hidden_layers": 0,
                "num_attention_heads": 1,
                "conv_dim": [1],
                "conv_stride": [1],
                "conv_kernel": [1],
                "do_stable_layer_norm": true
            })
            .to_string(),
        )
        .unwrap();

        let report = inspect_wav2vec2_bundle_layout(temp.path()).unwrap();

        assert!(report.do_stable_layer_norm);
        assert!(report
            .unsupported_reasons
            .iter()
            .any(|reason| reason.contains("unsupported_runtime")
                && reason.contains("do_stable_layer_norm")));
    }

    #[test]
    fn wav2vec2_feature_extractor_group_norm_executes_tiny_bundle() {
        let temp = tempfile::tempdir().unwrap();
        write_tiny_feature_norm_bundle(temp.path(), "group");

        let emissions = emit_wav2vec2_ctc(temp.path(), &alignment_request("hello")).unwrap();

        assert!(!emissions.is_empty());
        assert!(emissions
            .iter()
            .all(|frame| frame.len() == 10 && frame.iter().all(|score| score.is_finite())));
    }

    #[test]
    fn wav2vec2_feature_extractor_layer_norm_executes_tiny_bundle() {
        let temp = tempfile::tempdir().unwrap();
        write_tiny_feature_norm_bundle(temp.path(), "layer");

        let emissions = emit_wav2vec2_ctc(temp.path(), &alignment_request("hello")).unwrap();

        assert!(!emissions.is_empty());
        assert!(emissions
            .iter()
            .all(|frame| frame.len() == 10 && frame.iter().all(|score| score.is_finite())));
    }

    #[test]
    fn positional_conv_weight_norm_layout_reconstructs_weight() {
        let temp = tempfile::tempdir().unwrap();
        write_tiny_weight_norm_bundle(temp.path(), false, false);

        let emissions = emit_wav2vec2_ctc(temp.path(), &alignment_request("hello")).unwrap();

        assert!(!emissions.is_empty());
        assert!(emissions
            .iter()
            .all(|frame| frame.len() == 10 && frame.iter().all(|score| score.is_finite())));
    }

    #[test]
    fn positional_conv_kernel_weight_norm_layout_reconstructs_weight() {
        let temp = tempfile::tempdir().unwrap();
        write_tiny_kernel_weight_norm_bundle(temp.path());

        let emissions = emit_wav2vec2_ctc(temp.path(), &alignment_request("hello")).unwrap();

        assert!(!emissions.is_empty());
        assert!(emissions
            .iter()
            .all(|frame| frame.len() == 10 && frame.iter().all(|score| score.is_finite())));
    }

    #[test]
    fn positional_conv_parametrization_layout_reconstructs_weight() {
        let temp = tempfile::tempdir().unwrap();
        write_tiny_weight_norm_bundle(temp.path(), true, false);

        let emissions = emit_wav2vec2_ctc(temp.path(), &alignment_request("hello")).unwrap();

        assert!(!emissions.is_empty());
        assert!(emissions
            .iter()
            .all(|frame| frame.len() == 10 && frame.iter().all(|score| score.is_finite())));
    }

    #[test]
    fn positional_conv_weight_norm_rejects_bad_shape() {
        let temp = tempfile::tempdir().unwrap();
        write_tiny_weight_norm_bundle(temp.path(), false, true);

        let error = emit_wav2vec2_ctc(temp.path(), &alignment_request("hello"))
            .unwrap_err()
            .to_string();

        assert!(error.contains("model_output_mismatch"));
        assert!(error.contains("positional convolution weight norm"));
    }

    #[test]
    fn tiny_wav2vec2_model_emits_finite_log_probs() {
        let temp = tempfile::tempdir().unwrap();
        write_tiny_bundle(temp.path());
        let emissions = emit_wav2vec2_ctc(temp.path(), &alignment_request("hello")).unwrap();
        assert!(!emissions.is_empty());
        assert!(emissions
            .iter()
            .all(|frame| frame.len() == 10 && frame.iter().all(|score| score.is_finite())));
    }

    #[test]
    fn alignment_with_tiny_wav2vec2_bundle_returns_words() {
        let temp = tempfile::tempdir().unwrap();
        write_tiny_bundle(temp.path());
        let result = align_wav2vec2_ctc(
            temp.path(),
            &alignment_request("hello"),
            &ResolvedNativeDevice::Cpu,
            AlignmentInterpolationMethod::Nearest,
            true,
        )
        .unwrap();
        assert_eq!(result.words.len(), 1);
        assert_eq!(result.words[0].segment_index, 7);
        assert_eq!(result.words[0].text, "hello");
        assert!(result.words[0].start_seconds >= 0.0);
        assert!(result.words[0].end_seconds <= 1.0);
        assert!(result.words[0].end_seconds >= result.words[0].start_seconds);
        assert!(result.words[0].confidence.is_some());
        assert_eq!(result.chars.len(), 6);
        assert_eq!(result.chars[0].character, " ");
        assert!(result.chars[0].start_seconds.is_none());
        assert_eq!(result.chars[1].character, "h");
        assert!(result.chars[1].start_seconds.is_some());
    }

    #[test]
    fn whisperx_compatible_chars_include_leading_space_without_word_change() {
        let chars = vec![
            AlignedChar {
                segment_index: 3,
                char_index: 0,
                character: "T".to_string(),
                start_seconds: Some(0.1),
                end_seconds: Some(0.2),
                confidence: Some(0.9),
            },
            AlignedChar {
                segment_index: 3,
                char_index: 1,
                character: "h".to_string(),
                start_seconds: Some(0.2),
                end_seconds: Some(0.3),
                confidence: Some(0.9),
            },
        ];
        let projected = whisperx_compatible_segment_chars(3, chars);

        assert_eq!(projected.len(), 3);
        assert_eq!(projected[0].character, " ");
        assert_eq!(projected[0].char_index, 0);
        assert!(projected[0].start_seconds.is_none());
        assert_eq!(projected[1].character, "T");
        assert_eq!(projected[1].char_index, 1);
        assert_eq!(projected[2].char_index, 2);
    }

    #[test]
    fn smoke_sentence_char_projection_matches_whisperx_count() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("tokenizer.json");
        std::fs::write(&path, minimal_tokenizer()).unwrap();
        let vocab = parse_ctc_vocabulary(&path, None).unwrap();
        let chars = normalize_text_to_aligned_chars("This is a test.", &vocab).unwrap();
        let aligned = chars
            .iter()
            .map(|character| AlignedChar {
                segment_index: 0,
                char_index: character.char_index,
                character: character.character.clone(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.01),
                confidence: Some(1.0),
            })
            .collect();
        let projected = whisperx_compatible_segment_chars(0, aligned);

        assert_eq!(projected.len(), 16);
        assert_eq!(projected[0].character, " ");
        assert!(projected[1..].iter().all(|character| {
            character.start_seconds.is_some() && character.end_seconds.is_some()
        }));
    }
}
