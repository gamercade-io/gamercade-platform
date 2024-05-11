use futures::TryFutureExt;
use gamercade_interface::{
    auth::{login_request::Provider, LoginRequest, SignUpRequest},
    Session,
};

use crate::{
    app_state::{AppState, AuthState},
    auth_client,
};

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

#[tauri::command]
async fn login(
    state: State<'_, AppState>,
    username: String,
    password: String,
) -> Result<(), String> {
    let mut client = auth_client().await?;

    let request = LoginRequest {
        provider: Some(Provider::Username(username)),
        password,
    };

    let response = client
        .login(request)
        .map_err(|e| e.to_string())
        .await?
        .into_inner();

    let session = Session::from(response.session);
    let mut lock = state.auth_state.lock().await;
    *lock = AuthState::SessionHeld(session);

    Ok(())
}

#[tauri::command]
async fn signup(
    state: State<'_, AppState>,
    username: String,
    email: String,
    password: String,
) -> Result<(), String> {
    let mut client = auth_client().await?;

    let request = SignUpRequest {
        username,
        email,
        password,
    };

    let response = client
        .sign_up(request)
        .map_err(|e| e.to_string())
        .await?
        .into_inner();

    let session = Session::from(response.session);
    let mut lock = state.auth_state.lock().await;
    *lock = AuthState::SessionHeld(session);

    Ok(())
}

#[tauri::command]
async fn update_password(
    state: State<'_, AppState>,
    previous: String,
    new: String,
) -> Result<(), String> {
    // TODO: Implement This
    Err("Not Implemented".to_string())
}

pub fn auth_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("auth")
        .invoke_handler(tauri::generate_handler![login, signup, update_password])
        .build()
}
