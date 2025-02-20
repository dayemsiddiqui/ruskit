# Models and Entities

Ruskit uses a clear separation between entities (data structures) and models (business logic). Entities represent your database tables and their fields, while models contain the business logic, relationships, and database operations.

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
1. Create a new entity file in `src/app/entities/`
2. Create a new model file in `src/app/models/`
3. Add the entity to `src/app/entities/mod.rs`
4. Add the model to `src/app/models/mod.rs`
5. Generate:
   - Entity struct with validation fields
   - Model implementation with business logic
   - Migration setup
   - Basic query methods

## Entity Structure

Entities define your data structure and validation rules:

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use rustavel_derive::GenerateValidationFields;
use crate::framework::database::model::{Field, ModelValidation};
use validator::ValidationError;

#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct Post {
    #[sqlx(default)]
    pub id: i64,
    pub title: String,
    pub content: String,
    pub user_id: i64,
    pub created_at: i64,
    pub updated_at: i64,
}
```

## Model Implementation

Models contain your business logic, relationships, and database operations:

```rust
use validator::ValidationError;
use crate::framework::database::{
    model::{Model, Rules, Validate, ValidationRules},
    query_builder::QueryBuilder,
    DatabaseError,
    migration::Migration,
};
use crate::app::entities::Post;
use async_trait::async_trait;

impl Post {
    /// Get recent records
    pub async fn recent(limit: i64) -> Result<Vec<Self>, DatabaseError> {
        QueryBuilder::table(Self::table_name())
            .order_by("created_at", "DESC")
            .limit(limit)
            .get::<Self>()
            .await
    }
}

impl ValidationRules for Post {
    fn validate_rules(&self) -> Result<(), ValidationError> {
        self.title.validate(Rules::new().required().min(3).max(255))?;
        self.content.validate(Rules::new().required())?;
        Ok(())
    }
}

#[async_trait]
impl Model for Post {
    fn table_name() -> &'static str {
        "posts"
    }

    fn id(&self) -> i64 {
        self.id
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

## Model Validation

Ruskit provides a powerful validation system using the `GenerateValidationFields` derive macro and the `ValidationRules` trait:

```rust
// In your entity file:
#[derive(Debug, Serialize, Deserialize, FromRow, GenerateValidationFields)]
pub struct User {
    #[sqlx(default)]
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// In your model file:
impl ValidationRules for User {
    fn validate_rules(&self) -> Result<(), ValidationError> {
        self.name.validate(Rules::new().required().min(3).max(255))?;
        self.email.validate(Rules::new().required().email())?;
        Ok(())
    }
}
```

The validation will be automatically applied when creating or updating records.

## Relationships

Ruskit provides three types of relationships: `HasOne`, `HasMany`, and `BelongsTo`. Here's how to use them:

### HasMany Relationship

Used when a model has multiple related records:

```rust
use crate::framework::database::model::HasMany;
use crate::app::entities::Post;

impl User {
    /// Get all posts by this user
    pub fn posts(&self) -> HasMany<Post> {
        HasMany::new::<Self>()
    }
}

// Usage:
let user = User::find(1).await?;
let posts = user.posts().get().await?;  // Get all posts
let new_post = user.posts().create(post).await?;  // Create a new post
```

### BelongsTo Relationship

Used when a model belongs to another model:

```rust
use crate::framework::database::model::BelongsTo;
use crate::app::entities::User;

impl Post {
    /// Get the user who created this post
    pub fn user(&self) -> BelongsTo<User> {
        BelongsTo::new::<Self>()
    }
}

// Usage:
let post = Post::find(1).await?;
let user = post.user().get().await?;  // Get the related user
```

### HasOne Relationship

Used when a model has exactly one related record:

```rust
use crate::framework::database::model::HasOne;
use crate::app::entities::{User, Profile};

impl User {
    /// Get user's profile
    pub fn profile(&self) -> HasOne<Self, Profile> {
        HasOne::new("user_id")
    }
}

// Usage:
let user = User::find(1).await?;
let profile = user.profile().get().await?;  // Get the related profile
```

### Custom Foreign Keys

You can specify custom foreign keys for any relationship:

```rust
// Custom foreign key for HasMany
HasMany::with_key("author_id")

// Custom foreign key for BelongsTo
BelongsTo::with_key("author_id")

// Custom foreign key for HasOne
HasOne::new("author_id")
```

## Basic Operations

Every model automatically gets these basic operations:

```rust
// Find by ID
let post = Post::find(1).await?;

// Get all records
let all_posts = Post::all().await?;

// Create a new record
let post = Post {
    id: 0,
    title: "My First Post".to_string(),
    content: "Hello World!".to_string(),
    user_id: 1,
    created_at: chrono::Utc::now().timestamp(),
    updated_at: chrono::Utc::now().timestamp(),
};
let created_post = Post::create(post).await?;

// Create with validation
let validated_post = Post::create_validated(post).await?;
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

## Best Practices

1. **Separation of Concerns**:
   - Keep data structure in entities
   - Keep business logic in models
   - Use relationships for model associations

2. **Validation**:
   - Use `GenerateValidationFields` for automatic validation field generation
   - Implement `ValidationRules` for custom validation logic
   - Validate data before saving to database

3. **Naming Conventions**:
   - Use singular names for entity and model structs (`Post`, not `Posts`)
   - Use snake_case for table names (`posts`, not `Posts`)
   - Use descriptive names for relationships and methods

4. **Field Types**:
   - Use appropriate types for your fields (e.g., `i64` for IDs and timestamps)
   - Consider using `Option<T>` for nullable fields
   - Use `bool` for boolean fields (SQLite stores them as INTEGER)

5. **Timestamps**:
   - Always include `created_at` and `updated_at` fields
   - Use UNIX timestamps (seconds since epoch) for consistency

6. **Security**:
   - Never expose sensitive fields directly
   - Implement proper access control in your models
   - Validate input data before creating/updating records

7. **Performance**:
   - Add indexes for frequently queried fields
   - Use appropriate field types for better performance
   - Consider adding caching for frequently accessed data 