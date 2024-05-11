use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[tauri::command]
async fn review_game() -> Result<(), String> {
    Err("Not Implemented".to_string())
}

pub fn review_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("review")
        .invoke_handler(tauri::generate_handler![review_game])
        .build()
}
