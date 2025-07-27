<template>
  <n-list bordered>
    <n-list-item v-for="(item, idx) in history" :key="idx">
      <div class="history-item">
        <div class="sql">{{ item.sql }}</div>
        <div class="meta">
          <span>{{ item.time }}</span>
          <n-button size="tiny" @click="$emit('restore', item)">恢复</n-button>
        </div>
      </div>
    </n-list-item>
    <template v-if="!history.length">
      <n-empty description="暂无历史记录" />
    </template>
  </n-list>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useSqlEditorStore } from '@/store/modules/sqlEditor'
const sqlEditor = useSqlEditorStore()
const history = computed(() => sqlEditor.history)
</script>
<style scoped>
.history-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.sql {
  font-family: monospace;
  color: #333;
  max-width: 400px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.meta {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #888;
  font-size: 12px;
}
</style>
