// TiHC 扩展弹窗脚本
// 只显示状态信息，不提供配置功能
console.log('[TiHC Popup] 弹窗脚本加载');

// 工具函数：格式化时间
function formatTime(timestamp) {
  if (!timestamp) return '暂无记录';
  return new Date(timestamp).toLocaleString('zh-CN');
}

// 加载页面信息
async function loadPageInfo() {
  try {
    // 获取当前活动标签页
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    if (!tab || !tab.url) {
      document.getElementById('current-domain').textContent = '无法获取';
      return;
    }
    
    const url = new URL(tab.url);
    const domain = url.hostname;
    
    // 发送消息获取页面状态
    chrome.tabs.sendMessage(tab.id, { action: 'GET_STATUS' }, (response) => {
      if (chrome.runtime.lastError) {
        console.log('[TiHC Popup] 无法连接到 content script');
        document.getElementById('current-domain').textContent = domain;
      } else if (response && response.success) {
        const data = response.data;
        document.getElementById('current-domain').textContent = data.domain || domain;
        
        // 更新采集状态
        updateCollectionStatus(data.isCollecting);
        
        if (data.lastCollectionTime) {
          document.getElementById('last-collection').textContent = formatTime(data.lastCollectionTime);
          document.getElementById('last-collection').classList.remove('inactive');
          document.getElementById('last-collection').classList.add('active');
        }
      } else {
        document.getElementById('current-domain').textContent = domain;
      }
    });
    
  } catch (error) {
    console.error('[TiHC Popup] 加载页面信息失败:', error);
    document.getElementById('current-domain').textContent = '获取失败';
  }
}

// 加载扩展状态
async function loadExtensionState() {
  try {
    const response = await new Promise((resolve) => {
      chrome.runtime.sendMessage({ type: 'GET_STATE' }, resolve);
    });
    
    if (response && response.success) {
      const state = response.state;
      
      // 更新统计数据
      document.getElementById('total-collections').textContent = state.collectionCount || 0;
      
      // 计算涉及的域名数量
      const domainCount = state.collectionDomains ? Object.keys(state.collectionDomains).length : 0;
      document.getElementById('total-domains').textContent = domainCount;
      
      // 更新最近采集时间
      if (state.lastCollectionTime) {
        const lastCollectionEl = document.getElementById('last-collection');
        lastCollectionEl.textContent = formatTime(state.lastCollectionTime);
        lastCollectionEl.classList.remove('inactive');
        lastCollectionEl.classList.add('active');
      }
    }
  } catch (error) {
    console.error('[TiHC Popup] 加载扩展状态失败:', error);
  }
}

// 更新采集状态显示
function updateCollectionStatus(isCollecting) {
  const statusEl = document.getElementById('collection-status');
  if (isCollecting) {
    statusEl.textContent = '采集中...';
    statusEl.classList.remove('inactive');
    statusEl.classList.add('warning');
  } else {
    statusEl.textContent = '待命中';
    statusEl.classList.remove('warning', 'active');
    statusEl.classList.add('inactive');
  }
}

// 监听来自 background script 的消息
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  console.log('[TiHC Popup] 收到消息:', message.type);
  
  if (message.type === 'COLLECTION_SUCCESS') {
    // 采集成功，刷新状态
    loadExtensionState();
    updateCollectionStatus(false);
  } else if (message.type === 'COLLECTION_FAILED') {
    // 采集失败
    updateCollectionStatus(false);
  }
});

// 页面加载完成事件
document.addEventListener('DOMContentLoaded', async function() {
  console.log('[TiHC Popup] DOM 加载完成');
  
  try {
    // 并行加载页面信息和扩展状态
    await Promise.all([
      loadPageInfo(),
      loadExtensionState()
    ]);
    
    // 显示主要内容，隐藏加载指示器
    document.getElementById('loading').style.display = 'none';
    document.getElementById('main-content').style.display = 'block';
    
    console.log('[TiHC Popup] 弹窗初始化完成');
    
  } catch (error) {
    console.error('[TiHC Popup] 初始化失败:', error);
    
    // 显示错误信息
    document.getElementById('loading').innerHTML = `
      <div style="color: #dc3545;">
        初始化失败<br>
        <small>${error.message}</small>
      </div>
    `;
  }
});

console.log('[TiHC Popup] 弹窗脚本初始化完成');
