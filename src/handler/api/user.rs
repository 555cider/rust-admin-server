use crate::{
    errors::AppError,
    model::dto::{
        common::ListQueryParams,
        user::{CreateUserRequest, UpdateUserRequest},
    },
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
use validator::Validate;

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_user).post(post_user))
        .route(
            "/{id}",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
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

async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    req.validate()?;

    state.service.user_service.update_user(id, req).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    state.service.user_service.delete_user(id).await?;

    Ok(StatusCode::NO_CONTENT)
}
