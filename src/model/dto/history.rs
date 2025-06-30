use crate::model::entity::history::History;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const DEFAULT_PAGE_SIZE: i64 = 20;
const MAX_PAGE_SIZE: i64 = 100;

/// Response DTO for history entries
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryResponse {
    pub id: i64,
    pub user_id: Option<i64>,
    pub username: Option<String>,
    /// Action that was performed (e.g., "login", "create_user")
    pub action: String,
    pub entity_id: Option<i64>,
    pub entity_type: Option<String>,
    pub details: Option<Value>,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<History> for HistoryResponse {
    fn from(log: History) -> Self {
        Self {
            id: log.id,
            user_id: log.user_id,
            username: None, // Will be populated if needed
            action: log.action,
            entity_id: log.entity_id,
            entity_type: None, // Will be populated if needed
            details: log.details.and_then(|d| serde_json::from_str(&d).ok()),
            ip_address: log.ip_address,
            created_at: log.created_at,
        }
    }
}

/// Query parameters for filtering and paginating history
#[derive(Debug, Deserialize, Default, Clone)]
pub struct HistoryListQuery {
    pub user_id: Option<i64>,
    pub action: Option<String>,
    pub entity_id: Option<i64>,
    pub entity_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    /// Page number (1-based)
    #[serde(default = "default_page")]
    pub page: Option<i64>,
    #[serde(default = "default_page_size", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl HistoryListQuery {
    /// Gets the page number (1-based)
    pub fn get_page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }

    /// Gets the number of items per page
    pub fn get_limit(&self) -> i64 {
        self.limit
            .or(self.per_page)
            .unwrap_or_else(|| MAX_PAGE_SIZE)
            .clamp(1, i64::MAX)
    }

    /// Calculates the offset for pagination
    pub fn get_offset(&self) -> i64 {
        self.offset
            .unwrap_or_else(|| (self.get_page() - 1) * self.get_limit())
    }

    /// Converts the query into SQL conditions and parameters
    pub fn to_sql_conditions(&self) -> (String, sqlx::sqlite::SqliteArguments) {
        use sqlx::Arguments;

        let mut conditions = Vec::new();
        let mut args = sqlx::sqlite::SqliteArguments::default();
        let mut param_count = 0;

        if let Some(user_id) = self.user_id {
            param_count += 1;
            conditions.push(format!("user_id = ${}", param_count));
            let _ = args.add(user_id);
        }

        if let Some(action) = &self.action {
            param_count += 1;
            conditions.push(format!("action = ${}", param_count));
            let _ = args.add(action);
        }

        if let Some(entity_id) = self.entity_id {
            param_count += 1;
            conditions.push(format!("entity_id = ${}", param_count));
            let _ = args.add(entity_id);
        }

        if let Some(entity_type) = &self.entity_type {
            param_count += 1;
            conditions.push(format!("action LIKE ${}", param_count));
            let _ = args.add(format!("%:{}%", entity_type));
        }

        if let Some(start_date) = self.start_date {
            param_count += 1;
            conditions.push(format!("created_at >= ${}", param_count));
            let _ = args.add(start_date);
        }

        if let Some(end_date) = self.end_date {
            param_count += 1;
            conditions.push(format!("created_at <= ${}", param_count));
            let _ = args.add(end_date);
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        (where_clause, args)
    }
}

fn default_page() -> Option<i64> {
    Some(1)
}

fn default_page_size() -> Option<i64> {
    Some(DEFAULT_PAGE_SIZE)
}
