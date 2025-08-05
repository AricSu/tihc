<template>
  <n-card class="query-editor-card" content-style="padding: 0; height: 100%; display: flex; flex-direction: column;" embedded style="height: 100%; display: flex; flex-direction: column;">
    <div class="editor-toolbar" style="display: flex; justify-content: space-between; align-items: center; padding: 10px 16px; border-bottom: 1px solid #e0e6ed; background: #fff;">
      <div class="toolbar-left" style="display: flex; align-items: center; gap: 8px;">
        <n-button-group>
          <n-button @click="handleExecuteQuery" type="primary" :loading="isExecuting">
            <template #icon><Icon icon="mdi:play" width="18" height="18" /></template>
            {{ t('sqlEditor.run') }} ({{ isMac ? '⌘' : 'Ctrl' }}+Enter)
          </n-button>
        </n-button-group>
        <n-divider vertical />
        <n-button-group>
          <n-button @click="handleFormatSql"><template #icon><Icon icon="mdi:pencil" width="18" height="18" /></template>{{ t('sqlEditor.format') }}</n-button>
          <n-button @click="$emit('clear-editor')"><template #icon><Icon icon="mdi:delete" width="18" height="18" /></template>{{ t('sqlEditor.clear') }}</n-button>
        </n-button-group>
        <n-divider vertical />
        <n-button-group>
          <n-button @click="$emit('toggle-slowlog-panel')">
            <template #icon>
              <Icon :icon="showSlowlogPanel ? 'mdi:chart-bar-off' : 'mdi:chart-bar'" :color="showSlowlogPanel ? '#b0b3b8' : '#1976d2'" width="20" height="20" />
            </template>
            {{ showSlowlogPanel ? t('sqlEditor.hideSlowlog') : t('sqlEditor.showSlowlog') }}
          </n-button>
        </n-button-group>
      </div>
      <div class="toolbar-right" style="flex: 1; display: flex; justify-content: flex-end; align-items: center;">
        <n-text depth="3" style="color: #888; font-size: 12px; margin-right: 8px;">
          {{ t('sqlEditor.lines') }}: {{ (modelValue || '').split('\n').length }} | {{ t('sqlEditor.length') }}: {{ (modelValue || '').length }}
        </n-text>
      </div>
    </div>
    <div style="flex: 1; display: flex; min-height: 0;">
      <MonacoEditor
        ref="monacoEditorRef"
        v-model:value="modelValue"
        language="sql"
        theme="vs"
        :options="editorOptions"
        style="height: 100%; width: 100%; flex: 1; border-radius: 0; border: none; box-shadow: none; background: #fff; min-height: 0;"
      />
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { Icon } from '@iconify/vue'
const { t } = useI18n()
import MonacoEditor from 'monaco-editor-vue3'
import { format as sqlFormat } from 'sql-formatter'

const props = defineProps<{
  sqlContent: string;
  isExecuting: boolean;
  isMac: boolean;
  showSlowlogPanel: boolean;
}>()
const emit = defineEmits<{
  (e: 'execute-query', sql: string): void;
  (e: 'update:sqlContent', value: string): void;
  (e: 'clear-editor'): void;
  (e: 'toggle-slowlog-panel'): void;
  (e: 'keydown', evt: KeyboardEvent, sql?: string): void;
}>()

const monacoEditorRef = ref()

// 修复 clear 功能，确保点击后清空内容并同步到父组件
function handleClearEditor() {
  modelValue.value = ''
  emit('update:sqlContent', '')
}

// 暴露 clear 方法和 monacoEditorRef 给父组件/模板
defineExpose({ handleClearEditor, monacoEditorRef })

const LOCAL_KEY = 'tihc_sql_editor_content'
const DEFAULT_SQL = 'select * from tihc.SLOW_QUERY limit 10;'
let initialContent = localStorage.getItem(LOCAL_KEY) || props.sqlContent || DEFAULT_SQL

const modelValue = ref(initialContent)

// 初始化时主动同步一次 SQL 内容到父组件
emit('update:sqlContent', modelValue.value)

// 响应外部 sqlContent 变化（如恢复、清空等）
watch(() => props.sqlContent, (val) => {
  if (val !== modelValue.value) {
    modelValue.value = val ?? ''
  }
})

watch(modelValue, (val) => {
  if (val !== props.sqlContent) {
    emit('update:sqlContent', val)
  }
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

// 新增：用于获取编辑器选中内容
function getSelectedOrAllSql() {
  const editor = monacoEditorRef.value?.editor
  if (editor) {
    const selection = editor.getSelection()
    const model = editor.getModel()
    if (selection && model) {
      const sql = model.getValueInRange(selection)
      // 如果有选区且内容不为空，返回选区内容，否则返回全部内容
      return sql && sql.trim() ? { sql, range: selection } : { sql: model.getValue(), range: null }
    }
  }
  // fallback: 全部内容
  return { sql: modelValue.value, range: null }
}

function handleExecuteQuery() {
  const editor = monacoEditorRef.value?.editor
  if (editor) {
    const selection = editor.getSelection()
    const model = editor.getModel()
    if (selection && model) {
      const sql = model.getValueInRange(selection)
      if (sql && sql.trim()) {
        emit('execute-query', sql)
        return
      }
    }
  }
  emit('execute-query', modelValue.value)
}

function handleFormatSql() {
  const editor = monacoEditorRef.value?.editor
  if (editor) {
    const { sql, range } = getSelectedOrAllSql()
    const formatted = sqlFormat(sql, { language: 'sql' })
    if (range) {
      // 格式化选区
      editor.executeEdits('format-sql', [{ range, text: formatted }])
    } else {
      // 格式化全部
      modelValue.value = formatted
      emit('update:sqlContent', formatted)
    }
  }
}
</script>



