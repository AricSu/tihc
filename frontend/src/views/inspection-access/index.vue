<template>
  <n-config-provider>
      <n-space vertical>
        <!-- 输入区域 -->
        <n-card title="输入配置" size="small" class="section-card">
          <!-- 单行输入：时间范围、时区和操作按钮 -->
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
            
            <div class="extension-status">
              <n-tag :type="extensionStatusType" size="small">
                {{ extensionStatusText }}
              </n-tag>
              <n-button 
                size="small" 
                text 
                @click="$router.push('/extension-config')"
              >
                配置扩展
              </n-button>
              
              <!-- 调试按钮 -->
              <n-button 
                size="small" 
                type="info"
                text
                @click="debugExtensionStatus"
              >
                调试状态
              </n-button>
            </div>
            
            <n-space>
              <n-button 
                type="primary" 
                @click="handleSubmit" 
                :loading="loading"
                :disabled="!canGenerate"
              >
                生成报告
              </n-button>
              
              <!-- 强制启用按钮（调试用） -->
              <n-button 
                v-if="!canGenerate"
                type="warning" 
                @click="forceGenerate" 
                :loading="loading"
                secondary
              >
                强制生成
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
  NDataTable,
  NPagination,
  NSpin,
  NPopover
} from 'naive-ui'
import { extensionData, extensionApiHandler } from '@/api/extension'
import { useRouter } from 'vue-router'
import axios from 'axios'

const router = useRouter()

// 响应式状态
const loading = ref(false)
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

// 简化的扩展状态
const extensionStatusType = computed(() => {
  switch (extensionData.connectionStatus) {
    case 'connected': return 'success'
    case 'waiting': return 'warning'
    default: return 'error'
  }
})

const extensionStatusText = computed(() => {
  switch (extensionData.connectionStatus) {
    case 'connected': return '扩展已连接'
    case 'waiting': return '等待连接'
    default: return '扩展未连接'
  }
})

const hasValidConfig = computed(() => {
  // 调试信息
  console.log('检查配置有效性:')
  console.log('- 扩展连接状态:', extensionData.connectionStatus)
  console.log('- tokens 数据:', extensionData.tokens)
  
  // 临时：如果扩展未连接，仍然允许生成报告（用于调试）
  if (extensionData.connectionStatus !== 'connected') {
    console.log('扩展未连接，但允许继续操作')
    return true // 临时改为 true
  }
  
  const tokens = extensionData.tokens || {}
  for (const domainData of Object.values(tokens)) {
    if (domainData.grafana || domainData.clinic) {
      return true
    }
  }
  return false
})

const canGenerate = computed(() => {
  const result = form.value.timeRange && 
         form.value.timeRange.length === 2 && 
         form.value.timezone && 
         hasValidConfig.value
  
  // 调试信息
  console.log('生成报告按钮可用性检查:')
  console.log('- 时间范围有效:', !!(form.value.timeRange && form.value.timeRange.length === 2))
  console.log('- 时区已选择:', !!form.value.timezone)
  console.log('- 配置有效:', hasValidConfig.value)
  console.log('- 最终结果:', result)
  
  return result
})

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
        Critical: 'error',
        Processing: 'info',
        Pending: 'default'
      }
      const statusText = {
        Healthy: '健康',
        Warning: '警告',
        Critical: '严重',
        Processing: '执行中',
        Pending: '待执行'
      }
      return h(NTag, { 
        type: statusColors[row.healthStatus] || 'default' 
      }, {
        default: () => statusText[row.healthStatus] || row.healthStatus
      })
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
            {
              default: () => '查看详情'
            }
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

// 方法
function getHealthStatusFromItem(item) {
  switch (item.type) {
    case 'inspection_report':
      return item.healthStatus || 'Healthy'
    case 'failed_task':
      return 'Critical'
    case 'running_task':
      return 'Processing'
    case 'created_task':
      return 'Pending'
    default:
      return 'Pending'
  }
}

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

async function handleSubmit(force = false) {
  if (!force && !canGenerate.value) {
    window.$message?.error('请完善配置信息或配置扩展')
    return
  }

  loading.value = true
  try {
    // 准备请求数据
    const payload = {
      time_range: form.value.timeRange.map(d => Math.floor(new Date(d).getTime() / 1000)),
      timezone: form.value.timezone,
      clinic_url: 'test3990'
    }

    console.log('发送巡检请求:', payload)

    // 调用新的巡检API
    const response = await axios.post('/api/inspection/create', payload)
    
    if (response.data.success) {
      window.$message?.success(`巡检任务创建成功！任务ID: ${response.data.task_id}`)
      console.log('巡检任务创建成功，任务ID:', response.data.task_id)
      
      // 刷新任务列表
      await fetchReports()
    } else {
      window.$message?.error(`创建失败: ${response.data.message}`)
    }
    
  } catch (error) {
    console.error('创建巡检任务失败:', error)
    const errorMessage = error.response?.data?.message || error.message || '创建巡检任务失败'
    window.$message?.error(errorMessage)
  } finally {
    loading.value = false
  }
}

async function fetchReports() {
  reportsLoading.value = true
  try {
    // 使用新的统一API获取巡检摘要（包含任务和报告）
    const response = await axios.get('/api/inspection/summary')
    if (response.data.success && response.data.summary && Array.isArray(response.data.summary.items)) {
      // 转换统一摘要数据为报告格式
      generatedReports.value = response.data.summary.items.map(item => ({
        reportId: item.id,
        reportName: item.title,
        timeRange: item.timeRange,
        healthStatus: getHealthStatusFromItem(item),
        recommendations: item.type === 'inspection_report' ? 
                        (item.recommendations || `针对 ${item.clinicUrl} 的巡检建议`) :
                        `任务状态: ${item.status}, Clinic URL: ${item.clinicUrl}`,
        createTime: item.createTime,
        taskType: item.type,
        status: item.status
      }))
      console.log('获取到巡检摘要:', generatedReports.value)
      console.log('统计信息 - 总计:', response.data.summary.total)
    } else {
      console.error('Invalid response format:', response.data)
      generatedReports.value = []
    }
  } catch (error) {
    console.error('Error fetching inspection summary:', error)
    generatedReports.value = []
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

// 强制生成报告（忽略配置验证）
function forceGenerate() {
  console.log('强制生成报告，忽略配置验证')
  handleSubmit(true) // 传递 force 参数
}

// 调试扩展状态的函数
function debugExtensionStatus() {
  console.log('=== 扩展状态调试信息 ===')
  console.log('连接状态:', extensionData.connectionStatus)
  console.log('扩展数据:', extensionData)
  console.log('配置有效性:', hasValidConfig.value)
  console.log('可生成报告:', canGenerate.value)
  console.log('===================')
  
  // 显示消息提示
  window.$message?.info(`扩展状态: ${extensionStatusText.value}`)
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

.extension-status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: #f7fafc;
  border-radius: 6px;
  border: 1px solid #e2e8f0;
}

@media (max-width: 768px) {
  .section-card .n-space {
    flex-direction: column !important;
    gap: 12px !important;
  }
}
</style>