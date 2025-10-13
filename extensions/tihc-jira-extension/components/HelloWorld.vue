<template>
  <div class="sidebar-panel">
    <header class="sidebar-header">
      <span class="sidebar-title">TiHC 认证采集侧边栏</span>
    </header>
    <main class="sidebar-main">
      <h2>欢迎使用 TiHC Chrome 扩展</h2>
      <p>侧边栏已用 Vue3 重构，可在此集成采集、配置、通信等功能。</p>
      <div class="sidebar-status">
        <span>连接状态：{{ status }}</span>
        <button @click="toggleStatus">切换状态</button>
      </div>
      <div class="sidebar-actions">
        <button @click="fetchData">采集数据</button>
        <span v-if="data">采集结果：{{ data }}</span>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
const status = ref('未连接');
const data = ref('');
function toggleStatus() {
  status.value = status.value === '未连接' ? '已连接' : '未连接';
}
function fetchData() {
  data.value = '采集成功！';
}
</script>

<style scoped>
.sidebar-panel {
  background: #fff;
  min-width: 320px;
  height: 100vh;
  box-shadow: 0 2px 12px rgba(0,0,0,0.08);
  font-family: Arial, sans-serif;
  display: flex;
  flex-direction: column;
}
.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  padding: 16px;
  border-bottom: 1px solid #eee;
}
.sidebar-title {
  font-size: 1.1rem;
  font-weight: bold;
}
.sidebar-main {
  flex: 1;
  padding: 24px 16px;
}
.sidebar-status {
  margin-bottom: 16px;
}
.sidebar-actions {
  margin-bottom: 16px;
}
.sidebar-actions button {
  background: #4f8cff;
  color: #fff;
  border: none;
  border-radius: 6px;
  padding: 8px 16px;
  cursor: pointer;
  font-size: 1rem;
}
</style>
