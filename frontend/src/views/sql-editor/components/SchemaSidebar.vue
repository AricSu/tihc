<template>
  <n-layout-sider 
    bordered 
    :width="380" 
    collapse-mode="width" 
    :collapsed-width="0"
    :collapsed="!showSidebar"
    show-trigger="arrow-circle"
    @collapse="$emit('update:showSidebar', false)"
    @expand="$emit('update:showSidebar', true)"
  >
    <div class="schema-browser">
      <div class="browser-header">
        <h3>Schema Browser</h3>
        <n-button @click="$emit('refresh-schema')" size="tiny" text>
          <template #icon><n-icon>🔄</n-icon></template>
        </n-button>
      </div>
      <div class="schema-tree">
        <div v-if="slowQuerySchema.length > 0" class="schema-content">
          <h4>cluster_slow_query columns:</h4>
          <div v-for="column in slowQuerySchema" :key="column.column_name" class="column-item">
            <span class="column-name">{{ column.column_name }}</span>
            <span class="column-type">{{ column.data_type }}</span>
            <span v-if="column.comment" class="column-comment">{{ column.comment }}</span>
          </div>
        </div>
        <n-empty v-else description="Click 'Get Schema' to load table structure" size="small" />
      </div>
    </div>
  </n-layout-sider>
</template>

<script setup lang="ts">
import type { PropType } from 'vue'
interface SlowQueryColumn {
  column_name: string
  data_type: string
  comment?: string
}
const props = defineProps({
  showSidebar: Boolean,
  slowQuerySchema: {
    type: Array as PropType<SlowQueryColumn[]>,
    default: () => []
  }
})
</script>

<style scoped>
.schema-browser {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: white;
  border-right: 1px solid #e0e6ed;
}
.browser-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border-bottom: 1px solid #e0e6ed;
}
.schema-tree {
  flex: 1;
  padding: 8px;
  overflow-y: auto;
}
</style>
