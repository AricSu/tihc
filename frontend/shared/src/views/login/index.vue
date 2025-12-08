<template>
  <div class="wh-full flex-col bg-[url(@/assets/images/login_bg.webp)] bg-cover">
    <div
      class="m-auto max-w-700 min-w-345 f-c-c rounded-8 auto-bg bg-opacity-20 bg-cover p-12 card-shadow"
    >
      <div class="hidden w-380 px-20 py-35 md:block">
        <img src="@/assets/images/login_banner.webp" class="w-full" alt="login_banner">
      </div>

      <div class="w-320 flex-col px-20 py-32">
        <h2 class="f-c-c text-24 text-#6a6a6a font-normal">
          <img src="@/assets/images/logo.png" class="mr-12 h-50">
          {{ title }}
        </h2>
        <n-input
          v-model:value="loginInfo.email"
          autofocus
          class="mt-32 h-40 items-center"
          placeholder="请输入邮箱"
          :maxlength="50"
          type="email"
        >
          <template #prefix>
            <i class="i-fe:mail mr-12 opacity-20" />
          </template>
        </n-input>
        <n-input
          v-model:value="loginInfo.password"
          class="mt-20 h-40 items-center"
          type="password"
          show-password-on="mousedown"
          placeholder="请输入密码"
          :maxlength="20"
          @keydown.enter="handleLogin()"
        >
          <template #prefix>
            <i class="i-fe:lock mr-12 opacity-20" />
          </template>
        </n-input>

        <div class="mt-20 flex items-center">
          <n-input
            v-model:value="loginInfo.captcha"
            class="h-40 items-center"
            palceholder="请输入验证码"
            :maxlength="4"
            @keydown.enter="handleLogin()"
          >
            <template #prefix>
              <i class="i-fe:key mr-12 opacity-20" />
            </template>
          </n-input>
          <img
            v-if="captchaUrl"
            :src="captchaUrl"
            alt="验证码"
            height="40"
            class="ml-12 w-80 cursor-pointer"
            @click="initCaptcha"
          >
        </div>

        <n-checkbox
          class="mt-20"
          :checked="isRemember"
          label="记住我"
          :on-update:checked="(val) => (isRemember = val)"
        />

        <div class="mt-20 flex flex-col gap-12">
          <n-button
            class="h-40 w-full rounded-5 text-16"
            type="primary"
            :loading="loading"
            @click="handleLogin()"
          >
            邮箱登录
          </n-button>

          <div class="flex items-center gap-12">
            <n-button
              class="h-40 flex-1 rounded-5 text-16"
              type="primary"
              ghost
              @click="quickLogin()"
            >
              <i class="i-mdi:github mr-8" />
              GitHub 登录
            </n-button>

            <n-button
              class="h-40 flex-1 rounded-5 text-16"
              type="primary"
              ghost
              @click="googleLogin()"
            >
              <i class="i-mdi:google mr-8" />
              Google 登录
            </n-button>
          </div>
        </div>
      </div>
    </div>

    <TheFooter class="py-12" />
  </div>
</template>

<script setup>
import { useStorage } from '@vueuse/core'
import { useAuthStore } from '@/store'
import { lStorage, throttle } from '@/utils'
import api, { githubOAuth, googleOAuth } from './api'

const authStore = useAuthStore()
const router = useRouter()
const route = useRoute()
const title = import.meta.env.VITE_TITLE

const loginInfo = ref({
  email: '',
  password: '',
  captcha: '',
})

const captchaUrl = ref('')
const captchaSessionId = ref('')

const initCaptcha = throttle(async () => {
  try {
    const { data } = await api.getCaptcha()
    if (
      (data && data.code === 200 && data.data)
      || (data && data.image_base64 && data.session_id)
    ) {
      captchaUrl.value = data.image_base64 || data.data?.image_base64
      captchaSessionId.value = data.session_id || data.data?.session_id
    }
    else {
      console.error('验证码响应格式错误:', data)
    }
  }
  catch (error) {
    console.error('获取验证码失败:', error)
  }
}, 500)

const localLoginInfo = lStorage.get('loginInfo')
if (localLoginInfo) {
  loginInfo.value.email = localLoginInfo.email || ''
  loginInfo.value.password = localLoginInfo.password || ''
}
initCaptcha()

const isRemember = useStorage('isRemember', true)
const loading = ref(false)

async function quickLogin() {
  loading.value = true
  const currentPath = route.query.redirect || '/'
  sessionStorage.setItem('oauth_redirect', currentPath)
  await githubOAuth({
    redirect: currentPath,
    $message,
    router,
    onSuccess(token) {
      authStore.setToken({ accessToken: token })
      $message.success('登录成功')
      router.push('/')
    },
    onError(err) {
      $message.error('GitHub 登录失败')
      console.error('[login] GitHub OAuth error:', err)
      loading.value = false
    },
  })
  loading.value = false
}

async function googleLogin() {
  loading.value = true
  const currentPath = route.query.redirect || '/'
  sessionStorage.setItem('oauth_redirect', currentPath)
  try {
    await googleOAuth({ redirect: currentPath })
  }
  catch (e) {
    $message.error('启动 Google 登录失败')
    console.error('[login] Google OAuth error:', e)
  }
  loading.value = false
}

async function handleLogin() {
  console.warn('[login] 邮箱登录按钮点击')
  const { email, password, captcha } = loginInfo.value
  if (!email || !password)
    return $message.warning('请输入邮箱和密码')
  if (!captcha)
    return $message.warning('请输入验证码')
  if (!captchaSessionId.value)
    return $message.warning('验证码会话无效，请刷新验证码')

  // 简单的邮箱格式验证
  if (!email.includes('@') || !email.includes('.'))
    return $message.warning('请输入有效的邮箱地址')

  try {
    loading.value = true
    $message.loading('正在验证，请稍后...', { key: 'login' })
    const loginParams = {
      email,
      password: password.toString(),
      captcha,
      captcha_session_id: captchaSessionId.value,
    }
    const loginUrl = `${import.meta.env.VITE_AXIOS_BASE_URL}/api/auth/login`
    console.warn('[login] 请求登录接口:', loginUrl)
    console.warn('[login] 登录参数:', loginParams)
    const { data } = await api.login(loginParams)
    console.warn('[login] 登录响应 data:', data)
    if (isRemember.value) {
      lStorage.set('loginInfo', { email, password })
    }
    else {
      lStorage.remove('loginInfo')
    }
    onLoginSuccess(data)
  }
  catch (error) {
    // 10003为验证码错误专属业务码
    if (error?.code === 10003) {
      // 为防止爆破，验证码错误则刷新验证码
      initCaptcha()
    }
    $message.destroy('login')
    console.error('[login] 登录异常:', error)
  }
  loading.value = false
}

async function onLoginSuccess(data = {}) {
  console.warn('[login] 登录成功，setToken:', data)
  authStore.setToken(data)
  $message.loading('登录中...', { key: 'login' })
  try {
    $message.success('登录成功', { key: 'login' })
    const path = route.query.redirect || '/'
    console.warn('[login] onLoginSuccess 跳转分析:')
    console.warn('[login] 当前 route.query.redirect:', route.query.redirect)
    console.warn('[login] 当前 window.location.href:', window.location.href)
    delete route.query.redirect
    if (typeof path === 'string' && path.startsWith('/')) {
      // 站内跳转
      console.warn('[login] 跳转 redirect:', path)
      router.push({ path, query: route.query })
    }
    else {
      // 非站内跳转，跳首页（如需站外跳转可扩展 window.location.href = path）
      console.warn('[login] 跳转首页')
      router.push('/')
    }
  }
  catch (error) {
    console.error(error)
    $message.destroy('login')
  }
}
</script>
