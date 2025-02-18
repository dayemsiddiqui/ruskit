# Database Migrations

Ruskit provides a robust database migration system that ensures your database schema changes are versioned, ordered, and can be rolled out consistently across different environments.

## Migration Commands

### Create a New Model with Migration

```bash
cargo kit make:model Post
```

This will:
1. Create a new model file at `src/app/models/post.rs`
2. Add the model to `src/app/models/mod.rs`
3. Create an initial migration with timestamp prefix (e.g., `1739885809_create_posts_table`)
4. Set up basic model structure with id, created_at, and updated_at fields

### Add a New Migration to Existing Model

```bash
cargo kit make:migration add_email_to_users --model User
```

This will:
1. Add a new migration to the specified model's migrations vector
2. Automatically prefix the migration with a timestamp (e.g., `1739885809_add_email_to_users`)
3. Create placeholders for UP and DOWN migrations

### Run Migrations

```bash
cargo kit migrate
```

This will:
1. Discover all models in your application
2. Collect all migrations from these models
3. Sort migrations by timestamp to ensure consistent order
4. Run any migrations that haven't been executed yet

### Fresh Migrations

```bash
cargo kit migrate:fresh
```

This command will:
1. Drop all tables in your database (including the migrations table)
2. Re-run all migrations from scratch

This is useful when you want to:
- Reset your database to a clean state
- Test your migrations work correctly from a fresh start
- Resolve issues with inconsistent migration states

Note: This command will delete all data in your database. Use with caution in production environments.

## Migration Structure

Each migration consists of:
1. A unique name with timestamp prefix (e.g., `1739885809_create_users_table`)
2. An UP migration (the changes to apply)
3. A DOWN migration (how to reverse the changes)

Example migration:
```rust
Migration::new(
    "1739885809_add_email_to_users",
    // UP migration
    "ALTER TABLE users ADD COLUMN email TEXT;",
    // DOWN migration
    "ALTER TABLE users DROP COLUMN email;"
)
```

## Migration Ordering

Migrations are automatically ordered by their timestamp prefix, ensuring that:
1. Migrations run in the order they were created
2. New migrations always run after existing ones
3. The order is consistent across all environments

## Best Practices

1. **One Change Per Migration**: Each migration should handle one specific change (e.g., adding a column, creating a table)

2. **Meaningful Names**: Use descriptive names for migrations:
   - `create_users_table`
   - `add_email_to_users`
   - `add_foreign_key_to_posts`

3. **Always Include DOWN Migrations**: Make sure your DOWN migrations correctly reverse the changes made in UP migrations

4. **Run Migrations Immediately**: After creating a new migration, run `cargo kit migrate` to apply it

5. **Version Control**: Commit your migrations along with related code changes

## Migration Files

Migrations are stored in your model files under the `migrations()` function:

```rust
impl Model for User {
    fn migrations() -> Vec<Migration> {
        vec![
            Migration::new(
                "1739885800_create_users_table",
                "CREATE TABLE users (...)",
                "DROP TABLE users"
            ),
            Migration::new(
                "1739885809_add_email_to_users",
                "ALTER TABLE users ADD COLUMN email TEXT;",
                "ALTER TABLE users DROP COLUMN email;"
            ),
        ]
    }
}
```

## Migration Table

Ruskit maintains a `migrations` table in your database to track which migrations have been run:

```sql
CREATE TABLE migrations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    batch INTEGER NOT NULL,
    migration_time INTEGER NOT NULL
)
```

This ensures that:
1. Each migration runs exactly once
2. Migrations can be rolled back by batch
3. You can track when each migration was applied 