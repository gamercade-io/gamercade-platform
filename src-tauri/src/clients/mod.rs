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
