#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(non_snake_case)]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod help;

use help::processes;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![processes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
