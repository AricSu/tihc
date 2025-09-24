// TiHC 扩展 background script v1.1
// 支持跨域cookie采集，无需打开目标页面
console.log('[TiHC Background] Service Worker 启动 v1.1');

// 扩展状态
let extensionState = {
  isActive: true,
  lastCollectionTime: null,
  collectionCount: 0,
  collectionDomains: {}
};

// 扩展安装时
chrome.runtime.onInstalled.addListener(function(details) {
  console.log('[TiHC Background] 扩展安装/更新:', details.reason);
  
  // 初始化状态
  chrome.storage.local.set({
    extensionState: extensionState
  }, function() {
    console.log('[TiHC Background] 初始状态已设置');
  });
});

// 监听消息
chrome.runtime.onMessage.addListener(function(message, sender, sendResponse) {
  console.log('[TiHC Background] 收到消息:', message.type);
  
  switch (message.type) {
    case 'COLLECT_TARGET_DATA':
      // 采集指定URL的数据
      collectTargetUrlData(message.targetUrl, message.pageType, message.backendUrl)
        .then(result => sendResponse(result))
        .catch(error => sendResponse({ success: false, error: error.message }));
      return true; // 异步响应
      
    case 'GET_STATE':
      // 获取扩展状态
      chrome.storage.local.get(['extensionState'], function(result) {
        const state = result.extensionState || extensionState;
        sendResponse({ success: true, state: state });
      });
      return true; // 异步响应
      
    case 'UPDATE_STATE':
      // 更新扩展状态
      const newState = { ...extensionState, ...message.state };
      extensionState = newState;
      
      chrome.storage.local.set({
        extensionState: newState
      }, function() {
        console.log('[TiHC Background] 状态已更新:', newState);
        sendResponse({ success: true });
      });
      return true; // 异步响应
      
    case 'COLLECTION_SUCCESS':
      // 记录采集成功
      updateCollectionStats(message.domain, message.pageType);
      sendResponse({ success: true });
      break;
      
    case 'COLLECTION_FAILED':
      // 记录采集失败
      console.log('[TiHC Background] 采集失败:', message);
      sendResponse({ success: true });
      break;
      
    default:
      sendResponse({ success: false, error: '未知消息类型' });
  }
});

// 采集指定URL的数据（使用chrome.cookies API）
async function collectTargetUrlData(targetUrl, pageType, backendUrl) {
  try {
    console.log('[TiHC Background] 开始采集目标URL数据:', targetUrl);
    
    const urlObj = new URL(targetUrl);
    const domain = urlObj.hostname;
    
    // 使用chrome.cookies API获取目标域名的所有cookies
    const cookies = await getCookiesForDomain(domain);
    console.log('[TiHC Background] 获取到cookies:', cookies.length, '条');
    
    // 尝试获取当前活跃标签页的存储数据（如果是同域）
    let localStorage = {};
    let sessionStorage = {};
    
    try {
      // 查询当前活跃的标签页
      const tabs = await chrome.tabs.query({active: true, currentWindow: true});
      if (tabs.length > 0) {
        const currentTab = tabs[0];
        const currentUrl = new URL(currentTab.url);
        
        // 如果当前标签页和目标URL是同域，可以获取存储数据
        if (currentUrl.hostname === domain) {
          console.log('[TiHC Background] 目标域名与当前页面同域，尝试获取存储数据');
          
          // 向content script请求存储数据
          const storageData = await new Promise((resolve) => {
            chrome.tabs.sendMessage(currentTab.id, {
              type: 'GET_STORAGE_DATA'
            }, (response) => {
              if (chrome.runtime.lastError) {
                console.log('[TiHC Background] 获取存储数据失败:', chrome.runtime.lastError.message);
                resolve({});
              } else {
                resolve(response || {});
              }
            });
          });
          
          localStorage = storageData.localStorage || {};
          sessionStorage = storageData.sessionStorage || {};
          console.log('[TiHC Background] 获取到存储数据:', Object.keys(localStorage).length, '个localStorage项');
        }
      }
    } catch (error) {
      console.log('[TiHC Background] 获取存储数据时出错:', error);
    }
    
    // 构建采集数据
    const collectedData = {
      url: targetUrl,
      domain: domain,
      timestamp: Date.now(),
      task_id: `tihc-${Date.now()}`,
      page_type: pageType || 'unknown',
      user_agent: navigator.userAgent,
      cookies: formatCookies(cookies),
      local_storage: localStorage,
      session_storage: sessionStorage,
      collection_method: 'background_api' // 标记采集方式
    };
    
    console.log('[TiHC Background] 采集到的数据:', collectedData);
    
    // 发送到后端
    const sendResult = await sendDataToBackend(collectedData, backendUrl);
    
    if (sendResult.success) {
      // 更新统计
      updateCollectionStats(domain, pageType);
      
      return {
        success: true,
        message: `成功采集 ${domain} 的数据`,
        data: {
          domain: domain,
          pageType: pageType,
          cookieCount: cookies.length,
          timestamp: collectedData.timestamp
        }
      };
    } else {
      throw new Error(sendResult.error);
    }
    
  } catch (error) {
    console.error('[TiHC Background] 采集失败:', error);
    return { success: false, error: error.message };
  }
}

// 获取指定域名的cookies
async function getCookiesForDomain(domain) {
  try {
    // 获取所有相关的cookies
    const allCookies = await chrome.cookies.getAll({
      domain: domain
    });
    
    // 也获取子域名的cookies
    const subDomainCookies = await chrome.cookies.getAll({
      domain: '.' + domain
    });
    
    // 合并并去重
    const cookieMap = new Map();
    [...allCookies, ...subDomainCookies].forEach(cookie => {
      const key = `${cookie.name}_${cookie.domain}_${cookie.path}`;
      cookieMap.set(key, cookie);
    });
    
    return Array.from(cookieMap.values());
    
  } catch (error) {
    console.error('[TiHC Background] 获取cookies失败:', error);
    return [];
  }
}

// 格式化cookies为字符串
function formatCookies(cookies) {
  return cookies.map(cookie => {
    return `${cookie.name}=${cookie.value}`;
  }).join('; ');
}

// 发送数据到后端
async function sendDataToBackend(data, backendUrl) {
  try {
    const requestUrl = `${backendUrl}/api/collect`;
    console.log('[TiHC Background] 发送请求到:', requestUrl);
    
    const response = await fetch(requestUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data)
    });
    
    console.log('[TiHC Background] 响应状态:', response.status, response.statusText);
    
    if (response.ok) {
      const responseData = await response.json();
      console.log('[TiHC Background] 数据发送成功:', responseData);
      return { success: true, data: responseData };
    } else {
      const errorText = await response.text();
      console.error('[TiHC Background] 数据发送失败:', response.status, errorText);
      return { 
        success: false, 
        error: `HTTP ${response.status}: ${errorText}` 
      };
    }
  } catch (error) {
    console.error('[TiHC Background] 网络请求失败:', error);
    return { 
      success: false, 
      error: `网络错误: ${error.message}` 
    };
  }
}

// 更新采集统计
function updateCollectionStats(domain, pageType) {
  chrome.storage.local.get(['extensionState'], function(result) {
    const state = result.extensionState || extensionState;
    
    // 更新统计
    state.lastCollectionTime = Date.now();
    state.collectionCount = (state.collectionCount || 0) + 1;
    state.collectionDomains = state.collectionDomains || {};
    state.collectionDomains[domain] = {
      count: (state.collectionDomains[domain]?.count || 0) + 1,
      pageType: pageType,
      lastTime: Date.now()
    };
    
    // 保存状态
    extensionState = state;
    chrome.storage.local.set({
      extensionState: state
    }, function() {
      console.log('[TiHC Background] 采集统计已更新:', {
        domain,
        pageType,
        totalCount: state.collectionCount
      });
    });
  });
}

// 获取扩展状态（供 popup 使用）
function getExtensionState() {
  return extensionState;
}

console.log('[TiHC Background] Service Worker 初始化完成');

// 处理数据采集
async function handleDataCollection(data) {
  try {
    if (!config.backend) {
      throw new Error('后端地址未配置');
    }
    
    console.log('[TiHC Extension] 处理数据采集:', data.domain);
    
    // 发送到后端
    const response = await fetch(`${config.backend}/api/collect`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data)
    });
    
    if (response.ok) {
      console.log('[TiHC Extension] 数据发送成功');
      return { success: true, message: '数据发送成功' };
    } else {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }
  } catch (error) {
    console.error('[TiHC Extension] 数据发送失败:', error);
    return { success: false, error: error.message };
  }
}

// 扩展启动时加载配置
chrome.storage.local.get('config', (result) => {
  if (result.config) {
    config = result.config;
    console.log('[TiHC Extension] 配置已从存储加载:', config);
  }
});

// 监听来自前端页面的外部消息 (postMessage)
chrome.runtime.onMessageExternal.addListener((message, sender, sendResponse) => {
  if (message.type === 'TIHC_EXTENSION_CHECK') {
    sendResponse({ installed: true, version: '1.0' });
  }
});

console.log('[TiHC Extension] Background script 初始化完成');