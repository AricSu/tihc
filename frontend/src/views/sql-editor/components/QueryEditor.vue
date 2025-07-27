<template>
  <div class="query-editor">
    <div class="editor-toolbar">
      <div class="toolbar-left">
        <n-button-group>
          <n-button @click="$emit('execute-query')" type="primary" :loading="isExecuting" :disabled="!sqlContent.trim() || connectionStatus !== 'connected'">
            <template #icon><n-icon>▶️</n-icon></template>
            Run ({{ isMac ? '⌘' : 'Ctrl' }}+Enter)
          </n-button>
        </n-button-group>
        <n-divider vertical />
        <n-button-group>
          <n-button @click="$emit('format-sql')"><template #icon><n-icon>📝</n-icon></template>Format</n-button>
          <n-button @click="$emit('clear-editor')"><template #icon><n-icon>🗑️</n-icon></template>Clear</n-button>
        </n-button-group>
        <n-divider vertical />
        <n-button-group>
          <n-button @click="$emit('toggle-slowlog-panel')">{{ showSlowlogPanel ? 'Hide' : 'Show' }} Slowlog Tools</n-button>
        </n-button-group>
      </div>
      <div class="toolbar-right">
        <n-text depth="3" style="font-size: 12px;">Lines: {{ lineCount }} | Length: {{ sqlContent.length }}</n-text>
      </div>
    </div>
    <div class="code-editor-container">
      <div class="editor-gutter">
        <div v-for="index in lineCount" :key="index" class="line-number">{{ index }}</div>
      </div>
      <textarea
        :value="sqlContent"
        ref="sqlTextarea"
        class="sql-textarea"
        placeholder="-- 输入你的 SQL 查询语句\n-- 使用 Ctrl+Enter (Mac: Cmd+Enter) 执行查询\n-- 仅支持对 cluster_slow_query 表的 SELECT 操作"
        @keydown="$emit('keydown', $event)"
        @input="e => { $emit('update:sqlContent', (e.target as HTMLTextAreaElement).value); $emit('input') }"
        @scroll="$emit('scroll', $event)"
        spellcheck="false"
      ></textarea>

    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { NDrawer, NDrawerContent, NCollapse, NCollapseItem, NList, NListItem, NThing, NButton, NCode, NText } from 'naive-ui'

const props = defineProps<{
  sqlContent: string;
  isExecuting: boolean;
  connectionStatus: string;
  isMac: boolean;
  lineCount: number;
  showSlowlogPanel: boolean;
}>()
const emit = defineEmits<{
  (e: 'execute-query'): void;
  (e: 'insert-template', sql: string): void;
  (e: 'format-sql'): void;
  (e: 'clear-editor'): void;
  (e: 'toggle-slowlog-panel'): void;
  (e: 'keydown', evt: KeyboardEvent): void;
  (e: 'input'): void;
  (e: 'scroll', evt: Event): void;
  (e: 'update:sqlContent', value: string): void;
}>()
const sqlTextarea = ref<HTMLTextAreaElement | null>(null)
const showTemplateSidebar = ref(false)

// SQL模板数据，支持中英文分类和名称，后续可从文件自动导入
const sqlTemplates = [
  {
    key: 'top-slow',
    labelZh: '最慢 SQL',
    labelEn: 'Top Slowest Queries',
    category: 'slow',
    categoryZh: '慢查询',
    categoryEn: 'Slow Query',
    template: `select Time,Query_time,wait_time,Process_time,Prewrite_time,Plan_digest,Query from information_schema.cluster_slow_query where Time >= '2025-06-12 10:00:00' and Time <= '2025-06-12 11:00:00' order by Query_time desc limit 10;`
  },
  {
    key: 'long-running',
    labelZh: '执行次数最多 SQL',
    labelEn: 'Top 10 Queries by Execution Count',
    category: 'slow',
    categoryZh: '慢查询',
    categoryEn: 'Slow Query',
    template: `SELECT Digest, COUNT(*) AS exec_count, MAX(Query_time) AS max_query_time, ROUND(AVG(Query_time), 2) AS avg_query_time, SUM(Query_time) AS total_query_time, LEFT(MIN(Query), 300) AS SUB_query\nFROM information_schema.cluster_slow_query\nWHERE Time >= '2025-06-12 10:00:00' AND Time <= '2025-06-12 11:00:00'\nGROUP BY Digest\nORDER BY exec_count DESC\nLIMIT 10;`
  },
  {
    key: 'hot-region',
    labelZh: 'Top 10 热点 Region',
    labelEn: 'Top 10 Hot Region',
    category: 'region',
    categoryZh: 'Region 热点',
    categoryEn: 'Region Hot',
    template: `SELECT \n    REGION_ID, \n    TABLE_NAME, \n    INDEX_NAME, \n    AVG(FLOW_BYTES) AS AVG_FLOW_BYTES \nFROM \n    TIDB_HOT_REGIONS_HISTORY \nWHERE \n    STORE_ID = "1"\n    AND UPDATE_TIME >= '2025-06-12 10:00:00'\n    AND UPDATE_TIME <= '2025-06-12 11:00:00'\nGROUP BY \n    REGION_ID, \n    TABLE_NAME, \n    INDEX_NAME \nORDER BY \n    AVG_FLOW_BYTES DESC \nLIMIT 10;`
  },
  {
    key: 'split-region',
    labelZh: '拆分表 Region',
    labelEn: 'Split Table Regions',
    category: 'region',
    categoryZh: 'Region 管理',
    categoryEn: 'Region Management',
    template: `-- Region 详情\nSELECT * FROM information_schema.tikv_region_status WHERE region_id={{REGION_ID}};\n\n-- 查看指定表 region key 范围\nSELECT START_KEY, TIDB_DECODE_KEY(START_KEY), END_KEY, TIDB_DECODE_KEY(END_KEY) FROM information_schema.tikv_region_status WHERE  DB_NAME = 'dh_app_1709' AND TABLE_NAME='dh_active';\n\n-- 查看 region key 范围\nSELECT START_KEY, TIDB_DECODE_KEY(START_KEY) AS lower_BOUND, END_KEY, TIDB_DECODE_KEY(END_KEY) AS upper_BOUND FROM information_schema.tikv_region_status WHERE  TABLE_ID={{TABLE_ID}};\n\n-- 拆分表 region\nSPLIT TABLE table_name BETWEEN {{LOWER_BOUND}} AND {{UPPER_BOUND}} REGIONS region_num;\n\n-- 禁止合并\nALTER TABLE {TABLE_NAME} ATTRIBUTES 'merge_option=deny';\n\n`
  },
  {
    key: 'top-tiflash',
    labelZh: 'Top 10 Tiflash 查询',
    labelEn: 'Top 10 Tiflash Query',
    category: 'tiflash',
    categoryZh: 'Tiflash 查询',
    categoryEn: 'Tiflash Query',
    template: `SELECT \n    query_time,\n    sql_text,\n    user,\n    host,\n    rows_examined,\n    rows_sent\nFROM information_schema.cluster_tiflash_query \nWHERE time >= NOW() - INTERVAL 1 HOUR\nORDER BY query_time DESC \nLIMIT 10;`
  },
  {
    key: 'estimated-by-stats',
    labelZh: '按统计信息估算',
    labelEn: 'Estimated by Statistics',
    category: 'disk',
    categoryZh: '磁盘空间',
    categoryEn: 'Disk Space',
    template: `SELECT table_schema AS 'Database', SUM(data_length + index_length) / 1024 / 1024 / 1024 AS 'Size (GB)' FROM information_schema.tables WHERE table_schema not in ('mysql','INFORMATION_SCHEMA','METRICS_SCHEMA', 'PERFORMANCE_SCHEMA', 'sys') GROUP BY table_schema;`
  },
  {
    key: 'estimated-by-region',
    labelZh: '按 Region 估算',
    labelEn: 'Estimated by Region',
    category: 'disk',
    categoryZh: '磁盘空间',
    categoryEn: 'Disk Space',
    template: `SELECT db_name, table_name, ROUND(SUM(total_size / cnt), 2) AS Approximate_Size,\nROUND(SUM(total_size / cnt / (\nSELECT ROUND(AVG(value), 2)\nFROM METRICS_SCHEMA.store_size_amplification\nWHERE value > 0\n)), 2) AS Disk_Size\nFROM (\nSELECT db_name, table_name, region_id, SUM(Approximate_Size) total_size, COUNT(*) cnt\nFROM information_schema.TIKV_REGION_STATUS\nWHERE db_name = @dbname\nAND table_name IN (@table_name)\nGROUP BY db_name, table_name, region_id\n) tabinfo\nGROUP BY db_name, table_name;`
  }
]

const templateCategories = [
  { key: 'slow', label: '慢查询 / Slow Query' },
  { key: 'region', label: 'Region' },
  { key: 'tiflash', label: 'Tiflash' },
  { key: 'disk', label: '磁盘空间 / Disk Space' }
]

const templatesByCategory = computed(() => {
  const map: Record<string, any[]> = {}
  for (const cat of templateCategories) map[cat.key] = []
  for (const tpl of sqlTemplates) {
    if (map[tpl.category]) map[tpl.category].push(tpl)
  }
  return map
})

function insertTemplate(sql: string) {
  emit('insert-template', sql)
  showTemplateSidebar.value = false
}

const fileInput = ref<HTMLInputElement|null>(null)
function triggerFileInput() {
  fileInput.value?.click()
}
function handleFileChange(e: Event) {
  const files = (e.target as HTMLInputElement).files
  if (!files || !files[0]) return
  const reader = new FileReader()
  reader.onload = (evt) => {
    const sql = evt.target?.result as string
    if (sql) insertTemplate(sql)
  }
  reader.readAsText(files[0]);
  // 清空 input 以便连续导入
  (e.target as HTMLInputElement).value = ''
}

</script>

<style scoped>
.query-editor {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: white;
}
.editor-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #e0e6ed;
  background: #f8f9fa;
}
.toolbar-left, .toolbar-right {
  display: flex;
  align-items: center;
  gap: 8px;
}
.code-editor-container {
  flex: 1;
  display: flex;
  position: relative;
  background: #ffffff;
}
.editor-gutter {
  width: 50px;
  background: #f8f9fa;
  border-right: 1px solid #e0e6ed;
  overflow: hidden;
  user-select: none;
}
.line-number {
  height: 21px;
  line-height: 21px;
  text-align: right;
  padding-right: 8px;
  color: #6b7280;
  font-size: 12px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
}
.sql-textarea {
  flex: 1;
  border: none;
  outline: none;
  resize: none;
  padding: 16px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  line-height: 21px;
  background: #ffffff;
  color: #1a202c;
  white-space: pre;
  overflow-wrap: normal;
  overflow-x: auto;
}
.sql-textarea::placeholder {
  color: #9ca3af;
}
</style>
