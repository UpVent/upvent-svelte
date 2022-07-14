import progress from 'vite-plugin-progress';
import { ViteWebfontDownload } from 'vite-plugin-webfont-dl';
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// https://vitejs.dev/config/
export default defineConfig({
  optimizeDeps: {
    include: ['lodash.get', 'lodash.isequal', 'lodash.clonedeep']
  },
  plugins: [
    svelte(),
    ViteWebfontDownload(),
    progress({
      format: 'Building UpVent - Tusk 3.2.1 [:bar] :percent'
    })
  ]
})
