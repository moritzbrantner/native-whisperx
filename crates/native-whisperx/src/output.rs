//! Output writers for WhisperX JSON, Native JSON, text, SRT, TSV, and VTT files.

use std::fs;
use std::path::Path;

use audio_analysis_transcription::TranscriptionPipelineResponse;
use text_transcripts::TranscriptionContract;

use crate::config::{
    ExpectedOutputComparison, ExpectedOutputFile, NativeWhisperxError, OutputComparisonMode,
    OutputConfig, OutputFile, OutputFormat, ParityTolerance, SubtitleConfig,
};

pub fn write_outputs(
    response: &TranscriptionPipelineResponse,
    output: &OutputConfig,
) -> Result<Vec<OutputFile>, NativeWhisperxError> {
    write_outputs_with_options(response, output, false)
}

pub(crate) fn write_outputs_with_options(
    response: &TranscriptionPipelineResponse,
    output: &OutputConfig,
    return_char_alignments: bool,
) -> Result<Vec<OutputFile>, NativeWhisperxError> {
    let Some(output_dir) = &output.output_dir else {
        return Ok(Vec::new());
    };
    fs::create_dir_all(output_dir)?;
    let basename = output
        .basename
        .clone()
        .or_else(|| {
            response
                .transcript
                .source
                .as_ref()
                .and_then(source_basename)
        })
        .unwrap_or_else(|| "transcript".to_string());

    output
        .formats
        .iter()
        .copied()
        .flat_map(expand_output_format)
        .map(|format| {
            let path = output_dir.join(format!("{basename}.{}", format.extension()));
            let contents = render_output(response, format, output, return_char_alignments)?;
            fs::write(&path, contents)?;
            Ok(OutputFile { format, path })
        })
        .collect()
}

pub(crate) fn compare_expected_outputs(
    actual_outputs: &[OutputFile],
    expected_outputs: &[ExpectedOutputFile],
) -> Result<Vec<ExpectedOutputComparison>, NativeWhisperxError> {
    expected_outputs
        .iter()
        .map(|expected| {
            let actual_path = actual_outputs
                .iter()
                .find(|actual| actual.format == expected.format)
                .map(|actual| actual.path.clone());
            let Some(actual_path_ref) = actual_path.as_ref() else {
                return Ok(ExpectedOutputComparison {
                    format: expected.format,
                    comparison: expected.comparison,
                    gating: expected.gating,
                    expected_path: expected.path.clone(),
                    actual_path,
                    passed: false,
                    difference: Some(format!("missing actual {:?} output", expected.format)),
                });
            };

            let comparison = match expected.comparison {
                OutputComparisonMode::Exact => {
                    compare_output_bytes(&expected.path, actual_path_ref)
                }
                OutputComparisonMode::JsonSemantic => {
                    compare_output_json(&expected.path, actual_path_ref)
                }
                OutputComparisonMode::SubtitleSemantic => {
                    compare_output_subtitles(&expected.path, actual_path_ref)
                }
            }?;

            Ok(ExpectedOutputComparison {
                format: expected.format,
                comparison: expected.comparison,
                gating: expected.gating,
                expected_path: expected.path.clone(),
                actual_path,
                passed: comparison.is_none(),
                difference: comparison,
            })
        })
        .collect()
}

fn compare_output_bytes(
    expected_path: &Path,
    actual_path: &Path,
) -> Result<Option<String>, NativeWhisperxError> {
    let expected = match fs::read(expected_path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Some(format!(
                "missing expected output {}",
                expected_path.display()
            )));
        }
        Err(error) => return Err(NativeWhisperxError::Io(error)),
    };
    let actual = fs::read(actual_path)?;
    if expected == actual {
        return Ok(None);
    }
    Ok(Some(first_output_difference(
        expected_path,
        actual_path,
        &expected,
        &actual,
    )))
}

pub(crate) fn compare_output_json(
    expected_path: &Path,
    actual_path: &Path,
) -> Result<Option<String>, NativeWhisperxError> {
    let expected = match fs::read(expected_path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Some(format!(
                "missing expected output {}",
                expected_path.display()
            )));
        }
        Err(error) => return Err(NativeWhisperxError::Io(error)),
    };
    let actual = fs::read(actual_path)?;
    let expected_json: serde_json::Value = serde_json::from_slice(&expected)?;
    let actual_json: serde_json::Value = serde_json::from_slice(&actual)?;
    if expected_json == actual_json {
        return Ok(None);
    }
    if looks_like_whisperx_transcript_json(&expected_json)
        && looks_like_whisperx_transcript_json(&actual_json)
    {
        return Ok(compare_whisperx_transcript_json(
            &expected_json,
            &actual_json,
            ParityTolerance::default(),
        ));
    }
    Ok(Some(format!(
        "JSON output differs: expected={} actual={}",
        expected_path.display(),
        actual_path.display()
    )))
}

#[derive(Debug, Clone, PartialEq)]
struct ParsedSubtitleCue {
    start: f64,
    end: f64,
    text: String,
}

fn compare_output_subtitles(
    expected_path: &Path,
    actual_path: &Path,
) -> Result<Option<String>, NativeWhisperxError> {
    let expected = match fs::read_to_string(expected_path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Some(format!(
                "missing expected output {}",
                expected_path.display()
            )));
        }
        Err(error) => return Err(NativeWhisperxError::Io(error)),
    };
    let actual = fs::read_to_string(actual_path)?;
    let expected_cues = parse_subtitle_cues(&expected);
    let actual_cues = parse_subtitle_cues(&actual);
    if expected_cues.len() != actual_cues.len() {
        return Ok(Some(format!(
            "subtitle cue count differs: expected={} actual={}",
            expected_cues.len(),
            actual_cues.len()
        )));
    }
    let tolerance = ParityTolerance::default().word_seconds;
    for (index, (expected, actual)) in expected_cues.iter().zip(actual_cues.iter()).enumerate() {
        if let Some(difference) =
            compare_subtitle_seconds(index, "start", expected.start, actual.start, tolerance)
        {
            return Ok(Some(difference));
        }
        if let Some(difference) =
            compare_subtitle_seconds(index, "end", expected.end, actual.end, tolerance)
        {
            return Ok(Some(difference));
        }
        if expected.text != actual.text {
            return Ok(Some(format!(
                "subtitle cue {index} text differs: expected {:?} actual {:?}",
                expected.text, actual.text
            )));
        }
    }
    Ok(None)
}

fn compare_subtitle_seconds(
    index: usize,
    field: &str,
    expected: f64,
    actual: f64,
    tolerance: f64,
) -> Option<String> {
    let delta = (expected - actual).abs();
    if delta <= tolerance {
        None
    } else {
        Some(format!(
            "subtitle cue {index} {field} differs: expected={expected:.3} actual={actual:.3} delta={delta:.3} tolerance={tolerance:.3}"
        ))
    }
}

fn parse_subtitle_cues(text: &str) -> Vec<ParsedSubtitleCue> {
    let normalized = text.replace("\r\n", "\n");
    normalized
        .split("\n\n")
        .filter_map(parse_subtitle_block)
        .collect()
}

fn parse_subtitle_block(block: &str) -> Option<ParsedSubtitleCue> {
    let mut lines = block
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && *line != "WEBVTT");
    let timing_line = lines.find(|line| line.contains("-->"))?;
    let (start, end) = parse_subtitle_timing_line(timing_line)?;
    let text = normalize_subtitle_text(&lines.collect::<Vec<_>>().join(" "));
    Some(ParsedSubtitleCue { start, end, text })
}

fn parse_subtitle_timing_line(line: &str) -> Option<(f64, f64)> {
    let (start, rest) = line.split_once("-->")?;
    let end = rest.split_whitespace().next()?;
    Some((
        timestamp_to_seconds(start.trim()),
        timestamp_to_seconds(end.trim()),
    ))
}

fn normalize_subtitle_text(text: &str) -> String {
    normalize_space(&text.replace("<u>", "").replace("</u>", ""))
}

fn looks_like_whisperx_transcript_json(value: &serde_json::Value) -> bool {
    value.as_object().is_some_and(|object| {
        object.contains_key("segments")
            || object.contains_key("word_segments")
            || (object.contains_key("language") && object.contains_key("text"))
    })
}

fn compare_whisperx_transcript_json(
    expected: &serde_json::Value,
    actual: &serde_json::Value,
    tolerance: ParityTolerance,
) -> Option<String> {
    let expected_object = match expected.as_object() {
        Some(object) => object,
        None => return Some("JSON transcript malformed: expected top-level object".to_string()),
    };
    let actual_object = match actual.as_object() {
        Some(object) => object,
        None => return Some("JSON transcript malformed: actual top-level object".to_string()),
    };

    if let Some(difference) = compare_json_language(expected_object, actual_object) {
        return Some(difference);
    }
    if let Some(difference) = compare_json_segments(expected_object, actual_object, tolerance) {
        return Some(difference);
    }
    if let Some(difference) = compare_json_words(expected_object, actual_object, tolerance) {
        return Some(difference);
    }
    if json_contains_chars(expected_object) || json_contains_chars(actual_object) {
        if let Some(difference) = compare_json_chars(expected_object, actual_object, tolerance) {
            return Some(difference);
        }
    }
    None
}

fn compare_json_language(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
) -> Option<String> {
    let expected_language = match optional_json_string(expected, "language", "expected language") {
        Ok(language) => language,
        Err(error) => return Some(error),
    };
    let actual_language = match optional_json_string(actual, "language", "actual language") {
        Ok(language) => language,
        Err(error) => return Some(error),
    };
    if expected_language != actual_language {
        return Some(format!(
            "JSON transcript language differs: expected={expected_language:?} actual={actual_language:?}"
        ));
    }
    None
}

fn compare_json_segments(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    tolerance: ParityTolerance,
) -> Option<String> {
    let expected_segments = match json_array_field(expected, "segments", "expected segments") {
        Ok(segments) => segments,
        Err(error) => return Some(error),
    };
    let actual_segments = match json_array_field(actual, "segments", "actual segments") {
        Ok(segments) => segments,
        Err(error) => return Some(error),
    };
    if expected_segments.len() != actual_segments.len() {
        return Some(format!(
            "JSON transcript segment count differs: expected={} actual={}",
            expected_segments.len(),
            actual_segments.len()
        ));
    }

    for (index, (expected_segment, actual_segment)) in expected_segments
        .iter()
        .zip(actual_segments.iter())
        .enumerate()
    {
        let expected_segment = match expected_segment.as_object() {
            Some(segment) => segment,
            None => {
                return Some(format!(
                    "JSON transcript segment {index} malformed: expected object"
                ));
            }
        };
        let actual_segment = match actual_segment.as_object() {
            Some(segment) => segment,
            None => {
                return Some(format!(
                    "JSON transcript segment {index} malformed: actual object"
                ));
            }
        };

        if let Some(difference) = compare_required_json_seconds(
            expected_segment,
            actual_segment,
            "start",
            &format!("segment {index} start"),
            tolerance.segment_seconds,
        ) {
            return Some(difference);
        }
        if let Some(difference) = compare_required_json_seconds(
            expected_segment,
            actual_segment,
            "end",
            &format!("segment {index} end"),
            tolerance.segment_seconds,
        ) {
            return Some(difference);
        }

        let expected_text = match required_json_string(
            expected_segment,
            "text",
            &format!("segment {index} expected text"),
        ) {
            Ok(text) => text,
            Err(error) => return Some(error),
        };
        let actual_text = match required_json_string(
            actual_segment,
            "text",
            &format!("segment {index} actual text"),
        ) {
            Ok(text) => text,
            Err(error) => return Some(error),
        };
        if normalize_space(expected_text) != normalize_space(actual_text) {
            return Some(format!(
                "JSON transcript segment {index} text differs: expected={expected_text:?} actual={actual_text:?}"
            ));
        }

        if expected_segment.contains_key("speaker") || actual_segment.contains_key("speaker") {
            let expected_speaker = match optional_json_string(
                expected_segment,
                "speaker",
                &format!("segment {index} expected speaker"),
            ) {
                Ok(speaker) => speaker,
                Err(error) => return Some(error),
            };
            let actual_speaker = match optional_json_string(
                actual_segment,
                "speaker",
                &format!("segment {index} actual speaker"),
            ) {
                Ok(speaker) => speaker,
                Err(error) => return Some(error),
            };
            if expected_speaker != actual_speaker {
                return Some(format!(
                    "JSON transcript segment {index} speaker differs: expected={expected_speaker:?} actual={actual_speaker:?}"
                ));
            }
        }
    }

    None
}

fn compare_json_words(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    tolerance: ParityTolerance,
) -> Option<String> {
    let expected_words = match flattened_json_words(expected, "expected") {
        Ok(words) => words,
        Err(error) => return Some(error),
    };
    let actual_words = match flattened_json_words(actual, "actual") {
        Ok(words) => words,
        Err(error) => return Some(error),
    };
    if expected_words.len() != actual_words.len() {
        return Some(format!(
            "JSON transcript word count differs: expected={} actual={}",
            expected_words.len(),
            actual_words.len()
        ));
    }

    for (index, (expected_word, actual_word)) in
        expected_words.iter().zip(actual_words.iter()).enumerate()
    {
        if let Some(difference) = compare_json_word(index, expected_word, actual_word, tolerance) {
            return Some(difference);
        }
    }

    None
}

fn compare_json_word(
    index: usize,
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    tolerance: ParityTolerance,
) -> Option<String> {
    let expected_text =
        match required_json_string(expected, "word", &format!("word {index} expected word")) {
            Ok(text) => text,
            Err(error) => return Some(error),
        };
    let actual_text =
        match required_json_string(actual, "word", &format!("word {index} actual word")) {
            Ok(text) => text,
            Err(error) => return Some(error),
        };
    if normalize_space(expected_text) != normalize_space(actual_text) {
        return Some(format!(
            "JSON transcript word {index} text differs: expected={expected_text:?} actual={actual_text:?}"
        ));
    }
    if let Some(difference) = compare_required_json_seconds(
        expected,
        actual,
        "start",
        &format!("word {index} start"),
        tolerance.word_seconds,
    ) {
        return Some(difference);
    }
    if let Some(difference) = compare_required_json_seconds(
        expected,
        actual,
        "end",
        &format!("word {index} end"),
        tolerance.word_seconds,
    ) {
        return Some(difference);
    }

    if expected.contains_key("score") && actual.contains_key("score") {
        let expected_score = match optional_json_number(
            expected,
            "score",
            &format!("word {index} expected score"),
        ) {
            Ok(Some(score)) => score,
            Ok(None) => return None,
            Err(error) => return Some(error),
        };
        let actual_score =
            match optional_json_number(actual, "score", &format!("word {index} actual score")) {
                Ok(Some(score)) => score,
                Ok(None) => return None,
                Err(error) => return Some(error),
            };
        if (expected_score - actual_score).abs() > 0.001 {
            return Some(format!(
                "JSON transcript word {index} score differs: expected={expected_score:.3} actual={actual_score:.3} tolerance=0.001"
            ));
        }
    }

    None
}

fn compare_json_chars(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    tolerance: ParityTolerance,
) -> Option<String> {
    let expected_chars = match flattened_json_chars(expected, "expected") {
        Ok(chars) => chars,
        Err(error) => return Some(error),
    };
    let actual_chars = match flattened_json_chars(actual, "actual") {
        Ok(chars) => chars,
        Err(error) => return Some(error),
    };
    if expected_chars.len() != actual_chars.len() {
        return Some(format!(
            "JSON transcript char count differs: expected={} actual={}",
            expected_chars.len(),
            actual_chars.len()
        ));
    }

    for (index, (expected_char, actual_char)) in
        expected_chars.iter().zip(actual_chars.iter()).enumerate()
    {
        let expected_text = match required_json_string(
            expected_char,
            "char",
            &format!("char {index} expected char"),
        ) {
            Ok(text) => text,
            Err(error) => return Some(error),
        };
        let actual_text =
            match required_json_string(actual_char, "char", &format!("char {index} actual char")) {
                Ok(text) => text,
                Err(error) => return Some(error),
            };
        if expected_text != actual_text {
            return Some(format!(
                "JSON transcript char {index} text differs: expected={expected_text:?} actual={actual_text:?}"
            ));
        }
        if let Some(difference) = compare_optional_json_seconds(
            expected_char,
            actual_char,
            "start",
            &format!("char {index} start"),
            tolerance.word_seconds,
        ) {
            return Some(difference);
        }
        if let Some(difference) = compare_optional_json_seconds(
            expected_char,
            actual_char,
            "end",
            &format!("char {index} end"),
            tolerance.word_seconds,
        ) {
            return Some(difference);
        }
    }

    None
}

fn json_contains_chars(object: &serde_json::Map<String, serde_json::Value>) -> bool {
    object
        .get("segments")
        .and_then(serde_json::Value::as_array)
        .is_some_and(|segments| {
            segments.iter().any(|segment| {
                segment
                    .as_object()
                    .is_some_and(|segment| segment.contains_key("chars"))
            })
        })
}

fn flattened_json_words<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    side: &str,
) -> Result<Vec<&'a serde_json::Map<String, serde_json::Value>>, String> {
    if let Some(words) = object.get("word_segments") {
        return json_value_array(words, &format!("{side} word_segments"))?
            .iter()
            .enumerate()
            .map(|(index, word)| {
                word.as_object().ok_or_else(|| {
                    format!(
                        "JSON transcript {side} word_segments[{index}] malformed: object expected"
                    )
                })
            })
            .collect();
    }

    let segments = json_array_field(object, "segments", &format!("{side} segments"))?;
    let mut words = Vec::new();
    for (segment_index, segment) in segments.iter().enumerate() {
        let Some(segment) = segment.as_object() else {
            return Err(format!(
                "JSON transcript {side} segment {segment_index} malformed: object expected"
            ));
        };
        if let Some(segment_words) = segment.get("words") {
            for (word_index, word) in json_value_array(
                segment_words,
                &format!("{side} segment {segment_index} words"),
            )?
            .iter()
            .enumerate()
            {
                words.push(word.as_object().ok_or_else(|| {
                    format!("JSON transcript {side} segment {segment_index} words[{word_index}] malformed: object expected")
                })?);
            }
        }
    }
    Ok(words)
}

fn flattened_json_chars<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    side: &str,
) -> Result<Vec<&'a serde_json::Map<String, serde_json::Value>>, String> {
    let segments = json_array_field(object, "segments", &format!("{side} segments"))?;
    let mut chars = Vec::new();
    for (segment_index, segment) in segments.iter().enumerate() {
        let Some(segment) = segment.as_object() else {
            return Err(format!(
                "JSON transcript {side} segment {segment_index} malformed: object expected"
            ));
        };
        if let Some(segment_chars) = segment.get("chars") {
            for (char_index, character) in json_value_array(
                segment_chars,
                &format!("{side} segment {segment_index} chars"),
            )?
            .iter()
            .enumerate()
            {
                chars.push(character.as_object().ok_or_else(|| {
                    format!("JSON transcript {side} segment {segment_index} chars[{char_index}] malformed: object expected")
                })?);
            }
        }
    }
    Ok(chars)
}

fn json_array_field<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
) -> Result<&'a Vec<serde_json::Value>, String> {
    let value = object
        .get(key)
        .ok_or_else(|| format!("JSON transcript missing array: {label}"))?;
    json_value_array(value, label)
}

fn json_value_array<'a>(
    value: &'a serde_json::Value,
    label: &str,
) -> Result<&'a Vec<serde_json::Value>, String> {
    value
        .as_array()
        .ok_or_else(|| format!("JSON transcript malformed field: {label} must be an array"))
}

fn required_json_string<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
) -> Result<&'a str, String> {
    let value = object
        .get(key)
        .ok_or_else(|| format!("JSON transcript malformed field: {label} missing"))?;
    value
        .as_str()
        .ok_or_else(|| format!("JSON transcript malformed field: {label} must be a string"))
}

fn optional_json_string<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
) -> Result<Option<&'a str>, String> {
    match object.get(key) {
        Some(serde_json::Value::Null) | None => Ok(None),
        Some(value) => value
            .as_str()
            .map(Some)
            .ok_or_else(|| format!("JSON transcript malformed field: {label} must be a string")),
    }
}

fn optional_json_number(
    object: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
) -> Result<Option<f64>, String> {
    match object.get(key) {
        Some(serde_json::Value::Null) | None => Ok(None),
        Some(value) => value
            .as_f64()
            .map(Some)
            .ok_or_else(|| format!("JSON transcript malformed field: {label} must be a number")),
    }
}

fn compare_required_json_seconds(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
    tolerance: f64,
) -> Option<String> {
    let expected_seconds = match optional_json_number(expected, key, &format!("{label} expected")) {
        Ok(Some(seconds)) => seconds,
        Ok(None) => {
            return Some(format!(
                "JSON transcript malformed field: {label} expected missing"
            ));
        }
        Err(error) => return Some(error),
    };
    let actual_seconds = match optional_json_number(actual, key, &format!("{label} actual")) {
        Ok(Some(seconds)) => seconds,
        Ok(None) => {
            return Some(format!(
                "JSON transcript malformed field: {label} actual missing"
            ));
        }
        Err(error) => return Some(error),
    };
    if (expected_seconds - actual_seconds).abs() > tolerance {
        return Some(format!(
            "JSON transcript {label} timing differs: expected={expected_seconds:.3}s actual={actual_seconds:.3}s delta={:.3}s tolerance={tolerance:.3}s",
            (expected_seconds - actual_seconds).abs()
        ));
    }
    None
}

fn compare_optional_json_seconds(
    expected: &serde_json::Map<String, serde_json::Value>,
    actual: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    label: &str,
    tolerance: f64,
) -> Option<String> {
    let expected_seconds = match optional_json_number(expected, key, &format!("{label} expected")) {
        Ok(seconds) => seconds,
        Err(error) => return Some(error),
    };
    let actual_seconds = match optional_json_number(actual, key, &format!("{label} actual")) {
        Ok(seconds) => seconds,
        Err(error) => return Some(error),
    };
    match (expected_seconds, actual_seconds) {
        (Some(expected_seconds), Some(actual_seconds)) => {
            if (expected_seconds - actual_seconds).abs() > tolerance {
                Some(format!(
                    "JSON transcript {label} timing differs: expected={expected_seconds:.3}s actual={actual_seconds:.3}s delta={:.3}s tolerance={tolerance:.3}s",
                    (expected_seconds - actual_seconds).abs()
                ))
            } else {
                None
            }
        }
        (None, None) => None,
        (Some(_), None) | (None, Some(_)) => Some(format!(
            "JSON transcript {label} timing shape differs: expected={} actual={}",
            timing_shape(expected_seconds),
            timing_shape(actual_seconds)
        )),
    }
}

fn timing_shape(value: Option<f64>) -> &'static str {
    if value.is_some() {
        "present"
    } else {
        "null"
    }
}

fn first_output_difference(
    expected_path: &Path,
    actual_path: &Path,
    expected: &[u8],
    actual: &[u8],
) -> String {
    let expected_text = std::str::from_utf8(expected);
    let actual_text = std::str::from_utf8(actual);
    if let (Ok(expected_text), Ok(actual_text)) = (expected_text, actual_text) {
        for (index, (expected_line, actual_line)) in
            expected_text.lines().zip(actual_text.lines()).enumerate()
        {
            if expected_line != actual_line {
                return format!(
                    "line {} differs: expected {:?}, actual {:?}",
                    index + 1,
                    expected_line,
                    actual_line
                );
            }
        }
    }
    format!(
        "output bytes differ: expected={} ({} bytes) actual={} ({} bytes)",
        expected_path.display(),
        expected.len(),
        actual_path.display(),
        actual.len()
    )
}

fn render_output(
    response: &TranscriptionPipelineResponse,
    format: OutputFormat,
    output: &OutputConfig,
    return_char_alignments: bool,
) -> Result<String, NativeWhisperxError> {
    match format {
        OutputFormat::All => Err(NativeWhisperxError::InvalidConfig(
            "internal error: all output format must be expanded before rendering".to_string(),
        )),
        OutputFormat::Json if output.pretty_json => Ok(serde_json::to_string_pretty(
            &whisperx_json_value(&response.transcript, return_char_alignments),
        )?),
        OutputFormat::Json => Ok(serde_json::to_string(&whisperx_json_value(
            &response.transcript,
            return_char_alignments,
        ))?),
        OutputFormat::NativeJson if output.pretty_json => {
            Ok(serde_json::to_string_pretty(&response.transcript)?)
        }
        OutputFormat::NativeJson => Ok(serde_json::to_string(&response.transcript)?),
        OutputFormat::Srt => Ok(format_srt_with_options(
            &response.transcript,
            &output.subtitles,
        )),
        OutputFormat::Vtt => Ok(format_webvtt_with_options(
            &response.transcript,
            &output.subtitles,
        )),
        OutputFormat::Txt => Ok(format_txt(&response.transcript)),
        OutputFormat::Tsv => Ok(format_tsv(&response.transcript)),
        OutputFormat::Audacity => Ok(format_audacity_labels(&response.transcript)),
    }
}

pub(crate) fn whisperx_json_value(
    transcript: &TranscriptionContract,
    return_char_alignments: bool,
) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert(
        "text".to_string(),
        serde_json::Value::String(transcript.text_or_joined()),
    );
    if let Some(language) = &transcript.language {
        object.insert(
            "language".to_string(),
            serde_json::Value::String(language.clone()),
        );
    }
    if let Some(source) = &transcript.source {
        object.insert(
            "source".to_string(),
            serde_json::Value::String(source.clone()),
        );
    }

    let segments = transcript
        .segments
        .iter()
        .map(|segment| whisperx_segment_value(segment, return_char_alignments))
        .collect::<Vec<_>>();
    let words = transcript
        .segments
        .iter()
        .flat_map(|segment| segment.words.iter())
        .map(whisperx_word_value)
        .collect::<Vec<_>>();

    object.insert("segments".to_string(), serde_json::Value::Array(segments));
    object.insert("word_segments".to_string(), serde_json::Value::Array(words));
    serde_json::Value::Object(object)
}

fn whisperx_segment_value(
    segment: &text_transcripts::TranscriptSegmentContract,
    return_char_alignments: bool,
) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert("id".to_string(), serde_json::Value::from(segment.index));
    insert_seconds(&mut object, "start", segment.start_seconds);
    insert_seconds(&mut object, "end", segment.end_seconds);
    object.insert(
        "text".to_string(),
        serde_json::Value::String(segment.text.clone()),
    );
    if let Some(speaker) = &segment.speaker {
        object.insert(
            "speaker".to_string(),
            serde_json::Value::String(speaker.clone()),
        );
    }
    if let Some(confidence) = segment.confidence {
        object.insert("score".to_string(), serde_json::Value::from(confidence));
    }
    if !segment.words.is_empty() {
        object.insert(
            "words".to_string(),
            serde_json::Value::Array(segment.words.iter().map(whisperx_word_value).collect()),
        );
    }
    if return_char_alignments && !segment.chars.is_empty() {
        object.insert(
            "chars".to_string(),
            serde_json::Value::Array(segment.chars.iter().map(whisperx_char_value).collect()),
        );
    }
    serde_json::Value::Object(object)
}

fn whisperx_word_value(word: &text_transcripts::TranscriptWordContract) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert(
        "word".to_string(),
        serde_json::Value::String(word.text.clone()),
    );
    insert_seconds(&mut object, "start", word.start_seconds);
    insert_seconds(&mut object, "end", word.end_seconds);
    if let Some(confidence) = word.confidence {
        object.insert("score".to_string(), serde_json::Value::from(confidence));
    }
    if let Some(speaker) = &word.speaker {
        object.insert(
            "speaker".to_string(),
            serde_json::Value::String(speaker.clone()),
        );
    }
    serde_json::Value::Object(object)
}

fn whisperx_char_value(character: &text_transcripts::TranscriptCharContract) -> serde_json::Value {
    let mut object = serde_json::Map::new();
    object.insert(
        "char".to_string(),
        serde_json::Value::String(character.character.clone()),
    );
    insert_seconds(&mut object, "start", character.start_seconds);
    insert_seconds(&mut object, "end", character.end_seconds);
    if let Some(confidence) = character.confidence {
        object.insert("score".to_string(), serde_json::Value::from(confidence));
    }
    serde_json::Value::Object(object)
}

fn insert_seconds(
    object: &mut serde_json::Map<String, serde_json::Value>,
    key: &str,
    value: Option<f64>,
) {
    if let Some(value) = value {
        object.insert(key.to_string(), serde_json::Value::from(value));
    }
}

pub(crate) fn expand_output_format(format: OutputFormat) -> Vec<OutputFormat> {
    match format {
        OutputFormat::All => vec![
            OutputFormat::Txt,
            OutputFormat::Vtt,
            OutputFormat::Srt,
            OutputFormat::Tsv,
            OutputFormat::Audacity,
            OutputFormat::Json,
        ],
        other => vec![other],
    }
}

fn format_txt(transcript: &TranscriptionContract) -> String {
    let text = transcript
        .segments
        .iter()
        .map(|segment| match &segment.speaker {
            Some(speaker) => format!("[{speaker}]: {}", segment.text.trim()),
            None => segment.text.trim().to_string(),
        })
        .collect::<Vec<_>>()
        .join("\n");
    if text.is_empty() {
        text
    } else {
        format!("{text}\n")
    }
}

fn format_tsv(transcript: &TranscriptionContract) -> String {
    let mut output = String::from("start\tend\ttext\n");
    for segment in &transcript.segments {
        let start = seconds_to_millis(segment.start_seconds);
        let end = seconds_to_millis(segment.end_seconds);
        output.push_str(&format!(
            "{start}\t{end}\t{}\n",
            segment.text.trim().replace('\t', " ")
        ));
    }
    output
}

fn format_audacity_labels(transcript: &TranscriptionContract) -> String {
    let mut output = String::new();
    for segment in &transcript.segments {
        let start = segment.start_seconds.unwrap_or(0.0);
        let end = segment.end_seconds.unwrap_or(start).max(start);
        let text = match &segment.speaker {
            Some(speaker) => format!("[[{speaker}]]{}", segment.text.trim().replace('\t', " ")),
            None => segment.text.trim().replace('\t', " "),
        };
        output.push_str(&format!("{start}\t{end}\t{text}\n"));
    }
    output
}

fn seconds_to_millis(seconds: Option<f64>) -> u64 {
    seconds.unwrap_or(0.0).max(0.0).mul_add(1000.0, 0.0).round() as u64
}

fn format_srt_with_options(
    transcript: &TranscriptionContract,
    subtitles: &SubtitleConfig,
) -> String {
    let mut output = String::new();
    for (index, cue) in subtitle_cues(transcript, subtitles).into_iter().enumerate() {
        output.push_str(&(index + 1).to_string());
        output.push('\n');
        output.push_str(&format_subtitle_timestamp(cue.start, true, ','));
        output.push_str(" --> ");
        output.push_str(&format_subtitle_timestamp(cue.end, true, ','));
        output.push('\n');
        output.push_str(&cue.text);
        output.push_str("\n\n");
    }
    output
}

fn format_webvtt_with_options(
    transcript: &TranscriptionContract,
    subtitles: &SubtitleConfig,
) -> String {
    let mut output = String::from("WEBVTT\n\n");
    for cue in subtitle_cues(transcript, subtitles) {
        output.push_str(&format_subtitle_timestamp(cue.start, false, '.'));
        output.push_str(" --> ");
        output.push_str(&format_subtitle_timestamp(cue.end, false, '.'));
        output.push('\n');
        output.push_str(&cue.text);
        output.push_str("\n\n");
    }
    output
}

#[derive(Debug, Clone)]
struct SubtitleCue {
    start: f64,
    end: f64,
    text: String,
}

#[derive(Debug, Clone)]
struct SubtitleTiming {
    word: String,
    start: Option<f64>,
    end: Option<f64>,
}

fn subtitle_cues(
    transcript: &TranscriptionContract,
    subtitles: &SubtitleConfig,
) -> Vec<SubtitleCue> {
    let Some(first_segment) = transcript.segments.first() else {
        return Vec::new();
    };
    if !first_segment.words.is_empty() {
        return subtitle_word_cues(transcript, subtitles);
    }
    transcript
        .segments
        .iter()
        .map(|segment| {
            let start = segment.start_seconds.unwrap_or(0.0);
            let end = segment.end_seconds.unwrap_or(start).max(start);
            let mut text = segment.text.trim().replace("-->", "->");
            if let Some(speaker) = &segment.speaker {
                text = format!("[{speaker}]: {text}");
            }
            SubtitleCue { start, end, text }
        })
        .collect()
}

fn subtitle_word_cues(
    transcript: &TranscriptionContract,
    subtitles: &SubtitleConfig,
) -> Vec<SubtitleCue> {
    let mut cues = Vec::new();
    let raw_max_line_width = subtitles.max_line_width;
    let max_line_count = subtitles.max_line_count;
    let max_line_width = raw_max_line_width.unwrap_or(1000);
    let preserve_segments = max_line_count.is_none() || raw_max_line_width.is_none();

    let mut line_len = 0usize;
    let mut line_count = 1usize;
    let mut subtitle = Vec::<SubtitleTiming>::new();
    let mut times = Vec::<(f64, f64, Option<String>)>::new();
    let mut last = transcript
        .segments
        .first()
        .and_then(|segment| segment.start_seconds)
        .unwrap_or(0.0);

    for segment in &transcript.segments {
        for (word_index, original_timing) in segment.words.iter().enumerate() {
            let mut timing = SubtitleTiming {
                word: original_timing.text.clone(),
                start: original_timing.start_seconds,
                end: original_timing.end_seconds,
            };
            let long_pause = if preserve_segments {
                false
            } else {
                timing.start.is_some_and(|start| start - last > 3.0)
            };
            let has_room = line_len + timing.word.chars().count() <= max_line_width;
            let seg_break = word_index == 0 && !subtitle.is_empty() && preserve_segments;
            if line_len > 0 && has_room && !long_pause && !seg_break {
                line_len += timing.word.chars().count();
            } else {
                timing.word = timing.word.trim().to_string();
                if (!subtitle.is_empty()
                    && max_line_count.is_some()
                    && (long_pause || line_count >= max_line_count.unwrap_or(0)))
                    || seg_break
                {
                    push_subtitle_cues(transcript, subtitles, &subtitle, &times, &mut cues);
                    subtitle.clear();
                    times.clear();
                    line_count = 1;
                } else if line_len > 0 {
                    line_count += 1;
                    timing.word = format!("\n{}", timing.word);
                }
                line_len = timing.word.trim().chars().count();
            }
            subtitle.push(timing);
            times.push((
                segment.start_seconds.unwrap_or(0.0),
                segment
                    .end_seconds
                    .unwrap_or_else(|| segment.start_seconds.unwrap_or(0.0)),
                segment.speaker.clone(),
            ));
            if let Some(start) = original_timing.start_seconds {
                last = start;
            }
        }
    }
    if !subtitle.is_empty() {
        push_subtitle_cues(transcript, subtitles, &subtitle, &times, &mut cues);
    }
    cues
}

fn push_subtitle_cues(
    transcript: &TranscriptionContract,
    subtitles: &SubtitleConfig,
    subtitle: &[SubtitleTiming],
    times: &[(f64, f64, Option<String>)],
    cues: &mut Vec<SubtitleCue>,
) {
    let Some((fallback_start, fallback_end, speaker)) = times.first() else {
        return;
    };
    let word_starts = subtitle.iter().filter_map(|word| word.start);
    let word_ends = subtitle.iter().filter_map(|word| word.end);
    let start = word_starts.reduce(f64::min).unwrap_or(*fallback_start);
    let end = word_ends.reduce(f64::max).unwrap_or(*fallback_end);
    let prefix = speaker
        .as_ref()
        .map(|speaker| format!("[{speaker}]: "))
        .unwrap_or_default();
    let subtitle_text = subtitle_text_for_language(transcript, subtitle);
    let has_timing = subtitle.iter().any(|word| word.start.is_some());

    if subtitles.highlight_words && has_timing {
        let mut last = format_subtitle_timestamp(start, true, ',');
        let all_words = subtitle
            .iter()
            .map(|timing| timing.word.clone())
            .collect::<Vec<_>>();
        for (index, timing) in subtitle.iter().enumerate() {
            let (Some(word_start), Some(word_end)) = (timing.start, timing.end) else {
                continue;
            };
            let start_text = format_subtitle_timestamp(word_start, true, ',');
            let end_text = format_subtitle_timestamp(word_end, true, ',');
            if last != start_text {
                cues.push(SubtitleCue {
                    start: timestamp_to_seconds(&last),
                    end: word_start,
                    text: format!("{prefix}{subtitle_text}"),
                });
            }
            cues.push(SubtitleCue {
                start: word_start,
                end: word_end,
                text: format!(
                    "{prefix}{}",
                    all_words
                        .iter()
                        .enumerate()
                        .map(|(word_index, word)| {
                            if word_index == index {
                                underline_word_preserving_leading_space(word)
                            } else {
                                word.clone()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
            });
            last = end_text;
        }
    } else {
        cues.push(SubtitleCue {
            start,
            end,
            text: format!("{prefix}{subtitle_text}"),
        });
    }
}

fn subtitle_text_for_language(
    transcript: &TranscriptionContract,
    subtitle: &[SubtitleTiming],
) -> String {
    let words = subtitle
        .iter()
        .map(|timing| timing.word.clone())
        .collect::<Vec<_>>();
    if transcript
        .language
        .as_deref()
        .is_some_and(|language| matches!(language, "ja" | "zh"))
    {
        words.join("")
    } else {
        words.join(" ")
    }
}

fn underline_word_preserving_leading_space(word: &str) -> String {
    let leading_bytes = word
        .char_indices()
        .find(|(_, character)| !character.is_whitespace())
        .map(|(index, _)| index)
        .unwrap_or(word.len());
    let (leading, rest) = word.split_at(leading_bytes);
    format!("{leading}<u>{rest}</u>")
}

fn format_subtitle_timestamp(
    seconds: f64,
    always_include_hours: bool,
    decimal_marker: char,
) -> String {
    let total_millis = (seconds.max(0.0) * 1_000.0).round() as u64;
    let millis = total_millis % 1_000;
    let total_seconds = total_millis / 1_000;
    let secs = total_seconds % 60;
    let total_minutes = total_seconds / 60;
    let minutes = total_minutes % 60;
    let hours = total_minutes / 60;
    if always_include_hours || hours > 0 {
        format!("{hours:02}:{minutes:02}:{secs:02}{decimal_marker}{millis:03}")
    } else {
        format!("{minutes:02}:{secs:02}{decimal_marker}{millis:03}")
    }
}

fn timestamp_to_seconds(timestamp: &str) -> f64 {
    let normalized = timestamp.replace(',', ".");
    let parts = normalized.split(':').collect::<Vec<_>>();
    match parts.as_slice() {
        [hours, minutes, seconds] => {
            hours.parse::<f64>().unwrap_or(0.0) * 3600.0
                + minutes.parse::<f64>().unwrap_or(0.0) * 60.0
                + seconds.parse::<f64>().unwrap_or(0.0)
        }
        [minutes, seconds] => {
            minutes.parse::<f64>().unwrap_or(0.0) * 60.0 + seconds.parse::<f64>().unwrap_or(0.0)
        }
        _ => 0.0,
    }
}

fn source_basename(source: &String) -> Option<String> {
    Path::new(source)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(|stem| stem.to_string())
        .filter(|stem| !stem.trim().is_empty())
}

pub(crate) fn normalize_space(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use audio_analysis_transcription::TranscriptionPipelineResponse;

    use crate::config::{
        ExpectedOutputFile, OutputComparisonMode, OutputConfig, OutputFile, OutputFormat,
        SegmentResolution, SubtitleConfig,
    };
    use crate::import_whisperx_json;

    const WHISPERX_SAMPLE: &[u8] =
        include_bytes!("../../../tests/fixtures/whisperx-parity-sample.json");

    #[test]
    fn writes_requested_outputs() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        let files = write_outputs_with_options(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![
                    OutputFormat::Json,
                    OutputFormat::NativeJson,
                    OutputFormat::Srt,
                    OutputFormat::Vtt,
                    OutputFormat::Txt,
                    OutputFormat::Tsv,
                    OutputFormat::Audacity,
                ],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig::default(),
            },
            true,
        )
        .expect("outputs should write");

        assert_eq!(files.len(), 7);
        let json_path = temp.path().join("sample.json");
        let native_json_path = temp.path().join("sample.native.json");
        let srt_path = temp.path().join("sample.srt");
        let vtt_path = temp.path().join("sample.vtt");
        let txt_path = temp.path().join("sample.txt");
        let tsv_path = temp.path().join("sample.tsv");
        let aud_path = temp.path().join("sample.aud");
        assert!(json_path.is_file());
        assert!(native_json_path.is_file());
        assert!(srt_path.is_file());
        assert!(vtt_path.is_file());
        assert!(txt_path.is_file());
        assert!(tsv_path.is_file());
        assert!(aud_path.is_file());

        let whisperx_json: serde_json::Value =
            serde_json::from_slice(&fs::read(json_path).expect("json"))
                .expect("valid whisperx json");
        assert!(whisperx_json.get("segments").is_some());
        assert!(whisperx_json.get("word_segments").is_some());
        assert!(whisperx_json["segments"][0].get("start").is_some());
        assert!(whisperx_json["segments"][0].get("end").is_some());
        assert!(whisperx_json["segments"][0].get("startSeconds").is_none());
        assert_eq!(whisperx_json["segments"][0]["chars"][0]["char"], "h");

        let native_json: serde_json::Value =
            serde_json::from_slice(&fs::read(native_json_path).expect("native json"))
                .expect("valid native json");
        assert!(native_json["segments"][0].get("startSeconds").is_some());
        assert!(native_json["segments"][0].get("chars").is_some());

        let txt = fs::read_to_string(txt_path).expect("txt");
        assert_eq!(
            txt,
            "[SPEAKER_00]: hello world\n[SPEAKER_01]: second speaker\n"
        );
        let srt = fs::read_to_string(srt_path).expect("srt");
        assert!(srt.contains("00:00:00,000 --> 00:00:01,100"));
        assert!(srt.contains("[SPEAKER_00]: hello world"));
        let vtt = fs::read_to_string(vtt_path).expect("vtt");
        assert!(vtt.starts_with("WEBVTT\n\n"));
        assert!(vtt.contains("00:01.350 --> 00:02.350"));
        assert!(vtt.contains("[SPEAKER_01]: second speaker"));
        let tsv = fs::read_to_string(tsv_path).expect("tsv");
        assert!(tsv.starts_with("start\tend\ttext\n"));
        assert!(tsv.contains("0\t1200\thello world"));
        assert!(tsv.contains("1350\t2400\tsecond speaker"));
        let aud = fs::read_to_string(aud_path).expect("aud");
        assert!(aud.contains("0\t1.2\t[[SPEAKER_00]]hello world"));
        assert!(aud.contains("1.35\t2.4\t[[SPEAKER_01]]second speaker"));
    }

    #[test]
    fn all_format_writes_whisperx_compatible_set_without_native_json() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs_with_options(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::All],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig::default(),
            },
            true,
        )
        .expect("outputs should write");

        let mut names = fs::read_dir(temp.path())
            .expect("read output dir")
            .map(|entry| {
                entry
                    .expect("dir entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<_>>();
        names.sort();
        assert_eq!(
            names,
            vec![
                "sample.aud",
                "sample.json",
                "sample.srt",
                "sample.tsv",
                "sample.txt",
                "sample.vtt",
            ]
        );
    }

    #[test]
    fn output_stems_keep_multi_input_writes_collision_safe() {
        let temp = tempfile::tempdir().expect("tempdir");
        let mut first = fixture_response_with_chars();
        first.transcript.source = Some("audio/first-input.wav".to_string());
        let mut second = fixture_response_with_chars();
        second.transcript.source = Some("audio/second-input.wav".to_string());
        let output = OutputConfig {
            output_dir: Some(temp.path().to_path_buf()),
            formats: vec![OutputFormat::All],
            basename: None,
            pretty_json: true,
            subtitles: SubtitleConfig::default(),
        };

        write_outputs_with_options(&first, &output, true).expect("first outputs should write");
        write_outputs_with_options(&second, &output, true).expect("second outputs should write");

        let mut names = fs::read_dir(temp.path())
            .expect("read output dir")
            .map(|entry| {
                entry
                    .expect("dir entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<_>>();
        names.sort();

        assert_eq!(
            names,
            vec![
                "first-input.aud",
                "first-input.json",
                "first-input.srt",
                "first-input.tsv",
                "first-input.txt",
                "first-input.vtt",
                "second-input.aud",
                "second-input.json",
                "second-input.srt",
                "second-input.tsv",
                "second-input.txt",
                "second-input.vtt",
            ]
        );
    }

    #[test]
    fn txt_writes_each_segment_without_speakers() {
        let mut response = fixture_response_with_chars();
        for segment in &mut response.transcript.segments {
            segment.speaker = None;
        }
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Txt],
                basename: Some("sample".to_string()),
                ..OutputConfig::default()
            },
        )
        .expect("txt should write");

        let txt = fs::read_to_string(temp.path().join("sample.txt")).expect("txt");
        assert_eq!(txt, "hello world\nsecond speaker\n");
    }

    #[test]
    fn tsv_includes_header_and_replaces_tabs() {
        let mut response = fixture_response_with_chars();
        response.transcript.segments[0].text = " hello\tworld ".to_string();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Tsv],
                basename: Some("sample".to_string()),
                ..OutputConfig::default()
            },
        )
        .expect("tsv should write");

        let tsv = fs::read_to_string(temp.path().join("sample.tsv")).expect("tsv");
        assert!(tsv.starts_with("start\tend\ttext\n"));
        assert!(tsv.contains("0\t1200\thello world\n"));
    }

    #[test]
    fn subtitle_options_highlight_and_wrap_text() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Srt, OutputFormat::Vtt],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig {
                    max_line_width: Some(8),
                    max_line_count: None,
                    highlight_words: true,
                    segment_resolution: SegmentResolution::Sentence,
                },
            },
        )
        .expect("subtitles should write");

        let srt = fs::read_to_string(temp.path().join("sample.srt")).expect("srt");
        assert!(srt.contains("<u>hello</u>"));
        assert!(srt.contains("[SPEAKER_00]: <u>hello</u> \nworld"));
        assert!(srt.contains("[SPEAKER_00]: hello \n<u>world</u>"));
        let vtt = fs::read_to_string(temp.path().join("sample.vtt")).expect("vtt");
        assert!(vtt.contains("<u>hello</u>"));
    }

    #[test]
    fn subtitle_max_line_count_merges_overflow() {
        let response = fixture_response_with_chars();
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Srt],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig {
                    max_line_width: Some(8),
                    max_line_count: Some(1),
                    highlight_words: false,
                    segment_resolution: SegmentResolution::Sentence,
                },
            },
        )
        .expect("subtitles should write");

        let srt = fs::read_to_string(temp.path().join("sample.srt")).expect("srt");
        assert!(srt.contains("[SPEAKER_00]: hello\n\n2"));
        assert!(srt.contains("[SPEAKER_00]: world\n\n3"));
        assert!(srt.contains("[SPEAKER_01]: second\n\n4"));
        assert!(srt.contains("[SPEAKER_01]: speaker\n\n"));
    }

    #[test]
    fn subtitle_word_cues_join_languages_without_spaces() {
        let mut response = fixture_response_with_chars();
        response.transcript.language = Some("ja".to_string());
        response.transcript.segments[0].speaker = None;
        let temp = tempfile::tempdir().expect("tempdir");
        write_outputs(
            &response,
            &OutputConfig {
                output_dir: Some(temp.path().to_path_buf()),
                formats: vec![OutputFormat::Srt],
                basename: Some("sample".to_string()),
                pretty_json: true,
                subtitles: SubtitleConfig::default(),
            },
        )
        .expect("subtitles should write");

        let srt = fs::read_to_string(temp.path().join("sample.srt")).expect("srt");
        assert!(srt.contains("helloworld"));
    }

    #[test]
    fn whisperx_json_omits_chars_when_not_requested() {
        let mut transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        transcript.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });

        let without_chars = whisperx_json_value(&transcript, false);
        let with_chars = whisperx_json_value(&transcript, true);

        assert!(without_chars["segments"][0].get("chars").is_none());
        assert!(with_chars["segments"][0].get("chars").is_some());
    }

    #[test]
    fn output_comparison_reports_exact_json_and_missing_outputs() {
        let temp = tempfile::tempdir().expect("tempdir");
        let expected_txt = temp.path().join("expected.txt");
        let actual_txt = temp.path().join("actual.txt");
        let expected_json = temp.path().join("expected.json");
        let actual_json = temp.path().join("actual.json");
        let missing_expected = temp.path().join("missing.srt");
        let actual_srt = temp.path().join("actual.srt");
        fs::write(&expected_txt, "hello\n").expect("expected txt");
        fs::write(&actual_txt, "hello changed\n").expect("actual txt");
        fs::write(&expected_json, "{\n  \"a\": 1\n}\n").expect("expected json");
        fs::write(&actual_json, "{\"a\":1}").expect("actual json");
        fs::write(&actual_srt, "1\n").expect("actual srt");

        let actual_outputs = vec![
            OutputFile {
                format: OutputFormat::Txt,
                path: actual_txt,
            },
            OutputFile {
                format: OutputFormat::Json,
                path: actual_json,
            },
            OutputFile {
                format: OutputFormat::Srt,
                path: actual_srt,
            },
        ];
        let comparisons = compare_expected_outputs(
            &actual_outputs,
            &[
                ExpectedOutputFile {
                    format: OutputFormat::Txt,
                    path: expected_txt,
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                },
                ExpectedOutputFile {
                    format: OutputFormat::Json,
                    path: expected_json,
                    comparison: OutputComparisonMode::JsonSemantic,
                    gating: true,
                },
                ExpectedOutputFile {
                    format: OutputFormat::Vtt,
                    path: temp.path().join("expected.vtt"),
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                },
                ExpectedOutputFile {
                    format: OutputFormat::Srt,
                    path: missing_expected,
                    comparison: OutputComparisonMode::Exact,
                    gating: true,
                },
            ],
        )
        .expect("comparison should run");

        assert!(!comparisons[0].passed);
        assert!(comparisons[0]
            .difference
            .as_deref()
            .is_some_and(|difference| difference.contains("line 1 differs")));
        assert!(comparisons[1].passed);
        assert!(!comparisons[2].passed);
        assert!(comparisons[2]
            .difference
            .as_deref()
            .is_some_and(|difference| difference.contains("missing actual")));
        assert!(!comparisons[3].passed);
        assert!(comparisons[3]
            .difference
            .as_deref()
            .is_some_and(|difference| difference.contains("missing expected")));
    }

    #[test]
    fn output_json_semantic_compares_whisperx_transcript_contract() {
        let difference =
            compare_json_output_values(semantic_expected_whisperx_json(), semantic_actual_json());

        assert_eq!(difference, None);
    }

    #[test]
    fn output_json_semantic_fails_changed_word_text() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["word_segments"][1]["word"] = serde_json::json!("planet");

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript word 1 text differs"));
    }

    #[test]
    fn output_json_semantic_fails_word_timing_beyond_tolerance() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["word_segments"][0]["start"] = serde_json::json!(0.200);

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript word 0 start timing differs"));
        assert!(difference.contains("tolerance=0.050s"));
    }

    #[test]
    fn output_json_semantic_fails_segment_timing_beyond_tolerance() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["segments"][0]["end"] = serde_json::json!(1.500);

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript segment 0 end timing differs"));
        assert!(difference.contains("tolerance=0.100s"));
    }

    #[test]
    fn output_json_semantic_fails_char_count_mismatch_when_chars_present() {
        let expected = semantic_expected_whisperx_json();
        let mut actual = semantic_actual_json();
        actual["segments"][0]["chars"] = serde_json::json!([
            {
                "char": "h",
                "start": 0.002,
                "end": 0.098
            }
        ]);

        let difference = compare_json_output_values(expected, actual).expect("should differ");

        assert!(difference.contains("JSON transcript char count differs"));
    }

    fn compare_json_output_values(
        expected: serde_json::Value,
        actual: serde_json::Value,
    ) -> Option<String> {
        let temp = tempfile::tempdir().expect("tempdir");
        let expected_path = temp.path().join("expected.json");
        let actual_path = temp.path().join("actual.json");
        fs::write(
            &expected_path,
            serde_json::to_string(&expected).expect("expected json"),
        )
        .expect("write expected json");
        fs::write(
            &actual_path,
            serde_json::to_string_pretty(&actual).expect("actual json"),
        )
        .expect("write actual json");
        compare_output_json(&expected_path, &actual_path).expect("json comparison")
    }

    fn semantic_expected_whisperx_json() -> serde_json::Value {
        serde_json::json!({
            "language": "en",
            "segments": [
                {
                    "start": 0.0,
                    "end": 1.2,
                    "text": " hello world",
                    "avg_logprob": -0.1,
                    "no_speech_prob": 0.01,
                    "words": [
                        {
                            "word": " hello",
                            "start": 0.0,
                            "end": 0.5,
                            "score": 0.9876
                        },
                        {
                            "word": "world",
                            "start": 0.55,
                            "end": 1.2,
                            "score": 0.902
                        }
                    ],
                    "chars": [
                        {
                            "char": "h",
                            "start": 0.0,
                            "end": 0.1
                        },
                        {
                            "char": "i",
                            "start": null,
                            "end": null
                        }
                    ]
                }
            ],
            "word_segments": [
                {
                    "word": " hello",
                    "start": 0.0,
                    "end": 0.5,
                    "score": 0.9876
                },
                {
                    "word": "world",
                    "start": 0.55,
                    "end": 1.2,
                    "score": 0.902
                }
            ]
        })
    }

    fn semantic_actual_json() -> serde_json::Value {
        serde_json::json!({
            "text": "hello world",
            "source": "sample.wav",
            "language": "en",
            "segments": [
                {
                    "id": 0,
                    "start": 0.004,
                    "end": 1.196,
                    "text": "hello world",
                    "score": 0.95,
                    "words": [
                        {
                            "word": "hello",
                            "start": 0.002,
                            "end": 0.501,
                            "score": 0.987
                        },
                        {
                            "word": " world",
                            "start": 0.552,
                            "end": 1.198,
                            "score": 0.9025
                        }
                    ],
                    "chars": [
                        {
                            "char": "h",
                            "start": 0.002,
                            "end": 0.098
                        },
                        {
                            "char": "i"
                        }
                    ]
                }
            ],
            "word_segments": [
                {
                    "word": "hello",
                    "start": 0.002,
                    "end": 0.501,
                    "score": 0.987
                },
                {
                    "word": " world",
                    "start": 0.552,
                    "end": 1.198,
                    "score": 0.9025
                }
            ]
        })
    }

    fn fixture_response_with_chars() -> TranscriptionPipelineResponse {
        let mut transcript = import_whisperx_json(WHISPERX_SAMPLE).expect("fixture should import");
        transcript.segments[0]
            .chars
            .push(text_transcripts::TranscriptCharContract {
                character: "h".to_string(),
                start_seconds: Some(0.0),
                end_seconds: Some(0.1),
                confidence: Some(0.9),
                attributes: Default::default(),
            });
        TranscriptionPipelineResponse {
            accepted: true,
            operation: "audio.transcription.transcribe".to_string(),
            provider: "fixture".to_string(),
            model_id: "fixture".to_string(),
            transcript,
            vad_segments: Vec::new(),
            alignment: None,
            diarization: None,
            artifacts: Vec::new(),
            diagnostics: Vec::new(),
        }
    }
}
