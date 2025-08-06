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
              <Icon :icon="showSlowlogPanel ? 'mdi:file-import-off' : 'mdi:file-import'" :color="showSlowlogPanel ? '#b0b3b8' : '#1976d2'" width="20" height="20" />
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
        @editor-mounted="onEditorMounted"
        style="height: 100%; width: 100%; flex: 1; border-radius: 0; border: none; box-shadow: none; background: #fff; min-height: 0;"
      />
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onBeforeUnmount, nextTick } from 'vue'
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
}>()

const monacoEditorRef = ref()
let globalKeyHandler = null

// 本地存储键和默认SQL
const LOCAL_KEY = 'tihc_sql_editor_content'
const DEFAULT_SQL = 'select * from tihc.SLOW_QUERY limit 10;'

// 初始化内容
const modelValue = ref(
  localStorage.getItem(LOCAL_KEY) || props.sqlContent || DEFAULT_SQL
)

// 暴露方法给父组件
defineExpose({ 
  handleClearEditor: () => {
    modelValue.value = ''
    emit('update:sqlContent', '')
  }, 
  monacoEditorRef 
})

// 同步数据：初始化时发送内容到父组件
emit('update:sqlContent', modelValue.value)

// 监听外部内容变化
watch(() => props.sqlContent, (newVal) => {
  if (newVal !== modelValue.value) {
    modelValue.value = newVal ?? ''
  }
})

// 监听内部内容变化，同步到父组件和本地存储
watch(modelValue, (newVal) => {
  if (newVal !== props.sqlContent) {
    emit('update:sqlContent', newVal)
  }
  localStorage.setItem(LOCAL_KEY, newVal ?? '')
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

// 键盘快捷键设置：全局监听 Ctrl+Enter / Cmd+Enter
function setupKeyboardShortcuts() {
  if (globalKeyHandler) {
    document.removeEventListener('keydown', globalKeyHandler, true)
  }

  globalKeyHandler = (event) => {
    const isExecuteShortcut = (event.ctrlKey || event.metaKey) && event.key === 'Enter'
    
    if (isExecuteShortcut) {
      const activeElement = document.activeElement
      const editorContainer = monacoEditorRef.value?.$el || document.querySelector('.monaco-editor')
      
      // 确保焦点在编辑器内或不在其他输入框内
      const isInEditor = editorContainer?.contains(activeElement) || activeElement?.closest('.monaco-editor')
      const isNotInOtherInput = !['INPUT', 'TEXTAREA', 'SELECT'].includes(activeElement?.tagName)
      
      if (isInEditor || isNotInOtherInput) {
        event.preventDefault()
        event.stopPropagation()
        event.stopImmediatePropagation()
        handleExecuteQuery()
        return false
      }
    }
  }

  document.addEventListener('keydown', globalKeyHandler, true)
}

// 编辑器挂载后的初始化
function onEditorMounted(editor) {
  nextTick(() => {
    setupKeyboardShortcuts()
    
    // 尝试添加 Monaco 内置快捷键作为备用
    try {
      editor?.addAction?.({
        id: 'execute-sql-query',
        label: 'Execute SQL Query',
        keybindings: [2063], // CtrlCmd + Enter
        run: handleExecuteQuery
      })
    } catch (error) {
      console.warn('[QueryEditor] Monaco action setup failed:', error)
    }
  })
}

// 组件生命周期管理
onMounted(() => nextTick(setupKeyboardShortcuts))
onBeforeUnmount(() => {
  if (globalKeyHandler) {
    document.removeEventListener('keydown', globalKeyHandler, true)
  }
})

// 获取编辑器选中内容或全部内容
function getSelectedOrAllSql() {
  const editor = monacoEditorRef.value?.editor
  if (!editor) return { sql: modelValue.value, range: null }
  
  const selection = editor.getSelection()
  const model = editor.getModel()
  if (selection && model && !selection.isEmpty()) {
    const sql = model.getValueInRange(selection)
    if (sql?.trim()) {
      return { sql, range: selection }
    }
  }
  
  return { sql: model?.getValue() || modelValue.value, range: null }
}

// SQL 执行：优先执行选中内容，否则执行全部
function handleExecuteQuery() {
  const { sql } = getSelectedOrAllSql()
  const trimmedSql = sql?.trim()
  
  if (trimmedSql) {
    emit('execute-query', trimmedSql)
  } else {
    console.warn('[QueryEditor] No SQL content to execute')
  }
}

// SQL 格式化：支持选中内容或全部内容
function handleFormatSql() {
  const editor = monacoEditorRef.value?.editor
  if (!editor) return
  
  const { sql, range } = getSelectedOrAllSql()
  try {
    const formatted = sqlFormat(sql, { language: 'sql' })
    
    if (range) {
      // 格式化选中内容
      editor.executeEdits('format-sql', [{ range, text: formatted }])
    } else {
      // 格式化全部内容
      modelValue.value = formatted
      emit('update:sqlContent', formatted)
    }
  } catch (error) {
    console.warn('[QueryEditor] SQL format failed:', error)
  }
}
</script>



