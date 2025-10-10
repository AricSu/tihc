// TiHC 扩展 background script v2 (WXT/TypeScript)
import { runtime, storage, tabs, cookies } from 'wxt/browser';

console.log('[TiHC Background] Service Worker 启动 v2');

interface ExtensionState {
  isActive: boolean;
  lastCollectionTime: number | null;
  collectionCount: number;
  collectionDomains: Record<string, {
    count: number;
    pageType: string;
    lastTime: number;
  }>;
}

let extensionState: ExtensionState = {
  isActive: true,
  lastCollectionTime: null,
  collectionCount: 0,
  collectionDomains: {}
};

runtime.onInstalled.addListener((details) => {
  console.log('[TiHC Background] 扩展安装/更新:', details.reason);
  storage.local.set({ extensionState });
});

runtime.onMessage.addListener((message, sender, sendResponse) => {
  console.log('[TiHC Background] 收到消息:', message.type);
  switch (message.type) {
    case 'COLLECT_TARGET_DATA':
      collectTargetUrlData(message.targetUrl, message.pageType, message.backendUrl)
        .then(sendResponse)
        .catch(error => sendResponse({ success: false, error: error.message }));
      return true;
    case 'GET_STATE':
      storage.local.get('extensionState').then(result => {
        sendResponse({ success: true, state: result.extensionState || extensionState });
      });
      return true;
    case 'UPDATE_STATE':
      extensionState = { ...extensionState, ...message.state };
      storage.local.set({ extensionState }).then(() => {
        console.log('[TiHC Background] 状态已更新:', extensionState);
        sendResponse({ success: true });
      });
      return true;
    case 'COLLECTION_SUCCESS':
      updateCollectionStats(message.domain, message.pageType);
      sendResponse({ success: true });
      break;
    case 'COLLECTION_FAILED':
      console.log('[TiHC Background] 采集失败:', message);
      sendResponse({ success: true });
      break;
    default:
      sendResponse({ success: false, error: '未知消息类型' });
  }
});

async function collectTargetUrlData(targetUrl: string, pageType: string, backendUrl: string) {
  try {
    const urlObj = new URL(targetUrl);
    const domain = urlObj.hostname;
    // 获取 cookies
    const cookiesArr = await cookies.getAll({ domain });
    const subDomainCookies = await cookies.getAll({ domain: '.' + domain });
    const cookieMap = new Map<string, chrome.cookies.Cookie>();
    [...cookiesArr, ...subDomainCookies].forEach(cookie => {
      const key = `${cookie.name}_${cookie.domain}_${cookie.path}`;
      cookieMap.set(key, cookie);
    });
    const allCookies = Array.from(cookieMap.values());
    // 获取当前标签页存储数据
    let localStorage: Record<string, string> = {};
    let sessionStorage: Record<string, string> = {};
    try {
      const tabArr = await tabs.query({ active: true, currentWindow: true });
      if (tabArr.length > 0) {
        const currentTab = tabArr[0];
        const currentUrl = new URL(currentTab.url || '');
        if (currentUrl.hostname === domain) {
          const storageData = await tabs.sendMessage(currentTab.id!, { type: 'GET_STORAGE_DATA' });
          localStorage = storageData.localStorage || {};
          sessionStorage = storageData.sessionStorage || {};
        }
      }
    } catch (error) {
      console.log('[TiHC Background] 获取存储数据时出错:', error);
    }
    const collectedData = {
      url: targetUrl,
      domain,
      timestamp: Date.now(),
      task_id: `tihc-${Date.now()}`,
      page_type: pageType || 'unknown',
      user_agent: navigator.userAgent,
      cookies: allCookies.map(c => `${c.name}=${c.value}`).join('; '),
      local_storage: localStorage,
      session_storage: sessionStorage,
      collection_method: 'background_api'
    };
    const sendResult = await sendDataToBackend(collectedData, backendUrl);
    if (sendResult.success) {
      updateCollectionStats(domain, pageType);
      return {
        success: true,
        message: `成功采集 ${domain} 的数据`,
        data: {
          domain,
          pageType,
          cookieCount: allCookies.length,
          timestamp: collectedData.timestamp
        }
      };
    } else {
      throw new Error(sendResult.error);
    }
  } catch (error: any) {
    return { success: false, error: error.message };
  }
}

async function sendDataToBackend(data: any, backendUrl: string) {
  try {
    const requestUrl = `${backendUrl}/api/collect`;
    const response = await fetch(requestUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data)
    });
    if (response.ok) {
      const responseData = await response.json();
      return { success: true, data: responseData };
    } else {
      const errorText = await response.text();
      return { success: false, error: `HTTP ${response.status}: ${errorText}` };
    }
  } catch (error: any) {
    return { success: false, error: `网络错误: ${error.message}` };
  }
}

function updateCollectionStats(domain: string, pageType: string) {
  storage.local.get('extensionState').then(result => {
    const state: ExtensionState = result.extensionState || extensionState;
    state.lastCollectionTime = Date.now();
    state.collectionCount = (state.collectionCount || 0) + 1;
    state.collectionDomains = state.collectionDomains || {};
    state.collectionDomains[domain] = {
      count: (state.collectionDomains[domain]?.count || 0) + 1,
      pageType,
      lastTime: Date.now()
    };
    extensionState = state;
    storage.local.set({ extensionState });
  });
}

runtime.onMessageExternal.addListener((message, sender, sendResponse) => {
  if (message.type === 'TIHC_EXTENSION_CHECK') {
    sendResponse({ installed: true, version: '2.0' });
  }
});

console.log('[TiHC Background] Service Worker 初始化完成');