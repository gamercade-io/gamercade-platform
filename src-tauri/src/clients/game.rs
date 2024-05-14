use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use super::game_client;

#[tauri::command]
async fn get_single_game_info() -> Result<(), String> {
    let mut client = game_client().await?;

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn get_multiple_games_info() -> Result<(), String> {
    let mut client = game_client().await?;

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn get_game_detailed_info() -> Result<(), String> {
    let mut client = game_client().await?;

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn create_game(
    game_title: String,
    short_description: String,
    long_description: Option<String>,
) -> Result<(), String> {
    let mut client = game_client().await?;

    // TODO: Auth 

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn update_game() -> Result<(), String> {
    let mut client = game_client().await?;

    // TODO: Auth

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn delete_game() -> Result<(), String> {
    let mut client = game_client().await?;

    // TODO: Auth

    Err("TODO: Not Implemented".to_string())
}

pub fn game_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("game")
        .invoke_handler(tauri::generate_handler![
            get_single_game_info,
            get_multiple_games_info,
            get_game_detailed_info,
            create_game,
            update_game,
            delete_game
        ])
        .build()
}
