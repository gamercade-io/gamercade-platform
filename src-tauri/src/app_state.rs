use gamercade_interface::Session;
use tauri::async_runtime::Mutex;

use crate::metadata::Metadata;

#[derive(Default, Debug)]
pub enum AuthState {
    // Default State
    #[default]
    Unauthorized,

    // Holding Session
    SessionHeld(Session),
}

pub struct AppState {
    pub auth_state: Mutex<AuthState>,
    pub metadata: Mutex<Metadata>,
}

impl std::default::Default for AppState {
    fn default() -> Self {
        Self {
            auth_state: Default::default(),
            metadata: Mutex::new(Metadata::new()),
        }
    }
}

impl AppState {
    pub async fn get_session(&self) -> Result<Session, String> {
        match &*self.auth_state.lock().await {
            AuthState::Unauthorized => Err("Session not found".to_string()),
            AuthState::SessionHeld(session) => Ok(session.clone()),
        }
    }
}
