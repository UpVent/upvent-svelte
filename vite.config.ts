import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';
import progress from 'vite-plugin-progress';

// https://vitejs.dev/config/
export default defineConfig({
  optimizeDeps: {
    include: ['lodash.get', 'lodash.isequal', 'lodash.clonedeep']
  },
  plugins: [
    svelte(),
    progress({
      format: 'Building UpVent - Tusk 3.0.1 [:bar] :percent'
    })
  ]
})
