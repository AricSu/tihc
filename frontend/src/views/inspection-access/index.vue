<template>
  <n-config-provider>
      <n-space vertical>
        <!-- 输入区域 -->
        <n-card title="输入配置" size="small" class="section-card">
          <!-- 单行输入：时间范围、时区、扩展状态和操作按钮 -->
          <n-space align="center" justify="space-between">
            <n-date-picker 
              v-model:value="form.timeRange" 
              type="datetimerange" 
              clearable 
              :actions="['confirm']" 
              :disabled-date="disableFutureDates"
              placeholder="选择巡检时间范围"
              style="width: 350px;"
            />
            
            <n-select 
              v-model:value="form.timezone" 
              :options="timezones" 
              placeholder="选择时区"
              filterable
              style="width: 180px;"
            />
            
            <div class="config-status-inline">
              <div class="status-indicator">
                <div class="status-dot" :class="statusClass"></div>
                <span class="status-text">{{ statusText }}</span>
              </div>
              <div v-if="hasExtensionData" class="config-tags">
                <n-tag v-if="grafanaConfig" type="success" size="small">Grafana</n-tag>
                <n-tag v-if="clinicConfig" type="success" size="small">TiDB Cloud</n-tag>
              </div>
            </div>
            
            <n-space>
              <n-button v-if="hasExtensionData" size="small" @click="refreshConfig" :loading="refreshing">
                刷新
              </n-button>
              
              <n-button size="small" @click="showGuideDrawer = true" text type="primary">
                帮助
              </n-button>
              
              <n-button 
                type="primary" 
                @click="handleSubmit" 
                :loading="loading"
                :disabled="!canGenerate"
              >
                生成报告
              </n-button>
              
              <n-button @click="resetForm" secondary>
                重置
              </n-button>
            </n-space>
          </n-space>
        </n-card>

        <!-- 输出区域 -->
        <n-card title="生成的巡检报告" size="small" class="section-card">
          <n-space vertical>
            <div v-if="reportsLoading" class="loading-container">
              <n-spin size="large" />
            </div>
            <div v-else>
              <n-data-table
                :columns="columns"
                :data="generatedReports"
                :pagination="{
                  page: currentPage,
                  pageSize: itemsPerPage,
                  itemCount: generatedReports.length
                }"
                @update:pagination="updatePagination"
              />
              <n-pagination
                v-model:page="currentPage"
                v-model:page-size="itemsPerPage"
                :page-sizes="[5, 10, 15, 20, 50]"
                :item-count="generatedReports.length"
                show-size-picker
                show-quick-jumper
                :default-page-size="15"
                style="margin-top: 20px; text-align: center;"
              />
            </div>
          </n-space>
        </n-card>
      </n-space>

    <!-- 配置帮助抽屉 -->
    <n-drawer v-model:show="showGuideDrawer" :width="400" placement="right">
      <n-drawer-content title="配置帮助" closable>
        <div class="help-content">
          <h4>如何配置扩展？</h4>
          <ol>
            <li>下载并安装 TiHC 扩展</li>
            <li>访问 Grafana 或 TiDB Cloud</li>
            <li>扩展会自动收集配置信息</li>
            <li>返回此页面即可看到配置状态</li>
          </ol>
        </div>
      </n-drawer-content>
    </n-drawer>
  </n-config-provider>
</template>

<script setup>
import { ref, computed, onMounted, h } from 'vue'
import { 
  NConfigProvider,
  NCard,
  NSpace,
  NDatePicker,
  NSelect,
  NButton,
  NTag,
  NIcon,
  NDrawer,
  NDrawerContent,
  NDataTable,
  NPagination,
  NSpin,
  NPopover
} from 'naive-ui'
import { extensionData, extensionApiHandler } from '@/api/extension'
import axios from 'axios'

// 响应式状态
const loading = ref(false)
const refreshing = ref(false)
const showGuideDrawer = ref(false)
const reportsLoading = ref(false)
const generatedReports = ref([])
const currentPage = ref(1)
const itemsPerPage = ref(15)

// 表单数据
const now = new Date()
const oneHourAgo = new Date(now.getTime() - 60 * 60 * 1000)
const form = ref({
  timeRange: [oneHourAgo, now],
  timezone: Intl.DateTimeFormat().resolvedOptions().timeZone
})

// 时区选项
const timezones = Intl.supportedValuesOf('timeZone').map(tz => ({ 
  label: tz, 
  value: tz 
}))

// 表格列定义
const columns = [
  { title: '#', key: 'index', render: (row, index) => index + 1 },
  { title: '报告名称', key: 'reportName' },
  { 
    title: '时间范围', 
    key: 'timeRange', 
    render: (row) => {
      if (!row.timeRange || !Array.isArray(row.timeRange)) return 'N/A'
      const [start, end] = row.timeRange.map((timestamp) => {
        const date = new Date(timestamp * 1000)
        return date.toLocaleString('zh-CN', { timeZone: form.value.timezone })
      })
      return `${start} - ${end}`
    }
  },
  { 
    title: '健康状态', 
    key: 'healthStatus', 
    render: (row) => {
      const statusColors = {
        Healthy: 'success',
        Warning: 'warning', 
        Critical: 'error'
      }
      const statusText = {
        Healthy: '健康',
        Warning: '警告',
        Critical: '严重'
      }
      return h(NTag, { 
        type: statusColors[row.healthStatus] || 'default' 
      }, statusText[row.healthStatus] || row.healthStatus)
    }
  },
  { 
    title: '建议', 
    key: 'recommendations', 
    render: (row) => {
      const text = row.recommendations || '无建议'
      const shortText = text.length > 30 ? text.slice(0, 30) + '...' : text
      return h(
        NPopover,
        {},
        {
          trigger: () => h('span', { style: { cursor: 'pointer' } }, shortText),
          default: () => h('div', { 
            style: { 
              maxWidth: '300px', 
              whiteSpace: 'normal', 
              wordWrap: 'break-word' 
            } 
          }, text)
        }
      )
    }
  },
  { 
    title: '创建时间', 
    key: 'createTime', 
    render: (row) => {
      if (!row.createTime) return 'N/A'
      const date = new Date(row.createTime * 1000)
      return isNaN(date.getTime()) ? 'Invalid Date' : date.toLocaleString('zh-CN', { timeZone: form.value.timezone })
    }
  },
  { 
    title: '操作', 
    key: 'actions', 
    render: (row) => {
      return h(
        NPopover,
        {},
        {
          trigger: () => h(
            NButton,
            {
              size: 'small',
              onClick: () => viewReport(row)
            },
            '查看详情'
          ),
          default: () => h('div', {
            style: {
              maxWidth: '300px',
              whiteSpace: 'normal',
            }
          }, `报告ID: ${row.reportId || '无ID'}`)
        }
      )
    }
  }
]

// 计算属性
const hasExtensionData = computed(() => {
  return extensionData.connectionStatus === 'connected' && 
         Object.keys(extensionData.tokens).length > 0
})

const statusClass = computed(() => {
  const status = extensionData.connectionStatus
  return {
    'connected': status === 'connected',
    'disconnected': status === 'disconnected' || status === 'error',
    'waiting': status === 'waiting'
  }
})

const statusText = computed(() => {
  switch (extensionData.connectionStatus) {
    case 'connected': return '扩展已连接'
    case 'waiting': return '等待连接...'
    case 'disconnected': return '扩展未连接'
    case 'error': return '连接错误'
    default: return '未知状态'
  }
})

const grafanaConfig = computed(() => {
  if (!hasExtensionData.value) return null
  
  for (const [domain, domainData] of Object.entries(extensionData.tokens)) {
    if (domainData.grafana) {
      return { domain, data: domainData.grafana }
    }
  }
  return null
})

const clinicConfig = computed(() => {
  if (!hasExtensionData.value) return null
  
  for (const [domain, domainData] of Object.entries(extensionData.tokens)) {
    if (domainData.clinic) {
      return { domain, data: domainData.clinic }
    }
  }
  return null
})

const canGenerate = computed(() => {
  return form.value.timeRange && 
         form.value.timeRange.length === 2 && 
         form.value.timezone && 
         (grafanaConfig.value || clinicConfig.value)
})

// 方法
function disableFutureDates(date) {
  return date.getTime() > Date.now()
}

function resetForm() {
  const now = new Date()
  const oneHourAgo = new Date(now.getTime() - 60 * 60 * 1000)
  
  form.value = {
    timeRange: [oneHourAgo, now],
    timezone: Intl.DateTimeFormat().resolvedOptions().timeZone
  }
  
  window.$message?.info('配置已重置')
}

async function refreshConfig() {
  refreshing.value = true
  try {
    await extensionApiHandler.manualSync()
    window.$message?.success('配置已刷新')
  } catch (error) {
    console.error('刷新配置失败:', error)
    window.$message?.error('刷新配置失败')
  } finally {
    refreshing.value = false
  }
}

async function handleSubmit() {
  if (!canGenerate.value) {
    window.$message?.error('请完善配置信息')
    return
  }

  loading.value = true
  try {
    const payload = {
      time_range: form.value.timeRange.map(d => Math.floor(new Date(d).getTime() / 1000)),
      timezone: form.value.timezone
    }

    // 添加配置信息
    if (grafanaConfig.value) {
      const protocol = grafanaConfig.value.domain.includes('localhost') ? 'http' : 'https'
      payload.grafana_url = `${protocol}://${grafanaConfig.value.domain}`
      
      const cookies = []
      const data = grafanaConfig.value.data
      if (data.session_id) cookies.push(`grafana_session=${data.session_id}`)
      if (data.csrf_token) cookies.push(`grafana_session_expiry=${data.csrf_token}`)
      if (data.auth_token) cookies.push(`grafana_token=${data.auth_token}`)
      
      payload.grafana_cookie = cookies.join('; ')
    }

    if (clinicConfig.value) {
      payload.clinic_config = clinicConfig.value.data
    }

    await axios.post('/api/report/generate', payload)
    window.$message?.success('巡检报告生成请求已提交')
    
    // 刷新报告列表
    await fetchReports()
    
  } catch (error) {
    console.error('生成报告失败:', error)
    window.$message?.error('生成报告失败')
  } finally {
    loading.value = false
  }
}

async function fetchReports() {
  reportsLoading.value = true
  try {
    const response = await axios.get('/api/report/summary')
    if (response.status === 200 && Array.isArray(response.data)) {
      generatedReports.value = response.data
    } else {
      console.error('Invalid response format:', response.data)
    }
  } catch (error) {
    console.error('Error fetching reports:', error)
  } finally {
    reportsLoading.value = false
  }
}

function updatePagination(pagination) {
  currentPage.value = pagination.page
  itemsPerPage.value = pagination.pageSize
}

function viewReport(row) {
  // 这里可以根据实际需求实现报告查看逻辑
  const reportId = row.reportId
  if (reportId) {
    // 如果有路由，可以打开新页面
    window.open(`/report/${reportId}`, '_blank')
  } else {
    window.$message?.warning('报告ID不存在')
  }
}

// 组件挂载时获取报告列表
onMounted(() => {
  fetchReports()
})
</script>

<style scoped>
.inspection-report {
  padding: 20px;
  max-width: 1000px;
  margin: 0 auto;
}

.section-card {
  margin-bottom: 20px;
}

.loading-container {
  text-align: center;
  padding: 20px;
}

.config-status-inline {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 12px;
  background: #f7fafc;
  border-radius: 6px;
  border: 1px solid #e2e8f0;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
}

.status-dot.connected {
  background: #48bb78;
}

.status-dot.waiting {
  background: #ed8936;
}

.status-dot.disconnected {
  background: #f56565;
}

.status-text {
  font-size: 13px;
  color: #4a5568;
  white-space: nowrap;
}

.config-tags {
  display: flex;
  gap: 4px;
}

@media (max-width: 768px) {
  .section-card .n-space {
    flex-direction: column !important;
    gap: 12px !important;
  }
}
</style>