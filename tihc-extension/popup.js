// 工具函数：显示状态消息
function showStatus(message, type = 'info') {
  const statusEl = document.getElementById('status');
  statusEl.textContent = message;
  statusEl.className = `status ${type}`;
  statusEl.style.display = 'block';
  
  setTimeout(() => {
    statusEl.style.display = 'none';
  }, 5000);
}

// 工具函数：格式化时间戳
function formatTimestamp(timestamp) {
  if (!timestamp) return '未知';
  const date = new Date(timestamp * 1000);
  return date.toLocaleString('zh-CN');
}

// 工具函数：截断长文本
function truncateText(text, maxLength = 100) {
  if (!text) return '';
  if (text.length <= maxLength) return text;
  return text.substring(0, maxLength) + '...';
}

// 获取当前活动标签页信息
async function getCurrentTab() {
  try {
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    return tab;
  } catch (error) {
    console.error('获取当前标签页失败:', error);
    return null;
  }
}

// 手动触发 content script 收集信息
async function triggerCollection() {
  try {
    const tab = await getCurrentTab();
    if (!tab) {
      showStatus('无法获取当前标签页', 'error');
      return;
    }

    showStatus('正在当前页面收集信息...', 'info');

    // 注入并执行 content script
    await chrome.scripting.executeScript({
      target: { tabId: tab.id },
      function: () => {
        // 发送消息触发收集
        window.postMessage({ type: 'TIHC_TRIGGER_COLLECTION' }, '*');
        
        // 如果有 collectLoginInfo 函数，直接调用
        if (typeof window.collectLoginInfo === 'function') {
          window.collectLoginInfo();
        }
      }
    });

    setTimeout(() => {
      refreshInfo();
    }, 2000);

  } catch (error) {
    console.error('触发收集失败:', error);
    showStatus('触发收集失败: ' + error.message, 'error');
  }
}

// 渲染 Token 信息
function renderTokenInfo(tokens) {
  const container = document.getElementById('token-info');
  
  if (!tokens || Object.keys(tokens).length === 0) {
    container.innerHTML = `
      <div class="empty-state">
        <p>暂无登录信息</p>
        <p style="font-size: 12px; color: #666; margin-top: 10px;">
          请确保：<br>
          1. 已访问目标网站并登录<br>
          2. 网站包含认证信息<br>
          3. 点击"手动收集"按钮
        </p>
      </div>`;
    return;
  }
  
  let html = '';
  
  Object.entries(tokens).forEach(([domain, data]) => {
    html += `
      <div class="info-item">
        <div class="info-label">🌐 ${domain}</div>
        
        ${data.clinic ? `
          <div style="margin-left: 15px; margin-top: 10px;">
            <div class="info-label">🏥 Clinic 信息</div>
            ${data.clinic.session_id ? `
              <div style="margin-bottom: 8px;">
                <strong>Session ID:</strong>
                <div class="info-value">${truncateText(data.clinic.session_id, 50)}</div>
              </div>
            ` : ''}
            ${data.clinic.csrf_token ? `
              <div style="margin-bottom: 8px;">
                <strong>CSRF Token:</strong>
                <div class="info-value">${truncateText(data.clinic.csrf_token, 50)}</div>
              </div>
            ` : ''}
            ${data.clinic.apikey ? `
              <div style="margin-bottom: 8px;">
                <strong>API Key:</strong>
                <div class="info-value">${truncateText(data.clinic.apikey, 50)}</div>
              </div>
            ` : ''}
            ${data.clinic.cookie ? `
              <div style="margin-bottom: 8px;">
                <strong>完整 Cookie:</strong>
                <div class="info-value">${truncateText(data.clinic.cookie, 100)}</div>
              </div>
            ` : ''}
            ${data.clinic.relevantCookies ? `
              <div style="margin-bottom: 8px;">
                <strong>相关 Cookies:</strong>
                <div class="info-value">${data.clinic.relevantCookies.join(', ')}</div>
              </div>
            ` : ''}
            ${data.clinic.relevantLocalStorage ? `
              <div style="margin-bottom: 8px;">
                <strong>LocalStorage 数据:</strong>
                <div class="info-value">${data.clinic.relevantLocalStorage.join(', ')}</div>
              </div>
            ` : ''}
            ${data.clinic.otherCookies ? `
              <div style="margin-bottom: 8px;">
                <strong>其他认证 Cookies:</strong>
                <div class="info-value">${data.clinic.otherCookies.join(', ')}</div>
              </div>
            ` : ''}
            ${data.clinic.timestamp ? `
              <div style="margin-bottom: 8px;">
                <strong>获取时间:</strong>
                <div class="info-value">${formatTimestamp(data.clinic.timestamp)}</div>
              </div>
            ` : ''}
          </div>
        ` : ''}
        
        ${data.grafana ? `
          <div style="margin-left: 15px; margin-top: 10px;">
            <div class="info-label">📊 Grafana 信息</div>
            ${data.grafana.session ? `
              <div style="margin-bottom: 8px;">
                <strong>Session:</strong>
                <div class="info-value">${truncateText(data.grafana.session, 100)}</div>
              </div>
            ` : ''}
            ${data.grafana.auth_token ? `
              <div style="margin-bottom: 8px;">
                <strong>Auth Token:</strong>
                <div class="info-value">${truncateText(data.grafana.auth_token, 100)}</div>
              </div>
            ` : ''}
            ${data.grafana.relevantCookies ? `
              <div style="margin-bottom: 8px;">
                <strong>相关 Cookies:</strong>
                <div class="info-value">${data.grafana.relevantCookies.join(', ')}</div>
              </div>
            ` : ''}
            ${data.grafana.timestamp ? `
              <div style="margin-bottom: 8px;">
                <strong>获取时间:</strong>
                <div class="info-value">${formatTimestamp(data.grafana.timestamp)}</div>
              </div>
            ` : ''}
          </div>
        ` : ''}

        ${data.generic ? `
          <div style="margin-left: 15px; margin-top: 10px;">
            <div class="info-label">🔍 通用信息</div>
            ${data.generic.authCookies ? `
              <div style="margin-bottom: 8px;">
                <strong>认证 Cookies:</strong>
                <div class="info-value">${data.generic.authCookies.join(', ')}</div>
              </div>
            ` : ''}
            ${data.generic.timestamp ? `
              <div style="margin-bottom: 8px;">
                <strong>获取时间:</strong>
                <div class="info-value">${formatTimestamp(data.generic.timestamp)}</div>
              </div>
            ` : ''}
          </div>
        ` : ''}
      </div>
    `;
  });
  
  container.innerHTML = html;
}

// 刷新信息
async function refreshInfo() {
  showStatus('正在获取存储的登录信息...', 'info');
  
  try {
    const data = await chrome.storage.local.get('tokens');
    console.log('[TiHC Popup] 存储的数据:', data); // 调试日志
    console.log('[TiHC Popup] tokens:', data.tokens); // 调试日志
    
    renderTokenInfo(data.tokens);
    
    if (data.tokens && Object.keys(data.tokens).length > 0) {
      showStatus(`成功获取 ${Object.keys(data.tokens).length} 个网站的信息！`, 'success');
    } else {
      showStatus('暂无存储的登录信息', 'info');
    }
  } catch (error) {
    console.error('获取信息失败:', error);
    showStatus('获取信息失败: ' + error.message, 'error');
  }
}

// 导出 JSON
async function exportTokens() {
  try {
    const data = await chrome.storage.local.get("tokens");
    
    if (!data.tokens || Object.keys(data.tokens).length === 0) {
      showStatus('暂无数据可导出', 'error');
      return;
    }
    
    const json = JSON.stringify(data.tokens, null, 2);
    const blob = new Blob([json], {type: "application/json"});
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `tihc_tokens_${new Date().toISOString().slice(0, 10)}.json`;
    a.click();
    URL.revokeObjectURL(url);
    
    showStatus('Token 已导出到下载文件夹！', 'success');
  } catch (error) {
    console.error('导出失败:', error);
    showStatus('导出失败: ' + error.message, 'error');
  }
}

// 清空所有数据
async function clearAllData() {
  try {
    await chrome.storage.local.clear();
    renderTokenInfo({});
    showStatus('所有数据已清空', 'success');
  } catch (error) {
    console.error('清空数据失败:', error);
    showStatus('清空数据失败: ' + error.message, 'error');
  }
}

// 事件监听器
document.addEventListener('DOMContentLoaded', () => {
  // 页面加载完成后初始化
document.addEventListener('DOMContentLoaded', () => {
  // 立即显示已存储的信息
  refreshInfo();
  
  // 绑定按钮事件 - "刷新显示"按钮只是重新显示存储的数据
  document.getElementById('refresh').addEventListener('click', refreshInfo);
  document.getElementById('export').addEventListener('click', exportTokens);
});
});
