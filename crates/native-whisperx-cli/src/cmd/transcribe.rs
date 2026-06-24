//! Transcribe command configuration, input expansion, and run dispatch.

use super::*;
use std::collections::{HashMap, HashSet};
use std::io::IsTerminal;

use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use native_whisperx::ConfigSelection;

use crate::CliVadMethod;

pub(crate) fn transcribe_command(mut args: TranscribeArgs) -> anyhow::Result<()> {
    args.input = expand_transcribe_inputs(&args.input)?;
    validate_transcribe_args(&args)?;
    validate_explicit_output_dir_collisions(&args)?;
    let configs = args
        .input
        .iter()
        .cloned()
        .map(|input| transcribe_config(&args, input))
        .collect::<Vec<_>>();

    let reports = if args.provider == CliProvider::Native {
        let mut progress = transcribe_progress_observer();
        run_many_with_observer(configs, progress.as_mut())?
    } else {
        run_many(configs)?
    };

    if let Some(report) = &args.report {
        write_transcribe_report(report, &reports)?;
    } else if args.provider == CliProvider::ExternalWhisperx {
        print_transcribe_report(&reports)?;
    }
    Ok(())
}

fn print_transcribe_report(reports: &[NativeWhisperxReport]) -> anyhow::Result<()> {
    if reports.len() == 1 {
        println!("{}", serde_json::to_string_pretty(&reports[0])?);
    } else {
        println!("{}", serde_json::to_string_pretty(reports)?);
    }
    Ok(())
}

fn write_transcribe_report(path: &Path, reports: &[NativeWhisperxReport]) -> anyhow::Result<()> {
    let mut json = if reports.len() == 1 {
        serde_json::to_string_pretty(&reports[0])?
    } else {
        serde_json::to_string_pretty(reports)?
    };
    json.push('\n');
    fs::write(path, json).with_context(|| format!("failed to write report {}", path.display()))
}

fn transcribe_progress_observer() -> Box<dyn TranscriptionProgressObserver> {
    transcribe_progress_observer_for(progress_renderer_kind(std::io::stdout().is_terminal()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProgressRendererKind {
    Indicatif,
    Plain,
}

fn progress_renderer_kind(stdout_is_terminal: bool) -> ProgressRendererKind {
    if stdout_is_terminal {
        ProgressRendererKind::Indicatif
    } else {
        ProgressRendererKind::Plain
    }
}

fn transcribe_progress_observer_for(
    kind: ProgressRendererKind,
) -> Box<dyn TranscriptionProgressObserver> {
    match kind {
        ProgressRendererKind::Indicatif => Box::new(IndicatifTranscribeProgress::new()),
        ProgressRendererKind::Plain => Box::new(PlainTranscribeProgress),
    }
}

struct PlainTranscribeProgress;

impl TranscriptionProgressObserver for PlainTranscribeProgress {
    fn observe(&mut self, event: TranscriptionProgressEvent) {
        println!("{}", plain_progress_line(&event));
    }
}

struct IndicatifTranscribeProgress {
    bar: ProgressBar,
}

impl IndicatifTranscribeProgress {
    fn new() -> Self {
        let bar = ProgressBar::with_draw_target(None, ProgressDrawTarget::stdout());
        if let Ok(style) = ProgressStyle::with_template("{spinner} {msg}") {
            bar.set_style(style);
        }
        bar.enable_steady_tick(Duration::from_millis(100));
        Self { bar }
    }
}

impl TranscriptionProgressObserver for IndicatifTranscribeProgress {
    fn observe(&mut self, event: TranscriptionProgressEvent) {
        let line = plain_progress_line(&event);
        match event {
            TranscriptionProgressEvent::RunEnd { .. } => {
                self.bar.finish_with_message(line);
            }
            TranscriptionProgressEvent::Failure { .. } => {
                self.bar.abandon_with_message(line);
            }
            _ => {
                self.bar.set_message(line);
            }
        }
    }
}

fn plain_progress_line(event: &TranscriptionProgressEvent) -> String {
    match event {
        TranscriptionProgressEvent::RunStart { total_files } => {
            format!("progress run start total_files={total_files}")
        }
        TranscriptionProgressEvent::RunEnd {
            total_files,
            duration_seconds,
        } => format!(
            "progress run complete total_files={total_files} elapsed={:.3}s",
            duration_seconds
        ),
        TranscriptionProgressEvent::FileStart {
            file_index,
            total_files,
            input,
        } => format!(
            "progress file start index={}/{} input={}",
            file_index + 1,
            total_files,
            input.display()
        ),
        TranscriptionProgressEvent::FileEnd {
            file_index,
            total_files,
            input,
            duration_seconds,
        } => format!(
            "progress file complete index={}/{} input={} elapsed={:.3}s",
            file_index + 1,
            total_files,
            input.display(),
            duration_seconds
        ),
        TranscriptionProgressEvent::TaskStart { file_index, task } => format!(
            "progress task start file={} task={}",
            file_index + 1,
            progress_task_name(*task)
        ),
        TranscriptionProgressEvent::TaskEnd {
            file_index,
            task,
            duration_seconds,
        } => format!(
            "progress task complete file={} task={} elapsed={:.3}s",
            file_index + 1,
            progress_task_name(*task),
            duration_seconds
        ),
        TranscriptionProgressEvent::ModelLoadStart {
            file_index,
            task,
            provider,
            model_id,
        } => format!(
            "progress model load-start file={} task={} provider={} model={}",
            file_index + 1,
            progress_task_name(*task),
            provider,
            model_id
        ),
        TranscriptionProgressEvent::ModelLoadEnd {
            file_index,
            task,
            provider,
            model_id,
            duration_seconds,
        } => format!(
            "progress model load-complete file={} task={} provider={} model={} elapsed={:.3}s",
            file_index + 1,
            progress_task_name(*task),
            provider,
            model_id,
            duration_seconds
        ),
        TranscriptionProgressEvent::ModelReuse {
            file_index,
            task,
            provider,
            model_id,
        } => format!(
            "progress model reuse file={} task={} provider={} model={}",
            file_index + 1,
            progress_task_name(*task),
            provider,
            model_id
        ),
        TranscriptionProgressEvent::Failure {
            file_index,
            input,
            task,
            duration_seconds,
            message,
        } => {
            let task = task.map(progress_task_name).unwrap_or("none");
            format!(
                "progress failure file={} input={} task={} elapsed={:.3}s message={}",
                file_index + 1,
                input.display(),
                task,
                duration_seconds,
                message
            )
        }
    }
}

fn progress_task_name(task: TranscriptionProgressTask) -> &'static str {
    match task {
        TranscriptionProgressTask::Decode => "decode",
        TranscriptionProgressTask::Vad => "vad",
        TranscriptionProgressTask::Asr => "asr",
        TranscriptionProgressTask::Alignment => "alignment",
        TranscriptionProgressTask::Diarization => "diarization",
        TranscriptionProgressTask::Translation => "translation",
        TranscriptionProgressTask::Output => "output",
    }
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

fn expand_transcribe_inputs(inputs: &[PathBuf]) -> anyhow::Result<Vec<PathBuf>> {
    let mut expanded = Vec::new();
    let mut seen = HashSet::new();

    for input in inputs {
        if input.is_file() {
            push_unique_input(&mut expanded, &mut seen, input.clone())?;
        } else if is_glob_pattern(input) {
            let pattern = input.to_string_lossy();
            let mut matches = glob::glob(&pattern)
                .with_context(|| format!("invalid input pattern `{pattern}`"))?
                .map(|entry| {
                    entry.with_context(|| format!("failed to read input pattern `{pattern}` match"))
                })
                .collect::<anyhow::Result<Vec<_>>>()?;
            matches.sort();
            if matches.is_empty() {
                anyhow::bail!("input pattern `{pattern}` matched no input files");
            }

            for matched in matches {
                if !matched.is_file() {
                    anyhow::bail!(
                        "input pattern `{pattern}` matched non-file input `{}`",
                        matched.display()
                    );
                }
                push_unique_input(&mut expanded, &mut seen, matched)?;
            }
        } else {
            push_unique_input(&mut expanded, &mut seen, input.clone())?;
        }
    }

    Ok(expanded)
}

fn is_glob_pattern(path: &Path) -> bool {
    path.to_string_lossy()
        .chars()
        .any(|character| matches!(character, '*' | '?' | '['))
}

fn push_unique_input(
    expanded: &mut Vec<PathBuf>,
    seen: &mut HashSet<PathBuf>,
    input: PathBuf,
) -> anyhow::Result<()> {
    let dedupe_key = input
        .canonicalize()
        .unwrap_or_else(|_| absolute_from_cwd(input.clone()).unwrap_or_else(|_| input.clone()));
    if seen.insert(dedupe_key) {
        expanded.push(input);
    }
    Ok(())
}

fn validate_explicit_output_dir_collisions(args: &TranscribeArgs) -> anyhow::Result<()> {
    if args.output_dir.is_none() {
        return Ok(());
    }

    let mut by_basename: HashMap<String, PathBuf> = HashMap::new();
    for input in &args.input {
        let basename = input
            .file_stem()
            .and_then(|stem| stem.to_str())
            .filter(|stem| !stem.is_empty())
            .unwrap_or("transcript")
            .to_string();
        if let Some(previous) = by_basename.insert(basename.clone(), input.clone()) {
            anyhow::bail!(
                "output basename collision `{basename}` for inputs `{}` and `{}`; choose distinct input filenames or omit --output-dir to write beside each input",
                previous.display(),
                input.display()
            );
        }
    }

    Ok(())
}

fn transcribe_config(args: &TranscribeArgs, input: PathBuf) -> NativeWhisperxConfig {
    let output_dir = transcribe_output_dir(args, &input);
    let provider = match args.provider {
        CliProvider::Native => AsrProvider::Native,
        CliProvider::ExternalWhisperx => AsrProvider::ExternalWhisperX,
    };
    let external_output_dir = match args.provider {
        CliProvider::ExternalWhisperx if args.output_dir.is_none() => {
            Some(unique_external_whisperx_output_dir())
        }
        CliProvider::ExternalWhisperx => output_dir.clone(),
        CliProvider::Native => None,
    };
    let diarize = args.diarize
        || args.speaker_embeddings
        || args.diarization_model_bundle.is_some()
        || args.speaker_embedding_bundle.is_some()
        || args.min_speakers.is_some()
        || args.max_speakers.is_some();
    let diarization_model_selection = diarization_model_selection(args, diarize);
    let diarize_model = args.diarize_model.clone().unwrap_or_else(|| {
        if diarization_model_selection.is_automatic() {
            DiarizationConfig::default().model_id
        } else {
            match args.provider {
                CliProvider::Native => DiarizationConfig::default().model_id,
                CliProvider::ExternalWhisperx => {
                    "pyannote/speaker-diarization-community-1".to_string()
                }
            }
        }
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
                output_dir: external_output_dir,
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
            method: vad_method(args),
            selection: vad_selection(args),
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
            model_selection: diarization_model_selection,
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

fn vad_method(args: &TranscribeArgs) -> VadMethod {
    match args.vad_method {
        CliVadMethod::Auto => VadMethod::Energy,
        method => method.into(),
    }
}

fn vad_selection(args: &TranscribeArgs) -> ConfigSelection {
    if args.vad_method == CliVadMethod::Auto && !has_explicit_vad_resource_args(args) {
        ConfigSelection::Automatic
    } else {
        ConfigSelection::Explicit
    }
}

fn has_explicit_vad_resource_args(args: &TranscribeArgs) -> bool {
    args.vad_model_bundle.is_some()
        || args.vad_model_file.is_some()
        || args.vad_input_name.is_some()
        || args.vad_output_name.is_some()
}

fn diarization_model_selection(args: &TranscribeArgs, diarize: bool) -> ConfigSelection {
    if args.provider == CliProvider::Native && diarize && !has_explicit_diarization_model_args(args)
    {
        ConfigSelection::Automatic
    } else {
        ConfigSelection::Explicit
    }
}

fn has_explicit_diarization_model_args(args: &TranscribeArgs) -> bool {
    args.diarize_model.is_some()
        || args.diarization_model_bundle.is_some()
        || args.diarization_manifest_file.is_some()
        || args.diarization_segmentation_model_file.is_some()
        || args.diarization_embedding_model_file.is_some()
        || args.diarization_plda_transform_file.is_some()
        || args.diarization_plda_model_file.is_some()
        || args.diarization_clustering_config_file.is_some()
        || args.speaker_embedding_bundle.is_some()
        || args.speaker_embedding_model_file.is_some()
        || args.speaker_embedding_dim.is_some()
        || args.speaker_embedding_sample_rate.is_some()
}

fn transcribe_output_dir(args: &TranscribeArgs, input: &Path) -> Option<PathBuf> {
    args.output_dir.clone().or_else(|| {
        input
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map(Path::to_path_buf)
            .or_else(|| Some(PathBuf::from(".")))
    })
}

fn unique_external_whisperx_output_dir() -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    std::env::temp_dir().join(format!(
        "native-whisperx-external-{}-{millis}",
        std::process::id()
    ))
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

#[cfg(test)]
mod tests {
    use super::{progress_renderer_kind, ProgressRendererKind};

    #[test]
    fn progress_renderer_uses_indicatif_for_terminal_stdout() {
        assert_eq!(
            progress_renderer_kind(true),
            ProgressRendererKind::Indicatif
        );
    }

    #[test]
    fn progress_renderer_uses_plain_lines_for_redirected_stdout() {
        assert_eq!(progress_renderer_kind(false), ProgressRendererKind::Plain);
    }
}
