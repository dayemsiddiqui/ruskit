# Model Factories

Model factories allow you to generate test data for your models using the Faker library. They are useful for seeding your database with realistic test data.

## Installation

After making changes to the factory system or any CLI commands, rebuild and reinstall the binary:

```bash
cargo install --path . --force
```

This ensures you have the latest version of the `cargo-kit` command available.

## Creating a Factory

To create a new factory, use the `make:factory` command:

```bash
cargo kit make:factory User
```

This will create a new factory file at `src/app/factories/user_factory.rs` with a basic implementation:

```rust
use crate::app::models::User;
use crate::framework::database::factory::Factory;
use fake::{faker::internet::en::*, faker::name::en::*, Fake};
use serde_json::json;

impl Factory for User {
    fn definition() -> serde_json::Value {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        json!({
            "name": Name().fake::<String>(),
            "email": FreeEmail().fake::<String>(),
            "created_at": now,
            "updated_at": now
        })
    }
}
```

## Using Factories

You can use factories in your tests or seeders:

```rust
// Create a single instance
let user = User::factory().await?;

// Create multiple instances
let users = User::create_many(10).await?;
```

## Available Faker Generators

The `fake` crate provides many generators for different types of data:

- Names: `Name()`, `FirstName()`, `LastName()`
- Internet: `FreeEmail()`, `SafeEmail()`, `Username()`
- Lorem: `Sentence()`, `Paragraph()`
- Numbers: `Number()`, `NumberWithFormat()`
- And many more...

Check the [fake crate documentation](https://docs.rs/fake) for more generators. 