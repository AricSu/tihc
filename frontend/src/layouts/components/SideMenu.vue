<!--------------------------------
 - @Author: Ronnie Zhang
 - @LastEditor: Ronnie Zhang
 - @LastEditTime: 2023/12/16 18:50:35
 - @Email: zclzone@outlook.com
 - Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 --------------------------------->

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

const appStore = useAppStore()
const staticMenus = [
  {
    label: 'SQL 编辑器',
    key: 'sql-editor',
    path: '/sql-editor',
    icon: () => h('i', { class: 'i-mdi-database-search text-16' }),
  },
  {
    label: '慢日志分析',
    key: 'slowlog',
    path: '/slowlog',
    icon: () => h('i', { class: 'i-mdi-timer-sand text-16' }),
  },
  {
    label: 'DDL 检查',
    key: 'ddl',
    path: '/ddl',
    icon: () => h('i', { class: 'i-mdi-table-edit text-16' }),
  },
]

const activeKey = ref('sql-editor')

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
