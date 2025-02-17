# Views and Templating in Rustavel

Rustavel uses [Askama](https://github.com/djc/askama) as its templating engine, providing compile-time template checking and high performance. The syntax is similar to Jinja2/Django templates, making it familiar for developers coming from other frameworks.

## Basic Template Usage

### Template Structure

Templates in Rustavel are stored in the `templates` directory at your project root. Each template is a `.html` file that can extend other templates.

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

Rustavel encourages template inheritance through a base layout:

```html
<!-- templates/base.html -->
<!DOCTYPE html>
<html>
<head>
    <title>{% block title %}Rustavel{% endblock %}</title>
</head>
<body>
    {% block content %}{% endblock %}
</body>
</html>

<!-- templates/home.html -->
{% extends "base.html" %}

{% block title %}Welcome | Rustavel{% endblock %}

{% block content %}
    <h1>Welcome to Rustavel!</h1>
{% endblock %}
```

## Tailwind CSS Integration

Rustavel comes with out-of-the-box support for Tailwind CSS, providing a modern utility-first CSS framework for your templates.

### Default Setup

Every Rustavel template automatically includes the latest version of Tailwind CSS via CDN:

```html
<script src="https://cdn.tailwindcss.com"></script>
<script>
    window.tailwind.config = {
        theme: {
            extend: {
                colors: {
                    'rustavel': '#B7410E',
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
        'rustavel': '#B7410E',
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

Here's how to use Tailwind CSS in your Rustavel templates:

```html
{% extends "base.html" %}

{% block content %}
<div class="min-h-screen bg-gray-100">
    <nav class="bg-white shadow-lg">
        <div class="max-w-6xl mx-auto px-4">
            <div class="flex justify-between">
                <div class="flex space-x-7">
                    <a href="/" class="flex items-center py-4">
                        <span class="font-semibold text-rustavel text-lg">
                            Rustavel
                        </span>
                    </a>
                </div>
                <div class="flex items-center space-x-3">
                    <a href="/login" class="py-2 px-4 bg-rustavel text-white rounded hover:bg-opacity-90">
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