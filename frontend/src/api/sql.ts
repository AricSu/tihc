import { request } from '@/utils/http'

export interface QueryResult {
  id: string
  type: 'success' | 'error' | 'non-query'
  executionTime: number
  columns?: string[]
  columnTypes?: string[]
  data?: any[]
  details?: string
  message?: string
}

export function executeSql(params: { connection_id: number|string, sql: string }) {
  return request({
    url: '/api/sql/execute',
    method: 'post',
    data: [params.connection_id, params.sql]
  })
}