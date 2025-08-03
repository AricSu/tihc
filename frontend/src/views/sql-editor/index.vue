<template>
  <div class="bytebase-sql-editor">
    <EditorHeader
      :selected-connection="sqlEditor.currentConnection"
      :connection-options="sqlEditor.connections"
      :connection-status="currentConnectionStatus"
      @switch-connection="conn => handleConnectionAction('switch', conn)"
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
              :connection-status="currentConnectionStatus"
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
          :connected="currentConnectionStatus === 'connected'"
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
      @save-connection="conn => handleConnectionAction('save', conn)"
      @update-connection="conn => handleConnectionAction('update', conn)"
      @test-connection="conn => handleConnectionAction('test', conn)"
      @connect-to-saved="conn => handleConnectionAction('switch', conn)"
      @delete-connection="conn => handleConnectionAction('delete', conn)"
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
// 慢日志相关事件处理
function handleScanFiles(files: any) {
  // 这里可以根据实际需求处理文件列表
  sqlEditor.slowlogFiles = files
  window.$message?.success('慢日志文件已扫描')
}

function handleProcessFiles(files: any) {
  // 这里可以根据实际需求处理文件内容
  // 例如触发后端 API 或更新 store
  window.$message?.success('慢日志文件已处理')
}
import EditorHeader from './components/EditorHeader.vue'
import SqlEditorSidebar from './components/SqlEditorSidebar.vue'
import QueryEditor from './components/QueryEditor.vue'
import ResultsPanel from './components/ResultsPanel.vue'
import ConnectionManager from './components/ConnectionManager.vue'
import SettingsForm from './components/SettingsForm.vue'
import QueryHistory from './components/QueryHistory.vue'
import SlowlogDrawer from './components/SlowlogDrawer.vue'
import {
  createConnection,
  testConnection,
  deleteConnection,
  listConnections,
  handleUpdateConnection,
  Connection
} from '@/api/connection'
import { computed, ref } from 'vue'
import { useSqlEditorStore } from '@/store/modules/sqlEditor'

const sqlEditor = useSqlEditorStore()
const isMac = /Mac/.test(navigator.userAgent)
const isExecuting = computed(() => sqlEditor.isExecuting)
const lineCount = computed(() => sqlEditor.sqlContent.split('\n').length)
const slowlogDrawerRef = ref()

// SQL 执行功能
import { executeSql } from '@/api/sql'
sqlEditor.executeQuery = async function () {
  const sql = sqlEditor.sqlContent
  const connection_id = sqlEditor.currentConnection?.id
  if (!sql || !connection_id) {
    window.$message?.error('SQL 或连接未选择')
    return
  }
  sqlEditor.isExecuting = true
  try {
    const res = await executeSql({ connection_id, sql })
    const data = res.data
    sqlEditor.queryResults.push({
      id: Date.now().toString(),
      type: data.error ? 'error' : 'success',
      executionTime: data.latency_ms,
      columns: data.column_names,
      data: data.rows,
      details: data.error,
      message: data.messages,
      columnTypes: data.column_type_names,
      statement: data.statement,
      rowsCount: data.rows_count,
    })
    sqlEditor.activeResultTab = sqlEditor.queryResults[sqlEditor.queryResults.length - 1].id
  } catch (e) {
    window.$message?.error('SQL 执行失败')
  } finally {
    sqlEditor.isExecuting = false
  }
}

// 当前连接状态从 connections 列表查找
const currentConnectionStatus = computed(() => {
  const conn = sqlEditor.connections.find(c => c.id === sqlEditor.currentConnection?.id)
  return conn?.status || 'disconnected'
})

// 统一连接管理方法
async function handleConnectionAction(action, conn) {
  showGlobalLoading()
  try {
    let res
    switch (action) {
      case 'save':
        await createConnection({ ...conn, id: conn.id ?? Date.now(), use_tls: conn.use_tls ?? false, ca_cert_path: conn.ca_cert_path ?? '' })
        await fetchConnections()
        handleQuerySuccess('连接已保存')
        break
      case 'update':
        await handleUpdateConnection({ ...conn, use_tls: conn.use_tls ?? false, ca_cert_path: conn.ca_cert_path ?? '', id: conn.id })
        await fetchConnections()
        handleQuerySuccess('连接已更新')
        break
      case 'delete':
        await deleteConnection(conn.id)
        await fetchConnections()
        handleQuerySuccess('连接已删除')
        break
      case 'test':
        res = await testConnection({ ...conn, use_tls: conn.use_tls ?? false, ca_cert_path: conn.ca_cert_path ?? '' })
        updateConnectionStatus(conn, res.data.status === 'success' ? 'connected' : 'disconnected')
        if (res.data.status === 'success') {
          handleQuerySuccess('连接测试成功')
        } else {
          handleQueryError(res.data.message || '连接测试失败')
        }
        break
      case 'switch':
        sqlEditor.setCurrentConnection(conn)
        await handleConnectionAction('test', conn)
        break
    }
  } catch {
    handleQueryError(`${getActionErrorMsg(action)}`)
    if (action === 'test' || action === 'switch') {
      updateConnectionStatus(conn, 'disconnected')
    }
  }
}

function updateConnectionStatus(conn, status) {
  const idx = sqlEditor.connections.findIndex(c => c.id === conn.id)
  if (idx !== -1) sqlEditor.connections[idx].status = status
}

function getActionErrorMsg(action) {
  switch (action) {
    case 'save': return '保存连接失败'
    case 'update': return '更新连接失败'
    case 'delete': return '删除连接失败'
    case 'test': return '连接测试失败'
    case 'switch': return '切换连接失败'
    default: return '操作失败'
  }
}

async function fetchConnections() {
  try {
    const res = await listConnections()
    sqlEditor.connections = (res.data.data || []).map(conn => ({ ...conn, status: conn.status || 'disconnected' }))
  } catch {
    sqlEditor.connections = []
  }
}

// 页面首次加载时同步后端连接列表
fetchConnections()

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


