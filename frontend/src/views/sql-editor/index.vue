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
      :title="$t('sqlEditor.connection')"
      :new-connection-label="$t('sqlEditor.newConnection')"
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
              ref="queryEditorRef"
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
              @clear-editor="queryEditorRef?.handleClearEditor()"
              @save-query="sqlEditor.saveQuery"
              @toggle-slowlog-panel="sqlEditor.toggleSlowlogPanel"
              @keydown="sqlEditor.handleKeyDown"
              @input="sqlEditor.handleInput"
              @scroll="sqlEditor.handleScroll"
            />
          </template>
          <template #2>
            <!-- ResultsPanel 只接收严格结构化的 SqlResult[]，每个对象必须包含 column_names、rows 等字段，且 rows 为二维数组，字段顺序与 column_names 完全一致。 -->
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
    <!-- SQL 历史弹窗 -->
    <n-modal v-model:show="sqlEditor.showQueryHistory" preset="card" :title="$t('sqlEditor.history')" style="width: 600px;">
      <QueryHistory @restore="onRestoreHistory" />
    </n-modal>
    </div>
</template>

<script setup lang="ts">
// Naive UI 全局 API 类型声明（防止 TS 报错）
declare global {
  interface Window {
    $message?: any
    $loadingBar?: any
    $dialog?: any
    $notification?: any
  }
}
// --- 依赖与状态 ---
import { ref, computed, nextTick } from 'vue'
import { useSqlEditorStore } from '@/store/modules/sqlEditor'
import QueryHistory from './components/QueryHistory.vue'
import EditorHeader from './components/EditorHeader.vue'
import SqlEditorSidebar from './components/SqlEditorSidebar.vue'
import QueryEditor from './components/QueryEditor.vue'
import ResultsPanel from './components/ResultsPanel.vue'
import ConnectionManager from './components/ConnectionManager.vue'
import SlowlogDrawer from './components/SlowlogDrawer.vue'
import { executeSql } from '@/api/sql'
import {
  createConnection,
  testConnection,
  deleteConnection,
  listConnections,
  handleUpdateConnection
} from '@/api/connection'
import { useI18n } from 'vue-i18n'

// Pinia store（仅用于状态，不直接扩展 actions 类型）
const sqlEditor = useSqlEditorStore() as any
const queryEditorRef = ref()
const slowlogDrawerRef = ref()
const isMac = /Mac/.test(navigator.userAgent)
const isExecuting = computed(() => sqlEditor.isExecuting)
const lineCount = computed(() => sqlEditor.sqlContent.split('\n').length)
const { t } = useI18n()

// 当前连接状态
const currentConnectionStatus = computed(() => {
  const conn = sqlEditor.currentConnection
  return conn?.status || 'disconnected'
})

// --- 连接管理相关 ---
/**
 * 连接管理、测试、切换、保存、删除
 */
async function handleConnectionAction(action, conn) {
  showGlobalLoading()
  try {
    let res
    switch (action) {
      case 'save':
        await createConnection({ ...conn, id: conn.id ?? Date.now(), use_tls: conn.use_tls ?? false, ca_cert_path: conn.ca_cert_path ?? '' })
        await fetchConnections()
        handleQuerySuccess(t('sqlEditor.successSave'))
        break
      case 'update':
        await handleUpdateConnection({ ...conn, use_tls: conn.use_tls ?? false, ca_cert_path: conn.ca_cert_path ?? '', id: conn.id })
        await fetchConnections()
        handleQuerySuccess(t('sqlEditor.successUpdate'))
        break
      case 'delete':
        await deleteConnection(conn.id)
        await fetchConnections()
        handleQuerySuccess(t('sqlEditor.successDeleteConn'))
        break
      case 'test':
        res = await testConnection({ ...conn, use_tls: conn.use_tls ?? false, ca_cert_path: conn.ca_cert_path ?? '' })
        updateConnectionStatus(conn, res.data.status === 'success' ? 'connected' : 'disconnected')
        if (res.data.status === 'success') {
          handleQuerySuccess(t('sqlEditor.successTest'))
        } else {
          handleQueryError(res.data.message || t('sqlEditor.failTest'))
        }
        break
      case 'switch':
        sqlEditor.setCurrentConnection(conn)
        await handleConnectionAction('test', conn)
        break
    }
  } catch {
    handleQueryError(getActionErrorMsg(action))
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
    case 'save': return t('sqlEditor.failSave')
    case 'update': return t('sqlEditor.failUpdate')
    case 'delete': return t('sqlEditor.failDelete')
    case 'test': return t('sqlEditor.failTest')
    case 'switch': return t('sqlEditor.failSwitch')
    default: return t('sqlEditor.failExec')
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
fetchConnections()

// --- SQL 执行与结果管理 ---
/**
 * 执行 SQL，管理结果与历史
 */
sqlEditor.executeQuery = async function (sql) {
  const connection_id = sqlEditor.currentConnection?.id
  if (!sql || !connection_id) {
    window.$message?.error(t('sqlEditor.failNoSqlOrConn'))
    return
  }
  sqlEditor.isExecuting = true
  try {
    const result = await executeSql({ connection_id, sql })
    if (result.data.error) {
      window.$message?.error(result.data.error)
      return
    }
    sqlEditor.queryResults.push({
      id: Date.now().toString(),
      ...result.data
    })
    sqlEditor.activeResultTab = sqlEditor.queryResults[sqlEditor.queryResults.length - 1].id
    sqlEditor.history = sqlEditor.history || []
    sqlEditor.history.unshift({ sql, time: new Date().toLocaleString() })
    if (sqlEditor.history.length > 50) sqlEditor.history.length = 50
  } catch {
    window.$message?.error(t('sqlEditor.failExec'))
  } finally {
    sqlEditor.isExecuting = false
  }
}
sqlEditor.handleTabClose = function (id) {
  const idx = sqlEditor.queryResults.findIndex((r: any) => r.id === id)
  if (idx !== -1) {
    sqlEditor.queryResults.splice(idx, 1)
    if (sqlEditor.activeResultTab === id) {
      sqlEditor.activeResultTab = sqlEditor.queryResults.length > 0 ? sqlEditor.queryResults[sqlEditor.queryResults.length - 1].id : ''
    }
    window.$message?.success(t('sqlEditor.successDelete'))
  }
}
sqlEditor.deleteResult = sqlEditor.handleTabClose

// --- 编辑器与模板插入 ---
/**
 * SQL 模板插入，始终追加到编辑器末尾
 */
sqlEditor.insertTemplate = function (sql) {
  // 日志：ref/editor/model
  console.log('[insertTemplate] queryEditorRef.value:', queryEditorRef.value)
  console.log('[insertTemplate] monacoEditorRef:', queryEditorRef.value?.monacoEditorRef)
  // 1. 优先直接插入到 MonacoEditor 当前光标处
  if (queryEditorRef.value && queryEditorRef.value.monacoEditorRef?.editor) {
    const editor = queryEditorRef.value.monacoEditorRef.editor
    console.log('[insertTemplate] editor:', editor)
    const model = editor.getModel()
    console.log('[insertTemplate] model:', model)
    if (model) {
      // 获取当前光标位置
      const selection = editor.getSelection()
      let range
      let insertText = sql + '\n'
      if (selection) {
        // 如果有选区则替换选区，否则在光标处插入
        range = {
          startLineNumber: selection.startLineNumber,
          startColumn: selection.startColumn,
          endLineNumber: selection.endLineNumber,
          endColumn: selection.endColumn
        }
        // 判断是否需要在前面加换行（如光标不在行首且前面有内容）
        const lineContent = model.getLineContent(selection.startLineNumber)
        if (selection.startColumn > 1 && lineContent.trim() !== '') {
          insertText = '\n' + insertText
        }
      } else {
        // fallback: 末尾插入
        const lastLine = model.getLineCount()
        const lastCol = model.getLineMaxColumn(lastLine)
        range = {
          startLineNumber: lastLine,
          startColumn: lastCol,
          endLineNumber: lastLine,
          endColumn: lastCol
        }
        const needsNewline = lastCol > 1 && model.getLineContent(lastLine).trim() !== ''
        if (needsNewline) insertText = '\n' + insertText
      }
      console.log('[insertTemplate] range:', range)
      console.log('[insertTemplate] insertText:', insertText)
      editor.executeEdits('insert-template', [
        {
          range,
          text: insertText,
          forceMoveMarkers: true
        }
      ])
      // 移动光标到插入末尾
      const pos = editor.getPosition()
      editor.setPosition({ lineNumber: pos.lineNumber, column: pos.column })
      editor.focus()
      // 同步 Pinia store 内容
      const newValue = model.getValue()
      console.log('[insertTemplate] setSqlContent value:', newValue)
      sqlEditor.setSqlContent(newValue)
      window.$message?.success(t('sqlEditor.successInsert'))
      return
    }
  }
  // 2. fallback：直接追加到 store 内容末尾
  let newContent = ''
  console.log('[insertTemplate] fallback sqlContent:', sqlEditor.sqlContent)
  if (sqlEditor.sqlContent && sqlEditor.sqlContent.trim()) {
    newContent = sqlEditor.sqlContent.replace(/\s*$/, '') + '\n' + sql
  } else {
    newContent = sql
  }
  console.log('[insertTemplate] fallback newContent:', newContent)
  sqlEditor.setSqlContent(newContent)
  window.$message?.success(t('sqlEditor.successInsert'))
}

// --- 历史恢复 ---
/**
 * 恢复历史 SQL 到编辑器
 */
function onRestoreHistory(item) {
  const newContent = sqlEditor.sqlContent && sqlEditor.sqlContent.trim()
    ? sqlEditor.sqlContent.replace(/\s*$/, '') + '\n' + item.sql
    : item.sql
  sqlEditor.setSqlContent(newContent)
  nextTick(() => {
    sqlEditor.setShowQueryHistory(false)
    window.$message?.success(t('sqlEditor.successRestore'))
  })
}

// --- 慢日志相关 ---
function handleScanFiles(files) {
  sqlEditor.slowlogFiles = files
  window.$message?.success(t('sqlEditor.successInsert'))
}
function handleProcessFiles(files) {
  window.$message?.success(t('sqlEditor.successInsert'))
}

// --- 全局消息与 loading ---
function showGlobalLoading() {
  window.$loadingBar && window.$loadingBar.start()
}
function hideGlobalLoading(success = true) {
  if (window.$loadingBar) {
    success ? window.$loadingBar.finish() : window.$loadingBar.error()
  }
}
function handleQuerySuccess(msg = t('common.success')) {
  hideGlobalLoading(true)
  window.$message && window.$message.success(msg)
}
function handleQueryError(msg = t('common.fail')) {
  hideGlobalLoading(false)
  window.$message && window.$message.error(msg)
}
</script>


