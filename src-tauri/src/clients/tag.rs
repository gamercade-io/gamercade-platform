use gamercade_interface::{
    common::Empty,
    tag::{AdjustGameTagRequest, AdjustGameTagResponse, Tag},
};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::app_state::AppState;

use super::{tag_client, WithSession};

#[tauri::command]
async fn adjust_game_tag(
    state: State<'_, AppState>,
    game_id: i64,
    tag_id: i32,
    set_to: bool,
) -> Result<AdjustGameTagResponse, String> {
    let mut client = tag_client().await?;

    client
        .adjust_game_tag(
            WithSession::new(
                &state.get_session().await?,
                AdjustGameTagRequest {
                    game_id,
                    tag_id,
                    set_to,
                },
            )
            .authorized_request(),
        )
        .await
        .map_err(|e| e.to_string())
        .map(|r| r.into_inner())
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
