import { request } from '@/utils/http'

// 严格映射后端 SqlResult 结构体
export interface SqlMessage {
  level: string
  content: string
}

export interface QueryResults {
  data: {
    column_names: string[]
    column_type_names: string[]
    rows: any[][]
    rows_count?: number
    error?: string
    latency_ms?: number
    statement?: string
    messages?: SqlMessage[]
  }
}

export function executeSql(params: { connection_id: number|string, sql: string }) {
  return request({
    url: '/api/sql/execute',
    method: 'post',
    data: params
  }) as Promise<QueryResults>;
}