use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct OAuthAuthorizeRequest {
    pub response_type: String, // "code"
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OAuthAuthorizeResponse {
    pub code: String,
    pub state: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OAuthTokenRequest {
    pub grant_type: String, // "authorization_code", "client_credentials", "password", "refresh_token"
    pub code: Option<String>, // authorization_code
    pub redirect_uri: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub username: Option<String>, // password grant
    pub password: Option<String>, // password grant
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OAuthTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}
