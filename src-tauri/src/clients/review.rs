use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use super::review_client;

#[tauri::command]
async fn review_game() -> Result<(), String> {
    let mut client = review_client().await?;

    // TODO: Auth

    Err("TODO: Not Implemented".to_string())
}

pub fn review_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("review")
        .invoke_handler(tauri::generate_handler![review_game])
        .build()
}
