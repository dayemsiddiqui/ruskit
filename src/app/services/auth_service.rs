use axum_login::{AuthUser, AuthnBackend};
use bcrypt::{hash, verify, DEFAULT_COST};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use crate::app::entities::{user, user::Entity as User};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthSession {
    pub user_id: i32,
    pub email: String,
    pub role: String,
}

impl AuthUser for AuthSession {
    type Id = i32;

    fn id(&self) -> Self::Id {
        self.user_id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.email.as_bytes()
    }
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: DatabaseConnection,
}

impl Backend {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password.as_bytes(), DEFAULT_COST)
    }
}

#[async_trait::async_trait]
impl AuthnBackend for Backend {
    type User = AuthSession;
    type Credentials = Credentials;
    type Error = sea_orm::DbErr;

    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        let user = User::find()
            .filter(user::Column::Email.eq(creds.email))
            .one(&self.db)
            .await?;

        if let Some(user) = user {
            if verify(creds.password.as_bytes(), &user.password).unwrap_or(false) {
                return Ok(Some(AuthSession {
                    user_id: user.id,
                    email: user.email,
                    role: user.role,
                }));
            }
        }

        Ok(None)
    }

    async fn get_user(&self, user_id: &i32) -> Result<Option<Self::User>, Self::Error> {
        let user = User::find_by_id(*user_id)
            .one(&self.db)
            .await?;

        Ok(user.map(|u| AuthSession {
            user_id: u.id,
            email: u.email,
            role: u.role,
        }))
    }
} 