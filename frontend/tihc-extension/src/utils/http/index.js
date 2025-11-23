import axios from 'axios'
import { setupInterceptors } from './interceptors'

export function createAxios(options = {}) {
  const defaultOptions = {
    baseURL: import.meta.env.VITE_AXIOS_BASE_URL,
    timeout: 12000,
  }
  const service = axios.create({
    ...defaultOptions,
    ...options,
  })
  // 全局请求拦截日志
  service.interceptors.request.use((config) => {
    console.warn('[axios] 请求发起:', config.method, config.url, config)
    return config
  }, (err) => {
    console.error('[axios] 请求拦截器异常:', err)
    throw err
  })
  // 全局响应拦截日志
  service.interceptors.response.use((resp) => {
    console.warn('[axios] 响应成功:', resp.config?.url, resp)
    return resp
  }, (err) => {
    if (err?.config) {
      console.error('[axios] 响应失败:', err.config.url, err)
    } else {
      console.error('[axios] 响应失败:', err)
    }
    throw err
  })
  setupInterceptors(service)
  return service
}

export const request = createAxios()

export const mockRequest = createAxios({
  baseURL: '/mock-api',
})
