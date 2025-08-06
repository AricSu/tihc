<template>
  <AppCard class="flex items-center px-12" border-b="1px solid light_border dark:dark_border">
    <MenuCollapse />
    <!-- <AppTab class="w-0 flex-1 px-12" /> -->
    <div class="flex-1"></div>
    <span class="mx-6 opacity-20">|</span>
    <div class="flex items-center px-12 text-18">
      <Icon icon="mdi:file-document-outline" class="mr-16 cursor-pointer" @click="open(locale === 'zh' ? 'https://www.askaric.com/zh/tihc/' : 'https://www.askaric.com/en/tihc/')" />
      <!-- <BeginnerGuide /> -->
      <!-- <ToggleTheme /> -->
      <Fullscreen />
      <Icon icon="mdi:github" class="mr-16 cursor-pointer" @click="open('https://github.com/aricSu/tihc')" />
      <Icon icon="mdi:account-group-outline" class="mr-16 cursor-pointer" @click="open('https://github.com/AricSu/aricsu.github.io/discussions/categories/askaric-tihc')" />
      <button class="ml-12 i18n-icon-btn" @click="showLangDrawer = true">
        <Icon icon="mdi:earth" class="text-20" />
      </button>
    </div>
    <n-drawer v-model:show="showLangDrawer" placement="right" width="220" :mask="true">
      <n-drawer-content :title="t('common.language')">
        <div class="flex flex-col gap-4 py-8 px-4">
          <button
            v-for="lang in langs"
            :key="lang"
            :class="['i18n-drawer-btn', { active: locale === lang }]"
            @click="switchLang(lang)"
          >
            <Icon :icon="lang === 'zh' ? 'twemoji:flag-china' : 'twemoji:flag-united-states'" class="mr-2 text-20" />
            {{ lang === 'zh' ? '中文简体' : 'English' }}
          </button>
        </div>
      </n-drawer-content>
    </n-drawer>
  </AppCard>
</template>

<script setup>
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Icon } from '@iconify/vue'
import { ToggleTheme } from '@/components'
import { AppTab, BeginnerGuide, Fullscreen, MenuCollapse } from '@/layouts/components'
const open = (url) => window.open(url)
const { locale, t } = useI18n()
const langs = ['zh', 'en']
const showLangDrawer = ref(false)
function switchLang(lang) {
  locale.value = lang
  showLangDrawer.value = false
}
</script>

<style scoped>
.i18n-icon-btn {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: #f7f7fa;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 1px 4px rgba(60,60,60,0.06);
  cursor: pointer;
  transition: background 0.2s;
}
.i18n-icon-btn:hover {
  background: #e6f0ff;
}
.i18n-drawer-btn {
  width: 100%;
  padding: 12px 0;
  font-size: 18px;
  border-radius: 8px;
  border: 1px solid #eee;
  background: #fff;
  cursor: pointer;
  margin-bottom: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.2s, color 0.2s;
}
.i18n-drawer-btn.active {
  background: #409eff;
  color: #fff;
  border-color: #409eff;
}
.i18n-drawer-btn i {
  margin-right: 8px;
}
</style>
