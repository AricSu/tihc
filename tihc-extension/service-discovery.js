// 服务发现管理器 - 等待 TiHC Server 连接
class ServiceDiscoveryManager {
  constructor() {
    this.serverInfo = null;
    this.isConnected = false;
    this.connectionCallback = null;
    this.setupMessageListener();
    
    console.log('[TiHC Extension] 等待 TiHC Server 连接...');
  }

  // 设置消息监听器，等待 Server 连接
  setupMessageListener() {
    // 监听来自页面的消息（Server 通过页面脚本注入发送）
    window.addEventListener('message', (event) => {
      if (event.data.type === 'tihc_server_handshake') {
        this.handleServerHandshake(event.data);
      }
    });

    // 监听来自 background script 的消息
    if (chrome && chrome.runtime) {
      chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
        if (message.type === 'server_discovered') {
          this.handleServerDiscovery(message);
          sendResponse({ success: true });
        }
        return true;
      });
    }
  }

  // 处理 Server 握手
  handleServerHandshake(data) {
    console.log('[TiHC Extension] 收到 TiHC Server 握手:', data);
    
    const { serverUrl, port, version, capabilities } = data;
    
    this.serverInfo = {
      url: serverUrl,
      port: port,
      baseUrl: `${serverUrl}:${port}`,
      version: version,
      capabilities: capabilities || [],
      connectedAt: Date.now()
    };

    this.isConnected = true;
    
    // 回应握手
    this.sendHandshakeResponse();
    
    // 触发连接回调
    if (this.connectionCallback) {
      this.connectionCallback(this.serverInfo);
    }

    console.log('[TiHC Extension] TiHC Server 连接成功:', this.serverInfo);
  }

  // 发送握手响应
  sendHandshakeResponse() {
    window.postMessage({
      type: 'tihc_extension_handshake_response',
      success: true,
      extensionInfo: {
        version: chrome.runtime.getManifest().version,
        capabilities: ['token_collection', 'auto_sync', 'clinic_support', 'grafana_support'],
        id: chrome.runtime.id
      }
    }, '*');
  }

  // 处理服务发现
  handleServerDiscovery(message) {
    const { serverUrl, port } = message;
    
    this.serverInfo = {
      url: serverUrl,
      port: port,
      baseUrl: `${serverUrl}:${port}`,
      discoveredAt: Date.now()
    };

    this.isConnected = true;
    console.log('[TiHC Extension] 通过服务发现连接到 Server:', this.serverInfo);
  }

  // 等待服务器连接
  waitForConnection(timeout = 30000) {
    return new Promise((resolve, reject) => {
      if (this.isConnected) {
        resolve(this.serverInfo);
        return;
      }

      this.connectionCallback = resolve;

      // 设置超时
      setTimeout(() => {
        if (!this.isConnected) {
          reject(new Error('等待 TiHC Server 连接超时'));
        }
      }, timeout);
    });
  }

  // 获取服务器信息
  getServerInfo() {
    return this.serverInfo;
  }

  // 检查连接状态
  isServerConnected() {
    return this.isConnected && this.serverInfo !== null;
  }

  // 获取 API 基础 URL
  getApiBaseUrl() {
    if (!this.isConnected || !this.serverInfo) {
      throw new Error('TiHC Server 未连接');
    }
    return `${this.serverInfo.baseUrl}/api`;
  }

  // 断开连接
  disconnect() {
    this.serverInfo = null;
    this.isConnected = false;
    this.connectionCallback = null;
    console.log('[TiHC Extension] 与 TiHC Server 断开连接');
  }

  // 测试连接
  async testConnection() {
    if (!this.isConnected) {
      return false;
    }

    try {
      const response = await fetch(`${this.getApiBaseUrl()}/extension/ping`, {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
          'X-Source': 'tihc-extension'
        }
      });

      return response.ok;
    } catch (error) {
      console.error('[TiHC Extension] 连接测试失败:', error);
      return false;
    }
  }
}

// 创建全局服务发现管理器
window.serviceDiscoveryManager = new ServiceDiscoveryManager();