use std::path::Path;

use video_analysis_core::{DetectError, Result};

use crate::{
    invalid_request, normalize_samples_source, resample_linear, LoadedAudio, TranscriptionSource,
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) struct NativeDecodeDiagnostics {
    pub decode_route: &'static str,
    pub source_path_extension: Option<String>,
    pub input_sample_rate: Option<u32>,
    pub output_sample_rate: u32,
    pub output_channels: u16,
}

pub(crate) fn mono_16khz_from_source(source: &TranscriptionSource) -> Result<LoadedAudio> {
    Ok(mono_16khz_from_source_with_diagnostics(source)?.0)
}

#[allow(dead_code)]
pub(crate) fn mono_16khz_from_source_with_diagnostics(
    source: &TranscriptionSource,
) -> Result<(LoadedAudio, NativeDecodeDiagnostics)> {
    match source {
        TranscriptionSource::Samples {
            samples,
            sample_rate,
            channels,
            source,
        } => {
            let audio = normalize_samples_source(samples, *sample_rate, *channels, source.clone())?;
            let output_sample_rate = audio.sample_rate;
            let output_channels = audio.channels;
            Ok((
                audio,
                NativeDecodeDiagnostics {
                    decode_route: "direct-samples",
                    source_path_extension: None,
                    input_sample_rate: Some(*sample_rate),
                    output_sample_rate,
                    output_channels,
                },
            ))
        }
        TranscriptionSource::Path { path } => load_path_mono_16khz_with_diagnostics(path),
    }
}

fn load_path_mono_16khz_with_diagnostics(
    path: &Path,
) -> Result<(LoadedAudio, NativeDecodeDiagnostics)> {
    let extension = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase());
    if is_wav_path(path) {
        let (audio, input_sample_rate) = load_wav_mono_16khz_with_input_sample_rate(path)?;
        let output_sample_rate = audio.sample_rate;
        let output_channels = audio.channels;
        Ok((
            audio,
            NativeDecodeDiagnostics {
                decode_route: "native-wav-reader",
                source_path_extension: extension,
                input_sample_rate: Some(input_sample_rate),
                output_sample_rate,
                output_channels,
            },
        ))
    } else {
        let (audio, input_sample_rate) = load_media_mono_16khz_with_input_sample_rate(path)?;
        let output_sample_rate = audio.sample_rate;
        let output_channels = audio.channels;
        Ok((
            audio,
            NativeDecodeDiagnostics {
                decode_route: "audio-io-media-decode",
                source_path_extension: extension,
                input_sample_rate,
                output_sample_rate,
                output_channels,
            },
        ))
    }
}

fn load_wav_mono_16khz_with_input_sample_rate(path: &Path) -> Result<(LoadedAudio, u32)> {
    let mut reader = hound::WavReader::open(path).map_err(|error| {
        invalid_request(format!("failed to open WAV `{}`: {error}", path.display()))
    })?;
    let spec = reader.spec();
    if spec.sample_rate == 0 || spec.channels == 0 {
        return Err(DetectError::InvalidAudioFormat {
            sample_rate: spec.sample_rate,
            channels: spec.channels,
        });
    }
    let interleaved = match spec.sample_format {
        hound::SampleFormat::Float => {
            if spec.bits_per_sample != 32 {
                return Err(invalid_request(format!(
                    "unsupported float WAV bit depth {}; expected 32",
                    spec.bits_per_sample
                )));
            }
            reader
                .samples::<f32>()
                .map(|sample| {
                    sample.map_err(|error| {
                        invalid_request(format!(
                            "failed to read WAV `{}` sample: {error}",
                            path.display()
                        ))
                    })
                })
                .collect::<Result<Vec<_>>>()?
        }
        hound::SampleFormat::Int => read_int_samples(&mut reader, spec.bits_per_sample, path)?,
    };
    if interleaved.is_empty() {
        return Err(invalid_request("empty audio"));
    }
    if interleaved.iter().any(|sample| !sample.is_finite()) {
        return Err(invalid_request("audio samples must be finite"));
    }
    let channels = spec.channels as usize;
    let mono = if channels == 1 {
        interleaved
    } else {
        interleaved
            .chunks_exact(channels)
            .map(|frame| frame.iter().copied().sum::<f32>() / channels as f32)
            .collect::<Vec<_>>()
    };
    let samples = if spec.sample_rate == 16_000 {
        mono
    } else {
        resample_linear(&mono, spec.sample_rate, 16_000)
    };
    Ok((
        LoadedAudio {
            samples,
            sample_rate: 16_000,
            channels: 1,
            source: Some(path.to_string_lossy().into_owned()),
        },
        spec.sample_rate,
    ))
}

#[cfg(feature = "audio-io")]
fn load_media_mono_16khz_with_input_sample_rate(path: &Path) -> Result<(LoadedAudio, Option<u32>)> {
    let (metadata, mono) = audio_analysis_io::decode_audio_to_mono_f32(
        audio_analysis_io::AudioInput::File(path.to_path_buf()),
        audio_analysis_io::AudioInputOptions::recorded(),
        audio_analysis_io::ChannelMix::Average,
    )?;
    let audio = normalize_samples_source(
        &mono,
        metadata.sample_rate,
        1,
        Some(path.to_string_lossy().into_owned()),
    )?;
    Ok((audio, Some(metadata.sample_rate)))
}

#[cfg(not(feature = "audio-io"))]
fn load_media_mono_16khz_with_input_sample_rate(
    _path: &Path,
) -> Result<(LoadedAudio, Option<u32>)> {
    Err(crate::unsupported_runtime(
        "native path decoding currently supports WAV files only; pass Samples, enable the audio-io feature for FFmpeg-backed container/video input, or use externalWhisperX",
    ))
}

fn is_wav_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("wav"))
}

fn read_int_samples(
    reader: &mut hound::WavReader<std::io::BufReader<std::fs::File>>,
    bits_per_sample: u16,
    path: &Path,
) -> Result<Vec<f32>> {
    match bits_per_sample {
        1..=8 => reader
            .samples::<i8>()
            .map(|sample| {
                sample.map(|sample| sample as f32 / 128.0).map_err(|error| {
                    invalid_request(format!(
                        "failed to read WAV `{}` sample: {error}",
                        path.display()
                    ))
                })
            })
            .collect(),
        9..=16 => reader
            .samples::<i16>()
            .map(|sample| {
                sample
                    .map(|sample| sample as f32 / 32_768.0)
                    .map_err(|error| {
                        invalid_request(format!(
                            "failed to read WAV `{}` sample: {error}",
                            path.display()
                        ))
                    })
            })
            .collect(),
        17..=32 => {
            let scale = 2_f32.powi(bits_per_sample as i32 - 1);
            reader
                .samples::<i32>()
                .map(|sample| {
                    sample.map(|sample| sample as f32 / scale).map_err(|error| {
                        invalid_request(format!(
                            "failed to read WAV `{}` sample: {error}",
                            path.display()
                        ))
                    })
                })
                .collect()
        }
        _ => Err(invalid_request(format!(
            "unsupported integer WAV bit depth {bits_per_sample}"
        ))),
    }
}

pub(crate) fn validate_loaded_audio(audio: &LoadedAudio) -> Result<()> {
    if audio.sample_rate != 16_000 {
        return Err(invalid_request(format!(
            "native transcription expects 16 kHz audio, got {} Hz",
            audio.sample_rate
        )));
    }
    if audio.channels != 1 {
        return Err(invalid_request(format!(
            "native transcription expects mono audio, got {} channels",
            audio.channels
        )));
    }
    if audio.samples.is_empty() {
        return Err(invalid_request("empty audio"));
    }
    if audio.samples.iter().any(|sample| !sample.is_finite()) {
        return Err(invalid_request("audio samples must be finite"));
    }
    Ok(())
}
