import { ref } from 'vue'
import chatApi from '@/api/chat'
import { useAuthStore, useUserStore } from '@/store'

// 浏览器存储工具函数（仅业务层使用）
const CHAT_STORAGE_KEY = 'tihc_chat_messages'
const MAX_STORED_MESSAGES = 2

function loadMessagesFromStorage(userId) {
  try {
    const stored = localStorage.getItem(`${CHAT_STORAGE_KEY}_${userId}`)
    if (stored) {
      const data = JSON.parse(stored)
      if (data.timestamp && Date.now() - data.timestamp < 24 * 60 * 60 * 1000) {
        return Array.isArray(data.messages) ? data.messages.slice(-MAX_STORED_MESSAGES) : []
      }
    }
  }
  catch (error) {
    console.warn('Failed to load messages from storage:', error)
  }
  return []
}
function saveMessagesToStorage(userId, messages) {
  try {
    const aiMessages = messages.filter(msg => msg.senderId === 'assistant').slice(-MAX_STORED_MESSAGES)
    const dataToStore = { messages: aiMessages, timestamp: Date.now() }
    localStorage.setItem(`${CHAT_STORAGE_KEY}_${userId}`, JSON.stringify(dataToStore))
  }
  catch (error) {
    console.warn('Failed to save messages to storage:', error)
  }
}

function clearMessagesFromStorage(userId) {
  try {
    localStorage.removeItem(`${CHAT_STORAGE_KEY}_${userId}`)
  }
  catch (error) {
    console.warn('Failed to clear messages from storage:', error)
  }
}

export function useChat() {
  const authStore = useAuthStore()
  const userStore = useUserStore()
  const loggedInUser = userStore.userInfo
  const backendUserId = ref(loggedInUser?.id ?? 0)
  const currentUserId = ref(loggedInUser ? String(loggedInUser.id) : 'jira-assistant')
  const rooms = ref([
    {
      roomId: 'general',
      roomName: 'General AI',
      avatar: '',
      users: [
        { _id: 'jira-assistant', username: 'jira assistant' },
        ...(loggedInUser
          ? [{
              _id: String(loggedInUser.id),
              username: loggedInUser.nickName || loggedInUser.username || 'current user',
            }]
          : []),
      ],
    },
  ])
  const messages = ref([])
  const messagesLoaded = ref(false)
  const composer = ref('')
  // ...existing code...
  // 兼容 vue-advanced-chat 的接口（实际不做任何事）
  function addMessages(_reset = false) {
    return []
  }

  // 会话管理（如有需要可扩展）
  const currentSessionId = ref(null)
  async function ensureChatSession(title = 'default') {
    if (currentSessionId.value)
      return currentSessionId.value
    try {
      const response = await chatApi.createChatSession(backendUserId.value, title)
      const session = response?.data
      if (session?.id)
        currentSessionId.value = session.id
    }
    catch (error) {
      console.error('[useChat] Failed to create chat session:', error)
    }
    return currentSessionId.value
  }

  // 加载历史消息（优先本地缓存，必要时请求后端）
  const loadingHistory = ref(false)
  async function fetchMessages({ reset = false } = {}) {
    if (loadingHistory.value)
      return
    loadingHistory.value = true
    try {
      // 优先本地缓存
      const storedMessages = loadMessagesFromStorage(backendUserId.value)
      if (storedMessages.length > 0 && reset) {
        messages.value = storedMessages
        messagesLoaded.value = true
        loadingHistory.value = false
        return
      }
      // 后端拉取
      const sessionId = await ensureChatSession()
      const historyResponse = await chatApi.getChatHistory(backendUserId.value, { sessionId, limit: 5 })
      const histories = Array.isArray(historyResponse?.data) ? historyResponse.data : []
      const historyMessages = histories.filter(h => h.assistant).map((h, index) => ({
        _id: `history_assistant_${index}`,
        content: h.assistant,
        senderId: 'assistant',
        username: 'TiDB Assistant',
        timestamp: h.timestamp ? new Date(h.timestamp).toString().substring(16, 21) : '',
        date: h.timestamp ? new Date(h.timestamp).toDateString() : '',
        system: false,
        replyMessage: h.user ? { content: h.user, senderId: currentUserId.value } : null,
      }))
      messages.value = reset ? historyMessages : [...messages.value, ...historyMessages]
      if (historyMessages.length > 0)
        saveMessagesToStorage(backendUserId.value, historyMessages)
      messagesLoaded.value = true
    }
    catch (error) {
      console.error('[useChat] Failed to load chat history:', error)
    }
    finally {
      loadingHistory.value = false
    }
  }

  // 兼容 vue-advanced-chat 的 fetch-messages 事件
  function handleFetchMessages(detail) {
    const [{ options = {} } = {}] = detail || []
    fetchMessages({ reset: options.reset })
  }

  // 发送消息（流式/普通）
  async function sendMessage(msg) {
    if (!msg || !msg.content)
      return
    // 先 push 用户消息
    messages.value.push({
      _id: `user_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`,
      content: msg.content,
      senderId: currentUserId.value,
      username: 'User',
      timestamp: new Date().toString().substring(16, 21),
      date: new Date().toDateString(),
    })
    const sessionId = await ensureChatSession()
    const token = authStore?.accessToken
    // 流式响应
    await chatApi.streamChat(backendUserId.value, [{ role: 'user', content: msg.content }], {
      token,
      onData: (data) => {
        try {
          const content = data?.choices?.[0]?.message?.content || ''
          if (!content)
            return
          const lastMessage = messages.value[messages.value.length - 1]
          if (lastMessage && lastMessage.senderId === 'assistant') {
            lastMessage.content += content
          }
          else {
            messages.value.push({
              _id: `assistant_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`,
              content,
              senderId: 'assistant',
              username: 'TiDB Assistant',
              timestamp: new Date().toString().substring(16, 21),
              date: new Date().toDateString(),
            })
          }
        }
        catch (error) {
          console.error('[useChat] Error processing stream data:', error)
        }
      },
      onError: () => {
        messages.value.push({
          _id: messages.value.length,
          content: '抱歉，回答生成失败，请重试',
          senderId: 'assistant',
          username: 'System',
          timestamp: new Date().toString().substring(16, 21),
          date: new Date().toDateString(),
          system: true,
        })
      },
    })
    // 持久化
    const assistantMessage = messages.value[messages.value.length - 1]?.senderId === 'assistant'
      ? messages.value[messages.value.length - 1].content
      : ''
    if (assistantMessage && sessionId) {
      try {
        await chatApi.addChatMessage({
          userId: backendUserId.value,
          sessionId,
          userMessage: msg.content,
          assistantMessage,
        })
        saveMessagesToStorage(backendUserId.value, messages.value)
      }
      catch (error) {
        console.error('[useChat] Failed to persist chat history:', error)
      }
    }
  }

  // 兼容原有输入框发送
  function sendNative() {
    if (!composer.value.trim())
      return
    sendMessage({ content: composer.value })
    composer.value = ''
  }

  // 已彻底移除扩展场景的 startTidbChat 函数及 Promise 相关代码

  // 清理本地缓存
  function clearStorage() {
    clearMessagesFromStorage(backendUserId.value)
  }

  // 强制从服务器拉取
  function forceReload() {
    clearStorage()
    fetchMessages({ reset: true })
  }

  return {
    currentUserId,
    rooms,
    messages,
    messagesLoaded,
    composer,
    addMessages,
    fetchMessages,
    handleFetchMessages,
    sendMessage,
    sendNative,
    clearStorage,
    forceReload,
    currentSessionId,
  }
}
