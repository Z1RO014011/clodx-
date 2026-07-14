use clodx_lib::{parse_window_for_test, reset_credit_count_for_test};

#[test]
fn parses_camel_case_remaining_percentage() {
    let value = serde_json::json!({
        "remainingPercent": 73.4,
        "resetsAt": "2026-07-11T12:00:00Z",
        "windowSeconds": 18000
    });

    let window = parse_window_for_test(&value).expect("window should parse");
    assert_eq!(window.remaining_percent, 73.4);
    assert_eq!(window.window_seconds, 18_000);
}

#[test]
fn calculates_remaining_percentage_from_fractional_utilization() {
    let value = serde_json::json!({"utilization": 0.25, "windowSeconds": 604800});
    assert_eq!(parse_window_for_test(&value).unwrap().remaining_percent, 75.0);
}

#[test]
fn extracts_current_reset_credit_count() {
    let value = serde_json::json!({"available_count": 2});
    assert_eq!(reset_credit_count_for_test(&value), Some(2));
}
