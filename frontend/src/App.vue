<template>
  <n-config-provider
    class="wh-full"
    :locale="zhCN"
    :date-locale="dateZhCN"
    :theme="appStore.isDark ? darkTheme : undefined"
    :theme-overrides="appStore.naiveThemeOverrides"
    :hljs="$hljs"
  >
    <router-view v-if="Layout" v-slot="{ Component, route: curRoute }">
      <component :is="Layout">
        <transition name="fade-slide" mode="out-in" appear>
          <KeepAlive :include="keepAliveNames">
            <component :is="Component" v-if="!tabStore.reloading" :key="curRoute.fullPath" />
          </KeepAlive>
        </transition>
      </component>

      <LayoutSetting v-if="layoutSettingVisible" class="fixed right-12 top-1/2 z-999" />
    </router-view>
  </n-config-provider>
</template>

<script setup>
import { darkTheme, dateZhCN, zhCN } from 'naive-ui'
import { getCurrentInstance } from 'vue'
import i18n from './i18n'
import { LayoutSetting } from '@/components'
import { useAppStore, useTabStore } from '@/store'
import { layoutSettingVisible } from './settings'



// 全局挂载 i18n
const app = getCurrentInstance()?.appContext.app
if (app && !app.$i18n) {
  app.use(i18n)
}

const layouts = new Map()
function getLayout(name) {
  if (layouts.get(name))
    return layouts.get(name)
  const layout = markRaw(defineAsyncComponent(() => import(`@/layouts/${name}/index.vue`)))
  layouts.set(name, layout)
  return layout
}

const route = useRoute()
const appStore = useAppStore()
if (appStore.layout === 'default')
  appStore.setLayout('')
const Layout = computed(() => {
  if (!route.matched?.length)
    return null
  return getLayout(route.meta?.layout || appStore.layout)
})

const tabStore = useTabStore()
const keepAliveNames = computed(() => {
  return tabStore.tabs.filter(item => item.keepAlive).map(item => item.name)
})

watchEffect(() => {
  appStore.setThemeColor(appStore.primaryColor, appStore.isDark)
})
</script>
