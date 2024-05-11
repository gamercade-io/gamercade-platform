// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{app_state::AppState, auth::auth_plugin};

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(auth_plugin())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
