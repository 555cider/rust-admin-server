use crate::{
    config::auth::authn_user::AuthnUser,
    errors::AppError,
    model::dto::auth::{CurrentUserResponse, LoginRequest, LoginResponse, RegisterRequest},
    util::cookie_util,
    AppState,
};
use axum::{
    extract::{ConnectInfo, Form, Request, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_extra::{headers::UserAgent, TypedHeader};
use http::{header, header::HeaderValue, HeaderMap, StatusCode};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

pub fn route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(post_auth_login))
        .route("/refresh", post(post_auth_refresh))
        .route("/me", get(get_auth_me))
        .route("/register", post(post_auth_register))
        .route("/logout", post(post_auth_logout))
}

async fn post_auth_login(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Form(mut req): Form<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    info!("Login request for user: {}", req.username);

    if req.redirect_url.is_none() {
        req.redirect_url = Some("/dashboard".to_string());
    }

    let ip_address = Some(addr.ip().to_string());
    let user_agent = Some(user_agent.to_string());

    let (access_token, refresh_token) = state
        .service
        .auth_service
        .login(&state.config, req, ip_address, user_agent)
        .await?;

    let access_cookie = cookie_util::create_access_token_cookie(&state.config, &access_token);
    let refresh_cookie = cookie_util::create_refresh_token_cookie(&state.config, &refresh_token);

    let response = LoginResponse {
        access_token: access_token.clone(),
        token_type: "Bearer".to_string(),
        expires_in: state.config.token.access_exp,
        redirect_url: Some("/dashboard".to_string()),
    };

    // Set-Cookie 헤더를 명확하게 여러 개 추가
    let res = Response::builder()
        .status(StatusCode::OK)
        .header(
            header::SET_COOKIE,
            HeaderValue::from_str(&access_cookie.to_string()).unwrap(),
        )
        .header(
            header::SET_COOKIE,
            HeaderValue::from_str(&refresh_cookie.to_string()).unwrap(),
        )
        .header("Cache-Control", "no-store")
        .header("Pragma", "no-cache")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&response).unwrap())
        .unwrap();

    Ok(res)
}

async fn post_auth_refresh(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    request: Request,
) -> Result<impl IntoResponse, AppError> {
    let headers = request.headers();
    let refresh_token = cookie_util::get_refresh_token(Some(&state.config), headers)
        .ok_or_else(|| AppError::Unauthorized("No refresh token provided".to_string()))?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = Some(user_agent.to_string());

    let (access_token, new_refresh_token) = state
        .service
        .auth_service
        .refresh_access_token(
            &state.config,
            refresh_token.to_string(),
            ip_address,
            user_agent,
        )
        .await?;

    let response = LoginResponse {
        access_token: access_token.clone(),
        token_type: "Bearer".to_string(),
        expires_in: state.config.token.access_exp,
        redirect_url: None,
    };

    let access_cookie = cookie_util::create_access_token_cookie(&state.config, &access_token);
    let refresh_cookie =
        cookie_util::create_refresh_token_cookie(&state.config, &new_refresh_token);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        access_cookie.to_string().parse().unwrap(),
    );
    headers.append(
        header::SET_COOKIE,
        refresh_cookie.to_string().parse().unwrap(),
    );

    Ok((StatusCode::OK, headers, Json(response)))
}

async fn get_auth_me(
    State(state): State<Arc<AppState>>,
    current_user: AuthnUser,
) -> Result<Json<CurrentUserResponse>, AppError> {
    let response = state
        .service
        .auth_service
        .get_current_user(current_user)
        .await?;
    Ok(Json(response))
}

async fn post_auth_register(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Form(req): Form<RegisterRequest>,
) -> Result<(StatusCode, Json<i64>), AppError> {
    info!("Register request for username: {}", req.username);

    let ip_address = Some(addr.ip().to_string());
    let user_agent = Some(user_agent.to_string());

    let user_id = state
        .service
        .auth_service
        .register(req, ip_address, user_agent)
        .await?;

    Ok((StatusCode::CREATED, Json(user_id)))
}

async fn post_auth_logout(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, AppError> {
    // 토큰 쿠키 삭제
    let access_cookie = cookie_util::expire_access_token_cookie(&state.config);
    let refresh_cookie = cookie_util::expire_refresh_token_cookie(&state.config);

    // 쿠키 헤더 추가
    let mut headers = HeaderMap::new();
    if let Ok(access_cookie_str) = access_cookie.to_string().parse::<HeaderValue>() {
        headers.append(header::SET_COOKIE, access_cookie_str);
    }
    if let Ok(refresh_cookie_str) = refresh_cookie.to_string().parse::<HeaderValue>() {
        headers.append(header::SET_COOKIE, refresh_cookie_str);
    }

    // 클라이언트 측에서도 토큰을 제거할 수 있도록 리다이렉트 응답 반환
    Ok((
        headers,
        Json(serde_json::json!({
            "success": true,
            "redirect": "/"
        })),
    ))
}
