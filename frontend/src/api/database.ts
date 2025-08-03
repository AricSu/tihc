import { request } from '@/utils/http';

export async function fetchDatabaseList(connectionId: string): Promise<Array<{ schema_name: string; default_collation_name: string; default_character_set_name: string }>> {
  const res = await request({ url: `/api/databases/list?connection_id=${encodeURIComponent(connectionId)}`, method: 'get' })
  console.log('数据库列表接口返回:', res.data)
  // 如果后端返回 status 字段且不是 success，则抛出异常
  if (res.data?.status === 'failed' || res.data?.message) {
    throw new Error(res.data.message || 'Database error')
  }
  // 兼容直接返回数组
  return Array.isArray(res.data) ? res.data : res.data.data || []
}
