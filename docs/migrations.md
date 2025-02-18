# Database Migrations

Migrations in Rustavel provide a way to evolve your database schema over time. Each migration represents a change to your database schema and includes both the change itself ("up" migration) and how to reverse it ("down" migration).

## Basic Migration Structure

Migrations are defined in your model's `migrations()` method:

```rust
fn migrations() -> Vec<Migration> {
    vec![
        Migration::new(
            "create_posts_table",                     // Migration name
            "CREATE TABLE posts (...)",               // Up migration
            "DROP TABLE posts"                        // Down migration
        )
    ]
}
```

## Running Migrations

Rustavel provides several commands for managing migrations:

```bash
# Run pending migrations
cargo kit migrate

# Rollback the last batch of migrations
cargo kit migrate:rollback

# Drop all tables and re-run all migrations
cargo kit migrate:fresh
```

## Migration Types

### 1. Creating Tables

```rust
Migration::new(
    "create_users_table",
    "CREATE TABLE users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE,
        password TEXT NOT NULL,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL
    )",
    "DROP TABLE users"
)
```

### 2. Adding Columns

```rust
Migration::new(
    "add_avatar_to_users",
    "ALTER TABLE users ADD COLUMN avatar TEXT",
    "ALTER TABLE users DROP COLUMN avatar"
)
```

### 3. Adding Indexes

```rust
Migration::new(
    "add_users_email_index",
    "CREATE UNIQUE INDEX idx_users_email ON users(email)",
    "DROP INDEX idx_users_email"
)
```

### 4. Adding Foreign Keys

```rust
Migration::new(
    "create_comments_table",
    "CREATE TABLE comments (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        post_id INTEGER NOT NULL,
        user_id INTEGER NOT NULL,
        content TEXT NOT NULL,
        created_at INTEGER NOT NULL,
        updated_at INTEGER NOT NULL,
        FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
    )",
    "DROP TABLE comments"
)
```

### 5. Modifying Columns (SQLite Workaround)

Since SQLite doesn't support directly modifying columns, we need to:
1. Create a new table with the desired schema
2. Copy the data
3. Drop the old table
4. Rename the new table

```rust
Migration::new(
    "make_content_nullable",
    "ALTER TABLE posts RENAME TO posts_old;
     CREATE TABLE posts (
         id INTEGER PRIMARY KEY AUTOINCREMENT,
         title TEXT NOT NULL,
         content TEXT,           -- Now nullable
         user_id INTEGER NOT NULL,
         created_at INTEGER NOT NULL,
         updated_at INTEGER NOT NULL,
         FOREIGN KEY (user_id) REFERENCES users(id)
     );
     INSERT INTO posts SELECT * FROM posts_old;
     DROP TABLE posts_old;",
    "ALTER TABLE posts RENAME TO posts_old;
     CREATE TABLE posts (
         id INTEGER PRIMARY KEY AUTOINCREMENT,
         title TEXT NOT NULL,
         content TEXT NOT NULL,  -- Back to NOT NULL
         user_id INTEGER NOT NULL,
         created_at INTEGER NOT NULL,
         updated_at INTEGER NOT NULL,
         FOREIGN KEY (user_id) REFERENCES users(id)
     );
     INSERT INTO posts SELECT * FROM posts_old;
     DROP TABLE posts_old;"
)
```

## Complex Migration Examples

### 1. Creating a Join Table

```rust
Migration::new(
    "create_post_tags_table",
    "CREATE TABLE post_tags (
        post_id INTEGER NOT NULL,
        tag_id INTEGER NOT NULL,
        created_at INTEGER NOT NULL,
        PRIMARY KEY (post_id, tag_id),
        FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE,
        FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
    )",
    "DROP TABLE post_tags"
)
```

### 2. Adding Multiple Indexes

```rust
Migration::new(
    "add_posts_indexes",
    "CREATE INDEX idx_posts_created_at ON posts(created_at);
     CREATE INDEX idx_posts_user_published ON posts(user_id, published);",
    "DROP INDEX idx_posts_created_at;
     DROP INDEX idx_posts_user_published;"
)
```

### 3. Data Migration

```rust
Migration::new(
    "convert_timestamps_to_unix",
    "UPDATE posts SET 
        created_at = strftime('%s', datetime(created_at)),
        updated_at = strftime('%s', datetime(updated_at));",
    "UPDATE posts SET 
        created_at = datetime(created_at, 'unixepoch'),
        updated_at = datetime(updated_at, 'unixepoch');"
)
```

## Best Practices

1. **Migration Naming**:
   - Use descriptive names that indicate what the migration does
   - Include a timestamp or sequence number if needed
   - Use snake_case for consistency

2. **Data Safety**:
   - Always provide both "up" and "down" migrations
   - Test migrations with real data before running in production
   - Back up your database before running migrations
   - Consider data volume when writing migrations

3. **Schema Design**:
   - Use appropriate field types
   - Add indexes for fields used in WHERE clauses
   - Consider foreign key constraints
   - Use NOT NULL when appropriate

4. **Performance**:
   - Break large migrations into smaller steps
   - Consider running data migrations in batches
   - Add indexes after bulk data loading
   - Remove unused indexes

5. **Maintenance**:
   - Keep migrations reversible
   - Document complex migrations
   - Don't modify existing migrations
   - Use transactions for data consistency

6. **SQLite Specific**:
   - Remember SQLite limitations (e.g., ALTER TABLE)
   - Use the table recreation pattern for column modifications
   - Enable foreign key constraints
   - Consider using WAL mode for better concurrency 