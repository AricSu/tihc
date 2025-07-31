import { request } from '@/utils/http';

export async function fetchDatabaseSchema(): Promise<Array<{ column_name: string; data_type: string; comment?: string }>> {
    const res = await request({ url: '/api/databases/list', method: 'get' })
  return res.data?.data?.[0]?.schema || []
}
