use crate::{
    errors::AppError,
    model::dto::{common::ListQueryParams, user::CreateUserRequest},
    AppState,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_user).post(post_user))
        .route("/{id}", get(get_user_by_id))
}

async fn post_user(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.service.user_service.create_user(req).await?;
    Ok((StatusCode::CREATED, Json(response)).into_response())
}

async fn get_user(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.service.user_service.get_user_array(query).await?;
    Ok((StatusCode::OK, Json(response)).into_response())
}

async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.service.user_service.get_user_by_id(id).await?;
    Ok((StatusCode::OK, Json(response)).into_response())
}
