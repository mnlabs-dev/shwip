use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let scan_now = MenuItem::with_id(app, "scan_now", "Scan Now", true, None::<&str>)?;
    let open_dashboard =
        MenuItem::with_id(app, "open_dashboard", "Open Dashboard", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit shwip", true, Some("CmdOrCtrl+Q"))?;

    let menu = Menu::with_items(
        app,
        &[
            &scan_now,
            &open_dashboard,
            &separator,
            &settings,
            &separator,
            &quit,
        ],
    )?;

    let mut builder = TrayIconBuilder::new()
        .icon_as_template(true)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "scan_now" => {
                let _ = app.emit("tray-scan", ());
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
                }
                let _ = app.emit("tray-settings", ());
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        });

    let tray_rgba = include_bytes!("../icons/tray-icon.rgba");
    let tray_icon = tauri::image::Image::new(tray_rgba, 64, 64);
    builder = builder.icon(tray_icon);

    builder.build(app)?;
    Ok(())
}

pub fn hide_dock(app: &AppHandle) {
    #[cfg(target_os = "macos")]
    {
        let _ = app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    }
}
