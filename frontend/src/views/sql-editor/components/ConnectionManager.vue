<template>
  <n-list bordered>
    <n-list-item v-for="(conn, idx) in connections" :key="conn.id">
      <div class="conn-item">
        <div>
          <b>{{ conn.name }}</b>
          <span class="meta">({{ conn.host }}:{{ conn.port }})</span>
        </div>
        <n-button size="tiny" @click="$emit('switch', conn)">切换</n-button>
      </div>
    </n-list-item>
    <template v-if="!connections.length">
      <n-empty description="暂无连接" />
    </template>
    <template v-if="connections.length">
      <n-list-item>
        <n-button block type="info" @click="$emit('open-slowlog')">打开慢日志工具</n-button>
      </n-list-item>
    </template>
  </n-list>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useSqlEditorStore } from '@/store/modules/sqlEditor'
const sqlEditor = useSqlEditorStore()
const connections = computed(() => sqlEditor.connections)
const emit = defineEmits(['switch', 'open-slowlog'])
</script>

<style scoped>
.conn-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.meta {
  color: #888;
  font-size: 12px;
  margin-left: 8px;
}
</style>
