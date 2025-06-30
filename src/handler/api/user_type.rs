use crate::{
    errors::AppError,
    model::dto::{
        common::ListQueryParams,
        user_type::{CreateUserTypeRequest, UpdateUserTypeRequest},
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

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_user_type).post(post_user_type))
        .route(
            "/{id}",
            get(get_user_type_by_id)
                .put(put_user_type)
                .delete(delete_user_type),
        )
}

async fn post_user_type(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateUserTypeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .user_type_service
        .create_user_type(req)
        .await?;
    Ok((StatusCode::CREATED, Json(response)).into_response())
}

async fn get_user_type(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListQueryParams>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .user_type_service
        .get_user_type_array(query)
        .await?;
    Ok((StatusCode::OK, Json(response)).into_response())
}

async fn get_user_type_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .user_type_service
        .get_user_type_by_id(id)
        .await?;
    Ok((StatusCode::OK, Json(response)).into_response())
}

async fn put_user_type(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateUserTypeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .service
        .user_type_service
        .update_user_type(id, req)
        .await?;
    Ok((StatusCode::OK, Json(response)).into_response())
}

async fn delete_user_type(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    state.service.user_type_service.delete_user_type(id).await?;
    Ok(StatusCode::NO_CONTENT.into_response())
}
