use tauri::{AppHandle, Manager};
use tauri::tray::TrayIconId;
use tauri::menu::{Menu, MenuItem};

pub struct TrayState {
    pub id: TrayIconId,
}

impl TrayState {
    pub fn get_tray(app: &AppHandle) -> Option<tauri::tray::TrayIcon> {
        let state = app.state::<TrayState>();
        app.tray_by_id(&state.id)
    }
}

pub fn add_menu_item(app: &AppHandle, id: &str, label: &str) -> Result<(), Box<dyn std::error::Error>> {
    let Some(tray) = TrayState::get_tray(app) else {
        return Ok(());
    };

    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let open_i = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let new_i = MenuItem::with_id(app, id, label, true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&open_i, &new_i, &quit_i])?;
    tray.set_menu(Some(menu))?;

    Ok(())
}

pub fn setup_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let open_i = MenuItem::with_id(app, "open", "Open", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open_i, &quit_i])?;

    let mut tray_builder = tauri::tray::TrayIconBuilder::new();

    if let Some(icon) = app.default_window_icon() {
        tray_builder = tray_builder.icon(icon.clone());
    }

    let tray = tray_builder
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            if event.id.as_ref() == "quit" {
                app.exit(0);
            }
            if event.id.as_ref() == "open" {
                let window = app
                    .get_webview_window("CrushMainWindow")
                    .or_else(|| app.get_webview_window("crushBoostrapChoiceWindow"));

                if let Some(window) = window {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .on_tray_icon_event(|tray, event| {
            use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};

            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                let window = app
                    .get_webview_window("CrushMainWindow")
                    .or_else(|| app.get_webview_window("crushBoostrapChoiceWindow"));

                if let Some(window) = window {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    // Store the tray ID so we can retrieve it anywhere
    app.manage(TrayState { id: tray.id().clone() });

    Ok(())
}