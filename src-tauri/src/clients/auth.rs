use futures::TryFutureExt;
use gamercade_interface::{
    auth::{login_request::Provider, LoginRequest, SignUpRequest, UpdatePasswordRequest},
    Session,
};

use crate::app_state::{AppState, AuthState};

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State,
};

use super::{auth_client, WithSession};

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

    let response = client
        .sign_up(SignUpRequest {
            username,
            email,
            password,
        })
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
    let mut client = auth_client().await?;

    client
        .update_password(
            WithSession::new(
                &state.get_session().await?,
                UpdatePasswordRequest {
                    previous_password: previous,
                    new_password: new,
                },
            )
            .authorized_request(),
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn auth_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("auth")
        .invoke_handler(tauri::generate_handler![login, signup, update_password])
        .build()
}
