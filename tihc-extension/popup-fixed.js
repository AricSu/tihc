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

// 渲染 Token 信息 - 修复版本
function renderTokenInfo(tokens) {
  const container = document.getElementById('token-info');
  
  if (!tokens || Object.keys(tokens).length === 0) {
    container.innerHTML = `
      <div class="empty-state">
        <p>暂无登录信息</p>
        <p style="font-size: 12px; color: #666; margin-top: 10px;">
          请确保已访问 Clinic 或 Grafana 页面
        </p>
      </div>`;
    return;
  }
  
  let html = '';
  
  Object.entries(tokens).forEach(([domain, domainData]) => {
    html += `
      <div class="info-item">
        <div class="info-label">🌐 ${domain}</div>
    `;
    
    // 处理 clinic 数据
    if (domainData.clinic) {
      const clinic = domainData.clinic;
      html += `
        <div style="margin-left: 15px; margin-top: 10px;">
          <div class="info-label">🏥 Clinic 信息</div>
          
          ${clinic.session_id ? `
            <div style="margin-bottom: 8px;">
              <strong>Session ID:</strong>
              <div class="info-value">${truncateText(clinic.session_id, 50)}</div>
            </div>
          ` : ''}
          
          ${clinic.csrf_token ? `
            <div style="margin-bottom: 8px;">
              <strong>CSRF Token:</strong>
              <div class="info-value">${truncateText(clinic.csrf_token, 50)}</div>
            </div>
          ` : ''}
          
          ${clinic.apikey ? `
            <div style="margin-bottom: 8px;">
              <strong>API Key:</strong>
              <div class="info-value">${truncateText(clinic.apikey, 50)}</div>
            </div>
          ` : ''}
          
          ${clinic.relevantCookies && clinic.relevantCookies.length > 0 ? `
            <div style="margin-bottom: 8px;">
              <strong>相关 Cookies:</strong>
              <div class="info-value">${clinic.relevantCookies.join(', ')}</div>
            </div>
          ` : ''}
          
          ${clinic.relevantLocalStorage && clinic.relevantLocalStorage.length > 0 ? `
            <div style="margin-bottom: 8px;">
              <strong>LocalStorage 数据:</strong>
              <div class="info-value">${clinic.relevantLocalStorage.join(', ')}</div>
            </div>
          ` : ''}
          
          ${clinic.cookie ? `
            <div style="margin-bottom: 8px;">
              <strong>完整 Cookie:</strong>
              <div class="info-value">${truncateText(clinic.cookie, 100)}</div>
            </div>
          ` : ''}
          
          ${clinic.timestamp ? `
            <div style="margin-bottom: 8px;">
              <strong>获取时间:</strong>
              <div class="info-value">${formatTimestamp(clinic.timestamp)}</div>
            </div>
          ` : ''}
        </div>
      `;
    }
    
    // 处理 grafana 数据
    if (domainData.grafana) {
      const grafana = domainData.grafana;
      html += `
        <div style="margin-left: 15px; margin-top: 10px;">
          <div class="info-label">📊 Grafana 信息</div>
          
          ${grafana.session ? `
            <div style="margin-bottom: 8px;">
              <strong>Session:</strong>
              <div class="info-value">${truncateText(grafana.session, 50)}</div>
            </div>
          ` : ''}
          
          ${grafana.auth_token ? `
            <div style="margin-bottom: 8px;">
              <strong>Auth Token:</strong>
              <div class="info-value">${truncateText(grafana.auth_token, 50)}</div>
            </div>
          ` : ''}
          
          ${grafana.allCookies ? `
            <div style="margin-bottom: 8px;">
              <strong>完整 Cookie:</strong>
              <div class="info-value">${truncateText(grafana.allCookies, 100)}</div>
            </div>
          ` : ''}
          
          ${grafana.timestamp ? `
            <div style="margin-bottom: 8px;">
              <strong>获取时间:</strong>
              <div class="info-value">${formatTimestamp(grafana.timestamp)}</div>
            </div>
          ` : ''}
        </div>
      `;
    }
    
    html += `</div>`;
  });
  
  container.innerHTML = html;
}

// 刷新信息
async function refreshInfo() {
  showStatus('正在获取存储的登录信息...', 'info');
  
  try {
    const data = await chrome.storage.local.get('tokens');
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

// 页面加载完成后初始化
document.addEventListener('DOMContentLoaded', () => {
  // 立即显示已存储的信息
  refreshInfo();
  
  // 绑定按钮事件
  document.getElementById('refresh').addEventListener('click', refreshInfo);
  document.getElementById('export').addEventListener('click', exportTokens);
  
  // 每10秒自动刷新显示
  let refreshCount = 0;
  setInterval(() => {
    refreshCount++;
    console.log(`[TiHC Popup] 自动刷新显示 #${refreshCount}`);
    
    // 闪烁指示器
    const indicator = document.getElementById('auto-refresh-indicator');
    if (indicator) {
      indicator.style.color = '#ffc107';
      setTimeout(() => {
        indicator.style.color = '#28a745';
      }, 200);
    }
    
    refreshInfo();
  }, 10000); // 10秒
});