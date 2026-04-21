pub mod error;
mod menu;
pub mod models;
mod notifications;
pub mod scanner;
pub mod scanners;
pub mod settings;
pub mod trash;

use models::ScanResult;
use settings::Settings;
use tauri::Manager;
use tauri_plugin_store::StoreExt;

#[tauri::command]
async fn scan(app: tauri::AppHandle) -> Result<Vec<ScanResult>, String> {
    let results = scanner::scan_all().await.map_err(|e| e.to_string())?;

    let total_bytes: u64 = results.iter().map(|r| r.size_bytes).sum();
    let size_gb = total_bytes as f64 / 1_073_741_824.0;
    notifications::notify_scan_complete(&app, results.len(), size_gb);

    Ok(results)
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
        .invoke_handler(tauri::generate_handler![scan, load_settings, save_settings])
        .setup(|app| {
            let handle = app.handle().clone();
            menu::hide_dock(&handle);
            menu::setup_tray(&handle)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
