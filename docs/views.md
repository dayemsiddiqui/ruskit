# Views and Templating in Ruskit

Ruskit uses [Askama](https://github.com/djc/askama) as its templating engine, providing compile-time template checking and high performance. The syntax is similar to Jinja2/Django templates, making it familiar for developers coming from other frameworks.

## Template Storage

Templates in Ruskit are stored in the `templates` directory at your project root. Each template is a `.html` file that can extend other templates.

```rust
// Define a template in your route handler
#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

// Use it in your handler
async fn home() -> Response {
    let template = HomeTemplate;
    template.into_response()
}
```

### Template Inheritance

Ruskit encourages template inheritance through a base layout:

```html
<!-- templates/base.html -->
<!DOCTYPE html>
<html>
<head>
    <title>{% block title %}Ruskit{% endblock %}</title>
</head>
<body>
    {% block content %}{% endblock %}
</body>
</html>

<!-- templates/home.html -->
{% extends "base.html" %}

{% block title %}Welcome | Ruskit{% endblock %}

{% block content %}
    <h1>Welcome to Ruskit!</h1>
{% endblock %}
```

## Tailwind CSS Integration

Ruskit comes with out-of-the-box support for Tailwind CSS, providing a modern utility-first CSS framework for your templates.

### Default Setup

Every Ruskit template automatically includes the latest version of Tailwind CSS via CDN:

```html
<script src="https://cdn.tailwindcss.com"></script>
<script>
    window.tailwind.config = {
        theme: {
            extend: {
                colors: {
                    'ruskit': '#B7410E',
                },
            },
        },
    }
</script>
```

### Production Setup

For production, we recommend using the minified build with a specific version:

```html
<script src="https://cdn.tailwindcss.com/3.4.1"></script>
```

### Custom Configuration

To customize Tailwind, create a `tailwind.config.js` file in your project root:

```javascript
module.exports = {
  content: ["./templates/**/*.html"],
  theme: {
    extend: {
      colors: {
        'ruskit': '#B7410E',
      },
    },
  },
  plugins: [],
}
```

Then use the local build process:

1. Install Node.js dependencies:
```bash
npm init -y
npm install -D tailwindcss
npx tailwindcss init
```

2. Create a CSS file (`static/css/app.css`):
```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

3. Add build script to `package.json`:
```json
{
  "scripts": {
    "build:css": "tailwindcss -i ./static/css/app.css -o ./static/css/tailwind.css"
  }
}
```

4. Update your base template to use the local build:
```html
<link href="/static/css/tailwind.css" rel="stylesheet">
```

### Example Usage

Here's how to use Tailwind CSS in your Ruskit templates:

```html
{% extends "base.html" %}

{% block content %}
<div class="min-h-screen bg-gray-100">
    <nav class="bg-white shadow-lg">
        <div class="max-w-6xl mx-auto px-4">
            <div class="flex justify-between">
                <div class="flex space-x-7">
                    <a href="/" class="flex items-center py-4">
                        <span class="font-semibold text-ruskit text-lg">
                            Ruskit
                        </span>
                    </a>
                </div>
                <div class="flex items-center space-x-3">
                    <a href="/login" class="py-2 px-4 bg-ruskit text-white rounded hover:bg-opacity-90">
                        Login
                    </a>
                </div>
            </div>
        </div>
    </nav>
    
    <main class="container mx-auto mt-8 px-4">
        <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
            <h1 class="text-2xl font-bold mb-4">{{ title }}</h1>
            <p class="text-gray-700">{{ content }}</p>
        </div>
    </main>
</div>
{% endblock %}
```

## Passing Data to Templates

### Basic Data

```rust
#[derive(Template)]
#[template(path = "users/show.html")]
struct UserTemplate {
    username: String,
    email: String,
}

async fn show_user() -> Response {
    let template = UserTemplate {
        username: "john_doe".to_string(),
        email: "john@example.com".to_string(),
    };
    template.into_response()
}
```

```html
<!-- templates/users/show.html -->
{% extends "base.html" %}

{% block content %}
    <h1>User Profile</h1>
    <p>Username: {{ username }}</p>
    <p>Email: {{ email }}</p>
{% endblock %}
```

### Collections and Loops

```rust
#[derive(Template)]
#[template(path = "users/index.html")]
struct UsersTemplate {
    users: Vec<User>,
}

async fn users_index() -> Response {
    let users = vec![
        User { name: "John".to_string() },
        User { name: "Jane".to_string() },
    ];
    let template = UsersTemplate { users };
    template.into_response()
}
```

```html
<!-- templates/users/index.html -->
{% extends "base.html" %}

{% block content %}
    <h1>Users</h1>
    <ul>
    {% for user in users %}
        <li>{{ user.name }}</li>
    {% endfor %}
    </ul>
{% endblock %}
```

## Template Syntax

### Variables

```html
{{ variable }}
{{ user.name }}
{{ get_user_name() }}
```

### Control Structures

#### Conditionals
```html
{% if user.is_admin %}
    <span>Admin User</span>
{% else %}
    <span>Regular User</span>
{% endif %}
```

#### Loops
```html
{% for item in items %}
    <li>{{ item.name }}</li>
{% endfor %}
```

### Including Other Templates

```html
{% include "partials/header.html" %}
```

### Raw Content

```html
{% raw %}
    This content will not be processed by the template engine
{% endraw %}
```

## Best Practices

1. **Template Organization**
   - Keep templates in the `templates` directory
   - Use subdirectories for different sections (e.g., `users`, `admin`, `auth`)
   - Create a `partials` directory for reusable components

2. **Layout Structure**
   ```
   templates/
   ├── base.html
   ├── partials/
   │   ├── header.html
   │   └── footer.html
   ├── users/
   │   ├── index.html
   │   └── show.html
   └── auth/
       ├── login.html
       └── register.html
   ```

3. **Template Data**
   - Use strongly typed structs for template data
   - Keep template logic minimal
   - Move complex logic to the handler or service layer

4. **Error Handling**
   ```rust
   async fn show_user() -> Response {
       let template = UserTemplate::new()
           .map_err(|e| // handle error)?;
       template.into_response()
   }
   ```

## Security Considerations

1. **Auto-escaping**: Askama automatically escapes HTML special characters in variables
2. **CSRF Protection**: Use CSRF tokens in forms (documentation coming soon)
3. **XSS Prevention**: Don't use `{% raw %}` with user-provided content

## Coming Soon

- Form helpers
- CSRF protection
- Asset management
- Cache management
- Custom template functions
- Localization support

## Metadata System

Ruskit provides a powerful metadata system that allows you to manage page metadata (title, description, etc.) at both global and per-page levels.

### Global Metadata

Global metadata can be set during application bootstrap and serves as the default for all pages:

```rust
// In bootstrap.rs
app.metadata(|| {
    Metadata::new("Ruskit")
        .with_description("A modern web framework for Rust")
        .with_keywords("rust, web framework, ruskit")
        .with_author("Ruskit Team")
}).await;
```

### Per-Page Metadata

You can override metadata for specific pages using the `with_metadata` method:

```rust
async fn about() -> Response {
    AboutTemplate::with_metadata(
        Metadata::new("About Us")
            .with_description("Learn more about our team")
            .with_og_title("About Us")
    ).into_response()
}
```

### Combining Template Data with Metadata

Often you'll need to combine custom template data with metadata. Here's a complete example:

```rust
/// About page template with custom fields
#[derive(Template, Default)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    first_name: String,
    last_name: String,
}

// Route handler combining metadata with template data
async fn about() -> Response {
    let mut about_template = AboutTemplate::with_metadata(
        Metadata::new("About Us")
            .with_description("Learn more about our team")
            .with_og_title("About Us")
            .with_og_description("Meet John Doe, a key member of our team")
    );
    
    // Set template-specific data
    about_template.first_name = "John".to_string();
    about_template.last_name = "Doe".to_string();
    
    about_template.into_response()
}
```

And the corresponding template:

```html
{% extends "base.html" %}

{% block content %}
<div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4">
    <h1 class="text-2xl font-bold mb-4">{{ self.metadata().title }}</h1>
    
    {% match self.metadata().description %}
        {% when Some with (desc) %}
            <p class="text-gray-700 mb-4">{{ desc }}</p>
        {% when None %}
    {% endmatch %}

    <div class="prose">
        <p class="text-gray-600">
            Meet our team member {{ first_name }} {{ last_name }}.
        </p>
    </div>
</div>
{% endblock %}
```

This example shows how to:
1. Define a template with custom fields (`first_name`, `last_name`)
2. Set page-specific metadata using `with_metadata`
3. Set template-specific data after creating the template
4. Access both metadata and template data in the template file

The resulting page will have:
- Custom metadata for SEO and social sharing
- Template-specific content with the person's name
- Proper type safety for all fields
- Clean separation between metadata and content

### Available Metadata Fields

The `Metadata` struct provides several fields for SEO and social sharing:

- `title`: The page title
- `description`: Meta description for SEO
- `keywords`: Meta keywords
- `author`: Page author
- `og_title`: OpenGraph title for social sharing
- `og_description`: OpenGraph description
- `og_image`: OpenGraph image URL

### Using Metadata in Templates

Metadata can be accessed in templates using `self.metadata()`:

```html
<title>{{ self.metadata().title }}</title>

{% match self.metadata().description %}
    {% when Some with (desc) %}
        <meta name="description" content="{{ desc }}">
    {% when None %}
{% endmatch %}
```

### Base Template Example

Here's a complete example of a base template using all metadata fields:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    
    <!-- Basic Metadata -->
    <title>{% block title %}{{ self.metadata().title }}{% endblock %}</title>
    
    {% match self.metadata().description %}
        {% when Some with (desc) %}
            <meta name="description" content="{{ desc }}">
        {% when None %}
    {% endmatch %}
    
    {% match self.metadata().keywords %}
        {% when Some with (kw) %}
            <meta name="keywords" content="{{ kw }}">
        {% when None %}
    {% endmatch %}
    
    {% match self.metadata().author %}
        {% when Some with (author) %}
            <meta name="author" content="{{ author }}">
        {% when None %}
    {% endmatch %}
    
    <!-- OpenGraph Metadata -->
    {% match self.metadata().og_title %}
        {% when Some with (title) %}
            <meta property="og:title" content="{{ title }}">
        {% when None %}
    {% endmatch %}
    
    {% match self.metadata().og_description %}
        {% when Some with (desc) %}
            <meta property="og:description" content="{{ desc }}">
        {% when None %}
    {% endmatch %}
    
    {% match self.metadata().og_image %}
        {% when Some with (img) %}
            <meta property="og:image" content="{{ img }}">
        {% when None %}
    {% endmatch %}
</head>
<body>
    {% block content %}{% endblock %}
</body>
</html>
```

### How It Works

The metadata system uses thread-local storage to manage per-request metadata:

1. Global metadata is stored in a static `OnceLock`
2. Per-request metadata is stored in thread-local storage
3. The `metadata()` method checks for local metadata first, falling back to global metadata
4. Local metadata is automatically cleared after the response is sent

This ensures that:
- Each request can have its own metadata
- Metadata changes don't leak between requests
- Global defaults are always available
- No manual cleanup is required

### Best Practices

1. Set sensible global defaults during bootstrap
2. Use descriptive titles and descriptions for SEO
3. Include OpenGraph metadata for social sharing
4. Keep metadata concise and relevant
5. Use per-page metadata for unique pages
6. Reuse global metadata for generic pages

### Type Safety

The metadata system is fully type-safe:
- All fields have appropriate types
- Optional fields use `Option<String>`
- Template access is checked at compile time
- No runtime metadata errors possible

### Performance

The metadata system is designed for efficiency:
- Zero-cost abstractions
- No heap allocations in the hot path
- Thread-local storage for fast access
- Automatic cleanup
- No locking or synchronization needed 