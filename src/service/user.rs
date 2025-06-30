use crate::model::dto::user::UpdateUserRequest;
use crate::{
    errors::AppError,
    model::dto::{common::ListQueryParams, user::CreateUserRequest, user::UserResponse},
    repository::user::UserRepository,
    util::password_util,
};
use validator::Validate;

pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn create_user(&self, req: CreateUserRequest) -> Result<i64, AppError> {
        req.validate()?;
        let password_hash = password_util::hash_password(&req.password).await?;
        let is_active = req._is_active.unwrap_or(true);

        // Check if email is already in use using the repository method
        if self.user_repo.exists_by_email(&req.email).await? {
            return Err(AppError::Conflict("Email is already in use".to_string()));
        }

        let user_id = self
            .user_repo
            .create(
                req.username,
                Some(req.email),
                password_hash,
                req.user_type_id,
                is_active,
            )
            .await?;

        Ok(user_id)
    }

    pub async fn get_user_array(
        &self,
        query_params: ListQueryParams,
    ) -> Result<Vec<UserResponse>, AppError> {
        self.user_repo.find_all(&query_params).await
    }

    pub async fn get_user_by_id(&self, id: i64) -> Result<UserResponse, AppError> {
        self.user_repo.find_by_id(id).await
    }

    pub async fn count_users(&self) -> Result<i64, AppError> {
        self.user_repo.count_users().await
    }

    pub async fn count_active_users(&self) -> Result<i64, AppError> {
        self.user_repo.count_active_users().await
    }

    pub async fn update_user(&self, id: i64, req: UpdateUserRequest) -> Result<(), AppError> {
        // Check if email is being updated and if it's already in use
        if let Some(email) = &req.email {
            if self.user_repo.is_email_in_use(email, Some(id)).await? {
                return Err(AppError::Conflict("Email is already in use".to_string()));
            }
        }

        self.user_repo
            .update_user(
                id,
                req.username,
                req.email,
                None,
                req.user_type_id,
                req._is_active,
            )
            .await?;

        Ok(())
    }

    pub async fn delete_user(&self, id: i64) -> Result<(), AppError> {
        self.user_repo.delete_user(id).await?;
        Ok(())
    }
}
