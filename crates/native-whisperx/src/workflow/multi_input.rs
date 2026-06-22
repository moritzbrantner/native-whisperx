use std::time::Instant;

use audio_analysis_transcription::{
    EnergyVadTranscriptionProvider, ReusableCandleWhisperTranscriber,
    TranscriptionProviderSelection,
};

use crate::config::{
    AsrProvider, NativeWhisperxConfig, NativeWhisperxError, NativeWhisperxReport, VadMethod,
};
use crate::config_mapping::build_transcription_request;
use crate::output::write_outputs_with_options;
use crate::report::{append_native_alignment_diagnostics, append_native_diarization_diagnostics};

use super::execution::run_with_reusable_asr;
use super::run;

pub fn run_many(
    configs: Vec<NativeWhisperxConfig>,
) -> Result<Vec<NativeWhisperxReport>, NativeWhisperxError> {
    if should_reuse_native_asr_provider(&configs) {
        return run_many_reusing_native_provider(configs);
    }
    configs.into_iter().map(run).collect()
}

pub fn run_many_reusing_native_provider(
    configs: Vec<NativeWhisperxConfig>,
) -> Result<Vec<NativeWhisperxReport>, NativeWhisperxError> {
    let mut reports = Vec::with_capacity(configs.len());
    let mut reusable_asr: Option<ReusableCandleWhisperTranscriber> = None;

    for config in configs {
        let run_started = Instant::now();
        let request = build_transcription_request(&config)?;
        let TranscriptionProviderSelection::CandleWhisper(options) = &request.provider else {
            return Err(NativeWhisperxError::InvalidConfig(
                "native multi-input reuse requires the Candle Whisper native provider".to_string(),
            ));
        };

        let reused_provider = reusable_asr
            .as_ref()
            .is_some_and(|provider| provider.options == *options);
        if !reused_provider {
            reusable_asr = Some(ReusableCandleWhisperTranscriber::new(options.clone()));
        }
        let asr_provider = reusable_asr
            .as_mut()
            .expect("native ASR provider should be initialized");
        let mut vad = EnergyVadTranscriptionProvider;
        let mut response = run_with_reusable_asr(request, &config, &mut vad, asr_provider)?;
        response.diagnostics.push(if reused_provider {
            "nativeMultiInputAsrProvider=reused".to_string()
        } else {
            "nativeMultiInputAsrProvider=loaded".to_string()
        });
        append_native_alignment_diagnostics(&mut response, &config);
        append_native_diarization_diagnostics(&mut response, &config);
        crate::save_draft_speakers_from_response(&mut response, &config)?;
        let output_started = Instant::now();
        let output_files = write_outputs_with_options(
            &response,
            &config.output,
            config.alignment.return_char_alignments,
        )?;
        response.diagnostics.push(format!(
            "phaseOutputSeconds={:.6}",
            output_started.elapsed().as_secs_f64()
        ));
        response.diagnostics.push(format!(
            "phaseNativeTotalSeconds={:.6}",
            run_started.elapsed().as_secs_f64()
        ));
        reports.push(NativeWhisperxReport {
            response,
            output_files,
        });
    }

    Ok(reports)
}

fn should_reuse_native_asr_provider(configs: &[NativeWhisperxConfig]) -> bool {
    configs.len() > 1
        && configs.iter().all(|config| {
            config.asr.provider == AsrProvider::Native
                && !config.translation.enabled
                && matches!(config.vad.method, VadMethod::Energy)
        })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::{
        AlignmentConfig, AsrConfig, AsrProvider, DiarizationConfig, InputSource,
        NativeWhisperxConfig, OutputConfig, TranslationConfig, VadConfig, VadMethod,
    };

    use super::should_reuse_native_asr_provider;

    #[test]
    fn native_multi_input_reuse_is_limited_to_energy_vad_without_translation() {
        let first = native_config("first.wav");
        let second = native_config("second.wav");

        assert!(should_reuse_native_asr_provider(&[
            first.clone(),
            second.clone()
        ]));
        assert!(!should_reuse_native_asr_provider(std::slice::from_ref(
            &first
        )));
        assert!(!should_reuse_native_asr_provider(&[
            first.clone(),
            NativeWhisperxConfig {
                vad: VadConfig {
                    method: VadMethod::Silero,
                    ..VadConfig::default()
                },
                ..second.clone()
            }
        ]));
        assert!(!should_reuse_native_asr_provider(&[
            first.clone(),
            NativeWhisperxConfig {
                asr: AsrConfig {
                    provider: AsrProvider::ExternalWhisperX,
                    ..AsrConfig::default()
                },
                ..second.clone()
            }
        ]));
        assert!(!should_reuse_native_asr_provider(&[
            first,
            NativeWhisperxConfig {
                translation: TranslationConfig {
                    enabled: true,
                    model_id: Some("Helsinki-NLP/opus-mt-de-en".to_string()),
                    ..TranslationConfig::default()
                },
                ..second
            }
        ]));
    }

    fn native_config(input: &str) -> NativeWhisperxConfig {
        NativeWhisperxConfig {
            input: InputSource::Path {
                path: PathBuf::from(input),
            },
            asr: AsrConfig::default(),
            translation: TranslationConfig::default(),
            vad: VadConfig::default(),
            alignment: AlignmentConfig::default(),
            diarization: DiarizationConfig::default(),
            output: OutputConfig::default(),
        }
    }
}
