<template>
  <n-form :model="settings" label-width="80">
    <n-form-item label="字体大小">
      <n-input-number v-model:value="settings.fontSize" :min="10" :max="32" />
    </n-form-item>
    <n-form-item label="Tab 宽度">
      <n-input-number v-model:value="settings.tabSize" :min="2" :max="8" />
    </n-form-item>
    <n-form-item label="自动换行">
      <n-switch v-model:value="settings.wordWrap" />
    </n-form-item>
    <n-form-item label="自动保存">
      <n-switch v-model:value="settings.autoSave" />
    </n-form-item>
    <n-form-item label="超时时间">
      <n-input-number v-model:value="settings.queryTimeout" :min="5" :max="120" />
    </n-form-item>
    <n-form-item>
      <n-space>
        <n-button type="primary" @click="saveSettings">保存</n-button>
        <n-button @click="resetSettings">重置为默认</n-button>
      </n-space>
    </n-form-item>
  </n-form>
</template>

<script setup lang="ts">
import { useSqlEditorStore } from '@/store/modules/sqlEditor'
import { useMessage } from 'naive-ui'

const DEFAULT_SETTINGS = {
  fontSize: 14,
  tabSize: 2,
  wordWrap: true,
  autoSave: false,
  queryTimeout: 30
}

const sqlEditor = useSqlEditorStore()
const settings = sqlEditor.settings
const message = useMessage()

function saveSettings() {
  message.success('设置已保存')
}

function resetSettings() {
  Object.assign(settings, DEFAULT_SETTINGS)
  message.info('已恢复默认设置')
}
</script>
