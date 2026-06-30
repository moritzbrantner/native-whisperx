use std::path::{Path, PathBuf};

use video_analysis_core::Result;

use crate::native_audio::validate_loaded_audio;
use crate::{
    invalid_request, model_output_mismatch, setup_error, AlignedWord, AlignmentOptions,
    AlignmentRequest, AlignmentResponse, NoopTranscriptionPipelineObserver,
    TranscriptionPipelineEvent, TranscriptionPipelineObserver,
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct CtcVocabulary {
    pub blank_id: usize,
    pub tokens: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct CtcEmissionFrame {
    pub log_probs: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct CtcAlignedToken {
    pub token_id: usize,
    pub token_index: usize,
    pub frame_index: usize,
    pub score: f32,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct CtcAlignedWord {
    pub text: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub confidence: Option<f32>,
}

#[allow(dead_code)]
pub(crate) fn align(
    options: &AlignmentOptions,
    request: AlignmentRequest,
) -> Result<AlignmentResponse> {
    let mut observer = NoopTranscriptionPipelineObserver;
    align_with_observer(options, request, &mut observer)
}

pub(crate) fn align_with_observer(
    options: &AlignmentOptions,
    request: AlignmentRequest,
    observer: &mut dyn TranscriptionPipelineObserver,
) -> Result<AlignmentResponse> {
    validate_loaded_audio(&request.audio)?;
    validate_transcript_ranges(&request)?;
    let resolved = resolve_alignment_model(options, &request.model_id)?;
    let resolved_device = crate::native_device::resolve_native_device(options.device)?;
    let model_id = resolved.model_id.clone();
    let aligned = crate::native_wav2vec2::align_wav2vec2_ctc_with_load_observer(
        &resolved.bundle,
        &request,
        &resolved_device,
        options.interpolate_method,
        options.return_char_alignments,
        |event| match event {
            crate::native_wav2vec2::Wav2Vec2ModelLoadEvent::Start => {
                observer.observe(TranscriptionPipelineEvent::ModelLoadStart {
                    stage: "alignment".to_string(),
                    provider: "ctc-forced-aligner".to_string(),
                    model_id: model_id.clone(),
                });
            }
            crate::native_wav2vec2::Wav2Vec2ModelLoadEvent::End { duration_seconds } => {
                observer.observe(TranscriptionPipelineEvent::ModelLoadEnd {
                    stage: "alignment".to_string(),
                    provider: "ctc-forced-aligner".to_string(),
                    model_id: model_id.clone(),
                    duration_seconds,
                });
            }
        },
    )?;
    Ok(AlignmentResponse {
        model_id: resolved.model_id,
        words: aligned.words,
        chars: aligned.chars,
        diagnostics: vec![
            "alignment=wav2vec2Ctc".to_string(),
            "alignmentProvider=ctc-forced-aligner".to_string(),
            "alignmentModelExecution=candle-wav2vec2".to_string(),
            format!("alignmentModelResolved={}", resolved.bundle.display()),
            format!("alignmentModelSource={}", resolved.source),
            format!("alignmentDevice={}", resolved_device.diagnostic_name()),
            format!("alignmentCuda={}", resolved_device.cuda_active()),
            format!(
                "alignmentInterpolateMethod={}",
                options.interpolate_method.as_whisperx_arg()
            ),
            format!("returnCharAlignments={}", options.return_char_alignments),
        ],
    })
}

struct ResolvedAlignmentModel {
    model_id: String,
    bundle: PathBuf,
    source: &'static str,
}

fn resolve_alignment_model(
    options: &AlignmentOptions,
    requested_model_id: &str,
) -> Result<ResolvedAlignmentModel> {
    let model_id = canonical_alignment_model_id(requested_model_id)?;
    if let Some(bundle) = &options.model_bundle {
        crate::native_wav2vec2::resolve_wav2vec2_bundle_paths(bundle)?;
        return Ok(ResolvedAlignmentModel {
            model_id,
            bundle: bundle.clone(),
            source: "explicit-bundle",
        });
    }

    #[cfg(feature = "model-bundles")]
    {
        let required = required_alignment_files();
        if options.model_cache_only {
            let bundle = resolve_cached_alignment_model(&model_id, options.model_dir.as_deref())
                .ok_or_else(|| missing_alignment_model_error(&model_id, options, &required))?;
            crate::native_wav2vec2::resolve_wav2vec2_bundle_paths(&bundle)?;
            return Ok(ResolvedAlignmentModel {
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
            .download(&alignment_model_spec(&model_id))
            .map_err(|error| {
                missing_alignment_model_error_with_source(&model_id, options, &required, error)
            })?;
        let bundle = downloaded
            .model_dir()
            .map(Path::to_path_buf)
            .ok_or_else(|| {
                setup_error(format!(
                    "alignment model `{model_id}` resolved without a local model directory"
                ))
            })?;
        Ok(ResolvedAlignmentModel {
            model_id,
            bundle,
            source: "hugging-face-cache",
        })
    }

    #[cfg(not(feature = "model-bundles"))]
    {
        Err(setup_error(format!(
            "alignment model `{model_id}` requires a local --alignment-bundle or the model-bundles feature for Hugging Face resolution"
        )))
    }
}

fn canonical_alignment_model_id(value: &str) -> Result<String> {
    match value {
        "WAV2VEC2_ASR_BASE_960H" | "facebook/wav2vec2-base-960h" => {
            Ok("facebook/wav2vec2-base-960h".to_string())
        }
        other if looks_like_hf_repo_id(other) => Ok(other.to_string()),
        other => Err(crate::unsupported_runtime(format!(
            "unsupported alignment model alias `{other}`; pass a Hugging Face repo ID with a safetensors Wav2Vec2ForCTC layout or WAV2VEC2_ASR_BASE_960H"
        ))),
    }
}

fn looks_like_hf_repo_id(value: &str) -> bool {
    let mut parts = value.split('/');
    matches!((parts.next(), parts.next(), parts.next()), (Some(owner), Some(repo), None) if !owner.is_empty() && !repo.is_empty())
}

fn required_alignment_files() -> Vec<&'static str> {
    vec![
        "config.json",
        "preprocessor_config.json",
        "tokenizer.json or vocab.json",
        "model.safetensors",
    ]
}

fn missing_alignment_model_error(
    model_id: &str,
    options: &AlignmentOptions,
    required: &[&str],
) -> video_analysis_core::DetectError {
    setup_error(format!(
        "failed to resolve alignment model `{model_id}`; required files: {}; --model-dir={}; cache-only={}",
        required.join(", "),
        options
            .model_dir
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<default huggingface cache>".to_string()),
        options.model_cache_only
    ))
}

fn missing_alignment_model_error_with_source(
    model_id: &str,
    options: &AlignmentOptions,
    required: &[&str],
    source: impl std::fmt::Display,
) -> video_analysis_core::DetectError {
    setup_error(format!(
        "failed to resolve alignment model `{model_id}`; required files: {}; --model-dir={}; cache-only={}: {source}",
        required.join(", "),
        options
            .model_dir
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<default huggingface cache>".to_string()),
        options.model_cache_only
    ))
}

#[cfg(feature = "model-bundles")]
fn alignment_model_spec(model_id: &str) -> model_runtime::HuggingFaceModelSpec {
    let mut spec = model_runtime::HuggingFaceModelSpec::new(
        model_id.to_string(),
        model_runtime::ModelTask::SpeechRecognition,
    );
    spec.files = vec![
        model_runtime::ModelFileRequest::required("config.json"),
        model_runtime::ModelFileRequest::required("preprocessor_config.json"),
        model_runtime::ModelFileRequest::first_available(["tokenizer.json", "vocab.json"]),
        model_runtime::ModelFileRequest::required("model.safetensors"),
    ];
    spec
}

#[cfg(feature = "model-bundles")]
fn resolve_cached_alignment_model(model_id: &str, model_dir: Option<&Path>) -> Option<PathBuf> {
    let mut roots = Vec::new();
    if let Some(model_dir) = model_dir {
        roots.push(model_dir.to_path_buf());
    } else if let Some(home) = std::env::var_os("HF_HOME") {
        roots.push(PathBuf::from(home).join("hub"));
    } else if let Some(home) = std::env::var_os("HOME") {
        roots.push(PathBuf::from(home).join(".cache/huggingface/hub"));
    }
    for root in roots {
        for candidate in cache_candidates(&root, model_id) {
            if crate::native_wav2vec2::resolve_wav2vec2_bundle_paths(&candidate).is_ok() {
                return Some(candidate);
            }
        }
    }
    None
}

#[cfg(feature = "model-bundles")]
fn cache_candidates(root: &Path, model_id: &str) -> Vec<PathBuf> {
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

#[allow(dead_code)]
pub(crate) fn normalize_alignment_text(text: &str) -> Vec<String> {
    text.chars()
        .filter_map(|character| {
            if character.is_alphanumeric() {
                Some(character.to_lowercase().collect::<String>())
            } else {
                None
            }
        })
        .collect()
}

#[allow(dead_code)]
pub(crate) fn build_ctc_trellis(
    emissions: &[Vec<f32>],
    token_ids: &[usize],
    blank_id: usize,
) -> Result<Vec<Vec<f32>>> {
    validate_emissions(emissions, blank_id, token_ids)?;
    let frame_count = emissions.len();
    let token_count = token_ids.len();
    let mut trellis = vec![vec![f32::NEG_INFINITY; token_count + 1]; frame_count + 1];
    trellis[0][0] = 0.0;
    for frame in 0..frame_count {
        for token_index in 0..=token_count {
            let stay = trellis[frame][token_index] + emissions[frame][blank_id];
            let change = if token_index > 0 {
                trellis[frame][token_index - 1]
                    + emission_token_score(&emissions[frame], token_ids[token_index - 1], blank_id)
            } else {
                f32::NEG_INFINITY
            };
            trellis[frame + 1][token_index] = stay.max(change);
        }
    }
    Ok(trellis)
}

#[allow(dead_code)]
pub(crate) fn backtrack_ctc(
    trellis: &[Vec<f32>],
    emissions: &[Vec<f32>],
    token_ids: &[usize],
    blank_id: usize,
) -> Result<Vec<CtcAlignedToken>> {
    validate_emissions(emissions, blank_id, token_ids)?;
    if trellis.len() != emissions.len() + 1
        || trellis.iter().any(|row| row.len() != token_ids.len() + 1)
    {
        return Err(model_output_mismatch(
            "CTC trellis dimensions are inconsistent",
        ));
    }
    let mut frame = emissions.len();
    let mut token_index = token_ids.len();
    if !trellis[frame][token_index].is_finite() {
        return Err(model_output_mismatch("CTC path is impossible"));
    }
    let mut path = Vec::new();
    while token_index > 0 {
        if frame == 0 {
            return Err(model_output_mismatch("CTC backtracking exhausted frames"));
        }
        let current_frame = frame - 1;
        let token_id = token_ids[token_index - 1];
        let changed = trellis[current_frame][token_index - 1]
            + emission_token_score(&emissions[current_frame], token_id, blank_id);
        let stayed = trellis[current_frame][token_index] + emissions[current_frame][blank_id];
        if changed >= stayed {
            path.push(CtcAlignedToken {
                token_id,
                token_index: token_index - 1,
                frame_index: current_frame,
                score: emission_token_score(&emissions[current_frame], token_id, blank_id).exp(),
            });
            token_index -= 1;
        }
        frame -= 1;
    }
    path.reverse();
    Ok(path)
}

fn emission_token_score(frame: &[f32], token_id: usize, blank_id: usize) -> f32 {
    if token_id == usize::MAX {
        frame
            .iter()
            .enumerate()
            .filter(|(index, score)| *index != blank_id && score.is_finite())
            .map(|(_, score)| *score)
            .max_by(f32::total_cmp)
            .unwrap_or(f32::NEG_INFINITY)
    } else {
        frame[token_id]
    }
}

#[allow(dead_code)]
pub(crate) fn tokens_to_words(
    tokens: &[CtcAlignedToken],
    transcript_words: &[String],
    segment_start: f64,
    segment_end: f64,
    frame_seconds: f64,
) -> Result<Vec<AlignedWord>> {
    tokens_to_segment_words(
        0,
        tokens,
        transcript_words,
        segment_start,
        segment_end,
        frame_seconds,
    )
}

#[allow(dead_code)]
pub(crate) fn tokens_to_segment_words(
    segment_index: u64,
    tokens: &[CtcAlignedToken],
    transcript_words: &[String],
    segment_start: f64,
    segment_end: f64,
    frame_seconds: f64,
) -> Result<Vec<AlignedWord>> {
    if !segment_start.is_finite()
        || !segment_end.is_finite()
        || !frame_seconds.is_finite()
        || segment_end <= segment_start
        || frame_seconds <= 0.0
    {
        return Err(invalid_request("invalid CTC word timing range"));
    }
    let mut cursor = 0;
    let mut aligned = Vec::new();
    for (word_index, word) in transcript_words.iter().enumerate() {
        let token_len = ctc_word_token_len(word).max(1);
        let end_cursor = (cursor + token_len).min(tokens.len());
        let word_tokens = &tokens[cursor..end_cursor];
        let (start_seconds, end_seconds, confidence) =
            if let (Some(first), Some(last)) = (word_tokens.first(), word_tokens.last()) {
                let start = segment_start + first.frame_index as f64 * frame_seconds;
                let end = segment_start + (last.frame_index + 1) as f64 * frame_seconds;
                let confidence = word_tokens.iter().map(|token| token.score).sum::<f32>()
                    / word_tokens.len() as f32;
                (
                    start.clamp(segment_start, segment_end),
                    end.clamp(segment_start, segment_end),
                    Some(confidence),
                )
            } else {
                let width = (segment_end - segment_start) / transcript_words.len().max(1) as f64;
                let start = segment_start + word_index as f64 * width;
                (start, (start + width).min(segment_end), None)
            };
        aligned.push(AlignedWord {
            segment_index,
            word_index,
            text: word.clone(),
            start_seconds,
            end_seconds: end_seconds.max(start_seconds),
            confidence,
        });
        cursor = end_cursor;
        if word_index + 1 < transcript_words.len() && cursor < tokens.len() {
            cursor += 1;
        }
    }
    extend_word_ends_to_delimiter_midpoints(&mut aligned);
    Ok(aligned)
}

fn ctc_word_token_len(word: &str) -> usize {
    word.chars()
        .filter(|character| !character.is_whitespace())
        .count()
}

fn extend_word_ends_to_delimiter_midpoints(words: &mut [AlignedWord]) {
    if words.len() < 2 {
        return;
    }
    for index in 0..words.len() - 1 {
        if ctc_word_token_len(&words[index].text) <= 1 {
            continue;
        }
        let next_start = words[index + 1].start_seconds;
        let end = words[index].end_seconds;
        if next_start > end {
            words[index].end_seconds = end + (next_start - end) * 0.5;
        }
    }
}

#[allow(dead_code)]
fn deterministic_words_from_transcript(request: &AlignmentRequest) -> Result<Vec<AlignedWord>> {
    let mut aligned = Vec::new();
    let duration = request.audio.duration_seconds();
    for segment in &request.transcript.segments {
        let segment_start = segment.start_seconds.unwrap_or(0.0);
        let segment_end = segment.end_seconds.unwrap_or(duration);
        if segment_end < segment_start {
            return Err(invalid_request("transcript segment has invalid timing"));
        }
        let words = if segment.words.is_empty() {
            segment
                .text
                .split_whitespace()
                .map(|word| word.trim().to_string())
                .filter(|word| !word.is_empty())
                .collect::<Vec<_>>()
        } else {
            segment
                .words
                .iter()
                .map(|word| word.text.trim().to_string())
                .collect::<Vec<_>>()
        };
        if words.is_empty() {
            continue;
        }
        let width = (segment_end - segment_start) / words.len() as f64;
        for (word_index, word) in words.iter().enumerate() {
            let contract_word = segment.words.get(word_index);
            let start_seconds = contract_word
                .and_then(|word| word.start_seconds)
                .unwrap_or(segment_start + word_index as f64 * width)
                .clamp(segment_start, segment_end);
            let end_seconds = contract_word
                .and_then(|word| word.end_seconds)
                .unwrap_or((start_seconds + width).min(segment_end))
                .clamp(segment_start, segment_end)
                .max(start_seconds);
            aligned.push(AlignedWord {
                segment_index: segment.index,
                word_index,
                text: word.clone(),
                start_seconds,
                end_seconds,
                confidence: contract_word.and_then(|word| word.confidence).or(Some(1.0)),
            });
        }
    }
    Ok(aligned)
}

fn validate_transcript_ranges(request: &AlignmentRequest) -> Result<()> {
    let duration = request.audio.duration_seconds();
    for segment in &request.transcript.segments {
        if let (Some(start), Some(end)) = (segment.start_seconds, segment.end_seconds) {
            if !start.is_finite() || !end.is_finite() || end < start || end > duration + 1e-6 {
                return Err(invalid_request(
                    "transcript segment timing is outside audio range",
                ));
            }
        }
        for word in &segment.words {
            if let (Some(start), Some(end)) = (word.start_seconds, word.end_seconds) {
                if !start.is_finite() || !end.is_finite() || end < start || end > duration + 1e-6 {
                    return Err(invalid_request(
                        "transcript word timing is outside audio range",
                    ));
                }
            }
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn validate_emissions(emissions: &[Vec<f32>], blank_id: usize, token_ids: &[usize]) -> Result<()> {
    if emissions.is_empty() {
        return Err(model_output_mismatch("CTC emissions are empty"));
    }
    let Some(vocab_size) = emissions.first().map(Vec::len) else {
        return Err(model_output_mismatch("CTC emissions are empty"));
    };
    if vocab_size == 0
        || blank_id >= vocab_size
        || token_ids
            .iter()
            .any(|token| *token != usize::MAX && *token >= vocab_size)
        || emissions
            .iter()
            .any(|frame| frame.len() != vocab_size || frame.iter().any(|score| !score.is_finite()))
    {
        return Err(model_output_mismatch(
            "CTC emission dimensions are inconsistent",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use text_transcripts::{TranscriptSegmentContract, TranscriptionContract};

    #[test]
    fn ctc_trellis_aligns_known_token_sequence() {
        let emissions = vec![
            vec![-0.1, -5.0, -5.0],
            vec![-5.0, -0.1, -5.0],
            vec![-0.1, -5.0, -5.0],
            vec![-5.0, -5.0, -0.1],
        ];
        let token_ids = vec![1, 2];
        let trellis = build_ctc_trellis(&emissions, &token_ids, 0).unwrap();
        let path = backtrack_ctc(&trellis, &emissions, &token_ids, 0).unwrap();
        assert_eq!(path.len(), 2);
        assert_eq!(path[0].token_id, 1);
        assert_eq!(path[1].token_id, 2);
    }

    #[test]
    fn ctc_backtracking_rejects_impossible_path() {
        let emissions = vec![vec![-0.1, -5.0], vec![-0.1, -5.0]];
        let token_ids = vec![1, 1, 1];
        let trellis = build_ctc_trellis(&emissions, &token_ids, 0).unwrap();
        let error = backtrack_ctc(&trellis, &emissions, &token_ids, 0)
            .unwrap_err()
            .to_string();
        assert!(error.contains("model_output_mismatch"));
    }

    #[test]
    fn ctc_words_stay_inside_segment_ranges() {
        let tokens = vec![
            CtcAlignedToken {
                token_id: 1,
                token_index: 0,
                frame_index: 0,
                score: 0.9,
            },
            CtcAlignedToken {
                token_id: 2,
                token_index: 1,
                frame_index: 5,
                score: 0.8,
            },
        ];
        let words = tokens_to_words(&tokens, &["hi".to_string()], 1.0, 1.2, 0.1).unwrap();
        assert_eq!(words[0].start_seconds, 1.0);
        assert_eq!(words[0].end_seconds, 1.2);
    }

    #[test]
    fn ctc_word_conversion_preserves_segment_index() {
        let tokens = vec![CtcAlignedToken {
            token_id: 1,
            token_index: 0,
            frame_index: 2,
            score: 0.9,
        }];
        let words =
            tokens_to_segment_words(42, &tokens, &["a".to_string()], 1.0, 2.0, 0.1).unwrap();
        assert_eq!(words[0].segment_index, 42);
        assert_eq!(words[0].word_index, 0);
    }

    #[test]
    fn ctc_word_conversion_skips_word_delimiter_tokens() {
        let tokens = (0..8)
            .map(|index| CtcAlignedToken {
                token_id: index,
                token_index: index,
                frame_index: index,
                score: 0.9,
            })
            .collect::<Vec<_>>();
        let words = tokens_to_segment_words(
            0,
            &tokens,
            &["This".to_string(), "is".to_string()],
            0.0,
            1.0,
            0.1,
        )
        .unwrap();

        assert_eq!(words[0].start_seconds, 0.0);
        assert_eq!(words[0].end_seconds, 0.45);
        assert_eq!(words[1].start_seconds, 0.5);
        assert!((words[1].end_seconds - 0.7).abs() < 1e-9);
    }

    #[test]
    fn ctc_word_conversion_includes_punctuation_in_word_span() {
        let tokens = (0..5)
            .map(|index| CtcAlignedToken {
                token_id: index,
                token_index: index,
                frame_index: index + 10,
                score: 0.9,
            })
            .collect::<Vec<_>>();
        let words =
            tokens_to_segment_words(0, &tokens, &["test.".to_string()], 1.0, 3.0, 0.05).unwrap();

        assert_eq!(words[0].text, "test.");
        assert_eq!(words[0].start_seconds, 1.5);
        assert_eq!(words[0].end_seconds, 1.75);
    }

    #[test]
    fn alignment_without_bundle_or_cache_returns_setup_error() {
        let mut segment = TranscriptSegmentContract::new(0, "hello world");
        segment.start_seconds = Some(0.0);
        segment.end_seconds = Some(1.0);
        let transcript =
            TranscriptionContract::from_segments(None, Some("en".to_string()), vec![segment])
                .unwrap();
        let response = align(
            &AlignmentOptions::default(),
            AlignmentRequest {
                audio: crate::LoadedAudio {
                    samples: vec![0.0; 16_000],
                    sample_rate: 16_000,
                    channels: 1,
                    source: None,
                },
                transcript,
                language: Some("en".to_string()),
                model_id: "deterministic".to_string(),
            },
        )
        .unwrap_err()
        .to_string();
        assert!(response.contains("setup_error") || response.contains("unsupported_runtime"));
        assert!(!response.contains("deterministic transcript timing alignment completed"));
    }

    fn write_valid_wav2vec2_bundle(root: &Path) {
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
                "conv_bias": false,
                "num_conv_pos_embeddings": 0,
                "num_conv_pos_embedding_groups": 1
            })
            .to_string(),
        )
        .unwrap();
        std::fs::write(
            root.join("tokenizer.json"),
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
            .to_string(),
        )
        .unwrap();
        std::fs::write(root.join("preprocessor_config.json"), "{}").unwrap();
        std::fs::write(root.join("model.safetensors"), "").unwrap();
    }

    fn alignment_request_for_tests() -> AlignmentRequest {
        let mut segment = TranscriptSegmentContract::new(0, "hello world");
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
    fn alignment_with_wav2vec2_bundle_reports_unsupported_layout() {
        let temp = tempfile::tempdir().unwrap();
        write_valid_wav2vec2_bundle(temp.path());
        let error = align(
            &AlignmentOptions {
                enabled: true,
                model_bundle: Some(temp.path().to_path_buf()),
                ..AlignmentOptions::default()
            },
            alignment_request_for_tests(),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("unsupported_runtime"));
        assert!(error.contains("safetensors"));
    }

    #[test]
    fn wav2vec2_model_error_does_not_fall_back_to_deterministic_alignment() {
        let temp = tempfile::tempdir().unwrap();
        write_valid_wav2vec2_bundle(temp.path());

        let error = align(
            &AlignmentOptions {
                enabled: true,
                model_bundle: Some(temp.path().to_path_buf()),
                ..AlignmentOptions::default()
            },
            alignment_request_for_tests(),
        )
        .unwrap_err()
        .to_string();

        assert!(error.contains("safetensors"));
        assert!(!error.contains("deterministic transcript timing alignment completed"));
    }

    #[test]
    fn missing_alignment_bundle_files_return_setup_error() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(temp.path().join("config.json"), "{}").unwrap();
        let error = align(
            &AlignmentOptions {
                enabled: true,
                model_bundle: Some(temp.path().to_path_buf()),
                ..AlignmentOptions::default()
            },
            alignment_request_for_tests(),
        )
        .unwrap_err()
        .to_string();
        assert!(error.contains("setup_error"));
        assert!(error.contains("vocab.json"));
    }
}
