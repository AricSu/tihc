import { request } from '@/utils/http'

// Chat API 模块：只负责与后端接口通信，不做本地缓存或业务逻辑
const chatApi = {}

/**
 * 发起普通聊天请求
 * @param {number} userId - 用户ID
 * @param {Array} messages - 消息数组 [{ role: 'user', content: string }]
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
 * 发起流式聊天请求（支持 abort）
 * @param {number} userId
 * @param {Array} messages
 * @param {object} options { onData, onError, onComplete, token }
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
 * @param {number} userId - 用户ID
 * @param {object} options { sessionId, limit }
 * @returns {Promise}
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
 * @returns {Promise}
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
 * @param {object} options { limit }
 * @returns {Promise}
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
 * @param {object} params { userId, sessionId, userMessage, assistantMessage }
 * @returns {Promise}
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

// 管理员相关API
chatApi.admin = {
  /** 获取当前API Key信息（脱敏显示） */
  getCurrentKey() {
    return request({
      url: '/chat/admin/key',
      method: 'get',
    })
  },
  /** 更新API Key */
  updateKey(key) {
    return request({
      url: '/chat/admin/key',
      method: 'post',
      data: { key },
    })
  },
  /** 删除API Key */
  deleteKey() {
    return request({
      url: '/chat/admin/key',
      method: 'delete',
    })
  },
}

export default chatApi
