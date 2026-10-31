//! `parity-goldens` command: plan, WhisperX arguments, and copy-out.

use super::*;

pub(crate) fn parity_goldens_command(args: ParityGoldensArgs) -> anyhow::Result<()> {
    ensure_whisperx_compat_enabled("Python WhisperX parity golden generation")?;
    let bytes = fs::read(&args.manifest)
        .with_context(|| format!("failed to read {}", args.manifest.display()))?;
    let mut suite: ParityFixtureSuite = serde_json::from_slice(&bytes)
        .with_context(|| format!("failed to parse {}", args.manifest.display()))?;
    let root = smoke_root_or_arg(args.root, "parity-goldens")?;
    let whisperx_command = absolute_from_cwd(args.whisperx_command)?;
    let model_dir = args
        .model_dir
        .map(absolute_from_cwd)
        .transpose()?
        .unwrap_or_else(|| root.join("models"));
    let filters = args.cases.iter().cloned().collect::<HashSet<_>>();
    let mut selected = Vec::new();

    for mut fixture in suite.fixtures.drain(..) {
        if !args.include_non_gating && !fixture.gating {
            continue;
        }
        if !filters.is_empty() && !filters.contains(&fixture.name) {
            continue;
        }
        if fixture.expected_json.is_none() && fixture.expected_outputs.is_empty() {
            continue;
        }
        fixture.input = resolve_cli_path_with_root(fixture.input, &root);
        fixture.expected_json = fixture
            .expected_json
            .take()
            .map(|path| resolve_cli_path_with_root(path, &root));
        for output in &mut fixture.expected_outputs {
            output.path = resolve_cli_path_with_root(output.path.clone(), &root);
        }
        selected.push(fixture);
    }

    for case_name in &filters {
        if !suite_case_name_exists(&selected, case_name) {
            anyhow::bail!("no golden-generating case named `{case_name}` matched the manifest");
        }
    }

    if selected.is_empty() {
        println!("No golden-generating cases matched.");
        return Ok(());
    }

    for fixture in selected {
        let plan = build_golden_plan(
            &fixture,
            &root,
            &whisperx_command,
            &model_dir,
            args.model_cache_only,
        )?;
        ensure_golden_targets_can_write(&plan, args.overwrite, args.dry_run)?;
        if args.dry_run {
            print_golden_plan(&plan);
            continue;
        }
        fs::create_dir_all(&plan.generated_dir)
            .with_context(|| format!("failed to create {}", plan.generated_dir.display()))?;
        let mut command = ProcessCommand::new(&plan.command);
        command.args(&plan.args);
        if let Some(token) = &plan.hf_token {
            command.env("HF_TOKEN", token);
        }
        let status = command
            .status()
            .with_context(|| format!("failed to run {}", plan.command.display()))?;
        if !status.success() {
            anyhow::bail!(
                "WhisperX golden generation for `{}` failed with status {status}",
                fixture.name
            );
        }
        copy_golden_outputs(&plan, args.overwrite)?;
    }

    Ok(())
}

pub(super) struct GoldenPlan {
    pub(super) case_name: String,
    pub(super) command: PathBuf,
    pub(super) args: Vec<String>,
    pub(super) generated_dir: PathBuf,
    pub(super) copies: Vec<GoldenCopy>,
    pub(super) hf_token: Option<String>,
}

#[derive(Debug)]
pub(super) struct GoldenCopy {
    pub(super) format: OutputFormat,
    pub(super) source: PathBuf,
    pub(super) target: PathBuf,
}

pub(super) fn build_golden_plan(
    fixture: &ParityFixtureCase,
    root: &Path,
    whisperx_command: &Path,
    model_dir: &Path,
    model_cache_only: bool,
) -> anyhow::Result<GoldenPlan> {
    let generated_dir = root
        .join("expected")
        .join("whisperx-3.8.6")
        .join("generated")
        .join(&fixture.name);
    let requested_formats = golden_requested_formats(fixture)?;
    let output_format = golden_output_format(fixture, &requested_formats);
    let input_stem = fixture
        .input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .with_context(|| format!("input {} has no UTF-8 file stem", fixture.input.display()))?;

    let mut args = vec![
        fixture.input.display().to_string(),
        "--model".to_string(),
        fixture.whisperx.model.clone(),
        "--model_dir".to_string(),
        model_dir.display().to_string(),
    ];
    if model_cache_only
        || fixture.native_asr.model_cache_only
        || fixture.alignment.model_cache_only
        || fixture.translation.model_cache_only
    {
        args.extend(["--model_cache_only".to_string(), "True".to_string()]);
    }
    if let Some(language) = &fixture.language {
        args.extend(["--language".to_string(), language.clone()]);
    }
    match fixture.native_asr.device {
        DevicePreference::Auto => {}
        DevicePreference::Cpu => args.extend(["--device".to_string(), "cpu".to_string()]),
        DevicePreference::Cuda => args.extend(["--device".to_string(), "cuda".to_string()]),
    }
    if let Some(device_index) = &fixture.native_asr.device_index {
        args.extend(["--device_index".to_string(), device_index.clone()]);
    }
    if let Some(compute_type) = fixture
        .native_asr
        .compute_type
        .as_ref()
        .or(fixture.whisperx.compute_type.as_ref())
    {
        args.extend(["--compute_type".to_string(), compute_type.clone()]);
    }
    if let Some(batch_size) = fixture
        .native_asr
        .max_batch_size
        .or(fixture.whisperx.batch_size)
    {
        args.extend(["--batch_size".to_string(), batch_size.to_string()]);
    }
    args.extend(["--output_format".to_string(), output_format.to_string()]);
    args.extend([
        "--output_dir".to_string(),
        generated_dir.display().to_string(),
    ]);
    push_golden_args(fixture, &mut args)?;

    let mut copies = Vec::new();
    if let Some(expected_json) = &fixture.expected_json {
        copies.push(GoldenCopy {
            format: OutputFormat::Json,
            source: generated_dir.join(format!("{input_stem}.json")),
            target: expected_json.clone(),
        });
    }
    for expected_output in &fixture.expected_outputs {
        copies.push(GoldenCopy {
            format: expected_output.format,
            source: generated_dir.join(format!(
                "{input_stem}.{}",
                expected_output.format.extension()
            )),
            target: expected_output.path.clone(),
        });
    }
    copies = dedup_copies(copies);

    let command = fixture
        .whisperx
        .command_wrapper
        .as_ref()
        .map(|path| resolve_cli_path_with_root(path.clone(), root))
        .unwrap_or_else(|| whisperx_command.to_path_buf());
    if fixture.whisperx.command_wrapper.is_some() {
        args.extend([
            "--wrapped-command".to_string(),
            whisperx_command.display().to_string(),
        ]);
    }

    Ok(GoldenPlan {
        case_name: fixture.name.clone(),
        command,
        args,
        generated_dir,
        copies,
        hf_token: golden_hf_token(fixture),
    })
}

pub(super) fn golden_requested_formats(
    fixture: &ParityFixtureCase,
) -> anyhow::Result<Vec<OutputFormat>> {
    let mut formats = Vec::new();
    if fixture.expected_json.is_some() {
        formats.push(OutputFormat::Json);
    }
    for ExpectedOutputFile { format, .. } in &fixture.expected_outputs {
        if *format == OutputFormat::NativeJson {
            anyhow::bail!(
                "case `{}` requests native-json, which Python WhisperX cannot generate",
                fixture.name
            );
        }
        formats.push(*format);
    }
    Ok(formats)
}

pub(super) fn golden_output_format(
    fixture: &ParityFixtureCase,
    formats: &[OutputFormat],
) -> &'static str {
    if fixture.output.formats.contains(&OutputFormat::All)
        || formats.contains(&OutputFormat::All)
        || formats.len() > 1
    {
        "all"
    } else {
        formats
            .first()
            .copied()
            .unwrap_or(OutputFormat::Json)
            .as_transcription_format()
    }
}

pub(super) fn push_golden_args(
    fixture: &ParityFixtureCase,
    args: &mut Vec<String>,
) -> anyhow::Result<()> {
    args.extend([
        "--task".to_string(),
        fixture.native_asr.task.as_whisperx_arg().to_string(),
    ]);
    if !fixture.alignment.enabled {
        args.push("--no_align".to_string());
    } else {
        args.extend([
            "--align_model".to_string(),
            fixture
                .whisperx
                .align_model
                .clone()
                .unwrap_or_else(|| fixture.alignment.model_id.clone()),
        ]);
        if fixture.alignment.return_char_alignments {
            args.push("--return_char_alignments".to_string());
        }
    }
    if fixture.vad.method != VadMethod::Energy {
        args.extend([
            "--vad_method".to_string(),
            fixture.vad.method.as_whisperx_arg().to_string(),
        ]);
    }
    push_cli_arg_display(args, "--vad_onset", fixture.vad.onset);
    push_cli_arg_display(args, "--vad_offset", fixture.vad.offset);
    push_cli_arg_display(args, "--chunk_size", fixture.vad.chunk_size);

    let decode = &fixture.native_asr.decode;
    if !decode.temperature.is_empty() {
        args.extend([
            "--temperature".to_string(),
            decode
                .temperature
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ]);
    }
    push_cli_arg_display(args, "--best_of", decode.best_of);
    push_cli_arg_display(args, "--beam_size", decode.beam_size);
    push_cli_arg_display(args, "--patience", decode.patience);
    push_cli_arg_display(args, "--length_penalty", decode.length_penalty);
    push_cli_arg(args, "--suppress_tokens", decode.suppress_tokens.as_deref());
    if decode.suppress_numerals {
        args.push("--suppress_numerals".to_string());
    }
    push_cli_arg(args, "--initial_prompt", decode.initial_prompt.as_deref());
    push_cli_arg(args, "--hotwords", decode.hotwords.as_deref());
    push_cli_arg_bool(
        args,
        "--condition_on_previous_text",
        decode.condition_on_previous_text,
    );
    push_cli_arg_bool(args, "--fp16", decode.fp16);
    push_cli_arg_display(
        args,
        "--compression_ratio_threshold",
        decode.compression_ratio_threshold,
    );
    push_cli_arg_display(args, "--logprob_threshold", decode.logprob_threshold);
    push_cli_arg_display(args, "--no_speech_threshold", decode.no_speech_threshold);
    push_cli_arg_display(args, "--threads", decode.threads);

    let whisperx_diarization = fixture
        .whisperx_diarization
        .as_ref()
        .unwrap_or(&fixture.diarization);
    if whisperx_diarization.enabled {
        args.push("--diarize".to_string());
        args.extend([
            "--diarize_model".to_string(),
            whisperx_diarization.model_id.clone(),
        ]);
        push_cli_arg_display(args, "--min_speakers", whisperx_diarization.min_speakers);
        push_cli_arg_display(args, "--max_speakers", whisperx_diarization.max_speakers);
    }
    if whisperx_diarization.return_speaker_embeddings {
        args.push("--speaker_embeddings".to_string());
    }
    push_cli_arg_display(
        args,
        "--max_line_width",
        fixture.output.subtitles.max_line_width,
    );
    push_cli_arg_display(
        args,
        "--max_line_count",
        fixture.output.subtitles.max_line_count,
    );
    if fixture.output.subtitles.highlight_words {
        args.extend(["--highlight_words".to_string(), "True".to_string()]);
    }
    args.extend([
        "--segment_resolution".to_string(),
        match fixture.output.subtitles.segment_resolution {
            SegmentResolution::Sentence => "sentence",
            SegmentResolution::Chunk => "chunk",
        }
        .to_string(),
    ]);
    args.extend(fixture.whisperx.extra_args.clone());
    Ok(())
}

pub(super) fn golden_hf_token(fixture: &ParityFixtureCase) -> Option<String> {
    let whisperx_diarization = fixture
        .whisperx_diarization
        .as_ref()
        .unwrap_or(&fixture.diarization);
    if !whisperx_diarization.enabled {
        return None;
    }
    fixture
        .whisperx_diarization
        .as_ref()
        .and_then(|diarization| diarization.hf_token.clone())
        .or_else(|| whisperx_diarization.hf_token.clone())
        .or_else(|| {
            whisperx_diarization
                .hf_token_env
                .as_ref()
                .and_then(|name| std::env::var(name).ok())
        })
        .or_else(|| {
            fixture
                .diarization
                .hf_token_env
                .as_ref()
                .and_then(|name| std::env::var(name).ok())
        })
        .or_else(|| {
            fixture
                .whisperx
                .hf_token_env
                .as_ref()
                .and_then(|name| std::env::var(name).ok())
        })
}

pub(super) fn push_cli_arg(args: &mut Vec<String>, flag: &str, value: Option<&str>) {
    if let Some(value) = value {
        args.extend([flag.to_string(), value.to_string()]);
    }
}

pub(super) fn push_cli_arg_display<T: std::fmt::Display>(
    args: &mut Vec<String>,
    flag: &str,
    value: Option<T>,
) {
    if let Some(value) = value {
        args.extend([flag.to_string(), value.to_string()]);
    }
}

pub(super) fn push_cli_arg_bool(args: &mut Vec<String>, flag: &str, value: Option<bool>) {
    if let Some(value) = value {
        args.extend([
            flag.to_string(),
            if value { "True" } else { "False" }.to_string(),
        ]);
    }
}

pub(super) fn dedup_copies(copies: Vec<GoldenCopy>) -> Vec<GoldenCopy> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for copy in copies {
        if seen.insert(copy.target.clone()) {
            deduped.push(copy);
        }
    }
    deduped
}

pub(super) fn ensure_golden_targets_can_write(
    plan: &GoldenPlan,
    overwrite: bool,
    dry_run: bool,
) -> anyhow::Result<()> {
    if overwrite || dry_run {
        return Ok(());
    }
    for copy in &plan.copies {
        if copy.target.exists() {
            anyhow::bail!(
                "refusing to overwrite existing golden {}; pass --overwrite",
                copy.target.display()
            );
        }
    }
    Ok(())
}

pub(super) fn copy_golden_outputs(plan: &GoldenPlan, overwrite: bool) -> anyhow::Result<()> {
    for copy in &plan.copies {
        if copy.target.exists() && !overwrite {
            anyhow::bail!(
                "refusing to overwrite existing golden {}; pass --overwrite",
                copy.target.display()
            );
        }
        let parent = copy
            .target
            .parent()
            .with_context(|| format!("target {} has no parent", copy.target.display()))?;
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
        fs::copy(&copy.source, &copy.target).with_context(|| {
            format!(
                "failed to copy generated {} output from {} to {}",
                copy.format.as_transcription_format(),
                copy.source.display(),
                copy.target.display()
            )
        })?;
    }
    Ok(())
}

pub(super) fn print_golden_plan(plan: &GoldenPlan) {
    println!("case: {}", plan.case_name);
    println!("command: {}", shell_command(&plan.command, &plan.args));
    for copy in &plan.copies {
        println!(
            "target: {} <= {}",
            copy.target.display(),
            copy.source.display()
        );
    }
}

pub(super) fn shell_command(command: &Path, args: &[String]) -> String {
    std::iter::once(shell_quote(&command.display().to_string()))
        .chain(args.iter().enumerate().map(|(index, arg)| {
            if index > 0 && args[index - 1] == "--hf_token" {
                "<redacted>".to_string()
            } else {
                shell_quote(arg)
            }
        }))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn shell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || "-_./:=,".contains(character))
    {
        return value.to_string();
    }
    format!("'{}'", value.replace('\'', "'\\''"))
}
