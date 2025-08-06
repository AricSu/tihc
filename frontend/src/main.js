// 禁用 Monaco Editor 的 Web Workers，避免加载问题
// 这会让 Monaco Editor 在主线程中运行，适合嵌入式部署
window.MonacoEnvironment = {
  getWorker: function (moduleId, label) {
    return new Worker(
      URL.createObjectURL(
        new Blob(['self.MonacoEnvironment = { baseUrl: "/" }; self.module = undefined; self.process = undefined; self.Buffer = undefined;'], {
          type: 'application/javascript',
        })
      )
    )
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
