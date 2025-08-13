<template>
  <div class="output-root">
    <n-card embedded class="result-card output-scroll-card">
      <div class="result-header">
      <n-tag :type="statusTagType" size="large" class="result-status-tag">
        {{ t(statusText) }}
      </n-tag>
      <div class="result-title">{{ t(statusTitle) }}</div>
      <div class="result-desc">{{ t(statusDescription) }}</div>
    </div>
    <div class="result-meta">
      <div v-if="result && result.checkDuration !== undefined && result.checkDuration !== null" class="custom-statistic">
        <span class="stat-label">{{ t('ddlCheck.output.duration') }}</span>
        <span class="stat-value">{{ Number(result.checkDuration) }}<span class="stat-unit">ms</span></span>
      </div>
    </div>

    <n-alert v-if="result && result.error" type="error" class="result-alert">
      {{ result.error }}
    </n-alert>
    <div v-if="result && result.issues && result.issues.length" class="result-section">
      <div class="result-section-title">{{ t('ddlCheck.output.issues') }}</div>
      <ul class="result-list">
        <li v-for="(issue, idx) in result.issues" :key="idx">
          <n-icon size="16" color="#faad14" style="vertical-align: middle;"><Icon icon="mdi:alert-circle-outline" /></n-icon>
          <span class="result-list-text">{{ issue }}</span>
        </li>
      </ul>
    </div>
    <div v-if="result && result.recommendations && result.recommendations.length" class="result-section">
      <div class="result-section-title">{{ t('ddlCheck.output.recommendations') }}</div>
      <ul class="result-list">
        <li v-for="(rec, idx) in result.recommendations" :key="idx">
          <n-icon size="16" color="#36cfc9" style="vertical-align: middle;"><Icon icon="mdi:lightbulb-on-outline" /></n-icon>
          <span class="result-list-text">{{ rec }}</span>
        </li>
      </ul>
    </div>
    <div v-if="result && result.lossy_status === 'Lossy'" class="result-section">
      <div class="result-section-title">{{ t('ddlCheck.output.executionAdvice') }}</div>
      <div class="result-code-block">
        <n-button size="small" @click="copyExecutionExample" style="margin-bottom:8px;">{{ t('ddlCheck.output.copyCode') }}</n-button>
        <n-code language="sql" :code="generateExecutionExample()" show-line-numbers />
      </div>
    </div>
    </n-card>
  </div>
</template>


<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
const { t } = useI18n()
const props = defineProps<{ result: any }>()

const statusTagType = computed(() => {
  if (!props.result) return 'default'
  switch (props.result.lossy_status) {
    case 'Safe': return 'success'
    case 'Lossy': return 'error'
    case 'Unknown': return 'warning'
    default: return 'default'
  }
})
const statusText = computed(() => {
  if (!props.result) return 'ddlCheck.output.status.unknown'
  switch (props.result.lossy_status) {
    case 'Safe': return 'ddlCheck.output.status.safe'
    case 'Lossy': return 'ddlCheck.output.status.lossy'
    case 'Unknown': return 'ddlCheck.output.status.unknown'
    default: return 'ddlCheck.output.status.unknown'
  }
})
const statusTitle = computed(() => {
  if (!props.result) return 'ddlCheck.output.title.default'
  switch (props.result.lossy_status) {
    case 'Safe': return 'ddlCheck.output.title.safe'
    case 'Lossy': return 'ddlCheck.output.title.lossy'
    case 'Unknown': return 'ddlCheck.output.title.unknown'
    default: return 'ddlCheck.output.title.default'
  }
})
const statusDescription = computed(() => {
  if (!props.result) return 'ddlCheck.output.desc.default'
  switch (props.result.lossy_status) {
    case 'Safe': return 'ddlCheck.output.desc.safe'
    case 'Lossy': return 'ddlCheck.output.desc.lossy'
    case 'Unknown': return 'ddlCheck.output.desc.unknown'
    default: return 'ddlCheck.output.desc.default'
  }
})

function generateExecutionExample() {
  if (!props.result) return ''
  // 假设 result.sql 为原始 SQL
  const sql = props.result.sql || ''
  // 提取表名
  const tableRegex = /(?:CREATE\s+TABLE|ALTER\s+TABLE)\s+([\w.]+)/gi
  const tables: string[] = []
  let match
  while ((match = tableRegex.exec(sql))) {
    if (match[1]) tables.push(match[1])
  }
  return `-- 1. 执行你的 DDL 语句\n${sql}\n\n-- 2. 立即执行 ANALYZE TABLE 以更新统计信息\nANALYZE TABLE ${tables.join(', ')};`
}

async function copyExecutionExample() {
  try {
    await navigator.clipboard.writeText(generateExecutionExample())
    window.$message.success(t('ddlCheck.output.copySuccess'))
  } catch (e) {
    window.$message.error(t('ddlCheck.output.copyFail'))
  }
}
</script>



<style scoped>
.output-root {
  height: 100%;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
.result-card {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.04);
  padding: 0 0 16px 0;
  min-height: 0;
  height: 100%;
  display: flex;
  flex-direction: column;
}
.output-scroll-card {
  height: 100%;
  min-height: 0;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
.custom-statistic {
  font-size: 16px;
  font-weight: 500;
  padding: 0 0 4px 0;
  color: #333;
  display: flex;
  align-items: baseline;
  gap: 8px;
}
.stat-label {
  color: #888;
  font-size: 14px;
  margin-right: 2px;
}
.stat-value {
  font-size: 18px;
  font-weight: 600;
}
.stat-unit {
  font-size: 14px;
  color: #888;
  margin-left: 2px;
}
.output-scroll-card > .result-header,
.output-scroll-card > .result-meta,
.output-scroll-card > .result-alert,
.output-scroll-card > .result-section {
  flex-shrink: 0;
}
.output-scroll-card > .result-section:last-child {
  margin-bottom: 12px;
}
.output-scroll-card {
  flex: 1 1 auto;
}
.result-header {
  padding: 24px 24px 8px 24px;
  border-bottom: 1px solid #f0f0f0;
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 8px;
}
.result-status-tag {
  font-size: 15px;
  padding: 0 12px;
  height: 28px;
  display: flex;
  align-items: center;
}
.result-title {
  font-size: 20px;
  font-weight: 600;
  margin-top: 2px;
}
.result-desc {
  color: #888;
  font-size: 14px;
  margin-bottom: 2px;
}
.result-meta {
  padding: 12px 24px 0 24px;
}
.result-alert {
  margin: 16px 24px 0 24px;
}
.result-section {
  margin: 18px 24px 0 24px;
}
.result-section-title {
  font-size: 15px;
  font-weight: 500;
  margin-bottom: 6px;
}
.result-list {
  list-style: none;
  padding: 0;
  margin: 0;
}
.result-list li {
  display: flex;
  align-items: flex-start;
  gap: 6px;
  margin-bottom: 4px;
}
.result-list-text {
  font-size: 14px;
  color: #333;
}
.result-code-block {
  margin-top: 8px;
  background: #f8f8fa;
  border-radius: 8px;
  padding: 12px;
}
</style>