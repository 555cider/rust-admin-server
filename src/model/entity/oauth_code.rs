use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OAuthCode {
    pub code: String,
    pub client_id: String,
    pub user_id: Option<i64>,
    pub redirect_uri: String,
    pub scope: Option<String>,
    pub expires_at: DateTime<Utc>,
}
