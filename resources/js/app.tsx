import React from 'react';
import { createRoot } from 'react-dom/client';
import { createInertiaApp } from '@inertiajs/react';
import { Page } from '@inertiajs/core';
import '../css/app.css';

createInertiaApp({
  resolve: async (name: string) => {
    const pages = import.meta.glob<any>('./pages/**/*.tsx');
    const page = await pages[`./pages/${name}.tsx`]();
    return page.default;
  },
  setup({ el, App, props }) {
    createRoot(el).render(
      <React.StrictMode>
        <App {...props} />
      </React.StrictMode>
    );
  },
}); 