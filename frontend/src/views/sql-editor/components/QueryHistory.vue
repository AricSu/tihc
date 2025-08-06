<template>
  <n-list bordered class="history-list">
    <n-list-item v-for="(item, idx) in historySafe" :key="idx" class="history-list-item">
      <div class="history-item">
        <div class="sql-block">
          <n-ellipsis :tooltip="false" style="max-width: 420px;">
            <pre class="sql-pre">{{ item.sql }}</pre>
          </n-ellipsis>
        </div>
        <div class="meta-block">
          <span class="history-time">{{ item.time }}</span>
          <n-button size="tiny" quaternary type="primary" @click="$emit('restore', item)">
            <template #icon>
              <Icon icon="mdi:restore" width="18" height="18" />
            </template>
            <span class="restore-text">{{ t('sqlEditor.restore') }}</span>
          </n-button>
        </div>
      </div>
    </n-list-item>
    <template v-if="!historySafe.length">
      <n-empty :description="t('sqlEditor.noHistory')" />
    </template>
  </n-list>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useSqlEditorStore } from '../../../store/modules/sqlEditor'
import { Icon } from '@iconify/vue'
import { useI18n } from 'vue-i18n'
const { t } = useI18n()
const sqlEditor = useSqlEditorStore()
const historySafe = computed(() => Array.isArray(sqlEditor.history) ? sqlEditor.history : [])
</script>
<style scoped>
/* 必要布局和简洁样式 */
.history-list {
  max-height: 400px;
  overflow-y: auto;
}
.history-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 8px 0;
}
.sql-block {
  flex: 1;
  margin-right: 12px;
}
.sql-pre {
  font-family: monospace;
  font-size: 13px;
  margin: 0;
  white-space: pre-wrap;
  word-break: break-all;
}
.meta-block {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  min-width: 80px;
}
.history-time {
  color: #888;
  font-size: 12px;
}
.restore-text {
  margin-left: 2px;
  font-size: 13px;
  color: #1677ff;
}
</style>
