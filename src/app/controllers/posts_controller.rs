use axum::{
    extract::State,
    routing::{get, post, delete},
    Json, Router,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use crate::app::entities::{post, post::Entity as Post};

pub fn routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/posts", get(index))
        .route("/posts", post(store))
        .route("/posts/:id", get(show))
        .route("/posts/:id", post(update))
        .route("/posts/:id", delete(destroy))
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    title: String,
    content: String,
    slug: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePost {
    title: Option<String>,
    content: Option<String>,
    slug: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    id: i32,
    title: String,
    content: String,
    slug: String,
    created_at: String,
    updated_at: String,
}

impl From<post::Model> for PostResponse {
    fn from(post: post::Model) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            slug: post.slug,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}

async fn index(
    State(db): State<DatabaseConnection>,
) -> Json<Vec<PostResponse>> {
    let posts = Post::find()
        .all(&db)
        .await
        .unwrap_or_default();

    Json(posts.into_iter().map(Into::into).collect())
}

async fn store(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreatePost>,
) -> Json<PostResponse> {
    let post = post::ActiveModel {
        title: Set(payload.title),
        content: Set(payload.content),
        slug: Set(payload.slug),
        ..Default::default()
    };

    let post = post.insert(&db).await.unwrap();
    Json(post.into())
}

async fn show(
    State(db): State<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Json<Option<PostResponse>> {
    let post = Post::find_by_id(id)
        .one(&db)
        .await
        .unwrap()
        .map(Into::into);

    Json(post)
}

async fn update(
    State(db): State<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<i32>,
    Json(payload): Json<UpdatePost>,
) -> Json<Option<PostResponse>> {
    let post = match Post::find_by_id(id).one(&db).await.unwrap() {
        Some(post) => post,
        None => return Json(None),
    };

    let mut post: post::ActiveModel = post.into();

    if let Some(title) = payload.title {
        post.title = Set(title);
    }
    if let Some(content) = payload.content {
        post.content = Set(content);
    }
    if let Some(slug) = payload.slug {
        post.slug = Set(slug);
    }

    let post = post.update(&db).await.unwrap();
    Json(Some(post.into()))
}

async fn destroy(
    State(db): State<DatabaseConnection>,
    axum::extract::Path(id): axum::extract::Path<i32>,
) -> Json<bool> {
    let res = Post::delete_by_id(id).exec(&db).await.unwrap();
    Json(res.rows_affected > 0)
} 