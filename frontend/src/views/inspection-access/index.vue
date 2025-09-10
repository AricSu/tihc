<template>
<div class="inspection-access-page">
  <n-card title="巡检接入" size="large">
    <n-form :model="form" label-width="120">
      <div class="form-top-area">
        <n-form-item label="Metrics 地址">
          <n-input v-model:value="form.metrics.url" placeholder="请输入 Metrics 地址" style="width: 260px;" />
        </n-form-item>
        <n-form-item label="Metrics Cookie">
          <n-input v-model:value="form.metrics.cookie" placeholder="请输入 Metrics Cookie" style="width: 260px;" />
        </n-form-item>
        <n-form-item label="TiDB 地址">
          <n-input v-model:value="form.tidb.url" placeholder="请输入 TiDB 地址" style="width: 260px;" />
        </n-form-item>
        <n-form-item label="TiDB 用户">
          <n-input v-model:value="form.tidb.user" placeholder="请输入 TiDB 用户" style="width: 260px;" />
        </n-form-item>
        <n-form-item label="TiDB 密码">
          <n-input v-model:value="form.tidb.password" type="password" placeholder="请输入 TiDB 密码" style="width: 260px;" />
        </n-form-item>
        <n-form-item label="时区">
          <n-select v-model:value="form.timezone" :options="timezones" style="width: 180px;" />
        </n-form-item>
        <n-form-item label="报告目录">
          <n-input v-model:value="form.reportDir" placeholder="请输入报告目录" style="width: 180px;" />
        </n-form-item>
        <n-form-item label="时间范围">
          <n-date-picker v-model:value="form.timeRange" type="datetimerange" clearable :actions="['confirm']" :disabled-date="disableFutureDates" style="width: 220px;" />
        </n-form-item>
        <n-form-item label="报告名称">
          <n-input v-model:value="form.reportName" disabled style="width: 220px;" />
        </n-form-item>
        <div class="btn-area">
          <n-button type="primary" @click="handleSubmit" :loading="loading">生成报告</n-button>
        </div>
      </div>
    </n-form>
</n-card>
</div>
</template>

<script setup>
import { ref } from 'vue'

const columns = [
  { title: '#', key: 'index', render: (row, index) => index + 1 },
  { title: '报告名称', key: 'reportName' },
  { title: '时间范围', key: 'timeRange', render: (row) => {
      if (!row.timeRange || row.timeRange.length !== 2) return '';
      const [start, end] = row.timeRange.map(ts => {
        const date = new Date(ts * 1000)
        return date.toLocaleString('zh-CN')
      })
      return `${start} - ${end}`
    }
  },
  { title: '健康状态', key: 'healthStatus' },
  { title: '建议', key: 'recommendations' },
  { title: '创建时间', key: 'createTime', render: (row) => {
      if (!row.createTime) return 'N/A'
      const date = new Date(row.createTime * 1000)
      return isNaN(date.getTime()) ? 'Invalid Date' : date.toLocaleString('zh-CN')
    }
  }
]

const generatedReports = ref([])
const currentPage = ref(1)
const itemsPerPage = ref(15)
const loading = ref(false)

function updatePagination(pagination) {
  currentPage.value = pagination.page
  itemsPerPage.value = pagination.pageSize
}

function genReportName() {
  const now = new Date()
  return `inspection-${now.toISOString().replace(/[:.]/g, '-')}`
}

const now = new Date()
const oneHourAgo = new Date(now.getTime() - 60 * 60 * 1000)
const form = ref({
  reportName: genReportName(),
  metrics: { url: '', cookie: '' },
  tidb: { url: '', user: '', password: '' },
  timezone: Intl.DateTimeFormat().resolvedOptions().timeZone,
  reportDir: '',
  timeRange: [oneHourAgo, now]
})

watch(() => form.value.timeRange, (val) => {
  // 防止非法值导致 DatePicker 报错
  if (!Array.isArray(val) || val.length !== 2 || !(val[0] instanceof Date) || !(val[1] instanceof Date)) {
    form.value.timeRange = [oneHourAgo, now]
  }
  form.value.reportName = genReportName()
})

const timezones = Intl.supportedValuesOf('timeZone').map(tz => ({ label: tz, value: tz }))

function disableFutureDates(date) {
  return date.getTime() > Date.now()
}

async function handleSubmit() {
  loading.value = true
  try {
    const f = form.value
    if (!f.metrics.url || !f.metrics.cookie || !f.tidb.url || !f.tidb.user || !f.timezone || f.timeRange.length !== 2) {
      window.$message?.error('请填写完整信息')
      loading.value = false
      return
    }
    const payload = {
      report_name: f.reportName,
      grafana_url: f.metrics.url,
      grafana_cookie: f.metrics.cookie,
      db_url: f.tidb.url,
      db_user: f.tidb.user,
      db_password: f.tidb.password,
      timezone: f.timezone,
      time_range: f.timeRange.map(d => Math.floor(new Date(d).getTime() / 1000)),
      report_dir: f.reportDir
    }
    await axios.post('/api/report/generate', payload)
    window.$message?.success('报告生成请求已提交')
  } catch (e) {
    window.$message?.error('生成失败')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.inspection-access-page {
  padding: 24px;
}
</style>