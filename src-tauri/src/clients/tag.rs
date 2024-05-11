use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[tauri::command]
async fn adjust_game_tag() -> Result<(), String> {
    Err("Not Implemented".to_string())
}

#[tauri::command]
async fn get_global_tags() -> Result<(), String> {
    Err("Not Implemented".to_string())
}

pub fn tag_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("tag")
        .invoke_handler(tauri::generate_handler![adjust_game_tag, get_global_tags])
        .build()
}
