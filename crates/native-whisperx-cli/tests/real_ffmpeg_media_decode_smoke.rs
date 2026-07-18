use assert_cmd::Command;
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;
use std::time::{Duration, Instant};

const RUN_ENV: &str = "RUN_NATIVE_FFMPEG_MEDIA_DECODE_SMOKE";
const EVIDENCE_REPORT_ENV: &str = "NATIVE_FFMPEG_MEDIA_EVIDENCE_REPORT";
const SEGMENT_TIMING_TOLERANCE_SECONDS: f64 = 0.25;

#[test]
fn finite_media_evidence_scope_covers_every_supported_common_container() {
    let extensions = finite_media_fixtures()
        .iter()
        .map(|fixture| fixture.extension)
        .collect::<Vec<_>>();

    assert_eq!(
        extensions,
        ["mp3", "m4a", "aac", "flac", "ogg", "opus", "mp4", "mov", "mkv", "webm",]
    );
}

#[test]
fn ffmpeg_preflight_reports_all_missing_tools_codecs_and_muxers() {
    let preflight = FfmpegPreflight {
        missing_tools: ["ffmpeg", "ffprobe"]
            .into_iter()
            .map(str::to_string)
            .collect(),
        missing_encoders: ["aac", "libopus"].into_iter().map(str::to_string).collect(),
        missing_muxers: ["adts", "webm"].into_iter().map(str::to_string).collect(),
    };

    let error = preflight.actionable_error().expect("preflight should fail");
    for expected in [
        "missing tools: ffmpeg, ffprobe",
        "missing encoders: aac, libopus",
        "missing muxers: adts, webm",
        "ffmpeg -hide_banner -encoders",
        "ffmpeg -hide_banner -muxers",
    ] {
        assert!(
            error.contains(expected),
            "preflight error should contain '{expected}': {error}"
        );
    }
}

#[test]
fn ffmpeg_preflight_rejects_unknown_capability_help() {
    assert!(ffmpeg_capability_help_available(
        "encoder",
        "libopus",
        "Encoder libopus [libopus Opus]:"
    ));
    assert!(!ffmpeg_capability_help_available(
        "encoder",
        "libopus",
        "Codec 'libopus' is not recognized by FFmpeg."
    ));
    assert!(ffmpeg_capability_help_available(
        "muxer",
        "webm",
        "Muxer webm [WebM]:"
    ));
    assert!(!ffmpeg_capability_help_available(
        "muxer",
        "webm",
        "Unknown format 'webm'."
    ));
}

#[test]
#[ignore = "requires explicit opt-in, a spoken WAV, cached Whisper model, ffmpeg, and ffprobe"]
fn generated_common_media_containers_complete_native_workflow_with_equivalent_results() {
    if std::env::var(RUN_ENV).ok().as_deref() != Some("1") {
        eprintln!("set {RUN_ENV}=1 to run the finite media evidence smoke");
        return;
    }

    let smoke_root = smoke_root();
    let source_wav = smoke_root.join("audio/native-transcription-smoke.wav");
    let model_dir = smoke_root.join("models");
    assert!(
        source_wav.is_file(),
        "finite media evidence requires a spoken WAV at {}; set SMOKE_ROOT to the resource root",
        source_wav.display()
    );
    assert!(
        model_dir.is_dir(),
        "finite media evidence requires a cached tiny.en model under {}; set SMOKE_ROOT to the resource root",
        model_dir.display()
    );

    let fixtures = finite_media_fixtures();
    let preflight = inspect_ffmpeg_preflight(&fixtures);
    if let Some(error) = preflight.actionable_error() {
        panic!("{error}");
    }

    let temp = tempfile::tempdir().expect("finite media evidence tempdir");
    let media_dir = temp.path().join("media");
    let output_dir = temp.path().join("out");
    fs::create_dir_all(&media_dir).expect("media fixture directory");
    fs::create_dir_all(&output_dir).expect("workflow output directory");

    let (baseline_report, baseline_elapsed) =
        run_native_workflow(&source_wav, &model_dir, &output_dir);
    assert_non_empty_real_transcript(&baseline_report, &source_wav);
    let baseline_evidence =
        workflow_evidence("wav", &source_wav, &baseline_report, baseline_elapsed, None);

    let mut cases = Vec::with_capacity(fixtures.len());
    for fixture in &fixtures {
        let input = media_dir.join(format!(
            "spoken-{}.{}",
            fixture.extension, fixture.extension
        ));
        fixture.generate_from_spoken_wav(&source_wav, &input);

        let (report, elapsed) = run_native_workflow(&input, &model_dir, &output_dir);
        let comparison = compare_to_wav_baseline(&baseline_report, &report);
        cases.push(workflow_evidence(
            fixture.extension,
            &input,
            &report,
            elapsed,
            Some(comparison),
        ));
    }

    let passed = cases
        .iter()
        .all(|case| case.get("passed").and_then(Value::as_bool) == Some(true));
    let evidence = json!({
        "schemaVersion": 1,
        "sourceWav": source_wav,
        "model": "tiny.en",
        "segmentTimingToleranceSeconds": SEGMENT_TIMING_TOLERANCE_SECONDS,
        "passed": passed,
        "baseline": baseline_evidence,
        "cases": cases,
    });
    record_evidence(&evidence);

    assert!(
        passed,
        "one or more generated media workflows differed from the spoken WAV baseline; inspect the emitted finite media evidence report"
    );
}

fn finite_media_fixtures() -> Vec<MediaFixture> {
    vec![
        MediaFixture::audio("mp3", "libmp3lame", "mp3", &["-c:a", "libmp3lame"]),
        MediaFixture::audio("m4a", "aac", "ipod", &["-c:a", "aac"]),
        MediaFixture::audio("aac", "aac", "adts", &["-c:a", "aac", "-f", "adts"]),
        MediaFixture::audio("flac", "flac", "flac", &["-c:a", "flac"]),
        MediaFixture::audio("ogg", "libvorbis", "ogg", &["-c:a", "libvorbis"]),
        MediaFixture::audio("opus", "libopus", "ogg", &["-c:a", "libopus"]),
        MediaFixture::video(
            "mp4",
            &["mpeg4", "aac"],
            "mp4",
            &["-c:v", "mpeg4", "-c:a", "aac"],
        ),
        MediaFixture::video(
            "mov",
            &["mpeg4", "aac"],
            "mov",
            &["-c:v", "mpeg4", "-c:a", "aac"],
        ),
        MediaFixture::video(
            "mkv",
            &["mpeg4", "aac"],
            "matroska",
            &["-c:v", "mpeg4", "-c:a", "aac"],
        ),
        MediaFixture::video(
            "webm",
            &["libvpx", "libopus"],
            "webm",
            &["-c:v", "libvpx", "-c:a", "libopus"],
        ),
    ]
}

struct MediaFixture {
    extension: &'static str,
    kind: FixtureKind,
    encoders: Vec<&'static str>,
    muxer: &'static str,
    codec_args: &'static [&'static str],
}

impl MediaFixture {
    fn audio(
        extension: &'static str,
        encoder: &'static str,
        muxer: &'static str,
        codec_args: &'static [&'static str],
    ) -> Self {
        Self {
            extension,
            kind: FixtureKind::Audio,
            encoders: vec![encoder],
            muxer,
            codec_args,
        }
    }

    fn video(
        extension: &'static str,
        encoders: &'static [&'static str],
        muxer: &'static str,
        codec_args: &'static [&'static str],
    ) -> Self {
        Self {
            extension,
            kind: FixtureKind::Video,
            encoders: encoders.to_vec(),
            muxer,
            codec_args,
        }
    }

    fn generate_from_spoken_wav(&self, source_wav: &Path, output_path: &Path) {
        let mut command = ProcessCommand::new("ffmpeg");
        command
            .args(["-hide_banner", "-loglevel", "error", "-y", "-i"])
            .arg(source_wav);
        match self.kind {
            FixtureKind::Audio => {
                command.args(["-map", "0:a:0"]);
            }
            FixtureKind::Video => {
                command.args([
                    "-f",
                    "lavfi",
                    "-i",
                    "color=c=black:s=16x16:r=1",
                    "-map",
                    "1:v:0",
                    "-map",
                    "0:a:0",
                    "-shortest",
                    "-pix_fmt",
                    "yuv420p",
                ]);
            }
        }
        command
            .args(["-ar", "16000", "-ac", "1"])
            .args(self.codec_args)
            .arg(output_path);

        let output = command.output().expect("ffmpeg fixture generation");
        assert!(
            output.status.success(),
            "ffmpeg failed to generate {} from the spoken WAV at {}\nstderr:\n{}",
            self.extension,
            output_path.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[derive(Clone, Copy)]
enum FixtureKind {
    Audio,
    Video,
}

#[derive(Default)]
struct FfmpegPreflight {
    missing_tools: BTreeSet<String>,
    missing_encoders: BTreeSet<String>,
    missing_muxers: BTreeSet<String>,
}

impl FfmpegPreflight {
    fn actionable_error(&self) -> Option<String> {
        if self.missing_tools.is_empty()
            && self.missing_encoders.is_empty()
            && self.missing_muxers.is_empty()
        {
            return None;
        }

        let mut missing = Vec::new();
        if !self.missing_tools.is_empty() {
            missing.push(format!(
                "missing tools: {}",
                join_names(&self.missing_tools)
            ));
        }
        if !self.missing_encoders.is_empty() {
            missing.push(format!(
                "missing encoders: {}",
                join_names(&self.missing_encoders)
            ));
        }
        if !self.missing_muxers.is_empty() {
            missing.push(format!(
                "missing muxers: {}",
                join_names(&self.missing_muxers)
            ));
        }
        Some(format!(
            "finite media evidence FFmpeg preflight failed ({})\nInstall ffmpeg and ffprobe on PATH with the listed codec/container support. Inspect this installation with 'ffmpeg -hide_banner -encoders' and 'ffmpeg -hide_banner -muxers'.",
            missing.join("; ")
        ))
    }
}

fn inspect_ffmpeg_preflight(fixtures: &[MediaFixture]) -> FfmpegPreflight {
    let mut preflight = FfmpegPreflight::default();
    for tool in ["ffmpeg", "ffprobe"] {
        if !runtime_tool_available(tool) {
            preflight.missing_tools.insert(tool.to_string());
        }
    }

    let encoders = fixtures
        .iter()
        .flat_map(|fixture| fixture.encoders.iter().copied())
        .collect::<BTreeSet<_>>();
    let muxers = fixtures
        .iter()
        .map(|fixture| fixture.muxer)
        .collect::<BTreeSet<_>>();
    if !preflight.missing_tools.contains("ffmpeg") {
        for encoder in encoders {
            if !ffmpeg_capability_available("encoder", encoder) {
                preflight.missing_encoders.insert(encoder.to_string());
            }
        }
        for muxer in muxers {
            if !ffmpeg_capability_available("muxer", muxer) {
                preflight.missing_muxers.insert(muxer.to_string());
            }
        }
    } else {
        preflight
            .missing_encoders
            .extend(encoders.into_iter().map(str::to_string));
        preflight
            .missing_muxers
            .extend(muxers.into_iter().map(str::to_string));
    }
    preflight
}

fn runtime_tool_available(tool: &str) -> bool {
    ProcessCommand::new(tool)
        .arg("-version")
        .output()
        .is_ok_and(|output| output.status.success())
}

fn ffmpeg_capability_available(kind: &str, name: &str) -> bool {
    let Ok(output) = ProcessCommand::new("ffmpeg")
        .args(["-hide_banner", "-h", &format!("{kind}={name}")])
        .output()
    else {
        return false;
    };
    let help = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output.status.success() && ffmpeg_capability_help_available(kind, name, &help)
}

fn ffmpeg_capability_help_available(kind: &str, name: &str, help: &str) -> bool {
    let heading = match kind {
        "encoder" => "Encoder",
        "muxer" => "Muxer",
        _ => return false,
    };
    help.lines()
        .any(|line| line.starts_with(&format!("{heading} {name} [")))
}

fn join_names(names: &BTreeSet<String>) -> String {
    names.iter().cloned().collect::<Vec<_>>().join(", ")
}

fn run_native_workflow(input: &Path, model_dir: &Path, output_dir: &Path) -> (Value, Duration) {
    let started = Instant::now();
    let output = native_transcribe_command(input, model_dir, output_dir)
        .output()
        .expect("native-whisperx finite workflow should run");
    let elapsed = started.elapsed();
    assert!(
        output.status.success(),
        "native finite workflow failed for {}\nstderr:\n{}\nstdout:\n{}",
        input.display(),
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let report: Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("{} stdout should be JSON: {error}", input.display()));
    assert_eq!(
        report.pointer("/response/accepted"),
        Some(&Value::Bool(true)),
        "finite workflow should accept {}",
        input.display()
    );
    (report, elapsed)
}

fn native_transcribe_command(input: &Path, model_dir: &Path, output_dir: &Path) -> Command {
    let mut command = Command::cargo_bin("native-whisperx").expect("binary should build");
    command
        .arg("transcribe")
        .arg(input)
        .args(["--model", "tiny.en", "--model-dir"])
        .arg(model_dir)
        .args([
            "--model-cache-only",
            "--language",
            "en",
            "--no-align",
            "--format",
            "json",
            "--output-dir",
        ])
        .arg(output_dir);
    command
}

fn compare_to_wav_baseline(baseline: &Value, candidate: &Value) -> Value {
    let baseline_text = normalized_transcript_text(baseline);
    let candidate_text = normalized_transcript_text(candidate);
    let baseline_timings = segment_timings(baseline);
    let candidate_timings = segment_timings(candidate);
    let segment_count_matches = baseline_timings.len() == candidate_timings.len();
    let max_segment_timing_delta_seconds = segment_count_matches.then(|| {
        baseline_timings
            .iter()
            .zip(&candidate_timings)
            .flat_map(
                |((baseline_start, baseline_end), (candidate_start, candidate_end))| {
                    [
                        (baseline_start - candidate_start).abs(),
                        (baseline_end - candidate_end).abs(),
                    ]
                },
            )
            .fold(0.0_f64, f64::max)
    });
    let transcript_matches = baseline_text == candidate_text;
    let segment_timings_match = max_segment_timing_delta_seconds
        .is_some_and(|delta| delta <= SEGMENT_TIMING_TOLERANCE_SECONDS);

    json!({
        "transcriptMatches": transcript_matches,
        "segmentCountMatches": segment_count_matches,
        "segmentTimingsMatch": segment_timings_match,
        "maxSegmentTimingDeltaSeconds": max_segment_timing_delta_seconds,
        "normalizedTranscript": candidate_text,
    })
}

fn normalized_transcript_text(report: &Value) -> String {
    let transcript = report
        .pointer("/response/transcript")
        .expect("workflow report should contain response.transcript");
    let text = transcript
        .get("text")
        .and_then(Value::as_str)
        .filter(|text| !text.trim().is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| {
            transcript
                .get("segments")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|segment| segment.get("text").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join(" ")
        });

    text.chars()
        .map(|character| {
            if character.is_alphanumeric() || character.is_whitespace() {
                character.to_lowercase().collect::<String>()
            } else {
                " ".to_string()
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn segment_timings(report: &Value) -> Vec<(f64, f64)> {
    report
        .pointer("/response/transcript/segments")
        .and_then(Value::as_array)
        .expect("workflow report should contain transcript segments")
        .iter()
        .enumerate()
        .map(|(index, segment)| {
            let start = segment
                .get("start")
                .and_then(Value::as_f64)
                .unwrap_or_else(|| panic!("transcript segment {index} should have a start time"));
            let end = segment
                .get("end")
                .and_then(Value::as_f64)
                .unwrap_or_else(|| panic!("transcript segment {index} should have an end time"));
            (start, end)
        })
        .collect()
}

fn assert_non_empty_real_transcript(report: &Value, source_wav: &Path) {
    let normalized = normalized_transcript_text(report);
    assert!(
        !normalized.is_empty(),
        "spoken WAV baseline at {} should produce a real non-empty transcript",
        source_wav.display()
    );
    assert!(
        !segment_timings(report).is_empty(),
        "spoken WAV baseline at {} should produce timed segments",
        source_wav.display()
    );
}

fn workflow_evidence(
    format: &str,
    input: &Path,
    report: &Value,
    end_to_end: Duration,
    comparison: Option<Value>,
) -> Value {
    let diagnostics = report
        .pointer("/response/diagnostics")
        .and_then(Value::as_array)
        .expect("workflow report should contain response.diagnostics");
    let native_predecode_seconds =
        required_diagnostic_f64(diagnostics, "phaseNativePredecodeSeconds");
    let pipeline_decode_seconds = required_diagnostic_f64(diagnostics, "phaseDecodeSeconds");
    let sample_count = required_diagnostic_u64(diagnostics, "phaseDecodeSamples");
    let sample_rate = required_diagnostic_u64(diagnostics, "nativeDecodeOutputSampleRate");
    let channels = required_diagnostic_u64(diagnostics, "nativeDecodeOutputChannels");
    let decode_route = required_diagnostic_value(diagnostics, "nativeDecodeRoute");
    let passed = comparison.as_ref().is_none_or(|comparison| {
        comparison.get("transcriptMatches").and_then(Value::as_bool) == Some(true)
            && comparison
                .get("segmentTimingsMatch")
                .and_then(Value::as_bool)
                == Some(true)
    });

    json!({
        "format": format,
        "input": input.file_name().and_then(|name| name.to_str()).unwrap_or(format),
        "passed": passed,
        "metrics": {
            "decodeOnlySeconds": native_predecode_seconds + pipeline_decode_seconds,
            "nativePredecodeSeconds": native_predecode_seconds,
            "pipelineDecodeSeconds": pipeline_decode_seconds,
            "endToEndSeconds": end_to_end.as_secs_f64(),
            "sampleCount": sample_count,
            "sampleRate": sample_rate,
            "channels": channels,
            "decodeRoute": decode_route,
        },
        "comparison": comparison,
    })
}

fn required_diagnostic_value(diagnostics: &[Value], key: &str) -> String {
    let prefix = format!("{key}=");
    diagnostics
        .iter()
        .filter_map(Value::as_str)
        .find_map(|diagnostic| diagnostic.strip_prefix(&prefix))
        .map(str::to_string)
        .unwrap_or_else(|| {
            panic!("workflow diagnostics should contain '{key}=...': {diagnostics:?}")
        })
}

fn required_diagnostic_f64(diagnostics: &[Value], key: &str) -> f64 {
    required_diagnostic_value(diagnostics, key)
        .parse()
        .unwrap_or_else(|error| panic!("workflow diagnostic '{key}' should be numeric: {error}"))
}

fn required_diagnostic_u64(diagnostics: &[Value], key: &str) -> u64 {
    required_diagnostic_value(diagnostics, key)
        .parse()
        .unwrap_or_else(|error| panic!("workflow diagnostic '{key}' should be an integer: {error}"))
}

fn record_evidence(evidence: &Value) {
    let pretty = serde_json::to_string_pretty(evidence).expect("finite media evidence JSON");
    println!("finite media evidence report:\n{pretty}");
    if let Some(path) = std::env::var_os(EVIDENCE_REPORT_ENV).map(PathBuf::from) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|error| {
                panic!(
                    "failed to create evidence report directory {}: {error}",
                    parent.display()
                )
            });
        }
        fs::write(&path, format!("{pretty}\n")).unwrap_or_else(|error| {
            panic!(
                "failed to write finite media evidence report {}: {error}",
                path.display()
            )
        });
        eprintln!("finite media evidence written to {}", path.display());
    }
}

fn smoke_root() -> PathBuf {
    std::env::var_os("SMOKE_ROOT")
        .map(PathBuf::from)
        .expect("SMOKE_ROOT must be set to run finite media evidence")
}
