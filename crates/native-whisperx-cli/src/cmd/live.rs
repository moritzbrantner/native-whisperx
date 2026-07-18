//! Live Feed Transcription CLI surface and FFmpeg command planning.

use super::*;
use std::io::Write;
use std::process::Child;

use native_whisperx::{
    run_live_asr_window_with_observer, LiveAsrSegmentCandidate, LivePcmIngestionSession,
    LivePcmWindow, LivePcmWindowProcessor, LiveSessionStarted, LiveTranscriptError,
    LiveTranscriptEvent, LiveTranscriptionProgressObserver, LiveWindowProcessingError,
    LiveWindowState, LiveWindowTranscriptObservation, LiveWindowingConfig, LIVE_PCM_SAMPLE_RATE,
};
#[cfg(test)]
use native_whisperx::{LiveSessionEndReason, LiveSessionEnded};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FfmpegCommandPlan {
    pub(crate) program: OsString,
    pub(crate) args: Vec<OsString>,
}

pub(crate) fn live_transcribe_command(args: LiveTranscribeArgs) -> anyhow::Result<()> {
    validate_live_transcribe_args(&args)?;
    let ffmpeg = plan_ffmpeg_command(&args);
    if args.print_ffmpeg_plan {
        println!("{}", ffmpeg.to_json()?);
        return Ok(());
    }

    let session_id = live_session_id();
    let ingest_started_at = SystemTime::now();
    let mut stdout = std::io::stdout().lock();
    write_live_event(
        &mut stdout,
        &LiveTranscriptEvent::SessionStarted(LiveSessionStarted {
            session_id: session_id.clone(),
            sequence: 0,
            source: args.source.clone(),
            ingest_started_at_utc: format_utc_timestamp(ingest_started_at),
            sample_rate: LIVE_PCM_SAMPLE_RATE,
            channels: 1,
            model_id: args.model.clone(),
            language: args.language.clone(),
        }),
    )?;

    let config = live_windowing_config(&args);
    let mut session = LivePcmIngestionSession::new(session_id.clone(), config)?;
    let mut processor = live_window_processor(&args, session_id.clone(), ingest_started_at)?;
    let mut child = match spawn_ffmpeg(&ffmpeg) {
        Ok(child) => child,
        Err(error) => {
            write_live_event(
                &mut stdout,
                &LiveTranscriptEvent::Error(LiveTranscriptError {
                    session_id,
                    sequence: 0,
                    message: error.to_string(),
                    recoverable: false,
                }),
            )?;
            return Err(error);
        }
    };
    let mut pcm = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            let error = anyhow::anyhow!("FFmpeg stdout was not piped");
            write_live_event(
                &mut stdout,
                &LiveTranscriptEvent::Error(LiveTranscriptError {
                    session_id,
                    sequence: 0,
                    message: error.to_string(),
                    recoverable: false,
                }),
            )?;
            return Err(error);
        }
    };
    let report =
        session.ingest_reader_with_event_sink(&mut pcm, processor.as_mut(), &mut |event| {
            write_live_event(&mut stdout, event)
                .map_err(|error| LiveWindowProcessingError::new(error.to_string()))
        });
    let ffmpeg_status = wait_for_ffmpeg(&mut child);

    if let Err(error) = ffmpeg_status {
        write_live_event(
            &mut stdout,
            &LiveTranscriptEvent::Error(LiveTranscriptError {
                session_id,
                sequence: 0,
                message: error.to_string(),
                recoverable: false,
            }),
        )?;
        return Err(error);
    }
    if report.failed() {
        anyhow::bail!("live-transcribe failed; see JSONL error event for details");
    }

    Ok(())
}

fn spawn_ffmpeg(plan: &FfmpegCommandPlan) -> anyhow::Result<Child> {
    ProcessCommand::new(&plan.program)
        .args(&plan.args)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| {
            format!(
                "failed to start FFmpeg command `{}`",
                plan.program.to_string_lossy()
            )
        })
}

fn wait_for_ffmpeg(child: &mut Child) -> anyhow::Result<()> {
    let status = child.wait().context("failed to wait for FFmpeg")?;
    if !status.success() {
        anyhow::bail!("FFmpeg exited with status {status}");
    }
    Ok(())
}

fn write_live_event(writer: &mut dyn Write, event: &LiveTranscriptEvent) -> anyhow::Result<()> {
    serde_json::to_writer(&mut *writer, event)?;
    writer.write_all(b"\n")?;
    writer.flush()?;
    Ok(())
}

fn live_windowing_config(args: &LiveTranscribeArgs) -> LiveWindowingConfig {
    LiveWindowingConfig {
        window_seconds: args.window_seconds,
        hop_seconds: args.hop_seconds,
        finalize_lag_seconds: args.finalize_lag_seconds,
        max_buffer_lag_seconds: args.max_buffer_lag_seconds,
        ..LiveWindowingConfig::default()
    }
}

fn live_window_processor(
    args: &LiveTranscribeArgs,
    session_id: String,
    ingest_started_at: SystemTime,
) -> anyhow::Result<Box<dyn LivePcmWindowProcessor>> {
    if let Some(text) = &args.fake_live_asr_text {
        return Ok(Box::new(FakeLiveAsrWindowProcessor {
            session_id,
            ingest_started_at,
            state: LiveWindowState::new(live_windowing_config(args))?,
            text: text.clone(),
            language: args.language.clone(),
        }));
    }

    Ok(Box::new(NativeLiveAsrWindowProcessor {
        session_id,
        ingest_started_at,
        state: LiveWindowState::new(live_windowing_config(args))?,
        source: args.source.clone(),
        model: args.model.clone(),
        model_dir: args.model_dir.clone(),
        model_cache_only: args.model_cache_only,
        language: args.language.clone(),
    }))
}

fn live_session_id() -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    format!("live-{millis}-{}", std::process::id())
}

fn format_utc_timestamp(time: SystemTime) -> String {
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let total_seconds = duration.as_secs() as i64;
    let millis = duration.subsec_millis();
    let days = total_seconds.div_euclid(86_400);
    let seconds_of_day = total_seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{millis:03}Z")
}

fn civil_from_days(days_since_unix_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_unix_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if m <= 2 { 1 } else { 0 };
    (year as i32, m as u32, d as u32)
}

fn system_time_at_offset(start: SystemTime, seconds: f64) -> SystemTime {
    start + Duration::from_secs_f64(seconds.max(0.0))
}

fn live_asr_config(
    args: &NativeLiveAsrWindowProcessor,
    window: &LivePcmWindow,
) -> NativeWhisperxConfig {
    NativeWhisperxConfig {
        input: InputSource::Samples {
            samples: window.samples.clone(),
            sample_rate: LIVE_PCM_SAMPLE_RATE,
            channels: 1,
            source: Some(args.source.clone()),
        },
        asr: AsrConfig {
            provider: AsrProvider::Native,
            task: TranscriptionTask::Transcribe,
            model_id: args.model.clone(),
            model_dir: args.model_dir.clone(),
            model_cache_only: args.model_cache_only,
            language: args.language.clone(),
            ..AsrConfig::default()
        },
        vad: VadConfig {
            method: VadMethod::Energy,
            ..VadConfig::default()
        },
        alignment: native_whisperx::AlignmentConfig {
            enabled: false,
            ..native_whisperx::AlignmentConfig::default()
        },
        diarization: DiarizationConfig::default(),
        translation: TranslationConfig::default(),
        output: OutputConfig {
            formats: vec![OutputFormat::Json],
            ..OutputConfig::default()
        },
    }
}

struct NativeLiveAsrWindowProcessor {
    session_id: String,
    ingest_started_at: SystemTime,
    state: LiveWindowState,
    source: String,
    model: String,
    model_dir: Option<PathBuf>,
    model_cache_only: bool,
    language: Option<String>,
}

impl LivePcmWindowProcessor for NativeLiveAsrWindowProcessor {
    fn process_window(
        &mut self,
        window: LivePcmWindow,
        progress: &mut dyn LiveTranscriptionProgressObserver,
    ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowProcessingError> {
        let response = run_live_asr_window_with_observer(
            live_asr_config(self, &window),
            &self.session_id,
            window.window_index,
            progress,
        )
        .map_err(|error| LiveWindowProcessingError::new(error.to_string()))?;
        let candidates = response
            .transcript
            .segments
            .iter()
            .filter_map(|segment| candidate_from_segment(segment, &window, self.ingest_started_at))
            .collect::<Vec<_>>();

        Ok(self.state.observe_window(LiveWindowTranscriptObservation {
            session_id: self.session_id.clone(),
            window_start_seconds: window.start_seconds,
            window_end_seconds: window.end_seconds,
            window_start_at_utc: format_utc_timestamp(system_time_at_offset(
                self.ingest_started_at,
                window.start_seconds,
            )),
            window_end_at_utc: format_utc_timestamp(system_time_at_offset(
                self.ingest_started_at,
                window.end_seconds,
            )),
            latest_ingested_audio_seconds: window.latest_ingested_audio_seconds,
            segments: candidates,
        })?)
    }
}

fn candidate_from_segment(
    segment: &native_whisperx::TranscriptSegmentContract,
    window: &LivePcmWindow,
    ingest_started_at: SystemTime,
) -> Option<LiveAsrSegmentCandidate> {
    let text = segment.text.trim();
    if text.is_empty() {
        return None;
    }
    let relative_start = segment.start_seconds.unwrap_or(0.0).max(0.0);
    let relative_end = segment
        .end_seconds
        .unwrap_or(relative_start)
        .max(relative_start);
    let start_seconds = window.start_seconds + relative_start;
    let end_seconds = window.start_seconds + relative_end;
    Some(LiveAsrSegmentCandidate {
        start_seconds,
        end_seconds,
        start_at_utc: format_utc_timestamp(system_time_at_offset(ingest_started_at, start_seconds)),
        end_at_utc: format_utc_timestamp(system_time_at_offset(ingest_started_at, end_seconds)),
        text: text.to_string(),
        language: segment.language.clone(),
    })
}

struct FakeLiveAsrWindowProcessor {
    session_id: String,
    ingest_started_at: SystemTime,
    state: LiveWindowState,
    text: String,
    language: Option<String>,
}

impl LivePcmWindowProcessor for FakeLiveAsrWindowProcessor {
    fn process_window(
        &mut self,
        window: LivePcmWindow,
        _progress: &mut dyn LiveTranscriptionProgressObserver,
    ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowProcessingError> {
        let start_seconds = 0.4;
        let end_seconds = 1.8;
        Ok(self.state.observe_window(LiveWindowTranscriptObservation {
            session_id: self.session_id.clone(),
            window_start_seconds: window.start_seconds,
            window_end_seconds: window.end_seconds,
            window_start_at_utc: format_utc_timestamp(system_time_at_offset(
                self.ingest_started_at,
                window.start_seconds,
            )),
            window_end_at_utc: format_utc_timestamp(system_time_at_offset(
                self.ingest_started_at,
                window.end_seconds,
            )),
            latest_ingested_audio_seconds: window.latest_ingested_audio_seconds,
            segments: vec![LiveAsrSegmentCandidate {
                start_seconds,
                end_seconds,
                start_at_utc: format_utc_timestamp(system_time_at_offset(
                    self.ingest_started_at,
                    start_seconds,
                )),
                end_at_utc: format_utc_timestamp(system_time_at_offset(
                    self.ingest_started_at,
                    end_seconds,
                )),
                text: self.text.clone(),
                language: self.language.clone(),
            }],
        })?)
    }
}

pub(crate) fn plan_ffmpeg_command(args: &LiveTranscribeArgs) -> FfmpegCommandPlan {
    let mut ffmpeg_args = Vec::new();
    ffmpeg_args.extend(args.ffmpeg_input_options.iter().map(OsString::from));
    ffmpeg_args.push(OsString::from("-i"));
    ffmpeg_args.push(OsString::from(&args.source));
    ffmpeg_args.push(OsString::from("-vn"));
    ffmpeg_args.extend(args.ffmpeg_output_options.iter().map(OsString::from));
    ffmpeg_args.extend([
        OsString::from("-ac"),
        OsString::from("1"),
        OsString::from("-ar"),
        OsString::from("16000"),
        OsString::from("-f"),
        OsString::from("f32le"),
        OsString::from("pipe:1"),
    ]);

    FfmpegCommandPlan {
        program: OsString::from(&args.ffmpeg_bin),
        args: ffmpeg_args,
    }
}

#[cfg(test)]
fn seconds_to_sample_index(seconds: f64) -> usize {
    (seconds * LIVE_PCM_SAMPLE_RATE as f64).round() as usize
}

impl FfmpegCommandPlan {
    fn to_json(&self) -> anyhow::Result<String> {
        let program = self.program.to_string_lossy();
        let args = self
            .args
            .iter()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        Ok(serde_json::to_string_pretty(&serde_json::json!({
            "program": program.as_ref(),
            "args": args,
        }))?)
    }
}

fn validate_live_transcribe_args(args: &LiveTranscribeArgs) -> anyhow::Result<()> {
    for (name, value) in [
        ("--window-seconds", args.window_seconds),
        ("--hop-seconds", args.hop_seconds),
        ("--finalize-lag-seconds", args.finalize_lag_seconds),
        ("--max-buffer-lag-seconds", args.max_buffer_lag_seconds),
    ] {
        if !value.is_finite() || value <= 0.0 {
            anyhow::bail!("{name} must be a positive finite number");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Read};

    use native_whisperx::{
        LiveAsrSegmentCandidate, LiveWindowState, LiveWindowTranscriptObservation,
    };

    #[test]
    fn ffmpeg_plan_places_input_options_before_source_and_output_options_before_pcm_settings() {
        let args = live_args_with_options(["-rtsp_transport", "tcp"], ["-hide_banner", "-nostats"]);

        let plan = plan_ffmpeg_command(&args);
        let argv = plan_args(&plan);

        assert_eq!(plan.program, OsString::from("ffmpeg"));
        assert_eq!(
            argv,
            [
                "-rtsp_transport",
                "tcp",
                "-i",
                "rtsp://example.test/live",
                "-vn",
                "-hide_banner",
                "-nostats",
                "-ac",
                "1",
                "-ar",
                "16000",
                "-f",
                "f32le",
                "pipe:1",
            ]
        );
    }

    #[test]
    fn ffmpeg_plan_defaults_to_audio_only_stdout_pcm() {
        let args = live_args_with_options([], []);

        let plan = plan_ffmpeg_command(&args);
        let argv = plan_args(&plan);

        assert_eq!(
            argv,
            [
                "-i",
                "rtsp://example.test/live",
                "-vn",
                "-ac",
                "1",
                "-ar",
                "16000",
                "-f",
                "f32le",
                "pipe:1",
            ]
        );
    }

    fn live_args_with_options<const I: usize, const O: usize>(
        input_options: [&str; I],
        output_options: [&str; O],
    ) -> LiveTranscribeArgs {
        LiveTranscribeArgs {
            source: "rtsp://example.test/live".to_string(),
            model: "small".to_string(),
            model_dir: None,
            model_cache_only: false,
            language: None,
            ffmpeg_bin: "ffmpeg".to_string(),
            ffmpeg_input_options: input_options.into_iter().map(ToString::to_string).collect(),
            ffmpeg_output_options: output_options
                .into_iter()
                .map(ToString::to_string)
                .collect(),
            window_seconds: 5.0,
            hop_seconds: 2.5,
            finalize_lag_seconds: 5.0,
            max_buffer_lag_seconds: 30.0,
            print_ffmpeg_plan: false,
            fake_live_asr_text: None,
        }
    }

    fn plan_args(plan: &FfmpegCommandPlan) -> Vec<&str> {
        plan.args
            .iter()
            .map(|arg| arg.to_str().expect("test args are utf8"))
            .collect()
    }

    #[test]
    fn pcm_ingestion_consumes_little_endian_f32le_chunks_and_tracks_feed_seconds() {
        let mut reader = io::Cursor::new(pcm_bytes(
            (0..seconds_to_sample_index(7.5))
                .map(|index| index as f32 / 1000.0)
                .collect(),
        ));
        let mut processor = RecordingWindowProcessor::default();
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert_eq!(report.processed_sample_count, 120_000);
        assert_eq!(report.processed_audio_seconds, 7.5);
        assert_eq!(report.window_count, 2);
        assert_eq!(
            processor.windows,
            vec![
                WindowSummary {
                    start_seconds: 0.0,
                    end_seconds: 5.0,
                    sample_count: 80_000,
                    first_sample: 0.0,
                    last_sample: 79.999,
                },
                WindowSummary {
                    start_seconds: 2.5,
                    end_seconds: 7.5,
                    sample_count: 80_000,
                    first_sample: 40.0,
                    last_sample: 119.999,
                },
            ]
        );
        assert!(matches!(
            report.events.last(),
            Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                reason: LiveSessionEndReason::Completed,
                processed_audio_seconds: 7.5,
                ..
            }))
        ));
    }

    #[test]
    fn pcm_ingestion_handles_reads_split_across_sample_boundaries() {
        let bytes = pcm_bytes(vec![1.25; seconds_to_sample_index(5.0)]);
        let mut reader = ChunkedReader {
            bytes,
            offset: 0,
            max_chunk_len: 3,
        };
        let mut processor = RecordingWindowProcessor::default();
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert_eq!(report.processed_sample_count, 80_000);
        assert_eq!(report.window_count, 1);
        assert_eq!(processor.windows[0].first_sample, 1.25);
        assert_eq!(processor.windows[0].last_sample, 1.25);
        assert!(matches!(
            report.events.last(),
            Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                reason: LiveSessionEndReason::Completed,
                ..
            }))
        ));
    }

    #[test]
    fn pcm_ingestion_eof_flushes_eligible_final_events_before_session_end() {
        let mut reader = io::Cursor::new(pcm_bytes(vec![0.0; seconds_to_sample_index(7.5)]));
        let mut processor = StableWindowProcessor {
            state: LiveWindowState::new(LiveWindowingConfig {
                window_seconds: 5.0,
                hop_seconds: 2.5,
                finalize_lag_seconds: 5.0,
                max_buffer_lag_seconds: 30.0,
                stability_tolerance_seconds: 0.4,
            })
            .expect("valid window state"),
        };
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert!(report
            .events
            .iter()
            .any(|event| matches!(event, LiveTranscriptEvent::Final(_))));
        assert!(matches!(
            report.events.last(),
            Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                reason: LiveSessionEndReason::Completed,
                final_segment_count: 1,
                ..
            }))
        ));
    }

    #[test]
    fn pcm_ingestion_reports_truncated_sample_bytes() {
        let mut reader = io::Cursor::new(vec![0x00, 0x00, 0x80]);
        let mut processor = RecordingWindowProcessor::default();
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert_eq!(report.window_count, 0);
        assert!(matches!(
            &report.events[0],
            LiveTranscriptEvent::Error(LiveTranscriptError {
                message,
                recoverable: false,
                ..
            }) if message == "truncated f32le PCM frame: 3 trailing byte(s)"
        ));
        assert!(matches!(
            report.events.last(),
            Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                reason: LiveSessionEndReason::Error,
                ..
            }))
        ));
    }

    #[test]
    fn pcm_ingestion_reports_non_finite_samples_as_malformed_pcm() {
        let mut reader = io::Cursor::new(pcm_bytes(vec![0.0, f32::NAN]));
        let mut processor = RecordingWindowProcessor::default();
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert!(matches!(
            &report.events[0],
            LiveTranscriptEvent::Error(LiveTranscriptError {
                message,
                recoverable: false,
                ..
            }) if message == "non-finite f32le PCM sample at feed sample 1"
        ));
        assert_eq!(report.processed_sample_count, 1);
    }

    #[test]
    fn pcm_ingestion_propagates_reader_errors_as_error_events() {
        let mut reader = FailingReader { did_fail: false };
        let mut processor = RecordingWindowProcessor::default();
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert!(matches!(
            &report.events[0],
            LiveTranscriptEvent::Error(LiveTranscriptError {
                message,
                recoverable: false,
                ..
            }) if message == "live PCM reader failed: injected read failure"
        ));
        assert!(matches!(
            report.events.last(),
            Some(LiveTranscriptEvent::SessionEnded(LiveSessionEnded {
                reason: LiveSessionEndReason::Error,
                ..
            }))
        ));
    }

    #[test]
    fn pcm_ingestion_propagates_window_processor_errors_as_error_events() {
        let mut reader = io::Cursor::new(pcm_bytes(vec![0.0; seconds_to_sample_index(5.0)]));
        let mut processor = FailingWindowProcessor;
        let mut session = short_ingestion_session();

        let report = session.ingest_reader(&mut reader, &mut processor);

        assert!(matches!(
            &report.events[0],
            LiveTranscriptEvent::Error(LiveTranscriptError {
                message,
                recoverable: false,
                ..
            }) if message == "live PCM window processing failed: injected processor failure"
        ));
    }

    fn short_ingestion_session() -> LivePcmIngestionSession {
        LivePcmIngestionSession::new(
            "session-1",
            LiveWindowingConfig {
                window_seconds: 5.0,
                hop_seconds: 2.5,
                finalize_lag_seconds: 5.0,
                max_buffer_lag_seconds: 30.0,
                stability_tolerance_seconds: 0.4,
            },
        )
        .expect("valid ingestion session")
    }

    fn pcm_bytes(samples: Vec<f32>) -> Vec<u8> {
        samples
            .into_iter()
            .flat_map(f32::to_le_bytes)
            .collect::<Vec<_>>()
    }

    #[derive(Debug, Clone, PartialEq)]
    struct WindowSummary {
        start_seconds: f64,
        end_seconds: f64,
        sample_count: usize,
        first_sample: f32,
        last_sample: f32,
    }

    #[derive(Default)]
    struct RecordingWindowProcessor {
        windows: Vec<WindowSummary>,
    }

    impl LivePcmWindowProcessor for RecordingWindowProcessor {
        fn process_window(
            &mut self,
            window: LivePcmWindow,
            _progress: &mut dyn LiveTranscriptionProgressObserver,
        ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowProcessingError> {
            self.windows.push(WindowSummary {
                start_seconds: window.start_seconds,
                end_seconds: window.end_seconds,
                sample_count: window.samples.len(),
                first_sample: *window.samples.first().expect("window has samples"),
                last_sample: *window.samples.last().expect("window has samples"),
            });
            Ok(Vec::new())
        }
    }

    struct StableWindowProcessor {
        state: LiveWindowState,
    }

    impl LivePcmWindowProcessor for StableWindowProcessor {
        fn process_window(
            &mut self,
            window: LivePcmWindow,
            _progress: &mut dyn LiveTranscriptionProgressObserver,
        ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowProcessingError> {
            Ok(self.state.observe_window(LiveWindowTranscriptObservation {
                session_id: "session-1".to_string(),
                window_start_seconds: window.start_seconds,
                window_end_seconds: window.end_seconds,
                window_start_at_utc: format!("2026-06-22T15:30:{:02.0}Z", window.start_seconds),
                window_end_at_utc: format!("2026-06-22T15:30:{:02.0}Z", window.end_seconds),
                latest_ingested_audio_seconds: window.end_seconds,
                segments: vec![LiveAsrSegmentCandidate {
                    start_seconds: 0.4,
                    end_seconds: 1.8,
                    start_at_utc: "2026-06-22T15:30:00.400Z".to_string(),
                    end_at_utc: "2026-06-22T15:30:01.800Z".to_string(),
                    text: "hello world".to_string(),
                    language: Some("en".to_string()),
                }],
            })?)
        }
    }

    struct FailingReader {
        did_fail: bool,
    }

    struct ChunkedReader {
        bytes: Vec<u8>,
        offset: usize,
        max_chunk_len: usize,
    }

    impl Read for ChunkedReader {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.offset >= self.bytes.len() {
                return Ok(0);
            }

            let read_len = self
                .max_chunk_len
                .min(buf.len())
                .min(self.bytes.len() - self.offset);
            buf[..read_len].copy_from_slice(&self.bytes[self.offset..self.offset + read_len]);
            self.offset += read_len;
            Ok(read_len)
        }
    }

    impl Read for FailingReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            if self.did_fail {
                Ok(0)
            } else {
                self.did_fail = true;
                Err(io::Error::other("injected read failure"))
            }
        }
    }

    struct FailingWindowProcessor;

    impl LivePcmWindowProcessor for FailingWindowProcessor {
        fn process_window(
            &mut self,
            _window: LivePcmWindow,
            _progress: &mut dyn LiveTranscriptionProgressObserver,
        ) -> Result<Vec<LiveTranscriptEvent>, LiveWindowProcessingError> {
            Err(LiveWindowProcessingError::new("injected processor failure"))
        }
    }
}
