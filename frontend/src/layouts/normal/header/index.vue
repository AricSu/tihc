<template>
  <AppCard class="flex items-center px-12" border-b="1px solid light_border dark:dark_border">
    <MenuCollapse />
    <AppTab class="w-0 flex-1 px-12" />
    <span class="mx-6 opacity-20">|</span>
    <div class="flex items-center px-12 text-18">
      <BeginnerGuide />
      <ToggleTheme />
      <Fullscreen />
      <i class="i-fe:github mr-16 cursor-pointer" @click="open('https://github.com/aricSu/tihc')" />
      <i class="i-fe:github mr-16 cursor-pointer" @click="open('https://github.com/AricSu/aricsu.github.io/discussions/categories/askaric-tihc')" />
    </div>
  </AppCard>
</template>

<script setup>
import { ToggleTheme } from '@/components'
import { AppTab, BeginnerGuide, Fullscreen, MenuCollapse } from '@/layouts/components'

const open = (url) => window.open(url)
</script>
