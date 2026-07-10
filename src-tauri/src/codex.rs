use std::{fs, path::PathBuf};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION};
use serde_json::Value;
use crate::models::{ProviderSnapshot, UsageWindow};

const USAGE_URL: &str = "https://chatgpt.com/backend-api/wham/usage";
const CREDITS_URL: &str = "https://chatgpt.com/backend-api/wham/rate-limit-reset-credits";
const MAX_AUTH_BYTES: u64 = 256 * 1024;
const MAX_RESPONSE_BYTES: u64 = 1024 * 1024;

struct Auth { access_token: String, account_id: Option<String> }

fn auth_path() -> Option<PathBuf> {
    std::env::var_os("CODEX_HOME").map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|home| home.join(".codex")))
        .map(|home| home.join("auth.json"))
}
fn pick_string<'a>(value: &'a Value, keys: &[&str]) -> Option<&'a str> { keys.iter().find_map(|key| value.get(*key)?.as_str()) }
fn integer(value: &Value, keys: &[&str]) -> Option<u64> { keys.iter().find_map(|key| value.get(*key)?.as_u64().or_else(|| value.get(*key)?.as_i64().and_then(|item| u64::try_from(item).ok()))) }
fn timestamp(value: &Value, keys: &[&str]) -> Option<String> { keys.iter().find_map(|key| { let item = value.get(*key)?; item.as_str().map(str::to_owned).or_else(|| item.as_i64().and_then(|seconds| chrono::DateTime::from_timestamp(seconds, 0)).map(|time| time.to_rfc3339())) }) }

pub fn parse_window_for_test(value: &Value) -> Option<UsageWindow> { parse_window(Some(value)) }
pub fn reset_credit_count_for_test(value: &Value) -> Option<u64> { integer(value, &["available_count", "availableCount", "remaining", "count", "quantity"]) }
fn parse_window(value: Option<&Value>) -> Option<UsageWindow> {
    let value = value?;
    let remaining = ["remaining_percent", "remainingPercent", "remaining_pct", "remainingPct", "remaining_ratio", "remainingRatio", "remaining"].iter().find_map(|key| value.get(*key)?.as_f64()).or_else(|| {
        ["used_percent", "usedPercent", "used_pct", "usedPct", "used_ratio", "usedRatio", "utilization", "used"].iter().find_map(|key| value.get(*key)?.as_f64()).map(|used| 100.0 - if used <= 1.0 { used * 100.0 } else { used })
    })?;
    let key_is_ratio = ["remaining_ratio", "remainingRatio", "remaining"].iter().any(|key| value.get(*key).and_then(Value::as_f64).is_some_and(|number| number <= 1.0));
    Some(UsageWindow { remaining_percent: (if key_is_ratio { remaining * 100.0 } else { remaining }).clamp(0.0, 100.0), resets_at: timestamp(value, &["reset_at", "resetAt", "resets_at", "resetsAt", "reset_time", "resetTime"]), window_seconds: integer(value, &["limit_window_seconds", "limitWindowSeconds", "window_seconds", "windowSeconds", "duration_seconds", "durationSeconds", "period_seconds", "periodSeconds"]).unwrap_or(0) })
}
fn find_window<'a>(limit: &'a Value, names: &[&str], seconds: u64) -> Option<&'a Value> {
    for name in names { if limit.get(*name).and_then(|item| parse_window(Some(item))).is_some() { return limit.get(*name); } }
    for key in ["windows", "limit_windows", "limitWindows", "limits", "buckets"] { for item in limit.get(key).and_then(Value::as_array).into_iter().flatten() { if let Some(window) = parse_window(Some(item)) { if window.window_seconds.abs_diff(seconds) <= 60 { return Some(item); } } } }
    None
}
fn load_auth() -> Result<Auth, &'static str> {
    let path = auth_path().ok_or("Codex login was not found.")?;
    let metadata = fs::metadata(&path).map_err(|_| "Please sign in to Codex Desktop first.")?;
    if !metadata.is_file() || metadata.len() > MAX_AUTH_BYTES { return Err("Codex login data is unavailable."); }
    let value: Value = serde_json::from_str(&fs::read_to_string(path).map_err(|_| "Please sign in to Codex Desktop first.")?).map_err(|_| "Codex login format has changed.")?;
    let tokens = value.get("tokens").unwrap_or(&value);
    let access_token = pick_string(tokens, &["access_token", "accessToken"]).ok_or("Codex login expired. Please sign in again.")?.to_owned();
    let account_id = pick_string(tokens, &["account_id", "accountId"]).map(str::to_owned).or_else(|| {
        let payload = access_token.split('.').nth(1)?; let bytes = URL_SAFE_NO_PAD.decode(payload).ok()?; let claims: Value = serde_json::from_slice(&bytes).ok()?; pick_string(&claims, &["https://api.openai.com/auth.chatgpt_account_id", "chatgpt_account_id"]).map(str::to_owned)
    });
    Ok(Auth { access_token, account_id })
}
fn headers(auth: &Auth) -> Result<HeaderMap, &'static str> {
    let mut output = HeaderMap::new(); let mut bearer = HeaderValue::from_str(&format!("Bearer {}", auth.access_token)).map_err(|_| "Codex login data is invalid.")?; bearer.set_sensitive(true); output.insert(AUTHORIZATION, bearer); output.insert(ACCEPT, HeaderValue::from_static("application/json")); output.insert("originator", HeaderValue::from_static("Codex Desktop")); output.insert("OAI-Product-Sku", HeaderValue::from_static("CODEX")); if let Some(account_id) = &auth.account_id { let mut value = HeaderValue::from_str(account_id).map_err(|_| "Account identifier is invalid.")?; value.set_sensitive(true); output.insert("ChatGPT-Account-Id", value); } Ok(output)
}
async fn limited_json(mut response: reqwest::Response) -> Result<Value, ()> { if response.content_length().is_some_and(|length| length > MAX_RESPONSE_BYTES) { return Err(()); } let mut bytes = Vec::new(); while let Some(chunk) = response.chunk().await.map_err(|_| ())? { if bytes.len() + chunk.len() > MAX_RESPONSE_BYTES as usize { return Err(()); } bytes.extend_from_slice(&chunk); } serde_json::from_slice(&bytes).map_err(|_| ()) }
pub async fn fetch_snapshot(client: &reqwest::Client) -> ProviderSnapshot {
    let auth = match load_auth() { Ok(auth) => auth, Err(message) => return ProviderSnapshot::failure("signed_out", message) };
    let headers = match headers(&auth) { Ok(headers) => headers, Err(message) => return ProviderSnapshot::failure("signed_out", message) };
    let (usage_result, credits_result) = tokio::join!(client.get(USAGE_URL).headers(headers.clone()).send(), client.get(CREDITS_URL).headers(headers).send());
    let usage_response = match usage_result { Ok(response) if response.status().is_success() => response, Ok(response) if matches!(response.status().as_u16(), 401 | 403) => return ProviderSnapshot::failure("signed_out", "Codex login expired. Please sign in again."), Ok(_) | Err(_) => return ProviderSnapshot::failure("unavailable", "Quota service is temporarily unavailable.") };
    let usage = match limited_json(usage_response).await { Ok(value) => value, Err(_) => return ProviderSnapshot::failure("unavailable", "Quota response format has changed.") };
    let limit = usage.get("rate_limit").or_else(|| usage.get("rateLimit")).unwrap_or(&usage);
    let short_window = parse_window(find_window(limit, &["primary_window", "primaryWindow", "short_window", "shortWindow", "five_hour_window", "fiveHourWindow"], 18_000));
    if short_window.is_none() { return ProviderSnapshot::failure("unavailable", "Quota response is missing the 5h window."); }
    let usage_credits = usage.get("rate_limit_reset_credits").or_else(|| usage.get("rateLimitResetCredits")).and_then(reset_credit_count_for_test);
    let reset_credits = match credits_result { Ok(response) if response.status().is_success() => limited_json(response).await.ok().and_then(|value| reset_credit_count_for_test(&value)).or(usage_credits), _ => usage_credits };
    ProviderSnapshot { provider: "codex".into(), display_name: "CODEX".into(), plan: pick_string(&usage, &["plan_type", "planType"]).map(|value| value.to_uppercase()), short_window, weekly_window: parse_window(find_window(limit, &["secondary_window", "secondaryWindow", "weekly_window", "weeklyWindow"], 604_800)), reset_credits, reset_credit_expires_at: vec![], updated_at: chrono::Utc::now().to_rfc3339(), status: "ok".into(), message: None }
}
