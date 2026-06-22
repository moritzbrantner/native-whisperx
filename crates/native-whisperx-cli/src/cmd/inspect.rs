use super::*;

pub(crate) fn inspect_models_command(args: InspectModelsArgs) -> anyhow::Result<()> {
    let config = NativeWhisperxConfig {
        input: InputSource::Path {
            path: PathBuf::from("inspect-only.wav"),
        },
        asr: AsrConfig {
            model_id: args.model,
            whisper_bundle: args.whisper_bundle,
            model_dir: args.model_dir.clone(),
            model_cache_only: args.model_cache_only,
            task: if args.translation_model.is_some() || args.translation_bundle.is_some() {
                TranscriptionTask::Translate
            } else {
                TranscriptionTask::Transcribe
            },
            ..AsrConfig::default()
        },
        translation: translation_config(
            args.translation_model,
            args.translation_bundle,
            args.model_dir.clone(),
            args.model_cache_only,
            args.translation_source_language,
            args.translation_target_language,
            args.translation_max_new_tokens,
        ),
        vad: VadConfig::default(),
        alignment: alignment_config(
            args.no_align,
            args.alignment_model,
            args.alignment_bundle,
            args.model_dir,
            args.model_cache_only,
            args.interpolate_method,
            args.return_char_alignments,
        ),
        diarization: DiarizationConfig {
            enabled: args.speaker_embedding_bundle.is_some(),
            speaker_embedding_model_bundle: args.speaker_embedding_bundle,
            assignment_policy: args.speaker_assignment_policy.into(),
            ..DiarizationConfig::default()
        },
        output: OutputConfig::default(),
    };
    let request = build_transcription_request(&config)?;

    if config.translation.enabled {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "request": request,
                "translation": config.translation,
            }))?
        );
    } else {
        println!("{}", serde_json::to_string_pretty(&request)?);
    }
    Ok(())
}
