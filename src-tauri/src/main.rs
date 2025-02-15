// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    Manager,
    menu::{MenuBuilder,},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent,},
};

fn main() {
    screenpipe_tauri_template_lib::run()
}
