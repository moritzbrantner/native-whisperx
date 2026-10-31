//! `serve`: transcribe many requests from stdin while keeping models loaded.
//!
//! Each stdin line is one JSON request, `{"id": <any>, "args": [...]}`, where
//! `args` are `transcribe` arguments for exactly one input. Each request gets
//! one JSON line on stdout, in order:
//!
//! - success: `{"id", "transcript": <WhisperX JSON>, "provenance", "seconds", "outputFiles"}`
//! - failure: `{"id", "error": "<message>"}`
//!
//! The process keeps the loaded ASR and VAD models between requests while
//! their setup (model, device, compute type, bundles, VAD config) stays the
//! same. Output files are written only when a request passes `--output-dir`.
//! The process exits at end of input.

use std::io::{BufRead, Write};

use clap::Parser;
use native_whisperx::{whisperx_json_value, TranscriptionSession};
use serde::Deserialize;
use serde_json::{json, Value};

use super::transcribe::{transcribe_config, validate_transcribe_args};
use super::*;

#[derive(Debug, Parser)]
#[command(name = "serve-request", no_binary_name = true)]
struct ServeRequestArgs {
    #[command(flatten)]
    transcribe: TranscribeArgs,
}

#[derive(Debug, Deserialize)]
struct ServeRequest {
    #[serde(default)]
    id: Value,
    args: Vec<String>,
}

pub(crate) fn serve_command() -> anyhow::Result<()> {
    let mut session = TranscriptionSession::new();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().lock();
    for line in stdin.lock().lines() {
        let line = line.context("failed to read a serve request from stdin")?;
        if line.trim().is_empty() {
            continue;
        }
        let response = serve_line(&mut session, &line);
        serde_json::to_writer(&mut stdout, &response)?;
        stdout.write_all(b"\n")?;
        stdout.flush()?;
    }
    Ok(())
}

fn serve_line(session: &mut TranscriptionSession, line: &str) -> Value {
    let request = match serde_json::from_str::<ServeRequest>(line) {
        Ok(request) => request,
        Err(error) => {
            return json!({ "id": Value::Null, "error": format!("invalid serve request: {error}") })
        }
    };
    match serve_request(session, &request.args) {
        Ok(mut response) => {
            response["id"] = request.id;
            response
        }
        Err(error) => json!({ "id": request.id, "error": format!("{error:#}") }),
    }
}

fn serve_request(session: &mut TranscriptionSession, args: &[String]) -> anyhow::Result<Value> {
    let started = Instant::now();
    let args = ServeRequestArgs::try_parse_from(args)?.transcribe;
    if args.provider == CliProvider::ExternalWhisperx {
        anyhow::bail!("serve supports the native provider only");
    }
    if args.input.len() != 1 {
        anyhow::bail!("a serve request takes exactly one input");
    }
    if args.audio_track.is_some() {
        anyhow::bail!("serve does not support --audio-track");
    }
    if args.report.is_some() {
        anyhow::bail!("serve returns the report on stdout; drop --report");
    }
    validate_transcribe_args(&args)?;
    let mut config = transcribe_config(&args, args.input[0].clone());
    config.output.output_dir = args.output_dir.clone();
    let return_char_alignments = config.alignment.return_char_alignments;
    let report = session.run(config)?;
    Ok(json!({
        "transcript": whisperx_json_value(&report.transcript, return_char_alignments),
        "provenance": report.provenance,
        "seconds": started.elapsed().as_secs_f64(),
        "outputFiles": report.output_files,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_json_and_bad_args_answer_with_errors() {
        let mut session = TranscriptionSession::new();

        let response = serve_line(&mut session, "not json");
        assert_eq!(response["id"], Value::Null);
        assert!(response["error"]
            .as_str()
            .unwrap()
            .starts_with("invalid serve request"));

        let response = serve_line(&mut session, r#"{"id":7,"args":["a.wav","b.wav"]}"#);
        assert_eq!(response["id"], 7);
        assert_eq!(response["error"], "a serve request takes exactly one input");

        let response = serve_line(
            &mut session,
            r#"{"id":"x","args":["missing.wav","--no-such-flag"]}"#,
        );
        assert_eq!(response["id"], "x");
        assert!(response["error"]
            .as_str()
            .unwrap()
            .contains("--no-such-flag"));
    }
}
