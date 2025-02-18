# Commands

Ruskit provides a set of CLI commands to help you scaffold and manage your application. These commands are available through the `cargo kit` command-line interface.

## Available Commands

### Database Commands

- `cargo kit migrate` - Run all pending database migrations
- `cargo kit migrate:fresh` - Drop all tables and re-run all migrations
- `cargo kit db:seed` - Seed the database with sample data

### Development Commands

- `cargo kit dev` - Start development server with hot reload
- `cargo kit serve` - Start production server

### Make Commands

#### Models and Database

- `cargo kit make:model <Name>` - Create a new model with migration
  ```bash
  cargo kit make:model User
  ```

- `cargo kit make:migration <name> --model <ModelName>` - Create a new migration for an existing model
  ```bash
  cargo kit make:migration add_email_to_users --model User
  ```

- `cargo kit make:factory <Name>` - Create a new factory for a model
  ```bash
  cargo kit make:factory User
  ```

- `cargo kit make:seeder <Name>` - Create a new database seeder
  ```bash
  cargo kit make:seeder Users
  ```

#### Controllers and DTOs

- `cargo kit make:controller <Name>` - Create a new controller
  ```bash
  cargo kit make:controller User
  ```

- `cargo kit make:dto <Name>` - Create a new DTO (Data Transfer Object)
  ```bash
  cargo kit make:dto User
  ```

#### All-in-One Scaffolding

- `cargo kit make:all <Name>` - Create model, DTO, and controller in one command
  ```bash
  cargo kit make:all Post
  ```
  This command will:
  1. Create a model with migration
  2. Create DTOs for requests and responses
  3. Create a controller with basic CRUD operations

## Command Behavior

### Model Generation
- Creates a new model file in `src/app/models`
- Automatically creates a migration
- Updates `mod.rs` to include the new model
- Includes basic timestamp fields and ID

### Controller Generation
- Creates a new controller in `src/app/controllers`
- Checks for existence of related model and DTOs
- Generates CRUD methods if model and DTOs exist
- Updates `mod.rs` to include the new controller

### DTO Generation
- Creates request and response DTOs in `src/app/dtos`
- Includes validation support
- Generates From implementations for model conversion
- Updates `mod.rs` to include the new DTOs

### Migration Generation
- Creates a new migration in the model file
- Uses timestamp-based ordering
- Provides up and down migration templates

### Factory Generation
- Creates a new factory in `src/app/factories`
- Includes fake data generation setup
- Updates `mod.rs` to include the new factory

### Seeder Generation
- Creates a new seeder in `src/app/seeders`
- Includes basic seeder structure
- Updates `mod.rs` to include the new seeder

## Best Practices

1. Use PascalCase for model, controller, and DTO names:
   ```bash
   cargo kit make:model User
   cargo kit make:controller BlogPost
   cargo kit make:dto UserProfile
   ```

2. Use snake_case for migration names:
   ```bash
   cargo kit make:migration add_email_to_users --model User
   ```

3. When using `make:all`, ensure the name represents a single entity:
   ```bash
   cargo kit make:all BlogPost  # Good
   cargo kit make:all blog_posts  # Not recommended
   ```

4. Run migrations after creating or modifying models:
   ```bash
   cargo kit migrate
   ```

5. Use `migrate:fresh` during development to reset the database:
   ```bash
   cargo kit migrate:fresh
   ``` 