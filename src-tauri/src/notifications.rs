use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

pub fn notify_scan_complete(app: &AppHandle, count: usize, size_gb: f64) {
    let body = if count == 0 {
        "Nothing to clean. Your Mac is tidy.".to_string()
    } else {
        format!(
            "{} items found, {:.1} GB reclaimable",
            count, size_gb
        )
    };

    let _ = app
        .notification()
        .builder()
        .title("shwip")
        .body(&body)
        .show();
}
