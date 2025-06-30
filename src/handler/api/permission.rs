use crate::{
    errors::AppError,
    filter::auth,
    model::{
        dto::common::ListQueryParams, dto::permission::CreatePermissionRequest,
        dto::permission::UpdatePermissionRequest,
    },
    AppState,
};
use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_permission).post(post_permission))
        .route("/{id}", get(get_permission_by_id).put(update_permission))
        .layer(middleware::from_fn(auth))
}

async fn post_permission(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreatePermissionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .permission_service
        .create_permission(req)
        .await?;
    Ok((StatusCode::CREATED, Json(response)).into_response())
}

async fn get_permission(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .permission_service
        .get_permissions(query)
        .await?;
    Ok((StatusCode::OK, Json(response)).into_response())
}

async fn get_permission_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .permission_service
        .get_permission_by_id(id)
        .await?;
    Ok((StatusCode::OK, Json(response)).into_response())
}

async fn update_permission(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(req): Json<UpdatePermissionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .permission_service
        .update_permission(id, req)
        .await?;
    Ok((StatusCode::OK, Json(response)).into_response())
}
