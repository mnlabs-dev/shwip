use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

const THRESHOLD_BYTES: u64 = 500_000_000;

pub fn notify_scan_complete(app: &AppHandle, count: usize, total_bytes: u64) {
    let size_gb = total_bytes as f64 / 1_073_741_824.0;

    let body = if count == 0 {
        "Your Mac is clean. Nothing to remove.".to_string()
    } else if total_bytes >= THRESHOLD_BYTES {
        format!("{count} items found, {size_gb:.1} GB reclaimable. Time to clean up!")
    } else {
        format!("{count} items found ({size_gb:.1} GB). Your Mac is fairly clean.")
    };

    let _ = app.notification().builder().title("shwip").body(&body).show();
}
