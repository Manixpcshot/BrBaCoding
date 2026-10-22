use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, path::PathBuf};
use tauri::{AppHandle, Emitter, Manager};

fn state_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| format!("Could not create app data folder: {e}"))?;
    Ok(dir.join("workspace.json"))
}

#[tauri::command]
fn load_workspace(app: AppHandle) -> Result<Option<Value>, String> {
    let path = state_path(&app)?;
    if !path.exists() { return Ok(None); }
    let raw = fs::read_to_string(path).map_err(|e| format!("Could not read workspace: {e}"))?;
    serde_json::from_str(&raw).map(Some).map_err(|e| format!("Workspace file is invalid: {e}"))
}

#[tauri::command]
fn save_workspace(app: AppHandle, workspace: Value) -> Result<(), String> {
    let path = state_path(&app)?;
    let data = serde_json::to_vec_pretty(&workspace).map_err(|e| e.to_string())?;
    let temp = path.with_extension("json.tmp");
    fs::write(&temp, data).map_err(|e| format!("Could not save workspace: {e}"))?;
    fs::rename(temp, path).map_err(|e| format!("Could not finalize workspace: {e}"))
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProviderRequest { base_url: String, api_key: String }

fn endpoint(base: &str, suffix: &str) -> Result<String, String> {
    let clean = base.trim().trim_end_matches('/');
    if clean.is_empty() { return Err("Enter a provider base URL in Settings.".into()); }
    let mut url = clean.to_string();
    if !url.ends_with(suffix) { url.push('/'); url.push_str(suffix); }
    Ok(url)
}

#[tauri::command]
async fn fetch_models(request: ProviderRequest) -> Result<Value, String> {
    let url = endpoint(&request.base_url, "models")?;
    let response = reqwest::Client::new().get(url).bearer_auth(request.api_key.trim()).send().await.map_err(|e| format!("Could not reach provider: {e}"))?;
    let status = response.status();
    let body: Value = response.json().await.map_err(|e| format!("Provider returned invalid JSON: {e}"))?;
    if !status.is_success() { return Err(body.get("error").and_then(|e| e.get("message")).and_then(Value::as_str).unwrap_or("Model request failed. Check the base URL and API key.").to_string()); }
    Ok(body)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ChatRequest {
    base_url: String,
    api_key: String,
    model: String,
    messages: Vec<Value>,
    effort: String,
    request_id: String,
}

#[tauri::command]
async fn run_chat(app: AppHandle, request: ChatRequest) -> Result<(), String> {
    let url = endpoint(&request.base_url, "chat/completions")?;
    let body = json!({ "model": request.model, "messages": request.messages, "stream": true, "reasoning_effort": request.effort });
    let response = reqwest::Client::new().post(url).bearer_auth(request.api_key.trim()).json(&body).send().await.map_err(|e| format!("Could not reach provider: {e}"))?;
    let status = response.status();
    if !status.is_success() {
        let text = response.text().await.unwrap_or_default();
        let message = serde_json::from_str::<Value>(&text).ok().and_then(|v| v.get("error").and_then(|e| e.get("message")).and_then(Value::as_str).map(str::to_owned)).unwrap_or_else(|| format!("Request failed (HTTP {status}). {text}"));
        return Err(message);
    }
    let mut stream = response.bytes_stream();
    let mut buffer = String::new();
    while let Some(chunk) = stream.next().await {
        let bytes = chunk.map_err(|e| format!("Stream interrupted: {e}"))?;
        buffer.push_str(&String::from_utf8_lossy(&bytes));
        while let Some(index) = buffer.find('\n') {
            let line = buffer[..index].trim().to_string();
            buffer.drain(..=index);
            if let Some(data) = line.strip_prefix("data:").map(str::trim) {
                if data == "[DONE]" { continue; }
                if let Ok(value) = serde_json::from_str::<Value>(data) {
                    let delta = &value["choices"][0]["delta"];
                    let content = delta["content"].as_str();
                    if let Some(text) = content { let _ = app.emit("chat-stream", json!({"request_id":request.request_id,"text":text})); }
                }
            }
        }
    }
    let _ = app.emit("chat-stream-done", json!({"request_id":request.request_id}));
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_workspace, save_workspace, fetch_models, run_chat])
        .run(tauri::generate_context!())
        .expect("error while running Br Ba Coding Agent");
}
