import { request } from '@/utils/http'

export interface Connection {
  id?: string | number
  name: string
  engine: string
  host: string
  port: number
  username: string
  password?: string
  database?: string
  created_at?: string
}

export function createConnection(conn: Connection) {
  return request.post('/api/connections/create', {
    ...conn,
    created_at: conn.created_at || new Date().toISOString()
  })
}

export function testConnection(conn: Connection) {
  return request.post('/api/connections/test', {
    id: typeof conn.id === 'number' ? conn.id : 0,
    name: conn.name,
    engine: conn.type || conn.engine,
    host: conn.host,
    port: conn.port,
    username: conn.username,
    password: conn.password,
    database: conn.database,
    created_at: conn.created_at || new Date().toISOString()
  })
}

export function deleteConnection(id: string | number) {
  return request.delete(`/api/connections/${id}`)
}

export function listConnections() {
  return request.get('/api/connections/list')
}

export function getConnection(id: string | number) {
  return request.get(`/connections/${id}`)
}

export function updateConnection(id: string | number, conn: Connection) {
  return request.put(`/connections/${id}`, conn)
}
