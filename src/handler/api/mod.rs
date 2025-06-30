mod auth;
mod dashboard;
mod history;
mod oauth;
mod permission;
mod user;
mod user_type;

use crate::AppState;
use axum::Router;
use std::sync::Arc;

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/auth", auth::route())
        .nest("/dashboard", dashboard::route())
        .nest("/history", history::route())
        .nest("/oauth", oauth::route())
        .nest("/permission", permission::route())
        .nest("/user", user::route())
        .nest("/user-type", user_type::route())
}
