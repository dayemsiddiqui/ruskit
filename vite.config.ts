import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
    plugins: [
        react({
            jsxRuntime: 'automatic'
        })
    ],
    root: '.',
    base: '/',
    server: {
        port: 3000,
        hmr: {
            protocol: 'ws',
            host: 'localhost'
        },
        fs: {
            strict: false,
            allow: ['.']
        }
    },
    build: {
        outDir: 'public/dist',
        manifest: true,
        rollupOptions: {
            input: 'resources/js/app.tsx'
        }
    },
    optimizeDeps: {
        include: ['@inertiajs/react']
    }
}); 