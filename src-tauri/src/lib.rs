mod codex;
mod models;
pub use codex::{parse_window_for_test, reset_credit_count_for_test};
pub use models::UsageWindow;
use std::{sync::Mutex, time::{Duration, Instant}};
use models::ProviderSnapshot;
use tauri::{Manager, State};

struct AppState { client: reqwest::Client, cache: Mutex<Option<(Instant, Vec<ProviderSnapshot>)>>, refresh: tokio::sync::Mutex<()> }
#[tauri::command]
async fn get_snapshots(state: State<'_, AppState>) -> Result<Vec<ProviderSnapshot>, String> {
    if let Ok(cache) = state.cache.lock() { if let Some((time, value)) = &*cache { if time.elapsed() < Duration::from_secs(30) { return Ok(value.clone()); } } }
    refresh(state).await
}
#[tauri::command]
async fn refresh_snapshots(state: State<'_, AppState>) -> Result<Vec<ProviderSnapshot>, String> { refresh(state).await }
async fn refresh(state: State<'_, AppState>) -> Result<Vec<ProviderSnapshot>, String> { let _guard = state.refresh.lock().await; let value = vec![codex::fetch_snapshot(&state.client).await]; if let Ok(mut cache) = state.cache.lock() { *cache = Some((Instant::now(), value.clone())); } Ok(value) }
pub fn run() {
    tauri::Builder::default().setup(|app| { let client = reqwest::Client::builder().timeout(Duration::from_secs(12)).redirect(reqwest::redirect::Policy::none()).user_agent("Clodx/0.1").build().expect("valid static client"); app.manage(AppState { client, cache: Mutex::new(None), refresh: tokio::sync::Mutex::new(()) }); if let Some(window) = app.get_webview_window("main") { let _ = window.set_always_on_top(true); } Ok(()) }).invoke_handler(tauri::generate_handler![get_snapshots, refresh_snapshots]).run(tauri::generate_context!()).expect("run Clodx");
}
