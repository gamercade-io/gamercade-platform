use std::{thread, time::Duration};

use gamercade_interface::{
    auth::{auth_service_client::AuthServiceClient, login_request::Provider, LoginRequest},
    Session,
};

use crate::auth_client;

use super::SERVICE_IP_GRPC;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

#[tauri::command]
async fn try_login(username: String, password: String) -> Result<(), String> {
    println!("trying to login...");
    let mut client = auth_client().await?;

    let request = LoginRequest {
        provider: Some(Provider::Username(username)),
        password,
    };

    let response = client.login(request).await;

    match response {
        Ok(ok) => {
            let response = ok.into_inner();
            let session = Session::from(response.session);
            println!("logged in successfully!");
            Ok(())
        }
        Err(err) => Err(format!("try_login error: {err}")),
    }
}

#[tauri::command]
async fn try_signup(username: String, email: String, password: String) -> Result<(), String> {
    thread::sleep(Duration::from_secs(1));
    Err("Not Implemented".to_string())
}

pub fn auth_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("auth")
        .invoke_handler(tauri::generate_handler![try_login, try_signup])
        .build()
}
