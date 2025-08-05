import { request } from '@/utils/http'

// API endpoint paths for SQL Editor module
const API_PATH = {
  connections: '/api/sql_editor/connections',      // Connection management
  databases: '/api/sql_editor/databases',          // Database list
  tables: '/api/sql_editor/tables',                // Table list
  columns: '/api/sql_editor/columns',              // Column list
  indexes: '/api/sql_editor/indexes',              // Index list
  sql: '/api/sql_editor/sql'                       // SQL execution
}

// ---------- Type Definitions ----------

// Database connection information
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
// Table metadata
export interface TableInfo {
  table_schema: string;
  table_name: string;
  create_time?: string;
  table_comment?: string;
}
// Column metadata
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
// Index metadata
export interface IndexInfo {
  table_schema: string;
  table_name: string;
  non_unique?: number;
  key_name: string;
  column_name: string;
  index_comment?: string;
}
// SQL execution message
export interface SqlMessage {
  level: string
  content: string
}
// SQL execution result structure
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

// ---------- Connection API ----------
// CRUD operations for database connections
export const ConnectionAPI = {
  // Create a new connection
  create(conn: Connection) {
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
    return request.post(`${API_PATH.connections}/create`, payload)
  },
  // Test connection parameters
  test(conn: Connection) {
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
    payload.id = Number(payload.id)
    return request.post(`${API_PATH.connections}/test`, payload)
  },
  // Delete a connection by ID
  delete(id: string | number) {
    return request.delete(`${API_PATH.connections}/${id}`)
  },
  // List all connections
  list() {
    return request.get(`${API_PATH.connections}/list`)
  },
  // Update connection details
  update(id: string | number, conn: Connection) {
    return request.put(`${API_PATH.connections}/${id}`, conn)
  },
  // Update connection, ensure ID is number
  async handleUpdate(conn: Connection) {
    if (!conn.id) throw new Error('Connection update requires id')
    const idNum = typeof conn.id === 'number' ? conn.id : Number(conn.id)
    const payload: any = {
      ...conn,
      id: idNum,
      use_tls: conn.use_tls ?? false,
      ca_cert_path: conn.ca_cert_path ?? '',
      created_at: conn.created_at || new Date().toISOString()
    }
    return ConnectionAPI.update(idNum, payload)
  }
}

// ---------- Database API ----------
// List all databases for a connection
export const DatabaseAPI = {
  async list(connectionId: string): Promise<Array<{ schema_name: string; default_collation_name: string; default_character_set_name: string }>> {
    const res = await request({ url: `${API_PATH.databases}/list?connection_id=${encodeURIComponent(connectionId)}`, method: 'get' })
    if (res.data?.status === 'failed' || res.data?.message) {
      throw new Error(res.data.message || 'Database error')
    }
    return Array.isArray(res.data) ? res.data : res.data.data || []
  }
}

// ---------- Table & Column API ----------
// Table, column, and index metadata operations
export const TableAPI = {
  // List all tables in a database
  async list(connectionId: string, databaseName: string): Promise<TableInfo[]> {
    const res = await request({
      url: `${API_PATH.tables}/list?connection_id=${encodeURIComponent(connectionId)}&database=${encodeURIComponent(databaseName)}`,
      method: 'get'
    })
    if (res.data?.status === 'failed' || res.data?.message) {
      throw new Error(res.data.message || 'Table list error')
    }
    return Array.isArray(res.data) ? res.data : res.data.data || []
  },
  // List all columns in a table
  async columns(connectionId: string, schema: string, table: string): Promise<ColumnInfo[]> {
    const res = await request({
      url: `${API_PATH.columns}/list?connection_id=${encodeURIComponent(connectionId)}&schema=${encodeURIComponent(schema)}&table=${encodeURIComponent(table)}`,
      method: 'get'
    })
    if (res.data?.status === 'failed' || res.data?.message) {
      throw new Error(res.data.message || 'Column list error')
    }
    return Array.isArray(res.data) ? res.data : res.data.data || []
  },
  // List all indexes in a table
  async indexes(connectionId: string, schema: string, table: string): Promise<IndexInfo[]> {
    const res = await request({
      url: `${API_PATH.indexes}/list?connection_id=${encodeURIComponent(connectionId)}&schema=${encodeURIComponent(schema)}&table=${encodeURIComponent(table)}`,
      method: 'get'
    })
    if (res.data?.status === 'failed' || res.data?.message) {
      throw new Error(res.data.message || 'Index list error')
    }
    return Array.isArray(res.data) ? res.data : res.data.data || []
  }
}

// ---------- SQL Execution API ----------
// Execute SQL statement and return results
export const SqlAPI = {
  execute(params: { connection_id: number|string, sql: string }) {
    return request({
      url: `${API_PATH.sql}/execute`,
      method: 'post',
      data: params
    }) as Promise<QueryResults>;
  }
}
