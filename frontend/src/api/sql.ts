import { request } from '../utils/http'

export async function executeSql(sql: string, connectionId: number) {
  return request({
    url: '/api/sql/execute',
    method: 'post',
    data: { sql, connection_id: connectionId }
  })
}
