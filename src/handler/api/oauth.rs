use crate::{
    model::dto::oauth::{OAuthAuthorizeRequest, OAuthTokenRequest},
    AppState,
};
use axum::{
    extract::{Form, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/authorize", get(authorize))
        .route("/token", post(token))
}

pub async fn authorize(
    State(state): State<Arc<AppState>>,
    Form(req): Form<OAuthAuthorizeRequest>,
) -> impl IntoResponse {
    match state.service.oauth_service.authorize(req).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => (axum::http::StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn token(
    State(state): State<Arc<AppState>>,
    Form(req): Form<OAuthTokenRequest>,
) -> impl IntoResponse {
    match state.service.oauth_service.token(req).await {
        Ok(resp) => Json(resp).into_response(),
        Err(e) => (axum::http::StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}
