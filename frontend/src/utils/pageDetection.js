/**
 * 页面类型检测工具
 * 从插件移到前端的页面类型检测逻辑，供 TiHC 前端统一使用
 */

/**
 * 检测当前页面类型
 * @param {string} [url] - 可选的URL，默认使用当前页面URL
 * @param {string} [domain] - 可选的域名，默认使用当前页面域名  
 * @param {string} [title] - 可选的标题，默认使用当前页面标题
 * @returns {string} 页面类型: 'grafana' | 'clinic' | 'unknown'
 */
export function detectPageType(url, domain, title) {
  const currentUrl = (url || window.location.href).toLowerCase()
  const currentDomain = (domain || window.location.hostname).toLowerCase()
  const currentTitle = (title || document.title).toLowerCase()
  
  // Grafana 检测逻辑
  if (isGrafanaPage(currentDomain, currentUrl, currentTitle)) {
    return 'grafana'
  }
  
  // TiDB Clinic 检测逻辑
  if (isClinicPage(currentDomain, currentUrl, currentTitle)) {
    return 'clinic'
  }
  
  return 'unknown'
}

/**
 * 检测是否为 Grafana 页面
 */
function isGrafanaPage(domain, url, title) {
  // 域名检测
  if (domain.includes('grafana')) {
    return true
  }
  
  // URL 检测
  if (url.includes('grafana')) {
    return true
  }
  
  // 标题检测
  if (title.includes('grafana')) {
    return true
  }
  
  // DOM 元素检测
  if (typeof document !== 'undefined') {
    // Grafana 特有的数据属性
    if (document.querySelector('[data-grafana-version]')) {
      return true
    }
    
    // Grafana 应用容器
    if (document.querySelector('.grafana-app')) {
      return true
    }
    
    // Grafana 根元素
    if (document.querySelector('#grafana-root')) {
      return true
    }
    
    // Grafana 侧边栏
    if (document.querySelector('.sidemenu')) {
      return true
    }
    
    // Grafana 面板
    if (document.querySelector('.panel-content')) {
      return true
    }
  }
  
  return false
}

/**
 * 检测是否为 TiDB Clinic 页面
 */
function isClinicPage(domain, url, title) {
  // 域名检测
  const clinicDomains = ['clinic', 'tidb', 'pingcap']
  if (clinicDomains.some(keyword => domain.includes(keyword))) {
    return true
  }
  
  // URL 检测
  const clinicUrlKeywords = ['clinic', 'tidb']
  if (clinicUrlKeywords.some(keyword => url.includes(keyword))) {
    return true
  }
  
  // 标题检测
  const clinicTitleKeywords = ['tidb', 'clinic', 'pingcap']
  if (clinicTitleKeywords.some(keyword => title.includes(keyword))) {
    return true
  }
  
  // DOM 元素检测
  if (typeof document !== 'undefined') {
    // TiDB 特有的数据属性
    if (document.querySelector('[data-tidb]')) {
      return true
    }
    
    // TiDB 应用容器
    if (document.querySelector('.tidb-app')) {
      return true
    }
    
    // Clinic 特有元素
    if (document.querySelector('.clinic-container')) {
      return true
    }
    
    // PingCAP 相关元素
    if (document.querySelector('[class*="pingcap"]')) {
      return true
    }
  }
  
  return false
}

/**
 * 获取页面类型的显示名称
 * @param {string} pageType - 页面类型
 * @returns {string} 显示名称
 */
export function getPageTypeDisplayName(pageType) {
  switch (pageType) {
    case 'grafana':
      return 'Grafana'
    case 'clinic':
      return 'TiDB Clinic'
    case 'unknown':
      return '通用页面'
    default:
      return '其他页面'
  }
}

/**
 * 获取页面类型的标签样式
 * @param {string} pageType - 页面类型
 * @returns {string} 标签类型
 */
export function getPageTypeTagType(pageType) {
  switch (pageType) {
    case 'grafana':
      return 'info'
    case 'clinic':
      return 'success'
    case 'unknown':
      return 'warning'
    default:
      return 'default'
  }
}

/**
 * 检测页面是否支持数据采集
 * @param {string} [pageType] - 页面类型，如果不提供则自动检测
 * @returns {boolean} 是否支持采集
 */
export function isPageSupportedForCollection(pageType) {
  // 移除限制，所有页面都支持数据采集
  return true
}

/**
 * 获取页面采集的相关信息
 * @returns {Object} 页面采集信息
 */
export function getPageCollectionInfo() {
  const pageType = detectPageType()
  
  return {
    pageType,
    displayName: getPageTypeDisplayName(pageType),
    tagType: getPageTypeTagType(pageType),
    supported: isPageSupportedForCollection(pageType),
    url: window.location.href,
    domain: window.location.hostname,
    title: document.title
  }
}

/**
 * 监听页面变化并执行回调
 * @param {Function} callback - 页面变化时的回调函数
 * @returns {Function} 清理函数
 */
export function watchPageChanges(callback) {
  let currentUrl = window.location.href
  let currentPageType = detectPageType()
  
  // 检查页面变化的函数
  const checkChanges = () => {
    const newUrl = window.location.href
    const newPageType = detectPageType()
    
    if (newUrl !== currentUrl || newPageType !== currentPageType) {
      currentUrl = newUrl
      currentPageType = newPageType
      
      callback({
        url: newUrl,
        pageType: newPageType,
        pageInfo: getPageCollectionInfo()
      })
    }
  }
  
  // 监听 popstate 事件（浏览器前进后退）
  window.addEventListener('popstate', checkChanges)
  
  // 使用 MutationObserver 监听 DOM 变化（适用于 SPA）
  const observer = new MutationObserver(checkChanges)
  observer.observe(document, { subtree: true, childList: true })
  
  // 定时检查（作为后备方案）
  const interval = setInterval(checkChanges, 2000)
  
  // 返回清理函数
  return () => {
    window.removeEventListener('popstate', checkChanges)
    observer.disconnect()
    clearInterval(interval)
  }
}