/**********************************
 * @Author: Ronnie Zhang
 * @LastEditor: Ronnie Zhang
 * @LastEditTime: 2023/12/05 21:25:07
 * @Email: zclzone@outlook.com
 * Copyright © 2023 Ronnie Zhang(大脸怪) | https://isme.top
 **********************************/

import api from '@/api'
import { useAuthStore, usePermissionStore, useUserStore } from '@/store'
import { getPermissions, getUserInfo } from '@/store/helper'

const WHITE_LIST = ['/login', '/404']
export function createPermissionGuard(router) {
  router.beforeEach(async (to) => {
    const authStore = useAuthStore()

    const oauthCallback = to.query.oauth_callback
    const accessToken = to.query.access_token
    const oauthError = to.query.oauth_error

    if (oauthCallback === '1') {
      if (accessToken) {
        authStore.setToken({ accessToken })

        // 显示成功消息（使用 setTimeout 确保在页面加载后显示）
        setTimeout(() => {
          window.$message?.success('GitHub 登录成功！')
        }, 100)

        // OAuth 成功后，检查是否有 redirect 参数，如果有则跳转到原目标页面
        const redirectPath = to.query.redirect || '/'

        // 清理 URL 中的 OAuth 参数，跳转到目标页面
        const cleanQuery = { ...to.query }
        delete cleanQuery.oauth_callback
        delete cleanQuery.access_token
        delete cleanQuery.redirect

        return {
          path: redirectPath,
          query: Object.keys(cleanQuery).length > 0 ? cleanQuery : undefined,
          replace: true,
        }
      }
      else if (oauthError) {
        // OAuth 失败：显示错误消息，跳转到登录页
        setTimeout(() => {
          window.$message?.error(`GitHub 登录失败: ${decodeURIComponent(oauthError)}`)
        }, 100)

        // 清理 OAuth 参数，跳转到登录页
        const cleanQuery = { ...to.query }
        delete cleanQuery.oauth_callback
        delete cleanQuery.oauth_error

        return {
          path: '/login',
          query: Object.keys(cleanQuery).length > 0 ? cleanQuery : undefined,
          replace: true,
        }
      }
    }

    const token = authStore.accessToken

    /** 没有token */
    if (!token) {
      if (WHITE_LIST.includes(to.path))
        return true
      // 只用站内路径作为 redirect，防止拼接完整 URL
      let redirectPath = to.path
      console.warn('[permission-guard] 当前 to.path:', to.path)
      console.warn('[permission-guard] 当前 window.location.href:', window.location.href)
      if (typeof redirectPath !== 'string' || !redirectPath.startsWith('/')) {
        console.warn('[permission-guard] redirectPath 非站内路径，重置为 /')
        redirectPath = '/'
      }
      console.warn('[permission-guard] 最终重定向到 /login?redirect=', redirectPath)
      return { path: '/login', query: { ...to.query, redirect: redirectPath } }
    }

    // 有token的情况
    if (to.path === '/login')
      return { path: '/' }
    if (WHITE_LIST.includes(to.path))
      return true

    const userStore = useUserStore()
    const permissionStore = usePermissionStore()
    if (!userStore.userInfo) {
      const [user, permissions] = await Promise.all([getUserInfo(), getPermissions()])
      userStore.setUser(user)
      permissionStore.setPermissions(permissions)
      const routeComponents = import.meta.glob('@/views/**/*.vue')
      permissionStore.accessRoutes.forEach((route) => {
        route.component = routeComponents[route.component] || undefined
        !router.hasRoute(route.name) && router.addRoute(route)
      })
      return { ...to, replace: true }
    }

    const routes = router.getRoutes()
    if (routes.find(route => route.name === to.name))
      return true

    // 判断是无权限还是404
    const { data: hasMenu } = await api.validateMenuPath(to.path)
    return hasMenu
      ? { name: '403', query: { path: to.fullPath }, state: { from: 'permission-guard' } }
      : { name: '404', query: { path: to.fullPath } }
  })
}
