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
