// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{app_state::AppState, *};

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(auth_plugin())
        .plugin(author_plugin())
        .plugin(game_plugin())
        .plugin(platform_plugin())
        .plugin(review_plugin())
        .plugin(tag_plugin())
        .plugin(user_plugin())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
