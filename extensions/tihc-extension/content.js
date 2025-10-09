// TiHC 浏览器扩展 - 内容脚本 v1.1
// 专门接收 TiHC 前端的指令并委托background执行跨域数据采集
console.log('[TiHC Content] 内容脚本加载 v1.1，域名:', window.location.hostname);

// 标记扩展已安装（供 TiHC 前端检测）
window.TiHCExtensionInstalled = true;
window.TiHCExtensionVersion = '1.1';

// 采集状态
let isCollecting = false;
let lastCollectionTime = null;
let collectionCount = 0;

// 页面类型由 TiHC 前端检测并传入，插件专注于数据采集执行

// 监听来自 TiHC 前端的消息
window.addEventListener('message', function(event) {
  if (event.source !== window) return;
  
  switch (event.data.type) {
    case 'TIHC_EXTENSION_CHECK':
      // 响应扩展检测请求
      console.log('[TiHC Content] 响应扩展检测请求');
      
      window.postMessage({
        type: 'TIHC_EXTENSION_RESPONSE',
        installed: true,
        version: '1.0',
        domain: window.location.hostname,
        url: window.location.href
      }, '*');
      break;
      
    case 'TIHC_START_COLLECTION':
      // 开始数据采集（页面类型由前端传入）
      console.log('[TiHC Content] 收到采集指令:', event.data);
      if (event.data.pageType) {
        handleCollectionRequest(event.data.config, event.data.pageType);
      } else {
        console.error('[TiHC Content] 缺少页面类型信息');
        window.postMessage({
          type: 'TIHC_COLLECTION_ERROR',
          error: '缺少页面类型信息'
        }, '*');
      }
      break;
      
    case 'TIHC_STOP_COLLECTION':
      // 停止数据采集
      console.log('[TiHC Content] 收到停止采集指令');
      isCollecting = false;
      window.postMessage({
        type: 'TIHC_COLLECTION_STOPPED'
      }, '*');
      break;
      
    case 'TIHC_GET_STATUS':
      // 获取采集状态
      window.postMessage({
        type: 'TIHC_COLLECTION_STATUS',
        isCollecting: isCollecting,
        lastCollectionTime: lastCollectionTime,
        collectionCount: collectionCount,
        domain: window.location.hostname,
        url: window.location.href
      }, '*');
      break;
  }
});

// 处理采集请求
// 处理采集请求
async function handleCollectionRequest(config, pageType) {
  try {
    if (!config || !config.backendUrl) {
      console.error('[TiHC Content] 配置无效，缺少后端地址');
      window.postMessage({
        type: 'TIHC_COLLECTION_ERROR',
        error: '配置无效，缺少后端地址'
      }, '*');
      return;
    }
    
    const targetUrl = config.targetUrl || window.location.href;
    
    console.log('[TiHC Content] 开始采集目标URL数据:', targetUrl);
    console.log('[TiHC Content] 页面类型:', pageType);
    
    isCollecting = true;
    
    // 使用background service worker进行跨域采集
    console.log('[TiHC Content] 请求background进行跨域采集');
    
    const response = await new Promise((resolve, reject) => {
      chrome.runtime.sendMessage({
        type: 'COLLECT_TARGET_DATA',
        targetUrl: targetUrl,
        pageType: pageType,
        backendUrl: config.backendUrl
      }, (response) => {
        if (chrome.runtime.lastError) {
          reject(new Error(chrome.runtime.lastError.message));
        } else {
          console.log('[TiHC Content] Background采集响应:', response);
          resolve(response);
        }
      });
    });
    
    if (response.success) {
      collectionCount++;
      lastCollectionTime = Date.now();
      
      console.log('[TiHC Content] 跨域采集成功:', response);
      window.postMessage({
        type: 'TIHC_COLLECTION_SUCCESS',
        data: {
          pageType: pageType,
          domain: response.data.domain,
          timestamp: lastCollectionTime,
          count: collectionCount,
          targetUrl: targetUrl,
          cookieCount: response.data.cookieCount,
          method: 'background_api'
        }
      }, '*');
      
    } else {
      console.error('[TiHC Content] 跨域采集失败:', response.error);
      window.postMessage({
        type: 'TIHC_COLLECTION_ERROR',
        error: `跨域采集失败: ${response.error}`
      }, '*');
    }
    
  } catch (error) {
    console.error('[TiHC Content] 采集处理异常:', error);
    window.postMessage({
      type: 'TIHC_COLLECTION_ERROR',
      error: `采集处理异常: ${error.message}`
    }, '*');
  } finally {
    isCollecting = false;
  }
}

// 判断是否为认证相关的 key
function isAuthRelatedKey(key) {
  const authKeywords = [
    'auth', 'token', 'session', 'login', 'user', 'jwt', 'csrf',
    'grafana', 'clinic', 'tidb', 'pingcap', 'api', 'key', 'secret'
  ];
  
  const lowerKey = key.toLowerCase();
  return authKeywords.some(keyword => lowerKey.includes(keyword));
}

// 监听来自 popup 的消息
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message.action === 'GET_STATUS') {
    sendResponse({
      success: true,
      data: {
        isCollecting: isCollecting,
        lastCollectionTime: lastCollectionTime,
        collectionCount: collectionCount,
        domain: window.location.hostname,
        url: window.location.href
      }
    });
  } else if (message.type === 'GET_STORAGE_DATA') {
    // background请求获取当前页面的存储数据
    console.log('[TiHC Content] 收到获取存储数据请求');
    
    const storageData = {
      localStorage: {},
      sessionStorage: {}
    };
    
    // 采集 localStorage
    try {
      for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i);
        if (key && isAuthRelatedKey(key)) {
          storageData.localStorage[key] = localStorage.getItem(key);
        }
      }
    } catch (e) {
      console.warn('[TiHC Content] 无法访问 localStorage:', e);
    }
    
    // 采集 sessionStorage
    try {
      for (let i = 0; i < sessionStorage.length; i++) {
        const key = sessionStorage.key(i);
        if (key && isAuthRelatedKey(key)) {
          storageData.sessionStorage[key] = sessionStorage.getItem(key);
        }
      }
    } catch (e) {
      console.warn('[TiHC Content] 无法访问 sessionStorage:', e);
    }
    
    console.log('[TiHC Content] 返回存储数据:', storageData);
    sendResponse(storageData);
    return true; // 异步响应
  }
});

console.log('[TiHC Content] 内容脚本初始化完成，等待 TiHC 前端指令');