# Frontend Development with Ruskit

Ruskit provides a modern frontend development experience using React, TypeScript, and Tailwind CSS, with full type safety between your Rust backend and React frontend.

## Table of Contents
- [React with Inertia.js](#react-with-inertiajs)
- [Type-Safe Props](#type-safe-props)
- [Creating Pages](#creating-pages)
- [Tailwind CSS](#tailwind-css)
- [Development Workflow](#development-workflow)

## React with Inertia.js

Ruskit uses [Inertia.js](https://inertiajs.com/) to connect your React frontend with your Rust backend, providing a seamless single-page application experience without building an API.

### Type-Safe Props

Ruskit automatically generates TypeScript types from your Rust DTOs using `ts-rs`. This ensures complete type safety between your backend and frontend.

1. First, define your DTO in Rust with `Serialize` and `TS` derives:

```rust
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export)]
pub struct AboutPageProps {
    pub title: String,
    pub description: String,
    pub tech_stack: Vec<String>,
    pub why_choose_us: Vec<String>,
}
```

2. The types are automatically generated in `resources/js/types/generated.ts`:

```typescript
export interface AboutPageProps {
    title: string;
    description: string;
    tech_stack: string[];
    why_choose_us: string[];
}
```

### Creating Pages

Pages in Ruskit are React components stored in the `resources/js/pages` directory. Here's a complete example of creating a type-safe page:

1. Create your controller method (`src/app/controllers/inertia_controller.rs`):

```rust
use axum::response::IntoResponse;
use axum_inertia::Inertia;
use crate::app::dtos::about::AboutPageProps;

impl InertiaController {
    pub async fn about(inertia: Inertia) -> impl IntoResponse {
        inertia.render("About", AboutPageProps {
            title: String::from("About"),
            description: String::from("Learn more about Ruskit"),
            tech_stack: vec![
                String::from("Rust"),
                String::from("React"),
                String::from("TypeScript"),
                String::from("Tailwind CSS")
            ],
            why_choose_us: vec![
                String::from("Performance"),
                String::from("Reliability"),
                String::from("Scalability"),
                String::from("Ease of Use")
            ],
        })
    }
}
```

2. Create your React page (`resources/js/pages/About.tsx`):

```tsx
import React from 'react';
import { Head } from '@inertiajs/react';
import { AboutPageProps } from '../types/generated';

interface Props {
    title: string;
    description: string;
    tech_stack: string[];
    why_choose_us: string[];
}

export default function About({ 
    title, 
    description, 
    tech_stack,
    why_choose_us 
}: AboutPageProps) {
    return (
        <>
            <Head title={title} />
            <div className="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                <div className="text-center">
                    <h1 className="text-4xl font-bold text-gray-900 sm:text-5xl">
                        {title}
                    </h1>
                    <p className="mt-4 text-xl text-gray-500">
                        {description}
                    </p>
                </div>

                <div className="mt-16">
                    <h2 className="text-2xl font-bold text-gray-900">
                        Our Tech Stack
                    </h2>
                    <ul className="mt-4 grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4">
                        {tech_stack.map((tech, index) => (
                            <li 
                                key={index}
                                className="bg-white p-4 rounded-lg shadow"
                            >
                                {tech}
                            </li>
                        ))}
                    </ul>
                </div>

                <div className="mt-16">
                    <h2 className="text-2xl font-bold text-gray-900">
                        Why Choose Us
                    </h2>
                    <ul className="mt-4 grid grid-cols-1 gap-4 sm:grid-cols-2">
                        {why_choose_us.map((reason, index) => (
                            <li 
                                key={index}
                                className="bg-white p-6 rounded-lg shadow"
                            >
                                {reason}
                            </li>
                        ))}
                    </ul>
                </div>
            </div>
        </>
    );
}
```

3. Add the route (`src/web.rs`):

```rust
let inertia_router = Router::new()
    .route("/about", get(InertiaController::about))
    .with_state(inertia_config);
```

### Type Safety Benefits

- **Compile-time Type Checking**: TypeScript will catch any mismatches between your Rust DTOs and React components
- **Automatic Type Generation**: No need to manually maintain TypeScript interfaces
- **IDE Support**: Get full autocomplete and type hints in your editor
- **Refactoring Safety**: Renaming properties in your Rust DTOs will cause TypeScript errors if not updated in React

## Tailwind CSS

Ruskit uses Tailwind CSS for styling, providing a utility-first CSS framework.

### Configuration

Tailwind is configured in `tailwind.config.js`:

```js
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./resources/js/**/*.{js,ts,jsx,tsx}",
    "./templates/**/*.{html,js}"
  ],
  theme: {
    extend: {
      // Add your custom theme extensions here
    },
  },
  plugins: [],
}
```

### Using Tailwind

You can use Tailwind's utility classes directly in your components:

```tsx
export default function Button({ children }) {
  return (
    <button className="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors">
      {children}
    </button>
  );
}
```

### Custom Styles

For custom styles, you can use the `@layer` directive in `resources/css/app.css`:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer components {
  .btn-primary {
    @apply px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors;
  }
}
```

## Development Workflow

### Starting the Development Server

Run both the Rust backend and Vite development server:

```bash
npm run dev
```

This command uses `concurrently` to run both servers simultaneously.

### Building for Production

To build your application for production:

```bash
npm run build
```

This will:
1. Compile your TypeScript code
2. Process your Tailwind CSS
3. Bundle all assets with Vite
4. Generate a manifest file for asset versioning

### Hot Module Replacement (HMR)

Ruskit supports HMR out of the box. Your components will automatically update in the browser when you make changes to your code.

### Type Checking

TypeScript errors will be shown in your editor and during the build process. Make sure to fix type errors before deploying to production.

## Best Practices

1. **Component Organization**
   - Keep components in `resources/js/components`
   - Use TypeScript interfaces for prop types
   - Implement proper error boundaries

2. **Styling**
   - Use Tailwind's utility classes for most styling needs
   - Create custom components for reusable UI elements
   - Use `@apply` for complex, repeated patterns

3. **State Management**
   - Use Inertia.js for server-side state
   - React hooks for local state
   - Consider Zustand or Jotai for complex client-side state

4. **Performance**
   - Implement code splitting using dynamic imports
   - Use React.memo for expensive computations
   - Optimize images and assets

## Troubleshooting

### Common Issues

1. **Styles not loading**
   - Make sure `app.css` is imported in `app.tsx`
   - Check if Tailwind classes are purged correctly
   - Clear your browser cache

2. **TypeScript errors**
   - Run `tsc --noEmit` to check for type errors
   - Make sure all dependencies have type definitions
   - Check your `tsconfig.json` configuration

3. **HMR not working**
   - Check if the Vite dev server is running
   - Verify WebSocket connection in browser console
   - Clear the `.vite` cache directory 