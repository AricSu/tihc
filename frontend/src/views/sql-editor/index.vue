<template>
  <div class="bytebase-sql-editor">
    <EditorHeader
      :selected-connection="sqlEditor.currentConnection"
      :connection-options="sqlEditor.connections"
      :connection-status="sqlEditor.currentConnection?.status || 'disconnected'"
      @switch-connection="sqlEditor.setCurrentConnection"
      @open-new-connection-modal="() => sqlEditor.setShowConnectionModal(true)"
      @open-connection-management-modal="() => sqlEditor.setShowConnectionModal(true)"
      @show-query-history="sqlEditor.setShowQueryHistory(true)"
      @show-settings="sqlEditor.setShowSettings(true)"
    />
    <n-layout has-sider style="height: calc(100vh - 120px);">
      <SqlEditorSidebar
        :showSidebar="sqlEditor.showSidebar"
        :slowQuerySchema="sqlEditor.slowQuerySchema"
        :loadingSchema="sqlEditor.loadingSchema"
        @refresh-schema="sqlEditor.refreshSchema"
        @update:showSidebar="sqlEditor.setShowSidebar($event)"
        @insert-template="sqlEditor.insertTemplate"
      />
      <n-layout-content>
        <n-split direction="vertical" :min="0.3" :max="0.7" :default-size="0.5" :resizer-style="{ backgroundColor: '#e0e6ed', height: '2px' }">
          <template #1>
            <QueryEditor
              v-model:sqlContent="sqlEditor.sqlContent"
              :is-executing="isExecuting"
              :connection-status="sqlEditor.currentConnection?.status || 'disconnected'"
              :is-mac="isMac"
              :line-count="lineCount"
              :show-slowlog-panel="sqlEditor.showSlowlogPanel"
              :sql-templates="sqlEditor.sqlTemplates"
              @execute-query="sqlEditor.executeQuery"
              @insert-template="sqlEditor.insertTemplate"
              @format-sql="sqlEditor.formatSQL"
              @clear-editor="sqlEditor.clearEditor"
              @save-query="sqlEditor.saveQuery"
              @toggle-slowlog-panel="sqlEditor.toggleSlowlogPanel"
              @keydown="sqlEditor.handleKeyDown"
              @input="sqlEditor.handleInput"
              @scroll="sqlEditor.handleScroll"
            />
          </template>
          <template #2>
            <ResultsPanel
              :query-results="sqlEditor.queryResults"
              :active-result-tab="sqlEditor.activeResultTab"
              :is-mac="isMac"
              @close-tab="sqlEditor.handleTabClose"
              @export-data="sqlEditor.exportData"
              @copy-result="sqlEditor.copyResultToClipboard"
              @delete-result="sqlEditor.deleteResult"
              @update:activeResultTab="sqlEditor.setActiveResultTab($event)"
            />
          </template>
        </n-split>
      </n-layout-content>
      <n-layout-sider v-if="sqlEditor.showSlowlogPanel" width="420" content-style="padding: 0; background: #fff;" position="right">
        <SlowlogDrawer
          ref="slowlogDrawerRef"
          v-model:show="sqlEditor.showSlowlogPanel"
          :model-value="sqlEditor.showSlowlogPanel"
          :connected="sqlEditor.currentConnection?.status === 'connected'"
          :files="sqlEditor.slowlogFiles"
          @scan-files="handleScanFiles"
          @process-files="handleProcessFiles"
        />
      </n-layout-sider>
    </n-layout>
<!-- 连接管理弹窗 -->
    <ConnectionManager
      v-model:modelValue="sqlEditor.showConnectionModal"
      :savedConnections="sqlEditor.connections"
      :currentConnection="sqlEditor.currentConnection"
      :activeTab="sqlEditor.activeTab"
      :connectingTo="sqlEditor.connectingTo"
      @update:activeTab="sqlEditor.setActiveTab"
      @save-connection="handleSaveConnection"
      @test-connection="handleTestConnection"
      @connect-to-saved="sqlEditor.setCurrentConnection"
      @duplicate-connection="sqlEditor.duplicateConnection"
      @delete-connection="handleDeleteConnection"
      @open-slowlog="sqlEditor.setShowSlowlogPanel(true)"
    />
    <!-- 设置弹窗 -->
    <n-modal v-model:show="sqlEditor.showSettings" preset="dialog" title="设置" style="width: 480px">
        <SettingsForm />
      <template #action>
        <n-button @click="sqlEditor.setShowSettings(false)">关闭</n-button>
      </template>
    </n-modal>
    <!-- 查询历史弹窗 -->
    <n-modal v-model:show="sqlEditor.showQueryHistory" preset="dialog" title="查询历史" style="width: 600px">
      <QueryHistory @restore="sqlEditor.setSqlContent($event.sql)" />
      <template #action>
        <n-button @click="sqlEditor.setShowQueryHistory(false)">关闭</n-button>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useSqlEditorStore } from '@/store/modules/sqlEditor'
import EditorHeader from './components/EditorHeader.vue'
import SqlEditorSidebar from './components/SqlEditorSidebar.vue'
import QueryEditor from './components/QueryEditor.vue'
import ResultsPanel from './components/ResultsPanel.vue'
import ConnectionManager from './components/ConnectionManager.vue'
import SettingsForm from './components/SettingsForm.vue'
import QueryHistory from './components/QueryHistory.vue'
import SlowlogDrawer from './components/SlowlogDrawer.vue'
import { ref } from 'vue'
import {
  createConnection,
  testConnection,
  deleteConnection,
  listConnections as getConnections,
  getConnection,
  updateConnection,
  Connection
} from '@/api/connection'
const sqlEditor = useSqlEditorStore()
const isMac = /Mac/.test(navigator.userAgent)
const isExecuting = computed(() => sqlEditor.isExecuting)
const lineCount = computed(() => sqlEditor.sqlContent.split('\n').length)
const slowlogDrawerRef = ref()

async function handleSaveConnection(conn: Connection) {
  showGlobalLoading()
  try {
    await createConnection(conn)
    await fetchConnections()
    handleQuerySuccess('连接已保存')
  } catch (e) {
    handleQueryError('保存连接失败')
  }
}

async function handleTestConnection(conn: Connection) {
  showGlobalLoading()
  try {
    await testConnection(conn)
    handleQuerySuccess('连接测试成功')
  } catch (e) {
    handleQueryError('连接测试失败')
  }
}

async function handleDeleteConnection(conn: Connection) {
  showGlobalLoading()
  try {
    await deleteConnection(conn.id as string | number)
    await fetchConnections()
    handleQuerySuccess('连接已删除')
  } catch (e) {
    handleQueryError('删除连接失败')
  }
}

async function fetchConnections() {
  try {
    const res = await getConnections()
    sqlEditor.connections = res.data.data || []
  } catch (e) {
    sqlEditor.connections = []
  }
}

// 初始化时自动加载连接列表
fetchConnections()


// 事件处理示例，可根据实际业务完善
// 模拟后端 API
async function fakeScanApi(logDir: string, pattern: string) {
  // 实际应为 await fetch/post
  await new Promise(r => setTimeout(r, 800))
  if (logDir && pattern) {
    return [
      { path: logDir + '/tidb-slow-20250727.log', name: 'tidb-slow-20250727.log', size: '2.1MB', modified: '2025-07-27 10:00' },
      { path: logDir + '/tidb-slow-20250726.log', name: 'tidb-slow-20250726.log', size: '1.8MB', modified: '2025-07-26 09:00' }
    ]
  }
  return []
}
async function fakeProcessApi(files) {
  // 实际应为 await fetch/post
  for (let i = 0; i <= 100; i += 20) {
    sqlEditor.slowlogProcessStatus = { progress: i, status: 'info', message: `Processing...${i}%`, details: '' }
    await new Promise(r => setTimeout(r, 200))
  }
  sqlEditor.slowlogProcessStatus = { progress: 100, status: 'success', message: 'Done', details: '' }
}

// Pinia 状态：slowlogFiles, slowlogProcessStatus
if (typeof sqlEditor.slowlogFiles === 'undefined') (sqlEditor as any).slowlogFiles = [];
if (typeof sqlEditor.slowlogProcessStatus === 'undefined') (sqlEditor as any).slowlogProcessStatus = null;

async function handleScanFiles({ logDir, pattern }) {
  showGlobalLoading()
  if (slowlogDrawerRef.value) slowlogDrawerRef.value.setScanning(true)
  const files = await fakeScanApi(logDir, pattern)
  sqlEditor.slowlogFiles = files
  if (slowlogDrawerRef.value) slowlogDrawerRef.value.setScanResult(files)
  hideGlobalLoading()
  if (files.length) {
    window.$message?.success(`共发现 ${files.length} 个日志文件`)
  } else {
    window.$message?.warning('未发现匹配文件')
  }
}
async function handleProcessFiles({ logDir, pattern, files }) {
  showGlobalLoading()
  if (slowlogDrawerRef.value) slowlogDrawerRef.value.setProcessing(true)
  await fakeProcessApi(files)
  if (slowlogDrawerRef.value) slowlogDrawerRef.value.setProcessing(false)
  hideGlobalLoading()
  window.$message?.success('慢日志解析完成')
}
// ...existing code...

// Naive UI 全局 API 类型声明（防止 TS 报错）
declare global {
  interface Window {
    $message?: any
    $loadingBar?: any
    $dialog?: any
    $notification?: any
  }
}

// 推荐用法：全局 loading
function showGlobalLoading() {
  window.$loadingBar && window.$loadingBar.start()
}
function hideGlobalLoading(success = true) {
  if (window.$loadingBar) {
    success ? window.$loadingBar.finish() : window.$loadingBar.error()
  }
}

// 推荐用法：全局消息
function handleQuerySuccess(msg = '执行成功') {
  hideGlobalLoading(true)
  window.$message && window.$message.success(msg)
}
function handleQueryError(msg = '执行失败') {
  hideGlobalLoading(false)
  window.$message && window.$message.error(msg)
}
</script>


