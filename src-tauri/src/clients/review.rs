use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::app_state::AppState;

use super::review_client;

#[tauri::command]
async fn review_game(state: State<'_, AppState>) -> Result<(), String> {
    let mut client = review_client().await?;

    // TODO: Auth

    Err("TODO: Not Implemented".to_string())
}

pub fn review_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("review")
        .invoke_handler(tauri::generate_handler![review_game])
        .build()
}
