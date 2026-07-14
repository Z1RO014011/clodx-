use clodx_lib::format_tray_title;

#[test]
fn prefers_five_hour_quota_and_falls_back_to_weekly_quota() {
    assert_eq!(format_tray_title(Some(73.4), Some(97.0)), "5H 73%");
    assert_eq!(format_tray_title(None, Some(97.0)), "WK 97%");
    assert_eq!(format_tray_title(None, None), "C --");
}
