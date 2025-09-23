// API 客户端 - 扩展与前端的 HTTP 通信
class TiHCApiClient {
  constructor() {
    this.timeout = 5000; // 5秒超时
    this.retryAttempts = 3;
    this.serviceDiscovery = window.serviceDiscoveryManager;
  }

  // 获取 API 基础 URL（动态从服务发现获取）
  async getBaseUrl() {
    if (!this.serviceDiscovery.isServerConnected()) {
      throw new Error('TiHC Server 未连接，请等待服务发现完成');
    }
    
    return this.serviceDiscovery.getApiBaseUrl();
  }

  // 等待服务器连接
  async waitForServer(timeout = 30000) {
    try {
      await this.serviceDiscovery.waitForConnection(timeout);
      console.log('[TiHC API Client] 服务器连接成功');
      return true;
    } catch (error) {
      console.error('[TiHC API Client] 等待服务器连接失败:', error);
      return false;
    }
  }

  // 通用 HTTP 请求方法
  async request(endpoint, options = {}) {
    // 确保服务器已连接
    if (!this.serviceDiscovery.isServerConnected()) {
      console.log('[TiHC API] 等待服务器连接...');
      const connected = await this.waitForServer();
      if (!connected) {
        return { success: false, error: 'TiHC Server 连接失败' };
      }
    }

    const baseUrl = await this.getBaseUrl();
    const url = `${baseUrl}${endpoint}`;
    
    const config = {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
        'X-Source': 'tihc-extension'
      },
      ...options
    };

    for (let attempt = 1; attempt <= this.retryAttempts; attempt++) {
      try {
        console.log(`[TiHC API] 请求: ${config.method} ${url} (尝试 ${attempt})`);
        
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), this.timeout);
        
        const response = await fetch(url, {
          ...config,
          signal: controller.signal
        });

        clearTimeout(timeoutId);

        if (!response.ok) {
          throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }

        const data = await response.json();
        console.log(`[TiHC API] 响应成功:`, data);
        return { success: true, data };

      } catch (error) {
        console.error(`[TiHC API] 请求失败 (尝试 ${attempt}):`, error.message);
        
        if (attempt === this.retryAttempts) {
          return { success: false, error: error.message };
        }
        
        // 重试前等待
        await new Promise(resolve => setTimeout(resolve, 1000 * attempt));
      }
    }
  }

  // 发送认证数据到前端
  async sendAuthData(authData) {
    return await this.request('/extension/auth-data', {
      method: 'POST',
      body: JSON.stringify({
        timestamp: Date.now(),
        source: 'extension',
        data: authData
      })
    });
  }

  // 发送实时 token 更新
  async updateTokens(domain, service, tokenData) {
    return await this.request('/extension/tokens', {
      method: 'POST',
      body: JSON.stringify({
        domain,
        service,
        data: tokenData,
        timestamp: Date.now()
      })
    });
  }

  // 心跳检测 - 检查前端是否在线
  async ping() {
    return await this.request('/extension/ping');
  }

  // 获取前端配置
  async getConfig() {
    return await this.request('/extension/config');
  }

  // 批量同步所有数据
  async syncAllData() {
    try {
      const result = await chrome.storage.local.get('tokens');
      return await this.request('/extension/sync', {
        method: 'POST',
        body: JSON.stringify({
          tokens: result.tokens || {},
          timestamp: Date.now(),
          syncType: 'full'
        })
      });
    } catch (error) {
      console.error('[TiHC API] 同步数据失败:', error);
      return { success: false, error: error.message };
    }
  }

  // 检查前端连接状态
  async checkConnection() {
    const result = await this.ping();
    return result.success;
  }
}

// 创建全局 API 客户端实例
window.tiHCApiClient = new TiHCApiClient();