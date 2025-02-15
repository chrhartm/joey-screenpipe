use tauri_plugin_shell::{ShellExt, process::CommandChild};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent, MouseButtonState},
    Manager, RunEvent
};
use tauri_plugin_positioner::{Position, WindowExt};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

struct ChildCommands(Option<CommandChild>);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Build the Tauri application
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            
            // Load the icon
            let icon = app.default_window_icon()
                .expect("failed to get default icon")
                .clone();

            app.handle().plugin(tauri_plugin_positioner::init());
            let tray = TrayIconBuilder::new()
                .icon(icon)
                .menu(&menu)
                .on_tray_icon_event(|tray_handle, event| match event {
                    tauri::tray::TrayIconEvent::Click { button_state, button, .. }
                        if button_state == tauri::tray::MouseButtonState::Down 
                            && button == tauri::tray::MouseButton::Left =>
                    {
                        let window = tray_handle.app_handle().get_webview_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                        } else {
                            let _ = window.move_window(Position::TrayCenter);
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    _ => {
                        tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event)
                    }
                })
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|_app_handle, event| match event {
            tauri::WindowEvent::Focused(is_focused) => {
                // detect click outside of the focused window and hide the app
                if !is_focused {
                    let window = _app_handle.get_webview_window("main").unwrap();
                    window.hide().unwrap();
                }
            }
            _ => {}
        });
    let app = builder
        .build(tauri::generate_context!()).unwrap();

    // Start the actual Tauri app
    let mut child_commands = ChildCommands(None);
    app
    .run(move |app_handle, event| match event {
        RunEvent::Ready => {
            // Start screenpipe server
            let (_, screenpipe_child) = app_handle.
            shell()
            .sidecar("screenpipe")
            .unwrap()
            .spawn()
            .expect("Failed to start screenpipe");

            child_commands.0 = Some(screenpipe_child);
        }
        RunEvent::ExitRequested { code: _, api: _, .. } => {
            // Kill screenpipe server.
            // Tauri is supposed to do this with sidecars but with Screenpipe
            // it doesn't seem to be working
            if let Some(child) = child_commands.0.take() {
                child.kill().expect("Failed to close subprocesses");
            }
        }
        _ => {}
    });
}