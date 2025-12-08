// Extension/Frontend OAuth统一入口
import { request } from '@/utils'

export async function githubOAuth({ redirect, onSuccess, onError, $message }) {
  try {
    $message?.loading('正在跳转到 GitHub 登录...', { key: 'login' })
    const redirectUrl = encodeURIComponent(redirect || '/')
    let startUrl = `${import.meta.env.VITE_AXIOS_BASE_URL}/api/auth/oauth/github/start?redirect=${redirectUrl}`
    if (typeof window.chrome !== 'undefined' && window.chrome.runtime && window.chrome.runtime.id) {
      startUrl += `&source=extension`
    }
    else {
      startUrl += `&source=frontend`
    }
    const resp = await fetch(startUrl)
    const result = await resp.json()
    if (!resp.ok) {
      $message?.error(result.message || '无法启动 GitHub 登录，请稍后重试')
      onError?.(result)
      return
    }
    const oauthData = result.data || result
    const authorizeUrl = oauthData?.authorize_url
    if (!authorizeUrl) {
      $message?.error('无法获取GitHub授权地址')
      onError?.(oauthData)
      return
    }
    const identity = window.chrome?.identity || window.browser?.identity
    if (identity && typeof identity.launchWebAuthFlow === 'function') {
      identity.launchWebAuthFlow({ url: authorizeUrl, interactive: true }, async (redirectUrl) => {
        if ((window.chrome && window.chrome.runtime && window.chrome.runtime.lastError) || (window.browser && window.browser.runtime && window.browser.runtime.lastError)) {
          const lastError = (window.chrome && window.chrome.runtime && window.chrome.runtime.lastError) || (window.browser && window.browser.runtime && window.browser.runtime.lastError)
          $message?.error(`Extension OAuth 失败: ${lastError.message}`)
          onError?.(lastError)
          return
        }
        if (redirectUrl) {
          try {
            const urlObj = new URL(redirectUrl)
            const code = urlObj.searchParams.get('code')
            const state = urlObj.searchParams.get('state')
            if (!code) {
              $message?.error('未获取到授权 code')
              onError?.('no_code')
              return
            }
            const callbackUrl = `${import.meta.env.VITE_AXIOS_BASE_URL}/api/auth/oauth/github/callback?code=${encodeURIComponent(code)}&state=${encodeURIComponent(state || '')}`
            const resp = await fetch(callbackUrl, { credentials: 'include' })
            const data = await resp.json()
            const token = data?.data?.accessToken || data?.data?.access_token || data?.token || data?.access_token
            if (resp.ok && token) {
              onSuccess?.(token, data)
            }
            else {
              $message?.error(data?.message || '登录失败')
              onError?.(data)
            }
          }
          catch (err) {
            $message?.error('OAuth 回调处理异常')
            onError?.(err)
          }
        }
        else {
          $message?.error('未获取到 OAuth 回调地址')
          onError?.('no_redirect')
        }
      })
    }
    else {
      window.open(authorizeUrl, '_blank')
    }
  }
  catch (e) {
    $message?.error('启动 GitHub 登录失败')
    onError?.(e)
  }
}

export async function googleOAuth({ redirect }) {
  const redirectUrl = encodeURIComponent(redirect || '/')
  const startUrl = `${import.meta.env.VITE_AXIOS_BASE_URL}/api/auth/oauth/google/start?redirect=${redirectUrl}`
  const resp = await fetch(startUrl)
  const result = await resp.json()
  if (!resp.ok)
    throw new Error(result.message || '无法启动 Google 登录')
  const oauthData = result.data || result
  const authorizeUrl = oauthData?.authorize_url
  if (!authorizeUrl)
    throw new Error('无法获取Google授权地址')
  window.location.href = authorizeUrl
}

export default {
  toggleRole: data => request.post('/auth/role/toggle', data),
  login: data => request.post('/auth/login', data, { needToken: false }),
  getUser: () => request.get('/user/detail'),
  getCaptcha: () => request.get('/auth/captcha'),
}
