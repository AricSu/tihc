<template>
  <AppPage>
    <!-- 页面标题 -->
    <div class="flex items-center justify-center mb-8">
      <n-icon class="text-orange-500 mr-3" size="32">
        <svg viewBox="0 0 24 24">
          <path d="M12 2L1 21h22L12 2zm0 3.99L19.53 19H4.47L12 5.99zM11 16h2v2h-2zm0-6h2v4h-2z" fill="currentColor"/>
        </svg>
      </n-icon>
      <div class="text-center">
        <h1 class="text-3xl font-bold text-gray-800 mb-2">DDL 预检查工具</h1>
        <p class="text-gray-600">检查 DDL 语句是否会造成数据丢失，提供安全建议</p>
      </div>
    </div>

    <n-grid :cols="24" :x-gap="24">
      <!-- SQL 输入区域 -->
      <n-gi :span="showResults ? 12 : 24">
        <n-card title="SQL 语句输入">
          <n-form ref="formRef" :model="formData" :rules="rules">
            <n-form-item label="SQL 语句" path="sql">
              <n-input
                v-model:value="formData.sql"
                type="textarea"
                :rows="14"
                placeholder="请输入完整的 DDL 语句，例如：&#10;CREATE DATABASE testdb;&#10;CREATE TABLE testdb.users (id INT PRIMARY KEY, name VARCHAR(50));&#10;ALTER TABLE testdb.users MODIFY COLUMN name VARCHAR(100);"
                :disabled="loading"
                @input="clearResults"
              />
            </n-form-item>

            <n-form-item>
              <n-checkbox v-model:checked="formData.collationEnabled">
                启用新的排序规则
              </n-checkbox>
            </n-form-item>

            <n-form-item>
              <n-space>
                <n-button
                  type="primary"
                  size="large"
                  :loading="loading"
                  :disabled="!formData.sql.trim()"
                  @click="runPrecheck"
                >
                  <template #icon>
                    <n-icon>
                      <svg viewBox="0 0 24 24">
                        <path d="M8 5v14l11-7z" fill="currentColor"/>
                      </svg>
                    </n-icon>
                  </template>
                  {{ loading ? '检查中...' : '开始检查' }}
                </n-button>
                
                <n-button size="large" @click="clearAll">
                  <template #icon>
                    <n-icon>
                      <svg viewBox="0 0 24 24">
                        <path d="M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z" fill="currentColor"/>
                      </svg>
                    </n-icon>
                  </template>
                  清空
                </n-button>
              </n-space>
            </n-form-item>
          </n-form>
        </n-card>
      </n-gi>

      <!-- 结果显示区域 -->
      <n-gi v-if="showResults" :span="12">
        <n-card title="检查结果">
          <!-- 状态标识 -->
          <div class="flex items-center justify-between mb-4">
            <span class="text-lg font-semibold">检查状态</span>
            <n-tag :type="statusTagType" size="large">
              <template #icon>
                <n-icon>
                  <component :is="statusIconComponent" />
                </n-icon>
              </template>
              {{ statusText }}
            </n-tag>
          </div>

          <!-- 风险级别 -->
          <n-alert
            :type="riskAlertType"
            class="mb-4"
          >
            <template #header>
              <div class="flex items-center">
                <n-icon class="mr-2">
                  <component :is="riskIconComponent" />
                </n-icon>
                风险级别：{{ result?.risk_level }}
              </div>
            </template>
          </n-alert>

          <!-- 错误信息 -->
          <n-alert
            v-if="result?.error"
            type="error"
            class="mb-4"
          >
            <template #header>
              <div class="flex items-center">
                <n-icon class="mr-2">
                  <svg viewBox="0 0 24 24">
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10s10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5l1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z" fill="currentColor"/>
                  </svg>
                </n-icon>
                错误信息
              </div>
            </template>
            {{ result.error }}
          </n-alert>

          <!-- 检测问题 -->
          <div v-if="result?.issues?.length" class="mb-4">
            <n-alert type="warning">
              <template #header>
                <div class="flex items-center">
                  <n-icon class="mr-2">
                    <svg viewBox="0 0 24 24">
                      <path d="M1 21h22L12 2L1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z" fill="currentColor"/>
                    </svg>
                  </n-icon>
                  检测到的问题
                </div>
              </template>
              <n-list>
                <n-list-item v-for="(issue, index) in result.issues" :key="index">
                  <div class="flex items-start">
                    <n-icon class="mr-2 mt-1 text-red-500">
                      <svg viewBox="0 0 24 24">
                        <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10s10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5l1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z" fill="currentColor"/>
                      </svg>
                    </n-icon>
                    {{ issue }}
                  </div>
                </n-list-item>
              </n-list>
            </n-alert>
          </div>

          <!-- 建议操作 -->
          <div v-if="result?.recommendations?.length" class="mb-4">
            <n-alert type="info">
              <template #header>
                <div class="flex items-center">
                  <n-icon class="mr-2">
                    <svg viewBox="0 0 24 24">
                      <path d="M9 21c0 .5.4 1 1 1h4c.6 0 1-.5 1-1v-1H9v1zm3-19C8.1 2 5 5.1 5 9c0 2.4 1.2 4.5 3 5.7V17c0 .5.4 1 1 1h6c.6 0 1-.5 1-1v-2.3c1.8-1.3 3-3.4 3-5.7c0-3.9-3.1-7-7-7z" fill="currentColor"/>
                    </svg>
                  </n-icon>
                  建议操作
                </div>
              </template>
              <n-list>
                <n-list-item v-for="(recommendation, index) in result.recommendations" :key="index">
                  <div class="flex items-start">
                    <n-icon class="mr-2 mt-1 text-green-500">
                      <svg viewBox="0 0 24 24">
                        <path d="M9 16.17L4.83 12l-1.42 1.41L9 19L21 7l-1.41-1.41z" fill="currentColor"/>
                      </svg>
                    </n-icon>
                    {{ recommendation }}
                  </div>
                </n-list-item>
              </n-list>
            </n-alert>
          </div>

          <!-- 执行示例 -->
          <div v-if="result?.lossy_status === 'Lossy'">
            <n-alert type="warning">
              <template #header>
                <div class="flex items-center">
                  <n-icon class="mr-2">
                    <svg viewBox="0 0 24 24">
                      <path d="M9.4 16.6L4.8 12l4.6-4.6L8 6l-6 6l6 6l1.4-1.4zm5.2 0l4.6-4.6l-4.6-4.6L16 6l6 6l-6 6l-1.4-1.4z" fill="currentColor"/>
                    </svg>
                  </n-icon>
                  执行示例
                </div>
              </template>
              <n-code
                language="sql"
                :code="generateExecutionExample()"
                show-line-numbers
              />
            </n-alert>
          </div>
        </n-card>
      </n-gi>
    </n-grid>
  </AppPage>
</template>

<script setup lang="ts">
import { ref, computed, h } from 'vue'
import { ddlPrecheckAPI, type DDLPrecheckRequest, type DDLPrecheckResponse } from '@/api/ddl-precheck'
import type { FormInst, FormRules } from 'naive-ui'

// 表单引用
const formRef = ref<FormInst>()

// 表单数据
const formData = ref({
  sql: '',
  collationEnabled: true
})

// 表单验证规则
const rules: FormRules = {
  sql: {
    required: true,
    message: '请输入 SQL 语句',
    trigger: 'blur'
  }
}

// 响应式数据
const loading = ref(false)
const result = ref<DDLPrecheckResponse | null>(null)
const showResults = ref(false)

// 状态图标组件
const CheckmarkIcon = () => h('svg', { viewBox: '0 0 24 24' }, [
  h('path', { d: 'M9 16.17L4.83 12l-1.42 1.41L9 19L21 7l-1.41-1.41z', fill: 'currentColor' })
])

const WarningIcon = () => h('svg', { viewBox: '0 0 24 24' }, [
  h('path', { d: 'M1 21h22L12 2L1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z', fill: 'currentColor' })
])

const HelpIcon = () => h('svg', { viewBox: '0 0 24 24' }, [
  h('path', { d: 'M11 18h2v-2h-2v2zm1-16C6.48 2 2 6.48 2 12s4.48 10 10 10s10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8s8 3.59 8 8s-3.59 8-8 8zm0-14c-2.21 0-4 1.79-4 4h2c0-1.1.9-2 2-2s2 .9 2 2c0 2-3 1.75-3 5h2c0-2.25 3-2.5 3-5c0-2.21-1.79-4-4-4z', fill: 'currentColor' })
])

const ShieldIcon = () => h('svg', { viewBox: '0 0 24 24' }, [
  h('path', { d: 'M12 1L3 5v6c0 5.55 3.84 10.74 9 12c5.16-1.26 9-6.45 9-12V5l-9-4z', fill: 'currentColor' })
])

const WarningHexIcon = () => h('svg', { viewBox: '0 0 24 24' }, [
  h('path', { d: 'M23 12l-2.44-2.44l.34-3.46l-3.46-.34L15 2l-3 1.56L9 2L6.56 5.76l-3.46.34l.34 3.46L1 12l2.44 2.44l-.34 3.46l3.46.34L9 22l3-1.56L15 22l2.44-3.76l3.46-.34l-.34-3.46L23 12zm-7 5h-4v-2h4v2zm0-4h-4V7h4v6z', fill: 'currentColor' })
])

// 计算属性
const statusTagType = computed(() => {
  if (!result.value) return 'default'
  
  switch (result.value.lossy_status) {
    case 'Safe':
      return 'success'
    case 'Lossy':
      return 'error'
    case 'Unknown':
      return 'warning'
    default:
      return 'default'
  }
})

const statusIconComponent = computed(() => {
  if (!result.value) return null
  
  switch (result.value.lossy_status) {
    case 'Safe':
      return CheckmarkIcon
    case 'Lossy':
      return WarningIcon
    case 'Unknown':
      return HelpIcon
    default:
      return null
  }
})

const statusText = computed(() => {
  if (!result.value) return ''
  
  switch (result.value.lossy_status) {
    case 'Safe':
      return '安全操作'
    case 'Lossy':
      return '有损操作'
    case 'Unknown':
      return '状态未知'
    default:
      return result.value.lossy_status
  }
})

const riskAlertType = computed(() => {
  if (!result.value) return 'info'
  
  switch (result.value.risk_level) {
    case 'Safe':
      return 'success'
    case 'High':
      return 'error'
    default:
      return 'info'
  }
})

const riskIconComponent = computed(() => {
  if (!result.value) return ShieldIcon
  
  switch (result.value.risk_level) {
    case 'Safe':
      return ShieldIcon
    case 'High':
      return WarningHexIcon
    default:
      return ShieldIcon
  }
})

// 方法
const runPrecheck = async () => {
  if (!formData.value.sql.trim()) {
    window.$message?.warning('请输入 SQL 语句')
    return
  }

  // 表单验证
  try {
    await formRef.value?.validate()
  } catch {
    return
  }

  loading.value = true
  showResults.value = false

  try {
    const requestData: DDLPrecheckRequest = {
      sql: formData.value.sql.trim(),
      collation_enabled: formData.value.collationEnabled
    }

    const response = await ddlPrecheckAPI.precheck(requestData)
    result.value = response.data
    showResults.value = true

    // 根据结果显示不同的消息
    if (result.value.lossy_status === 'Safe') {
      window.$message?.success('DDL 检查完成：操作安全')
    } else if (result.value.lossy_status === 'Lossy') {
      window.$message?.warning('DDL 检查完成：检测到有损操作')
    } else {
      window.$message?.info('DDL 检查完成：状态未知，请检查语法')
    }
  } catch (error: any) {
    window.$message?.error(`检查失败: ${error.response?.data?.message || error.message}`)
    console.error('DDL precheck error:', error)
  } finally {
    loading.value = false
  }
}

const clearResults = () => {
  showResults.value = false
  result.value = null
}

const clearAll = () => {
  formData.value.sql = ''
  formData.value.collationEnabled = true
  clearResults()
}

const extractTableNames = (sql: string): string[] => {
  // 简单的表名提取，实际项目中可能需要更复杂的解析
  const tableRegex = /(?:CREATE\s+TABLE|ALTER\s+TABLE)\s+(\w+\.?\w+)/gi
  const matches = sql.matchAll(tableRegex)
  const tables = new Set<string>()
  
  for (const match of matches) {
    if (match[1]) {
      tables.add(match[1])
    }
  }
  
  return Array.from(tables)
}

const generateExecutionExample = () => {
  const tableNames = extractTableNames(formData.value.sql)
  return `-- 1. 执行你的 DDL 语句
${formData.value.sql.trim()}

-- 2. 立即执行 ANALYZE TABLE 以更新统计信息
ANALYZE TABLE ${tableNames.join(', ')};`
}
</script>

<style scoped>
/* 由于使用了 Naive UI 组件，大部分样式已经内置，这里只需要少量自定义样式 */
</style>
