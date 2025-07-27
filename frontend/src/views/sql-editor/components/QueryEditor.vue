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
          <n-dropdown :options="sqlTemplates" @select="$emit('insert-template', $event)">
            <n-button>Templates</n-button>
          </n-dropdown>
          <n-button @click="$emit('format-sql')"><template #icon><n-icon>📝</n-icon></template>Format</n-button>
          <n-button @click="$emit('clear-editor')"><template #icon><n-icon>🗑️</n-icon></template>Clear</n-button>
          <n-button @click="$emit('save-query')"><template #icon><n-icon>💾</n-icon></template>Save</n-button>
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
import { ref } from 'vue'
const props = defineProps<{
  sqlContent: string;
  isExecuting: boolean;
  connectionStatus: string;
  isMac: boolean;
  lineCount: number;
  showSlowlogPanel: boolean;
  sqlTemplates: any[];
}>()
const emit = defineEmits<{
  (e: 'execute-query'): void;
  (e: 'insert-template', key: string): void;
  (e: 'format-sql'): void;
  (e: 'clear-editor'): void;
  (e: 'save-query'): void;
  (e: 'toggle-slowlog-panel'): void;
  (e: 'keydown', evt: KeyboardEvent): void;
  (e: 'input'): void;
  (e: 'scroll', evt: Event): void;
  (e: 'update:sqlContent', value: string): void;
}>()
const sqlTextarea = ref<HTMLTextAreaElement | null>(null)
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
