use gamercade_interface::Session;
use tauri::async_runtime::Mutex;

#[derive(Default, Debug)]
pub enum AuthState {
    // Default State
    #[default]
    Unauthorized,

    // Holding Session
    SessionHeld(Session),
}

#[derive(Default)]
pub struct AppState {
    pub auth_state: Mutex<AuthState>,
}

impl AppState {
    pub async fn get_session(&self) -> Result<Session, String> {
        match &*self.auth_state.lock().await {
            AuthState::Unauthorized => Err("Session not found".to_string()),
            AuthState::SessionHeld(session) => Ok(session.clone()),
        }
    }
}
