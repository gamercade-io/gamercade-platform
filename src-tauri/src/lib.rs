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
pub mod auth;

pub(crate) async fn auth_client() -> Result<AuthServiceClient<Channel>, String> {
    AuthServiceClient::connect(SERVICE_IP_GRPC)
        .map_err(|e| e.to_string())
        .await
}

pub(crate) async fn author_client() -> Result<AuthorServiceClient<Channel>, String> {
    AuthorServiceClient::connect(SERVICE_IP_GRPC)
        .map_err(|e| e.to_string())
        .await
}

pub(crate) async fn game_client() -> Result<GameServiceClient<Channel>, String> {
    GameServiceClient::connect(SERVICE_IP_GRPC)
        .map_err(|e| e.to_string())
        .await
}

pub(crate) async fn platform_client() -> Result<PlatformServiceClient<Channel>, String> {
    PlatformServiceClient::connect(SERVICE_IP_GRPC)
        .map_err(|e| e.to_string())
        .await
}

pub(crate) async fn tag_client() -> Result<TagServiceClient<Channel>, String> {
    TagServiceClient::connect(SERVICE_IP_GRPC)
        .map_err(|e| e.to_string())
        .await
}
