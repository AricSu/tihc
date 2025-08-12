<template>
  <div style="min-height: 100vh; display: flex; flex-direction: column;">
    <n-card embedded>
      <n-space align="center" justify="center">
        <n-icon color="#f59e0b" size="40">
          <Icon icon="mdi:database-check" />
        </n-icon>
        <n-space vertical size="small">
          <n-h1 style="margin: 0; font-size: 2rem;">DDL 预检查工具</n-h1>
          <n-text depth="3" style="font-size: 1.1rem;">检查 DDL 语句是否会造成数据丢失，提供安全建议</n-text>
        </n-space>
      </n-space>
    </n-card>
    <n-grid :cols="24" :x-gap="18" responsive="screen" style="flex: 1 1 0%; min-height: 0;">
      <n-gi :span="showResults ? 12 : 24" style="display: flex; flex-direction: column; min-height: 0;">
        <n-card title="SQL 语句输入" embedded style="flex: 1 1 0%; display: flex; flex-direction: column; min-height: 0;">
          <n-scrollbar style="flex: 1 1 0%; min-height: 0;">
            <n-space vertical size="large">
            <n-alert type="info">
              <Icon icon="mdi:information-outline" style="vertical-align: middle; margin-right: 6px;" />
              请输入需要检查的 DDL 语句，支持建库、建表、修改表。每个输入框仅支持一个语句，内容自动保存。
            </n-alert>
            <n-form ref="formRef" :model="formData" :rules="rules">
              <n-form-item label="建库语句" path="createDatabase">
                  <n-input
                    v-model:value="formData.createDatabase"
                    type="textarea"
                    :rows="4"
                    placeholder="例如：CREATE DATABASE testdb DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"
                    :disabled="loading"
                    @input="clearResults"
                  />
                <template #feedback>
                  <n-text depth="3" tag="div" style="font-size: 12px; margin-top: 4px;">
                    <Icon icon="mdi:lightbulb-on-outline" style="vertical-align: middle; margin-right: 4px;" />必须以 CREATE DATABASE 开头，以分号结尾，只能输入一个语句
                  </n-text>
                </template>
              </n-form-item>

              <n-form-item label="建表语句" path="createTable">
                  <n-input
                    v-model:value="formData.createTable"
                    type="textarea"
                    :rows="8"
                    placeholder="例如：CREATE TABLE testdb.users (id INT PRIMARY KEY AUTO_INCREMENT, name VARCHAR(50) NOT NULL);"
                    :disabled="loading"
                    @input="clearResults"
                  />
                <template #feedback>
                  <n-text depth="3" tag="div" style="font-size: 12px; margin-top: 4px;">
                    <Icon icon="mdi:lightbulb-on-outline" style="vertical-align: middle; margin-right: 4px;" />必须以 CREATE TABLE 开头，包含列定义，以分号结尾，只能输入一个语句
                  </n-text>
                </template>
              </n-form-item>

              <n-form-item label="修改表语句" path="alterTable">
                  <n-input
                    v-model:value="formData.alterTable"
                    type="textarea"
                    :rows="6"
                    placeholder="例如：ALTER TABLE testdb.users MODIFY COLUMN name VARCHAR(100) NOT NULL;"
                    :disabled="loading"
                    @input="clearResults"
                  />
                <template #feedback>
                  <n-text depth="3" tag="div" style="font-size: 12px; margin-top: 4px;">
                    <Icon icon="mdi:lightbulb-on-outline" style="vertical-align: middle; margin-right: 4px;" />必须以 ALTER TABLE 开头，以分号结尾，只能输入一个语句
                  </n-text>
                </template>
              </n-form-item>

              <n-form-item>
                <n-checkbox v-model:checked="formData.collationEnabled">
                  <Icon icon="mdi:sort-variant" style="vertical-align: middle; margin-right: 4px;" />启用新的排序规则
                </n-checkbox>
              </n-form-item>

              <n-form-item>
                <n-space justify="center">
                  <n-button
                    type="primary"
                    size="large"
                    :loading="loading"
                    :disabled="!hasAnySQLInput"
                    @click="runPrecheck"
                  >
                    <template #icon>
                      <n-icon><Icon icon="mdi:play" /></n-icon>
                    </template>
                    {{ loading ? '检查中...' : '开始检查' }}
                  </n-button>
                  <n-button size="large" @click="clearAll">
                    <template #icon>
                      <n-icon><Icon icon="mdi:close" /></n-icon>
                    </template>
                    清空
                  </n-button>
                </n-space>
              </n-form-item>
            </n-form>
            </n-space>
          </n-scrollbar>
        </n-card>
      </n-gi>

      <!-- 结果显示区域，右侧 sidebar，移动端自动垂直排列 -->
      <n-gi v-if="showResults" :span="12" style="display: flex; flex-direction: column; min-height: 0; height: 100%;">
        <div style="flex: 1 1 0%; display: flex; flex-direction: column; min-height: 0; height: 100%;">
          <n-card embedded style="flex: 1 1 0%; display: flex; flex-direction: column; min-height: 0; height: 100%; box-shadow: none;">
            <div style="flex: 1 1 0%; min-height: 0; display: flex; flex-direction: column;">
              <n-scrollbar style="flex: 1 1 0%; min-height: 0; height: 100%; overflow: auto;">
                <n-space vertical size="large">
                <!-- 风险等级和主结论 -->
            <n-space align="center" justify="space-between">
              <n-tag :type="statusTagType" size="large" style="font-size: 1.2rem; padding: 8px 16px;">
                <Icon :icon="resultStatus === 'success' ? 'mdi:shield-check' : resultStatus === 'error' ? 'mdi:shield-alert' : 'mdi:shield-question'" style="vertical-align: middle; margin-right: 8px; font-size: 1.5rem;" />
                {{ statusText }}
              </n-tag>
              <n-tag :type="riskAlertType" size="large" style="font-size: 1.1rem;">
                <Icon :icon="riskAlertType === 'success' ? 'mdi:check-circle-outline' : riskAlertType === 'error' ? 'mdi:alert-circle-outline' : 'mdi:help-circle-outline'" style="vertical-align: middle; margin-right: 6px; font-size: 1.2rem;" />
                风险等级: {{ result?.risk_level || 'Unknown' }}
              </n-tag>
            </n-space>

            <!-- 主要结果卡片 -->
            <n-card size="large">
              <n-space align="center" justify="center">
                <n-icon size="48" color="#18a058">
                  <Icon :icon="resultStatus === 'success' ? 'mdi:check-circle' : resultStatus === 'error' ? 'mdi:alert-circle' : 'mdi:help-circle'" style="font-size: 2.5rem;" />
                </n-icon>
                <n-h2 style="margin: 0; font-size: 2rem;">{{ statusTitle }}</n-h2>
              </n-space>
              <n-text>{{ statusDescription }}</n-text>
            </n-card>

            <!-- 统计信息分组 -->
            <n-card size="small">
              <n-grid :cols="2" :x-gap="12">
                <n-gi>
                  <n-statistic label="已检查SQL语句" :value="sqlStatementsCount" suffix="条" />
                </n-gi>
                <n-gi>
                  <n-statistic label="耗时" :value="checkDuration" suffix="ms" />
                </n-gi>
              </n-grid>
            </n-card>

            <!-- 错误信息 -->
            <n-card v-if="result?.error" size="small">
              <n-alert type="error">
                <Icon icon="mdi:alert" style="vertical-align: middle; margin-right: 6px;" />
                {{ result.error }}
              </n-alert>
            </n-card>

            <!-- 检测问题、建议等分组展示，折叠面板 -->
            <n-collapse>
              <n-collapse-item v-if="result?.issues?.length" title="检测到的问题" :name="'issues'">
                <n-list>
                  <n-list-item v-for="(issue, index) in result.issues" :key="index">
                    <n-thing>
                      <template #avatar>
                        <n-avatar color="#d03050">
                          <n-icon><Icon icon="mdi:alert" /></n-icon>
                        </n-avatar>
                      </template>
                      <n-text>{{ issue }}</n-text>
                    </n-thing>
                  </n-list-item>
                </n-list>
              </n-collapse-item>
              <n-collapse-item v-if="result?.recommendations?.length" title="建议操作" :name="'recommendations'">
                <n-list>
                  <n-list-item v-for="(recommendation, index) in result.recommendations" :key="index">
                    <n-thing>
                      <template #avatar>
                        <n-avatar color="#18a058">
                          <n-icon><Icon icon="mdi:check" /></n-icon>
                        </n-avatar>
                      </template>
                      <n-text>{{ recommendation }}</n-text>
                    </n-thing>
                  </n-list-item>
                </n-list>
              </n-collapse-item>
            </n-collapse>

            <!-- 执行示例 -->
            <n-card v-if="result?.lossy_status === 'Lossy'" size="small">
              <template #header-extra>
                <n-button size="small" @click="copyExecutionExample">
                  <template #icon>
                    <n-icon><Icon icon="mdi:content-copy" /></n-icon>
                  </template>
                  复制代码
                </n-button>
              </template>
              <n-code language="sql" :code="generateExecutionExample()" show-line-numbers />
            </n-card>
            </n-space>
          </n-scrollbar>
            </div>
        </n-card>
      </div>
      </n-gi>
    </n-grid>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, h, watch, onMounted } from 'vue'
import { Icon } from '@iconify/vue'
import { ddlPrecheckAPI, type DDLPrecheckRequest, type DDLPrecheckResponse } from '@/api/ddl-precheck'
import type { FormInst, FormRules } from 'naive-ui'

// 图标组件
const PlayIcon = () => h('svg', { viewBox: '0 0 24 24' }, [
  h('path', { d: 'M8 5v14l11-7z', fill: 'currentColor' })
])

const ClearIcon = () => h('svg', { viewBox: '0 0 24 24' }, [
  h('path', { d: 'M19 6.41L17.59 5L12 10.59L6.41 5L5 6.41L10.59 12L5 17.59L6.41 19L12 13.41L17.59 19L19 17.59L13.41 12z', fill: 'currentColor' })
])

const CheckmarkIcon = () => h('svg', { viewBox: '0 0 24 24' }, [
  h('path', { d: 'M9 16.17L4.83 12l-1.42 1.41L9 19L21 7l-1.41-1.41z', fill: 'currentColor' })
])

const WarningIcon = () => h('svg', { viewBox: '0 0 24 24' }, [
  h('path', { d: 'M1 21h22L12 2L1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z', fill: 'currentColor' })
])

const CopyIcon = () => h('svg', { viewBox: '0 0 24 24' }, [
  h('path', { d: 'M16 1H4C2.9 1 2 1.9 2 3v14h2V3h12V1zm3 4H8C6.9 5 6 5.9 6 7v14c0 1.1.9 2 2 2h11c1.1 0 2-.9 2-2V7c0-1.1-.9-2-2-2zm0 16H8V7h11v14z', fill: 'currentColor' })
])

// 响应式数据
const formRef = ref<FormInst>()
const loading = ref(false)
const result = ref<DDLPrecheckResponse | null>(null)
const showResults = ref(false)
const checkStartTime = ref<number>(0)
const checkDuration = ref<number>(0)

// 本地存储的键名
const STORAGE_KEY = 'ddl-precheck-form-data'

// 从本地存储加载数据
const loadFromLocalStorage = () => {
  try {
    const saved = localStorage.getItem(STORAGE_KEY)
    if (saved) {
      const data = JSON.parse(saved)
      return {
        createDatabase: data.createDatabase || '',
        createTable: data.createTable || '',
        alterTable: data.alterTable || '',
        collationEnabled: data.collationEnabled !== undefined ? data.collationEnabled : true
      }
    }
  } catch (error) {
    console.warn('Failed to load form data from localStorage:', error)
  }
  return {
    createDatabase: '',
    createTable: '',
    alterTable: '',
    collationEnabled: true
  }
}

// 保存到本地存储
const saveToLocalStorage = (data: any) => {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(data))
  } catch (error) {
    console.warn('Failed to save form data to localStorage:', error)
  }
}

const formData = ref(loadFromLocalStorage())

// 智能检测SQL语句数量，忽略字符串内的分号
const countSQLStatements = (sql: string): number => {
  let count = 0
  let inSingleQuote = false
  let inDoubleQuote = false
  let inBacktick = false
  
  for (let i = 0; i < sql.length; i++) {
    const char = sql[i]
    const prevChar = i > 0 ? sql[i - 1] : ''
    
    // 处理转义字符
    if (prevChar === '\\') {
      continue
    }
    
    // 处理引号状态
    if (char === "'" && !inDoubleQuote && !inBacktick) {
      inSingleQuote = !inSingleQuote
    } else if (char === '"' && !inSingleQuote && !inBacktick) {
      inDoubleQuote = !inDoubleQuote
    } else if (char === '`' && !inSingleQuote && !inDoubleQuote) {
      inBacktick = !inBacktick
    }
    
    // 如果不在引号内，检查分号
    if (char === ';' && !inSingleQuote && !inDoubleQuote && !inBacktick) {
      // 检查分号后是否还有非空白字符
      const remaining = sql.substring(i + 1).trim()
      if (remaining.length > 0) {
        count++
      }
    }
  }
  
  return count + 1 // 至少有一个语句
}

// 表单验证规则
const rules: FormRules = {
  createDatabase: {
    required: false,
    validator: (rule, value: string) => {
      if (!value.trim()) return true
      const trimmedValue = value.trim().toUpperCase()
      if (!trimmedValue.startsWith('CREATE DATABASE')) {
        return new Error('建库语句必须以 CREATE DATABASE 开头')
      }
      if (!value.trim().endsWith(';')) {
        return new Error('SQL语句必须以分号(;)结尾')
      }
      // 智能检查是否只有一个语句
      if (countSQLStatements(value.trim()) > 1) {
        return new Error('每个输入框只能输入一个SQL语句')
      }
      return true
    },
    trigger: 'blur'
  },
  createTable: {
    required: false,
    validator: (rule, value: string) => {
      if (!value.trim()) return true
      const trimmedValue = value.trim().toUpperCase()
      if (!trimmedValue.startsWith('CREATE TABLE')) {
        return new Error('建表语句必须以 CREATE TABLE 开头')
      }
      if (!value.trim().endsWith(';')) {
        return new Error('SQL语句必须以分号(;)结尾')
      }
      if (!value.includes('(') || !value.includes(')')) {
        return new Error('建表语句必须包含列定义，使用括号包围')
      }
      // 智能检查是否只有一个语句
      if (countSQLStatements(value.trim()) > 1) {
        return new Error('每个输入框只能输入一个SQL语句')
      }
      return true
    },
    trigger: 'blur'
  },
  alterTable: {
    required: false,
    validator: (rule, value: string) => {
      if (!value.trim()) return true
      const trimmedValue = value.trim().toUpperCase()
      if (!trimmedValue.startsWith('ALTER TABLE')) {
        return new Error('修改表语句必须以 ALTER TABLE 开头')
      }
      if (!value.trim().endsWith(';')) {
        return new Error('SQL语句必须以分号(;)结尾')
      }
      // 智能检查是否只有一个语句
      if (countSQLStatements(value.trim()) > 1) {
        return new Error('每个输入框只能输入一个SQL语句')
      }
      return true
    },
    trigger: 'blur'
  }
}

// 计算属性
const combinedSQL = computed(() => {
  const sqls = [
    formData.value.createDatabase.trim(),
    formData.value.createTable.trim(),
    formData.value.alterTable.trim()
  ].filter(sql => sql.length > 0)
  return sqls.join('\n\n')
})

const hasAnySQLInput = computed(() => {
  return formData.value.createDatabase.trim() || formData.value.createTable.trim() || formData.value.alterTable.trim()
})

const sqlStatementsCount = computed(() => {
  const sqls = [
    formData.value.createDatabase.trim(),
    formData.value.createTable.trim(),
    formData.value.alterTable.trim()
  ].filter(sql => sql.length > 0)
  return sqls.length
})

const resultStatus = computed(() => {
  if (!result.value) return 'info'
  switch (result.value.lossy_status) {
    case 'Safe': return 'success'
    case 'Lossy': return 'error'
    case 'Unknown': return 'warning'
    default: return 'info'
  }
})

const statusTagType = computed(() => {
  if (!result.value) return 'default'
  switch (result.value.lossy_status) {
    case 'Safe': return 'success'
    case 'Lossy': return 'error'
    case 'Unknown': return 'warning'
    default: return 'default'
  }
})

const statusText = computed(() => {
  if (!result.value) return ''
  switch (result.value.lossy_status) {
    case 'Safe': return '安全操作'
    case 'Lossy': return '有损操作'
    case 'Unknown': return '状态未知'
    default: return result.value.lossy_status
  }
})

const statusTitle = computed(() => {
  if (!result.value) return 'DDL 安全检查'
  switch (result.value.lossy_status) {
    case 'Safe': return 'DDL 操作安全'
    case 'Lossy': return '检测到有损操作'
    case 'Unknown': return '操作状态未知'
    default: return 'DDL 检查完成'
  }
})

const statusDescription = computed(() => {
  if (!result.value) return '正在进行安全性分析...'
  switch (result.value.lossy_status) {
    case 'Safe': return '您的DDL操作不会造成数据丢失，可以安全执行'
    case 'Lossy': return '警告：检测到可能导致数据丢失的操作，请谨慎执行'
    case 'Unknown': return '无法确定操作的安全性，建议进一步检查SQL语法'
    default: return '检查已完成，请查看详细结果'
  }
})

const riskAlertType = computed(() => {
  if (!result.value) return 'info'
  switch (result.value.risk_level) {
    case 'Safe': return 'success'
    case 'High': return 'error'
    default: return 'info'
  }
})

// 方法
const validateSQLStatements = () => {
  const errors: string[] = []
  
  if (formData.value.createDatabase.trim()) {
    const createDB = formData.value.createDatabase.trim()
    if (!createDB.toUpperCase().startsWith('CREATE DATABASE')) {
      errors.push('建库语句必须以 CREATE DATABASE 开头')
    }
    if (!createDB.endsWith(';')) {
      errors.push('建库语句必须以分号结尾')
    }
    // 智能检查语句数量
    if (countSQLStatements(createDB) > 1) {
      errors.push('建库语句输入框只能包含一个SQL语句')
    }
  }
  
  if (formData.value.createTable.trim()) {
    const createTbl = formData.value.createTable.trim()
    if (!createTbl.toUpperCase().startsWith('CREATE TABLE')) {
      errors.push('建表语句必须以 CREATE TABLE 开头')
    }
    if (!createTbl.endsWith(';')) {
      errors.push('建表语句必须以分号结尾')
    }
    if (!createTbl.includes('(') || !createTbl.includes(')')) {
      errors.push('建表语句必须包含列定义，使用括号包围')
    }
    // 智能检查语句数量
    if (countSQLStatements(createTbl) > 1) {
      errors.push('建表语句输入框只能包含一个SQL语句')
    }
  }
  
  if (formData.value.alterTable.trim()) {
    const alterTbl = formData.value.alterTable.trim()
    if (!alterTbl.toUpperCase().startsWith('ALTER TABLE')) {
      errors.push('修改表语句必须以 ALTER TABLE 开头')
    }
    if (!alterTbl.endsWith(';')) {
      errors.push('修改表语句必须以分号结尾')
    }
    // 智能检查语句数量
    if (countSQLStatements(alterTbl) > 1) {
      errors.push('修改表语句输入框只能包含一个SQL语句')
    }
  }
  
  return errors
}

const runPrecheck = async () => {
  const sqlToCheck = combinedSQL.value
  if (!sqlToCheck) {
    window.$message?.warning('请至少输入一个 SQL 语句')
    return
  }

  const validationErrors = validateSQLStatements()
  if (validationErrors.length > 0) {
    window.$message?.error(`SQL 格式错误：${validationErrors[0]}`)
    return
  }

  try {
    await formRef.value?.validate()
  } catch {
    return
  }

  loading.value = true
  showResults.value = false
  checkStartTime.value = Date.now()

  try {
    const requestData: DDLPrecheckRequest = {
      sql: sqlToCheck,
      collation_enabled: formData.value.collationEnabled
    }

    const response = await ddlPrecheckAPI.precheck(requestData)
    result.value = response.data
    checkDuration.value = Date.now() - checkStartTime.value
    showResults.value = true

    if (result.value.lossy_status === 'Safe') {
      window.$message?.success('DDL 检查完成：操作安全')
    } else if (result.value.lossy_status === 'Lossy') {
      window.$message?.warning('DDL 检查完成：检测到有损操作')
    } else {
      window.$message?.info('DDL 检查完成：状态未知，请检查语法')
    }
  } catch (error: any) {
    checkDuration.value = Date.now() - checkStartTime.value
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
  formData.value.createDatabase = ''
  formData.value.createTable = ''
  formData.value.alterTable = ''
  formData.value.collationEnabled = true
  clearResults()
  // 清除本地存储
  try {
    localStorage.removeItem(STORAGE_KEY)
    window.$message?.success('已清空所有内容和缓存')
  } catch (error) {
    console.warn('Failed to clear localStorage:', error)
    window.$message?.success('已清空所有内容')
  }
}

const extractTableNames = (sql: string): string[] => {
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
  const sqlToCheck = combinedSQL.value
  const tableNames = extractTableNames(sqlToCheck)
  return `-- 1. 执行你的 DDL 语句
${sqlToCheck}

-- 2. 立即执行 ANALYZE TABLE 以更新统计信息
ANALYZE TABLE ${tableNames.join(', ')};`
}

const copyExecutionExample = async () => {
  try {
    const exampleCode = generateExecutionExample()
    await navigator.clipboard.writeText(exampleCode)
    window.$message?.success('执行示例已复制到剪贴板')
  } catch (error) {
    console.error('复制失败:', error)
    window.$message?.error('复制失败，请手动选择复制')
  }
}

// 监听表单数据变化，自动保存到本地存储
watch(
  () => formData.value,
  (newData) => {
    saveToLocalStorage(newData)
  },
  { deep: true }
)

// 组件挂载时的初始化
onMounted(() => {
  // 如果有缓存的数据，显示提示
  const saved = localStorage.getItem(STORAGE_KEY)
  if (saved) {
    try {
      const data = JSON.parse(saved)
      const hasContent = data.createDatabase || data.createTable || data.alterTable
      if (hasContent) {
        window.$message?.info('已恢复上次保存的输入内容', {
          duration: 3000
        })
      }
    } catch (error) {
      // 忽略解析错误
    }
  }
})
</script>


