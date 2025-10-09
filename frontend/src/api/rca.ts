import { request } from '@/utils/http'

// RCA问诊台相关接口类型定义
interface ChatMessage {
  role: 'user' | 'assistant'
  content: string
  timestamp: number
}

interface ChatResponse {
  message: string
  session_id: string
  suggestions?: string[]
}

interface SessionResponse {
  session_id: string
  created_at: string
}

interface ChatHistory {
  session_id: string
  messages: ChatMessage[]
  created_at: string
  updated_at: string
}

interface AnalysisParams {
  query?: string
  database_type?: string
  time_range?: {
    start: string
    end: string
  }
  [key: string]: any
}

interface AnalysisResponse {
  result: string
  recommendations: string[]
  severity: 'low' | 'medium' | 'high' | 'critical'
  metadata?: Record<string, any>
}

interface HealthStatus {
  status: 'healthy' | 'warning' | 'error'
  checks: {
    name: string
    status: string
    message?: string
  }[]
  timestamp: string
}

// 发送消息到RCA助手
export function sendMessage(message: string, sessionId?: string): Promise<ChatResponse> {
  return request<ChatResponse>({
    url: '/api/rca/chat',
    method: 'POST',
    data: {
      message,
      session_id: sessionId,
      timestamp: Date.now()
    }
  }).then(res => res.data)
}

// 创建新会话
export function createSession(): Promise<SessionResponse> {
  return request<SessionResponse>({
    url: '/api/rca/session',
    method: 'POST'
  }).then(res => res.data)
}

// 获取会话历史
export function getChatHistory(sessionId: string): Promise<ChatHistory> {
  return request<ChatHistory>({
    url: `/api/rca/history/${sessionId}`,
    method: 'GET'
  }).then(res => res.data)
}

// 获取所有会话列表
export function getChatSessions(): Promise<ChatHistory[]> {
  return request<ChatHistory[]>({
    url: '/api/rca/sessions',
    method: 'GET'
  }).then(res => res.data)
}

// 删除会话
export function deleteSession(sessionId: string): Promise<void> {
  return request({
    url: `/api/rca/session/${sessionId}`,
    method: 'DELETE'
  }).then(() => {})
}

// 分析数据库性能
export function analyzePerformance(params: AnalysisParams): Promise<AnalysisResponse> {
  return request<AnalysisResponse>({
    url: '/api/rca/analyze/performance',
    method: 'POST',
    data: params
  }).then(res => res.data)
}

// 分析慢查询
export function analyzeSlowQuery(params: AnalysisParams): Promise<AnalysisResponse> {
  return request<AnalysisResponse>({
    url: '/api/rca/analyze/slowquery',
    method: 'POST',
    data: params
  }).then(res => res.data)
}

// 诊断连接问题
export function diagnoseConnection(params: AnalysisParams): Promise<AnalysisResponse> {
  return request<AnalysisResponse>({
    url: '/api/rca/diagnose/connection',
    method: 'POST',
    data: params
  }).then(res => res.data)
}

// 分析资源监控
export function analyzeResources(params: AnalysisParams): Promise<AnalysisResponse> {
  return request<AnalysisResponse>({
    url: '/api/rca/analyze/resources',
    method: 'POST',
    data: params
  }).then(res => res.data)
}

// 获取系统健康状态
export function getHealthStatus(): Promise<HealthStatus> {
  return request<HealthStatus>({
    url: '/api/rca/health',
    method: 'GET'
  }).then(res => res.data)
}

// 上传日志文件进行分析
export function uploadLogFile(file: File, logType: string = 'slowquery'): Promise<AnalysisResponse> {
  const formData = new FormData()
  formData.append('file', file)
  formData.append('log_type', logType)

  return request<AnalysisResponse>({
    url: '/api/rca/upload/log',
    method: 'POST',
    data: formData,
    headers: {
      'Content-Type': 'multipart/form-data'
    }
  }).then(res => res.data)
}

// 获取推荐的解决方案
export function getRecommendations(problemType: string, context: Record<string, any> = {}): Promise<AnalysisResponse> {
  return request<AnalysisResponse>({
    url: '/api/rca/recommendations',
    method: 'POST',
    data: {
      problem_type: problemType,
      context,
      timestamp: Date.now()
    }
  }).then(res => res.data)
}

// 获取RCA配置
export function getRCAConfig(): Promise<Record<string, any>> {
  return request<Record<string, any>>({
    url: '/api/rca/config',
    method: 'GET'
  }).then(res => res.data)
}

// 更新RCA配置
export function updateRCAConfig(config: Record<string, any>): Promise<void> {
  return request({
    url: '/api/rca/config',
    method: 'PUT',
    data: config
  }).then(() => {})
}