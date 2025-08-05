import { generate, getRgbStr } from '@arco-design/color'
import { useDark } from '@vueuse/core'
import { defineStore } from 'pinia'
import { defaultLayout, defaultPrimaryColor } from '@/settings'

export const useAppStore = defineStore('app', {
  state: () => ({
    collapsed: false,
    isDark: useDark(),
    layout: defaultLayout,
    primaryColor: defaultPrimaryColor,
    naiveThemeOverrides: {
      common: {
        primaryColor: defaultPrimaryColor,
        primaryColorHover: '#316C72E3',
        primaryColorPressed: '#2B4C59FF',
        primaryColorSuppl: '#316C72E3',
      },
    },
  }),
  actions: {
    switchCollapsed() {
      this.collapsed = !this.collapsed
    },
    setCollapsed(b) {
      this.collapsed = b
    },
    toggleDark() {
      this.isDark = !this.isDark
    },
    setLayout(v) {
      this.layout = v
    },
    setPrimaryColor(color) {
      this.primaryColor = color
      // 同步主题色
      this.naiveThemeOverrides.common.primaryColor = color
    },
    setThemeColor(color, isDark) {
      this.primaryColor = color
      this.isDark = isDark
      // 同步 Naive UI 主题色
      this.naiveThemeOverrides.common.primaryColor = color
    },
    // ...existing code...
  },
  persist: {
    pick: ['collapsed', 'layout', 'primaryColor'],
    storage: sessionStorage,
  },
})
