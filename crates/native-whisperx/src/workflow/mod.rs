use std::time::Instant;

mod execution;
mod multi_input;

use crate::config::{
    AsrProvider, NativeWhisperxConfig, NativeWhisperxError, NativeWhisperxReport, VadMethod,
};
use crate::config_mapping::{build_transcription_request, run_native_with_selected_vad};
use crate::output::write_outputs_with_options;
use crate::report::{append_native_alignment_diagnostics, append_native_diarization_diagnostics};

pub(crate) use execution::run_with_phase_observer;
pub use multi_input::{run_many, run_many_reusing_native_provider};

pub fn run(config: NativeWhisperxConfig) -> Result<NativeWhisperxReport, NativeWhisperxError> {
    let run_started = Instant::now();
    let request = build_transcription_request(&config)?;
    let mut response = if config.asr.provider == AsrProvider::Native && config.translation.enabled {
        crate::run_native_with_translation(request, &config)?
    } else if config.asr.provider == AsrProvider::Native
        && matches!(config.vad.method, VadMethod::Silero | VadMethod::Pyannote)
    {
        run_native_with_selected_vad(request, &config)?
    } else {
        run_with_phase_observer(request, &config)?
    };
    append_native_alignment_diagnostics(&mut response, &config);
    append_native_diarization_diagnostics(&mut response, &config);
    crate::save_draft_speakers_from_response(&mut response, &config)?;
    let output_started = Instant::now();
    let output_files = write_outputs_with_options(
        &response,
        &config.output,
        config.alignment.return_char_alignments,
    )?;
    response.diagnostics.push(format!(
        "phaseOutputSeconds={:.6}",
        output_started.elapsed().as_secs_f64()
    ));
    response.diagnostics.push(format!(
        "phaseNativeTotalSeconds={:.6}",
        run_started.elapsed().as_secs_f64()
    ));
    Ok(NativeWhisperxReport {
        response,
        output_files,
    })
}
