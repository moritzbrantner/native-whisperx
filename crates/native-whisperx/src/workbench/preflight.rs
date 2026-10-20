//! Product resource messages composed from canonical, non-loading inspections.
use serde::Serialize;

use crate::{
    resolve_automatic_workflow_selection, AsrProvider, InputSource, NativeWhisperxConfig,
    NativeWhisperxError, VadMethod,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkbenchPreflight {
    /// All selected resource checks passed. This does not verify inference.
    pub ready: bool,
    pub checks: Vec<WorkbenchResourceCheck>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkbenchResourceCheck {
    pub resource: &'static str,
    pub available: bool,
    pub message: String,
}

impl WorkbenchPreflight {
    fn check(&mut self, resource: &'static str, result: Result<String, impl std::fmt::Display>) {
        let (available, message) = match result {
            Ok(message) => (true, message),
            Err(error) => (false, error.to_string()),
        };
        self.ready &= available;
        self.checks.push(WorkbenchResourceCheck {
            resource,
            available,
            message,
        });
    }
}

/// Inspects selected workbench resources without decoding input, downloading or
/// constructing model sessions. Provider inspection may read manifests, file
/// checksums and static ONNX metadata. Callers authorize paths. Available files
/// remain subject to normal execution validation.
pub fn preflight_workbench(config: &NativeWhisperxConfig) -> WorkbenchPreflight {
    let mut report = WorkbenchPreflight {
        ready: true,
        checks: Vec::new(),
    };
    let resolved = (|| {
        if !cfg!(feature = "native") || config.asr.provider != AsrProvider::Native {
            return Err(NativeWhisperxError::InvalidConfig(
                "Workbench transcription requires the native feature and provider".into(),
            ));
        }
        if config.asr.device == crate::DevicePreference::Cuda && !cfg!(feature = "cuda") {
            return Err(NativeWhisperxError::InvalidConfig("CUDA execution requires a build with the cuda feature; select CPU or install a CUDA build".into()));
        }
        crate::config_mapping::validate_pre_resolution_support(config)?;
        let mut cached = config.clone();
        cached.asr.model_cache_only = true;
        cached.alignment.model_cache_only = true;
        cached.translation.model_cache_only = true;
        let config = resolve_automatic_workflow_selection(&cached)?.config;
        let request =
            crate::config_mapping::build_transcription_request_from_resolved_config(&config)?;
        Ok((config, request))
    })();
    let (config, request) = match resolved {
        Ok(resolved) => resolved,
        Err(error) => {
            report.check("configuration", Err::<String, _>(error));
            return report;
        }
    };
    if let InputSource::Path { path } = &config.input {
        if !path
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("wav"))
        {
            #[cfg(feature = "media-decode")]
            report.check(
                "media",
                if audio_analysis_io::is_ffmpeg_available()
                    && audio_analysis_io::is_ffprobe_available()
                {
                    Ok("FFmpeg and FFprobe are available for local media decoding".to_owned())
                } else {
                    Err("Install local FFmpeg and FFprobe, or select a readable WAV file")
                },
            );
            #[cfg(not(feature = "media-decode"))]
            report.check("media", Err::<String,_>("This build lacks media-decode; select a WAV file or install a build with media decoding"));
        }
    }
    let whisper = match &request.provider {
        audio_analysis_transcription::TranscriptionProviderSelection::CandleWhisper(options) => {
            crate::config_mapping::build_native_runtime_controls(&config.asr)
                .map_err(|error| error.to_string())
                .and_then(|controls| audio_analysis_transcription::inspect_candle_whisper_resources(options, controls).map_err(|error| error.to_string()))
                .map(|inspection| format!("{} cached files are present · {:?} · {}. Model execution is not yet verified.", inspection.model.model_id, inspection.model.device, inspection.compute_type.as_str()))
        }
        _ => Err("Select the supported native transcription provider".into()),
    };
    report.check("transcription", whisper);
    if config.alignment.enabled {
        report.check(
            "alignment",
            audio_analysis_transcription::inspect_alignment_resources(&request.alignment).map(
                |_| "Alignment model files are present; execution is not yet verified".to_owned(),
            ),
        );
    }
    if config.vad.enabled {
        match config.vad.method {
            VadMethod::Energy => {}
            VadMethod::Pyannote => {
                report.check(
                    "speech detection",
                    config
                        .vad
                        .model_bundle
                        .as_deref()
                        .ok_or_else(|| "Select a prepared local pyannote VAD bundle".to_owned())
                        .and_then(|bundle| {
                            crate::verify_pyannote_vad_bundle(bundle)
                                .map_err(|error| error.to_string())
                        })
                        .map(|_| "Local pyannote VAD bundle verified".to_owned()),
                );
            }
            VadMethod::Silero => report.check(
                "speech detection",
                Err::<String, _>("This workbench supports energy or pyannote speech detection"),
            ),
        }
    }
    if config.diarization.enabled {
        inspect_speakers(&config, &mut report);
    }
    if config.translation.enabled {
        report.check("translation", inspect_translation(&config));
    }
    report
}

#[cfg(feature = "pyannote-diarization")]
fn inspect_speakers(config: &NativeWhisperxConfig, report: &mut WorkbenchPreflight) {
    use audio_analysis_speakers::{
        inspect_pyannote_community_diarization_bundle, PyannoteCommunityDiarizationConfig,
    };
    let Some(bundle) = &config.diarization.model_bundle else {
        report.check(
            "speaker model",
            Err::<String, _>("Select a prepared pyannote community diarization bundle"),
        );
        return;
    };
    let d = &config.diarization;
    let inspected =
        inspect_pyannote_community_diarization_bundle(PyannoteCommunityDiarizationConfig {
            bundle_path: bundle.clone(),
            manifest_file: d.manifest_file.clone(),
            segmentation_model_file: d.segmentation_model_file.clone(),
            embedding_model_file: d.embedding_model_file.clone(),
            plda_transform_file: d.plda_transform_file.clone(),
            plda_model_file: d.plda_model_file.clone(),
            clustering_config_file: d.clustering_config_file.clone(),
            min_speakers: d.min_speakers,
            max_speakers: d.max_speakers,
            return_speaker_embeddings: false,
        });
    let inspection = match inspected {
        Ok(inspection) => inspection,
        Err(error) => {
            report.check("speaker model", Err::<String, _>(error));
            return;
        }
    };
    report.check(
        "speaker model",
        Ok::<_, String>(
            "Local pyannote community bundle verified; inference has not started".to_owned(),
        ),
    );
    if d.disable_speaker_library {
        return;
    }
    let library = (|| {
        let cwd = std::env::current_dir().map_err(|error| error.to_string())?;
        let directory = crate::resolve_speaker_directory(&d.speaker_directory, &cwd)
            .map_err(|error| error.to_string())?;
        let library = crate::speaker_directory::load_speaker_library_if_present(&directory.path)
            .map_err(|error| error.to_string())?;
        match library.and_then(|library| library.embedding_model().cloned()) {
            Some(model) if model != inspection.embedding_model => Err("The active Speaker Library uses an incompatible embedding model. Select a compatible index or create an empty index before running.".to_owned()),
            Some(_) => Ok("Active Speaker Library is compatible with the selected speaker model".to_owned()),
            None => Ok("No saved speaker identities in the selected index; this run can produce anonymous labels".to_owned()),
        }
    })();
    report.check("speaker library", library);
}

#[cfg(not(feature = "pyannote-diarization"))]
fn inspect_speakers(_: &NativeWhisperxConfig, report: &mut WorkbenchPreflight) {
    report.check(
        "speaker model",
        Err::<String, _>("Speaker identification requires the pyannote-diarization feature"),
    );
}

fn inspect_translation(config: &NativeWhisperxConfig) -> Result<String, String> {
    let translation = &config.translation;
    validate_translation_choice(
        translation.model_id.as_deref().unwrap_or(""),
        translation.source_language.as_deref().unwrap_or(""),
        translation.target_language.as_deref().unwrap_or(""),
    )?;
    #[cfg(feature = "translation")]
    {
        let bundle = crate::translation::resolve_translation_bundle(translation)
            .map_err(|error| error.to_string())?;
        for file in [
            &bundle.config_json,
            &bundle.generation_config_json,
            &bundle.source_spm,
            &bundle.target_spm,
            &bundle.vocab_json,
            &bundle.model_weights,
        ] {
            if !file.is_file() {
                return Err(format!(
                    "Translation resource `{}` must be a regular file",
                    file.display()
                ));
            }
        }
        Ok(
            "Supported translation model files are cached locally; execution is not yet verified"
                .to_owned(),
        )
    }
    #[cfg(not(feature = "translation"))]
    {
        Err("Translation requires a build with the translation feature".into())
    }
}

pub(super) fn validate_translation_choice(
    model_id: &str,
    source: &str,
    target: &str,
) -> Result<(), String> {
    let plan = crate::TranslationPlan::from_language_codes(source, target)
        .map_err(|error| error.to_string())?;
    if plan.legs().len() != 1 || plan.legs()[0].model_id() != model_id {
        return Err("Select a supported direct language pair and its matching OPUS-MT model. This workbench does not execute multi-model pivot translation.".into());
    }
    Ok(())
}
