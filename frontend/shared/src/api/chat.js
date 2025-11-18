import { request } from '@/utils/http'

/**
 * 发起普通聊天请求
 * @param {number} userId - 用户ID
 * @param {Array} messages - 消息数组，格式: [{ role: 'user', content: string }]
 * @returns {Promise<{ id: string, choices: Array<{ message: { role: string, content: string } }> }>}
 */
export function startChat(userId, messages) {
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
export function streamChat(userId, messages, { onData, onError, onComplete, token } = {}) {
  return new Promise((resolve, reject) => {
    const candidateBase = request?.defaults?.baseURL
      || import.meta.env.VITE_AXIOS_BASE_URL
      || window.location.origin
    const normalizedBase = candidateBase.endsWith('/') ? candidateBase : `${candidateBase}/`
    const url = new URL('chat/stream', normalizedBase)
    url.searchParams.set('messages', JSON.stringify(messages))
    url.searchParams.set('userId', userId.toString())
    if (token)
      url.searchParams.set('token', token)

    const eventSource = new EventSource(url.toString(), { withCredentials: true })

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
}

/**
 * 获取用户的最近聊天记录
 * @param {string} userId - 用户ID
 * @param {object} options - 可选参数
 * @param {string} [options.sessionId] - 会话ID，未提供时后端返回最近一次会话记录
 * @param {number} [options.limit] - 返回的记录数量，默认5条，最大10条
 * @returns {Promise<ApiResponse>} 包含历史记录数组的统一响应结构
 */
export function getChatHistory(userId, { sessionId, limit = 5 } = {}) {
  return request({
    url: '/chat/history',
    method: 'get',
    params: {
      userId,
      sessionId,
      limit: Math.min(Math.max(1, limit), 10), // 确保limit在1-10之间
    },
  })
}

/**
 * 创建聊天会话
 * @param {number} userId - 用户ID
 * @param {string} [title] - 会话标题
 * @returns {Promise<ApiResponse>} 包含新创建会话信息的响应
 */
export function createChatSession(userId, title) {
  return request({
    url: '/chat/sessions',
    method: 'post',
    data: {
      userId,
      title,
    },
  })
}

/**
 * 获取用户的聊天会话列表
 * @param {number} userId - 用户ID
 * @param {object} options - 可选参数
 * @param {number} [options.limit] - 返回的会话数量限制，默认20
 * @returns {Promise<ApiResponse>} 包含会话列表的响应
 */
export function listChatSessions(userId, { limit = 20 } = {}) {
  return request({
    url: '/chat/sessions',
    method: 'get',
    params: {
      userId,
      limit,
    },
  })
}

export function addChatMessage({ userId, sessionId, userMessage, assistantMessage }) {
  return request({
    url: '/chat/history',
    method: 'post',
    data: {
      userId,
      sessionId,
      userMessage,
      assistantMessage,
    },
  })
}

// 仅管理员可用的API
export const adminApi = {
  /**
   * 获取当前API Key信息（脱敏显示）
   */
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
  deleteKey() {
    return request({
      url: '/chat/admin/key',
      method: 'delete',
    })
  },
}
