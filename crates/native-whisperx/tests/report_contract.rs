use std::collections::BTreeSet;

use native_whisperx::NativeWhisperxReport;
use serde_json::Value;

const REPORT_V1: &str = include_str!("fixtures/native-whisperx-report-v1.json");
const CONTRACT_V1: &str = include_str!("fixtures/native-whisperx-report-v1.contract.json");

#[test]
fn product_report_v1_round_trips_without_shape_drift() {
    let expected: Value = serde_json::from_str(REPORT_V1).expect("parse v1 product report fixture");
    let report: NativeWhisperxReport =
        serde_json::from_value(expected.clone()).expect("deserialize v1 product report fixture");
    let actual = serde_json::to_value(report).expect("serialize v1 product report fixture");

    assert_eq!(actual, expected);
}

#[test]
fn product_report_v1_contract_manifest_matches_the_fixture() {
    let contract: Value = serde_json::from_str(CONTRACT_V1).expect("parse v1 contract manifest");
    let fixture: Value = serde_json::from_str(REPORT_V1).expect("parse v1 product report fixture");

    assert_eq!(contract["schemaVersion"], 1);
    assert_eq!(contract["contract"], "NativeWhisperxReport");
    assert_eq!(contract["fixture"], "native-whisperx-report-v1.json");

    let actual_fields = fixture
        .as_object()
        .expect("product report fixture is an object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let canonical_fields = contract["canonicalTopLevelFields"]
        .as_array()
        .expect("canonicalTopLevelFields is an array")
        .iter()
        .map(|field| field.as_str().expect("canonical field is a string"))
        .collect::<BTreeSet<_>>();

    assert_eq!(actual_fields, canonical_fields);

    for forbidden_field in contract["forbiddenTopLevelFields"]
        .as_array()
        .expect("forbiddenTopLevelFields is an array")
    {
        let forbidden_field = forbidden_field
            .as_str()
            .expect("forbidden field is a string");
        assert!(
            fixture.get(forbidden_field).is_none(),
            "product-owned report v1 must not leak upstream field {forbidden_field}"
        );
    }
}
