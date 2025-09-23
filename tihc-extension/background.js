// 调试日志函数
function debugLog(message, data = null) {
  console.log(`[TiHC Helper Background] ${message}`, data || '');
}

// 监听消息
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  debugLog('收到消息', { type: message.type, sender: sender.tab?.url });
  
  if (message.type === 'setToken') {
    handleTokenMessage(message, sender);
  } else if (message.type === 'trigger_collection') {
    handleTriggerCollection(message, sender);
  }
  
  // 保持连接活跃
  sendResponse({ success: true });
  return true;
});

// 处理 token 消息
async function handleTokenMessage(message, sender) {
  try {
    const { domain, service, data } = message;
    
    debugLog('处理 Token 消息', { domain, service, dataKeys: Object.keys(data || {}) });
    
    // 获取现有的 tokens
    const result = await chrome.storage.local.get('tokens');
    const tokens = result.tokens || {};
    
    // 确保域名对象存在
    if (!tokens[domain]) {
      tokens[domain] = {};
    }
    
    // 存储服务相关数据
    tokens[domain][service] = {
      ...data,
      lastUpdated: Math.floor(Date.now() / 1000),
      tabId: sender.tab ? sender.tab.id : null,
      tabUrl: sender.tab ? sender.tab.url : null
    };
    
    // 保存到 storage
    await chrome.storage.local.set({ tokens });
    
    debugLog('Token 已保存', {
      domain,
      service,
      timestamp: data.timestamp,
      totalDomains: Object.keys(tokens).length
    });

    // 🔗 发送数据到前端 API
    await sendToFrontendAPI(domain, service, tokens[domain][service]);
    
    // 发送通知给其他部分（如果需要）
    chrome.runtime.sendMessage({
      type: 'tokenUpdated',
      domain,
      service,
      data
    }).catch(() => {
      // 忽略没有监听者的错误
    });
    
  } catch (error) {
    debugLog('保存 Token 失败', error);
  }
}

// 发送数据到前端 API
async function sendToFrontendAPI(domain, service, tokenData) {
  try {
    // 注入 API 客户端并发送数据
    chrome.scripting.executeScript({
      target: { tabId: chrome.tabs.TAB_ID_NONE },
      func: async (domain, service, tokenData) => {
        if (window.tiHCApiClient) {
          const result = await window.tiHCApiClient.updateTokens(domain, service, tokenData);
          console.log('[TiHC Extension] API 发送结果:', result);
        }
      },
      args: [domain, service, tokenData]
    }).catch(() => {
      // 如果直接注入失败，尝试使用 fetch
      sendDirectApiRequest(domain, service, tokenData);
    });
  } catch (error) {
    debugLog('发送到前端 API 失败', error);
  }
}

// 直接发送 API 请求
async function sendDirectApiRequest(domain, service, tokenData) {
  try {
    const response = await fetch('http://localhost:5173/api/extension/tokens', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'X-Source': 'tihc-extension'
      },
      body: JSON.stringify({
        domain,
        service,
        data: tokenData,
        timestamp: Date.now()
      })
    });

    if (response.ok) {
      const result = await response.json();
      debugLog('前端 API 响应成功', result);
    } else {
      debugLog('前端 API 响应失败', response.status);
    }
  } catch (error) {
    debugLog('直接 API 请求失败', error);
  }
}

// 扩展安装时的初始化
chrome.runtime.onInstalled.addListener((details) => {
  debugLog('扩展已安装/更新', details.reason);
  
  // 清理旧版本数据格式（如果需要）
  if (details.reason === 'update') {
    migrateOldData();
  }
  
  // 设置右键菜单（可选）
  createContextMenus();
});

// 创建右键菜单
function createContextMenus() {
  try {
    chrome.contextMenus.create({
      id: 'tihc-collect',
      title: '收集登录信息',
      contexts: ['page']
    });
    
    chrome.contextMenus.create({
      id: 'tihc-export',
      title: '导出所有 Token',
      contexts: ['page']
    });
    
    debugLog('右键菜单已创建');
  } catch (error) {
    debugLog('创建右键菜单失败', error);
  }
}

// 处理右键菜单点击
chrome.contextMenus?.onClicked?.addListener((info, tab) => {
  debugLog('右键菜单点击', info.menuItemId);
  
  if (info.menuItemId === 'tihc-collect') {
    // 在当前页面执行收集脚本
    chrome.scripting.executeScript({
      target: { tabId: tab.id },
      function: () => {
        if (typeof window.collectLoginInfo === 'function') {
          window.collectLoginInfo();
        }
      }
    }).catch(error => {
      debugLog('执行收集脚本失败', error);
    });
  } else if (info.menuItemId === 'tihc-export') {
    // 触发导出功能
    exportAllTokens();
  }
});

// 导出所有 tokens
async function exportAllTokens() {
  try {
    const data = await chrome.storage.local.get('tokens');
    if (data.tokens && Object.keys(data.tokens).length > 0) {
      const json = JSON.stringify(data.tokens, null, 2);
      
      // 创建下载
      const blob = new Blob([json], { type: 'application/json' });
      const url = URL.createObjectURL(blob);
      
      chrome.downloads.download({
        url: url,
        filename: `tihc_tokens_${new Date().toISOString().slice(0, 10)}.json`
      });
      
      debugLog('导出完成');
    }
  } catch (error) {
    debugLog('导出失败', error);
  }
}

// 迁移旧数据格式（兼容性处理）
async function migrateOldData() {
  try {
    const result = await chrome.storage.local.get('tokens');
    const tokens = result.tokens || {};
    
    let needsUpdate = false;
    
    // 检查是否是旧格式的数据
    Object.keys(tokens).forEach(domain => {
      if (tokens[domain].token && !tokens[domain].clinic && !tokens[domain].grafana) {
        // 这是旧格式，需要迁移
        const oldData = tokens[domain];
        tokens[domain] = {
          clinic: {
            apikey: oldData.token,
            timestamp: Math.floor(Date.now() / 1000),
            migrated: true
          }
        };
        needsUpdate = true;
      }
    });
    
    if (needsUpdate) {
      await chrome.storage.local.set({ tokens });
      debugLog('数据格式已迁移');
    }
    
  } catch (error) {
    debugLog('数据迁移失败', error);
  }
}

// 监听标签页更新
chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === 'complete' && tab.url) {
    debugLog('标签页更新完成', tab.url);
    
    // 检查是否是目标网站
    const url = tab.url;
    if (url.includes('clinic.pingcap.com') || 
        url.includes('tidbcloud.com') || 
        url.includes('grafana') ||
        url.includes('pingcap.com')) {
      
      debugLog('检测到目标网站，准备收集信息', url);
      
      // 延迟执行收集，确保页面完全加载
      setTimeout(() => {
        chrome.scripting.executeScript({
          target: { tabId: tabId },
          function: () => {
            if (typeof window.collectLoginInfo === 'function') {
              window.collectLoginInfo();
            }
          }
        }).catch(error => {
          debugLog('自动执行收集失败', error);
        });
      }, 2000);
    }
  }
});

// 处理扩展错误
chrome.runtime.onSuspend.addListener(() => {
  debugLog('扩展即将暂停');
});

chrome.runtime.onStartup.addListener(() => {
  debugLog('扩展启动');
  // 启动后台定期收集任务
  startPeriodicCollection();
});

// 定期收集任务
function startPeriodicCollection() {
  // 每10秒检查是否有目标页面并收集数据
  setInterval(async () => {
    try {
      const tabs = await chrome.tabs.query({});
      
      for (const tab of tabs) {
        if (tab.url && 
            (tab.url.includes('clinic.pingcap.com') || 
             tab.url.includes('tidbcloud.com') || 
             tab.url.includes('grafana') ||
             tab.url.includes('pingcap.com'))) {
          
          debugLog('后台定期收集', { tabId: tab.id, url: tab.url });
          
          // 在目标页面执行收集
          chrome.scripting.executeScript({
            target: { tabId: tab.id },
            function: () => {
              if (typeof window.collectLoginInfo === 'function') {
                window.collectLoginInfo();
              }
            }
          }).catch(error => {
            // 忽略页面未准备好的错误
            if (!error.message.includes('Cannot access')) {
              debugLog('后台收集失败', error);
            }
          });
        }
      }
    } catch (error) {
      debugLog('后台定期收集任务失败', error);
    }
  }, 10000); // 10秒
}

// 扩展安装后立即启动定期收集
startPeriodicCollection();
