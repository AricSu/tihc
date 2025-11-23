import Unocss from 'unocss/vite'
import AutoImport from 'unplugin-auto-import/vite'
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers'
import Components from 'unplugin-vue-components/vite'
import { defineConfig } from 'wxt'
// @ts-expect-error: Could not find a declaration file for module './build/plugin-isme'.
import { pluginIcons, pluginPagePathes } from './build/plugin-isme'

// See https://wxt.dev/api/config.html
export default defineConfig({
  vite: () => ({
    plugins: [
      // Add Vite plugins here
      Unocss(),
      AutoImport({
        imports: ['vue', 'vue-router'],
        dts: false,
      }),
      Components({
        resolvers: [NaiveUiResolver()],
        dts: false,
      }),
      // 自定义插件，用于生成页面文件的path，并添加到虚拟模块
      pluginPagePathes(),
      // 自定义插件，用于生成自定义icon，并添加到虚拟模块
      pluginIcons(),
    ],
    vue: {
      template: {
        compilerOptions: {
          isCustomElement: (tag: string) => tag === 'vue-advanced-chat',
        },
      },
    },
  }),
  modules: ['@wxt-dev/module-vue', '@wxt-dev/auto-icons'],
  srcDir: 'src',
  publicDir: 'public',
  manifest: {
    action: {
      default_title: 'TIHC Jira Extension',
    },
    permissions: ['sidePanel', 'storage', 'identity'],
    host_permissions: [
      'http://127.0.0.1:8080/*',
      'http://localhost:8080/*',
      'https://127.0.0.1:8080/*',
      'https://localhost:8080/*',
      'https://github.com/*',
      'https://api.github.com/*',
    ],
    side_panel: {
      default_page: 'entrypoints/sidepanel/index.html',
    },
  },
})
