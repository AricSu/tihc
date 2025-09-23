// WebSocket 桥接模块 - 连接扩展与前端
class WebSocketBridge {
  constructor() {
    this.ws = null;
    this.connected = false;
    this.reconnectAttempts = 0;
    this.maxReconnectAttempts = 5;
    this.reconnectDelay = 2000;
    this.frontendUrl = 'ws://localhost:3000/ws'; // 前端 WebSocket 地址
  }

  // 连接到前端 WebSocket
  connect() {
    try {
      this.ws = new WebSocket(this.frontendUrl);
      
      this.ws.onopen = () => {
        console.log('[TiHC Bridge] WebSocket 连接已建立');
        this.connected = true;
        this.reconnectAttempts = 0;
        
        // 发送初始化消息
        this.send({
          type: 'extension_connected',
          timestamp: Date.now(),
          source: 'tihc_extension'
        });
      };

      this.ws.onmessage = (event) => {
        try {
          const data = JSON.parse(event.data);
          this.handleMessage(data);
        } catch (error) {
          console.error('[TiHC Bridge] 消息解析失败:', error);
        }
      };

      this.ws.onclose = () => {
        console.log('[TiHC Bridge] WebSocket 连接关闭');
        this.connected = false;
        this.attemptReconnect();
      };

      this.ws.onerror = (error) => {
        console.error('[TiHC Bridge] WebSocket 错误:', error);
      };

    } catch (error) {
      console.error('[TiHC Bridge] 连接失败:', error);
      this.attemptReconnect();
    }
  }

  // 处理来自前端的消息
  handleMessage(data) {
    console.log('[TiHC Bridge] 收到前端消息:', data);
    
    switch (data.type) {
      case 'request_tokens':
        this.sendTokensToFrontend();
        break;
      case 'request_collection':
        this.triggerCollection();
        break;
      case 'frontend_ready':
        this.sendAllData();
        break;
      default:
        console.log('[TiHC Bridge] 未知消息类型:', data.type);
    }
  }

  // 发送消息到前端
  send(data) {
    if (this.connected && this.ws.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(data));
      return true;
    }
    console.warn('[TiHC Bridge] WebSocket 未连接，消息发送失败');
    return false;
  }

  // 发送 tokens 到前端
  async sendTokensToFrontend() {
    try {
      const result = await chrome.storage.local.get('tokens');
      this.send({
        type: 'tokens_update',
        data: result.tokens || {},
        timestamp: Date.now()
      });
    } catch (error) {
      console.error('[TiHC Bridge] 发送 tokens 失败:', error);
    }
  }

  // 发送所有数据到前端
  async sendAllData() {
    try {
      const result = await chrome.storage.local.get(null);
      this.send({
        type: 'full_data_sync',
        data: result,
        timestamp: Date.now()
      });
    } catch (error) {
      console.error('[TiHC Bridge] 发送全部数据失败:', error);
    }
  }

  // 触发数据收集
  triggerCollection() {
    // 通知 background script 执行收集
    chrome.runtime.sendMessage({
      type: 'trigger_collection',
      source: 'frontend_request'
    });
  }

  // 重连逻辑
  attemptReconnect() {
    if (this.reconnectAttempts < this.maxReconnectAttempts) {
      this.reconnectAttempts++;
      console.log(`[TiHC Bridge] 尝试重连 (${this.reconnectAttempts}/${this.maxReconnectAttempts})`);
      
      setTimeout(() => {
        this.connect();
      }, this.reconnectDelay * this.reconnectAttempts);
    } else {
      console.log('[TiHC Bridge] 达到最大重连次数，停止重连');
    }
  }

  // 断开连接
  disconnect() {
    if (this.ws) {
      this.ws.close();
      this.connected = false;
    }
  }
}

// 创建全局实例
window.webSocketBridge = new WebSocketBridge();