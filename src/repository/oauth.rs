use crate::model::entity::{
    oauth_client::OAuthClient, oauth_code::OAuthCode, oauth_token::OAuthToken,
};
use chrono::TimeZone;
use sqlx::SqlitePool;
use std::sync::Arc;

// OAuth 관련 DB 접근 (클라이언트, 코드, 토큰)
#[derive(Clone)]
pub struct OAuthRepository {
    pool: Arc<SqlitePool>,
}

impl OAuthRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn find_client(&self, client_id: &str) -> Option<OAuthClient> {
        sqlx::query_as::<_, OAuthClient>(
            r#"SELECT id, client_id, client_secret, redirect_uri, scope, grant_types FROM oauth_client WHERE client_id = ?"#,
        )
        .bind(client_id)
        .fetch_optional(self.pool.as_ref())
        .await
        .ok()?
    }
    pub async fn save_code(&self, code: &OAuthCode) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO oauth_code (code, client_id, user_id, redirect_uri, scope, expires_at) VALUES (?, ?, ?, ?, ?, ?)"#
        )
        .bind(&code.code)
        .bind(&code.client_id)
        .bind(code.user_id)
        .bind(&code.redirect_uri)
        .bind(&code.scope)
        .bind(code.expires_at)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
    pub async fn find_code(&self, code: &str) -> Option<OAuthCode> {
        #[derive(sqlx::FromRow)]
        struct OAuthCodeRow {
            code: String,
            client_id: String,
            user_id: Option<i64>,
            redirect_uri: String,
            scope: Option<String>,
            expires_at: chrono::NaiveDateTime,
        }
        let row: Option<OAuthCodeRow> = sqlx::query_as(
            r#"
            SELECT 
                code, 
                client_id, 
                user_id, 
                redirect_uri, 
                scope, 
                expires_at 
            FROM oauth_code 
            WHERE code = ? AND expires_at > CURRENT_TIMESTAMP"#,
        )
        .bind(code)
        .fetch_optional(self.pool.as_ref())
        .await
        .ok()?;
        row.map(|r| OAuthCode {
            code: r.code,
            client_id: r.client_id,
            user_id: r.user_id,
            redirect_uri: r.redirect_uri,
            scope: r.scope,
            expires_at: chrono::Utc.from_utc_datetime(&r.expires_at),
        })
    }
    pub async fn save_token(&self, token: &OAuthToken) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO oauth_token (access_token, refresh_token, client_id, user_id, scope, expires_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&token.access_token)
        .bind(&token.refresh_token)
        .bind(&token.client_id)
        .bind(token.user_id)
        .bind(&token.scope)
        .bind(token.expires_at)
        .bind(token.created_at)
        .execute(self.pool.as_ref())
        .await?;
        Ok(())
    }
    pub async fn find_token(&self, access_token: &str) -> Option<OAuthToken> {
        #[derive(sqlx::FromRow)]
        struct OAuthTokenRow {
            access_token: String,
            refresh_token: Option<String>,
            client_id: String,
            user_id: Option<i64>,
            scope: Option<String>,
            expires_at: chrono::NaiveDateTime,
            created_at: chrono::NaiveDateTime,
        }

        let row: Option<OAuthTokenRow> = sqlx::query_as(
            r#"
            SELECT 
                access_token, 
                refresh_token, 
                client_id, 
                user_id, 
                scope, 
                expires_at, 
                created_at 
            FROM oauth_token 
            WHERE access_token = ? AND expires_at > CURRENT_TIMESTAMP"#,
        )
        .bind(access_token)
        .fetch_optional(self.pool.as_ref())
        .await
        .ok()?;

        row.map(|r| OAuthToken {
            access_token: r.access_token,
            refresh_token: r.refresh_token,
            client_id: r.client_id,
            user_id: r.user_id,
            scope: r.scope,
            expires_at: chrono::Utc.from_utc_datetime(&r.expires_at),
            created_at: chrono::Utc.from_utc_datetime(&r.created_at),
        })
    }
    pub async fn find_refresh_token(&self, refresh_token: &str) -> Option<OAuthToken> {
        sqlx::query_as::<_, OAuthToken>(
            r#"SELECT access_token, refresh_token, client_id, user_id, scope, expires_at, created_at FROM oauth_token WHERE refresh_token = ? AND expires_at > CURRENT_TIMESTAMP"#,
        )
        .bind(refresh_token)
        .fetch_optional(self.pool.as_ref())
        .await
        .ok()?
    }
}
