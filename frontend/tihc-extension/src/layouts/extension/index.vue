<template>
  <div class="wh-full flex">
    <article class="w-0 flex-col flex-1">
      <slot />
    </article>
    <aside
      class="sidebar-ext flex-col flex-shrink-0 transition-width-300"
      :class="appStore.collapsed ? 'w-64' : 'w-220'"
      border-r="1px solid light_border dark:dark_border"
    >
      <SideBar />
    </aside>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import { useAppStore } from '@/store'
import SideBar from './sidebar/index.vue'

const appStore = useAppStore()
const isMobile = ref(false)
onMounted(() => {
  const check = () => {
    isMobile.value = window.innerWidth <= 768
  }
  check()
  window.addEventListener('resize', check)
})
</script>

<style>
.collapsed {
  width: 64px;
}
.sidebar-ext {
  z-index: auto;
}
@media (max-width: 768px) {
  .sidebar-ext.w-220 {
    position: fixed !important;
    right: 0;
    top: 0;
    width: 100vw !important;
    height: 100vh !important;
    max-width: none;
    min-width: 0;
    border-radius: 0;
    background: var(--body-bg, #fff);
    box-shadow: none;
    z-index: 2000;
    left: auto !important;
  }
  .sidebar-ext.w-64 {
    z-index: auto !important;
  }
  .sidebar-ext.w-64 {
    position: static !important;
    width: 64px !important;
    height: auto !important;
  }
}
</style>
