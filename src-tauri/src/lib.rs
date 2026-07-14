mod codex;
mod models;
pub use codex::{parse_window_for_test, reset_credit_count_for_test};
pub use models::UsageWindow;
use std::{sync::Mutex, time::{Duration, Instant}};
use models::ProviderSnapshot;
use tauri::{menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}, AppHandle, Emitter, Manager, State};

pub fn format_tray_title(percent: Option<f64>) -> String {
    percent.map(|value| format!("C {:.0}%", value.clamp(0.0, 100.0))).unwrap_or_else(|| "C --".into())
}

struct AppState { client: reqwest::Client, cache: Mutex<Option<(Instant, Vec<ProviderSnapshot>)>>, refresh: tokio::sync::Mutex<()> }
#[tauri::command]
async fn get_snapshots(state: State<'_, AppState>) -> Result<Vec<ProviderSnapshot>, String> {
    if let Ok(cache) = state.cache.lock() { if let Some((time, value)) = &*cache { if time.elapsed() < Duration::from_secs(30) { return Ok(value.clone()); } } }
    refresh(state).await
}
#[tauri::command]
async fn refresh_snapshots(state: State<'_, AppState>) -> Result<Vec<ProviderSnapshot>, String> { refresh(state).await }
async fn refresh(state: State<'_, AppState>) -> Result<Vec<ProviderSnapshot>, String> { let _guard = state.refresh.lock().await; let value = vec![codex::fetch_snapshot(&state.client).await]; if let Ok(mut cache) = state.cache.lock() { *cache = Some((Instant::now(), value.clone())); } Ok(value) }
#[tauri::command]
fn set_tray_quota(percent: Option<f64>, app: AppHandle) -> Result<(), String> {
    let title = format_tray_title(percent);
    let tray = app.tray_by_id("status").ok_or("status tray unavailable")?;
    tray.set_title(Some(title.clone())).map_err(|error| error.to_string())?;
    tray.set_tooltip(Some(title)).map_err(|error| error.to_string())
}
#[tauri::command]
fn show_main_window(app: AppHandle) -> Result<(), String> {
    let window = app.get_webview_window("main").ok_or("main window unavailable")?;
    window.show().map_err(|error| error.to_string())?;
    window.set_focus().map_err(|error| error.to_string())
}
fn toggle_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) { let _ = window.hide(); } else { let _ = window.show(); let _ = window.set_focus(); }
    }
}
fn setup_tray(app: &tauri::App) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "显示 / 隐藏", true, None::<&str>)?;
    let refresh = MenuItem::with_id(app, "refresh", "立即刷新", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "退出 Clodx", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &refresh, &quit])?;
    let mut builder = TrayIconBuilder::with_id("status").menu(&menu).title("C --").tooltip("C --");
    if let Some(icon) = app.default_window_icon() { builder = builder.icon(icon.clone()); }
    builder.on_menu_event(|app, event| match event.id.as_ref() {
        "show" => toggle_window(app),
        "refresh" => { let _ = app.emit("refresh-requested", ()); },
        "quit" => app.exit(0),
        _ => {}
    }).on_tray_icon_event(|app, event| {
        if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event { toggle_window(app.app_handle()); }
    }).build(app)?;
    Ok(())
}
pub fn run() {
    tauri::Builder::default().setup(|app| { let client = reqwest::Client::builder().timeout(Duration::from_secs(12)).redirect(reqwest::redirect::Policy::none()).user_agent("Clodx/0.1").build().expect("valid static client"); app.manage(AppState { client, cache: Mutex::new(None), refresh: tokio::sync::Mutex::new(()) }); setup_tray(app)?; if let Some(window) = app.get_webview_window("main") { let _ = window.set_always_on_top(true); } Ok(()) }).invoke_handler(tauri::generate_handler![get_snapshots, refresh_snapshots, set_tray_quota, show_main_window]).run(tauri::generate_context!()).expect("run Clodx");
}
