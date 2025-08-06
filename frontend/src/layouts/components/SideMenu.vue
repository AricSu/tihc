<template>
  <n-menu
    ref="menu"
    class="side-menu"
    accordion
    :indent="18"
    :collapsed-icon-size="22"
    :collapsed-width="64"
    :collapsed="appStore.collapsed"
    :options="staticMenus"
    :value="activeKey"
    @update:value="handleMenuSelect"
  />
</template>

<script setup>
import { useAppStore } from '@/store'
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'

const appStore = useAppStore()
const { t } = useI18n()

const staticMenus = computed(() => [
  {
    label: t('menu.home'),
    key: 'home',
    path: '/home',
    icon: () => h('i', { class: 'i-mdi-home text-16' }),
  },
  {
    label: t('menu.sqlEditor'),
    key: 'sql-editor',
    path: '/sql-editor',
    icon: () => h('i', { class: 'i-mdi-database-search text-16' }),
  },
  // {
  //   label: t('menu.ddlCheck'),
  //   key: 'ddl',
  //   path: '/ddl',
  //   icon: () => h('i', { class: 'i-mdi-table-edit text-16' }),
  // },
])

// 根据当前 hash 路由自动设置高亮菜单 key
const activeKey = ref(getActiveKeyByHash())

function getActiveKeyByHash() {
  const hash = window.location.hash.replace(/^#/, '')
  const found = staticMenus.value.find(m => m.path === hash)
  return found ? found.key : staticMenus.value[0].key
}

window.addEventListener('hashchange', () => {
  activeKey.value = getActiveKeyByHash()
})

function handleMenuSelect(key, item) {
  if (!item.path) return
  window.location.hash = '#' + item.path
}
</script>

<style>
.side-menu:not(.n-menu--collapsed) {
  .n-menu-item-content {
    &::before {
      left: 8px;
      right: 8px;
    }
    &.n-menu-item-content--selected::before {
      border-left: 4px solid rgb(var(--primary-color));
    }
  }
}
</style>
