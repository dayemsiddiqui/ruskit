/// <reference types="vite/client" />

interface ImportMetaEnv {
    readonly VITE_APP_TITLE: string
}

interface ImportMeta {
    readonly env: ImportMetaEnv
    glob(pattern: string): Record<string, () => Promise<any>>
    glob<T>(pattern: string): Record<string, () => Promise<T>>
} 