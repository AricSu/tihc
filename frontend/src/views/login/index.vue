<template>
  <div class="wh-full flex-col bg-[url(@/assets/images/login_bg.webp)] bg-cover relative">
    <div
      class="m-auto max-w-700 min-w-345 f-c-c rounded-8 auto-bg bg-opacity-20 bg-cover p-12 card-shadow"
    >
      <div class="hidden w-380 px-20 py-35 md:block">
        <img src="@/assets/images/logo.png" class="w-full" alt="login_banner">
      </div>

      <div class="w-320 flex-col px-20 py-32">
        <h2 class="f-c-c text-24 text-#6a6a6a font-normal">
          <!-- <img src="@/assets/images/logo.png" class="mr-12 h-50"> -->
          {{ $t('login.welcome') }}
        </h2>
        <div class="mt-32 flex items-center justify-center">
          <n-button
            class="h-44 w-full rounded-6 text-16 font-medium"
            type="primary"
            ghost
            :loading="loading"
            @click="handleLogin"
          >
            {{ $t('login.oneClickExperience') }}
          </n-button>
        </div>
        
        <!-- 语言切换按钮 -->
        <div class="mt-6 flex items-center justify-center">
          <n-dropdown 
            :options="languageOptions" 
            @select="handleLanguageChange"
            trigger="click"
            placement="bottom"
            :show-arrow="false"
          >
            <n-button 
              text
              size="small"
              class="px-3 py-1 rounded-full bg-gradient-to-r from-white/85 to-gray-100/85 backdrop-blur-md hover:from-white/95 hover:to-gray-100/95 border border-gray-300/50 transition-all duration-300 text-gray-700 hover:text-gray-800 shadow-lg hover:shadow-xl"
            >
              <template #icon>
                <Icon icon="carbon:earth-filled" class="text-14 mr-1.5 text-blue-600" />
              </template>
              <span class="text-13 font-medium">
                {{ locale === 'zh' ? '简体中文' : 'English' }}
              </span>
              <Icon icon="carbon:chevron-down" class="text-12 ml-1.5 opacity-60" />
            </n-button>
          </n-dropdown>
        </div>
      </div>
    </div>

    <TheFooter class="py-12" />
  </div>
</template>

<script setup>
import { useI18n } from 'vue-i18n'
import { Icon } from '@iconify/vue'

const router = useRouter()
const route = useRoute()
const { t, locale } = useI18n()

const loading = ref(false)

// 语言选项
const languageOptions = computed(() => [
  {
    label: '简体中文',
    key: 'zh',
    icon: () => h('span', { class: 'mr-2' }, '🇨🇳')
  },
  {
    label: 'English',
    key: 'en',
    icon: () => h('span', { class: 'mr-2' }, '🇺🇸')
  }
])

import { fetchLang, setLang } from '@/api/lang'
import { onMounted } from 'vue'
// 初始化时自动同步后端语言
onMounted(async () => {
  try {
    const lang = await fetchLang()
    if (['zh', 'en'].includes(lang)) {
      locale.value = lang
      localStorage.setItem('language', lang)
    }
  } catch (e) {
    // ignore
  }
})

// 语言切换处理
async function handleLanguageChange(key) {
  locale.value = key
  localStorage.setItem('language', key)
  try {
    await setLang(key)
  } catch (e) {
    // ignore
  }
}

async function handleLogin() {
  loading.value = true
  window.$message.loading(t('login.verifying'), { key: 'login' })
  try {
    await onLoginSuccess()
  } catch (error) {
    window.$message.destroy('login')
    console.error('Login error:', error)
  } finally {
    loading.value = false
  }
}

async function onLoginSuccess() {
  window.$message.loading(t('login.loggingIn'), { key: 'login' })
  try {
    window.$message.success(t('login.loginSuccess'), { key: 'login' })
    
    // 清除消息
    window.$message.destroy('login')
    
    // 强制跳转
    const targetPath = route.query.redirect || '/home'
    console.log('Redirecting to:', targetPath)
    
    if (route.query.redirect) {
      const query = { ...route.query }
      delete query.redirect
      router.replace({ path: targetPath, query })
    } else {
      router.replace('/home')
    }
  } catch (error) {
    console.error('Login redirect error:', error)
    window.$message.destroy('login')
    // 即使出错也尝试跳转到首页
    router.replace('/home')
  }
}
</script>
