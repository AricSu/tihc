
import axios from 'axios'
import { setupInterceptors } from './interceptors'

function createAxios(options = {}) {
  const defaultOptions = {
    baseURL: import.meta.env.VITE_AXIOS_BASE_URL || '/api',
    timeout: 15000,
  }
  const service = axios.create({
    ...defaultOptions,
    ...options,
  })
  setupInterceptors(service)
  return service
}

export const request = createAxios()

// 支持自定义超时时间，业务方自行传递 timeout
export function requestWithCustomTimeout(config) {
  return request(config)
}
