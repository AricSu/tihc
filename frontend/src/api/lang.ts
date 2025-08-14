import { request } from '@/utils/http'

// 获取当前语言
export function fetchLang(): Promise<string> {
  return request<{ lang: string }>({ url: '/api/lang', method: 'GET' }).then(res => res.data.lang)
}

// 设置语言
export function setLang(lang: string): Promise<void> {
  return request({ url: '/api/lang', method: 'POST', data: { lang } }).then(() => {})
}
