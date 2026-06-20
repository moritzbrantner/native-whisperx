use super::*;

pub(crate) fn transcribe_command(args: TranscribeArgs) -> anyhow::Result<()> {
    validate_transcribe_args(&args)?;
    let configs = args
        .input
        .iter()
        .cloned()
        .map(|input| transcribe_config(&args, input))
        .collect::<Vec<_>>();
    let reports = run_many(configs)?;

    if reports.len() == 1 {
        println!("{}", serde_json::to_string_pretty(&reports[0])?);
    } else {
        println!("{}", serde_json::to_string_pretty(&reports)?);
    }
    Ok(())
}

fn validate_transcribe_args(args: &TranscribeArgs) -> anyhow::Result<()> {
    validate_speaker_directory_args(&args.speaker_directory)?;
    let subtitle_layout_requested =
        args.highlight_words || args.max_line_width.is_some() || args.max_line_count.is_some();
    if args.no_align && subtitle_layout_requested {
        anyhow::bail!(
            "--highlight_words, --max_line_width, and --max_line_count require alignment; remove --no_align"
        );
    }
    if args.task == CliTask::Translate
        && args.provider == CliProvider::Native
        && args.translation_model.is_none()
        && args.translation_bundle.is_none()
    {
        anyhow::bail!(
            "native --task translate requires --translation-model or --translation-bundle; use --provider external-whisperx for WhisperX built-in translation"
        );
    }
    let native_pyannote_model = args.provider == CliProvider::Native
        && args
            .diarize_model
            .as_deref()
            .is_some_and(is_pyannote_diarization_model);
    if args.speaker_embeddings
        && args.provider == CliProvider::Native
        && !(native_pyannote_model && args.diarization_model_bundle.is_some())
    {
        anyhow::bail!(
            "native speaker embeddings require --diarize-model pyannote/... and --diarization-model-bundle"
        );
    }
    if native_pyannote_model && args.diarization_model_bundle.is_none() {
        anyhow::bail!("native pyannote diarization requires --diarization-model-bundle");
    }
    if args.provider == CliProvider::Native
        && args.diarization_model_bundle.is_some()
        && !native_pyannote_model
    {
        anyhow::bail!("native --diarization-model-bundle requires --diarize-model pyannote/...");
    }
    if args.basename.is_some() && args.input.len() > 1 {
        anyhow::bail!("--basename cannot be used with multiple input files");
    }
    Ok(())
}

fn transcribe_config(args: &TranscribeArgs, input: PathBuf) -> NativeWhisperxConfig {
    let output_dir = args.output_dir.clone();
    let provider = match args.provider {
        CliProvider::Native => AsrProvider::Native,
        CliProvider::ExternalWhisperx => AsrProvider::ExternalWhisperX,
    };
    let diarize = args.diarize
        || args.speaker_embeddings
        || args.diarization_model_bundle.is_some()
        || args.speaker_embedding_bundle.is_some()
        || args.min_speakers.is_some()
        || args.max_speakers.is_some();
    let diarize_model = args
        .diarize_model
        .clone()
        .unwrap_or_else(|| match args.provider {
            CliProvider::Native => DiarizationConfig::default().model_id,
            CliProvider::ExternalWhisperx => "pyannote/speaker-diarization-community-1".to_string(),
        });

    NativeWhisperxConfig {
        input: InputSource::Path { path: input },
        asr: AsrConfig {
            provider,
            task: args.task.into(),
            model_id: args.model.clone(),
            language: args.language.clone(),
            whisper_bundle: args.whisper_bundle.clone(),
            model_dir: args.model_dir.clone(),
            model_cache_only: args.model_cache_only,
            device: args.device.into(),
            device_index: args.device_index.clone(),
            compute_type: args.compute_type.clone(),
            batch_chunks: true,
            max_batch_size: args.batch_size,
            decode: decode_config(args),
            external_whisperx: ExternalWhisperxConfig {
                model: args.model.clone(),
                output_dir: output_dir.clone(),
                extra_args: logging_extra_args(args),
                ..ExternalWhisperxConfig::default()
            },
        },
        translation: translation_config(
            args.translation_model.clone(),
            args.translation_bundle.clone(),
            args.model_dir.clone(),
            args.model_cache_only,
            args.translation_source_language.clone(),
            args.translation_target_language.clone(),
            args.translation_max_new_tokens,
        ),
        vad: VadConfig {
            method: args.vad_method.into(),
            onset: args.vad_onset,
            offset: args.vad_offset,
            chunk_size: args.chunk_size,
            model_bundle: args.vad_model_bundle.clone(),
            model_file: args.vad_model_file.clone(),
            input_name: args.vad_input_name.clone(),
            output_name: args.vad_output_name.clone(),
            ..VadConfig::default()
        },
        alignment: alignment_config(
            args.no_align
                || args.task == CliTask::Translate
                    && args.provider == CliProvider::Native
                    && args.translation_model.is_none()
                    && args.translation_bundle.is_none(),
            args.alignment_model.clone(),
            args.alignment_bundle.clone(),
            args.model_dir.clone(),
            args.model_cache_only,
            args.interpolate_method,
            args.return_char_alignments,
        ),
        diarization: DiarizationConfig {
            enabled: diarize,
            model_id: diarize_model,
            hf_token: args.hf_token.clone(),
            return_speaker_embeddings: args.speaker_embeddings,
            model_bundle: args.diarization_model_bundle.clone(),
            manifest_file: args.diarization_manifest_file.clone(),
            segmentation_model_file: args.diarization_segmentation_model_file.clone(),
            embedding_model_file: args.diarization_embedding_model_file.clone(),
            plda_transform_file: args.diarization_plda_transform_file.clone(),
            plda_model_file: args.diarization_plda_model_file.clone(),
            clustering_config_file: args.diarization_clustering_config_file.clone(),
            speaker_embedding_model_bundle: args.speaker_embedding_bundle.clone(),
            speaker_embedding_model_file: args.speaker_embedding_model_file.clone(),
            speaker_embedding_dimension: args.speaker_embedding_dim,
            speaker_embedding_sample_rate: args.speaker_embedding_sample_rate,
            min_speakers: args.min_speakers,
            max_speakers: args.max_speakers,
            assignment_policy: args.speaker_assignment_policy.into(),
            speaker_directory: args
                .speaker_directory
                .clone()
                .try_into()
                .expect("transcribe args were validated"),
            disable_speaker_library: args.no_speaker_library || args.no_speaker_store,
            save_draft_speakers: !args.no_save_draft_speakers,
            use_draft_speakers: !args.no_use_draft_speakers,
            ..DiarizationConfig::default()
        },
        output: OutputConfig {
            output_dir,
            formats: args.formats.iter().copied().map(Into::into).collect(),
            basename: args.basename.clone(),
            pretty_json: true,
            subtitles: SubtitleConfig {
                max_line_width: args.max_line_width,
                max_line_count: args.max_line_count,
                highlight_words: args.highlight_words,
                segment_resolution: args.segment_resolution.into(),
            },
        },
    }
}

fn is_pyannote_diarization_model(model_id: &str) -> bool {
    model_id
        .trim()
        .to_ascii_lowercase()
        .starts_with("pyannote/")
}

fn decode_config(args: &TranscribeArgs) -> WhisperxDecodeConfig {
    WhisperxDecodeConfig {
        temperature: args.temperature.clone(),
        best_of: args.best_of,
        beam_size: args.beam_size,
        patience: args.patience,
        length_penalty: args.length_penalty,
        suppress_tokens: args.suppress_tokens.clone(),
        suppress_numerals: args.suppress_numerals,
        initial_prompt: args.initial_prompt.clone(),
        hotwords: args.hotwords.clone(),
        condition_on_previous_text: args.condition_on_previous_text,
        fp16: args.fp16,
        compression_ratio_threshold: args.compression_ratio_threshold,
        logprob_threshold: args.logprob_threshold,
        no_speech_threshold: args.no_speech_threshold,
        threads: args.threads,
    }
}

fn logging_extra_args(args: &TranscribeArgs) -> Vec<String> {
    let mut extra_args = Vec::new();
    if let Some(verbose) = &args.verbose {
        extra_args.extend(["--verbose".to_string(), verbose.clone()]);
    }
    if let Some(log_level) = &args.log_level {
        extra_args.extend(["--log-level".to_string(), log_level.clone()]);
    }
    if args.print_progress {
        extra_args.push("--print_progress".to_string());
    }
    extra_args
}

pub(crate) fn import_whisperx_command(args: ImportWhisperxArgs) -> anyhow::Result<()> {
    let bytes = fs::read(&args.whisperx_json)
        .with_context(|| format!("failed to read {}", args.whisperx_json.display()))?;
    let transcript = import_whisperx_json(&bytes)?;
    let json = serde_json::to_string_pretty(&transcript)?;
    if let Some(output) = args.output {
        fs::write(&output, json)
            .with_context(|| format!("failed to write {}", output.display()))?;
    } else {
        println!("{json}");
    }
    Ok(())
}
