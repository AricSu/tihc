<template>
  <div class="rca-assistant">
    <!-- 左侧边栏 -->
    <div class="sidebar">
      <div class="sidebar-header">
        <button class="new-chat-btn" @click="startNewChat">
          <Icon icon="material-symbols:add" class="icon" />
          新对话
        </button>
      </div>
      
      <div class="chat-history">
        <div 
          v-for="chat in chatList" 
          :key="chat.id"
          :class="['chat-item', { active: chat.id === currentChatId }]"
          @click="selectChat(chat.id)"
        >
          <Icon icon="material-symbols:chat-bubble-outline" class="icon" />
          <span class="chat-title">{{ chat.title }}</span>
        </div>
      </div>
      
      <div class="sidebar-footer">
        <div class="user-info">
          <div class="user-avatar">
            <Icon icon="material-symbols:person" class="icon" />
          </div>
          <span>RCA Assistant</span>
        </div>
      </div>
    </div>

    <!-- 主聊天区域 -->
    <div class="main-content">
      <div class="chat-header">
        <h2>RCA 问诊台</h2>
        <p>数据库根因分析助手</p>
      </div>
      
      <div class="chat-messages" ref="messagesContainer">
        <div v-if="messages.length === 0" class="welcome-message">
          <div class="welcome-icon">🔍</div>
          <h3>欢迎使用 RCA 问诊台</h3>
          <p>我是您的数据库根因分析助手，可以帮您分析：</p>
          <ul>
            <li>🚀 数据库性能问题</li>
            <li>🐌 慢查询优化</li>
            <li>🔗 连接异常诊断</li>
            <li>📊 资源监控分析</li>
          </ul>
          <p>请描述您遇到的问题，我会为您提供专业的分析建议。</p>
        </div>
        
        <div v-for="message in messages" :key="message.id" :class="['message', message.role]">
          <div class="message-avatar">
            <Icon v-if="message.role === 'user'" icon="material-symbols:person" class="icon" />
            <Icon v-else icon="material-symbols:smart-toy-outline" class="icon" />
          </div>
          <div class="message-content">
            <div class="message-text" v-html="formatMessage(message.content)"></div>
            <div class="message-time">{{ formatTime(message.timestamp) }}</div>
          </div>
        </div>
        
        <div v-if="isLoading" class="message assistant">
          <div class="message-avatar">
            <Icon icon="material-symbols:smart-toy-outline" class="icon" />
          </div>
          <div class="message-content">
            <div class="typing-indicator">
              <span></span>
              <span></span>
              <span></span>
            </div>
          </div>
        </div>
      </div>
      
      <div class="chat-input-area">
        <div class="input-container">
          <textarea
            v-model="userInput"
            placeholder="描述您遇到的数据库问题..."
            class="message-input"
            rows="1"
            @keydown="handleKeyDown"
            @input="autoResize"
            ref="inputRef"
          ></textarea>
          <button 
            class="send-button" 
            @click="sendMessage"
            :disabled="!userInput.trim() || isLoading"
          >
            <Icon icon="material-symbols:send" class="icon" />
          </button>
        </div>
        
        <div class="quick-actions">
          <button 
            v-for="action in quickActions" 
            :key="action.key"
            class="quick-action-btn"
            @click="selectQuickAction(action)"
          >
            {{ action.label }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted, nextTick, computed } from 'vue'
import { Icon } from '@iconify/vue'
import * as rcaApi from '@/api/rca'

// 响应式数据
const messagesContainer = ref(null)
const inputRef = ref(null)
const userInput = ref('')
const isLoading = ref(false)
const currentChatId = ref(null)
const currentSessionId = ref(null)

// 消息列表
const messages = ref([])

// 聊天列表
const chatList = ref([
  { 
    id: 1, 
    title: "数据库性能问题分析",
    messages: [],
    sessionId: null
  },
  { 
    id: 2, 
    title: "慢查询优化建议",
    messages: [],
    sessionId: null
  },
  { 
    id: 3, 
    title: "连接池配置问题",
    messages: [],
    sessionId: null
  }
])

// 快捷操作
const quickActions = ref([
  { key: 'performance', label: '性能分析' },
  { key: 'slowquery', label: '慢查询分析' },
  { key: 'connection', label: '连接诊断' },
  { key: 'monitoring', label: '资源监控' }
])

// 开始新对话
const startNewChat = async () => {
  try {
    // 创建新会话
    const sessionData = await rcaApi.createSession()
    
    const newChat = {
      id: Date.now(),
      title: "新对话",
      messages: [],
      sessionId: sessionData.session_id
    }
    
    chatList.value.unshift(newChat)
    selectChat(newChat.id)
  } catch (error) {
    console.error('创建新会话失败:', error)
    // 离线模式
    const newChat = {
      id: Date.now(),
      title: "新对话",
      messages: [],
      sessionId: null
    }
    
    chatList.value.unshift(newChat)
    selectChat(newChat.id)
  }
}

// 选择对话
const selectChat = (chatId) => {
  currentChatId.value = chatId
  const chat = chatList.value.find(c => c.id === chatId)
  if (chat) {
    messages.value = [...chat.messages]
    currentSessionId.value = chat.sessionId
  }
}

// 发送消息
const sendMessage = async () => {
  if (!userInput.value.trim() || isLoading.value) return
  
  const messageText = userInput.value.trim()
  userInput.value = ''
  
  // 添加用户消息
  const userMessage = {
    id: Date.now(),
    role: 'user',
    content: messageText,
    timestamp: new Date()
  }
  
  messages.value.push(userMessage)
  
  // 更新当前聊天的消息
  const currentChat = chatList.value.find(c => c.id === currentChatId.value)
  if (currentChat) {
    currentChat.messages.push(userMessage)
    
    // 更新聊天标题
    if (currentChat.title === "新对话") {
      currentChat.title = messageText.substring(0, 20) + (messageText.length > 20 ? "..." : "")
    }
  }
  
  // 滚动到底部
  await nextTick()
  scrollToBottom()
  
  // 显示加载状态
  isLoading.value = true
  
  try {
    // 调用API获取回复
    let response
    if (currentSessionId.value) {
      response = await rcaApi.sendMessage(messageText, currentSessionId.value)
    } else {
      // 离线模式，使用本地生成的回复
      response = {
        message: generateRCAResponse(messageText),
        session_id: currentSessionId.value
      }
    }
    
    // 添加AI回复
    const aiMessage = {
      id: Date.now() + 1,
      role: 'assistant',
      content: response.message,
      timestamp: new Date()
    }
    
    messages.value.push(aiMessage)
    
    if (currentChat) {
      currentChat.messages.push(aiMessage)
    }
    
  } catch (error) {
    console.error('发送消息失败:', error)
    
    // 显示错误消息
    const errorMessage = {
      id: Date.now() + 1,
      role: 'assistant',
      content: '抱歉，我暂时无法回复您的消息。请检查网络连接或稍后再试。',
      timestamp: new Date()
    }
    
    messages.value.push(errorMessage)
    
    if (currentChat) {
      currentChat.messages.push(errorMessage)
    }
  } finally {
    isLoading.value = false
    await nextTick()
    scrollToBottom()
  }
}

// 选择快捷操作
const selectQuickAction = (action) => {
  const quickMessages = {
    performance: '我的数据库性能出现问题，请帮我分析一下',
    slowquery: '请帮我分析慢查询问题',
    connection: '数据库连接出现异常，请帮我诊断',
    monitoring: '请帮我分析数据库资源监控指标'
  }
  
  userInput.value = quickMessages[action.key]
  nextTick(() => {
    inputRef.value?.focus()
  })
}

// 处理键盘事件
const handleKeyDown = (event) => {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault()
    sendMessage()
  }
}

// 自动调整输入框高度
const autoResize = () => {
  const textarea = inputRef.value
  if (textarea) {
    textarea.style.height = 'auto'
    textarea.style.height = Math.min(textarea.scrollHeight, 120) + 'px'
  }
}

// 滚动到底部
const scrollToBottom = () => {
  if (messagesContainer.value) {
    messagesContainer.value.scrollTop = messagesContainer.value.scrollHeight
  }
}

// 格式化消息内容
const formatMessage = (content) => {
  return content
    .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
    .replace(/`(.*?)`/g, '<code>$1</code>')
    .replace(/```([\s\S]*?)```/g, '<pre><code>$1</code></pre>')
    .replace(/\n/g, '<br>')
}

// 格式化时间
const formatTime = (timestamp) => {
  return new Date(timestamp).toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 生成RCA回复（离线模式）
const generateRCAResponse = (userMessage) => {
  const lowerMessage = userMessage.toLowerCase()
  
  if (lowerMessage.includes('性能') || lowerMessage.includes('慢')) {
    return `🔍 **性能问题诊断**

基于您的描述，我建议按以下步骤进行排查：

**1. 查看慢查询日志**
\`\`\`sql
SELECT * FROM INFORMATION_SCHEMA.SLOW_QUERY 
WHERE Time > '2024-01-01' 
ORDER BY Query_time DESC LIMIT 10;
\`\`\`

**2. 检查索引使用情况**
- 确认相关表是否有适当的索引
- 使用 EXPLAIN 分析查询执行计划

**3. 监控资源使用**
- CPU 使用率
- 内存消耗 
- 磁盘 I/O

需要我帮您深入分析哪个方面？`
  } else if (lowerMessage.includes('连接') || lowerMessage.includes('connection')) {
    return `🔗 **连接问题诊断**

连接问题通常由以下原因引起：

**可能原因：**
1. 连接数达到上限
2. 网络连接不稳定
3. 认证配置问题
4. 防火墙限制

**排查步骤：**
1. 检查当前连接数：\`SHOW PROCESSLIST;\`
2. 查看最大连接数：\`SHOW VARIABLES LIKE 'max_connections';\`
3. 检查网络延迟：\`ping [数据库服务器IP]\`

您具体遇到什么连接错误信息？`
  } else if (lowerMessage.includes('资源') || lowerMessage.includes('监控')) {
    return `📊 **资源监控分析**

以下是关键监控指标：

**CPU 监控：**
- 查询处理器使用率
- 等待事件分析

**内存监控：**
- Buffer Pool 使用情况
- 缓存命中率

**存储监控：**
- 磁盘I/O延迟
- 空间使用情况

**网络监控：**
- 连接数变化
- 网络吞吐量

建议设置告警阈值，及时发现异常。需要查看具体哪项资源的详细信息？`
  } else {
    return `🤖 **RCA智能分析**

感谢您的问题！为了更好地帮您进行根因分析，请提供更多信息：

**建议描述内容：**
- 🕐 问题发生的时间
- 📊 具体的错误信息或症状  
- 🔄 问题的复现频率
- 💻 受影响的系统/应用
- 📈 相关的监控数据

您也可以直接上传日志文件或错误截图，我会帮您进行专业分析！

**快捷诊断选项：**
- 输入"性能分析"进行性能问题排查
- 输入"连接诊断"解决连接问题  
- 输入"慢查询分析"优化SQL性能`
  }
}

onMounted(() => {
  // 默认选择第一个对话
  if (chatList.value.length > 0) {
    selectChat(chatList.value[0].id)
  }
})
</script>

<style scoped>
.rca-assistant {
  display: flex;
  height: 100vh;
  background-color: #f5f5f5;
}

/* 左侧边栏 */
.sidebar {
  width: 280px;
  background-color: #1a1a1a;
  color: #ffffff;
  display: flex;
  flex-direction: column;
  border-right: 1px solid #333;
}

.sidebar-header {
  padding: 20px;
  border-bottom: 1px solid #333;
}

.new-chat-btn {
  width: 100%;
  padding: 12px 16px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  border-radius: 12px;
  color: #ffffff;
  font-weight: 500;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: all 0.3s ease;
}

.new-chat-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.chat-history {
  flex: 1;
  overflow-y: auto;
  padding: 16px 8px;
}

.chat-item {
  padding: 12px 16px;
  margin: 4px 0;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 12px;
  color: #ccc;
  border-radius: 12px;
  transition: all 0.3s ease;
}

.chat-item:hover {
  background-color: #2a2a2a;
  color: #ffffff;
}

.chat-item.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #ffffff;
}

.chat-title {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 14px;
}

.sidebar-footer {
  padding: 20px;
  border-top: 1px solid #333;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
  color: #ccc;
}

.user-avatar {
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #ffffff;
}

/* 主内容区域 */
.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  background-color: #ffffff;
  min-height: 0;
}

.chat-header {
  padding: 24px 32px;
  border-bottom: 1px solid #e5e7eb;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #ffffff;
}

.chat-header h2 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
}

.chat-header p {
  margin: 4px 0 0 0;
  opacity: 0.9;
  font-size: 14px;
}

.chat-messages {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
}

.welcome-message {
  max-width: 600px;
  margin: 0 auto;
  text-align: center;
  padding: 48px 24px;
  background: #ffffff;
  border-radius: 20px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.welcome-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.welcome-message h3 {
  color: #1a1a1a;
  margin-bottom: 16px;
  font-size: 20px;
  font-weight: 600;
}

.welcome-message p {
  color: #666;
  margin-bottom: 16px;
  line-height: 1.6;
}

.welcome-message ul {
  list-style: none;
  padding: 0;
  text-align: left;
  display: inline-block;
  margin-bottom: 16px;
}

.welcome-message li {
  padding: 8px 0;
  color: #555;
}

.message {
  display: flex;
  margin-bottom: 24px;
  max-width: 80%;
}

.message.user {
  margin-left: auto;
  flex-direction: row-reverse;
}

.message-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 12px;
  flex-shrink: 0;
}

.message.user .message-avatar {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #ffffff;
}

.message.assistant .message-avatar {
  background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
  color: #ffffff;
}

.message-content {
  flex: 1;
  background: #ffffff;
  border-radius: 18px;
  padding: 16px 20px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  position: relative;
}

.message.user .message-content {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #ffffff;
}

.message-text {
  line-height: 1.6;
  word-wrap: break-word;
}

.message-time {
  font-size: 12px;
  opacity: 0.6;
  margin-top: 8px;
}

.typing-indicator {
  display: flex;
  align-items: center;
  gap: 4px;
}

.typing-indicator span {
  width: 6px;
  height: 6px;
  background-color: #667eea;
  border-radius: 50%;
  animation: typing 1.4s infinite;
}

.typing-indicator span:nth-child(2) {
  animation-delay: 0.2s;
}

.typing-indicator span:nth-child(3) {
  animation-delay: 0.4s;
}

@keyframes typing {
  0%, 60%, 100% {
    transform: translateY(0);
  }
  30% {
    transform: translateY(-10px);
  }
}

.chat-input-area {
  padding: 24px;
  border-top: 1px solid #e5e7eb;
  background: #ffffff;
}

.input-container {
  display: flex;
  align-items: flex-end;
  gap: 12px;
  max-width: 800px;
  margin: 0 auto;
}

.message-input {
  flex: 1;
  min-height: 50px;
  padding: 16px 20px;
  border: 2px solid #e5e7eb;
  border-radius: 25px;
  font-size: 16px;
  resize: none;
  outline: none;
  transition: border-color 0.3s ease;
  font-family: inherit;
  line-height: 1.5;
}

.message-input:focus {
  border-color: #667eea;
}

.message-input::placeholder {
  color: #999;
}

.send-button {
  width: 50px;
  height: 50px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  border-radius: 50%;
  color: #ffffff;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  flex-shrink: 0;
}

.send-button:hover:not(:disabled) {
  transform: scale(1.1);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.send-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.quick-actions {
  display: flex;
  gap: 8px;
  margin-top: 16px;
  justify-content: center;
  flex-wrap: wrap;
}

.quick-action-btn {
  padding: 8px 16px;
  background: #f3f4f6;
  border: 1px solid #e5e7eb;
  border-radius: 20px;
  color: #666;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.3s ease;
}

.quick-action-btn:hover {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #ffffff;
  border-color: #667eea;
}

.icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

/* 代码样式 */
:deep(pre) {
  background-color: #f8f9fa;
  padding: 16px;
  border-radius: 8px;
  overflow-x: auto;
  margin: 12px 0;
}

:deep(code) {
  background-color: #f1f3f4;
  padding: 2px 6px;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
}

:deep(pre code) {
  background-color: transparent;
  padding: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .rca-assistant {
    flex-direction: column;
  }
  
  .sidebar {
    width: 100%;
    height: 80px;
    flex-direction: row;
    overflow-x: auto;
  }
  
  .sidebar-header {
    padding: 16px;
    border-bottom: none;
    border-right: 1px solid #333;
  }
  
  .chat-history {
    flex-direction: row;
    padding: 16px 8px;
  }
  
  .chat-item {
    white-space: nowrap;
    margin-right: 8px;
  }
  
  .sidebar-footer {
    padding: 16px;
    border-top: none;
    border-left: 1px solid #333;
  }
  
  .main-content {
    height: calc(100vh - 80px);
  }
  
  .chat-messages {
    padding: 16px;
  }
  
  .message {
    max-width: 90%;
  }
  
  .chat-input-area {
    padding: 16px;
  }
}
</style>