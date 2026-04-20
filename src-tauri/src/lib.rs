mod scanner;

#[tauri::command]
fn scan() -> String {
    let results = scanner::scan_all();
    serde_json::to_string_pretty(&results).unwrap_or_else(|e| format!("Error: {e}"))
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
