// 修复 Monaco Editor Web Worker 警告（本地静态资源方案）
if (typeof window !== 'undefined') {
  window.MonacoEnvironment = {
    getWorkerUrl: function (moduleId, label) {
      return `data:text/javascript;charset=utf-8,${encodeURIComponent(`
        self.MonacoEnvironment = { baseUrl: location.origin };
        importScripts('${location.origin}/monaco-editor/min/vs/base/worker/workerMain.js');
      `)}`;
    }
  }
}
import { createApp } from 'vue'
import App from './App.vue'

import { setupRouter } from './router'
import { setupStore } from './store'
import { setupNaiveDiscreteApi } from './utils'
import hljs from './hljs-setup'
import '@/styles/reset.css'
import '@/styles/global.css'
import 'uno.css'

async function bootstrap() {
  const app = createApp(App)
  app.config.globalProperties.$hljs = hljs
  setupStore(app)
  await setupRouter(app)
  app.mount('#app')
  setupNaiveDiscreteApi()
}

bootstrap()
