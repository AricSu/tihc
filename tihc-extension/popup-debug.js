// 调试版本的 popup.js
console.log('[TiHC Popup] 调试版本加载');

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

// 简化的渲染函数
function renderTokenInfo(tokens) {
  const container = document.getElementById('token-info');
  
  console.log('[TiHC Popup Debug] 渲染数据:', tokens);
  
  if (!tokens || Object.keys(tokens).length === 0) {
    container.innerHTML = `
      <div class="empty-state">
        <p>暂无数据</p>
        <p style="font-size: 12px; color: #666;">请检查控制台日志</p>
      </div>`;
    return;
  }
  
  // 直接显示原始 JSON
  container.innerHTML = `
    <div style="font-family: monospace; font-size: 11px; padding: 10px;">
      <h4>调试数据:</h4>
      <pre style="background: #f5f5f5; padding: 10px; border-radius: 4px; overflow: auto; max-height: 300px;">
${JSON.stringify(tokens, null, 2)}
      </pre>
    </div>`;
}

// 刷新信息
async function refreshInfo() {
  showStatus('正在获取数据...', 'info');
  
  try {
    console.log('[TiHC Popup Debug] 开始获取存储数据');
    
    // 获取所有存储数据
    const allData = await chrome.storage.local.get(null);
    console.log('[TiHC Popup Debug] 所有存储数据:', allData);
    
    // 特别获取 tokens
    const tokensData = await chrome.storage.local.get('tokens');
    console.log('[TiHC Popup Debug] tokens 数据:', tokensData);
    
    renderTokenInfo(tokensData.tokens);
    
    if (tokensData.tokens && Object.keys(tokensData.tokens).length > 0) {
      showStatus(`找到 ${Object.keys(tokensData.tokens).length} 个域名的数据`, 'success');
    } else {
      showStatus('未找到数据', 'info');
    }
  } catch (error) {
    console.error('[TiHC Popup Debug] 错误:', error);
    showStatus('获取数据失败: ' + error.message, 'error');
  }
}

// 导出功能
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
    
    showStatus('数据已导出！', 'success');
  } catch (error) {
    console.error('导出失败:', error);
    showStatus('导出失败: ' + error.message, 'error');
  }
}

// 页面加载完成后初始化
document.addEventListener('DOMContentLoaded', () => {
  console.log('[TiHC Popup Debug] DOM 加载完成');
  
  // 立即显示数据
  refreshInfo();
  
  // 绑定按钮事件
  document.getElementById('refresh').addEventListener('click', refreshInfo);
  document.getElementById('export').addEventListener('click', exportTokens);
});