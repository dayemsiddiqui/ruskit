# Database Migrations

Database migrations in Ruskit allow you to evolve your database schema over time. Each migration represents a set of changes to your database structure.

## Migration Commands

- `cargo kit migrate` - Run pending migrations and generate entities
- `cargo kit migrate:fresh` - Drop all tables, re-run migrations, and generate entities
- `cargo kit make:migration` - Create a new migration file

## Creating Migrations

### New Table Migration

```bash
cargo kit make:migration create_users_table --model User
```

This creates a new migration file in `database/migrations/` with a timestamp prefix.

### Adding Fields

```bash
cargo kit make:migration add_status_to_users --model User
```

## Migration Structure

Each migration file contains an `up` and `down` function:

```rust
use ruskit::framework::database::migration::Migration;
use ruskit::framework::database::DatabaseError;
use sqlx::{Pool, Sqlite};

pub struct CreateUsersTable;

#[async_trait::async_trait]
impl Migration for CreateUsersTable {
    async fn up(&self, pool: &Pool<Sqlite>) -> Result<(), DatabaseError> {
        sqlx::query(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )"
        )
        .execute(pool)
        .await?;
        
        Ok(())
    }

    async fn down(&self, pool: &Pool<Sqlite>) -> Result<(), DatabaseError> {
        sqlx::query("DROP TABLE users")
            .execute(pool)
            .await?;
            
        Ok(())
    }
}
```

## Auto-Generation Process

When you run migrations, Ruskit:

1. Executes pending migrations
2. Inspects the database schema
3. Generates entity files based on the schema
4. Updates the entities module exports

The generated entities include:
- Documentation about auto-generation
- Instructions for regeneration
- Field-level documentation
- Proper type mappings

## Best Practices

1. **Migration Names**:
   - Use descriptive names (`create_users_table`)
   - Include model name for context
   - Use snake_case

2. **Schema Design**:
   - Always include `id`, `created_at`, `updated_at`
   - Use appropriate SQLite types
   - Consider indexing for performance
   - Define foreign key constraints

3. **Running Migrations**:
   - Test migrations locally first
   - Back up production data
   - Run `migrate:fresh` in development
   - Use `migrate` in production

4. **Entity Generation**:
   - Never edit generated entities
   - Create new migrations for changes
   - Review generated entities after migration

## Common Patterns

### Primary Keys

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,  -- Auto-incrementing primary key
    -- other fields
);
```

### Timestamps

```sql
CREATE TABLE posts (
    -- other fields
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);
```

### Foreign Keys

```sql
CREATE TABLE comments (
    id INTEGER PRIMARY KEY,
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (post_id) REFERENCES posts(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### Nullable Fields

```sql
CREATE TABLE profiles (
    id INTEGER PRIMARY KEY,
    user_id INTEGER NOT NULL,
    bio TEXT,  -- Nullable
    website TEXT,  -- Nullable
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### Unique Constraints

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    -- other fields
);
```

## Troubleshooting

If entity generation fails:

1. Check migration syntax
2. Verify foreign key constraints
3. Run `migrate:fresh` to start clean
4. Check database connection
5. Review migration logs 