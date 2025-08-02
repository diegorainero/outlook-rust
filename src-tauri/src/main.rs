use std::sync::Mutex;
use tauri::{
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager,
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Crea icona iniziale (blu Outlook)
            let icon = create_icon(0, 120, 215, 32);

            // Costruisci la tray icon
            let tray = TrayIconBuilder::new()
                .icon(tauri::image::Image::new_owned(icon, 32, 32))
                .tooltip("Outlook Viewer")
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Salva l'handle per modifiche successive
            app.manage(Mutex::new(tray));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![update_tray_icon])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                window.hide().unwrap();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_icon(r: u8, g: u8, b: u8, size: u32) -> Vec<u8> {
    let mut icon = vec![0; (size * size * 4) as usize];
    for i in (0..icon.len()).step_by(4) {
        icon[i] = r;
        icon[i + 1] = g;
        icon[i + 2] = b;
        icon[i + 3] = 255;
    }
    icon
}

#[tauri::command]
fn update_tray_icon(app: tauri::AppHandle, has_notifications: bool) {
    let icon = if has_notifications {
        create_icon(255, 0, 0, 32) // Rosso per notifiche
    } else {
        create_icon(0, 120, 215, 32) // Blu Outlook
    };

    if let Some(tray) = app.try_state::<Mutex<tauri::tray::TrayIcon>>() {
        let _ = tray
            .lock()
            .unwrap()
            .set_icon(Some(tauri::image::Image::new_owned(icon, 32, 32)));
    }
}
