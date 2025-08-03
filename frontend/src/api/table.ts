import { request } from '@/utils/http'

export interface TableInfo {
  table_schema: string;
  table_name: string;
  create_time?: string;
  table_comment?: string;
}

export async function fetchTableList(connectionId: string, databaseName: string): Promise<TableInfo[]> {
  // 这里假设后端有 /api/tables/list?connection_id=xxx&database=yyy
  const res = await request({
    url: `/api/tables/list?connection_id=${encodeURIComponent(connectionId)}&database=${encodeURIComponent(databaseName)}`,
    method: 'get'
  })
  if (res.data?.status === 'failed' || res.data?.message) {
    throw new Error(res.data.message || 'Table list error')
  }
  return Array.isArray(res.data) ? res.data : res.data.data || []
}
