# Database Seeders

Database seeders allow you to populate your database with test data. They use model factories to generate realistic data.

## Installation

After making changes to the seeder system or any CLI commands, rebuild and reinstall the binary:

```bash
cargo install --path . --force
```

This ensures you have the latest version of the `cargo-kit` command available.

## Creating a Seeder

To create a new seeder, use the `make:seeder` command:

```bash
cargo kit make:seeder User
```

This will create a new seeder file at `src/app/seeders/userseeder.rs`:

```rust
use crate::framework::database::seeder::{Seeder, DatabaseSeeder};
use crate::framework::database::DatabaseError;
use crate::app::models::User;
use crate::framework::database::factory::Factory;

pub struct UserSeeder;

#[async_trait::async_trait]
impl Seeder for UserSeeder {
    async fn run(&self) -> Result<(), DatabaseError> {
        // Create test data using the factory
        User::create_many(10).await?;
        Ok(())
    }
}

// Auto-register the seeder
static SEEDER: Lazy<()> = Lazy::new(|| {
    DatabaseSeeder::register(Box::new(UserSeeder));
});
```

## Running Seeders

To run all registered seeders:

```bash
cargo kit db:seed
```

## Writing Custom Seeders

You can customize your seeder to create more complex data:

```rust
async fn run(&self) -> Result<(), DatabaseError> {
    // Create admin user
    let admin = User::create(json!({
        "name": "Admin User",
        "email": "admin@example.com",
        "role": "admin",
        "created_at": now,
        "updated_at": now
    })).await?;

    // Create regular users
    let users = User::create_many(10).await?;

    // Create related data
    for user in users {
        Post::create(json!({
            "user_id": user.id,
            "title": Sentence().fake::<String>(),
            "content": Paragraph().fake::<String>(),
            "created_at": now,
            "updated_at": now
        })).await?;
    }

    Ok(())
}
```

## Auto-Discovery

Seeders are automatically discovered and registered when they are placed in the `src/app/seeders` directory. Each seeder should:

1. Implement the `Seeder` trait
2. Include the static registration code
3. Be added to the `mod.rs` file (done automatically by the generator) 