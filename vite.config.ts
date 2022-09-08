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
    svelte({
      experimental: {
        useVitePreprocess: true,
        prebundleSvelteLibraries: true
      }
    }),
    ViteWebfontDownload(),
    progress({
      format: 'Building UpVent - Tusk 3.8.2 [:bar] :percent'
    }),
    VitePWA({
      registerType: 'autoUpdate',
      devOptions: {
        enabled: true
      },
      includeAssets: ['favicon.png'],
      manifest: {
        name: 'UpVent Technologies',
        short_name: 'UpVent',
        description: 'UpVent es un proveedor experto de soluciones de código libre y empresarial para los pequeños y medianos negocios Mexicanos.',
        theme_color: '#007FBC'
      }
    })
  ]
})
