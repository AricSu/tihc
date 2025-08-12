import type { AxiosResponse } from 'axios'
import { request } from '@/utils/http'

// DDL 预检查请求接口
export interface DDLPrecheckRequest {
  sql: string
  collation_enabled?: boolean
}

// DDL 预检查响应接口
export interface DDLPrecheckResponse {
  lossy_status: 'Safe' | 'Lossy' | 'Unknown'
  risk_level: string
  issues: string[]
  error?: string
  recommendations: string[]
}

// DDL 预检查 API
export const ddlPrecheckAPI = {
  /**
   * 执行 DDL 预检查
   */
  precheck: (data: DDLPrecheckRequest): Promise<AxiosResponse<DDLPrecheckResponse>> => {
    return request.post('/api/ddl/precheck', data)
  }
}
