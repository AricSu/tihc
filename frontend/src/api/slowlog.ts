import { request, requestWithCustomTimeout } from '@/utils/http'

/** 获取慢日志文件列表（不需要 connectionId） */
/** 获取慢日志文件列表（需传递 logDir 和 pattern） */
export interface SlowlogScanResult {
  files?: any[]
  code?: number
  reason?: string
  error?: string
}
export function getSlowlogFiles(params: { logDir: string, pattern: string }): Promise<SlowlogScanResult> {
  return request.post('/api/slowlog/scan-files', { params })
}

/** 处理慢日志文件（需要指定连接和文件列表） */
export interface SlowlogProcessResult {
  status?: string
  processed?: string[]
  error?: string
  result?: any
}
export function processSlowlogFiles(connectionId: number, logDir: string, pattern: string): Promise<SlowlogProcessResult> {
  return requestWithCustomTimeout({
    url: '/api/slowlog/process',
    method: 'post',
    data: { connectionId, logDir, pattern },
    timeout: 600000 // 60秒超时
  }).then(res => res.data)
}
