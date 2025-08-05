import axios from 'axios'

// 请求拦截器
export function setupInterceptors(instance) {
  instance.interceptors.request.use(
    config => {
      // 可在此处添加 token、全局 loading 等逻辑
      // config.headers['Authorization'] = 'Bearer ...'
      return config
    },
    error => Promise.reject(error)
  )

  instance.interceptors.response.use(
    response => {
      // 可在此处统一处理响应数据
      return response
    },
    error => {
      // 全局错误处理
      if (error.response) {
        // 可根据 error.response.status 做统一提示
      }
      return Promise.reject(error)
    }
  )
}
