// 只在浏览器环境下执行内容脚本逻辑
if (typeof window !== 'undefined' && typeof document !== 'undefined') {
  // TiHC 浏览器扩展 - 内容脚本 v2 (WXT/TypeScript)
  console.log('[TiHC Content] 内容脚本加载 v2，域名:', window.location.hostname);

  (window as any).TiHCExtensionInstalled = true;
  (window as any).TiHCExtensionVersion = '2.0';

  let isCollecting = false;
  let lastCollectionTime: number | null = null;
  let collectionCount = 0;

  window.addEventListener('message', function(event) {
    if (event.source !== window) return;
    switch (event.data.type) {
      case 'TIHC_EXTENSION_CHECK':
        window.postMessage({
          type: 'TIHC_EXTENSION_RESPONSE',
          installed: true,
          version: '2.0',
          domain: window.location.hostname,
          url: window.location.href
        }, '*');
        break;
      case 'TIHC_START_COLLECTION':
        if (event.data.pageType) {
          handleCollectionRequest(event.data.config, event.data.pageType);
        } else {
          window.postMessage({
            type: 'TIHC_COLLECTION_ERROR',
            error: '缺少页面类型信息'
          }, '*');
        }
        break;
      case 'TIHC_STOP_COLLECTION':
        isCollecting = false;
        window.postMessage({ type: 'TIHC_COLLECTION_STOPPED' }, '*');
        break;
      case 'TIHC_GET_STATUS':
        window.postMessage({
          type: 'TIHC_COLLECTION_STATUS',
          isCollecting,
          lastCollectionTime,
          collectionCount,
          domain: window.location.hostname,
          url: window.location.href
        }, '*');
        break;
    }
  });

  async function handleCollectionRequest(config: any, pageType: string) {
    try {
      if (!config || !config.backendUrl) {
        window.postMessage({
          type: 'TIHC_COLLECTION_ERROR',
          error: '配置无效，缺少后端地址'
        }, '*');
        return;
      }
      const targetUrl = config.targetUrl || window.location.href;
      isCollecting = true;
      const response = await new Promise<any>((resolve, reject) => {
        chrome.runtime.sendMessage({
          type: 'COLLECT_TARGET_DATA',
          targetUrl,
          pageType,
          backendUrl: config.backendUrl
        }, (response) => {
          if (chrome.runtime.lastError) {
            reject(new Error(chrome.runtime.lastError.message));
          } else {
            resolve(response);
          }
        });
      });
      if (response.success) {
        collectionCount++;
        lastCollectionTime = Date.now();
        window.postMessage({
          type: 'TIHC_COLLECTION_SUCCESS',
          data: {
            pageType,
            domain: response.data.domain,
            timestamp: lastCollectionTime,
            count: collectionCount,
            targetUrl,
            cookieCount: response.data.cookieCount,
            method: 'background_api'
          }
        }, '*');
      } else {
        window.postMessage({
          type: 'TIHC_COLLECTION_ERROR',
          error: `跨域采集失败: ${response.error}`
        }, '*');
      }
    } catch (error: any) {
      window.postMessage({
        type: 'TIHC_COLLECTION_ERROR',
        error: `采集处理异常: ${error.message}`
      }, '*');
    } finally {
      isCollecting = false;
    }
  }

  function isAuthRelatedKey(key: string) {
    const authKeywords = [
      'auth', 'token', 'session', 'login', 'user', 'jwt', 'csrf',
      'grafana', 'clinic', 'tidb', 'pingcap', 'api', 'key', 'secret'
    ];
    const lowerKey = key.toLowerCase();
    return authKeywords.some(keyword => lowerKey.includes(keyword));
  }

  chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
    if (message.action === 'GET_STATUS') {
      sendResponse({
        success: true,
        data: {
          isCollecting,
          lastCollectionTime,
          collectionCount,
          domain: window.location.hostname,
          url: window.location.href
        }
      });
    } else if (message.type === 'GET_STORAGE_DATA') {
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
      sendResponse(storageData);
      return true;
    }
  });

  console.log('[TiHC Content] 内容脚本初始化完成，等待 TiHC 前端指令');
}