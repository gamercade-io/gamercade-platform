use gamercade_interface::{common::Empty, tag::Tag};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use super::tag_client;

#[tauri::command]
async fn adjust_game_tag() -> Result<(), String> {
    let mut client = tag_client().await?;

    // TODO: Auth

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn get_global_tags() -> Result<Vec<Tag>, String> {
    let mut client = tag_client().await?;

    let response = client
        .get_global_tags(Empty {})
        .await
        .map_err(|e| e.to_string())?
        .into_inner();

    Ok(response.tags)
}

pub fn tag_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("tag")
        .invoke_handler(tauri::generate_handler![adjust_game_tag, get_global_tags])
        .build()
}
