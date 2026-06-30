use std::collections::HashMap;
use std::path::Path;

use candle_core::{DType, Device, Tensor};
use candle_nn::{ops, Activation, Conv1d, Conv1dConfig, GroupNorm, LayerNorm, Linear, Module};
use video_analysis_core::Result;

use crate::native_wav2vec2::{Wav2Vec2CtcConfig, Wav2Vec2PreprocessorConfig};
use crate::{model_output_mismatch, unsupported_runtime};

#[derive(Clone)]
pub(crate) struct Wav2Vec2ForCtc {
    config: Wav2Vec2CtcConfig,
    preprocessor: Wav2Vec2PreprocessorConfig,
    device: Device,
    feature_extractor: Vec<FeatureExtractorLayer>,
    feature_projection_norm: LayerNorm,
    feature_projection: Linear,
    pos_conv: Option<Conv1d>,
    encoder_layers: Vec<EncoderLayer>,
    encoder_norm: Option<LayerNorm>,
    lm_head: Linear,
}

#[derive(Clone)]
struct FeatureExtractorLayer {
    conv: Conv1d,
    norm: Option<FeatureNorm>,
    activation: Activation,
}

#[derive(Clone)]
enum FeatureNorm {
    Group(GroupNorm),
    Layer(LayerNorm),
}

#[derive(Clone)]
struct EncoderLayer {
    attention: SelfAttention,
    layer_norm: LayerNorm,
    feed_forward: FeedForward,
    final_layer_norm: LayerNorm,
}

#[derive(Clone)]
struct SelfAttention {
    q_proj: Linear,
    k_proj: Linear,
    v_proj: Linear,
    out_proj: Linear,
    num_heads: usize,
    head_dim: usize,
}

#[derive(Clone)]
struct FeedForward {
    intermediate_dense: Linear,
    output_dense: Linear,
    activation: Activation,
}

impl Wav2Vec2ForCtc {
    pub(crate) fn load(
        model_safetensors: &Path,
        config: Wav2Vec2CtcConfig,
        preprocessor: Wav2Vec2PreprocessorConfig,
        device: Device,
    ) -> Result<Self> {
        let tensors =
            candle_core::safetensors::load(model_safetensors, &device).map_err(|error| {
                unsupported_runtime(format!(
                    "failed to load wav2vec2 safetensors `{}`: {error}",
                    model_safetensors.display()
                ))
            })?;
        Self::from_tensors_inner(tensors, config, preprocessor, device)
    }

    #[cfg(test)]
    pub(crate) fn from_tensors(
        tensors: HashMap<String, Tensor>,
        config: Wav2Vec2CtcConfig,
        preprocessor: Wav2Vec2PreprocessorConfig,
    ) -> Result<Self> {
        Self::from_tensors_inner(tensors, config, preprocessor, Device::Cpu)
    }

    fn from_tensors_inner(
        tensors: HashMap<String, Tensor>,
        config: Wav2Vec2CtcConfig,
        preprocessor: Wav2Vec2PreprocessorConfig,
        device: Device,
    ) -> Result<Self> {
        let mut feature_extractor = Vec::with_capacity(config.conv_dim.len());
        let activation = parse_activation(&config.feat_extract_activation)?;
        for index in 0..config.conv_dim.len() {
            let prefix = format!("wav2vec2.feature_extractor.conv_layers.{index}");
            let conv = conv1d(
                &tensors,
                &format!("{prefix}.conv"),
                config.conv_stride[index],
                0,
                1,
                config.conv_bias,
            )?;
            let norm = match config.feat_extract_norm.as_deref() {
                Some("group") if index == 0 => Some(FeatureNorm::Group(group_norm(
                    &tensors,
                    &format!("{prefix}.layer_norm"),
                    config.conv_dim[index],
                    config.conv_dim[index],
                    config.layer_norm_eps,
                )?)),
                Some("layer") => Some(FeatureNorm::Layer(layer_norm(
                    &tensors,
                    &format!("{prefix}.layer_norm"),
                    config.conv_dim[index],
                    config.layer_norm_eps,
                )?)),
                Some("group") | None => None,
                Some(other) => {
                    return Err(unsupported_runtime(format!(
                        "unsupported wav2vec2 feature extractor norm `{other}`"
                    )));
                }
            };
            feature_extractor.push(FeatureExtractorLayer {
                conv,
                norm,
                activation,
            });
        }

        let projection_prefix = "wav2vec2.feature_projection";
        let feature_projection_norm = layer_norm(
            &tensors,
            &format!("{projection_prefix}.layer_norm"),
            *config.conv_dim.last().unwrap_or(&config.hidden_size),
            config.layer_norm_eps,
        )?;
        let feature_projection =
            linear(&tensors, &format!("{projection_prefix}.projection"), true)?;
        let pos_conv = load_pos_conv(&tensors, &config)?;

        let mut encoder_layers = Vec::with_capacity(config.num_hidden_layers);
        let hidden_act = parse_activation(&config.hidden_act)?;
        for index in 0..config.num_hidden_layers {
            let prefix = format!("wav2vec2.encoder.layers.{index}");
            let attention_prefix = format!("{prefix}.attention");
            encoder_layers.push(EncoderLayer {
                attention: SelfAttention {
                    q_proj: linear(&tensors, &format!("{attention_prefix}.q_proj"), true)?,
                    k_proj: linear(&tensors, &format!("{attention_prefix}.k_proj"), true)?,
                    v_proj: linear(&tensors, &format!("{attention_prefix}.v_proj"), true)?,
                    out_proj: linear(&tensors, &format!("{attention_prefix}.out_proj"), true)?,
                    num_heads: config.num_attention_heads,
                    head_dim: config.hidden_size / config.num_attention_heads,
                },
                layer_norm: layer_norm(
                    &tensors,
                    &format!("{prefix}.layer_norm"),
                    config.hidden_size,
                    config.layer_norm_eps,
                )?,
                feed_forward: FeedForward {
                    intermediate_dense: linear(
                        &tensors,
                        &format!("{prefix}.feed_forward.intermediate_dense"),
                        true,
                    )?,
                    output_dense: linear(
                        &tensors,
                        &format!("{prefix}.feed_forward.output_dense"),
                        true,
                    )?,
                    activation: hidden_act,
                },
                final_layer_norm: layer_norm(
                    &tensors,
                    &format!("{prefix}.final_layer_norm"),
                    config.hidden_size,
                    config.layer_norm_eps,
                )?,
            });
        }
        let encoder_norm = if tensor_exists(&tensors, "wav2vec2.encoder.layer_norm.weight") {
            Some(layer_norm(
                &tensors,
                "wav2vec2.encoder.layer_norm",
                config.hidden_size,
                config.layer_norm_eps,
            )?)
        } else {
            None
        };
        let lm_head = linear(
            &tensors,
            "lm_head",
            optional_tensor(&tensors, "lm_head.bias").is_some(),
        )?;
        Ok(Self {
            config,
            preprocessor,
            device,
            feature_extractor,
            feature_projection_norm,
            feature_projection,
            pos_conv,
            encoder_layers,
            encoder_norm,
            lm_head,
        })
    }

    pub(crate) fn emit_log_probs(&self, samples: &[f32]) -> Result<Vec<Vec<f32>>> {
        if samples.is_empty() {
            return Err(model_output_mismatch(
                "wav2vec2 CTC segment audio slice is empty",
            ));
        }
        let samples = if self.preprocessor.do_normalize.unwrap_or(true) {
            normalize_samples(samples)
        } else {
            samples.to_vec()
        };
        let mut hidden = Tensor::new(samples.as_slice(), &self.device)
            .map_err(candle_mismatch)?
            .reshape((1, 1, samples.len()))
            .map_err(candle_mismatch)?;
        for layer in &self.feature_extractor {
            hidden = layer.conv.forward(&hidden).map_err(candle_mismatch)?;
            if let Some(norm) = &layer.norm {
                hidden = match norm {
                    FeatureNorm::Group(norm) => norm.forward(&hidden).map_err(candle_mismatch)?,
                    FeatureNorm::Layer(norm) => {
                        let normalized = hidden.transpose(1, 2).map_err(candle_mismatch)?;
                        let normalized = norm.forward(&normalized).map_err(candle_mismatch)?;
                        normalized.transpose(1, 2).map_err(candle_mismatch)?
                    }
                };
            }
            hidden = layer.activation.forward(&hidden).map_err(candle_mismatch)?;
        }

        hidden = hidden.transpose(1, 2).map_err(candle_mismatch)?;
        hidden = self
            .feature_projection_norm
            .forward(&hidden)
            .map_err(candle_mismatch)?;
        hidden = self
            .feature_projection
            .forward(&hidden)
            .map_err(candle_mismatch)?;

        if let Some(pos_conv) = &self.pos_conv {
            let sequence_len = hidden.dim(1).map_err(candle_mismatch)?;
            let positional = pos_conv
                .forward(&hidden.transpose(1, 2).map_err(candle_mismatch)?)
                .map_err(candle_mismatch)?
                .transpose(1, 2)
                .map_err(candle_mismatch)?;
            let positional = match positional
                .dim(1)
                .map_err(candle_mismatch)?
                .cmp(&sequence_len)
            {
                std::cmp::Ordering::Greater => positional
                    .narrow(1, 0, sequence_len)
                    .map_err(candle_mismatch)?,
                std::cmp::Ordering::Less => {
                    return Err(unsupported_runtime(
                        "wav2vec2 positional convolution produced a shorter sequence than the feature projection",
                    ));
                }
                std::cmp::Ordering::Equal => positional,
            };
            hidden = hidden.broadcast_add(&positional).map_err(candle_mismatch)?;
        }

        for layer in &self.encoder_layers {
            hidden = layer.forward(&hidden).map_err(candle_mismatch)?;
        }
        if let Some(norm) = &self.encoder_norm {
            hidden = norm.forward(&hidden).map_err(candle_mismatch)?;
        }
        let logits = self.lm_head.forward(&hidden).map_err(candle_mismatch)?;
        let log_probs =
            ops::log_softmax(&logits, candle_core::D::Minus1).map_err(candle_mismatch)?;
        let log_probs = log_probs.squeeze(0).map_err(candle_mismatch)?;
        let emissions = log_probs.to_vec2::<f32>().map_err(candle_mismatch)?;
        if emissions.is_empty() {
            return Err(model_output_mismatch("wav2vec2 CTC emissions are empty"));
        }
        if emissions.iter().any(|frame| {
            frame.len() != self.config.vocab_size || frame.iter().any(|v| !v.is_finite())
        }) {
            return Err(model_output_mismatch(
                "wav2vec2 CTC emission dimensions are inconsistent",
            ));
        }
        Ok(emissions)
    }
}

impl EncoderLayer {
    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        let residual = xs;
        let attention = self.attention.forward(xs)?;
        let hidden = residual.broadcast_add(&attention)?;
        let attention_norm = self.layer_norm.forward(&hidden)?;
        let feed_forward = self.feed_forward.forward(&attention_norm)?;
        let hidden = attention_norm.broadcast_add(&feed_forward)?;
        self.final_layer_norm.forward(&hidden)
    }
}

impl SelfAttention {
    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        let (batch, time, hidden) = xs.dims3()?;
        let q = self
            .q_proj
            .forward(xs)?
            .reshape((batch, time, self.num_heads, self.head_dim))?
            .transpose(1, 2)?
            .contiguous()?;
        let k = self
            .k_proj
            .forward(xs)?
            .reshape((batch, time, self.num_heads, self.head_dim))?
            .transpose(1, 2)?
            .contiguous()?;
        let v = self
            .v_proj
            .forward(xs)?
            .reshape((batch, time, self.num_heads, self.head_dim))?
            .transpose(1, 2)?
            .contiguous()?;
        let k_t = k.transpose(2, 3)?.contiguous()?;
        let scores = (q.matmul(&k_t)? / (self.head_dim as f64).sqrt())?;
        let weights = ops::softmax(&scores, candle_core::D::Minus1)?;
        let context = weights
            .matmul(&v)?
            .transpose(1, 2)?
            .contiguous()?
            .reshape((batch, time, hidden))?;
        self.out_proj.forward(&context)
    }
}

impl FeedForward {
    fn forward(&self, xs: &Tensor) -> candle_core::Result<Tensor> {
        let hidden = self.intermediate_dense.forward(xs)?;
        let hidden = self.activation.forward(&hidden)?;
        self.output_dense.forward(&hidden)
    }
}

fn normalize_samples(samples: &[f32]) -> Vec<f32> {
    let mean = samples.iter().copied().sum::<f32>() / samples.len() as f32;
    let variance = samples
        .iter()
        .map(|sample| {
            let centered = *sample - mean;
            centered * centered
        })
        .sum::<f32>()
        / samples.len() as f32;
    let scale = variance.sqrt().max(1e-7);
    samples
        .iter()
        .map(|sample| (*sample - mean) / scale)
        .collect()
}

fn parse_activation(name: &str) -> Result<Activation> {
    match name {
        "gelu" => Ok(Activation::Gelu),
        "gelu_new" | "gelu_pytorch_tanh" => Ok(Activation::GeluPytorchTanh),
        "relu" => Ok(Activation::Relu),
        other => Err(unsupported_runtime(format!(
            "unsupported wav2vec2 activation `{other}`"
        ))),
    }
}

fn load_pos_conv(
    tensors: &HashMap<String, Tensor>,
    config: &Wav2Vec2CtcConfig,
) -> Result<Option<Conv1d>> {
    if config.num_conv_pos_embeddings == 0 {
        return Ok(None);
    }
    let prefix = "wav2vec2.encoder.pos_conv_embed.conv";
    let weight = load_pos_conv_weight(tensors, prefix)?;
    let bias = optional_tensor(tensors, &format!("{prefix}.bias"));
    Ok(Some(Conv1d::new(
        weight,
        bias,
        Conv1dConfig {
            padding: config.num_conv_pos_embeddings / 2,
            groups: config.num_conv_pos_embedding_groups.max(1),
            ..Default::default()
        },
    )))
}

fn load_pos_conv_weight(tensors: &HashMap<String, Tensor>, prefix: &str) -> Result<Tensor> {
    if let Some(weight) = optional_tensor(tensors, &format!("{prefix}.weight")) {
        return Ok(weight);
    }
    if let (Some(weight_g), Some(weight_v)) = (
        optional_tensor(tensors, &format!("{prefix}.weight_g")),
        optional_tensor(tensors, &format!("{prefix}.weight_v")),
    ) {
        return reconstruct_pos_conv_weight_norm(&weight_g, &weight_v);
    }
    if let (Some(weight_g), Some(weight_v)) = (
        optional_tensor(
            tensors,
            &format!("{prefix}.parametrizations.weight.original0"),
        ),
        optional_tensor(
            tensors,
            &format!("{prefix}.parametrizations.weight.original1"),
        ),
    ) {
        return reconstruct_pos_conv_weight_norm(&weight_g, &weight_v);
    }
    if tensor_exists(tensors, &format!("{prefix}.weight_g"))
        || tensor_exists(tensors, &format!("{prefix}.weight_v"))
        || tensor_exists(
            tensors,
            &format!("{prefix}.parametrizations.weight.original0"),
        )
        || tensor_exists(
            tensors,
            &format!("{prefix}.parametrizations.weight.original1"),
        )
    {
        return Err(model_output_mismatch(
            "wav2vec2 positional convolution weight norm tensors are incomplete",
        ));
    }
    Err(unsupported_runtime(format!(
        "unsupported wav2vec2 safetensors layout; missing `{prefix}.weight` or known positional convolution weight norm tensors"
    )))
}

fn reconstruct_pos_conv_weight_norm(weight_g: &Tensor, weight_v: &Tensor) -> Result<Tensor> {
    let (out_channels, in_channels_per_group, kernel) =
        weight_v.dims3().map_err(|_| {
            model_output_mismatch(
                "wav2vec2 positional convolution weight norm weight_v must have shape [out_channels, in_channels_per_group, kernel]",
            )
        })?;
    if weight_g.elem_count() != out_channels && weight_g.elem_count() != kernel {
        return Err(model_output_mismatch(format!(
            "wav2vec2 positional convolution weight norm weight_g has {} elements but expected {out_channels} or {kernel}",
            weight_g.elem_count(),
        )));
    }
    let scale = weight_g
        .flatten_all()
        .map_err(candle_mismatch)?
        .to_vec1::<f32>()
        .map_err(candle_mismatch)?;
    let raw = weight_v.to_vec3::<f32>().map_err(candle_mismatch)?;
    let mut reconstructed = Vec::with_capacity(weight_v.elem_count());
    if scale.len() == out_channels {
        for (out, out_values) in raw.iter().take(out_channels).enumerate() {
            let norm = out_values
                .iter()
                .flat_map(|channel| channel.iter())
                .map(|value| value * value)
                .sum::<f32>()
                .sqrt()
                .max(1e-12);
            let multiplier = scale[out] / norm;
            for channel_values in out_values.iter().take(in_channels_per_group) {
                for value in channel_values.iter().take(kernel) {
                    reconstructed.push(value * multiplier);
                }
            }
        }
    } else {
        let mut norms = vec![0.0f32; kernel];
        for out_values in raw.iter().take(out_channels) {
            for channel_values in out_values.iter().take(in_channels_per_group) {
                for (position, value) in channel_values.iter().take(kernel).enumerate() {
                    norms[position] += value * value;
                }
            }
        }
        for norm in &mut norms {
            *norm = norm.sqrt().max(1e-12);
        }
        for out_values in raw.iter().take(out_channels) {
            for channel_values in out_values.iter().take(in_channels_per_group) {
                for (position, value) in channel_values.iter().take(kernel).enumerate() {
                    reconstructed.push(value * scale[position] / norms[position]);
                }
            }
        }
    }
    Tensor::new(reconstructed.as_slice(), weight_v.device())
        .and_then(|tensor| tensor.reshape((out_channels, in_channels_per_group, kernel)))
        .map_err(candle_mismatch)
}

fn linear(tensors: &HashMap<String, Tensor>, prefix: &str, bias: bool) -> Result<Linear> {
    let weight = required_tensor(tensors, &format!("{prefix}.weight"))?;
    let bias = if bias {
        optional_tensor(tensors, &format!("{prefix}.bias"))
    } else {
        None
    };
    Ok(Linear::new(weight, bias))
}

fn conv1d(
    tensors: &HashMap<String, Tensor>,
    prefix: &str,
    stride: usize,
    padding: usize,
    groups: usize,
    bias: bool,
) -> Result<Conv1d> {
    let weight = required_tensor(tensors, &format!("{prefix}.weight"))?;
    let bias = if bias {
        optional_tensor(tensors, &format!("{prefix}.bias"))
    } else {
        None
    };
    Ok(Conv1d::new(
        weight,
        bias,
        Conv1dConfig {
            stride,
            padding,
            groups,
            ..Default::default()
        },
    ))
}

fn layer_norm(
    tensors: &HashMap<String, Tensor>,
    prefix: &str,
    size: usize,
    eps: f64,
) -> Result<LayerNorm> {
    let weight = required_tensor(tensors, &format!("{prefix}.weight"))?;
    let bias = required_tensor(tensors, &format!("{prefix}.bias"))?;
    if weight.elem_count() != size || bias.elem_count() != size {
        return Err(model_output_mismatch(format!(
            "wav2vec2 layer norm `{prefix}` has incompatible shape"
        )));
    }
    Ok(LayerNorm::new(weight, bias, eps))
}

fn group_norm(
    tensors: &HashMap<String, Tensor>,
    prefix: &str,
    channels: usize,
    groups: usize,
    eps: f64,
) -> Result<GroupNorm> {
    let weight = required_tensor(tensors, &format!("{prefix}.weight"))?;
    let bias = required_tensor(tensors, &format!("{prefix}.bias"))?;
    GroupNorm::new(weight, bias, channels, groups, eps).map_err(candle_mismatch)
}

fn required_tensor(tensors: &HashMap<String, Tensor>, name: &str) -> Result<Tensor> {
    optional_tensor(tensors, name).ok_or_else(|| {
        unsupported_runtime(format!(
            "unsupported wav2vec2 safetensors layout; missing `{name}`"
        ))
    })
}

fn optional_tensor(tensors: &HashMap<String, Tensor>, name: &str) -> Option<Tensor> {
    tensors.get(name).map(|tensor| {
        if tensor.dtype() == DType::F32 {
            tensor.clone()
        } else {
            tensor
                .to_dtype(DType::F32)
                .unwrap_or_else(|_| tensor.clone())
        }
    })
}

fn tensor_exists(tensors: &HashMap<String, Tensor>, name: &str) -> bool {
    tensors.contains_key(name)
}

fn candle_mismatch(error: candle_core::Error) -> video_analysis_core::DetectError {
    model_output_mismatch(format!("wav2vec2 Candle execution failed: {error}"))
}
