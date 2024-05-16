pub const SERVICE_IP_GRPC: &str = "http://127.0.0.1:50051";
pub const SERVICE_IP_HTTP: &str = "http://127.0.0.1:3000";

pub mod app_state;
mod clients;

pub use clients::plugins::*;
