<template>
  <n-menu
    ref="menu"
    class="side-menu"
    accordion
    :indent="18"
    :collapsed-icon-size="22"
    :collapsed-width="64"
    :collapsed="appStore.collapsed"
    :options="permissionStore.menus"
    :value="activeKey"
    @update:value="handleMenuSelect"
  />
</template>

<script setup>
import { useAppStore, usePermissionStore } from '@/store'
import { isExternal } from '@/utils'

const router = useRouter()
const route = useRoute()
const appStore = useAppStore()
const permissionStore = usePermissionStore()

const activeKey = computed(() => route.meta?.parentKey || route.name)

const menu = ref(null)
watch(route, async () => {
  await nextTick()
  menu.value?.showOption()
})

function handleMenuSelect(key, item) {
  const shouldAutoCollapse = appStore.layout === 'extension'
  if (isExternal(item.originPath)) {
    $dialog.confirm({
      type: 'info',
      title: `请选择打开方式`,
      positiveText: '外链打开',
      negativeText: '在本站内嵌打开',
      confirm() {
        window.open(item.originPath)
        if (shouldAutoCollapse)
          appStore.collapsed = true
      },
      cancel: () => {
        router.push(item.path)
        if (shouldAutoCollapse)
          appStore.collapsed = true
      },
    })
  }
  else {
    if (!item.path)
      return
    router.push(item.path)
    if (shouldAutoCollapse)
      appStore.collapsed = true
  }
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
