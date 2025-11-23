<template>
  <div class="chat-root-wrapper">
    <div class="chat-widget">
      <vue-advanced-chat
        ref="chatWindow"
        class="chat-host"
        :height="screenHeight"
        :room-message="roomMessage"
        show-audio="false"
        show-files="false"
        show-search="false"
        show-add-room="false"
        :current-user-id="currentUserId"
        :rooms="JSON.stringify(rooms)"
        :rooms-loaded="true"
        :messages="JSON.stringify(messages)"
        :messages-loaded="messagesLoaded"
        :single-room="true"
        :responsive-breakpoint="0"
        @send-message="sendMessage"
        @fetch-messages="fetchMessages"
      />
    </div>
  </div>
</template>

<script>
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useChat } from '../../composables/useChat.js'

// `vue-advanced-chat` is registered at the sidepanel entry (`sidepanel.ts`).
// This component now delegates state and handlers to the shared `useChat` composable
export default {
  setup() {
    const chat = useChat()

    const roomMessage = ref('')
    // detect small-screen devices; keep in sync on resize
    const isDevice = ref(typeof window !== 'undefined' ? window.innerWidth <= 900 : false)
    function onResize() {
      isDevice.value = window.innerWidth <= 900
    }
    onMounted(() => {
      window.addEventListener('resize', onResize)
      // 初始加载历史记录
      fetchMessages({ detail: [{ options: { reset: true } }] })
    })
    onUnmounted(() => {
      window.removeEventListener('resize', onResize)
    })

    const screenHeight = computed(() => (isDevice.value ? `${window.innerHeight}px` : '100%'))

    function menuActionHandler(evt) {
      // The library emits an event with detail usually in detail[0]. Try to
      // extract action/message in a defensive way and set roomMessage so the
      // web component can display the reply preview.
      const detail = evt && evt.detail ? (evt.detail[0] || evt.detail) : evt
      const action = detail && (detail.action || detail)
      if (action && action.name === 'reply') {
        // try common shapes: detail.message, action.message, or detail.payload
        const msg = detail.message || action.message || detail.payload || ''
        roomMessage.value = msg
      }
    }

    function fetchMessages(evt) {
      // vue-advanced-chat emits detail as an array; forward to composable
      chat.handleFetchMessages([evt.detail && evt.detail[0] ? evt.detail[0] : undefined])
    }

    function addMessages(reset) {
      // reuse composable's helper (if available)
      return typeof chat.addMessages === 'function' ? chat.addMessages(reset) : []
    }

    function sendMessage(evt) {
      const msg = evt.detail && evt.detail[0] ? evt.detail[0] : evt.detail
      chat.handleSendMessage([msg])
      // clear any active reply anchor
      roomMessage.value = ''
    }

    // 开发调试函数
    function clearStorageAndReload() {
      chat.forceReloadFromServer()
    }

    return {
      currentUserId: chat.currentUserId,
      rooms: chat.rooms,
      messages: chat.messages,
      messagesLoaded: chat.messagesLoaded,
      fetchMessages,
      addMessages,
      sendMessage,
      roomMessage,
      menuActionHandler,
      clearStorageAndReload,
      // expose for template / debugging
      isDevice,
      screenHeight,
    }
  },
  // 在组件挂载后注入自定义CSS到Shadow DOM
  mounted() {
    // 等待下一个tick确保Shadow DOM已经创建
    this.$nextTick(() => {
      const injectStyle = (chatWindow) => {
        if (chatWindow && chatWindow.shadowRoot) {
          const style = document.createElement('style')
          style.innerHTML = `
            .vac-message-wrapper .vac-message-box {
              flex: 0 0 100% !important;
              max-width: 100% !important;
              width: 100% !important;
            }
            .vac-message-container {
              width: 100% !important;
              max-width: 100% !important;
            }
            /* 消息内容折叠展示，超出2行省略号 */
            .vac-message-text {
              display: -webkit-box !important;
              -webkit-line-clamp: 2 !important;
              -webkit-box-orient: vertical !important;
              overflow: hidden !important;
              text-overflow: ellipsis !important;
              white-space: normal !important;
              word-break: break-all !important;
              max-height: 3.2em !important;
            }
            /* 只隐藏横向滚动条，保留竖向滚动条 */
            ::-webkit-scrollbar:horizontal {
              display: none !important;
              height: 0 !important;
            }
            /* 兼容 Firefox/IE 横向滚动条 */
            .vac-message-list,
            .vac-message-container {
              overflow-x: hidden !important;
            }
          `
          chatWindow.shadowRoot.appendChild(style)
        }
      }
      const chatWindow = this.$refs.chatWindow
      if (chatWindow && chatWindow.shadowRoot) {
        injectStyle(chatWindow)
      }
      else {
        console.warn('Shadow DOM not found, retrying in 1 second...')
        setTimeout(() => {
          const chatWindow = this.$refs.chatWindow
          if (chatWindow && chatWindow.shadowRoot) {
            injectStyle(chatWindow)
          }
        }, 1000)
      }
    })
  },
  // no Options API computed; all reactive values come from setup()
}
</script>

<style scoped>
/* 只隐藏横向滚动条，保留竖向滚动条 */
.chat-root-wrapper,
.chat-widget,
.chat-host {
  overflow-x: hidden !important;
}
.chat-root-wrapper::-webkit-scrollbar:horizontal,
.chat-widget::-webkit-scrollbar:horizontal,
.chat-host::-webkit-scrollbar:horizontal {
  display: none !important;
  height: 0 !important;
}

/* Fill the parent content area and make the chat element stretch */
.chat-root-wrapper {
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  min-height: 0;
  min-width: 0;
  height: 100%;
}

.chat-widget {
  flex: 1 1 auto;
  min-height: 0;
  height: 100%;
  min-width: 0;
  display: flex;
}

.chat-host {
  flex: 1 1 auto;
  height: 100%;
  min-width: 0;
  min-height: 0;
}
</style>
