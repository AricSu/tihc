<template>
  <router-link
    class="h-60 f-c-c cursor-pointer"
    to="/"
    @click.native="refreshIfLogin"
  >
    <TheLogo />
    <h2
      v-show="!appStore.collapsed"
      class="ml-10 max-w-140 flex-shrink-0 text-16 color-primary font-bold"
    >
      {{ title }}
    </h2>
  </router-link>
</template>

<script setup>
import { useAppStore } from '@/store'

const title = import.meta.env.VITE_TITLE
const appStore = useAppStore()
import { useRouter } from 'vue-router'
const router = useRouter()
function refreshIfLogin(e) {
  if (router.currentRoute.value.path === '/') {
    e.preventDefault()
    window.location.reload()
  }
}
</script>
