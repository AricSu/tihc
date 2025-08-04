import { request } from '@/utils/http'

export interface Connection {
  id?: number
  name: string
  engine: string
  host: string
  port: number
  username: string
  password?: string
  database?: string
  use_tls?: boolean
  ca_cert_path?: string
  created_at?: string
}

export function createConnection(conn: Connection) {
  const payload: any = {
    id: typeof conn.id === 'number' ? conn.id : Date.now(),
    name: conn.name,
    engine: conn.engine,
    host: conn.host,
    port: conn.port,
    username: conn.username,
    password: conn.password,
    database: conn.database,
    use_tls: conn.use_tls ?? false,
    ca_cert_path: conn.ca_cert_path ?? '',
    created_at: conn.created_at || new Date().toISOString()
  }
  return request.post('/api/connections/create', payload)
}

export function testConnection(conn: Connection) {
  const payload: any = {
    id: typeof conn.id === 'number' ? conn.id : Date.now(),
    name: conn.name,
    engine: conn.engine,
    host: conn.host,
    port: conn.port,
    username: conn.username,
    password: conn.password,
    database: conn.database,
    use_tls: conn.use_tls ?? false,
    ca_cert_path: conn.ca_cert_path ?? '',
    created_at: conn.created_at || new Date().toISOString()
  }
  // 强制 id 为 number 类型
  payload.id = Number(payload.id)
  return request.post('/api/connections/test', payload)
}

export function deleteConnection(id: string | number) {
  return request.delete(`/api/connections/${id}`)
}

export function listConnections() {
  return request.get('/api/connections/list')
}

export function updateConnection(id: string | number, conn: Connection) {
  return request.put(`/api/connections/${id}`, conn)
}

export async function handleUpdateConnection(conn: Connection) {
  if (!conn.id) {
    throw new Error('更新连接必须有 id')
  }
  // 确保 id 为 number 类型
  const idNum = typeof conn.id === 'number' ? conn.id : Number(conn.id)
  const payload: any = {
    ...conn,
    id: idNum,
    use_tls: conn.use_tls ?? false,
    ca_cert_path: conn.ca_cert_path ?? '',
    created_at: conn.created_at || new Date().toISOString()
  }
  return updateConnection(idNum, payload)
}