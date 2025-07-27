<template>
  <n-form :model="settings" label-width="80">
    <SettingsItem label="字体大小">
      <n-input-number v-model:value="settings.fontSize" :min="10" :max="32" />
    </SettingsItem>
    <SettingsItem label="Tab 宽度">
      <n-input-number v-model:value="settings.tabSize" :min="2" :max="8" />
    </SettingsItem>
    <SettingsItem label="自动换行">
      <n-switch v-model:value="settings.wordWrap" />
    </SettingsItem>
    <SettingsItem label="自动保存">
      <n-switch v-model:value="settings.autoSave" />
    </SettingsItem>
    <SettingsItem label="超时时间">
      <n-input-number v-model:value="settings.queryTimeout" :min="5" :max="120" />
    </SettingsItem>
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
import SettingsItem from './SettingsItem.vue'

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
