use gamercade_interface::{
    common::Empty,
    game::MultipleGamesInfoResponse,
    platform::{EditableGamesResponse, FrontPageRequest, FrontPageResponse, VotedGamesResponse},
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::app_state::AppState;

use super::{platform_client, WithSession};

#[tauri::command]
async fn front_page() -> Result<FrontPageResponse, String> {
    let mut client = platform_client().await?;

    client
        .front_page(FrontPageRequest {})
        .await
        .map_err(|e| e.to_string())
        .map(|r| r.into_inner())
}

#[tauri::command]
async fn game_search() -> Result<MultipleGamesInfoResponse, String> {
    let mut client = platform_client().await?;

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn get_editable_games(state: State<'_, AppState>) -> Result<EditableGamesResponse, String> {
    let mut client = platform_client().await?;

    client
        .get_editable_games(
            WithSession::new(&state.get_session().await?, Empty {}).authorized_request(),
        )
        .await
        .map_err(|e| e.to_string())
        .map(|r| r.into_inner())
}

#[tauri::command]
async fn get_voted_games(state: State<'_, AppState>) -> Result<VotedGamesResponse, String> {
    let mut client = platform_client().await?;

    client
        .get_voted_games(
            WithSession::new(&state.get_session().await?, Empty {}).authorized_request(),
        )
        .await
        .map_err(|e| e.to_string())
        .map(|r| r.into_inner())
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
