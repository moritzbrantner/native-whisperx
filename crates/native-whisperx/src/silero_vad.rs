#![cfg_attr(
    not(any(feature = "silero-vad", feature = "pyannote-vad", test)),
    allow(dead_code)
)]

#[cfg(feature = "pyannote-vad")]
use std::collections::BTreeMap;
#[cfg(feature = "pyannote-vad")]
use std::fs;
#[cfg(feature = "pyannote-vad")]
use std::path::Path;
use std::path::PathBuf;

use audio_analysis_transcription::{
    SpeechActivitySegment, TranscriptionVadProvider, VadRequest, VadResponse,
};
#[cfg(any(feature = "silero-vad", feature = "pyannote-vad"))]
use runtime_onnx::OnnxRunner;
#[cfg(feature = "pyannote-vad")]
use serde::Deserialize;
use video_analysis_core::{DetectError, Result};

const SILERO_SAMPLE_RATE: u32 = 16_000;
const SILERO_WINDOW_SAMPLES: usize = 512;
const PYANNOTE_SAMPLE_RATE: u32 = 16_000;
#[cfg(feature = "pyannote-vad")]
const PYANNOTE_DEFAULT_WINDOW_SECONDS: f64 = 10.0;
#[cfg(feature = "pyannote-vad")]
const PYANNOTE_DEFAULT_STEP_RATIO: f64 = 0.1;
#[cfg_attr(not(feature = "silero-vad"), allow(dead_code))]
const SILERO_CONTEXT_SAMPLES: usize = 64;
#[cfg_attr(not(feature = "silero-vad"), allow(dead_code))]
const SILERO_STATE_VALUES: usize = 2 * 128;

#[cfg_attr(not(feature = "silero-vad"), allow(dead_code))]
pub(crate) struct SileroVadOptions {
    pub model_path: PathBuf,
    pub input_name: Option<String>,
    pub output_name: Option<String>,
    pub threshold: f32,
    pub max_speech_duration_seconds: f64,
    pub min_speech_duration_ms: usize,
    pub min_silence_duration_ms: usize,
    pub speech_pad_ms: usize,
}

#[cfg_attr(not(feature = "silero-vad"), allow(dead_code))]
impl SileroVadOptions {
    pub(crate) fn detector(&self) -> SileroTimestampDetector {
        SileroTimestampDetector {
            threshold: self.threshold,
            max_speech_duration_seconds: self.max_speech_duration_seconds,
            min_speech_duration_ms: self.min_speech_duration_ms,
            min_silence_duration_ms: self.min_silence_duration_ms,
            speech_pad_ms: self.speech_pad_ms,
        }
    }
}

pub(crate) trait SileroProbabilityRunner {
    fn speech_probabilities(&mut self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>>;
}

#[cfg_attr(not(feature = "pyannote-vad"), allow(dead_code))]
pub(crate) struct PyannoteVadOptions {
    pub model_path: PathBuf,
    pub input_name: Option<String>,
    pub output_name: Option<String>,
    pub onset: f32,
    pub offset: f32,
    pub chunk_size: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PyannoteVadFrame {
    start_seconds: f64,
    end_seconds: f64,
    score: f32,
}

pub(crate) struct PyannoteFrameBatch {
    frames: Vec<PyannoteVadFrame>,
    windows: usize,
}

pub(crate) trait PyannoteFrameRunner {
    fn speech_frames(&mut self, samples: &[f32], sample_rate: u32) -> Result<PyannoteFrameBatch>;
}

pub(crate) struct SileroVadTranscriptionProvider {
    detector: SileroTimestampDetector,
    runner: Box<dyn SileroProbabilityRunner + Send>,
    diagnostics: Vec<String>,
}

impl SileroVadTranscriptionProvider {
    pub(crate) fn new_for_runner(
        detector: SileroTimestampDetector,
        runner: Box<dyn SileroProbabilityRunner + Send>,
        diagnostics: Vec<String>,
    ) -> Self {
        Self {
            detector,
            runner,
            diagnostics,
        }
    }

    #[cfg(feature = "silero-vad")]
    pub(crate) fn from_options(
        options: SileroVadOptions,
        diagnostics: Vec<String>,
    ) -> Result<Self> {
        let detector = options.detector();
        let runner = OnnxSileroRunner::from_options(options)?;
        Ok(Self::new_for_runner(
            detector,
            Box::new(runner),
            diagnostics,
        ))
    }
}

pub(crate) struct PyannoteVadTranscriptionProvider {
    onset: f32,
    offset: f32,
    chunk_size: f64,
    runner: Box<dyn PyannoteFrameRunner + Send>,
    diagnostics: Vec<String>,
}

impl PyannoteVadTranscriptionProvider {
    pub(crate) fn new_for_runner(
        onset: f32,
        offset: f32,
        chunk_size: f64,
        runner: Box<dyn PyannoteFrameRunner + Send>,
        diagnostics: Vec<String>,
    ) -> Self {
        Self {
            onset,
            offset,
            chunk_size,
            runner,
            diagnostics,
        }
    }

    #[cfg(feature = "pyannote-vad")]
    pub(crate) fn from_options(
        options: PyannoteVadOptions,
        diagnostics: Vec<String>,
    ) -> Result<Self> {
        let onset = options.onset;
        let offset = options.offset;
        let chunk_size = options.chunk_size;
        let runner = OnnxPyannoteRunner::from_options(options)?;
        Ok(Self::new_for_runner(
            onset,
            offset,
            chunk_size,
            Box::new(runner),
            diagnostics,
        ))
    }
}

impl TranscriptionVadProvider for PyannoteVadTranscriptionProvider {
    fn provider_id(&self) -> &str {
        "pyannote-vad"
    }

    fn detect_speech(&mut self, request: VadRequest) -> Result<VadResponse> {
        if request.audio.sample_rate != PYANNOTE_SAMPLE_RATE || request.audio.channels != 1 {
            return Err(DetectError::InvalidArgument(format!(
                "pyannote VAD requires 16000 Hz mono audio, got sample_rate={} channels={}",
                request.audio.sample_rate, request.audio.channels
            )));
        }
        let batch = self
            .runner
            .speech_frames(&request.audio.samples, request.audio.sample_rate)?;
        let raw_segments = pyannote_frames_to_segments(&batch.frames, self.onset, self.offset)?;
        let segments = merge_whisperx_vad_chunks(raw_segments, self.chunk_size)?;
        let mut diagnostics = vec![
            format!("pyannoteVadWindows={}", batch.windows),
            format!("pyannoteVadFrames={}", batch.frames.len()),
            "native pyannote VAD completed".to_string(),
        ];
        diagnostics.extend(self.diagnostics.clone());
        Ok(VadResponse {
            segments,
            diagnostics,
        })
    }
}

impl TranscriptionVadProvider for SileroVadTranscriptionProvider {
    fn provider_id(&self) -> &str {
        "silero-vad"
    }

    fn detect_speech(&mut self, request: VadRequest) -> Result<VadResponse> {
        if request.audio.sample_rate != SILERO_SAMPLE_RATE || request.audio.channels != 1 {
            return Err(DetectError::InvalidArgument(format!(
                "silero VAD requires 16000 Hz mono audio, got sample_rate={} channels={}",
                request.audio.sample_rate, request.audio.channels
            )));
        }
        let probabilities = self
            .runner
            .speech_probabilities(&request.audio.samples, request.audio.sample_rate)?;
        let raw_segments = self
            .detector
            .detect_from_probabilities(&probabilities, request.audio.samples.len())?;
        let segments = merge_whisperx_vad_chunks(
            raw_segments,
            request.options.max_chunk_seconds.max(f64::EPSILON),
        )?;
        let mut diagnostics = vec![
            format!("sileroVadProbabilityWindows={}", probabilities.len()),
            "native Silero VAD completed".to_string(),
        ];
        diagnostics.extend(self.diagnostics.clone());
        Ok(VadResponse {
            segments,
            diagnostics,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SileroTimestampDetector {
    pub threshold: f32,
    pub max_speech_duration_seconds: f64,
    pub min_speech_duration_ms: usize,
    pub min_silence_duration_ms: usize,
    pub speech_pad_ms: usize,
}

impl SileroTimestampDetector {
    pub(crate) fn detect_from_probabilities(
        &self,
        probabilities: &[f32],
        audio_len_samples: usize,
    ) -> Result<Vec<SpeechActivitySegment>> {
        if probabilities
            .iter()
            .any(|probability| !probability.is_finite())
        {
            return Err(DetectError::InvalidArgument(
                "silero VAD probabilities must be finite".to_string(),
            ));
        }
        if probabilities.is_empty() || audio_len_samples == 0 {
            return Ok(Vec::new());
        }

        let threshold = self.threshold;
        let neg_threshold = (threshold - 0.15).max(0.01);
        let min_speech_samples =
            SILERO_SAMPLE_RATE as f64 * self.min_speech_duration_ms as f64 / 1000.0;
        let speech_pad_samples =
            (SILERO_SAMPLE_RATE as f64 * self.speech_pad_ms as f64 / 1000.0) as usize;
        let min_silence_samples =
            SILERO_SAMPLE_RATE as f64 * self.min_silence_duration_ms as f64 / 1000.0;
        let min_silence_at_max_speech = SILERO_SAMPLE_RATE as f64 * 98.0 / 1000.0;
        let max_speech_samples = (SILERO_SAMPLE_RATE as f64 * self.max_speech_duration_seconds
            - SILERO_WINDOW_SAMPLES as f64
            - 2.0 * speech_pad_samples as f64)
            .max(0.0);

        let mut triggered = false;
        let mut current_speech = RawSpeech::default();
        let mut speeches = Vec::<RawSpeech>::new();
        let mut temp_end = 0usize;
        let mut prev_end = 0usize;
        let mut next_start = 0usize;
        let mut possible_ends = Vec::<(usize, usize)>::new();

        for (index, speech_prob) in probabilities.iter().copied().enumerate() {
            let cur_sample = SILERO_WINDOW_SAMPLES * index;

            if speech_prob >= threshold && temp_end != 0 {
                let silence_duration = cur_sample.saturating_sub(temp_end);
                if silence_duration as f64 > min_silence_at_max_speech {
                    possible_ends.push((temp_end, silence_duration));
                }
                temp_end = 0;
                if next_start < prev_end {
                    next_start = cur_sample;
                }
            }

            if speech_prob >= threshold && !triggered {
                triggered = true;
                current_speech = RawSpeech {
                    start: cur_sample,
                    end: 0,
                };
                continue;
            }

            if triggered
                && (cur_sample.saturating_sub(current_speech.start)) as f64 > max_speech_samples
            {
                if let Some((best_end, duration)) = possible_ends
                    .iter()
                    .copied()
                    .max_by_key(|(_, duration)| *duration)
                {
                    current_speech.end = best_end;
                    speeches.push(current_speech);
                    current_speech = RawSpeech::default();
                    next_start = best_end + duration;
                    if next_start < best_end + cur_sample {
                        current_speech.start = next_start;
                    } else {
                        triggered = false;
                    }
                    prev_end = 0;
                    next_start = 0;
                    temp_end = 0;
                    possible_ends.clear();
                } else {
                    current_speech.end = cur_sample;
                    speeches.push(current_speech);
                    current_speech = RawSpeech::default();
                    prev_end = 0;
                    next_start = 0;
                    temp_end = 0;
                    triggered = false;
                    possible_ends.clear();
                }
                continue;
            }

            if speech_prob < neg_threshold && triggered {
                if temp_end == 0 {
                    temp_end = cur_sample;
                }
                let silence_duration = cur_sample.saturating_sub(temp_end);
                if (silence_duration as f64) < min_silence_samples {
                    continue;
                }
                current_speech.end = temp_end;
                if (current_speech.end.saturating_sub(current_speech.start)) as f64
                    > min_speech_samples
                {
                    speeches.push(current_speech);
                }
                current_speech = RawSpeech::default();
                prev_end = 0;
                next_start = 0;
                temp_end = 0;
                triggered = false;
                possible_ends.clear();
            }
        }

        if triggered
            && current_speech.start < audio_len_samples
            && audio_len_samples.saturating_sub(current_speech.start) as f64 > min_speech_samples
        {
            current_speech.end = audio_len_samples;
            speeches.push(current_speech);
        }

        apply_speech_padding(&mut speeches, speech_pad_samples, audio_len_samples);
        speeches
            .into_iter()
            .filter(|speech| speech.end > speech.start)
            .map(|speech| {
                let score = max_probability_for_span(probabilities, speech.start, speech.end);
                SpeechActivitySegment::new(
                    speech.start as f64 / SILERO_SAMPLE_RATE as f64,
                    speech.end as f64 / SILERO_SAMPLE_RATE as f64,
                    score,
                )
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct RawSpeech {
    start: usize,
    end: usize,
}

fn apply_speech_padding(
    speeches: &mut [RawSpeech],
    speech_pad_samples: usize,
    audio_len_samples: usize,
) {
    for index in 0..speeches.len() {
        if index == 0 {
            speeches[index].start = speeches[index].start.saturating_sub(speech_pad_samples);
        }
        if index + 1 < speeches.len() {
            let silence_duration = speeches[index + 1]
                .start
                .saturating_sub(speeches[index].end);
            if silence_duration < 2 * speech_pad_samples {
                let half = silence_duration / 2;
                speeches[index].end += half;
                speeches[index + 1].start = speeches[index + 1]
                    .start
                    .saturating_sub(silence_duration - half);
            } else {
                speeches[index].end =
                    (speeches[index].end + speech_pad_samples).min(audio_len_samples);
                speeches[index + 1].start =
                    speeches[index + 1].start.saturating_sub(speech_pad_samples);
            }
        } else {
            speeches[index].end = (speeches[index].end + speech_pad_samples).min(audio_len_samples);
        }
    }
}

fn max_probability_for_span(probabilities: &[f32], start: usize, end: usize) -> f32 {
    probabilities
        .iter()
        .enumerate()
        .filter_map(|(index, probability)| {
            let window_start = index * SILERO_WINDOW_SAMPLES;
            let window_end = window_start + SILERO_WINDOW_SAMPLES;
            (window_end > start && window_start < end).then_some(*probability)
        })
        .fold(0.0, f32::max)
}

pub(crate) fn merge_whisperx_vad_chunks(
    segments: Vec<SpeechActivitySegment>,
    chunk_size: f64,
) -> Result<Vec<SpeechActivitySegment>> {
    let Some(first) = segments.first() else {
        return Ok(Vec::new());
    };
    let mut merged = Vec::new();
    let mut curr_start = first.start_seconds;
    let mut curr_end = 0.0;
    let mut curr_score = first.score;

    for segment in segments {
        if segment.end_seconds - curr_start > chunk_size && curr_end - curr_start > 0.0 {
            merged.push(SpeechActivitySegment::new(
                curr_start, curr_end, curr_score,
            )?);
            curr_start = segment.start_seconds;
            curr_score = segment.score;
        } else {
            curr_score = curr_score.max(segment.score);
        }
        curr_end = segment.end_seconds;
    }

    merged.push(SpeechActivitySegment::new(
        curr_start, curr_end, curr_score,
    )?);
    Ok(merged)
}

fn pyannote_frames_to_segments(
    frames: &[PyannoteVadFrame],
    onset: f32,
    offset: f32,
) -> Result<Vec<SpeechActivitySegment>> {
    if frames.iter().any(|frame| {
        !frame.start_seconds.is_finite()
            || !frame.end_seconds.is_finite()
            || !frame.score.is_finite()
            || frame.end_seconds <= frame.start_seconds
    }) {
        return Err(DetectError::InvalidArgument(
            "pyannote VAD frames must be finite and have positive duration".to_string(),
        ));
    }

    let mut sorted = frames.to_vec();
    sorted.sort_by(|left, right| {
        left.start_seconds
            .partial_cmp(&right.start_seconds)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                left.end_seconds
                    .partial_cmp(&right.end_seconds)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });

    let mut segments = Vec::new();
    let mut active: Option<(f64, f64, f32)> = None;
    for frame in sorted {
        if active.is_none() {
            if frame.score >= onset {
                active = Some((frame.start_seconds, frame.end_seconds, frame.score));
            }
            continue;
        }

        let (start, end, score) = active.as_mut().expect("active segment");
        if frame.score < offset {
            segments.push(SpeechActivitySegment::new(*start, *end, *score)?);
            active = None;
        } else {
            *end = (*end).max(frame.end_seconds);
            *score = (*score).max(frame.score);
        }
    }

    if let Some((start, end, score)) = active {
        segments.push(SpeechActivitySegment::new(start, end, score)?);
    }
    Ok(segments)
}

#[cfg(feature = "silero-vad")]
struct OnnxSileroRunner {
    session: runtime_onnx::OnnxSession,
    input_name: String,
    output_name: Option<String>,
}

#[cfg(feature = "silero-vad")]
impl OnnxSileroRunner {
    fn from_options(options: SileroVadOptions) -> Result<Self> {
        let session = runtime_onnx::from_file_cpu_single_threaded(&options.model_path)
            .map_err(silero_onnx_source)?;
        let metadata = session.metadata().map_err(silero_onnx_source)?;
        validate_onnx_metadata(
            &metadata,
            options.input_name.as_deref().unwrap_or("input"),
            options.output_name.as_deref(),
        )?;
        Ok(Self {
            session,
            input_name: options.input_name.unwrap_or_else(|| "input".to_string()),
            output_name: options.output_name,
        })
    }
}

#[cfg(feature = "silero-vad")]
impl SileroProbabilityRunner for OnnxSileroRunner {
    fn speech_probabilities(&mut self, samples: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        if sample_rate != SILERO_SAMPLE_RATE {
            return Err(DetectError::InvalidArgument(format!(
                "silero VAD requires 16000 Hz audio, got {sample_rate}"
            )));
        }

        let mut probabilities = Vec::new();
        let mut context = vec![0.0_f32; SILERO_CONTEXT_SAMPLES];
        let mut state = vec![0.0_f32; SILERO_STATE_VALUES];

        for chunk in samples.chunks(SILERO_WINDOW_SAMPLES) {
            let mut padded = vec![0.0_f32; SILERO_WINDOW_SAMPLES];
            padded[..chunk.len()].copy_from_slice(chunk);
            let mut input = Vec::with_capacity(SILERO_CONTEXT_SAMPLES + SILERO_WINDOW_SAMPLES);
            input.extend_from_slice(&context);
            input.extend_from_slice(&padded);

            let outputs = self
                .session
                .run(vec![
                    runtime_onnx::single_f32_input(
                        self.input_name.clone(),
                        vec![1, SILERO_CONTEXT_SAMPLES + SILERO_WINDOW_SAMPLES],
                        input.clone(),
                    )
                    .map_err(silero_onnx_invalid)?,
                    runtime_onnx::single_f32_input("state", vec![2, 1, 128], state)
                        .map_err(silero_onnx_invalid)?,
                    runtime_onnx::single_i64_input("sr", vec![1], vec![SILERO_SAMPLE_RATE as i64])
                        .map_err(silero_onnx_invalid)?,
                ])
                .map_err(silero_onnx_source)?;

            let probability = if let Some(output_name) = self.output_name.as_deref() {
                runtime_onnx::f32_output_by_name_or_index(&outputs, output_name, 0)
                    .map_err(silero_onnx_invalid)?
            } else {
                runtime_onnx::first_f32_output(&outputs).map_err(silero_onnx_invalid)?
            };
            let probability = probability.values.first().copied().ok_or_else(|| {
                DetectError::InvalidArgument("silero ONNX probability output was empty".to_string())
            })?;

            let next_state = runtime_onnx::f32_output_by_name_or_index(&outputs, "", 1)
                .map_err(silero_onnx_invalid)?;
            if next_state.values.len() != SILERO_STATE_VALUES {
                return Err(DetectError::InvalidArgument(format!(
                    "silero ONNX state output must contain {SILERO_STATE_VALUES} f32 values, got {}",
                    next_state.values.len()
                )));
            }
            state = next_state.values.clone();
            context = input[input.len() - SILERO_CONTEXT_SAMPLES..].to_vec();
            probabilities.push(probability);
        }

        Ok(probabilities)
    }
}

#[cfg(feature = "pyannote-vad")]
struct OnnxPyannoteRunner {
    session: runtime_onnx::OnnxSession,
    model: PyannoteVadModelShape,
}

#[cfg(feature = "pyannote-vad")]
#[derive(Debug, Clone)]
struct PyannoteVadModelShape {
    input_name: String,
    output_name: Option<String>,
    input_shape: Vec<usize>,
    window_samples: usize,
    window_seconds: f64,
    step_samples: usize,
    frames: Option<usize>,
    speakers: Option<usize>,
}

#[cfg(feature = "pyannote-vad")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PyannoteVadManifest {
    #[serde(default)]
    sample_rate: Option<u32>,
    #[serde(default)]
    segmentation: Option<PyannoteVadSegmentationManifest>,
}

#[cfg(feature = "pyannote-vad")]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PyannoteVadSegmentationManifest {
    #[serde(default)]
    input_name: Option<String>,
    #[serde(default)]
    output_name: Option<String>,
    duration_seconds: f64,
    #[serde(default)]
    step_ratio: Option<f64>,
    #[serde(default)]
    frames: Option<usize>,
    #[serde(default)]
    local_speakers: Option<usize>,
}

#[cfg(feature = "pyannote-vad")]
impl OnnxPyannoteRunner {
    fn from_options(options: PyannoteVadOptions) -> Result<Self> {
        let session = runtime_onnx::from_file_cpu_single_threaded(&options.model_path)
            .map_err(pyannote_onnx_source)?;
        let metadata = session.metadata().map_err(pyannote_onnx_source)?;
        let manifest = load_pyannote_manifest(&options.model_path)?;
        let model = pyannote_model_shape(&metadata, &options, manifest.as_ref())?;
        Ok(Self { session, model })
    }
}

#[cfg(feature = "pyannote-vad")]
impl PyannoteFrameRunner for OnnxPyannoteRunner {
    fn speech_frames(&mut self, samples: &[f32], sample_rate: u32) -> Result<PyannoteFrameBatch> {
        if sample_rate != PYANNOTE_SAMPLE_RATE {
            return Err(DetectError::InvalidArgument(format!(
                "pyannote VAD requires 16000 Hz audio, got {sample_rate}"
            )));
        }

        let mut by_start = BTreeMap::<i64, PyannoteVadFrame>::new();
        let mut windows = 0usize;
        let mut start = 0usize;
        loop {
            let mut values = vec![0.0_f32; self.model.window_samples];
            let end = (start + self.model.window_samples).min(samples.len());
            if start < samples.len() {
                values[..end - start].copy_from_slice(&samples[start..end]);
            }
            let outputs = self
                .session
                .run(vec![runtime_onnx::single_f32_input(
                    self.model.input_name.clone(),
                    self.model.input_shape.clone(),
                    values,
                )
                .map_err(pyannote_onnx_invalid)?])
                .map_err(pyannote_onnx_source)?;
            let output = if let Some(output_name) = self.model.output_name.as_deref() {
                runtime_onnx::f32_output_by_name_or_index(&outputs, output_name, 0)
                    .map_err(pyannote_onnx_invalid)?
            } else {
                runtime_onnx::first_f32_output(&outputs).map_err(pyannote_onnx_invalid)?
            };
            let window_start_seconds = start as f64 / PYANNOTE_SAMPLE_RATE as f64;
            for frame in pyannote_output_frames(output, &self.model, window_start_seconds)? {
                let key = (frame.start_seconds * 1_000_000.0).round() as i64;
                by_start
                    .entry(key)
                    .and_modify(|existing| {
                        existing.end_seconds = existing.end_seconds.max(frame.end_seconds);
                        existing.score = existing.score.max(frame.score);
                    })
                    .or_insert(frame);
            }
            windows += 1;
            if end >= samples.len() {
                break;
            }
            start += self.model.step_samples;
        }

        Ok(PyannoteFrameBatch {
            frames: by_start.into_values().collect(),
            windows,
        })
    }
}

#[cfg(feature = "pyannote-vad")]
fn load_pyannote_manifest(model_path: &Path) -> Result<Option<PyannoteVadManifest>> {
    let Some(parent) = model_path.parent() else {
        return Ok(None);
    };
    for file_name in [
        "pyannote_vad_manifest.json",
        "pyannote_diarization_manifest.json",
    ] {
        let path = parent.join(file_name);
        if !path.is_file() {
            continue;
        }
        let text = fs::read_to_string(&path).map_err(|error| {
            DetectError::InvalidArgument(format!(
                "failed to read pyannote VAD manifest `{}`: {error}",
                path.display()
            ))
        })?;
        let manifest = serde_json::from_str(&text).map_err(|error| {
            DetectError::InvalidArgument(format!(
                "failed to parse pyannote VAD manifest `{}`: {error}",
                path.display()
            ))
        })?;
        return Ok(Some(manifest));
    }
    Ok(None)
}

#[cfg(feature = "pyannote-vad")]
fn pyannote_model_shape(
    metadata: &runtime_onnx::OnnxSessionMetadata,
    options: &PyannoteVadOptions,
    manifest: Option<&PyannoteVadManifest>,
) -> Result<PyannoteVadModelShape> {
    if manifest
        .and_then(|manifest| manifest.sample_rate)
        .is_some_and(|sample_rate| sample_rate != PYANNOTE_SAMPLE_RATE)
    {
        return Err(DetectError::InvalidArgument(format!(
            "pyannote VAD manifest sampleRate must be {PYANNOTE_SAMPLE_RATE}"
        )));
    }
    let manifest_segmentation = manifest.and_then(|manifest| manifest.segmentation.as_ref());
    let input_name = options
        .input_name
        .clone()
        .or_else(|| manifest_segmentation.and_then(|segmentation| segmentation.input_name.clone()))
        .or_else(|| metadata.inputs.first().map(|input| input.name.clone()))
        .ok_or_else(|| {
            DetectError::InvalidArgument("pyannote ONNX model has no inputs".to_string())
        })?;
    let input = metadata
        .inputs
        .iter()
        .find(|input| input.name == input_name)
        .ok_or_else(|| {
            DetectError::InvalidArgument(format!(
                "pyannote ONNX input `{input_name}` was not found; available inputs: {}",
                metadata
                    .inputs
                    .iter()
                    .map(|input| input.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })?;
    if input.element_type != Some(runtime_onnx::OnnxTensorElementType::F32) {
        return Err(DetectError::InvalidArgument(format!(
            "pyannote ONNX input `{input_name}` must be f32"
        )));
    }
    let output_name = options.output_name.clone().or_else(|| {
        manifest_segmentation.and_then(|segmentation| segmentation.output_name.clone())
    });
    if let Some(output_name) = output_name.as_deref() {
        let output = metadata
            .outputs
            .iter()
            .find(|output| output.name == output_name)
            .ok_or_else(|| {
                DetectError::InvalidArgument(format!(
                    "pyannote ONNX output `{output_name}` was not found; available outputs: {}",
                    metadata
                        .outputs
                        .iter()
                        .map(|output| output.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            })?;
        if output.element_type != Some(runtime_onnx::OnnxTensorElementType::F32) {
            return Err(DetectError::InvalidArgument(format!(
                "pyannote ONNX output `{output_name}` must be f32"
            )));
        }
    } else if !metadata
        .outputs
        .iter()
        .any(|output| output.element_type == Some(runtime_onnx::OnnxTensorElementType::F32))
    {
        return Err(DetectError::InvalidArgument(
            "pyannote ONNX model must expose at least one f32 output".to_string(),
        ));
    }

    let window_seconds = manifest_segmentation
        .map(|segmentation| segmentation.duration_seconds)
        .filter(|seconds| seconds.is_finite() && *seconds > 0.0)
        .or_else(|| {
            fixed_audio_samples(input).map(|samples| samples as f64 / PYANNOTE_SAMPLE_RATE as f64)
        })
        .unwrap_or(PYANNOTE_DEFAULT_WINDOW_SECONDS);
    let window_samples = ((window_seconds * PYANNOTE_SAMPLE_RATE as f64).round() as usize).max(1);
    let input_shape = fixed_input_shape(input).unwrap_or_else(|| vec![1, window_samples]);
    let step_ratio = manifest_segmentation
        .and_then(|segmentation| segmentation.step_ratio)
        .filter(|ratio| ratio.is_finite() && *ratio > 0.0)
        .unwrap_or(PYANNOTE_DEFAULT_STEP_RATIO);
    let step_samples = ((window_samples as f64 * step_ratio).round() as usize).max(1);
    Ok(PyannoteVadModelShape {
        input_name,
        output_name,
        input_shape,
        window_samples,
        window_seconds,
        step_samples,
        frames: manifest_segmentation.and_then(|segmentation| segmentation.frames),
        speakers: manifest_segmentation.and_then(|segmentation| segmentation.local_speakers),
    })
}

#[cfg(feature = "pyannote-vad")]
fn fixed_input_shape(input: &runtime_onnx::OnnxIoInfo) -> Option<Vec<usize>> {
    input
        .dimensions
        .iter()
        .map(|dimension| match dimension {
            runtime_onnx::OnnxDimension::Fixed(value) => Some(*value),
            runtime_onnx::OnnxDimension::Symbolic(_) | runtime_onnx::OnnxDimension::Unknown => None,
        })
        .collect()
}

#[cfg(feature = "pyannote-vad")]
fn fixed_audio_samples(input: &runtime_onnx::OnnxIoInfo) -> Option<usize> {
    fixed_input_shape(input).and_then(|shape| shape.into_iter().rfind(|value| *value > 1))
}

#[cfg(feature = "pyannote-vad")]
fn pyannote_output_frames(
    output: &runtime_onnx::OnnxF32Tensor,
    model: &PyannoteVadModelShape,
    window_start_seconds: f64,
) -> Result<Vec<PyannoteVadFrame>> {
    let (frames, speakers) = pyannote_output_shape(output, model)?;
    let frame_seconds = model.window_seconds / frames as f64;
    let mut result = Vec::with_capacity(frames);
    for frame in 0..frames {
        let mut score = 0.0_f32;
        for speaker in 0..speakers {
            score = score.max(output.values[frame * speakers + speaker]);
        }
        result.push(PyannoteVadFrame {
            start_seconds: window_start_seconds + frame as f64 * frame_seconds,
            end_seconds: window_start_seconds + (frame + 1) as f64 * frame_seconds,
            score,
        });
    }
    Ok(result)
}

#[cfg(feature = "pyannote-vad")]
fn pyannote_output_shape(
    output: &runtime_onnx::OnnxF32Tensor,
    model: &PyannoteVadModelShape,
) -> Result<(usize, usize)> {
    let values = output.values.len();
    match output.shape.as_slice() {
        [1, frames, speakers] => Ok((*frames, *speakers)),
        [1, frames] if model.speakers.is_none() || model.speakers == Some(1) => Ok((*frames, 1)),
        [frames, speakers] => Ok((*frames, *speakers)),
        [frames] if model.speakers.is_none() || model.speakers == Some(1) => Ok((*frames, 1)),
        _ => match (model.frames, model.speakers) {
            (Some(frames), Some(speakers)) if frames * speakers == values => Ok((frames, speakers)),
            (Some(frames), None) if values.is_multiple_of(frames) => Ok((frames, values / frames)),
            (None, Some(speakers)) if values.is_multiple_of(speakers) => {
                Ok((values / speakers, speakers))
            }
            _ => Err(DetectError::InvalidArgument(format!(
                "pyannote ONNX output shape {:?} could not be interpreted as VAD frames",
                output.shape
            ))),
        },
    }
}

#[cfg(feature = "silero-vad")]
fn validate_onnx_metadata(
    metadata: &runtime_onnx::OnnxSessionMetadata,
    input_name: &str,
    output_name: Option<&str>,
) -> Result<()> {
    let input = metadata
        .inputs
        .iter()
        .find(|input| input.name == input_name)
        .ok_or_else(|| {
            DetectError::InvalidArgument(format!(
                "silero ONNX input `{input_name}` was not found; available inputs: {}",
                metadata
                    .inputs
                    .iter()
                    .map(|input| input.name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })?;
    if input.element_type != Some(runtime_onnx::OnnxTensorElementType::F32) {
        return Err(DetectError::InvalidArgument(format!(
            "silero ONNX input `{input_name}` must be f32"
        )));
    }
    for required in ["state", "sr"] {
        if !metadata.inputs.iter().any(|input| input.name == required) {
            return Err(DetectError::InvalidArgument(format!(
                "silero ONNX model is missing required `{required}` input"
            )));
        }
    }
    if let Some(output_name) = output_name {
        let output = metadata
            .outputs
            .iter()
            .find(|output| output.name == output_name)
            .ok_or_else(|| {
                DetectError::InvalidArgument(format!(
                    "silero ONNX output `{output_name}` was not found; available outputs: {}",
                    metadata
                        .outputs
                        .iter()
                        .map(|output| output.name.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
            })?;
        if output.element_type != Some(runtime_onnx::OnnxTensorElementType::F32) {
            return Err(DetectError::InvalidArgument(format!(
                "silero ONNX output `{output_name}` must be f32"
            )));
        }
    } else if !metadata
        .outputs
        .iter()
        .any(|output| output.element_type == Some(runtime_onnx::OnnxTensorElementType::F32))
    {
        return Err(DetectError::InvalidArgument(
            "silero ONNX model must expose at least one f32 output".to_string(),
        ));
    }
    Ok(())
}

#[cfg(feature = "silero-vad")]
fn silero_onnx_source(error: runtime_onnx::OnnxRuntimeError) -> DetectError {
    DetectError::Source(format!("silero ONNX runtime error: {error}"))
}

#[cfg(feature = "silero-vad")]
fn silero_onnx_invalid(error: runtime_onnx::OnnxRuntimeError) -> DetectError {
    DetectError::InvalidArgument(format!("silero ONNX tensor error: {error}"))
}

#[cfg(feature = "pyannote-vad")]
fn pyannote_onnx_source(error: runtime_onnx::OnnxRuntimeError) -> DetectError {
    DetectError::Source(format!("pyannote ONNX runtime error: {error}"))
}

#[cfg(feature = "pyannote-vad")]
fn pyannote_onnx_invalid(error: runtime_onnx::OnnxRuntimeError) -> DetectError {
    DetectError::InvalidArgument(format!("pyannote ONNX tensor error: {error}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use audio_analysis_transcription::{LoadedAudio, VadOptions};

    #[derive(Debug)]
    struct MockSileroRunner {
        probabilities: Vec<f32>,
        calls: usize,
    }

    impl SileroProbabilityRunner for MockSileroRunner {
        fn speech_probabilities(
            &mut self,
            _samples: &[f32],
            _sample_rate: u32,
        ) -> Result<Vec<f32>> {
            self.calls += 1;
            Ok(self.probabilities.clone())
        }
    }

    #[derive(Debug)]
    struct MockPyannoteRunner {
        frames: Vec<PyannoteVadFrame>,
        windows: usize,
    }

    impl PyannoteFrameRunner for MockPyannoteRunner {
        fn speech_frames(
            &mut self,
            _samples: &[f32],
            _sample_rate: u32,
        ) -> Result<PyannoteFrameBatch> {
            Ok(PyannoteFrameBatch {
                frames: self.frames.clone(),
                windows: self.windows,
            })
        }
    }

    #[test]
    fn silero_timestamps_detect_speech_with_hysteresis() {
        let detector = detector();
        let probabilities = vec![
            0.1, 0.1, 0.6, 0.65, 0.7, 0.72, 0.74, 0.73, 0.7, 0.65, 0.4, 0.4, 0.3, 0.2, 0.1, 0.1,
            0.1, 0.1,
        ];
        let segments = detector
            .detect_from_probabilities(&probabilities, probabilities.len() * SILERO_WINDOW_SAMPLES)
            .expect("timestamps");

        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].start_seconds, 0.034);
        assert_eq!(segments[0].end_seconds, 0.414);
        assert_eq!(segments[0].score, 0.74);
    }

    #[test]
    fn silero_timestamps_filters_short_speech() {
        let detector = detector();
        let probabilities = vec![0.1, 0.6, 0.7, 0.1, 0.1, 0.1, 0.1];
        let segments = detector
            .detect_from_probabilities(&probabilities, probabilities.len() * SILERO_WINDOW_SAMPLES)
            .expect("timestamps");

        assert!(segments.is_empty());
    }

    #[test]
    fn silero_timestamps_splits_at_max_speech() {
        let detector = SileroTimestampDetector {
            max_speech_duration_seconds: 0.45,
            ..detector()
        };
        let probabilities = vec![0.7; 40];
        let segments = detector
            .detect_from_probabilities(&probabilities, probabilities.len() * SILERO_WINDOW_SAMPLES)
            .expect("timestamps");

        assert!(segments.len() > 1, "expected split spans, got {segments:?}");
        assert!(segments.iter().all(|segment| segment.score == 0.7));
    }

    #[test]
    fn merge_whisperx_vad_chunks_matches_expected_boundaries() {
        let merged = merge_whisperx_vad_chunks(
            vec![
                SpeechActivitySegment::new(0.0, 4.0, 0.5).unwrap(),
                SpeechActivitySegment::new(4.2, 8.0, 0.6).unwrap(),
                SpeechActivitySegment::new(8.2, 11.0, 0.7).unwrap(),
            ],
            7.0,
        )
        .expect("merge");

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].start_seconds, 0.0);
        assert_eq!(merged[0].end_seconds, 4.0);
        assert_eq!(merged[1].start_seconds, 4.2);
        assert_eq!(merged[1].end_seconds, 11.0);
        assert_eq!(merged[1].score, 0.7);
    }

    #[test]
    fn merge_whisperx_vad_chunks_accepts_empty_input() {
        let merged = merge_whisperx_vad_chunks(Vec::new(), 30.0).expect("merge");

        assert!(merged.is_empty());
    }

    #[test]
    fn merge_whisperx_vad_chunks_ignores_silero_onset_and_offset() {
        let segments = vec![
            SpeechActivitySegment::new(0.0, 4.0, 0.5).unwrap(),
            SpeechActivitySegment::new(4.2, 8.0, 0.6).unwrap(),
            SpeechActivitySegment::new(8.2, 11.0, 0.7).unwrap(),
        ];

        let default = merge_with_whisperx_silero_args(segments.clone(), 7.0, 0.5, Some(0.363))
            .expect("default merge");
        let changed =
            merge_with_whisperx_silero_args(segments, 7.0, 0.9, Some(0.01)).expect("changed merge");

        assert_eq!(default, changed);
    }

    #[test]
    fn merge_whisperx_vad_chunks_starts_new_chunk_only_after_progress() {
        let merged = merge_whisperx_vad_chunks(
            vec![
                SpeechActivitySegment::new(0.0, 10.0, 0.5).unwrap(),
                SpeechActivitySegment::new(10.0, 11.0, 0.6).unwrap(),
            ],
            5.0,
        )
        .expect("merge");

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].start_seconds, 0.0);
        assert_eq!(merged[0].end_seconds, 10.0);
        assert_eq!(merged[1].start_seconds, 10.0);
        assert_eq!(merged[1].end_seconds, 11.0);
    }

    #[test]
    fn silero_provider_uses_onnx_probabilities() {
        let probabilities = vec![
            0.1, 0.1, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7, 0.1, 0.1, 0.1, 0.1, 0.1,
        ];
        let mut provider = SileroVadTranscriptionProvider::new_for_runner(
            detector(),
            Box::new(MockSileroRunner {
                probabilities,
                calls: 0,
            }),
            Vec::new(),
        );
        let response = provider.detect_speech(vad_request(1.0)).expect("detect");

        assert_eq!(response.segments.len(), 1);
        assert_eq!(response.segments[0].score, 0.7);
    }

    #[test]
    fn silero_provider_resets_state_per_detection() {
        let probabilities = vec![
            0.1, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.6, 0.1, 0.1, 0.1, 0.1,
        ];
        let mut provider = SileroVadTranscriptionProvider::new_for_runner(
            detector(),
            Box::new(MockSileroRunner {
                probabilities,
                calls: 0,
            }),
            Vec::new(),
        );
        let first = provider.detect_speech(vad_request(1.0)).expect("first");
        let second = provider.detect_speech(vad_request(1.0)).expect("second");

        assert_eq!(first.segments, second.segments);
    }

    #[test]
    fn silero_provider_rejects_non_16khz_audio() {
        let mut request = vad_request(1.0);
        request.audio.sample_rate = 8_000;
        let mut provider = SileroVadTranscriptionProvider::new_for_runner(
            detector(),
            Box::new(MockSileroRunner {
                probabilities: vec![0.1],
                calls: 0,
            }),
            Vec::new(),
        );
        let error = provider.detect_speech(request).expect_err("should reject");

        assert!(error.to_string().contains("16000"));
    }

    #[test]
    fn pyannote_provider_uses_onset_and_offset_hysteresis() {
        let mut provider = PyannoteVadTranscriptionProvider::new_for_runner(
            0.5,
            0.3,
            30.0,
            Box::new(MockPyannoteRunner {
                frames: vec![
                    pyannote_frame(0.0, 0.1, 0.2),
                    pyannote_frame(0.1, 0.2, 0.55),
                    pyannote_frame(0.2, 0.3, 0.4),
                    pyannote_frame(0.3, 0.4, 0.35),
                    pyannote_frame(0.4, 0.5, 0.29),
                ],
                windows: 1,
            }),
            Vec::new(),
        );
        let response = provider.detect_speech(vad_request(1.0)).expect("detect");

        assert_eq!(response.segments.len(), 1);
        assert_eq!(response.segments[0].start_seconds, 0.1);
        assert_eq!(response.segments[0].end_seconds, 0.4);
        assert_eq!(response.segments[0].score, 0.55);
        assert!(response
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic == "pyannoteVadWindows=1"));
    }

    #[test]
    fn pyannote_provider_applies_chunk_size_to_merged_speech() {
        let mut provider = PyannoteVadTranscriptionProvider::new_for_runner(
            0.5,
            0.3,
            5.0,
            Box::new(MockPyannoteRunner {
                frames: vec![
                    pyannote_frame(0.0, 3.0, 0.8),
                    pyannote_frame(3.0, 3.1, 0.2),
                    pyannote_frame(3.2, 6.0, 0.8),
                    pyannote_frame(6.0, 6.1, 0.2),
                    pyannote_frame(6.2, 8.0, 0.8),
                    pyannote_frame(8.0, 8.1, 0.2),
                ],
                windows: 1,
            }),
            Vec::new(),
        );
        let response = provider.detect_speech(vad_request(12.0)).expect("detect");

        assert_eq!(response.segments.len(), 2);
        assert_eq!(response.segments[0].start_seconds, 0.0);
        assert_eq!(response.segments[0].end_seconds, 3.0);
        assert_eq!(response.segments[1].start_seconds, 3.2);
        assert_eq!(response.segments[1].end_seconds, 8.0);
    }

    #[cfg(feature = "silero-vad")]
    #[test]
    #[ignore]
    fn silero_real_onnx_smoke_from_env() {
        let Some(path) = std::env::var_os("NATIVE_WHISPERX_SILERO_ONNX") else {
            return;
        };
        let options = SileroVadOptions {
            model_path: PathBuf::from(path),
            input_name: None,
            output_name: None,
            threshold: 0.5,
            max_speech_duration_seconds: 30.0,
            min_speech_duration_ms: 250,
            min_silence_duration_ms: 100,
            speech_pad_ms: 30,
        };
        let mut provider =
            SileroVadTranscriptionProvider::from_options(options, Vec::new()).expect("provider");
        let mut request = vad_request(2.0);
        for sample in &mut request.audio.samples[8_000..24_000] {
            *sample = 0.2;
        }
        let response = provider.detect_speech(request).expect("detect");

        assert!(!response.segments.is_empty());
    }

    fn detector() -> SileroTimestampDetector {
        SileroTimestampDetector {
            threshold: 0.5,
            max_speech_duration_seconds: 30.0,
            min_speech_duration_ms: 250,
            min_silence_duration_ms: 100,
            speech_pad_ms: 30,
        }
    }

    fn vad_request(duration_seconds: f64) -> VadRequest {
        let samples = (duration_seconds * SILERO_SAMPLE_RATE as f64).ceil() as usize;
        VadRequest {
            audio: LoadedAudio {
                samples: vec![0.0; samples],
                sample_rate: SILERO_SAMPLE_RATE,
                channels: 1,
                source: Some("mock.wav".to_string()),
            },
            options: VadOptions {
                enabled: true,
                max_chunk_seconds: 30.0,
                ..VadOptions::default()
            },
        }
    }

    fn pyannote_frame(start_seconds: f64, end_seconds: f64, score: f32) -> PyannoteVadFrame {
        PyannoteVadFrame {
            start_seconds,
            end_seconds,
            score,
        }
    }

    fn merge_with_whisperx_silero_args(
        segments: Vec<SpeechActivitySegment>,
        chunk_size: f64,
        _onset: f32,
        _offset: Option<f32>,
    ) -> Result<Vec<SpeechActivitySegment>> {
        merge_whisperx_vad_chunks(segments, chunk_size)
    }
}
