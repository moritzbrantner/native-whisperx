#[cfg(feature = "diarization")]
use std::{fs, io::ErrorKind, path::PathBuf};

#[cfg(feature = "diarization")]
use audio_analysis_speakers::{
    AudioRuntime, DiarizationSegment, DiarizedSpeaker, EnergyVadConfig,
    EnergyVoiceActivityDetector, SpeakerAudio, SpeakerDiarizer, SpeakerIdentificationOptions,
    SpeakerLibrary, SpeakerSegmentPrediction, SpectralSpeakerEmbedder, SpeechSpan,
    WindowedSpeakerDiarizer,
};
#[cfg(feature = "diarization")]
use audio_analysis_transcription::{
    DiarizationOptions, LoadedAudio, NativeSpeakerDiarizationProvider, SpeakerDiarizationResponse,
    TranscriptDiarizationProvider,
};

#[cfg(feature = "diarization")]
use crate::config::{AsrProvider, NativeWhisperxConfig, NativeWhisperxError};

#[cfg(feature = "diarization")]
pub(crate) struct RuntimeSpeakerLibrary {
    path: PathBuf,
    profile_count: usize,
    filtered_draft_profiles: usize,
    use_draft_profiles: bool,
    library: SpeakerLibrary,
}

#[cfg(feature = "diarization")]
#[derive(Debug, Clone)]
pub(crate) enum RuntimeSpeakerLibraryStatus {
    NotRequested,
    Disabled,
    ExternalWhisperX,
    Missing(PathBuf),
    Loaded(RuntimeSpeakerLibrary),
}

#[cfg(feature = "diarization")]
impl RuntimeSpeakerLibraryStatus {
    pub(crate) fn library(&self) -> Option<SpeakerLibrary> {
        match self {
            Self::Loaded(library) => Some(library.library.clone()),
            _ => None,
        }
    }

    pub(crate) fn diagnostics(&self) -> Vec<String> {
        match self {
            Self::NotRequested => Vec::new(),
            Self::Disabled => vec!["speakerLibraryStatus=disabled".to_string()],
            Self::ExternalWhisperX => {
                vec!["speakerLibraryStatus=ignored-external-whisperx".to_string()]
            }
            Self::Missing(path) => vec![
                "speakerLibraryStatus=missing".to_string(),
                format!("speakerLibraryPath={}", path.display()),
            ],
            Self::Loaded(library) => vec![
                "speakerLibraryStatus=loaded".to_string(),
                format!("speakerLibraryPath={}", library.path.display()),
                format!("speakerLibraryProfiles={}", library.profile_count),
                format!(
                    "speakerLibraryDraftProfilesUsed={}",
                    library.use_draft_profiles
                ),
                format!(
                    "speakerLibraryDraftProfilesFiltered={}",
                    library.filtered_draft_profiles
                ),
            ],
        }
    }
}

#[cfg(feature = "diarization")]
fn runtime_speaker_library_status(
    config: &NativeWhisperxConfig,
) -> Result<RuntimeSpeakerLibraryStatus, NativeWhisperxError> {
    if !config.diarization.enabled {
        return Ok(RuntimeSpeakerLibraryStatus::NotRequested);
    }
    if config.asr.provider != AsrProvider::Native {
        return Ok(RuntimeSpeakerLibraryStatus::ExternalWhisperX);
    }
    if config.diarization.disable_speaker_library {
        return Ok(RuntimeSpeakerLibraryStatus::Disabled);
    }

    let current_dir = std::env::current_dir()?;
    let resolved =
        crate::resolve_speaker_directory(&config.diarization.speaker_directory, &current_dir)?;
    let path = crate::speaker_directory::speaker_library_path(&resolved.path);
    match fs::read_to_string(&path) {
        Ok(json) => {
            let library = SpeakerLibrary::from_json_str(&json).map_err(|error| {
                NativeWhisperxError::InvalidConfig(format!(
                    "Speaker Library `{}` is malformed or incompatible: {error}",
                    path.display()
                ))
            })?;
            let (library, filtered_draft_profiles) =
                crate::speaker_directory::filter_speaker_library_drafts(
                    &library,
                    config.diarization.use_draft_speakers,
                )?;
            Ok(RuntimeSpeakerLibraryStatus::Loaded(RuntimeSpeakerLibrary {
                path,
                profile_count: library.len(),
                filtered_draft_profiles,
                use_draft_profiles: config.diarization.use_draft_speakers,
                library,
            }))
        }
        Err(error) if error.kind() == ErrorKind::NotFound => {
            Ok(RuntimeSpeakerLibraryStatus::Missing(path))
        }
        Err(error) => Err(error.into()),
    }
}

#[cfg(feature = "diarization")]
#[derive(Debug, Clone)]
pub(crate) struct ConfiguredNativeDiarizationProvider {
    pub(crate) speaker_library: RuntimeSpeakerLibraryStatus,
}

#[cfg(feature = "diarization")]
pub(crate) fn native_diarization_provider(
    config: &NativeWhisperxConfig,
) -> Result<ConfiguredNativeDiarizationProvider, NativeWhisperxError> {
    Ok(ConfiguredNativeDiarizationProvider {
        speaker_library: runtime_speaker_library_status(config)?,
    })
}

#[cfg(feature = "diarization")]
impl TranscriptDiarizationProvider for ConfiguredNativeDiarizationProvider {
    fn provider_id(&self) -> &str {
        "native-speaker-diarization"
    }

    fn diarize(
        &mut self,
        audio: LoadedAudio,
        transcript: &text_transcripts::TranscriptionContract,
        options: &DiarizationOptions,
    ) -> video_analysis_core::Result<SpeakerDiarizationResponse> {
        let mut response = if options.is_pyannote_model() {
            let mut provider = NativeSpeakerDiarizationProvider;
            provider.diarize(audio, transcript, options)?
        } else if let Some(library) = self.speaker_library.library() {
            diarize_with_speaker_library(audio, transcript, options, library)?
        } else {
            let mut provider = NativeSpeakerDiarizationProvider;
            provider.diarize(audio, transcript, options)?
        };
        response
            .diagnostics
            .extend(self.speaker_library.diagnostics());
        if options.is_pyannote_model()
            && matches!(self.speaker_library, RuntimeSpeakerLibraryStatus::Loaded(_))
        {
            response.diagnostics.push(
                "speakerLibraryStatus=loaded-but-pyannote-provider-does-not-expose-known-speaker-identification".to_string(),
            );
        }
        Ok(response)
    }
}

#[cfg(feature = "diarization")]
fn diarize_with_speaker_library(
    audio: LoadedAudio,
    transcript: &text_transcripts::TranscriptionContract,
    options: &DiarizationOptions,
    library: SpeakerLibrary,
) -> video_analysis_core::Result<SpeakerDiarizationResponse> {
    validate_loaded_audio_for_diarization(&audio)?;
    if options.speaker_embedding_model_bundle.is_some() {
        return diarize_with_speaker_library_and_onnx_embeddings(
            audio, transcript, options, library,
        );
    }

    let speaker_audio = SpeakerAudio::mono(&audio.samples, audio.sample_rate)?;
    let embedder = SpectralSpeakerEmbedder::default();
    let spans = speech_spans_from_transcript_for_diarization(transcript, audio.duration_seconds())?;
    let result = if spans.is_empty() {
        let vad = EnergyVoiceActivityDetector::new(EnergyVadConfig::default())?;
        let mut diarizer = runtime_library_diarizer(embedder, vad, library)
            .cluster_threshold(0.95)?
            .speaker_bounds(options.min_speakers, options.max_speakers)?;
        SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?
    } else {
        let vad = TranscriptSpeechSpanVad { spans };
        let mut diarizer = runtime_library_diarizer(embedder, vad, library)
            .cluster_threshold(0.95)?
            .speaker_bounds(options.min_speakers, options.max_speakers)?;
        SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?
    };

    Ok(SpeakerDiarizationResponse {
        accepted: true,
        operation: "audio.speakers.diarize".to_string(),
        model_id: options.model_id.clone(),
        runtime: AudioRuntime::Heuristic,
        segments: stable_speaker_predictions_from_diarization(result.segments)?,
        speaker_embeddings: None,
        diagnostics: Vec::new(),
    })
}

#[cfg(all(feature = "diarization", feature = "onnx-diarization"))]
fn diarize_with_speaker_library_and_onnx_embeddings(
    audio: LoadedAudio,
    transcript: &text_transcripts::TranscriptionContract,
    options: &DiarizationOptions,
    library: SpeakerLibrary,
) -> video_analysis_core::Result<SpeakerDiarizationResponse> {
    let config = options.onnx_speaker_embedding_config()?;
    let speaker_audio = SpeakerAudio::mono(&audio.samples, audio.sample_rate)?;
    let embedder = audio_analysis_speakers::OnnxSpeakerEmbedder::from_config(config)?;
    let spans = speech_spans_from_transcript_for_diarization(transcript, audio.duration_seconds())?;
    let result = if spans.is_empty() {
        let vad = EnergyVoiceActivityDetector::default();
        let mut diarizer = runtime_library_diarizer(embedder, vad, library)
            .cluster_threshold(0.95)?
            .speaker_bounds(options.min_speakers, options.max_speakers)?;
        SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?
    } else {
        let vad = TranscriptSpeechSpanVad { spans };
        let mut diarizer = runtime_library_diarizer(embedder, vad, library)
            .cluster_threshold(0.95)?
            .speaker_bounds(options.min_speakers, options.max_speakers)?;
        SpeakerDiarizer::diarize(&mut diarizer, &speaker_audio)?
    };
    Ok(SpeakerDiarizationResponse {
        accepted: true,
        operation: "audio.speakers.diarize".to_string(),
        model_id: options.model_id.clone(),
        runtime: AudioRuntime::Onnx,
        segments: stable_speaker_predictions_from_diarization(result.segments)?,
        speaker_embeddings: None,
        diagnostics: Vec::new(),
    })
}

#[cfg(all(feature = "diarization", not(feature = "onnx-diarization")))]
fn diarize_with_speaker_library_and_onnx_embeddings(
    _audio: LoadedAudio,
    _transcript: &text_transcripts::TranscriptionContract,
    _options: &DiarizationOptions,
    _library: SpeakerLibrary,
) -> video_analysis_core::Result<SpeakerDiarizationResponse> {
    let mut provider = NativeSpeakerDiarizationProvider;
    provider.diarize(_audio, _transcript, _options)
}

#[cfg(feature = "diarization")]
fn runtime_library_diarizer<E, V>(
    embedder: E,
    vad: V,
    library: SpeakerLibrary,
) -> WindowedSpeakerDiarizer<E, V> {
    let identification_options = SpeakerIdentificationOptions {
        min_margin: None,
        ..SpeakerIdentificationOptions::default()
    };
    let mut diarizer = WindowedSpeakerDiarizer::new(embedder, vad).library(library);
    diarizer.identification_options = identification_options;
    diarizer
}

#[cfg(feature = "diarization")]
fn validate_loaded_audio_for_diarization(audio: &LoadedAudio) -> video_analysis_core::Result<()> {
    if audio.sample_rate == 0 || audio.channels == 0 {
        return Err(video_analysis_core::DetectError::InvalidAudioFormat {
            sample_rate: audio.sample_rate,
            channels: audio.channels,
        });
    }
    if audio.samples.is_empty() {
        return Err(video_analysis_core::DetectError::InvalidArgument(
            "diarization audio samples must not be empty".to_string(),
        ));
    }
    if audio.samples.iter().any(|sample| !sample.is_finite()) {
        return Err(video_analysis_core::DetectError::InvalidArgument(
            "diarization audio samples must be finite".to_string(),
        ));
    }
    Ok(())
}

#[cfg(feature = "diarization")]
fn speech_spans_from_transcript_for_diarization(
    transcript: &text_transcripts::TranscriptionContract,
    audio_duration_seconds: f64,
) -> video_analysis_core::Result<Vec<SpeechSpan>> {
    const AUDIO_DURATION_EPSILON: f64 = 1e-6;

    let has_timed_words = transcript.segments.iter().any(|segment| {
        segment.words.iter().any(|word| {
            !word.text.trim().is_empty()
                && word.start_seconds.is_some()
                && word.end_seconds.is_some()
        })
    });

    let mut spans = Vec::new();
    if has_timed_words {
        for segment in &transcript.segments {
            for word in &segment.words {
                if word.text.trim().is_empty() {
                    continue;
                }
                let Some((start, end)) = word.start_seconds.zip(word.end_seconds) else {
                    continue;
                };
                if end > audio_duration_seconds + AUDIO_DURATION_EPSILON {
                    return Err(video_analysis_core::DetectError::InvalidArgument(format!(
                        "diarization word end {:.6} exceeds audio duration {:.6}",
                        end, audio_duration_seconds
                    )));
                }
                spans.push(SpeechSpan::new(start, end, word.confidence.unwrap_or(1.0))?);
            }
        }
        return Ok(spans);
    }

    for segment in &transcript.segments {
        if segment.text.trim().is_empty() {
            continue;
        }
        let Some((start, end)) = segment.start_seconds.zip(segment.end_seconds) else {
            continue;
        };
        if end > audio_duration_seconds + AUDIO_DURATION_EPSILON {
            return Err(video_analysis_core::DetectError::InvalidArgument(format!(
                "diarization segment end {:.6} exceeds audio duration {:.6}",
                end, audio_duration_seconds
            )));
        }
        spans.push(SpeechSpan::new(start, end, 1.0)?);
    }
    Ok(spans)
}

#[cfg(feature = "diarization")]
fn stable_speaker_predictions_from_diarization(
    segments: Vec<DiarizationSegment>,
) -> video_analysis_core::Result<Vec<SpeakerSegmentPrediction>> {
    let mut unknown_labels: Vec<(String, String)> = Vec::new();
    let mut predictions = Vec::new();
    for segment in segments {
        let speaker = match segment.speaker {
            DiarizedSpeaker::Known(id) => id.as_str().to_string(),
            DiarizedSpeaker::Unknown(label) => {
                if let Some((_, stable)) = unknown_labels
                    .iter()
                    .find(|(existing, _)| existing == &label)
                {
                    stable.clone()
                } else {
                    let stable = format!("speaker_{}", unknown_labels.len());
                    unknown_labels.push((label.clone(), stable.clone()));
                    stable
                }
            }
        };
        predictions.push(SpeakerSegmentPrediction {
            speaker,
            start_seconds: segment.start_seconds as f32,
            end_seconds: segment.end_seconds as f32,
            score: Some(segment.score),
        });
    }
    Ok(predictions)
}

#[cfg(feature = "diarization")]
pub(crate) struct TranscriptSpeechSpanVad {
    spans: Vec<SpeechSpan>,
}

#[cfg(feature = "diarization")]
impl audio_analysis_speakers::VoiceActivityDetector for TranscriptSpeechSpanVad {
    fn detect_speech(
        &mut self,
        _audio: &audio_analysis_speakers::SpeakerAudio<'_>,
    ) -> video_analysis_core::Result<Vec<SpeechSpan>> {
        Ok(self.spans.clone())
    }
}
