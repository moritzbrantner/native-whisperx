use assert_cmd::Command;
use std::path::Path;
use std::process::Command as ProcessCommand;

#[test]
#[ignore = "requires RUN_NATIVE_FFMPEG_MEDIA_DECODE_SMOKE=1 plus local ffmpeg and ffprobe"]
fn generated_common_media_containers_decode_before_native_asr_model_resolution() {
    if std::env::var("RUN_NATIVE_FFMPEG_MEDIA_DECODE_SMOKE")
        .ok()
        .as_deref()
        != Some("1")
    {
        eprintln!("set RUN_NATIVE_FFMPEG_MEDIA_DECODE_SMOKE=1 to run this manual smoke");
        return;
    }

    assert_runtime_tool("ffmpeg");
    assert_runtime_tool("ffprobe");

    let temp = tempfile::tempdir().expect("tempdir");
    let media_dir = temp.path().join("media");
    let model_dir = temp.path().join("models");
    let output_dir = temp.path().join("out");
    std::fs::create_dir_all(&media_dir).expect("media dir");
    std::fs::create_dir_all(&model_dir).expect("model dir");
    std::fs::create_dir_all(&output_dir).expect("output dir");

    let fixtures = [
        MediaFixture::audio("mp3", &["-c:a", "libmp3lame"]),
        MediaFixture::audio("m4a", &["-c:a", "aac"]),
        MediaFixture::audio("aac", &["-c:a", "aac", "-f", "adts"]),
        MediaFixture::audio("flac", &["-c:a", "flac"]),
        MediaFixture::audio("ogg", &["-c:a", "libvorbis"]),
        MediaFixture::audio("opus", &["-c:a", "libopus"]),
        MediaFixture::video("mp4", &["-c:v", "mpeg4", "-c:a", "aac"]),
        MediaFixture::video("mov", &["-c:v", "mpeg4", "-c:a", "aac"]),
        MediaFixture::video("mkv", &["-c:v", "mpeg4", "-c:a", "aac"]),
        MediaFixture::video("webm", &["-c:v", "libvpx", "-c:a", "libopus"]),
    ];

    for fixture in fixtures {
        let input = media_dir.join(format!("tiny.{}", fixture.extension));
        fixture.generate(&input);

        let output = native_transcribe_decode_smoke_command(&input, &model_dir, &output_dir)
            .output()
            .expect("native-whisperx should run");
        assert!(
            !output.status.success(),
            "{} should stop at cache-only model resolution after media decode\nstdout:\n{}\nstderr:\n{}",
            input.display(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("failed to resolve native Candle Whisper model"),
            "{} should reach native ASR model resolution after decode, got stderr:\n{}",
            input.display(),
            stderr
        );
        assert!(
            stderr.contains("cache-only=true"),
            "{} should use cache-only model lookup, got stderr:\n{}",
            input.display(),
            stderr
        );
        assert!(
            !stderr.contains("native decode failed"),
            "{} should not fail in finite media decode, got stderr:\n{}",
            input.display(),
            stderr
        );
    }
}

struct MediaFixture {
    extension: &'static str,
    kind: FixtureKind,
    codec_args: &'static [&'static str],
}

impl MediaFixture {
    fn audio(extension: &'static str, codec_args: &'static [&'static str]) -> Self {
        Self {
            extension,
            kind: FixtureKind::Audio,
            codec_args,
        }
    }

    fn video(extension: &'static str, codec_args: &'static [&'static str]) -> Self {
        Self {
            extension,
            kind: FixtureKind::Video,
            codec_args,
        }
    }

    fn generate(&self, output_path: &Path) {
        let mut command = ProcessCommand::new("ffmpeg");
        command.args(["-hide_banner", "-loglevel", "error", "-y"]);
        match self.kind {
            FixtureKind::Audio => {
                command.args([
                    "-f",
                    "lavfi",
                    "-i",
                    "sine=frequency=1000:duration=0.25:sample_rate=8000",
                    "-ac",
                    "1",
                ]);
            }
            FixtureKind::Video => {
                command.args([
                    "-f",
                    "lavfi",
                    "-i",
                    "testsrc=size=16x16:rate=1:duration=0.25",
                    "-f",
                    "lavfi",
                    "-i",
                    "sine=frequency=1000:duration=0.25:sample_rate=8000",
                    "-shortest",
                    "-pix_fmt",
                    "yuv420p",
                ]);
            }
        }
        command.args(self.codec_args).arg(output_path);

        let output = command.output().expect("ffmpeg should run");
        assert!(
            output.status.success(),
            "ffmpeg failed to create {} fixture at {}\nstderr:\n{}",
            self.extension,
            output_path.display(),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

enum FixtureKind {
    Audio,
    Video,
}

fn assert_runtime_tool(tool: &str) {
    let output = ProcessCommand::new(tool)
        .arg("-version")
        .output()
        .unwrap_or_else(|error| panic!("{tool} must be available on PATH: {error}"));
    assert!(
        output.status.success(),
        "{tool} -version failed\nstderr:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn native_transcribe_decode_smoke_command(
    input: &Path,
    model_dir: &Path,
    output_dir: &Path,
) -> Command {
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
