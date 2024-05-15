use gamercade_interface::{author::PermissionLevel, common::Empty};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::app_state::AppState;

use super::author_client;

#[tauri::command]
async fn adjust_game_author(state: State<'_, AppState>) -> Result<(), String> {
    let mut client = author_client().await?;
    // TODO: Auth
    Err("TODO: Not Implemented".to_string())
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
