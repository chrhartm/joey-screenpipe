// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_shell::{ShellExt, process::CommandChild};

use tauri::{
    RunEvent,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

struct ChildCommands(Option<CommandChild>);

fn main() {
    let app = tauri::Builder::default()
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;
            let tray = TrayIconBuilder::new()
                .menu(&menu)
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
        }).unwrap();

        let mut child_commands = ChildCommands(None);

        app.run(move |app_handle, event| match event {
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
        })
        .expect("error while running tauri application");
}