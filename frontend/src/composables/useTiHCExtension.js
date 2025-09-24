/**
 * TiHC 浏览器扩展管理 Composable
 * 提供扩展检测、页面类型检测和数据采集功能
 */

import { ref, computed } from 'vue'
import { 
  getPageTypeDisplayName, 
  getPageTypeTagType,
  getPageCollectionInfo,
  watchPageChanges
} from '@/utils/pageDetection.js'

export function useTiHCExtension() {
  // 扩展状态
  const checking = ref(false)
  const extensionInstalled = ref(false)
  const extensionVersion = ref('')
  
  // 采集状态
  const collecting = ref(false)
  const collectionStatus = ref('')
  
  // 页面信息
  const pageInfo = ref({
    url: '',
    type: 'unknown',
    domain: '',
    title: '',
    supported: false
  })
  
  // 采集历史
  const collectionHistory = ref([])
  
  // 配置
  const backendUrl = import.meta.env.VITE_AXIOS_BASE_URL
  
  // 计算属性
  const isExtensionReady = computed(() => extensionInstalled.value)
  const canCollect = computed(() => isExtensionReady.value)
  const pageTypeDisplay = computed(() => getPageTypeDisplayName(pageInfo.value.type))
  const pageTypeTag = computed(() => getPageTypeTagType(pageInfo.value.type))
  
  // 清理函数数组
  const cleanupFunctions = []
  
  /**
   * 更新页面信息
   */
  function updatePageInfo() {
    const info = getPageCollectionInfo()
    pageInfo.value = {
      url: info.url,
      type: info.pageType,
      domain: info.domain,
      title: info.title,
      supported: true // 所有页面都支持数据采集
    }
    console.log('[Extension Composable] 页面信息更新:', pageInfo.value)
  }
  
  /**
   * 检测扩展安装状态
   */
  async function checkExtension() {
    checking.value = true
    console.log('[Extension Composable] 开始检测扩展')
    
    try {
      let detected = false
      let version = ''
      
      // 检查全局标识
      if (window.TiHCExtensionInstalled) {
        detected = true
        version = window.TiHCExtensionVersion || '1.0'
        console.log('[Extension Composable] ✓ 通过全局标识检测到扩展')
      } else {
        // 通过消息检测
        console.log('[Extension Composable] 尝试通过消息检测扩展')
        
        const result = await new Promise((resolve) => {
          const messageHandler = (event) => {
            if (event.source === window && event.data.type === 'TIHC_EXTENSION_RESPONSE') {
              console.log('[Extension Composable] ✓ 收到扩展响应:', event.data)
              resolve({ 
                detected: true, 
                version: event.data.version || '1.0' 
              })
              window.removeEventListener('message', messageHandler)
            }
          }
          
          window.addEventListener('message', messageHandler)
          
          // 发送检测消息
          console.log('[Extension Composable] 发送扩展检测消息')
          window.postMessage({ type: 'TIHC_EXTENSION_CHECK' }, '*')
          
          setTimeout(() => {
            window.removeEventListener('message', messageHandler)
            resolve({ detected: false, version: '' })
          }, 3000)
        })
        
        detected = result.detected
        version = result.version
      }
      
      extensionInstalled.value = detected
      extensionVersion.value = version
      
      if (detected) {
        updatePageInfo()
      }
      
      const message = detected ? '扩展检测成功' : '未检测到扩展'
      console.log(`[Extension Composable] ${message}`)
      
      return { success: detected, message, version }
      
    } catch (error) {
      console.error('[Extension Composable] 检测失败:', error)
      extensionInstalled.value = false
      throw error
    } finally {
      checking.value = false
    }
  }
  
  /**
   * 开始数据采集
   */
  async function startCollection(targetUrl) {
    if (!extensionInstalled.value) {
      throw new Error('请先安装并启用扩展')
    }
    
    if (!targetUrl) {
      throw new Error('请先选择目标URL')
    }
    
    // 移除页面类型限制，支持所有页面的数据采集
    const currentPageType = pageInfo.value.type || 'unknown'
    
    collecting.value = true
    collectionStatus.value = '准备发送采集指令...'
    
    try {
      const config = {
        backendUrl: backendUrl,
        targetUrl: targetUrl // 传递目标URL给扩展
      }
      
      const command = {
        type: 'TIHC_START_COLLECTION',
        pageType: currentPageType, // 传入当前页面类型，即使是 unknown 也允许采集
        config: config
      }
      
      console.log('[Extension Composable] 发送采集指令:', command)
      collectionStatus.value = '等待插件执行采集...'
      
      // 先设置消息监听器等待响应，再发送指令
      const result = await new Promise((resolve, reject) => {
        const timeout = setTimeout(() => {
          window.removeEventListener('message', messageHandler)
          reject(new Error('插件响应超时 (30秒)'))
        }, 30000)
        
        const messageHandler = (event) => {
          if (event.source !== window) return
          
          switch (event.data.type) {
            case 'TIHC_COLLECTION_SUCCESS':
              clearTimeout(timeout)
              window.removeEventListener('message', messageHandler)
              console.log('[Extension Composable] 采集成功:', event.data)
              
              // 记录采集历史
              const record = {
                id: Date.now(),
                domain: event.data.data.domain,
                pageType: event.data.data.pageType,
                timestamp: event.data.data.timestamp,
                count: event.data.data.count
              }
              collectionHistory.value.unshift(record)
              // 只保留最近20条记录
              if (collectionHistory.value.length > 20) {
                collectionHistory.value = collectionHistory.value.slice(0, 20)
              }
              
              resolve({
                success: true,
                message: `数据采集成功 - ${getPageTypeDisplayName(event.data.data.pageType)}`,
                data: event.data.data
              })
              break
              
            case 'TIHC_COLLECTION_ERROR':
              clearTimeout(timeout)
              window.removeEventListener('message', messageHandler)
              console.error('[Extension Composable] 采集失败:', event.data)
              reject(new Error(event.data.error || '采集失败'))
              break
          }
        }
        
        // 先添加监听器
        window.addEventListener('message', messageHandler)
        
        // 再发送指令
        console.log('[Extension Composable] 发送采集指令到插件')
        window.postMessage(command, '*')
      })
      
      // 等待响应
      const response = await result
      
      collectionStatus.value = response.message
      console.log('[Extension Composable] 采集完成:', response)
      
      return response
      
    } catch (error) {
      console.error('[Extension Composable] 采集失败:', error)
      collectionStatus.value = '采集失败: ' + error.message
      throw error
    } finally {
      collecting.value = false
    }
  }
  
  /**
   * 停止数据采集
   */
  function stopCollection() {
    console.log('[Extension Composable] 发送停止采集指令')
    window.postMessage({ type: 'TIHC_STOP_COLLECTION' }, '*')
    collecting.value = false
    collectionStatus.value = '已停止采集'
  }
  
  /**
   * 格式化时间
   */
  function formatTime(timestamp) {
    return new Date(timestamp).toLocaleString('zh-CN')
  }
  
  /**
   * 获取采集历史
   */
  function getCollectionHistory(limit = 10) {
    return collectionHistory.value.slice(0, limit)
  }
  
  /**
   * 清除采集历史
   */
  function clearCollectionHistory() {
    collectionHistory.value = []
  }
  
  /**
   * 初始化扩展管理器
   */
  function initialize() {
    // 立即检测扩展
    checkExtension()
    
    // 监听扩展消息
    const messageHandler = (event) => {
      if (event.source === window) {
        switch (event.data.type) {
          case 'TIHC_EXTENSION_LOADED':
            console.log('[Extension Composable] 收到扩展加载完成消息')
            extensionInstalled.value = true
            extensionVersion.value = event.data.version || '1.0'
            updatePageInfo()
            break
        }
      }
    }
    
    window.addEventListener('message', messageHandler)
    cleanupFunctions.push(() => window.removeEventListener('message', messageHandler))
    
    // 监听页面焦点变化
    const handleFocus = () => {
      if (!extensionInstalled.value) {
        console.log('[Extension Composable] 页面获得焦点，重新检测扩展')
        setTimeout(checkExtension, 1000)
      }
    }
    
    window.addEventListener('focus', handleFocus)
    cleanupFunctions.push(() => window.removeEventListener('focus', handleFocus))
    
    // 监听页面变化
    const stopWatching = watchPageChanges((info) => {
      if (extensionInstalled.value) {
        pageInfo.value = {
          url: info.url,
          type: info.pageType,
          domain: window.location.hostname,
          title: document.title,
          supported: true // 所有页面都支持数据采集
        }
        console.log('[Extension Composable] 页面变化检测:', pageInfo.value)
      }
    })
    
    cleanupFunctions.push(stopWatching)
  }
  
  /**
   * 清理资源
   */
  function cleanup() {
    cleanupFunctions.forEach(fn => fn())
    cleanupFunctions.length = 0
  }
  
  return {
    // 状态
    checking,
    extensionInstalled,
    extensionVersion,
    collecting,
    collectionStatus,
    pageInfo,
    collectionHistory,
    
    // 计算属性
    isExtensionReady,
    canCollect,
    pageTypeDisplay,
    pageTypeTag,
    
    // 方法
    checkExtension,
    startCollection,
    stopCollection,
    updatePageInfo,
    formatTime,
    getCollectionHistory,
    clearCollectionHistory,
    initialize,
    cleanup
  }
}