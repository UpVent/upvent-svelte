import progress from 'vite-plugin-progress';
import { ViteWebfontDownload } from 'vite-plugin-webfont-dl';
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import { VitePWA } from 'vite-plugin-pwa';

// https://vitejs.dev/config/
export default defineConfig({
  optimizeDeps: {
    include: ['lodash.get', 'lodash.isequal', 'lodash.clonedeep'],
    exclude: ['tinro']
  },
  plugins: [
    svelte(),
    ViteWebfontDownload(),
    progress({
      format: 'Building UpVent - Tusk 3.3.0 [:bar] :percent'
    }),
    VitePWA({
      registerType: 'autoUpdate',
      devOptions: {
        enabled: true
      }
    })
  ]
})
