use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OAuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub client_id: String,
    pub user_id: Option<i64>,
    pub scope: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
