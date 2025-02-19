# {{project-name}}

{{description}}

## Development

This project uses `cargo-make` for task automation. Here are the available commands:

```bash
# Start development server with hot reload
cargo make dev

# Build for production
cargo make build

# Run tests
cargo make test

# Format code
cargo make format

# Run linter
cargo make lint

# Run format and lint
cargo make check
```

## Project Structure

```
src/
├── main.rs          # Application entry point
├── web.rs           # Route definitions
└── app/
    ├── controllers/ # Request handlers
    ├── models/      # Database models
    ├── dtos/        # Data Transfer Objects
    ├── factories/   # Test data factories
    └── seeders/     # Database seeders

templates/           # HTML templates
├── base.html       # Base template
└── home.html       # Home page template
```

## Environment Variables

Copy `.env.example` to `.env` and update the values:

```bash
cp .env.example .env
```

Available environment variables:

- `DATABASE_URL`: Database connection string
- `APP_NAME`: Application name
- `APP_ENV`: Environment (local, production)
- `APP_DEBUG`: Enable debug mode
- `APP_URL`: Application URL 