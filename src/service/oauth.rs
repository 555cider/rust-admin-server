use crate::{
    model::{
        dto::oauth::OAuthAuthorizeRequest, dto::oauth::OAuthAuthorizeResponse,
        dto::oauth::OAuthTokenRequest, dto::oauth::OAuthTokenResponse,
        entity::oauth_code::OAuthCode, entity::oauth_token::OAuthToken,
    },
    repository::oauth::OAuthRepository,
};

// OAuth 관련 서비스 (authorize, token 등)

pub struct OAuthService {
    oauth_repo: OAuthRepository,
}

impl OAuthService {
    pub fn new(oauth_repo: OAuthRepository) -> Self {
        Self { oauth_repo }
    }

    pub async fn authorize(
        &self,
        req: OAuthAuthorizeRequest,
    ) -> Result<OAuthAuthorizeResponse, anyhow::Error> {
        // 1. client_id, redirect_uri 등 검증
        let client = self
            .oauth_repo
            .find_client(&req.client_id)
            .await
            .ok_or_else(|| anyhow::anyhow!("invalid_client_id"))?;
        if client.redirect_uri != req.redirect_uri {
            return Err(anyhow::anyhow!("invalid_redirect_uri"));
        }
        // 2. 권한 부여 코드 생성 및 저장
        use chrono::{Duration, Utc};
        use rand::Rng;
        let code = format!("{:x}", rand::thread_rng().gen::<u128>());
        let expires_at = Utc::now() + Duration::minutes(10);
        let oauth_code = OAuthCode {
            code: code.clone(),
            client_id: req.client_id.clone(),
            user_id: None, // 실제 서비스라면 인증된 user_id 필요
            redirect_uri: req.redirect_uri.clone(),
            scope: req.scope.clone(),
            expires_at,
        };
        self.oauth_repo.save_code(&oauth_code).await?;
        // 3. 응답 반환
        Ok(OAuthAuthorizeResponse {
            code,
            state: req.state,
        })
    }

    pub async fn token(&self, req: OAuthTokenRequest) -> Result<OAuthTokenResponse, anyhow::Error> {
        match req.grant_type.as_str() {
            "authorization_code" => {
                // 1. 코드 검증 및 토큰 발급
                let code = req
                    .code
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("missing_code"))?;
                let oauth_code = self
                    .oauth_repo
                    .find_code(code)
                    .await
                    .ok_or_else(|| anyhow::anyhow!("invalid_code"))?;
                if let Some(ref redirect_uri) = req.redirect_uri {
                    if oauth_code.redirect_uri != *redirect_uri {
                        return Err(anyhow::anyhow!("invalid_redirect_uri"));
                    }
                }
                // 토큰 생성
                use chrono::{Duration, Utc};
                use rand::Rng;
                let access_token = format!("at_{:x}", rand::thread_rng().gen::<u128>());
                let refresh_token = format!("rt_{:x}", rand::thread_rng().gen::<u128>());
                let expires_at = Utc::now() + Duration::hours(1);
                let token = OAuthToken {
                    access_token: access_token.clone(),
                    refresh_token: Some(refresh_token.clone()),
                    client_id: oauth_code.client_id.clone(),
                    user_id: oauth_code.user_id,
                    scope: oauth_code.scope.clone(),
                    expires_at,
                    created_at: Utc::now(),
                };
                self.oauth_repo.save_token(&token).await?;
                Ok(OAuthTokenResponse {
                    access_token,
                    token_type: "bearer".to_string(),
                    expires_in: 3600,
                    refresh_token: Some(refresh_token),
                    scope: oauth_code.scope,
                })
            }
            "client_credentials" => {
                // 1. 클라이언트 인증 및 토큰 발급
                let client_id = req
                    .client_id
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("missing_client_id"))?;
                let client_secret = req
                    .client_secret
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("missing_client_secret"))?;
                let client = self
                    .oauth_repo
                    .find_client(client_id)
                    .await
                    .ok_or_else(|| anyhow::anyhow!("invalid_client_id"))?;
                if client.client_secret != *client_secret {
                    return Err(anyhow::anyhow!("invalid_client_secret"));
                }
                use chrono::{Duration, Utc};
                use rand::Rng;
                let access_token = format!("at_{:x}", rand::thread_rng().gen::<u128>());
                let expires_at = Utc::now() + Duration::hours(1);
                let token = OAuthToken {
                    access_token: access_token.clone(),
                    refresh_token: None,
                    client_id: client_id.clone(),
                    user_id: None,
                    scope: req.scope.clone().or(client.scope.clone()),
                    expires_at,
                    created_at: Utc::now(),
                };
                self.oauth_repo.save_token(&token).await?;
                Ok(OAuthTokenResponse {
                    access_token,
                    token_type: "bearer".to_string(),
                    expires_in: 3600,
                    refresh_token: None,
                    scope: token.scope,
                })
            }
            "refresh_token" => {
                // 1. 리프레시 토큰 검증 및 액세스 토큰 재발급
                let refresh_token = req
                    .refresh_token
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("missing_refresh_token"))?;
                let old_token = self
                    .oauth_repo
                    .find_refresh_token(refresh_token)
                    .await
                    .ok_or_else(|| anyhow::anyhow!("invalid_refresh_token"))?;
                use chrono::{Duration, Utc};
                use rand::Rng;
                let access_token = format!("at_{:x}", rand::thread_rng().gen::<u128>());
                let expires_at = Utc::now() + Duration::hours(1);
                let token = OAuthToken {
                    access_token: access_token.clone(),
                    refresh_token: Some(refresh_token.clone()),
                    client_id: old_token.client_id.clone(),
                    user_id: old_token.user_id,
                    scope: old_token.scope.clone(),
                    expires_at,
                    created_at: Utc::now(),
                };
                self.oauth_repo.save_token(&token).await?;
                Ok(OAuthTokenResponse {
                    access_token,
                    token_type: "bearer".to_string(),
                    expires_in: 3600,
                    refresh_token: Some(refresh_token.clone()),
                    scope: old_token.scope,
                })
            }
            _ => return Err(anyhow::anyhow!("unsupported grant_type")),
        }
    }
}
