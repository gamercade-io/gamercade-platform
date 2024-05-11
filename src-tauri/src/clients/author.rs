use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[tauri::command]
async fn adjust_game_author() -> Result<(), String> {
    Err("Not Implemented".to_string())
}

#[tauri::command]
async fn get_global_permission_levels() -> Result<(), String> {
    Err("Not Implemented".to_string())
}

pub fn author_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("author")
        .invoke_handler(tauri::generate_handler![
            adjust_game_author,
            get_global_permission_levels
        ])
        .build()
}
