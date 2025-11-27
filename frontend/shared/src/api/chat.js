import { request } from '@/utils/http'

/**
 * Chat API 模块，统一导出所有聊天相关方法
 */
const chatApi = {}

/**
 * 发起普通聊天请求
 * @param {number} userId - 用户ID
 * @param {Array} messages - 消息数组，格式: [{ role: 'user', content: string }]
 * @returns {Promise<{ id: string, choices: Array<{ message: { role: string, content: string } }> }>}
 */
/**
 * 发起普通聊天请求
 * @param {number} userId - 用户ID
 * @param {Array} messages - 消息数组
 * @returns {Promise}
 */
chatApi.startChat = function (userId, messages) {
  if (!userId || !Array.isArray(messages))
    throw new Error('Invalid params')
  return request({
    url: '/chat/start',
    method: 'post',
    data: { userId, messages },
  })
}

/**
 * 发起流式聊天请求
 * @param {number} userId - 用户ID
 * @param {Array} messages - 消息数组
 * @param {object} options - 选项对象
 * @param {Function} [options.onData] - 处理每个数据片段的回调
 * @param {Function} [options.onError] - 错误处理回调
 * @param {Function} [options.onComplete] - 完成时的回调
 * @param {string} [options.token] - 认证token
 * @returns {Promise<void>} Promise对象
 */
/**
 * 发起流式聊天请求（支持 abort）
 * @param {number} userId
 * @param {Array} messages
 * @param {object} options
 * @returns {Promise<{ abort: Function }>} Promise对象，含 abort 方法
 */
chatApi.streamChat = function (userId, messages, { onData, onError, onComplete, token } = {}) {
  if (!userId || !Array.isArray(messages))
    throw new Error('Invalid params')
  let eventSource = null
  const promise = new Promise((resolve, reject) => {
    let candidateBase = request?.defaults?.baseURL || import.meta.env.VITE_AXIOS_BASE_URL || window.location.origin
    // 如果是以 / 开头且不是 http(s)，拼接 window.location.origin
    if (candidateBase.startsWith('/') && !candidateBase.startsWith('http')) {
      candidateBase = window.location.origin + candidateBase
    }
    const normalizedBase = candidateBase.endsWith('/') ? candidateBase : `${candidateBase}/`
    const url = new URL('chat/stream', normalizedBase)
    url.searchParams.set('messages', JSON.stringify(messages))
    url.searchParams.set('userId', userId.toString())
    if (token)
      url.searchParams.set('token', token)

    eventSource = new EventSource(url.toString(), { withCredentials: true })

    eventSource.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data)
        onData?.(data)
      }
      catch (err) {
        console.error('Failed to parse SSE data:', err)
      }
    }
    eventSource.onerror = (error) => {
      eventSource.close()
      onError?.(error)
      reject(error)
    }
    eventSource.addEventListener('complete', () => {
      eventSource.close()
      onComplete?.()
      resolve()
    })
  })
  promise.abort = () => {
    if (eventSource)
      eventSource.close()
  }
  return promise
}

/**
 * 获取用户的最近聊天记录
 * @param {string} userId - 用户ID
 * @param {object} options - 可选参数
 * @param {string} [options.sessionId] - 会话ID，未提供时后端返回最近一次会话记录
 * @param {number} [options.limit] - 返回的记录数量，默认5条，最大10条
 * @returns {Promise<ApiResponse>} 包含历史记录数组的统一响应结构
 */
/**
 * 获取用户的最近聊天记录
 */
chatApi.getChatHistory = function (userId, { sessionId, limit = 5 } = {}) {
  if (!userId)
    throw new Error('userId required')
  return request({
    url: '/chat/history',
    method: 'get',
    params: {
      userId,
      sessionId,
      limit: Math.min(Math.max(1, limit), 10),
    },
  })
}

/**
 * 创建聊天会话
 * @param {number} userId - 用户ID
 * @param {string} [title] - 会话标题
 * @returns {Promise<ApiResponse>} 包含新创建会话信息的响应
 */
/**
 * 创建聊天会话
 */
chatApi.createChatSession = function (userId, title) {
  if (!userId)
    throw new Error('userId required')
  return request({
    url: '/chat/sessions',
    method: 'post',
    data: { userId, title },
  })
}

/**
 * 获取用户的聊天会话列表
 * @param {number} userId - 用户ID
 * @param {object} options - 可选参数
 * @param {number} [options.limit] - 返回的会话数量限制，默认20
 * @returns {Promise<ApiResponse>} 包含会话列表的响应
 */
/**
 * 获取用户的聊天会话列表
 */
chatApi.listChatSessions = function (userId, { limit = 20 } = {}) {
  if (!userId)
    throw new Error('userId required')
  return request({
    url: '/chat/sessions',
    method: 'get',
    params: { userId, limit },
  })
}

/**
 * 添加聊天消息
 */
chatApi.addChatMessage = function ({ userId, sessionId, userMessage, assistantMessage }) {
  if (!userId || !sessionId || !userMessage)
    throw new Error('Invalid params')
  return request({
    url: '/chat/history',
    method: 'post',
    data: { userId, sessionId, userMessage, assistantMessage },
  })
}

/**
 * 管理员相关API
 */
chatApi.admin = {
  /**
   * 获取当前API Key信息（脱敏显示）
   */
  /** 获取当前API Key信息（脱敏显示） */
  getCurrentKey() {
    return request({
      url: '/chat/admin/key',
      method: 'get',
    })
  },

  /**
   * 更新API Key
   * @param {string} key - 新的API Key
   */
  /** 更新API Key */
  updateKey(key) {
    return request({
      url: '/chat/admin/key',
      method: 'post',
      data: { key },
    })
  },

  /**
   * 删除当前API Key
   */
  /** 删除API Key */
  deleteKey() {
    return request({
      url: '/chat/admin/key',
      method: 'delete',
    })
  },
}

export default chatApi
