use crate::{
    errors::AppError,
    model::{dto::common::ListQueryParams, dto::user::UserResponse, entity::admin_user::AdminUser},
};
use sqlx::{Arguments, SqlitePool};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepository {
    pool: Arc<SqlitePool>,
}

impl UserRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        username: String,
        email: Option<String>,
        password_hash: String,
        user_type_id: i64,
        is_active: bool,
    ) -> Result<i64, AppError> {
        let result = sqlx::query!(
            "INSERT INTO admin_user (username, email, password_hash, user_type_id, is_active) VALUES (?, ?, ?, ?, ?) RETURNING id",
            username,
            email,
            password_hash,
            user_type_id,
            is_active
        )
        .fetch_one(&*self.pool)
        .await?;

        result
            .id
            .ok_or_else(|| AppError::Conflict(String::from("Failed to create user")))
    }

    pub async fn find_all(
        &self,
        query_params: &ListQueryParams,
    ) -> Result<Vec<UserResponse>, AppError> {
        let limit = query_params.get_limit();
        let offset = query_params.get_offset();
        let allowed_sort_columns = [
            "id",
            "username",
            "user_type_id",
            "is_active",
            "last_login_at",
            "created_at",
            "updated_at",
        ];
        let order_by = query_params.get_order_by(&allowed_sort_columns);

        let base_query = "SELECT * FROM admin_user";
        let mut conditions = Vec::new();
        let mut args = sqlx::sqlite::SqliteArguments::default();

        if let Some(search_term) = &query_params.q {
            conditions.push("username LIKE ?");
            let _ = args.add(format!("%{}%", search_term));
        }

        if let Some(status) = &query_params.status {
            match status.as_str() {
                "active" => conditions.push("is_active = 1"),
                "inactive" => conditions.push("is_active = 0"),
                "suspended" => conditions.push("is_active = 0"), // Assuming suspended is the same as inactive for now
                _ => {}
            };
        }

        let where_clause = if conditions.is_empty() {
            "".to_string()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let query_str = format!(
            "{} {} ORDER BY {} LIMIT ? OFFSET ?",
            base_query, where_clause, order_by
        );
        let _ = args.add(limit);
        let _ = args.add(offset);

        let users = sqlx::query_as_with::<_, AdminUser, _>(&query_str, args)
            .fetch_all(&*self.pool)
            .await?;

        Ok(users.into_iter().map(UserResponse::from).collect())
    }

    pub async fn find_by_id(&self, id: i64) -> Result<UserResponse, AppError> {
        let user = sqlx::query_as!(
            AdminUser,
            r#"
            SELECT id, username, password_hash, email, user_type_id, is_active, 
                   last_login_at, created_at, updated_at
            FROM admin_user 
            WHERE id = ?"#,
            id
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|_| AppError::NotFound("User not found".to_string()))?;

        Ok(UserResponse::from(user))
    }

    pub async fn update_last_login(&self, user_id: i64) -> Result<(), AppError> {
        sqlx::query!(
            "UPDATE admin_user SET last_login_at = datetime('now') WHERE id = ?",
            user_id
        )
        .execute(&*self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_user(
        &self,
        id: i64,
        username: Option<String>,
        email: Option<String>,
        password_hash: Option<String>,
        user_type_id: Option<i64>,
        is_active: Option<bool>,
    ) -> Result<(), AppError> {
        let mut updates = Vec::new();
        let mut args = sqlx::sqlite::SqliteArguments::default();

        if let Some(username) = username {
            updates.push("username = ?");
            args.add(username).expect("Failed to add username");
        }

        if let Some(email) = email {
            updates.push("email = ?");
            args.add(email).expect("Failed to add email");
        }

        if let Some(password_hash) = password_hash {
            updates.push("password_hash = ?");
            args.add(password_hash)
                .expect("Failed to add password_hash");
        }

        if let Some(user_type_id) = user_type_id {
            updates.push("user_type_id = ?");
            args.add(user_type_id).expect("Failed to add user type id");
        }

        if let Some(is_active) = is_active {
            updates.push("is_active = ?");
            args.add(is_active).expect("Failed to add is_active");
        }

        if updates.is_empty() {
            return Ok(());
        }

        let query_str = format!("UPDATE admin_user SET {} WHERE id = ?", updates.join(", "));
        args.add(id).expect("Failed to add id");

        let result = sqlx::query_with(&query_str, args)
            .execute(&*self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    pub async fn delete_user(&self, id: i64) -> Result<(), AppError> {
        let result = sqlx::query!("DELETE FROM admin_user WHERE id = ?", id)
            .execute(&*self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("User not found".to_string()));
        }

        Ok(())
    }

    pub async fn exists_by_username(&self, username: &str) -> Result<bool, AppError> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM admin_user WHERE username = ?")
            .bind(username)
            .fetch_one(&*self.pool)
            .await?;

        Ok(count > 0)
    }

    pub async fn count_users(&self) -> Result<i64, AppError> {
        let result = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as count FROM admin_user
            "#
        )
        .fetch_one(&*self.pool)
        .await?;

        Ok(result)
    }

    pub async fn count_active_users(&self) -> Result<i64, AppError> {
        let count = sqlx::query_scalar!("SELECT COUNT(*) FROM admin_user WHERE is_active = true")
            .fetch_one(&*self.pool)
            .await?;
        Ok(count)
    }

    pub async fn exists_by_email(&self, email: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM admin_user WHERE email = ?)",
            email
        )
        .fetch_one(&*self.pool)
        .await?
            > 0;

        Ok(exists)
    }

    pub async fn is_email_in_use(
        &self,
        email: &str,
        exclude_user_id: Option<i64>,
    ) -> Result<bool, AppError> {
        let exists = match exclude_user_id {
            Some(id) => {
                sqlx::query_scalar!(
                    "SELECT EXISTS(SELECT 1 FROM admin_user WHERE email = ? AND id != ?)",
                    email,
                    id
                )
                .fetch_one(&*self.pool)
                .await?
                    == 1
            }
            None => self.exists_by_email(email).await?,
        };

        Ok(exists)
    }
}
