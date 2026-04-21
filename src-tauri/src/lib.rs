pub mod cli;
pub mod error;
pub mod history;
pub mod llm;
pub mod logger;
mod menu;
pub mod models;
mod notifications;
pub mod scanner;
pub mod scanners;
pub mod settings;
pub mod trash;

use history::{ScanHistory, ScanHistoryEntry};
use models::ScanResult;
use settings::Settings;
use tauri::Emitter;
use tauri_plugin_store::StoreExt;

#[tauri::command]
async fn clean_items(paths: Vec<String>) -> Result<u32, String> {
    let home = dirs::home_dir().ok_or_else(|| "cannot determine home directory".to_string())?;
    let mut count = 0u32;

    for path_str in &paths {
        let path = std::path::Path::new(path_str.as_str());
        let canonical = match path.canonicalize() {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!(path = %path_str, error = %e, "skipping: cannot canonicalize");
                continue;
            }
        };
        if !canonical.starts_with(&home) {
            tracing::warn!(path = %path_str, "rejected: path outside home directory");
            continue;
        }
        match crate::trash::move_to_trash(&canonical) {
            Ok(()) => count += 1,
            Err(e) => tracing::warn!(path = %path_str, error = %e, "failed to trash"),
        }
    }

    Ok(count)
}

#[tauri::command]
async fn scan(app: tauri::AppHandle) -> Result<Vec<ScanResult>, String> {
    let config = {
        let store = app.store("settings.json").map_err(|e| e.to_string())?;
        let s: Settings = store
            .get("settings")
            .and_then(|v| serde_json::from_value(v).ok())
            .unwrap_or_default();
        models::ScanConfig {
            profiles: s.profiles,
            exclusions: s.exclusions,
            min_size_bytes: models::ScanConfig::default().min_size_bytes,
        }
    };

    let emitter = app.clone();
    let results = scanner::scan_all_with_progress(config, move |name, ok| {
        let _ = emitter.emit(
            "scan-progress",
            serde_json::json!({ "scanner": name, "ok": ok }),
        );
    })
    .await
    .map_err(|e| e.to_string())?;

    let entry = history::record_scan(&results);
    save_history_entry(&app, entry);

    let total_bytes: u64 = results.iter().map(|r| r.size_bytes).sum();
    let size_gb = total_bytes as f64 / 1_073_741_824.0;
    notifications::notify_scan_complete(&app, results.len(), size_gb);

    Ok(results)
}

fn save_history_entry(app: &tauri::AppHandle, entry: ScanHistoryEntry) {
    let Ok(store) = app.store("settings.json") else {
        return;
    };
    let mut hist: ScanHistory = store
        .get("scan_history")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    hist.push(entry);
    if let Ok(val) = serde_json::to_value(&hist) {
        store.set("scan_history", val);
        let _ = store.save();
    }
}

#[tauri::command]
fn scan_history(app: tauri::AppHandle) -> Result<ScanHistory, String> {
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    let hist: ScanHistory = store
        .get("scan_history")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    Ok(hist)
}

#[tauri::command]
async fn explain_item(item: ScanResult) -> Result<String, String> {
    let client = llm::OllamaClient::default();
    Ok(client.explain_item(&item).await)
}

#[tauri::command]
fn load_settings(app: tauri::AppHandle) -> Result<Settings, String> {
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    match store.get("settings") {
        Some(val) => serde_json::from_value(val).map_err(|e| e.to_string()),
        None => Ok(Settings::default()),
    }
}

#[tauri::command]
fn save_settings(app: tauri::AppHandle, settings: Settings) -> Result<(), String> {
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    let val = serde_json::to_value(&settings).map_err(|e| e.to_string())?;
    store.set("settings", val);
    store.save().map_err(|e| e.to_string())
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            scan,
            clean_items,
            load_settings,
            save_settings,
            scan_history,
            explain_item
        ])
        .setup(|app| {
            let handle = app.handle().clone();
            menu::hide_dock(&handle);
            menu::setup_tray(&handle)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
