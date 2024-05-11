use futures::TryFutureExt;
use gamercade_interface::{
    auth::auth_service_client::AuthServiceClient,
    author::author_service_client::AuthorServiceClient,
    game::game_service_client::GameServiceClient,
    platform::platform_service_client::PlatformServiceClient,
    review::review_service_client::ReviewServiceClient, tag::tag_service_client::TagServiceClient,
    users::user_service_client::UserServiceClient,
};
use tonic::transport::Channel;

use crate::SERVICE_IP_GRPC;

mod auth;
mod author;
mod game;
mod platform;
mod review;
mod tag;
mod user;

pub mod plugins {
    pub use super::auth::auth_plugin;
    pub use super::author::author_plugin;
    pub use super::game::game_plugin;
    pub use super::platform::platform_plugin;
    pub use super::review::review_plugin;
    pub use super::tag::tag_plugin;
    pub use super::user::user_plugin;
}

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

pub(crate) async fn review_client() -> Result<ReviewServiceClient<Channel>, String> {
    ReviewServiceClient::connect(SERVICE_IP_GRPC)
        .map_err(|e| e.to_string())
        .await
}

pub(crate) async fn tag_client() -> Result<TagServiceClient<Channel>, String> {
    TagServiceClient::connect(SERVICE_IP_GRPC)
        .map_err(|e| e.to_string())
        .await
}

pub(crate) async fn user_client() -> Result<UserServiceClient<Channel>, String> {
    UserServiceClient::connect(SERVICE_IP_GRPC)
        .map_err(|e| e.to_string())
        .await
}
