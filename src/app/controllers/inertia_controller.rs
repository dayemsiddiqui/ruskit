use axum::extract::State;
use axum::response::IntoResponse;
use axum_inertia::Inertia;
use sea_orm::{EntityTrait, QueryOrder};
use serde_json::json;

use crate::{web::AppState, app::entities::post::Entity as Post};

pub struct InertiaController;

impl InertiaController {
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

        inertia.render("Posts/Index", json!({
            "posts": posts
        }))
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
} 