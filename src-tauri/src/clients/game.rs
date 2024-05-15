use gamercade_interface::game::{
    MultipleGamesInfoResponse, MultipleGamesRequest, SingleGameRequest, UpdateGameRequest,
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::app_state::AppState;

use super::{game_client, WithSession};

#[tauri::command]
async fn get_single_game_info(game_id: i64) -> Result<(), String> {
    let mut client = game_client().await?;

    let result = client
        .get_single_game_info(SingleGameRequest { game_id })
        .await
        .map_err(|e| e.to_string())?;

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn get_multiple_games_info(game_ids: Vec<i64>) -> Result<(), String> {
    let mut client = game_client().await?;

    let result = client
        .get_multiple_games_info(MultipleGamesRequest { game_ids })
        .await
        .map_err(|e| e.to_string())?;

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn get_game_detailed_info(game_id: i64) -> Result<(), String> {
    let mut client = game_client().await?;

    let result = client
        .get_game_detailed_info(SingleGameRequest { game_id })
        .await
        .map_err(|e| e.to_string())?;

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn create_game(
    state: State<'_, AppState>,
    game_title: String,
    short_description: String,
    long_description: Option<String>,
) -> Result<(), String> {
    let mut client = game_client().await?;

    let request = WithSession::new(
        &state.get_session().await?,
        UpdateGameRequest {
            game_id: None,
            title: Some(game_title),
            short_description: Some(short_description),
            long_description: long_description,
        },
    );

    client
        .create_game(request.authorized_request())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn update_game(state: State<'_, AppState>) -> Result<(), String> {
    let mut client = game_client().await?;

    // TODO: Auth

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn delete_game(state: State<'_, AppState>) -> Result<(), String> {
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
