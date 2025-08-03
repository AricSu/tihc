<script setup lang="ts">
import { ref, onMounted, watch, h, resolveComponent } from 'vue'
import { NIcon, NTree, NPopover } from 'naive-ui'
import { BookOutline, RefreshOutline, ListOutline } from '@vicons/ionicons5'
import SqlTemplateSidebar from './SqlTemplateSidebar.vue'
import { fetchDatabaseList } from '../../../api/database'
import { fetchTableList } from '../../../api/table'
import { useSqlEditorStore } from '@/store/modules/sqlEditor'

const props = defineProps({ showSidebar: Boolean })
const emit = defineEmits(['update:showSidebar'])

const treeData = ref([])
const loadingSchema = ref(false)
const schemaError = ref('')
const sqlEditor = useSqlEditorStore()

const renderLabel = ({ option: node }) => {
  try {
    console.log('[Tree] renderLabel: node', node)
    if (!node?.key) {
      console.warn('[Tree] renderLabel: node.key missing', node)
      return ''
    }
    // 严格区分 schema 和 table 节点
    if (/^db-[^\-]+$/.test(node.key)) {
      // schema节点 Popover 展示
      console.log('[Tree] renderLabel: schema node', node)
      const schemaContent = h('div', {
        style: 'display: flex; flex-direction: column; gap: 8px; min-width: 320px;'
      }, [
        h('div', [h('b', 'Schema 名称: '), node.label]),
        node.default_character_set_name && h('div', [h('b', '字符集: '), node.default_character_set_name]),
        node.default_collation_name && h('div', [h('b', '排序规则: '), node.default_collation_name])
      ].filter(Boolean))
      try {
        console.log('[Tree] renderLabel: rendering schema popover', node.label)
        return h('span', [
          h('span', { style: 'color: #333;' }, node.label),
          h(NPopover, {
            trigger: 'hover',
            placement: 'right-start',
            style: 'min-width: 320px;'
          }, {
            default: () => schemaContent,
            trigger: () => h('span', {
              style: 'display: inline-flex; alignItems: center; marginLeft: 6px; cursor: "pointer";'
            }, [
              h('svg', {
                width: '16', height: '16', viewBox: '0 0 16 16', fill: 'none', xmlns: 'http://www.w3.org/2000/svg',
                style: 'color: #409eff; verticalAlign: middle;'
              }, [
                h('circle', { cx: '8', cy: '8', r: '8', fill: '#409eff', opacity: '0.15' }),
                h('text', { x: '8', y: '12', textAnchor: 'middle', fontSize: '12', fill: '#409eff' }, 'i')
              ])
            ])
          })
        ])
      } catch (err) {
        console.error('[Tree] renderLabel: schema popover render error', err, node)
        return node.label
      }
    } else if (/^db-[^\-]+-table-/.test(node.key)) {
      // 表节点 Popover 展示
      console.log('[Tree] renderLabel: table node', node)
      const tableContent = h('div', {
        style: 'display: flex; flex-direction: column; gap: 8px; min-width: 320px;'
      }, [
        h('div', [h('b', '表名: '), node.label]),
        node.table_comment && h('div', [h('b', '注释: '), node.table_comment]),
        node.create_time && h('div', [h('b', '创建时间: '), node.create_time]),
        node.table_schema && h('div', [h('b', 'Schema: '), node.table_schema])
      ].filter(Boolean))
      try {
        console.log('[Tree] renderLabel: rendering table popover', node.label)
        return h('span', [
          h('span', { style: 'color: #333;' }, node.label),
          h(NPopover, {
            trigger: 'hover',
            placement: 'right-start',
            style: 'min-width: 320px;'
          }, {
            default: () => tableContent,
            trigger: () => h('span', {
              style: 'display: inline-flex; alignItems: center; marginLeft: 6px; cursor: "pointer";'
            }, [
              h('svg', {
                width: '16', height: '16', viewBox: '0 0 16 16', fill: 'none', xmlns: 'http://www.w3.org/2000/svg',
                style: 'color: #409eff; verticalAlign: middle;'
              }, [
                h('circle', { cx: '8', cy: '8', r: '8', fill: '#409eff', opacity: '0.15' }),
                h('text', { x: '8', y: '12', textAnchor: 'middle', fontSize: '12', fill: '#409eff' }, 'i')
              ])
            ])
          })
        ])
      } catch (err) {
        console.error('[Tree] renderLabel: table popover render error', err, node)
        return node.label
      }
    } else {
      // 其他节点类型直接显示 label
      return node.label
    }
  } catch (err) {
    console.error('[Tree] renderLabel: unexpected error', err, node)
    return node?.label || ''
  }
}

const fetchSchema = async () => {
  loadingSchema.value = true
  schemaError.value = ''
  const connectionId = sqlEditor.currentConnection?.id
  if (!connectionId) {
    schemaError.value = '请先选择连接'
    treeData.value = []
    loadingSchema.value = false
    return
  }
  try {
    const dbList = await fetchDatabaseList(connectionId)
    treeData.value = dbList.map(db => ({
      key: 'db-' + db.schema_name,
      label: db.schema_name,
      isLeaf: false,
      children: undefined,
      default_character_set_name: db.default_character_set_name,
      default_collation_name: db.default_collation_name
    }))
  } catch (err) {
    schemaError.value = err?.response?.data?.message || err?.message || 'Schema load failed'
    treeData.value = []
  }
  loadingSchema.value = false
}

onMounted(fetchSchema)
watch(() => sqlEditor.currentConnection?.id, (newId, oldId) => {
  if (newId && newId !== oldId) fetchSchema()
})
const handleRefresh = fetchSchema

const handleLoad = async (node) => {
  if (node.key?.startsWith('db-') && (!node.children || node.children.length === 0)) {
    const connectionId = sqlEditor.currentConnection?.id
    if (!connectionId) return
    try {
      const tableList = await fetchTableList(connectionId, node.label)
      node.children = tableList.map(tbl => ({
        key: node.key + '-table-' + tbl.table_name,
        label: tbl.table_name,
        isLeaf: true,
        table_schema: tbl.table_schema,
        create_time: tbl.create_time ? new Date(tbl.create_time).toLocaleString() : '',
        table_comment: tbl.table_comment
      }))
      treeData.value = [...treeData.value]
    } catch (err) {
      // 可选：错误处理
    }
  }
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
      <n-card size="small" :bordered="false">
        <n-space align="center" justify="space-between">
          <n-text strong>
            <n-icon size="18"><ListOutline /></n-icon>
            数据库列表
          </n-text>
          <n-button @click="handleRefresh" size="tiny" text circle>
            <template #icon>
              <n-icon><RefreshOutline /></n-icon>
            </template>
          </n-button>
        </n-space>
        <n-divider style="margin: 8px 0;" />
        <n-space vertical size="small" style="margin-top: 8px;">
          <template v-if="loadingSchema">
            <n-space align="center">
              <n-spin size="small" />
              <n-text depth="3">Loading...</n-text>
            </n-space>
          </template>
          <template v-else-if="schemaError">
            <n-alert type="error" :show-icon="true">{{ schemaError }}</n-alert>
          </template>
          <template v-else-if="treeData.length > 0">
            <div>
              <NTree
                :data="treeData"
                block-line
                :show-irrelevant-nodes="false"
                :default-expand-all="false"
                :expand-on-click="true"
                :on-load="handleLoad"
                :loading="loadingSchema"
                :render-label="renderLabel"
              />
            </div>
          </template>
          <template v-else>
            <n-empty description="暂无数据库信息" size="small" />
          </template>
        </n-space>
      </n-card>
      <n-card size="small" :bordered="false">
        <n-text strong>
          <n-icon size="18"><ListOutline /></n-icon>
          SQL Templates
        </n-text>
        <n-divider style="margin: 8px 0;" />
        <SqlTemplateSidebar :showSidebar="props.showSidebar" />
      </n-card>
    </n-space>
  </n-layout-sider>
</template>