use native_whisperx::{
    export_reviewed_transcription, import_review_transcription, review_transcription, OutputConfig,
    OutputFormat, SegmentCorrection,
};

#[test]
fn corrected_copy_changes_only_the_selected_segment_and_reopens_canonical_exports() {
    let temp = tempfile::tempdir().unwrap();
    let source = temp.path().join("original.json");
    let original_bytes = include_bytes!("../../../tests/fixtures/whisperx-parity-sample.json");
    std::fs::write(&source, original_bytes).unwrap();
    let original = import_review_transcription(original_bytes).unwrap();
    let before = original.clone();
    let reviewed = review_transcription(
        &original,
        &[SegmentCorrection {
            segment_id: 1,
            text: "Corrected second speaker.".into(),
            start_seconds: 1.5,
            end_seconds: 2.8,
            speaker: Some("Alice".into()),
        }],
    )
    .unwrap();
    assert_eq!(original, before);
    assert_eq!(reviewed.transcript().segments[0], original.segments[0]);
    let corrected = &reviewed.transcript().segments[1];
    assert_eq!(corrected.index, 1);
    assert_eq!(corrected.text, "Corrected second speaker.");
    assert_eq!(corrected.start_seconds(), Some(1.5));
    assert_eq!(corrected.end_seconds(), Some(2.8));
    assert_eq!(corrected.speaker.as_deref(), Some("Alice"));
    assert!(
        corrected.words().is_empty(),
        "old word alignment must not describe corrected text"
    );
    assert_eq!(reviewed.cleared_alignment(), &[1]);
    let outputs = export_reviewed_transcription(
        &reviewed,
        &OutputConfig {
            output_dir: Some(temp.path().to_path_buf()),
            basename: Some("corrected".into()),
            formats: vec![
                OutputFormat::NativeJson,
                OutputFormat::Json,
                OutputFormat::Srt,
            ],
            ..OutputConfig::default()
        },
    )
    .unwrap();
    assert_eq!(outputs.len(), 3);
    let reopened = import_review_transcription(
        &std::fs::read(temp.path().join("corrected.native.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(&reopened, reviewed.transcript());
    let srt = std::fs::read_to_string(temp.path().join("corrected.srt")).unwrap();
    assert!(
        srt.contains("00:00:01,500 --> 00:00:02,800\n[Alice]: Corrected second speaker."),
        "{srt}"
    );
    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(temp.path().join("corrected.json")).unwrap())
            .unwrap();
    assert_eq!(json["segments"][1]["speaker"], "Alice");
    assert_eq!(json["segments"][1]["start"], 1.5);
    assert_eq!(std::fs::read(&source).unwrap(), original_bytes);
}

#[test]
fn source_and_translation_corrections_remain_separate_in_the_canonical_copy() {
    use native_whisperx::{review_translation_source, translation_source_transcription};
    let source = import_review_transcription(include_bytes!(
        "../../../tests/fixtures/whisperx-parity-sample.json"
    ))
    .unwrap();
    let target_edits = source
        .segments
        .iter()
        .map(|segment| SegmentCorrection {
            segment_id: segment.index,
            text: format!("Translated {}", segment.index),
            start_seconds: segment.start_seconds().unwrap(),
            end_seconds: segment.end_seconds().unwrap(),
            speaker: segment.speaker.clone(),
        })
        .collect::<Vec<_>>();
    let mut translated = review_transcription(&source, &target_edits)
        .unwrap()
        .transcript()
        .clone();
    translated.language = Some("de".into());
    translated.attributes.insert(
        "nativeWhisperx.translation.source".into(),
        serde_json::to_string(&source).unwrap(),
    );
    translated.attributes.insert(
        "nativeWhisperx.translation.provenance".into(),
        r#"{"modelId":"fixture/en-de","sourceLanguage":"en","targetLanguage":"de"}"#.into(),
    );
    let target_before = translated.segments.clone();
    let provenance = translated.attributes["nativeWhisperx.translation.provenance"].clone();
    let correction = SegmentCorrection {
        segment_id: 1,
        text: "Changed translation only".into(),
        start_seconds: 1.4,
        end_seconds: 2.5,
        speaker: Some("Alice".into()),
    };
    let reviewed_target = review_transcription(&translated, &[correction]).unwrap();
    assert_eq!(
        translation_source_transcription(reviewed_target.transcript()).unwrap(),
        Some(source.clone())
    );
    let reviewed_source = review_translation_source(
        &translated,
        &[SegmentCorrection {
            segment_id: 1,
            text: "Changed source only".into(),
            start_seconds: 1.4,
            end_seconds: 2.5,
            speaker: Some("Bob".into()),
        }],
    )
    .unwrap();
    assert_eq!(reviewed_source.transcript().segments, target_before);
    assert_eq!(
        reviewed_source.transcript().attributes["nativeWhisperx.translation.provenance"],
        provenance
    );
    let corrected_source = translation_source_transcription(reviewed_source.transcript())
        .unwrap()
        .unwrap();
    assert_eq!(corrected_source.segments[0], source.segments[0]);
    assert_eq!(corrected_source.segments[1].text, "Changed source only");
    let temp = tempfile::tempdir().unwrap();
    native_whisperx::export_reviewed_transcription_layer(
        &reviewed_source,
        &OutputConfig {
            output_dir: Some(temp.path().to_path_buf()),
            basename: Some("bilingual".into()),
            formats: vec![OutputFormat::NativeJson, OutputFormat::Srt],
            ..OutputConfig::default()
        },
        native_whisperx::ReviewExportLayer::Source,
    )
    .unwrap();
    assert!(std::fs::read_to_string(temp.path().join("bilingual.srt"))
        .unwrap()
        .contains("[Bob]: Changed source only"));
    let reopened = import_review_transcription(
        &std::fs::read(temp.path().join("bilingual.native.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(reopened.segments, target_before);
    assert_eq!(
        translation_source_transcription(&reopened).unwrap(),
        Some(corrected_source)
    );
}

#[test]
fn invalid_correction_times_are_rejected_without_sorting_and_supported_overlap_is_preserved() {
    let original = import_review_transcription(include_bytes!(
        "../../../tests/fixtures/whisperx-parity-sample.json"
    ))
    .unwrap();
    for (id, start, end) in [(1, f64::NAN, 2.4), (1, 2.0, 1.0), (0, 2.0, 3.0)] {
        assert!(review_transcription(
            &original,
            &[SegmentCorrection {
                segment_id: id,
                text: "Correction".into(),
                start_seconds: start,
                end_seconds: end,
                speaker: None
            }]
        )
        .is_err());
    }
    let overlap = review_transcription(
        &original,
        &[SegmentCorrection {
            segment_id: 1,
            text: "Overlapping speech".into(),
            start_seconds: 1.0,
            end_seconds: 2.4,
            speaker: Some("SPEAKER_01".into()),
        }],
    )
    .unwrap();
    assert_eq!(
        overlap
            .transcript()
            .segments
            .iter()
            .map(|segment| segment.index)
            .collect::<Vec<_>>(),
        [0, 1]
    );
    assert_eq!(overlap.transcript().segments[1].start_seconds(), Some(1.0));
    assert_eq!(overlap.transcript().segments[0], original.segments[0]);
}

#[test]
fn speaker_annotation_preserves_word_times_and_conflicting_export_preserves_original_files() {
    let original_bytes = include_bytes!("../../../tests/fixtures/whisperx-parity-sample.json");
    let original = import_review_transcription(original_bytes).unwrap();
    let reviewed = review_transcription(
        &original,
        &[SegmentCorrection {
            segment_id: 1,
            text: original.segments[1].text.clone(),
            start_seconds: 1.35,
            end_seconds: 2.4,
            speaker: Some("Alice".into()),
        }],
    )
    .unwrap();
    assert!(reviewed.cleared_alignment().is_empty());
    assert_eq!(reviewed.transcript().segments[0], original.segments[0]);
    for (word, before) in reviewed.transcript().segments[1]
        .words()
        .iter()
        .zip(original.segments[1].words())
    {
        assert_eq!(word.text, before.text);
        assert_eq!(word.start_seconds(), before.start_seconds());
        assert_eq!(word.end_seconds(), before.end_seconds());
        assert_eq!(word.speaker.as_deref(), Some("Alice"));
    }
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("original.json");
    std::fs::write(&path, original_bytes).unwrap();
    let result = export_reviewed_transcription(
        &reviewed,
        &OutputConfig {
            output_dir: Some(temp.path().to_path_buf()),
            basename: Some("original".into()),
            formats: vec![OutputFormat::Srt, OutputFormat::Json],
            ..OutputConfig::default()
        },
    );
    assert!(result.unwrap_err().to_string().contains("already exists"));
    assert_eq!(std::fs::read(&path).unwrap(), original_bytes);
    assert!(!temp.path().join("original.srt").exists());
    assert_eq!(std::fs::read_dir(temp.path()).unwrap().count(), 1);
}
