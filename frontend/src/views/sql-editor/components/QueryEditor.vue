
<template>
  <n-card class="query-editor-card" content-style="padding: 0; height: 100%; display: flex; flex-direction: column;" embedded style="height: 100%; display: flex; flex-direction: column;">
    <div class="editor-toolbar" style="display: flex; justify-content: space-between; align-items: center; padding: 10px 16px; border-bottom: 1px solid #e0e6ed; background: #fff;">
      <div class="toolbar-left" style="display: flex; align-items: center; gap: 8px;">
        <n-button-group>
          <n-button @click="$emit('execute-query')" type="primary" :loading="isExecuting" :disabled="!sqlContent.trim() || connectionStatus !== 'connected'">
            <template #icon><n-icon>▶️</n-icon></template>
            Run ({{ isMac ? '⌘' : 'Ctrl' }}+Enter)
          </n-button>
        </n-button-group>
        <n-divider vertical />
        <n-button-group>
          <n-button @click="handleFormatSql"><template #icon><n-icon>📝</n-icon></template>Format</n-button>
          <n-button @click="$emit('clear-editor')"><template #icon><n-icon>🗑️</n-icon></template>Clear</n-button>
        </n-button-group>
        <n-divider vertical />
        <n-button-group>
          <n-button @click="$emit('toggle-slowlog-panel')">{{ showSlowlogPanel ? 'Hide' : 'Show' }} Slowlog Tools</n-button>
        </n-button-group>
      </div>
      <div class="toolbar-right" style="flex: 1; display: flex; justify-content: flex-end; align-items: center;">
        <n-text depth="3" style="color: #888; font-size: 12px; margin-right: 8px;">
          Lines: {{ (modelValue || '').split('\n').length }} | Length: {{ (modelValue || '').length }}
        </n-text>
      </div>
    </div>
    <div style="flex: 1; display: flex; min-height: 0;">
      <MonacoEditor
        v-model:value="modelValue"
        language="sql"
        theme="vs"
        :options="editorOptions"
        style="height: 100%; width: 100%; flex: 1; border-radius: 0; border: none; box-shadow: none; background: #fff; min-height: 0;"
        @keydown="$emit('keydown', $event)"
      />
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { format as sqlFormat } from 'sql-formatter'
import MonacoEditor from 'monaco-editor-vue3'

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



const LOCAL_KEY = 'tihc_sql_editor_content'
const DEFAULT_SQL = 'select * from tihc.slow_quey limit 10;'
let initialContent = localStorage.getItem(LOCAL_KEY)
if (initialContent === null || initialContent === '') {
  initialContent = props.sqlContent && props.sqlContent.trim() ? props.sqlContent : DEFAULT_SQL
}
const modelValue = ref(initialContent)

watch(() => props.sqlContent, (val) => {
  if (val !== modelValue.value) modelValue.value = val
})

watch(modelValue, (val) => {
  emit('update:sqlContent', val)
  emit('input')
  localStorage.setItem(LOCAL_KEY, val ?? '')
})

const editorOptions = {
  fontSize: 14,
  lineNumbers: 'on',
  minimap: { enabled: false },
  scrollBeyondLastLine: false,
  wordWrap: 'on',
  automaticLayout: true,
  tabSize: 4,
  theme: 'vs',
}

function handleFormatSql() {
  if (modelValue.value && modelValue.value.trim()) {
    const formatted = sqlFormat(modelValue.value, { language: 'sql' })
    modelValue.value = formatted
    emit('update:sqlContent', formatted)
  }
}
</script>



