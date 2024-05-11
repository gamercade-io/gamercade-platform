use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[tauri::command]
async fn front_page() -> Result<(), String> {
    Err("Not Implemented".to_string())
}

#[tauri::command]
async fn game_search() -> Result<(), String> {
    Err("Not Implemented".to_string())
}

#[tauri::command]
async fn get_editable_games() -> Result<(), String> {
    Err("Not Implemented".to_string())
}

#[tauri::command]
async fn get_voted_games() -> Result<(), String> {
    Err("Not Implemented".to_string())
}

pub fn platform_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("platform")
        .invoke_handler(tauri::generate_handler![
            front_page,
            game_search,
            get_editable_games,
            get_voted_games
        ])
        .build()
}
