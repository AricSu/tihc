import { createApp } from 'vue'
import { setupDirectives } from '../../directives'
import { setupRouter } from '../../router'

import { setupStore } from '../../store'
import { setupNaiveDiscreteApi } from '../../utils'
import App from './App.vue'
import '@/styles/reset.css'
import '@/styles/global.css'
import 'uno.css'

async function bootstrap() {
  const app = createApp(App)
  setupStore(app)
  setupDirectives(app)
  await setupRouter(app)
  app.mount('#app')
  setupNaiveDiscreteApi()

  // Try to register the vue-advanced-chat web component if available.
  // The ChatHome view uses <vue-advanced-chat />, which expects the
  // component to be globally registered via its `register()` helper.
  try {
    const mod = await import('vue-advanced-chat')
    if (mod && typeof mod.register === 'function') {
      try {
        mod.register()
        console.warn('[sidepanel] vue-advanced-chat registered')
      }
      catch (e) {
        console.warn('[sidepanel] vue-advanced-chat.register() threw', e)
      }
    }
  }
  catch (e) {
    // optional dependency; fail silently in non-chat builds
    console.warn('[sidepanel] vue-advanced-chat import failed (optional)', e)
  }
}

bootstrap()
