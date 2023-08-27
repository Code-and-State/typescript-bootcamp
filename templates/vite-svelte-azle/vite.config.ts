/// <reference types="vitest" />
import { defineConfig } from 'vite';
import { svelte, vitePreprocess } from "@sveltejs/vite-plugin-svelte"
import environment from 'vite-plugin-environment';
import dotenv from 'dotenv';

dotenv.config();
export default defineConfig({
  root: 'src',
  build: {
    outDir: '../dist',
    emptyOutDir: true,
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: 'globalThis',
      },
    },
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:4943',
        changeOrigin: true,
      },
    },
  },
  plugins: [
    svelte({
      preprocess: vitePreprocess(),
    }),
    environment('all'),
  ],
});
