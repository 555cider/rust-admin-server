use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// History entry representing user actions in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub id: i64,
    pub user_id: Option<i64>,
    /// Action name (e.g., "login", "create_user", "update_permission")
    pub action: String,
    pub entity_id: Option<i64>,
    pub details: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl History {
    /// Creates a new history entry
    pub fn new(
        user_id: Option<i64>,
        action: impl Into<String>,
        entity_id: Option<i64>,
        details: Option<Value>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        Self {
            id: 0, // Will be set by the database
            user_id,
            action: action.into(),
            entity_id,
            details: details.map(|v| v.to_string()),
            ip_address,
            user_agent,
            created_at: Utc::now(),
        }
    }

    /// Creates a system-generated history entry
    pub fn system(
        action: impl Into<String>,
        entity_id: Option<i64>,
        details: Option<Value>,
    ) -> Self {
        Self::new(None, action, entity_id, details, None, None)
    }
}
