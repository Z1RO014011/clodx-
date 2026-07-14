use clodx_lib::format_tray_title;

#[test]
fn formats_primary_quota_for_the_status_bar() {
    assert_eq!(format_tray_title(Some(73.4)), "C 73%");
    assert_eq!(format_tray_title(Some(9.8)), "C 10%");
    assert_eq!(format_tray_title(None), "C --");
}
