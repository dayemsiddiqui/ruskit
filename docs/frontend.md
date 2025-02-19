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

1. First, define your DTO in Rust with the `auto_ts_export` macro:

```rust
use serde::Serialize;
use ts_export_derive::auto_ts_export;

#[auto_ts_export]
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

Ruskit provides CLI commands to quickly scaffold Inertia pages and props:

```bash
# Create a complete Inertia page (props, controller, and React component)
cargo kit inertia:page Dashboard

# Create just the props type for an existing page
cargo kit inertia:prop Settings
```

The `inertia:page` command will:
1. Create a props type in `src/app/dtos/dashboard.rs` with TypeScript export
2. Create a controller in `src/app/controllers/dashboard_controller.rs`
3. Create a React component in `resources/js/pages/Dashboard.tsx`
4. Set up all necessary imports and exports

The `inertia:prop` command will only create the props type file, which is useful when:
- You want to add props to an existing page
- You're creating a shared props type used by multiple components
- You want to define the contract first before implementing the UI

Here's what each generated file looks like:

1. Props Type (`src/app/dtos/dashboard.rs`):
```rust
use serde::Serialize;
use ts_export_derive::auto_ts_export;

#[auto_ts_export]
pub struct DashboardProps {
    pub title: String,
    // TODO: Add your page props here
}
```

2. Controller (`src/app/controllers/dashboard_controller.rs`):
```rust
use axum::response::IntoResponse;
use axum_inertia::Inertia;
use crate::app::dtos::dashboard::DashboardProps;

pub struct DashboardController;

impl DashboardController {
    pub async fn show(inertia: Inertia) -> impl IntoResponse {
        inertia.render("Dashboard", DashboardProps {
            title: String::from("Dashboard"),
        })
    }
}
```

3. React Component (`resources/js/pages/Dashboard.tsx`):
```tsx
import React from 'react';
import { Head } from '@inertiajs/react';
import type { DashboardProps } from '../types/generated';

interface Props extends DashboardProps {}

export default function Dashboard({ title }: Props) {
    return (
        <>
            <Head title={title} />
            <div className="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
                <div className="text-center">
                    <h1 className="text-4xl font-bold text-gray-900 sm:text-5xl">
                        {title}
                    </h1>
                    {/* Add your page content here */}
                </div>
            </div>
        </>
    );
}
```

After generating the files, you'll need to:
1. Add your route in `src/web.rs`:
   ```rust
   .route("/dashboard", get(DashboardController::show))
   ```
2. Add your props in the DTO file
3. Customize the React component with your UI

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