# Frontend Development with Ruskit

Ruskit provides a modern frontend development experience using React, TypeScript, and Tailwind CSS. This guide will help you understand how to use these technologies effectively in your Ruskit application.

## Table of Contents
- [React with Inertia.js](#react-with-inertiajs)
- [TypeScript Support](#typescript-support)
- [Tailwind CSS](#tailwind-css)
- [Development Workflow](#development-workflow)

## React with Inertia.js

Ruskit uses [Inertia.js](https://inertiajs.com/) to connect your React frontend with your Rust backend. This allows you to build single-page applications without building an API.

### Creating Pages

Pages in Ruskit are React components stored in the `resources/js/pages` directory. Here's an example of the About page:

```tsx
import React from 'react';
import { Head } from '@inertiajs/react';
import { AboutPageProps } from '../types';

export default function About({ title, description }: AboutPageProps) {
  return (
    <>
      <Head title={title} />
      <div className="p-4">
        <h1 className="text-2xl font-bold">{title}</h1>
        <p className="mt-2 text-gray-600">{description}</p>
      </div>
    </>
  );
}
```

### Routing and Controllers

Routes are defined in your Rust backend (`src/web.rs`) and connected to controller methods:

```rust
let inertia_router = Router::new()
    .route("/", get(landing))
    .route("/about", get(InertiaController::about))
    .with_state(inertia_config);
```

The controller methods use Inertia to render pages with data:

```rust
use axum::response::IntoResponse;
use axum_inertia::Inertia;
use serde_json::json;

pub struct InertiaController;

impl InertiaController {
    pub async fn about(inertia: Inertia) -> impl IntoResponse {
        inertia.render(
            "About",
            json!({
                "title": "About",
                "description": "Learn more about Ruskit"
            })
        )
    }
}
```

### Type Safety

The TypeScript type system ensures that the data passed from your Rust controllers matches what your React components expect:

```typescript
// resources/js/types/index.ts
export interface AboutProps {
    title: string;
    description: string;
}

export type PageProps<T = {}> = T & SharedProps;
export type AboutPageProps = PageProps<AboutProps>;
```

This creates a type-safe bridge between your Rust backend and React frontend:
1. The Rust controller sends data using `serde_json::json!`
2. The TypeScript types define the expected shape of that data
3. Your React component receives properly typed props

## TypeScript Support

Ruskit includes full TypeScript support out of the box.

### Configuration

The TypeScript configuration is defined in `tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "jsx": "react-jsx",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true
  },
  "include": ["resources/js/**/*"]
}
```

### Type Definitions

For better type safety, define interfaces for your page props:

```tsx
// resources/js/types/index.ts
export interface User {
  id: number;
  name: string;
  email: string;
}

export interface PageProps {
  auth: {
    user: User | null;
  };
}
```

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