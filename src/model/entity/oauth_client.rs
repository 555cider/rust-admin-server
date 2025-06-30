use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OAuthClient {
    pub id: i64,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub grant_types: Option<String>, // comma-separated
}
