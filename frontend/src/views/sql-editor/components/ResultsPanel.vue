<template>
  <div class="results-panel">
    <n-tabs
      v-if="queryResults.length > 0"
      :value="activeResultTab"
      type="card"
      animated
      closable
      @update:value="$emit('update:activeResultTab', $event)"
      @close="$emit('delete-result', $event)"
      class="results-tabs"
    >
      <n-tab-pane
        v-for="(result, index) in queryResults"
        :key="result.id"
        :name="result.id"
        :tab="formatTabLabel(result, index)"
      >
        <div class="result-tab-pane-content">
          <div class="result-header">
            <n-tag :type="result.error ? 'error' : 'success'" size="small">{{ result.error ? t('sqlEditor.error') : t('sqlEditor.success') }}</n-tag>
            <n-text depth="3">{{ getDisplayText(result) }} • {{ t('sqlEditor.executionTime', { time: result.executionTime ?? result.latency_ms ?? 0 }) }}</n-text>
            <div class="result-actions">
              <n-button size="small" @click="handleExport(result, 'csv')">{{ t('sqlEditor.copyCsv') }}</n-button>
              <n-button size="small" @click="handleExport(result, 'json')">{{ t('sqlEditor.copyJson') }}</n-button>
              <n-button size="small" @click="handleCopy(result)">{{ t('sqlEditor.copy') }}</n-button>
              <n-button size="small" type="error" ghost @click="$emit('delete-result', result.id)">{{ t('sqlEditor.delete') }}</n-button>
            </div>
          </div>
          <div class="result-table-wrapper">
            <div v-if="hasRows(result)" class="result-table-scroll-wrapper">
              <n-data-table
                :columns="getColumns(result)"
                :data="getPagedData(result)"
                :scroll-x="Math.max(1200, getColumns(result).length * 150)"
                size="small"
                bordered
                striped
                :row-key="(row) => String(row._rowIndex)"
                class="result-table no-col-ellipsis"
              />
            </div>
            <n-empty v-else :description="t('sqlEditor.noDataReturned')" class="result-empty" />
          </div>
          <div v-if="hasRows(result)" class="result-pagination-wrapper">
            <n-pagination
              :page="getPage(result.id)"
              :page-size="getPageSize(result.id)"
              :page-sizes="pageSizes"
              :item-count="getData(result).length"
              show-size-picker
              show-quick-jumper
              @update:page="p => setPage(result.id, p)"
              @update:page-size="ps => setPageSize(result.id, ps)"
            />
          </div>
        </div>
      </n-tab-pane>
    </n-tabs>
    <n-empty
      v-else
      :description="t('sqlEditor.noResultsToDisplay')"
      class="results-empty"
    >
      <template #extra>
        <div class="results-empty-extra">
          <div>💡 {{ t('sqlEditor.shortcuts') }}</div>
          <div>• {{ isMac ? '⌘' : 'Ctrl' }}+Enter: {{ t('sqlEditor.executeQuery') }}</div>
        </div>
      </template>
    </n-empty>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { h } from 'vue'
const { t } = useI18n();
const pageMap = ref<Record<string, number>>({})
const pageSizeMap = ref<Record<string, number>>({})
const defaultPageSize = 50
const pageSizes = [20, 50, 100, 200, 500]

const props = defineProps<{
  queryResults: ({
    id: string
    column_names: string[]
    column_type_names: string[]
    rows: any[][]
    rows_count?: number
    error?: string
    latency_ms?: number
    statement?: string
    messages?: { level: string; content: string }[]
    executionTime?: number
  })[],
  activeResultTab: string,
  isMac: boolean
}>()

function getPage(id: string) {
  return pageMap.value[id] || 1
}
function getPageSize(id: string) {
  return pageSizeMap.value[id] || defaultPageSize
}
function setPage(id: string, page: number) {
  pageMap.value[id] = page
}
function setPageSize(id: string, size: number) {
  pageSizeMap.value[id] = size
}


function getColumns(result) {
  const columns = Array.isArray(result.column_names)
    ? result.column_names.map((col, idx) => ({
        title: col,
        key: col,
        type: result.column_type_names?.[idx] ?? '',
        minWidth: 120,
        render: (row) => {
          const val = row[col]
          if (val == null) return ''
          const str = String(val)
          const maxLen = 60
          const display = str.length > maxLen ? str.slice(0, maxLen) + '...' : str
          return h(
            'span',
            {
              class: 'cell-ellipsis copy-cell',
              title: str,
              ondblclick: (e) => {
                copyToClipboard(str)
                window.$message?.success(t('sqlEditor.copySuccess'))
                e.stopPropagation()
              }
            },
            display
          )
        },
      }))
    : []

  const rowNumColumn = {
    title: '#',
    key: 'rowNum',
    width: 60,
    render(row, index) {
      const page = getPage(result.id)
      const pageSize = getPageSize(result.id)
      return (page - 1) * pageSize + index + 1
    },
  }

  return [rowNumColumn, ...columns]
}
function getData(result) {
  const columnNames = result.column_names || []
  return Array.isArray(result.rows) && columnNames.length
    ? result.rows.map((row, i) => {
        const obj: Record<string, any> = {}
        columnNames.forEach((colName, j) => {
          obj[colName] = Array.isArray(row) ? row[j] ?? '' : ''
        })
        obj._rowIndex = i
        return obj
      })
    : []
}
function getPagedData(result) {
  const data = getData(result)
  const page = getPage(result.id)
  const size = getPageSize(result.id)
  return data.slice((page - 1) * size, page * size)
}
function hasRows(result) {
  return getData(result).length > 0
}
function getTotalRowCount(result) {
  return getData(result).length
}
function getDisplayText(result) {
  const totalRows = getTotalRowCount(result)
  if (totalRows === 0) return t('sqlEditor.noDataReturned')
  
  const page = getPage(result.id)
  const pageSize = getPageSize(result.id)
  const startRow = (page - 1) * pageSize + 1
  const endRow = Math.min(page * pageSize, totalRows)
  
  if (totalRows <= pageSize) {
    return t('sqlEditor.rowCount', { count: totalRows })
  } else {
    return `${startRow}-${endRow} / ${t('sqlEditor.rowCount', { count: totalRows })}`
  }
}
function formatTabLabel(result, index) {
  const queryNum = t('sqlEditor.queryNum', { num: index + 1 })
  const time = `${result.executionTime ?? result.latency_ms ?? 0}${t('sqlEditor.ms')}`
  const rowCount = Array.isArray(result.rows) ? result.rows.length : 0
  const status = result.error
    ? `✗ ${t('sqlEditor.error')}`
    : `✓ ${t('sqlEditor.rowCount', { count: rowCount })}`
  return `${queryNum} • ${status} • ${time}`
}
function handleExport(result, type: 'csv' | 'json') {
  const cols = getColumns(result)
  const data = getData(result)
  if (!data.length) return window.$message?.warning(t('sqlEditor.noDataToExport'))
  if (type === 'csv') {
    const header = cols.map(c => c.title).join(',')
    const rows = data.map(row => cols.map(c => JSON.stringify(row[c.key] ?? '')).join(','))
    copyToClipboard([header, ...rows].join('\n'))
    window.$message?.success(t('sqlEditor.csvCopied'))
  } else {
    copyToClipboard(JSON.stringify(data, null, 2))
    window.$message?.success(t('sqlEditor.jsonCopied'))
  }
}
function handleCopy(result) {
  const data = getData(result)
  if (!data.length) return window.$message?.warning(t('sqlEditor.noDataToCopy'))
  copyToClipboard(JSON.stringify(data, null, 2))
  window.$message?.success(t('sqlEditor.resultCopied'))
}
function copyToClipboard(text: string) {
  if (navigator.clipboard) navigator.clipboard.writeText(text)
  else {
    const textarea = document.createElement('textarea')
    textarea.value = text
    document.body.appendChild(textarea)
    textarea.select()
    document.execCommand('copy')
    document.body.removeChild(textarea)
  }
}

</script>



<style scoped>
.copy-cell {
  cursor: pointer;
  user-select: all;
  transition: background 0.2s;
}
.copy-cell:active {
  background: #e6f7ff;
}

.results-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background-color: #fdfdfd;
}

.results-tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

/* This is a workaround to make n-tabs content area flexible */
:deep(.n-tabs-pane-wrapper) {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}
:deep(.n-tab-pane) {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  height: 100%;
}

/* 只让表头（字段名）不折叠，内容单元格可省略 */
::v-deep(.no-col-ellipsis .n-data-table-th) {
  white-space: pre;
  text-overflow: initial !important;
  overflow: visible !important;
  word-break: break-all;
  max-width: none;
}

.result-tab-pane-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  height: 100%;
  padding: 8px;
  gap: 8px;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-shrink: 0;
}

.result-actions {
  display: flex;
  gap: 8px;
}

.result-table-wrapper {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  border: 1px solid #e0e0e6;
  border-radius: 4px;
  overflow: hidden;
}

.result-table-scroll-wrapper {
  width: 100%;
  height: 100%;
  overflow-x: auto;
  overflow-y: auto;
  /* 让内容宽度撑满，横向滚动条可拖到最右 */
  box-sizing: border-box;
}

.result-table {
  background: #fff;
}

.result-pagination-wrapper {
  flex-shrink: 0;
  display: flex;
  justify-content: flex-end;
  padding-top: 8px;
  border-top: 1px solid #e0e0e6;
}

.result-empty,
.results-empty {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  height: 100%;
}

.results-empty-extra {
  margin-top: 16px;
  color: #6b7280;
  font-size: 12px;
}
</style>
