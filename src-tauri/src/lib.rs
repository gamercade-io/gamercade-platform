use futures::TryFutureExt;
use gamercade_interface::{
    auth::auth_service_client::AuthServiceClient,
    author::{author_service_client::AuthorServiceClient, author_service_server::AuthorService},
    game::game_service_client::GameServiceClient,
    platform::platform_service_client::PlatformServiceClient,
    tag::tag_service_client::TagServiceClient,
};
use tonic::transport::Channel;

pub const SERVICE_IP_GRPC: &str = "http://127.0.0.1:50051";
pub const SERVICE_IP_HTTP: &str = "http://127.0.0.1:3000";

pub mod app_state;
mod clients;

pub use clients::plugins::*;
