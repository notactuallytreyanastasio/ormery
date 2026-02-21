import { defineConfig } from 'vite';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  root: 'tutorial',

  resolve: {
    alias: {
      '@temperlang/core': path.resolve(__dirname, 'temper.out/js/temper-core'),
      '@temperlang/std': path.resolve(__dirname, 'temper.out/js/std'),
    },
  },

  server: {
    port: 8000,
  },

  build: {
    outDir: path.resolve(__dirname, 'dist'),
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: path.resolve(__dirname, 'tutorial/index.html'),
        interactive: path.resolve(__dirname, 'tutorial/interactive.html'),
        demo: path.resolve(__dirname, 'tutorial/demo.html'),
      },
    },
  },
});
