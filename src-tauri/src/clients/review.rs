use gamercade_interface::review::ReviewGameRequest;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use crate::app_state::AppState;

use super::{review_client, WithSession};

#[tauri::command]
async fn review_game(
    state: State<'_, AppState>,
    game_id: i64,
    rating: Option<bool>,
) -> Result<(), String> {
    let mut client = review_client().await?;

    let request = WithSession::new(
        &state.get_session().await?,
        ReviewGameRequest { game_id, rating },
    );

    client
        .review_game(request.authorized_request())
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn review_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("review")
        .invoke_handler(tauri::generate_handler![review_game])
        .build()
}
