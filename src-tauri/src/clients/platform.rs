use gamercade_interface::{
    game::MultipleGamesInfoResponse,
    platform::{EditableGamesResponse, FrontPageResponse, VotedGamesResponse},
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::app_state::AppState;

use super::platform_client;

#[tauri::command]
async fn front_page() -> Result<FrontPageResponse, String> {
    let mut client = platform_client().await?;

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn game_search() -> Result<MultipleGamesInfoResponse, String> {
    let mut client = platform_client().await?;

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn get_editable_games(state: State<'_, AppState>) -> Result<EditableGamesResponse, String> {
    let mut client = platform_client().await?;

    // TODO: Auth

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn get_voted_games(state: State<'_, AppState>) -> Result<VotedGamesResponse, String> {
    let mut client = platform_client().await?;

    // TODO: Auth

    Err("TODO: Not Implemented".to_string())
}

pub fn platform_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("platform")
        .invoke_handler(tauri::generate_handler![
            front_page,
            game_search,
            get_editable_games,
            get_voted_games
        ])
        .build()
}
