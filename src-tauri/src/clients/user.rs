use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use super::user_client;

#[tauri::command]
async fn get_user_info() -> Result<(), String> {
    let mut client = user_client().await?;

    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn get_users_info() -> Result<(), String> {
    Err("TODO: Not Implemented".to_string())
}

#[tauri::command]
async fn update_email() -> Result<(), String> {
    Err("TODO: Not Implemented".to_string())
}

pub fn user_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("user")
        .invoke_handler(tauri::generate_handler![
            get_user_info,
            get_users_info,
            update_email
        ])
        .build()
}
