import { request } from '@/utils/http'

export interface TableInfo {
  table_schema: string;
  table_name: string;
  create_time?: string;
  table_comment?: string;
}

export interface ColumnInfo {
  column_name: string;
  column_default?: string;
  is_nullable?: string;
  data_type?: string;
  character_octet_length?: number;
  character_set_name?: string;
  collation_name?: string;
  column_type?: string;
}

export interface IndexInfo {
  table_schema: string;
  table_name: string;
  non_unique?: number;
  key_name: string;
  column_name: string;
  index_comment?: string;
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


export async function fetchColumnList(connectionId: string, schema: string, table: string): Promise<ColumnInfo[]> {
  const res = await request({
    url: `/api/columns/list?connection_id=${encodeURIComponent(connectionId)}&schema=${encodeURIComponent(schema)}&table=${encodeURIComponent(table)}`,
    method: 'get'
  })
  if (res.data?.status === 'failed' || res.data?.message) {
    throw new Error(res.data.message || 'Column list error')
  }
  return Array.isArray(res.data) ? res.data : res.data.data || []
}

export async function fetchIndexList(connectionId: string, schema: string, table: string): Promise<IndexInfo[]> {
  const res = await request({
    url: `/api/indexes/list?connection_id=${encodeURIComponent(connectionId)}&schema=${encodeURIComponent(schema)}&table=${encodeURIComponent(table)}`,
    method: 'get'
  })
  if (res.data?.status === 'failed' || res.data?.message) {
    throw new Error(res.data.message || 'Index list error')
  }
  return Array.isArray(res.data) ? res.data : res.data.data || []
}