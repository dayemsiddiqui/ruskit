use axum::{
    response::IntoResponse,
    extract::State,
    Json,
    response::Redirect,
};
use axum_login::AuthSession as AxumAuthSession;
use sea_orm::{ActiveModelTrait, Set, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use validator::Validate;
use axum_inertia::Inertia;
use serde_json::json;

use crate::app::{
    services::auth_service::{Backend, Credentials},
    entities::user,
    dtos::user::{CreateUserRequest},
};

pub struct AuthController;

impl AuthController {
    pub async fn login(
        mut auth: AxumAuthSession<Backend>,
        inertia: Inertia,
        Json(creds): Json<Credentials>,
    ) -> impl IntoResponse {
        let user = auth.authenticate(creds).await.unwrap();
        match user {
            Some(ref user) => {
                auth.login(user).await.unwrap();
                Redirect::to("/dashboard").into_response()
            }
            None => {
                inertia.render("Auth/Login", json!({
                    "error": "Invalid credentials"
                })).into_response()
            }
        }
    }

    pub async fn register(
        State(db): State<DatabaseConnection>,
        inertia: Inertia,
        Json(payload): Json<CreateUserRequest>,
    ) -> impl IntoResponse {
        // Validate the request
        if let Err(errors) = payload.validate() {
            return inertia.render("Auth/Register", json!({
                "errors": errors.to_string()
            })).into_response();
        }

        // Check if email already exists
        let existing_user = user::Entity::find()
            .filter(user::Column::Email.eq(&payload.email))
            .one(&db)
            .await
            .unwrap();

        if existing_user.is_some() {
            return inertia.render("Auth/Register", json!({
                "errors": {
                    "email": ["Email already exists"]
                }
            })).into_response();
        }

        // Hash the password
        let hashed_password = match Backend::hash_password(&payload.password).await {
            Ok(hash) => hash,
            Err(_) => {
                return inertia.render("Auth/Register", json!({
                    "errors": "Failed to hash password"
                })).into_response();
            }
        };

        // Create the user
        let user = user::ActiveModel {
            name: Set(payload.name),
            email: Set(payload.email),
            password: Set(hashed_password),
            role: Set("user".to_string()),
            ..Default::default()
        };

        match user.insert(&db).await {
            Ok(_) => {
                Redirect::to("/login").into_response()
            }
            Err(_) => {
                inertia.render("Auth/Register", json!({
                    "errors": "Failed to create user"
                })).into_response()
            }
        }
    }

    pub async fn logout(mut auth: AxumAuthSession<Backend>, _inertia: Inertia) -> impl IntoResponse {
        let _ = auth.logout().await;
        Redirect::to("/login")
    }

    pub async fn me(auth: AxumAuthSession<Backend>) -> impl IntoResponse {
        Json(auth.user)
    }
} 