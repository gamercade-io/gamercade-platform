use gamercade_interface::game::{
    GameInfoBasic, GameInfoDetailed, MultipleGamesInfoResponse, MultipleGamesRequest,
    SingleGameRequest, UpdateGameRequest,
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::app_state::AppState;

use super::{game_client, WithSession};

#[tauri::command]
async fn get_single_game_info(game_id: i64) -> Result<GameInfoBasic, String> {
    let mut client = game_client().await?;

    // TODO: Update Metadata
    let game_info = client
        .get_single_game_info(SingleGameRequest { game_id })
        .await
        .map_err(|e| e.to_string())
        .map(|r| r.into_inner())?;

    Ok(game_info)
}

#[tauri::command]
async fn get_multiple_games_info(game_ids: Vec<i64>) -> Result<MultipleGamesInfoResponse, String> {
    let mut client = game_client().await?;

    // TODO: Update Metadata
    let game_info = client
        .get_multiple_games_info(MultipleGamesRequest { game_ids })
        .await
        .map_err(|e| e.to_string())
        .map(|r| r.into_inner())?;

    Ok(game_info)
}

#[tauri::command]
async fn get_game_detailed_info(game_id: i64) -> Result<GameInfoDetailed, String> {
    let mut client = game_client().await?;

    // TODO: Update Metadata
    let game_info = client
        .get_game_detailed_info(SingleGameRequest { game_id })
        .await
        .map_err(|e| e.to_string())
        .map(|r| r.into_inner())?;

    Ok(game_info)
}

#[tauri::command]
async fn create_game(
    state: State<'_, AppState>,
    game_title: String,
    short_description: String,
    long_description: Option<String>,
) -> Result<GameInfoBasic, String> {
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

    let game_info = client
        .create_game(request.authorized_request())
        .await
        .map_err(|e| e.to_string())
        .map(|r| r.into_inner())?;

    // TODO: Update Metadata
    Ok(game_info)
}

#[tauri::command]
async fn update_game(
    state: State<'_, AppState>,
    game_id: i64,
    game_title: Option<String>,
    short_description: Option<String>,
    long_description: Option<String>,
) -> Result<GameInfoBasic, String> {
    let mut client = game_client().await?;

    let request = WithSession::new(
        &state.get_session().await?,
        UpdateGameRequest {
            game_id: Some(game_id),
            title: game_title,
            short_description,
            long_description,
        },
    );

    let game_info = client
        .update_game(request.authorized_request())
        .await
        .map_err(|e| e.to_string())
        .map(|r| r.into_inner())?;

    // TODO: Update Metadata
    Ok(game_info)
}

#[tauri::command]
async fn delete_game(state: State<'_, AppState>, game_id: i64) -> Result<(), String> {
    let mut client = game_client().await?;

    let request = WithSession::new(&state.get_session().await?, SingleGameRequest { game_id });

    client
        .delete_game(request.authorized_request())
        .await
        .map_err(|e| e.to_string())?;

    // TODO: Update Metadata

    Ok(())
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
