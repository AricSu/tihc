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
import api from './api'

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
    console.warn('[captcha] VITE_AXIOS_BASE_URL:', import.meta.env.VITE_AXIOS_BASE_URL)
    const captchaApiUrl = `${import.meta.env.VITE_AXIOS_BASE_URL}/auth/captcha`
    console.warn('[captcha] 请求验证码接口:', captchaApiUrl)
    console.warn('[captcha] 当前 window.location:', window.location.href)
    if (typeof browser !== 'undefined' && browser.runtime && browser.runtime.id) {
      console.warn('[captcha] 当前处于 extension 环境, runtime.id:', browser.runtime.id)
    }
    else {
      console.warn('[captcha] 当前不在 extension 环境')
    }
    const response = await fetch(captchaApiUrl)
    console.warn('[captcha] fetch response:', response)
    if (response.ok) {
      const data = await response.json()
      console.warn('[captcha] fetch response json:', data)
      if (data.code === 200 && data.data) {
        captchaUrl.value = data.data.image_base64
        captchaSessionId.value = data.data.session_id
      }
      else {
        console.error('验证码响应格式错误:', data)
      }
    }
    else {
      console.error('验证码请求失败:', response.status, response)
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
  console.warn('[login] GitHub 登录按钮点击')
  try {
    loading.value = true
    $message.loading('正在跳转到 GitHub 登录...', { key: 'login' })

    // 保存当前登录状态到sessionStorage，以便OAuth回调后恢复
    const currentPath = route.query.redirect || '/'
    sessionStorage.setItem('oauth_redirect', currentPath)
    console.warn('[login] 当前 redirect path:', currentPath)

    // 获取GitHub OAuth授权URL
    const redirectUrl = encodeURIComponent(currentPath)
    let startUrl = `${import.meta.env.VITE_AXIOS_BASE_URL}/auth/oauth/github/start?redirect=${redirectUrl}`
    // 检测 extension 环境，自动带上 source
    if (typeof window.chrome !== 'undefined' && chrome.runtime && chrome.runtime.id) {
      startUrl += `&source=extension`
      console.warn('[login] 检测到 extension 环境，附加参数:', startUrl)
    } else {
      startUrl += `&source=frontend`
    }
    console.warn('[login] 获取 GitHub 授权 URL:', startUrl)
    let resp, result
    try {
      resp = await fetch(startUrl)
      console.warn('[login] fetch resp:', resp)
    }
    catch (fetchErr) {
      console.error('[login] fetch 授权 URL 失败:', fetchErr)
      $message.error('无法连接到后端服务')
      loading.value = false
      return
    }

    try {
      result = await resp.json()
      console.warn('[login] GitHub 授权接口返回:', result)
    }
    catch (jsonErr) {
      console.error('[login] 授权接口返回非 JSON:', jsonErr, resp)
      $message.error('后端返回异常')
      loading.value = false
      return
    }

    if (!resp.ok) {
      // 显示详细的配置错误信息
      if (result.message && result.message.includes('GitHub OAuth 未正确配置')) {
        $message.error(result.message, { duration: 10000 })
        console.error('GitHub OAuth Configuration Error:', result.message)
      }
      else {
        $message.error('无法启动 GitHub 登录，请稍后重试')
        console.error('[login] fetch resp 非 200:', resp, result)
      }
      loading.value = false
      return
    }

    const oauthData = result.data || result
    const authorizeUrl = oauthData?.authorize_url
    console.warn('[login] authorizeUrl:', authorizeUrl)

    if (!authorizeUrl) {
      $message.error('无法获取GitHub授权地址')
      console.error('Missing authorize_url in response:', oauthData)
      loading.value = false
      return
    }

    // 优先使用 chrome.identity.launchWebAuthFlow 进行 OAuth 跳转
    const identity = window.chrome?.identity || window.browser?.identity
    if (identity && typeof identity.launchWebAuthFlow === 'function') {
      console.warn('[login] 检测到 extension 环境，使用 launchWebAuthFlow')
      identity.launchWebAuthFlow({
        url: authorizeUrl,
        interactive: true,
      }, async (redirectUrl) => {
        if ((window.chrome && chrome.runtime && chrome.runtime.lastError) || (window.browser && browser.runtime && browser.runtime.lastError)) {
          const lastError = (window.chrome && chrome.runtime && chrome.runtime.lastError) || (window.browser && browser.runtime && browser.runtime.lastError)
          console.error('[login] launchWebAuthFlow error:', lastError)
          $message.error(`Extension OAuth 失败: ${lastError.message}`)
          loading.value = false
          return
        }
        console.warn('[login] launchWebAuthFlow 回调 redirectUrl:', redirectUrl)
        if (redirectUrl) {
          try {
            const urlObj = new URL(redirectUrl)
            const code = urlObj.searchParams.get('code')
            const state = urlObj.searchParams.get('state')
            if (!code) {
              $message.error('未获取到授权 code')
              loading.value = false
              return
            }
            // 调用后端换 token，后端现在直接返回 JSON
            const callbackUrl = `${import.meta.env.VITE_AXIOS_BASE_URL}/auth/oauth/github/callback?code=${encodeURIComponent(code)}&state=${encodeURIComponent(state || '')}`
            const resp = await fetch(callbackUrl, { credentials: 'include' })
            let data
            try {
              data = await resp.json()
            }
            catch (jsonErr) {
              $message.error('后端返回非 JSON')
              console.error('OAuth 回调响应非 JSON:', jsonErr, resp)
              loading.value = false
              return
            }
            // 兼容后端返回格式（支持 accessToken 大小写）
            const token = data?.data?.accessToken || data?.data?.access_token || data?.token || data?.access_token
            if (resp.ok && token) {
              console.warn('[login] OAuth 回调后端返回 data:', data)
              authStore.setToken({ accessToken: token })
              $message.success('登录成功')
              if (window.chrome && window.chrome.runtime) {
                // 如果当前在 sidepanel.html 且 hash 包含 /login，则用 router 跳转首页，否则刷新 sidepanel.html
                if (window.location.pathname.endsWith('sidepanel.html') && window.location.hash.includes('/login')) {
                  console.warn('[login] 即将跳转首页')
                  router.push('/')
                  setTimeout(() => {
                    window.location.reload()
                  }, 100)
                }
                else {
                  window.location.href = window.chrome.runtime.getURL('sidepanel.html')
                }
              }
              else {
                $message.error('请在 Chrome 扩展环境下使用登录功能')
              }
            }
            else {
              $message.error(data?.message || '登录失败')
              console.error('[login] OAuth 回调失败:', data)
            }
          }
          catch (err) {
            console.error('OAuth 回调处理异常:', err)
            $message.error('登录处理异常')
          }
          loading.value = false
        }
        else {
          $message.error('未获取到 OAuth 回调地址')
          loading.value = false
        }
      })
    }
    else {
      // fallback: 新窗口打开授权页面
      console.warn('[login] fallback: window.open 跳转到 GitHub 授权页面:', authorizeUrl)
      window.open(authorizeUrl, '_blank')
    }
  }
  catch (e) {
    console.error('GitHub OAuth start error:', e)
    $message.error('启动 GitHub 登录失败')
    loading.value = false
  }
}

async function googleLogin() {
  console.warn('[login] Google 登录按钮点击')
  try {
    loading.value = true
    $message.loading('正在跳转到 Google 登录...', { key: 'login' })

    // 保存当前登录状态到sessionStorage，以便OAuth回调后恢复
    const currentPath = route.query.redirect || '/'
    sessionStorage.setItem('oauth_redirect', currentPath)

    // 获取Google OAuth授权URL
    const redirectUrl = encodeURIComponent(currentPath)
    const startUrl = `${import.meta.env.VITE_AXIOS_BASE_URL}/auth/oauth/google/start?redirect=${redirectUrl}`
    const resp = await fetch(startUrl)

    const result = await resp.json()

    if (!resp.ok) {
      // 显示详细的配置错误信息
      if (result.message && result.message.includes('Google OAuth 未正确配置')) {
        $message.error(result.message, { duration: 10000 })
        console.error('Google OAuth Configuration Error:', result.message)
      }
      else {
        $message.error('无法启动 Google 登录，请稍后重试')
      }
      return
    }

    const oauthData = result.data || result
    const authorizeUrl = oauthData?.authorize_url

    if (!authorizeUrl) {
      $message.error('无法获取Google授权地址')
      console.error('Missing authorize_url in response:', oauthData)
      return
    }

    // 直接跳转到Google授权页面
    // 后端已经配置了正确的 redirect_uri，不需要在前端修改
    window.location.href = authorizeUrl
  }
  catch (e) {
    console.error('Google OAuth start error:', e)
    $message.error('启动 Google 登录失败')
    loading.value = false
  }
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
    const loginUrl = `${import.meta.env.VITE_AXIOS_BASE_URL}/auth/login`
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
    if (route.query.redirect) {
      const path = route.query.redirect
      delete route.query.redirect
      console.warn('[login] 跳转 redirect:', path)
      router.push({ path, query: route.query })
    }
    else {
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
