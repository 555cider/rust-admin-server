use crate::{
    errors::AppError,
    filter::{auth, UserId},
    model::dto::dashboard::DashboardData,
    AppState,
};
use axum::{extract::State, middleware, response::Json, routing::get, Extension, Router};
use serde::Serialize;
use std::sync::Arc;

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(|state, auth| api_dashboard_data(state, auth)))
        .layer(middleware::from_fn(auth))
}

#[derive(Serialize)]
struct ApiDashboardResponse {
    dashboard_data: DashboardData,
}

async fn api_dashboard_data(
    State(state): State<Arc<AppState>>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<ApiDashboardResponse>, AppError> {
    // Extract the numeric ID from the UserId type
    let user_id_num = user_id.0;
    let dashboard_data = DashboardData::new(&state.pool, user_id_num).await?;
    Ok(Json(ApiDashboardResponse { dashboard_data }))
}
