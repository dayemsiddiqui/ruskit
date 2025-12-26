# Ruskit
**This REPO has been archieved in favor of a more well maintained project https://github.com/dayemsiddiqui/kit, If you are starting a new project please checkout https://github.com/dayemsiddiqui/kit**

Ruskit is a web application framework with expressive, elegant syntax inspired by Laravel, built for Rust. We believe development must be an enjoyable and creative experience. Ruskit takes the pain out of web development by easing common tasks used in many web projects.

## Features

- 🚀 Expressive routing system
- 🔒 Built-in authentication and authorization
- 📦 Powerful dependency injection container
- 🗄️ Elegant database ORM with:
  - Clear separation of entities and models
  - Automatic validation using derive macros
  - Fluent relationship definitions (HasOne, HasMany, BelongsTo)
  - Type-safe query builder
- ⚡ High performance and memory safety
- 🛠️ Developer-friendly CLI tools
- 🔧 Configuration management
- 📝 Robust logging system

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [cargo-generate](https://github.com/cargo-generate/cargo-generate) - Install with:
  ```bash
  cargo install cargo-generate
  ```
- [cargo-make](https://github.com/sagiegurari/cargo-make) - Install with:
  ```bash
  cargo install cargo-make
  ```

## Quick Start

First, install the Ruskit CLI tool:

```bash
# Install the Ruskit CLI tool
cargo install ruskit
```

Then create a new Ruskit project:

```bash
# Create a new project
cargo kit new my-project

# Navigate to project directory
cd my-project

# Start the development server
cargo make dev
```

## Development Tools

Ruskit comes with several CLI tools to help you develop your application:

```bash
# Create a new model (generates both entity and model files)
cargo kit make:model Post

# Create a new controller
cargo kit make:controller PostController

# Create a new DTO
cargo kit make:dto Post

# Create all components (entity, model, controller, DTO)
cargo kit make:all Post

# Run database migrations
cargo kit migrate

# Start development server with hot reload
cargo kit dev
```

## Project Structure

```
src/
├── app/
│   ├── entities/        # Data structures and validation rules
│   ├── models/          # Business logic and relationships
│   ├── controllers/     # Request handlers
│   ├── dtos/           # Data transfer objects
│   ├── factories/       # Test data factories
│   └── seeders/        # Database seeders
├── framework/          # Core framework components
└── web.rs             # Route definitions
```

## Documentation

For detailed documentation, please visit:
- [Routing](/docs/routing.md)
- [Models](/docs/models.md)
- [Controllers](/docs/controllers.md)
- [Views](/docs/views.md)
- [Database](/docs/database.md)
- [Authentication](/docs/auth.md)

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
