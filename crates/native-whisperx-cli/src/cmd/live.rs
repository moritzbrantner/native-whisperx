//! Live Feed Transcription CLI surface and FFmpeg command planning.

use super::*;

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
    anyhow::bail!(
        "live-transcribe execution is not implemented yet; this slice only plans the FFmpeg command"
    );
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
        }
    }

    fn plan_args(plan: &FfmpegCommandPlan) -> Vec<&str> {
        plan.args
            .iter()
            .map(|arg| arg.to_str().expect("test args are utf8"))
            .collect()
    }
}
