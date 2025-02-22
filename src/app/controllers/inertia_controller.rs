use axum::extract::State;
use axum::response::IntoResponse;
use axum_inertia::Inertia;
use sea_orm::{EntityTrait, QueryOrder};
use serde_json::json;
use axum_login::AuthSession;

use crate::{web::AppState, app::entities::post::Entity as Post};
use crate::app::dtos::post::{PostDto, PostListProps};
use crate::app::services::auth_service::Backend;

pub struct InertiaController;

impl InertiaController {
    pub async fn dashboard(
        auth: AuthSession<Backend>,
        inertia: Inertia,
    ) -> impl IntoResponse {
        if let Some(user) = auth.user {
            inertia.render("Dashboard", json!({
                "user": {
                    "name": user.email.split('@').next().unwrap_or("User"),
                    "email": user.email,
                    "role": user.role,
                }
            }))
        } else {
            inertia.render("Auth/Login", json!({}))
        }
    }

    pub async fn about(State(_state): State<AppState>, inertia: Inertia) -> impl IntoResponse {
        inertia.render("About", json!({
            "title": "About Page",
            "content": "This is the about page content"
        }))
    }

    pub async fn posts_index(State(state): State<AppState>, inertia: Inertia) -> impl IntoResponse {
        let posts = Post::find()
            .order_by_desc(crate::app::entities::post::Column::CreatedAt)
            .all(&state.db)
            .await
            .unwrap_or_default();

        let post_dtos: Vec<PostDto> = posts.into_iter()
            .map(|post| PostDto {
                id: post.id,
                title: post.title,
                content: post.content,
                slug: post.slug,
                created_at: post.created_at,
                updated_at: post.updated_at,
            })
            .collect();

        inertia.render("Posts/Index", PostListProps { posts: post_dtos })
    }

    pub async fn posts_show(State(_state): State<AppState>, inertia: Inertia) -> impl IntoResponse {
        inertia.render("Posts/Show", json!({
            "post": {}  // TODO: Implement post fetching by ID
        }))
    }

    pub async fn posts_create(State(_state): State<AppState>, inertia: Inertia) -> impl IntoResponse {
        inertia.render("Posts/Create", json!({}))
    }

    pub async fn posts_edit(State(_state): State<AppState>, inertia: Inertia) -> impl IntoResponse {
        inertia.render("Posts/Edit", json!({
            "post": {}  // TODO: Implement post fetching by ID
        }))
    }

    pub async fn login(State(_state): State<AppState>, inertia: Inertia) -> impl IntoResponse {
        inertia.render("Auth/Login", json!({}))
    }

    pub async fn register(State(_state): State<AppState>, inertia: Inertia) -> impl IntoResponse {
        inertia.render("Auth/Register", json!({}))
    }
} 