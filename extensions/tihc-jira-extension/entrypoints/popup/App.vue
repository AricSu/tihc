
<template>
  <div class="sider-panel">
    <header class="sider-header">
      <span class="sider-title">Sider: ChatGPT 侧边栏</span>
      <button class="sider-close" @click="closePanel">×</button>
    </header>
    <main class="sider-main">
      <h1>你好,</h1>
      <p>我今天能帮你什么？</p>
      <div class="sider-actions">
        <button>全屏聊天</button>
        <button>Deep Research</button>
        <button>我的高亮</button>
        <button>AI PPT</button>
      </div>
      <div class="sider-inbox">
        <span>📧 Inbox (3) - zhipeng.su@pingcap.com <a href="#">总结</a></span>
      </div>
      <div class="sider-tools">
        <button>剪切</button>
        <button>链接</button>
        <button>书签</button>
        <button>设置</button>
        <button>定时</button>
        <button>+</button>
      </div>
      <div class="sider-input">
        <input placeholder="问任何问题，@模型，/提示" />
        <button>发送</button>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
function closePanel() {
  window.close && window.close();
}
</script>

<style scoped>
.sider-panel {
  background: #fff;
  min-width: 320px;
  height: 100vh;
  box-shadow: 0 2px 12px rgba(0,0,0,0.08);
  font-family: Arial, sans-serif;
  display: flex;
  flex-direction: column;
}
.sider-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-bottom: 1px solid #eee;
}
.sider-title {
  font-size: 1.1rem;
  font-weight: bold;
}
.sider-close {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
}
.sider-main {
  flex: 1;
  padding: 24px 16px;
}
.sider-main h1 {
  font-size: 2rem;
  margin: 0 0 8px 0;
}
.sider-main p {
  font-size: 1.1rem;
  margin-bottom: 16px;
}
.sider-actions {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}
.sider-actions button {
  background: #f5f5f5;
  border: none;
  border-radius: 6px;
  padding: 8px 12px;
  cursor: pointer;
  font-size: 0.95rem;
}
.sider-inbox {
  margin-bottom: 16px;
  font-size: 0.95rem;
  color: #555;
}
.sider-inbox a {
  color: #4f8cff;
  text-decoration: underline;
  margin-left: 8px;
}
.sider-tools {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}
.sider-tools button {
  background: #f5f5f5;
  border: none;
  border-radius: 6px;
  padding: 6px 10px;
  cursor: pointer;
  font-size: 0.9rem;
}
.sider-input {
  display: flex;
  gap: 8px;
  margin-top: 16px;
}
.sider-input input {
  flex: 1;
  padding: 8px;
  border: 1px solid #eee;
  border-radius: 6px;
  font-size: 1rem;
}
.sider-input button {
  background: #4f8cff;
  color: #fff;
  border: none;
  border-radius: 6px;
  padding: 8px 16px;
  cursor: pointer;
  font-size: 1rem;
}
</style>
