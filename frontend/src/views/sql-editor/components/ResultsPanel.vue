<template>
  <div class="results-panel">
    <n-tabs 
      v-if="queryResults.length > 0" 
      :value="activeResultTab"
      @update:value="$emit('update:activeResultTab', $event)"
      type="card"
      animated
      closable
      @close="$emit('close-tab', $event)"
      class="results-tabs"
    >
      <n-tab-pane 
        v-for="(result, index) in queryResults" 
        :key="result.id"
        :name="result.id"
        :tab="formatTabLabel(result, index)"
      >
        <div class="result-content">
          <div v-if="result.type === 'success'" class="result-success">
            <div class="result-header">
              <div class="result-meta">
                <n-tag type="success" size="small">Success</n-tag>
                <n-text depth="3">{{ result.data?.length || 0 }} rows returned in {{ result.executionTime }}ms</n-text>
              </div>
              <div class="result-actions">
                <n-button-group size="small">
                  <n-button @click="$emit('export-data', result, 'csv')">Export CSV</n-button>
                  <n-button @click="$emit('export-data', result, 'json')">Export JSON</n-button>
                  <n-button @click="$emit('copy-result', result)">Copy</n-button>
                  <n-button @click="$emit('delete-result', result.id)" type="error" ghost>
                    <template #icon><n-icon>🗑️</n-icon></template>
                    Delete
                  </n-button>
                </n-button-group>
              </div>
            </div>
            <n-data-table
              v-if="result.data && result.data.length > 0"
              :columns="result.columns"
              :data="result.data"
              :pagination="{ pageSize: 50, showSizePicker: true, pageSizes: [20, 50, 100, 200, 500], showQuickJumper: true, prefix: ({ itemCount }) => `Total ${itemCount} rows` }"
              :scroll-x="Math.max(1200, (result.columns?.length ?? 0) * 150)"
              size="small"
              bordered
              striped
              virtual-scroll
              :max-height="500"
              :row-key="(row) => row._rowIndex"
              flex-height
              style="min-height: 200px;"
            />
            <n-empty v-else description="No data returned" />
          </div>
          <div v-else-if="result.type === 'error'" class="result-error">
            <n-alert type="error" :title="`SQL Error (${result.executionTime}ms)`">
              <pre class="error-details">{{ result.details }}</pre>
            </n-alert>
          </div>
          <div v-else-if="result.type === 'non-query'" class="result-non-query">
            <n-alert type="info" :title="`Query executed successfully (${result.executionTime}ms)`">
              {{ result.message }}
            </n-alert>
          </div>
        </div>
      </n-tab-pane>
    </n-tabs>
    <n-empty 
      v-else 
      description="No results to display. Run a query to see results here." 
      style="margin-top: 100px;"
    >
      <template #extra>
        <div style="margin-top: 16px; color: #6b7280; font-size: 12px;">
          <div>💡 Shortcuts:</div>
          <div>• {{ isMac ? '⌘' : 'Ctrl' }}+Enter: Execute query</div>
        </div>
      </template>
    </n-empty>
  </div>
</template>

<script setup lang="ts">
interface QueryResult {
  id: string
  type: 'success' | 'error' | 'non-query'
  executionTime: number
  data?: any[]
  columns?: any[]
  details?: string
  message?: string
}

const props = defineProps<{
  queryResults: QueryResult[],
  activeResultTab: string,
  isMac: boolean
}>()
const emit = defineEmits(['close-tab', 'export-data', 'copy-result', 'delete-result', 'update:activeResultTab'])
function formatTabLabel(result: any, index: number) {
  const queryNum = `Q${index + 1}`
  const time = `${result.executionTime}ms`
  let statusIcon = ''
  if (result.type === 'success') {
    const rowCount = result.data?.length || 0
    const formattedCount = rowCount.toLocaleString()
    statusIcon = `✓ ${formattedCount} row${rowCount !== 1 ? 's' : ''}`
  } else if (result.type === 'error') {
    statusIcon = '✗ Error'
  } else {
    statusIcon = 'ℹ Info'
  }
  return `${queryNum} • ${statusIcon} • ${time}`
}
</script>

<style scoped>
.results-panel {
  height: 100%;
  background: white;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
.results-tabs {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
.results-tabs .n-tabs-pane {
  flex: 1;
  overflow: auto;
}
.result-content {
  padding: 16px;
}
.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}
.result-meta {
  display: flex;
  align-items: center;
  gap: 12px;
}
.result-actions {
  display: flex;
  gap: 8px;
}
.error-details {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 12px;
}
</style>
