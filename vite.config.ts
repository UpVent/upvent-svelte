import { VitePWA } from 'vite-plugin-pwa';
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

// https://vitejs.dev/config/
export default defineConfig({
  optimizeDeps: {
    exclude: ['tinro']
  },
  plugins: [
    svelte({
      prebundleSvelteLibraries: true,
    }),
    VitePWA({
      registerType: 'autoUpdate',
      devOptions: {
        enabled: true
      },
      includeAssets: ['favicon.ico', 'apple-touch-icon.png', 'masked-icon.svg'],
      manifest: {
        name: 'UpVent Technologies',
        short_name: 'UpVent',
        description: 'UpVent es un proveedor experto de soluciones de código libre y empresarial para los pequeños y medianos negocios Mexicanos.',
        theme_color: '#007FBC',
        orientation: 'any',
        lang: 'es',
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
          },
          {
            src: 'pwa-512x512.png',
            sizes: '512x512',
            type: 'image/png',
            purpose: 'any maskable'
          }
        ]
      }
    })
  ]
})
