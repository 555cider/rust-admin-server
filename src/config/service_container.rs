use crate::{
    repository::{
        history::HistoryRepository, oauth::OAuthRepository, AuthRepository, PermissionRepository,
        UserRepository, UserTypeRepository,
    },
    service::{
        auth::AuthService, history::HistoryService, oauth::OAuthService,
        permission::PermissionService, user::UserService, user_type::UserTypeService,
    },
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ServiceContainer {
    pub auth_service: Arc<AuthService>,
    pub history_service: Arc<HistoryService>,
    pub oauth_service: Arc<OAuthService>,
    pub permission_service: Arc<PermissionService>,
    pub user_service: Arc<UserService>,
    pub user_type_service: Arc<UserTypeService>,
}

impl ServiceContainer {
    pub fn new(db: Arc<sqlx::SqlitePool>) -> Self {
        let auth_repo = AuthRepository::new(db.clone());
        let history_repo = HistoryRepository::new(db.clone());
        let oauth_repo = OAuthRepository::new(db.clone());
        let permission_repo = PermissionRepository::new(db.clone());
        let user_repo = UserRepository::new(db.clone());
        let user_type_repo = UserTypeRepository::new(db.clone());

        let history = Arc::new(HistoryService::new(history_repo));
        let auth = Arc::new(AuthService::new(
            auth_repo,
            user_repo.clone(),
            user_type_repo.clone(),
            history.clone(),
        ));
        let oauth = Arc::new(OAuthService::new(oauth_repo));
        let permission = Arc::new(PermissionService::new(permission_repo.clone()));
        let user = Arc::new(UserService::new(user_repo.clone()));
        let user_type = Arc::new(UserTypeService::new(user_type_repo.clone()));

        Self {
            auth_service: auth,
            history_service: history,
            oauth_service: oauth,
            permission_service: permission,
            user_service: user,
            user_type_service: user_type,
        }
    }
}
