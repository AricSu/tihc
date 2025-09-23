// 扩展数据接收 API - 适配动态服务发现
import { ref, reactive } from 'vue'

// 全局状态管理扩展数据
export const extensionData = reactive({
  tokens: {},
  lastUpdate: null,
  connectionStatus: 'waiting', // waiting | connected | disconnected | error
  serverInfo: null
})

// API 客户端类 - 与动态发现的 Rust 后端通信
export class ExtensionApiClient {
  private baseUrl: string | null = null

  constructor() {
    this.detectServerConnection()
  }

  // 检测服务器连接
  private async detectServerConnection() {
    // 等待服务发现完成
    const maxAttempts = 30; // 30秒
    for (let i = 0; i < maxAttempts; i++) {
      try {
        // 检查是否有服务发现脚本注入的服务器信息
        const serverInfo = await this.getServerInfoFromPage()
        if (serverInfo) {
          this.baseUrl = `${serverInfo.serverUrl}:${serverInfo.port}/api`
          extensionData.serverInfo = serverInfo
          extensionData.connectionStatus = 'connected'
          console.log('[TiHC Frontend] 检测到服务器:', serverInfo)
          return
        }
      } catch (error) {
        // 继续尝试
      }
      
      await new Promise(resolve => setTimeout(resolve, 1000))
    }
    
    extensionData.connectionStatus = 'disconnected'
    console.warn('[TiHC Frontend] 未能检测到 TiHC Server')
  }

  // 从页面获取服务器信息
  private async getServerInfoFromPage(): Promise<any> {
    return new Promise((resolve) => {
      // 监听服务发现消息
      const handleMessage = (event: MessageEvent) => {
        if (event.data.type === 'tihc_server_handshake') {
          window.removeEventListener('message', handleMessage)
          resolve(event.data)
        }
      }
      
      window.addEventListener('message', handleMessage)
      
      // 5秒后超时
      setTimeout(() => {
        window.removeEventListener('message', handleMessage)
        resolve(null)
      }, 5000)
    })
  }

  private getApiUrl(): string {
    if (!this.baseUrl) {
      // 如果没有动态发现，回退到默认值
      if (process.env.NODE_ENV === 'development') {
        return 'http://localhost:8080/api'
      }
      return '/api'
    }
    return this.baseUrl
  }

  // 获取扩展状态
  async getExtensionStatus() {
    if (extensionData.connectionStatus !== 'connected') {
      throw new Error('Server not connected')
    }
    
    try {
      const response = await fetch(`${this.getApiUrl()}/extension/status`)
      const result = await response.json()
      
      if (response.ok) {
        extensionData.connectionStatus = 'connected'
        return result
      } else {
        extensionData.connectionStatus = 'error'
        throw new Error(result.error || 'Failed to get extension status')
      }
    } catch (error) {
      extensionData.connectionStatus = 'error'
      throw error
    }
  }

  // 通用 HTTP 请求
  async request(endpoint: string, options: RequestInit = {}): Promise<any> {
    const url = `${this.getApiUrl()}${endpoint}`
    
    try {
      const response = await fetch(url, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
          'X-Source': 'tihc-frontend',
          ...options.headers,
        },
        ...options,
      })

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`)
      }

      const data = await response.json()
      return data
    } catch (error) {
      console.error(`[TiHC Frontend API] 请求失败: ${url}`, error)
      throw error
    }
  }

  // 获取所有扩展数据
  async getAllTokens() {
    try {
      const response = await this.request('/extension/data')
      return response.data || {}
    } catch (error) {
      console.error('[TiHC Frontend API] 获取扩展数据失败:', error)
      return {}
    }
  }

  // 心跳检测
  async ping() {
    try {
      const response = await this.request('/extension/ping')
      return response.success
    } catch (error) {
      return false
    }
  }

  // 获取配置
  async getConfig() {
    try {
      const response = await this.request('/extension/config')
      return response.data
    } catch (error) {
      console.error('[TiHC Frontend API] 获取配置失败:', error)
      return null
    }
  }
}

// API 处理器类 - 处理扩展数据更新
export class ExtensionApiHandler {
  private apiClient: ExtensionApiClient

  constructor() {
    this.apiClient = new ExtensionApiClient()
    this.startPeriodicUpdate()
  }

  // 启动定期更新
  private startPeriodicUpdate() {
    // 立即更新一次
    this.updateDataFromBackend()

    // 每10秒更新一次数据
    setInterval(() => {
      this.updateDataFromBackend()
    }, 10000)

    console.log('[TiHC Frontend] 扩展数据定期更新已启动')
  }

  // 从后端更新数据
  async updateDataFromBackend() {
    try {
      // 检查连接状态
      const isConnected = await this.apiClient.ping()
      extensionData.connectionStatus = isConnected ? 'connected' : 'disconnected'

      if (isConnected) {
        // 获取最新数据
        const tokens = await this.apiClient.getAllTokens()
        
        if (tokens && Object.keys(tokens).length > 0) {
          extensionData.tokens = tokens
          extensionData.lastUpdate = Date.now()
          
          // 处理业务逻辑
          this.processTokenData(tokens)
        }
      }
    } catch (error) {
      console.error('[TiHC Frontend] 更新数据失败:', error)
      extensionData.connectionStatus = 'error'
    }
  }

  // 处理 Token 数据的业务逻辑
  private processTokenData(tokens: any) {
    console.log('[TiHC Frontend] 处理扩展数据:', tokens)

    Object.entries(tokens).forEach(([domain, domainData]: [string, any]) => {
      // 处理 Clinic 数据
      if (domainData.clinic) {
        this.processClinicData(domain, domainData.clinic)
      }

      // 处理 Grafana 数据
      if (domainData.grafana) {
        this.processGrafanaData(domain, domainData.grafana)
      }
    })

    // 触发全局事件
    this.notifyComponents('tokens_updated', tokens)
  }

  // 处理 Clinic 数据
  private processClinicData(domain: string, clinicData: any) {
    console.log(`[TiHC Frontend] 处理 Clinic 数据: ${domain}`, clinicData)

    // 这里可以实现自动登录、设置请求头等业务逻辑
    if (clinicData.session_id && clinicData.csrf_token) {
      // 设置全局 API 认证信息
      this.setApiAuth('clinic', {
        sessionId: clinicData.session_id,
        csrfToken: clinicData.csrf_token,
        domain: domain
      })
    }
  }

  // 处理 Grafana 数据
  private processGrafanaData(domain: string, grafanaData: any) {
    console.log(`[TiHC Frontend] 处理 Grafana 数据: ${domain}`, grafanaData)

    if (grafanaData.session || grafanaData.auth_token) {
      this.setApiAuth('grafana', {
        session: grafanaData.session,
        authToken: grafanaData.auth_token,
        domain: domain
      })
    }
  }

  // 设置 API 认证信息
  private setApiAuth(service: string, authData: any) {
    console.log(`[TiHC Frontend] 设置 ${service} API 认证`, authData)

    // 可以在这里设置 axios 默认请求头或其他 HTTP 客户端配置
    // 例如：
    // import axios from 'axios'
    // axios.defaults.headers.common['X-Session-ID'] = authData.sessionId
    // axios.defaults.headers.common['X-CSRF-Token'] = authData.csrfToken
  }

  // 通知 Vue 组件
  private notifyComponents(eventType: string, data: any) {
    window.dispatchEvent(new CustomEvent('tihc-extension-update', {
      detail: { type: eventType, data }
    }))
  }

  // 手动同步数据
  async manualSync() {
    await this.updateDataFromBackend()
  }

  // 获取 API 客户端实例
  getApiClient() {
    return this.apiClient
  }
}

// 创建全局实例
export const extensionApiHandler = new ExtensionApiHandler()