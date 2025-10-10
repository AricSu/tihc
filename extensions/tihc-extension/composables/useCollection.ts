import { ref } from 'vue';

// 状态管理
export const isCollecting = ref(false);
export const lastCollectionTime = ref<number | null>(null);
export const collectionCount = ref(0);

// 统一消息分发
export function postToWindow(type: string, data: any = {}) {
  window.postMessage({ type, ...data }, '*');
}

// 认证相关 key 检查
export function isAuthRelatedKey(key: string) {
  const authKeywords = [
    'auth', 'token', 'session', 'login', 'user', 'jwt', 'csrf',
    'grafana', 'clinic', 'tidb', 'pingcap', 'api', 'key', 'secret'
  ];
  const lowerKey = key.toLowerCase();
  return authKeywords.some(keyword => lowerKey.includes(keyword));
}

// 存储数据采集
export function getStorageData() {
  const storageData: { localStorage: Record<string, string>, sessionStorage: Record<string, string> } = {
    localStorage: {},
    sessionStorage: {}
  };
  try {
    for (let i = 0; i < localStorage.length; i++) {
      const key = localStorage.key(i);
      if (key && isAuthRelatedKey(key)) {
        storageData.localStorage[key] = localStorage.getItem(key)!;
      }
    }
  } catch (e) {}
  try {
    for (let i = 0; i < sessionStorage.length; i++) {
      const key = sessionStorage.key(i);
      if (key && isAuthRelatedKey(key)) {
        storageData.sessionStorage[key] = sessionStorage.getItem(key)!;
      }
    }
  } catch (e) {}
  return storageData;
}

// 采集主流程
export async function startCollection(config: any, pageType: string) {
  if (!config || !config.backendUrl) {
    postToWindow('TIHC_COLLECTION_ERROR', { error: '配置无效，缺少后端地址' });
    return;
  }
  const targetUrl = config.targetUrl || window.location.href;
  isCollecting.value = true;
  try {
    const response = await sendCollectRequest(targetUrl, pageType, config.backendUrl);
    if (response.success) {
      collectionCount.value++;
      lastCollectionTime.value = Date.now();
      postToWindow('TIHC_COLLECTION_SUCCESS', {
        pageType,
        domain: response.data.domain,
        timestamp: lastCollectionTime.value,
        count: collectionCount.value,
        targetUrl,
        cookieCount: response.data.cookieCount,
        method: 'background_api'
      });
    } else {
      postToWindow('TIHC_COLLECTION_ERROR', { error: `跨域采集失败: ${response.error}` });
    }
  } catch (error: any) {
    postToWindow('TIHC_COLLECTION_ERROR', { error: `采集处理异常: ${error.message}` });
  } finally {
    isCollecting.value = false;
  }
}

// 采集请求
export function sendCollectRequest(targetUrl: string, pageType: string, backendUrl: string): Promise<any> {
  return new Promise((resolve, reject) => {
    chrome.runtime.sendMessage({
      type: 'COLLECT_TARGET_DATA',
      targetUrl,
      pageType,
      backendUrl
    }, (response) => {
      if (chrome.runtime.lastError) {
        reject(new Error(chrome.runtime.lastError.message));
      } else {
        resolve(response);
      }
    });
  });
}
