# Models

Models in Ruskit represent the data structure and business logic of your application. They provide an abstraction layer for interacting with your database and implementing domain-specific functionality.

## Generating Models

You can quickly generate a new model using the `kit make:model` command:

```bash
# Generate a model
cargo kit make:model Post

# Generate a model with migration
cargo kit make:model Post --migration

# After generating a model with migration, run:
cargo kit migrate
```

This will:
1. Create a new model file in `src/app/models/`
2. Add the model to `src/app/models/mod.rs`
3. Generate a basic model structure with:
   - Standard fields (id, created_at, updated_at)
   - Model trait implementation
   - Migration setup
   - Basic query methods

The generated model will include TODO comments to help you add your custom fields and methods.

## Creating a Model Manually

If you prefer to create a model manually, create a new file in `src/app/models/` and define your struct with the necessary derives:

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::framework::database::{
    model::Model,
    query_builder::QueryBuilder,
    DatabaseError,
    migration::Migration,
};
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub user_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}
```

Then implement the `Model` trait:

```rust
#[async_trait]
impl Model for Post {
    fn table_name() -> &'static str {
        "posts"
    }

    fn migrations() -> Vec<Migration> {
        vec![
            Migration::new(
                "create_posts_table",
                "CREATE TABLE posts (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    content TEXT NOT NULL,
                    user_id INTEGER NOT NULL,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL,
                    FOREIGN KEY (user_id) REFERENCES users(id)
                )",
                "DROP TABLE posts"
            )
        ]
    }
}
```

## Basic Operations

Every model automatically gets these basic operations:

```rust
// Find by ID
let post = Post::find(1).await?;

// Get all records
let all_posts = Post::all().await?;

// Create a new record
let new_post = Post::create(json!({
    "title": "My First Post",
    "content": "Hello World!",
    "user_id": 1,
    "created_at": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
    "updated_at": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
})).await?;
```

## Custom Query Methods

You can add custom query methods to your models:

```rust
impl Post {
    // Get posts by user
    pub async fn by_user(user_id: i64) -> Result<Vec<Self>, DatabaseError> {
        QueryBuilder::table(Self::table_name())
            .where_clause("user_id", "=", user_id)
            .get::<Self>()
            .await
    }

    // Get recent posts
    pub async fn recent(limit: i64) -> Result<Vec<Self>, DatabaseError> {
        QueryBuilder::table(Self::table_name())
            .order_by("created_at", "DESC")
            .limit(limit)
            .get::<Self>()
            .await
    }

    // Get published posts
    pub async fn published() -> Result<Vec<Self>, DatabaseError> {
        QueryBuilder::table(Self::table_name())
            .where_clause("published", "=", true)
            .get::<Self>()
            .await
    }
}
```

## Query Builder

The `QueryBuilder` provides a fluent interface for building SQL queries:

```rust
// Complex query example
let posts = QueryBuilder::table("posts")
    .select("posts.*, users.name as author_name")
    .where_clause("published", "=", true)
    .where_clause("created_at", ">", yesterday_timestamp)
    .order_by("created_at", "DESC")
    .limit(10)
    .offset(20)
    .get::<Post>()
    .await?;
```

## Relationships

You can define relationships between models:

```rust
impl Post {
    // Get the author of the post
    pub async fn author(&self) -> Result<User, DatabaseError> {
        User::find(self.user_id).await?
            .ok_or(DatabaseError::ConnectionError(
                sqlx::Error::RowNotFound
            ))
    }

    // Get comments for the post
    pub async fn comments(&self) -> Result<Vec<Comment>, DatabaseError> {
        Comment::by_post(self.id).await
    }
}
```

## Model Validation

You can add validation to your models using the `validator` crate:

```rust
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Post {
    pub id: i64,
    #[validate(length(min = 3, max = 100))]
    pub title: String,
    #[validate(length(min = 10))]
    pub content: String,
    pub user_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Post {
    pub async fn create_validated(attrs: serde_json::Value) -> Result<Self, ValidationError> {
        let post: Post = serde_json::from_value(attrs)?;
        post.validate()?;
        Ok(Self::create(attrs).await?)
    }
}
```

## Best Practices

1. **Naming Conventions**:
   - Use singular names for model structs (`Post`, not `Posts`)
   - Use snake_case for table names (`posts`, not `Posts`)
   - Use descriptive names for relationships and methods

2. **Field Types**:
   - Use appropriate types for your fields (e.g., `i64` for IDs and timestamps)
   - Consider using `Option<T>` for nullable fields
   - Use `bool` for boolean fields (SQLite stores them as INTEGER)

3. **Timestamps**:
   - Always include `created_at` and `updated_at` fields
   - Use UNIX timestamps (seconds since epoch) for consistency

4. **Security**:
   - Never expose sensitive fields directly
   - Implement proper access control in your models
   - Validate input data before creating/updating records

5. **Performance**:
   - Add indexes for frequently queried fields
   - Use appropriate field types for better performance
   - Consider adding caching for frequently accessed data 