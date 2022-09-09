import progress from 'vite-plugin-progress';
import { VitePWA } from 'vite-plugin-pwa';
import { ViteWebfontDownload } from 'vite-plugin-webfont-dl';
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

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
      includeAssets: ['favicon.ico','apple-touch-icon.png','masked-icon.svg'],
      manifest: {
        name: 'UpVent Technologies',
        short_name: 'UpVent',
        description: 'UpVent es un proveedor experto de soluciones de código libre y empresarial para los pequeños y medianos negocios Mexicanos.',
        theme_color: '#007FBC',
        icons: [
          {
            src: 'pwa-192x192.png',
            sizes: '192x192',
            type: 'image/png'
          },
          {
            src: 'pwa-512x512.png',
            sizes: '512x512',
            type: 'image/png'
          }
        ]
      }
    })
  ]
})
