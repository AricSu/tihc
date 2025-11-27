import { ref, watch } from 'vue'
import chatApi from '@/api/chat'
import { useAuthStore, useUserStore } from '@/store'

// 浏览器存储工具函数
const CHAT_STORAGE_KEY = 'tihc_chat_messages'
const MAX_STORED_MESSAGES = 5

function loadMessagesFromStorage(userId) {
  try {
    const stored = localStorage.getItem(`${CHAT_STORAGE_KEY}_${userId}`)
    if (stored) {
      const data = JSON.parse(stored)
      // 验证数据格式并检查是否过期（24小时）
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
    // 只保存最近的消息，排除用户消息（因为我们隐藏用户消息）
    const aiMessages = messages
      .filter(msg => msg.senderId === 'assistant')
      .slice(-MAX_STORED_MESSAGES)

    const dataToStore = {
      messages: aiMessages,
      timestamp: Date.now(),
    }
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

let _singleton = null

export function useChat() {
  const authStore = useAuthStore()
  const userStore = useUserStore()
  const loggedInUser = userStore.userInfo
  const backendUserId = ref(loggedInUser?.id ?? 0)
  const currentUserId = ref(loggedInUser ? String(loggedInUser.id) : 'jira-assistant')
  // simple inline Jira-styled SVG avatar as a data URL (fallback-friendly)
  const _jiraAvatarSvg = `<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'><rect width='24' height='24' fill='%230052CC'/><text x='12' y='16' font-size='12' font-family='Arial,Helvetica,sans-serif' fill='white' text-anchor='middle' font-weight='700'>J</text></svg>`
  const jiraAvatarDataUrl = `data:image/svg+xml;utf8,${encodeURIComponent(_jiraAvatarSvg)}`

  const rooms = ref([
    {
      roomId: 'general',
      roomName: 'General AI',
      avatar: jiraAvatarDataUrl,
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

  watch(
    () => userStore.userInfo,
    (value) => {
      const hasUser = Boolean(value?.id)
      backendUserId.value = hasUser ? value.id : 0
      currentUserId.value = hasUser ? String(value.id) : 'jira-assistant'

      const room = rooms.value[0]
      if (room) {
        room.users = [
          { _id: 'jira-assistant', username: 'jira assistant' },
          ...(hasUser
            ? [{
                _id: String(value.id),
                username: value.nickName || value.username || 'current user',
              }]
            : []),
        ]
      }
    },
    { immediate: false },
  )

  const messages = ref([])
  const messagesLoaded = ref(false)
  const composer = ref('')
  const activeRoomId = ref(null)
  const loadingHistory = ref(false)
  const currentSessionId = ref(null)
  // Track active runtime listeners so we can remove them on destroy
  const activeListeners = new Map()

  function addMessages(_reset = false) {
    console.log('[useChat] addMessages called, reset:', _reset)
    // 保留此函数以保持兼容性
    return []
  }

  async function ensureChatSession(title = 'default') {
    if (currentSessionId.value) {
      console.log('[useChat] Existing sessionId:', currentSessionId.value)
      return currentSessionId.value
    }
    try {
      console.log('[useChat] Creating new chat session for user:', backendUserId.value, 'title:', title)
      const response = await chatApi.createChatSession(backendUserId.value, title)
      const session = response?.data
      if (session?.id) {
        currentSessionId.value = session.id
        console.log('[useChat] New sessionId:', session.id)
      }
    }
    catch (error) {
      console.error('[useChat] Failed to create chat session:', error)
    }
    return currentSessionId.value
  }

  async function loadChatHistory(reset = false) {
    if (loadingHistory.value) {
      console.log('[useChat] Already loading history, abort')
      return
    }
    loadingHistory.value = true
    try {
      console.log('[useChat] Attempting to load chat history, reset:', reset)
      // 首先尝试从浏览器存储加载数据
      const storedMessages = loadMessagesFromStorage(backendUserId.value)
      if (storedMessages.length > 0 && reset) {
        console.warn('📱 [useChat] Loading messages from browser storage:', storedMessages.length)
        messages.value = storedMessages
        messagesLoaded.value = true
        loadingHistory.value = false
        return
      }
      // 如果没有存储的数据或不是重置操作，从服务器获取
      console.warn('🌐 [useChat] Loading messages from server...')
      const sessionId = await ensureChatSession()
      console.log('[useChat] Using sessionId for history:', sessionId)
      const historyResponse = await chatApi.getChatHistory(backendUserId.value, { sessionId, limit: 5 })
      const histories = Array.isArray(historyResponse?.data) ? historyResponse.data : []
      console.log('[useChat] Server returned histories:', histories)
      if (!currentSessionId.value && histories.length > 0)
        currentSessionId.value = histories[0].sessionId
      // 转换历史记录格式 - 只显示AI回复，隐藏用户消息
      const historyMessages = []
      histories.forEach((h, index) => {
        const timestamp = h.timestamp ? new Date(h.timestamp) : new Date()
        const timeStr = timestamp.toString().substring(16, 21)
        const dateStr = timestamp.toDateString()
        // 只添加助手回复，用户问题作为replyMessage展示上下文
        if (h.assistant) {
          historyMessages.push({
            _id: `history_assistant_${index}`,
            content: h.assistant,
            senderId: 'assistant',
            username: 'TiDB Assistant',
            timestamp: timeStr,
            date: dateStr,
            system: false,
            // 在回复消息中显示用户的问题作为上下文
            replyMessage: h.user
              ? {
                  content: h.user,
                  senderId: currentUserId.value,
                }
              : null,
          })
        }
      })
      console.log('[useChat] Parsed historyMessages:', historyMessages)
      if (reset) {
        messages.value = historyMessages
        console.log('[useChat] Messages reset to historyMessages')
      }
      else {
        messages.value = [...messages.value, ...historyMessages]
        console.log('[useChat] Messages appended with historyMessages')
      }
      // 保存到浏览器存储
      if (historyMessages.length > 0) {
        saveMessagesToStorage(backendUserId.value, historyMessages)
        console.warn('💾 [useChat] Messages saved to browser storage')
      }
      messagesLoaded.value = true
    }
    catch (error) {
      console.error('[useChat] Failed to load chat history:', error)
    }
    finally {
      loadingHistory.value = false
      console.log('[useChat] Finished loading history')
    }
  }

  function handleFetchMessages(detail) {
    const [{ options = {} } = {}] = detail || []
    console.log('[useChat] handleFetchMessages called, options:', options)
    // 使用真实的历史记录替代模拟数据
    loadChatHistory(options.reset)
  }

  async function handleSendMessage(detail) {
    const [message] = detail || []
    console.log('[useChat] handleSendMessage called, message:', message)
    if (!message) {
      console.log('[useChat] No message provided, abort')
      return
    }
    const sessionId = await ensureChatSession()
    console.log('[useChat] Using sessionId for sendMessage:', sessionId)
    const userMessageContent = message.content
    // 不显示用户消息，直接开始AI回复
    console.warn('📤 [useChat] Processing user message (hidden):', userMessageContent.slice(0, 50))
    // 使用流式 API
    const token = authStore?.accessToken
    chatApi.streamChat(backendUserId.value, [{ role: 'user', content: userMessageContent }], {
      token,
      onData: (data) => {
        console.warn('📨 [useChat] Received stream data:', data?.choices?.[0]?.message?.content?.slice(0, 50)) // Debug log
        try {
          const content = data?.choices?.[0]?.message?.content || ''
          if (!content) {
            console.log('[useChat] No content in stream data')
            return
          }
          const lastMessage = messages.value[messages.value.length - 1]
          if (lastMessage && lastMessage.senderId === 'assistant') {
            // 追加内容到现有回复
            lastMessage.content += content
            console.warn('📝 [useChat] Updated existing message, new length:', lastMessage.content.length) // Debug log
          }
          else {
            // 创建新的助手回复
            const assistantMsgId = `assistant_${Date.now()}_${Math.random().toString(36).slice(2, 8)}`
            const newMessage = {
              _id: assistantMsgId,
              content,
              senderId: 'assistant',
              username: 'TiDB Assistant',
              timestamp: new Date().toString().substring(16, 21),
              date: new Date().toDateString(),
            }
            messages.value.push(newMessage)
            console.warn('📝 [useChat] Created new assistant message, total messages:', messages.value.length) // Debug log
          }
        }
        catch (error) {
          console.error('[useChat] Error processing stream data:', error)
        }
      },
      onError: (error) => {
        console.error('[useChat] Chat stream error:', error)
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
      .then(async () => {
        const assistantMessage = messages.value[messages.value.length - 1]?.senderId === 'assistant'
          ? messages.value[messages.value.length - 1].content
          : ''
        if (!assistantMessage || !sessionId) {
          console.log('[useChat] No assistantMessage or sessionId, abort persist')
          return
        }
        try {
          await chatApi.addChatMessage({
            userId: backendUserId.value,
            sessionId,
            userMessage: userMessageContent,
            assistantMessage,
          })
          // 保存最新消息到浏览器存储
          saveMessagesToStorage(backendUserId.value, messages.value)
          console.warn('💾 [useChat] Updated messages saved to browser storage')
        }
        catch (error) {
          console.error('[useChat] Failed to persist chat history:', error)
        }
      })
      .catch((error) => {
        console.error('[useChat] Failed to start stream chat:', error)
      })
  }

  function sendNative() {
    if (!composer.value.trim())
      return
    handleSendMessage([{ content: composer.value }])
    composer.value = ''
  }

  async function startTidbChat(opts = {}) {
    const tokenFromStore = authStore.accessToken
    const token = opts.token || tokenFromStore || undefined
    if (!token)
      throw new Error('token required for tidb chat')

    const userText = opts.userMessage ?? composer.value
    if (!userText)
      return

    // append user message
    messages.value.push({
      _id: messages.value.length,
      content: userText,
      senderId: currentUserId.value,
      username: 'jira assistant',
      timestamp: new Date().toString().substring(16, 21),
      date: new Date().toDateString(),
    })

    // placeholder assistant message
    const aiIndex = messages.value.length
    messages.value.push({ _id: aiIndex, content: '', senderId: 'assistant', username: 'Assistant', timestamp: new Date().toString().substring(16, 21), date: new Date().toDateString() })

    // build request id
    const requestId = `tidb-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`

    // Prepare messages payload (simple mapping)
    const chatMessages = messages.value.map(m => ({ role: m.senderId === currentUserId.value ? 'user' : m.senderId === 'assistant' ? 'assistant' : 'system', content: String(m.content) }))

    const runtimeApi = window.browser ?? window.chrome

    return new Promise((resolve, reject) => {
      let settled = false
      const timeoutMs = opts.timeoutMs ?? 120000 // default 2 min

      const cleanup = () => {
        try {
          if (activeListeners.has(requestId)) {
            const handler = activeListeners.get(requestId)
            if (runtimeApi?.runtime?.onMessage?.removeListener)
              runtimeApi.runtime.onMessage.removeListener(handler)
            activeListeners.delete(requestId)
          }
        }
        catch (e) {
          console.warn('[useChat] cleanup error', e)
        }
      }

      const onRuntime = (msg) => {
        try {
          if (!msg || msg.requestId !== requestId)
            return
          if (msg.type === 'chat-chunk') {
            const msgObj = messages.value[aiIndex]
            if (msgObj)
              msgObj.content = (msgObj.content || '') + msg.chunk
          }
          else if (msg.type === 'chat-done') {
            if (!settled) {
              settled = true
              cleanup()
              resolve()
            }
          }
          else if (msg.type === 'chat-error') {
            const msgObj = messages.value[aiIndex]
            if (msgObj)
              msgObj.content = `Error: ${msg.error}`
            if (!settled) {
              settled = true
              cleanup()
              reject(new Error(msg.error || 'unknown tidb error'))
            }
          }
        }
        catch (e) {
          console.error('runtime message handler error', e)
        }
      }

      if (runtimeApi?.runtime?.onMessage?.addListener) {
        // 支持 abort/cancel
        runtimeApi.runtime.onMessage.addListener(onRuntime)
        // 启动流式聊天（可根据实际参数调整）
        const userMessageContent = composer.value || ''
        const streamPromise = chatApi.streamChat(
          backendUserId.value,
          [{ role: 'user', content: userMessageContent }],
          {
            onData: (data) => {
              // 处理流式数据
              const msgObj = messages.value[aiIndex]
              if (msgObj) {
                msgObj.content = (msgObj.content || '') + (data.chunk || '')
              }
            },
            onError: (error) => {
              cleanup()
              reject(error)
            },
            onComplete: () => {
              cleanup()
              resolve()
            },
            token,
          },
        )
        // 支持 abort/cancel
        streamPromise.abort = () => {
          cleanup()
        }
      }

      // safety timeout
      const to = setTimeout(() => {
        if (!settled) {
          settled = true
          cleanup()
          const msgObj = messages.value[aiIndex]
          if (msgObj)
            msgObj.content = `Error: stream timeout`
          reject(new Error('stream timeout'))
        }
      }, timeoutMs)

      // send request to background to start streaming
      if (runtimeApi?.runtime?.sendMessage) {
        runtimeApi.runtime.sendMessage({ type: 'startTidbChat', token, userMessage: userText, requestId, messages: chatMessages })
      }
      else {
        clearTimeout(to)
        reject(new Error('runtime not available'))
      }
    })
  }

  function getIssueKeyFromUrl(url) {
    try {
      const u = url.trim()
      const m = u.match(/atlassian\.net\/browse\/([\w-]+-\d+)/i)
      if (m && m[1])
        return m[1].toUpperCase()
      const m2 = u.match(/browse\/([\w-]+-\d+)/i)
      if (m2 && m2[1])
        return m2[1].toUpperCase()
    }
    catch {
      // ignore
    }
    return null
  }

  function ensureGeneralRoom() {
    const exists = rooms.value.find(r => r.roomId === 'general')
    if (!exists) {
      rooms.value.unshift({ roomId: 'general', roomName: 'General', avatar: '', users: [] })
    }
  }

  function openRoomForUrl(url) {
    const key = getIssueKeyFromUrl(url)
    ensureGeneralRoom()
    if (!key) {
      activeRoomId.value = 'general'
      return { roomId: 'general', created: false }
    }

    const roomId = key
    let room = rooms.value.find(r => r.roomId === roomId)
    let created = false
    if (!room) {
      room = { roomId, roomName: key, avatar: '', users: [] }
      const genIndex = rooms.value.findIndex(r => r.roomId === 'general')
      if (genIndex >= 0)
        rooms.value.splice(genIndex + 1, 0, room)
      else rooms.value.push(room)
      created = true
    }

    activeRoomId.value = roomId
    return { roomId, created }
  }

  function destroy() {
    try {
      const runtimeApi = window.browser ?? window.chrome
      for (const [rid, handler] of activeListeners.entries()) {
        if (runtimeApi?.runtime?.onMessage?.removeListener)
          runtimeApi.runtime.onMessage.removeListener(handler)
        activeListeners.delete(rid)
      }
    }
    catch (e) {
      console.warn('[useChat] destroy error', e)
    }
  }

  // 清理浏览器存储中的聊天数据
  function clearChatStorage() {
    clearMessagesFromStorage(backendUserId.value)
    console.warn('🗑️ Cleared chat messages from browser storage')
  }

  // 强制从服务器重新加载数据
  function forceReloadFromServer() {
    clearChatStorage()
    loadChatHistory(true)
  }

  _singleton = {
    currentUserId,
    rooms,
    messages,
    messagesLoaded,
    composer,
    addMessages,
    handleFetchMessages,
    handleSendMessage,
    sendNative,
    startTidbChat,
    destroy,
    activeRoomId,
    openRoomForUrl,
    getIssueKeyFromUrl,
    loadChatHistory,
    currentSessionId,
    // 浏览器存储管理方法
    clearChatStorage,
    forceReloadFromServer,
  }

  return _singleton
}
