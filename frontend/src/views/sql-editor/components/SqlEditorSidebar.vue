
<script setup lang="ts">
import { h, ref, onMounted } from 'vue'
import { NIcon } from 'naive-ui'
import { KeyOutline, ListOutline, RefreshOutline, BookOutline } from '@vicons/ionicons5'
import SqlTemplateSidebar from './SqlTemplateSidebar.vue'
import { fetchDatabaseSchema } from '../../../api/database'

const nodeProps = ({ option }) => {
  return {
    icon: option.icon,
    suffix: option.suffix ? h('span', { style: 'color: #888; marginLeft: ' + '8px' }, option.suffix) : undefined,
    label: option.isPrimary ? h('span', [option.label, h(NIcon, { style: 'color: #f90; marginLeft: ' + '4px' }, { default: () => h(KeyOutline) })]) : option.label
  }
}

const props = defineProps({
  showSidebar: Boolean
})
const emit = defineEmits(['update:showSidebar'])
const showDetailModal = ref(false)
const tableColumns = [
  { title: '字段名', key: 'column_name', width: 200 },
  { title: '数据类型', key: 'data_type', width: 120 },
  { title: '备注', key: 'comment', width: 400 }
]

const slowQuerySchema = ref<Array<{ column_name: string; data_type: string; comment?: string }>>([])
const schemaTree = ref([])
const loadingSchema = ref(false)
const schemaError = ref('')

const fetchSchema = async () => {
  loadingSchema.value = true
  schemaError.value = ''
  try {
    const schema = await fetchDatabaseSchema()
    slowQuerySchema.value = schema
    schemaTree.value = buildSchemaTree(schema)
  } catch (err: any) {
    schemaError.value = err?.message || 'Schema load failed'
    slowQuerySchema.value = []
    schemaTree.value = []
  } finally {
    loadingSchema.value = false
  }
}

function buildSchemaTree(schema) {
  // 仅支持单表，后续可扩展多表
  return [
    {
      label: '表',
      key: 'tables',
      children: [
        {
          label: 'cluster_slow_query',
          key: 'table-cluster_slow_query',
          icon: () => h(NIcon, null, { default: () => h(ListOutline) }),
          children: [
            {
              label: '列',
              key: 'columns',
              children: schema.map(col => ({
                label: col.column_name,
                key: 'col-' + col.column_name,
                icon: () => h(NIcon, null, { default: () => h(KeyOutline) }),
                suffix: col.data_type,
                isPrimary: col.column_name === 'id' // 可根据实际主键字段调整
              }))
            },
            {
              label: '索引',
              key: 'indexes',
              children: [
                {
                  label: 'PRIMARY',
                  key: 'idx-primary',
                  icon: () => h(NIcon, null, { default: () => h(KeyOutline) })
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}

onMounted(() => {
  fetchSchema()
})

const handleRefresh = () => {
  fetchSchema()
}
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
          <n-button @click="handleRefresh" size="tiny" text circle>
            <template #icon>
              <n-icon><RefreshOutline /></n-icon>
            </template>
          </n-button>
        </n-space>
        <n-divider style="margin: 8px 0;" />
        <n-space align="center" justify="space-between">
          <n-button 
            v-if="slowQuerySchema.length > 0" 
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
          <template v-if="loadingSchema">
            <n-space align="center">
              <n-spin size="small" />
              <n-text depth="3">Loading schema...</n-text>
            </n-space>
          </template>
          <template v-else-if="schemaError">
            <n-alert type="error" :show-icon="true">{{ schemaError }}</n-alert>
          </template>
          <template v-else-if="schemaTree.length > 0">
            <n-tree
              :data="schemaTree"
              block-line
              :node-props="nodeProps"
            />
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
              :data="slowQuerySchema"
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


