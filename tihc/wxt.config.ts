import { defineConfig } from 'wxt';
import tailwindcss from '@tailwindcss/vite'

// See https://wxt.dev/api/config.html
export default defineConfig({
  manifest: {
    action: {
      default_title: 'TIHC',
    },
    permissions: ['sidePanel', 'identity'],
    host_permissions: ['http://localhost/*', 'https://*/*'],
    side_panel: {
      default_page: 'entrypoints/sidepanel/index.html',
    },
  },
  modules: ['@wxt-dev/module-react'],
  srcDir: 'src',
  dev: {
    server: {
      port: 3002,
    }
  },
  vite: () => ({
    plugins: [tailwindcss()],
  }),
});
