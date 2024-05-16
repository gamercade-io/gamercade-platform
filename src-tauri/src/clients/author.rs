use gamercade_interface::{
    author::{AdjustAuthorRequest, AdjustAuthorResponse, PermissionLevel},
    common::Empty,
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::app_state::AppState;

use super::{author_client, WithSession};

#[tauri::command]
async fn adjust_game_author(
    state: State<'_, AppState>,
    game_id: i64,
    user_id: i64,
    title: Option<String>,
    permission_level_id: Option<i32>,
) -> Result<AdjustAuthorResponse, String> {
    let mut client = author_client().await?;

    let result = client
        .adjust_game_author(
            WithSession::new(
                &state.get_session().await?,
                AdjustAuthorRequest {
                    game_id,
                    user_id,
                    title,
                    permission_level_id,
                },
            )
            .authorized_request(),
        )
        .await
        .map_err(|e| e.to_string())
        .map(|r| r.into_inner())?;

    // TOOD: Update metadata
    Ok(result)
}

#[tauri::command]
async fn get_global_permission_levels() -> Result<Vec<PermissionLevel>, String> {
    let mut client = author_client().await?;

    let response = client
        .get_global_permission_levels(Empty {})
        .await
        .map_err(|e| e.to_string())?
        .into_inner();

    Ok(response.levels)
}

pub fn author_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("author")
        .invoke_handler(tauri::generate_handler![
            adjust_game_author,
            get_global_permission_levels
        ])
        .build()
}
