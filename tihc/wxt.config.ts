import { defineConfig } from 'wxt';
import tailwindcss from '@tailwindcss/vite'

const googleChromeClientId =
  process.env.WXT_GOOGLE_OAUTH_CHROME_CLIENT_ID ??
  process.env.WXT_GOOGLE_OAUTH_CLIENT_ID;
const googleScopes = ['openid', 'email', 'profile'];

// See https://wxt.dev/api/config.html
export default defineConfig({
  manifest: {
    action: {
      default_title: 'TIHC',
    },
    options_ui: {
      page: 'entrypoints/options/index.html',
      open_in_tab: true,
    },
    permissions: ['identity', 'sidePanel', 'storage', 'tabs'],
    host_permissions: ['http://*/*', 'https://*/*'],
    oauth2: googleChromeClientId
      ? {
          client_id: googleChromeClientId,
          scopes: googleScopes,
        }
      : undefined,
    browser_specific_settings: {
      gecko: {
        id: process.env.WXT_FIREFOX_EXTENSION_ID ?? 'tihc@example.com',
        strict_min_version: '121.0',
      },
    },
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
