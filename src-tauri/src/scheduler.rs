use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Emitter};
use tauri_plugin_store::StoreExt;

use crate::{history, models, notifications, settings::Settings};

pub static SCANNING: AtomicBool = AtomicBool::new(false);

pub struct ScanGuard;

impl ScanGuard {
    pub fn acquire() -> Option<Self> {
        SCANNING
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .ok()
            .map(|_| Self)
    }
}

impl Drop for ScanGuard {
    fn drop(&mut self) {
        SCANNING.store(false, Ordering::SeqCst);
    }
}

pub fn start(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut last_scan = Instant::now() - Duration::from_secs(365 * 24 * 3600);
        tokio::time::sleep(Duration::from_secs(60)).await;

        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;

            let settings = load_settings(&app);
            if !settings.schedule_enabled {
                continue;
            }

            let interval = Duration::from_secs(settings.schedule_interval_hours * 3600);
            if last_scan.elapsed() < interval {
                continue;
            }

            let _guard = match ScanGuard::acquire() {
                Some(g) => g,
                None => continue,
            };

            tracing::info!("scheduled scan starting");

            let config = models::ScanConfig {
                profiles: settings.profiles.clone(),
                exclusions: settings.exclusions.clone(),
                min_size_bytes: models::ScanConfig::default().min_size_bytes,
            };

            match crate::scanner::scan_all_with_progress(config, |_, _| {}).await {
                Ok(results) => {
                    let total: u64 = results.iter().map(|r| r.size_bytes).sum();
                    let entry = history::record_scan(&results);
                    save_history(&app, entry);
                    if settings.show_notifications {
                        notifications::notify_scan_complete(&app, results.len(), total);
                    }
                    let _ = app.emit("scheduled-scan-complete", results.len());
                    tracing::info!(count = results.len(), "scheduled scan complete");
                }
                Err(e) => {
                    tracing::warn!("scheduled scan failed: {e}");
                }
            }

            last_scan = Instant::now();
        }
    });
}

fn load_settings(app: &AppHandle) -> Settings {
    app.store("settings.json")
        .ok()
        .and_then(|store| store.get("settings"))
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default()
}

fn save_history(app: &AppHandle, entry: history::ScanHistoryEntry) {
    let Ok(store) = app.store("settings.json") else {
        return;
    };
    let mut hist: history::ScanHistory = store
        .get("scan_history")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    hist.push(entry);
    if let Ok(val) = serde_json::to_value(&hist) {
        store.set("scan_history", val);
        let _ = store.save();
    }
}
