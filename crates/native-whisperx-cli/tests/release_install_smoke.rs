use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[test]
#[ignore = "release smoke installs the CLI package into an isolated Cargo root"]
fn cargo_install_package_exposes_native_whisperx_command() {
    #[cfg(all(feature = "pyannote-vad", feature = "pyannote-diarization"))]
    assert_default_cli_packaging_includes_automatic_pyannote_paths();

    let temp = tempfile::tempdir().expect("tempdir");
    let install_root = temp.path().join("install-root");
    let run_dir = temp.path().join("run-dir");
    std::fs::create_dir_all(&run_dir).expect("run dir");

    let install = cargo_command()
        .args(["install", "--path", env!("CARGO_MANIFEST_DIR"), "--root"])
        .arg(&install_root)
        .args(["--locked", "--debug"])
        .output()
        .expect("cargo install should run");
    assert_success(&install, "cargo install native-whisperx-cli");

    let binary = installed_binary(&install_root);
    assert!(
        binary.is_file(),
        "installed native-whisperx binary should exist at {}",
        binary.display()
    );

    let version = run_installed(&binary, &run_dir, ["--version"]);
    assert_success(&version, "native-whisperx --version");
    let version_stdout = String::from_utf8_lossy(&version.stdout);
    assert!(
        version_stdout.contains("native-whisperx"),
        "version output should name the terminal command, got {version_stdout:?}"
    );
    assert!(
        version_stdout.contains(env!("CARGO_PKG_VERSION")),
        "version output should contain package version {}, got {version_stdout:?}",
        env!("CARGO_PKG_VERSION")
    );

    let help = run_installed(&binary, &run_dir, ["--help"]);
    assert_success(&help, "native-whisperx --help");
    let help_stdout = String::from_utf8_lossy(&help.stdout);
    assert!(
        help_stdout.contains("WhisperX-style workflows"),
        "help output should describe the CLI surface, got {help_stdout:?}"
    );

    let speakers_path = run_installed(&binary, &run_dir, ["speakers", "path", "--scope", "local"]);
    assert_success(
        &speakers_path,
        "native-whisperx speakers path --scope local",
    );
    let expected_speaker_directory = run_dir.join(".native-whisperx/speakers");
    let speakers_stdout = String::from_utf8_lossy(&speakers_path.stdout);
    assert!(
        speakers_stdout.contains(expected_speaker_directory.to_string_lossy().as_ref()),
        "offline smoke should resolve the local Speaker Directory without model/media resources, got {speakers_stdout:?}"
    );
}

#[cfg(all(feature = "pyannote-vad", feature = "pyannote-diarization"))]
#[test]
fn default_cli_packaging_includes_automatic_pyannote_paths() {
    assert_default_cli_packaging_includes_automatic_pyannote_paths();
}

fn assert_default_cli_packaging_includes_automatic_pyannote_paths() {
    assert!(
        cfg!(feature = "pyannote-vad"),
        "default native-whisperx-cli packaging should include pyannote VAD code paths"
    );
    assert!(
        cfg!(feature = "pyannote-diarization"),
        "default native-whisperx-cli packaging should include pyannote diarization code paths"
    );
}

fn cargo_command() -> Command {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut command = Command::new(cargo);
    command.current_dir(workspace_root());
    command
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("CLI crate should be under crates/native-whisperx-cli")
        .to_path_buf()
}

fn installed_binary(install_root: &Path) -> PathBuf {
    let binary_name = if cfg!(windows) {
        "native-whisperx.exe"
    } else {
        "native-whisperx"
    };
    install_root.join("bin").join(binary_name)
}

fn run_installed<const N: usize>(binary: &Path, current_dir: &Path, args: [&str; N]) -> Output {
    Command::new(binary)
        .current_dir(current_dir)
        .args(args)
        .env_remove("HF_TOKEN")
        .env_remove("HUGGING_FACE_HUB_TOKEN")
        .env_remove("SMOKE_ROOT")
        .output()
        .expect("installed native-whisperx should run")
}

fn assert_success(output: &Output, label: &str) {
    assert!(
        output.status.success(),
        "{label} failed\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
