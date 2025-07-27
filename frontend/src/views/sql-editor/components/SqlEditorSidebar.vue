
<script setup lang="ts">
import { ref, PropType, computed } from 'vue'
import SqlTemplateSidebar from './SqlTemplateSidebar.vue'
import { RefreshOutline, BookOutline, ListOutline } from '@vicons/ionicons5'

const props = defineProps({
  showSidebar: Boolean,
  slowQuerySchema: {
    type: Array as PropType<Array<{ column_name: string; data_type: string; comment?: string }>>,
    default: () => []
  },
  loadingSchema: {
    type: Boolean,
    default: false
  }
})
const emit = defineEmits(['update:showSidebar', 'refresh-schema'])
const showDetailModal = ref(false)
const tableColumns = [
  { title: '字段名', key: 'column_name', width: 200 },
  { title: '数据类型', key: 'data_type', width: 120 },
  { title: '备注', key: 'comment', width: 400 }
]
</script>

<template>
  <n-layout-sider
    bordered
    :width="340"
    collapse-mode="width"
    :collapsed-width="0"
    :collapsed="!props.showSidebar"
    show-trigger="arrow-circle"
    @collapse="emit('update:showSidebar', false)"
    @expand="emit('update:showSidebar', true)"
  >
    <n-space vertical size="large" style="height: 100%;">
      <!-- Schema Section -->
      <n-card size="small" :bordered="false">
        <n-space align="center" justify="space-between">
          <n-text strong>
            <n-icon size="18"><ListOutline /></n-icon>
            TiHC Schema
          </n-text>
          <n-button @click="emit('refresh-schema')" size="tiny" text circle>
            <template #icon>
              <n-icon><RefreshOutline /></n-icon>
            </template>
          </n-button>
        </n-space>
        <n-divider style="margin: 8px 0;" />
        <n-space align="center" justify="space-between">
          <n-button 
            v-if="props.slowQuerySchema.length > 0" 
            size="small" 
            type="primary" 
            @click="showDetailModal = true"
          >
            <template #icon>
              <n-icon><BookOutline /></n-icon>
            </template>
            详细说明
          </n-button>
        </n-space>
        <n-space vertical size="small" style="margin-top: 8px;">
          <template v-if="props.loadingSchema">
            <n-space align="center">
              <n-spin size="small" />
              <n-text depth="3">Loading schema...</n-text>
            </n-space>
          </template>
          <template v-else-if="props.slowQuerySchema.length > 0">
            <n-grid :cols="2" x-gap="8" y-gap="4">
              <n-gi v-for="column in props.slowQuerySchema" :key="column.column_name">
                <n-tag size="small" type="info">{{ column.column_name }}({{ column.data_type }})</n-tag>
              </n-gi>
            </n-grid>
            <n-space align="center" justify="space-between">
              <n-text depth="3">共 {{ props.slowQuerySchema.length }} 个字段</n-text>
              <n-button text type="primary" size="small" @click="showDetailModal = true">
                <template #icon>
                  <n-icon><BookOutline /></n-icon>
                </template>
                查看详细说明
              </n-button>
            </n-space>
          </template>
          <n-empty v-else description="Connect to database to load table structure" size="small" />
        </n-space>
        <!-- Schema Detail Modal -->
        <n-modal 
          v-model:show="showDetailModal" 
          preset="card" 
          title="cluster_slow_query 表结构详细说明"
          style="max-width: 1200px; max-height: 80vh;"
        >
          <n-space vertical size="large">
            <n-text>
              TiDB 慢查询表包含了数据库中执行时间较长的 SQL 语句信息，用于性能分析和优化。
            </n-text>
            <n-button 
              tag="a" 
              href="https://docs.pingcap.com/zh/tidb/stable/identify-slow-queries/#%E6%85%A2%E6%9F%A5%E8%AF%A2%E6%97%A5%E5%BF%97" 
              target="_blank" 
              type="primary" 
              size="small"
            >
              <template #icon>
                <n-icon><BookOutline /></n-icon>
              </template>
              查看 TiDB 官方文档
            </n-button>
            <n-data-table
              :columns="tableColumns"
              :data="props.slowQuerySchema"
              :pagination="{ pageSize: 15 }"
              :max-height="400"
              :scroll-x="800"
              size="small"
              striped
            />
          </n-space>
        </n-modal>
      </n-card>
      <!-- SQL Template Section -->
      <n-card size="small" :bordered="false">
        <n-text strong>
          <n-icon size="18"><ListOutline /></n-icon>
          SQL Templates
        </n-text>
        <n-divider style="margin: 8px 0;" />
        <SqlTemplateSidebar :showSidebar="props.showSidebar" />
      </n-card>
      <!-- 未来可扩展更多工具 -->
    </n-space>
  </n-layout-sider>
</template>


