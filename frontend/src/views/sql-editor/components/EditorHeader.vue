<template>
  <div class="editor-header">
    <div class="header-left">
      <h2>SQL Editor</h2>
      <n-divider vertical />
      <n-select
        v-model:value="selectedConnectionProxy"
        :options="connectionOptions"
        placeholder="Select Connection"
        style="width: 200px;"
        @update:value="$emit('switch-connection', $event)"
      />
      <n-button @click="$emit('open-new-connection-modal')" size="small" type="primary" secondary>
        <template #icon>
          <n-icon>🗄️</n-icon>
        </template>
        New Connection
      </n-button>
    </div>
    <div class="header-right">
      <n-tag v-if="connectionStatus === 'connected'" type="success">
        <template #icon><n-icon>✅</n-icon></template>
        Connected
      </n-tag>
      <n-tag v-else-if="connectionStatus === 'connecting'" type="warning">
        <template #icon><n-icon>⏳</n-icon></template>
        Connecting...
      </n-tag>
      <n-tag v-else type="error">
        <template #icon><n-icon>❌</n-icon></template>
        Disconnected
      </n-tag>
      <n-button-group>
        <n-button @click="$emit('open-connection-management-modal')" size="small">
          <template #icon><n-icon>🗄️</n-icon></template>
          Connections
        </n-button>
        <n-button @click="$emit('show-query-history')" size="small">
          <template #icon><n-icon>📜</n-icon></template>
          History
        </n-button>
        <n-button @click="$emit('show-settings')" size="small">
          <template #icon><n-icon>⚙️</n-icon></template>
          Settings
        </n-button>
      </n-button-group>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
const props = defineProps({
  selectedConnection: {
    type: [String, Number, Object],
    default: ''
  },
  connectionOptions: {
    type: Array,
    default: () => []
  },
  connectionStatus: {
    type: String,
    default: 'disconnected'
  }
})
const emit = defineEmits([
  'update:selectedConnection',
  'switch-connection',
  'open-new-connection-modal',
  'open-connection-management-modal',
  'show-query-history',
  'show-settings'
])

const selectedConnectionProxy = computed({
  get: () => props.selectedConnection,
  set: (val) => emit('update:selectedConnection', val)
})
</script>

<style scoped>
.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px;
  background: white;
  border-bottom: 1px solid #e0e6ed;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}
.header-left, .header-right {
  display: flex;
  align-items: center;
  gap: 12px;
}
</style>
