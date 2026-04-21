use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};
use tauri_plugin_updater::UpdaterExt;

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let scan_now = MenuItem::with_id(app, "scan_now", "Scan Now", true, None::<&str>)?;
    let open_dashboard =
        MenuItem::with_id(app, "open_dashboard", "Open Dashboard", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)?;
    let check_updates =
        MenuItem::with_id(app, "check_updates", "Check for Updates...", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit shwip", true, Some("CmdOrCtrl+Q"))?;

    let menu = Menu::with_items(
        app,
        &[
            &scan_now,
            &open_dashboard,
            &separator,
            &settings,
            &check_updates,
            &separator,
            &quit,
        ],
    )?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(true)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "scan_now" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("window.__shwipScanFromTray && window.__shwipScanFromTray()");
                }
            }
            "open_dashboard" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "settings" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    let _ = window.eval("window.__shwipOpenSettings && window.__shwipOpenSettings()");
                }
            }
            "check_updates" => {
                let handle = app.clone();
                tauri::async_runtime::spawn(async move {
                    match handle.updater().expect("updater not configured").check().await {
                        Ok(Some(update)) => {
                            tracing::info!(version = %update.version, "update available");
                            let _ = update.download_and_install(|_, _| {}, || {}).await;
                        }
                        Ok(None) => {
                            tracing::info!("no update available");
                        }
                        Err(e) => {
                            tracing::warn!("update check failed: {e}");
                        }
                    }
                });
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}

pub fn hide_dock(app: &AppHandle) {
    #[cfg(target_os = "macos")]
    {
        let _ = app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    }
}
