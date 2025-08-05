<script setup lang="ts">
import { ref, onMounted, watch, h, resolveComponent } from 'vue'
import { NIcon, NTree, NPopover } from 'naive-ui'
import { RefreshOutline, ListOutline, ServerOutline, GridOutline, FolderOutline, DocumentTextOutline, KeyOutline } from '@vicons/ionicons5'
import SqlTemplateSidebar from './SqlTemplateSidebar.vue'
import { DatabaseAPI, TableAPI } from '@/api/sql-editor'
import { useSqlEditorStore } from '@/store/modules/sqlEditor'

const props = defineProps({ showSidebar: Boolean })
const emit = defineEmits(['update:showSidebar', 'insert-template'])

const treeData = ref([])
const loadingSchema = ref(false)
const schemaError = ref('')
const sqlEditor = useSqlEditorStore()

const getColumnIcon = (col) => {
  // 主键列用KeyOutline，普通列用DocumentTextOutline
  if (col.column_key === 'PRI') {
    return h(NIcon, { size: 18, color: '#e6a23c', style: { marginRight: '4px' } }, () => h(KeyOutline))
  }
  return h(NIcon, { size: 18, color: '#409eff', style: { marginRight: '4px' } }, () => h(DocumentTextOutline))
}
const getIndexIcon = () => h('span', { style: 'color: #f56c6c; marginRight: 4px;' }, [
  h(NIcon, { size: 18, color: '#f56c6c', style: { marginRight: '4px' } }, () => h(KeyOutline))
])
const getTableIcon = () => h('span', { style: 'color: #67c23a; marginRight: 4px;' }, [
  h(NIcon, { size: 18, color: '#67c23a', style: { marginRight: '4px' } }, () => h(GridOutline))
])
const getFolderIcon = () => h('span', { style: 'color: #909399; marginRight: 4px;' }, [
  h(NIcon, { size: 18, color: '#909399', style: { marginRight: '4px' } }, () => h(FolderOutline))
])

const renderLabel = ({ option: node }) => {
  try {
    // 分组节点（列/索引文件夹）
    if (/columns$/.test(node.key) || /indexes$/.test(node.key)) {
      return h('span', [
        getFolderIcon(),
        h('span', { style: 'font-weight: 600; color: #606266; marginLeft: ' + '4px' }, node.label)
      ])
    }
    // 表节点
    if (/^db-[^\-]+-table-[^\-]+$/.test(node.key)) {
      return h('span', [
        h(NPopover, {
          trigger: 'hover',
          placement: 'right-start',
          style: 'min-width: 320px;'
        }, {
          default: () => h('div', {
            style: 'display: flex; flex-direction: column; gap: 8px; min-width: 320px;'
          }, [
            h('div', [h('b', '表名: '), node.label]),
            node.table_schema && h('div', [h('b', 'Schema: '), node.table_schema]),
            node.create_time && h('div', [h('b', '创建时间: '), node.create_time]),
            node.table_comment && h('div', [h('b', '注释: '), node.table_comment])
          ].filter(Boolean)),
          trigger: () => getTableIcon()
        }),
        h('span', { style: 'font-weight: 500; color: #333; marginLeft: ' + '4px' }, node.label)
      ])
    }
    // 列节点
    if (/^db-[^\-]+-table-[^\-]+-column-/.test(node.key)) {
      // 只展示类型，其他信息全部悬浮显示
      const infoArr = []
      if (node.column_type) infoArr.push(node.column_type)
      return h(NPopover, {
        trigger: 'hover',
        placement: 'right-start',
        style: 'min-width: 320px;'
      }, {
        default: () => h('div', {
          style: 'display: flex; flex-direction: column; gap: 8px; min-width: 320px;'
        }, [
          h('div', [h('b', '字段名: '), node.label]),
          node.data_type && h('div', [h('b', '类型: '), node.data_type]),
          node.column_type && h('div', [h('b', '类型定义: '), node.column_type]),
          node.column_key && h('div', [h('b', '主键: '), node.column_key]),
          node.column_default !== undefined && h('div', [h('b', '默认值: '), node.column_default ?? '']),
          node.is_nullable !== undefined && h('div', [h('b', '可空: '), node.is_nullable === 'YES' || node.is_nullable === true ? '✔️' : '❌']),
          node.character_set_name && h('div', [h('b', '字符集: '), node.character_set_name]),
          node.collation_name && h('div', [h('b', '排序规则: '), node.collation_name]),
          node.character_octet_length !== undefined && h('div', [h('b', '长度: '), node.character_octet_length]),
          node.table_schema && h('div', [h('b', 'Schema: '), node.table_schema]),
          node.table_name && h('div', [h('b', '表名: '), node.table_name])
        ].filter(Boolean)),
        trigger: () => h('span', [
          getColumnIcon(node),
          h('span', { style: { fontWeight: 400, color: '#333', fontSize: '13px' } }, [
            node.label,
            infoArr.length > 0 ? h('span', { style: { marginLeft: '6px', color: '#999', fontSize: '11px', maxWidth: '120px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', verticalAlign: 'middle' } }, infoArr[0]) : null
          ])
        ])
      })
    }
    // 索引节点
    if (/^db-[^\-]+-table-[^\-]+-index-/.test(node.key)) {
      const infoArr = []
      if (node.non_unique !== undefined) infoArr.push(node.non_unique === 0 ? '唯一' : '非唯一')
      if (node.column_name) infoArr.push('字段: ' + node.column_name)
      if (node.index_type) infoArr.push('类型: ' + node.index_type)
      return h(NPopover, {
        trigger: 'hover',
        placement: 'right-start',
        style: 'min-width: 320px;'
      }, {
        default: () => h('div', {
          style: 'display: flex; flex-direction: column; gap: 8px; min-width: 320px;'
        }, [
          h('div', [h('b', '索引名: '), node.key_name ?? node.label]),
          node.column_name && h('div', [h('b', '字段: '), node.column_name]),
          node.index_type && h('div', [h('b', '类型: '), node.index_type]),
          node.non_unique !== undefined && h('div', [h('b', '是否唯一: '), node.non_unique === 0 ? '是' : '否']),
          node.index_comment && h('div', [h('b', '注释: '), node.index_comment]),
          node.table_schema && h('div', [h('b', 'Schema: '), node.table_schema]),
          node.table_name && h('div', [h('b', '表名: '), node.table_name])
        ].filter(Boolean)),
        trigger: () => h('span', [
          getIndexIcon(),
          h('span', { style: 'font-weight: 400; color: #333;' }, [
            node.label,
            infoArr.length > 0 ? h('span', { style: { marginLeft: '8px', color: '#999', fontSize: '12px' } }, infoArr.join(' | ')) : null
          ])
        ])
      })
    }
    // schema节点
    if (/^db-[^\-]+$/.test(node.key)) {
      return h('span', [
        h(NPopover, {
          trigger: 'hover',
          placement: 'right-start',
          style: 'min-width: 320px;'
        }, {
          default: () => h('div', {
            style: 'display: flex; flex-direction: column; gap: 8px; min-width: 320px;'
          }, [
            h('div', [h('b', 'Schema: '), node.label]),
            node.default_character_set_name && h('div', [h('b', '字符集: '), node.default_character_set_name]),
            node.default_collation_name && h('div', [h('b', '排序规则: '), node.default_collation_name])
          ].filter(Boolean)),
          trigger: () => h(NIcon, { size: 18, color: '#909399', style: { marginRight: '4px' } }, () => h(ServerOutline))
        }),
        h('span', { style: 'color: #333; marginLeft: ' + '4px' }, node.label)
      ])
    }
    // 其他节点
    return node.label
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
    const dbList = await DatabaseAPI.list(connectionId)
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
  // 懒加载表节点（schema下的table）
  if (/^db-[^\-]+$/.test(node.key) && (!node.children || node.children.length === 0)) {
    console.log('[Tree] handleLoad: schema node, key=', node.key, node)
    const connectionId = sqlEditor.currentConnection?.id
    if (!connectionId) {
      console.warn('[Tree] handleLoad: no connectionId, abort')
      return
    }
    try {
      console.log('[Tree] handleLoad: fetchTableList', connectionId, node.label)
      const tableList = await TableAPI.list(connectionId, node.label)
      console.log('[Tree] handleLoad: tableList result', tableList)
      node.children = tableList.map(tbl => ({
        key: node.key + '-table-' + tbl.table_name,
        label: tbl.table_name,
        isLeaf: false,
        table_schema: tbl.table_schema,
        create_time: tbl.create_time ? new Date(tbl.create_time).toLocaleString() : '',
        table_comment: tbl.table_comment,
        children: undefined // 关键：初始化为 undefined，确保懒加载
      }))
      console.log('[Tree] handleLoad: table children set', node.children)
      treeData.value = [...treeData.value]
      console.log('[Tree] handleLoad: treeData updated', treeData.value)
    } catch (err) {
      console.error('[Tree] handleLoad: fetchTableList error', err)
    }
  }
  // 懒加载列和索引节点（table下的column和index）
  else if (/^db-[^\-]+-table-[^\-]+$/.test(node.key) && (!node.children || node.children.length === 0)) {
    // 分组展示“列”和“索引”
    const connectionId = sqlEditor.currentConnection?.id
    if (!connectionId) {
      console.warn('[Tree] handleLoad: no connectionId, abort')
      return
    }
    const [schema, table] = (() => {
      const m = node.key.match(/^db-([^\-]+)-table-([^\-]+)$/)
      return m ? [m[1], m[2]] : ['', '']
    })()
    console.log('[Tree] handleLoad: fetchColumnList & fetchIndexList', connectionId, schema, table)
    try {
      const columnList = await TableAPI.columns(connectionId, schema, table)
      const indexList = await TableAPI.indexes(connectionId, schema, table)
      node.children = [
        {
          key: node.key + '-columns',
          label: '列',
          isLeaf: false,
          children: columnList.map(col => ({
            key: node.key + '-column-' + col.column_name,
            label: col.column_name,
            isLeaf: true,
            ...col
          }))
        },
        {
          key: node.key + '-indexes',
          label: '索引',
          isLeaf: false,
          children: indexList.map(idx => ({
            key: node.key + '-index-' + idx.key_name,
            label: idx.key_name,
            isLeaf: true,
            ...idx
          }))
        }
      ]
      treeData.value = [...treeData.value]
    } catch (err) {
      console.error('[Tree] handleLoad: fetchColumnList/fetchIndexList error', err)
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
        <SqlTemplateSidebar :showSidebar="props.showSidebar" @insert-template="(sql) => emit('insert-template', sql)" />
      </n-card>
    </n-space>
  </n-layout-sider>
</template>