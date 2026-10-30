use native_whisperx::{
    create_speaker_library, prepare_speaker_library_export, prepare_speaker_library_import,
    read_speaker_directory_state, ResolvedSpeakerDirectory, ResolvedSpeakerDirectoryScope,
    SpeakerLibraryImportMode,
};

#[test]
fn portable_index_round_trip_preserves_identities_and_keeps_trace_separate() {
    let root = tempfile::tempdir().unwrap();
    let first = root.path().join("first");
    let second = root.path().join("second");
    std::fs::create_dir(&first).unwrap();
    std::fs::write(
        first.join("library.json"),
        include_bytes!("../../../tests/fixtures/speaker-library-with-drafts.json"),
    )
    .unwrap();
    let exported = root.path().join("portable.json");
    let export = prepare_speaker_library_export(&first, &exported).unwrap();
    assert_eq!(export.summary().profile_count, 2);
    assert_eq!(export.summary().draft_profiles, 1);
    export.apply().unwrap();
    create_speaker_library(&second).unwrap();
    let trace = second.join("speaker-trace.json");
    std::fs::write(&trace, b"derived trace must remain untouched").unwrap();
    let import =
        prepare_speaker_library_import(&second, &exported, SpeakerLibraryImportMode::Merge)
            .unwrap();
    assert_eq!(import.summary().incoming_profiles, 2);
    let receipt = import.apply().unwrap();
    assert!(receipt.changed);
    let active = ResolvedSpeakerDirectory {
        path: second.clone(),
        scope: ResolvedSpeakerDirectoryScope::Explicit,
    };
    let state = read_speaker_directory_state(&active).unwrap();
    assert_eq!(state.library.profile_count, 2);
    assert_eq!(state.profiles[0].id, "confirmed-a");
    assert_eq!(
        state.profiles[1].metadata.get("status").map(String::as_str),
        Some("draft")
    );
    let modified = std::fs::metadata(second.join("library.json"))
        .unwrap()
        .modified()
        .unwrap();
    let repeated =
        prepare_speaker_library_import(&second, &exported, SpeakerLibraryImportMode::Merge)
            .unwrap();
    assert!(!repeated.apply().unwrap().changed);
    assert_eq!(
        std::fs::metadata(second.join("library.json"))
            .unwrap()
            .modified()
            .unwrap(),
        modified
    );
    assert_eq!(
        std::fs::read(trace).unwrap(),
        b"derived trace must remain untouched"
    );
}

#[test]
fn unreadable_derived_trace_does_not_hide_the_selected_library_state() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("library.json"),
        include_bytes!("../../../tests/fixtures/speaker-library-with-drafts.json"),
    )
    .unwrap();
    // A directory at the trace filename reliably exercises a filesystem read
    // failure without depending on the invoking user's permission privileges.
    std::fs::create_dir(root.path().join("speaker-trace.json")).unwrap();
    let active = ResolvedSpeakerDirectory {
        path: root.path().to_path_buf(),
        scope: ResolvedSpeakerDirectoryScope::Explicit,
    };
    let state = read_speaker_directory_state(&active)
        .expect("identity state survives a derived trace read failure");
    assert_eq!(state.library.profile_count, 2);
    assert_eq!(
        state.trace.status,
        native_whisperx::SpeakerTraceStateStatus::Invalid
    );
    assert!(state.trace.message.unwrap().contains("could not be read"));
    assert!(root.path().join("speaker-trace.json").is_dir());
}

#[test]
fn importing_an_empty_snapshot_into_an_absent_index_is_a_true_no_op() {
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("empty-source");
    create_speaker_library(&source).unwrap();
    let absent = root.path().join("absent-index");
    let preview = prepare_speaker_library_import(
        &absent,
        &source.join("library.json"),
        SpeakerLibraryImportMode::Merge,
    )
    .unwrap();
    assert!(!preview.summary().changed);
    assert!(!preview.apply().unwrap().changed);
    assert!(!absent.exists());
}
