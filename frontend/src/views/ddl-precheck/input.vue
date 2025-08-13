
<template>
  <n-card class="query-editor-card ddl-precheck-card" content-style="padding: 0; height: 100%; display: flex; flex-direction: column;" embedded>
    <div class="ddl-precheck-body">
      <n-form ref="formRef" :model="formData" :rules="rules" label-placement="top" class="ddl-precheck-form">
        <n-form-item
          v-for="item in inputItems"
          :key="item.key"
          :path="item.key"
          style="width:100%;"
        >
          <template #label>
            <span class="form-label-inline">
              <Icon :icon="item.icon" :class="item.iconClass" />{{ item.label }}
              <span class="editor-tip" v-if="item.tip" v-html="item.tip"></span>
            </span>
          </template>
          <n-input
            v-model:value="formData[item.key]"
            type="textarea"
            :rows="item.rows"
            :disabled="loading"
            :placeholder="item.placeholder"
            @input="clearResults"
            class="ddl-input-textarea"
          />
        </n-form-item>
        <n-form-item class="checkbox-item">
          <template #label>
            <span class="form-label-inline">
              <Icon icon="mdi:sort-variant" class="icon-collation" />排序规则
              <span class="editor-tip">默认启用新的排序规则</span>
            </span>
          </template>
          <n-switch v-model:value="formData.collationEnabled">
            <template #checked>
              <Icon icon="mdi:sort-variant" class="checkbox-icon" /> 启用
            </template>
            <template #unchecked>
              <Icon icon="mdi:sort-variant" class="checkbox-icon" /> 关闭
            </template>
          </n-switch>
        </n-form-item>
        <n-form-item class="action-bar-item" style="margin-top: 18px;">
          <div class="editor-action-bar center-bar">
            <n-button-group>
              <n-button type="primary" :loading="loading" :disabled="!hasAnySQLInput" @click="runPrecheck">
                <template #icon>
                  <n-icon><Icon icon="mdi:play" /></n-icon>
                </template>
                {{ loading ? '检查中...' : '开始检查' }}
              </n-button>
              <n-button @click="clearAll">
                <template #icon>
                  <n-icon><Icon icon="mdi:close" /></n-icon>
                </template>
                清空
              </n-button>
            </n-button-group>
          </div>
        </n-form-item>
      </n-form>
    </div>
  </n-card>
</template>



<script setup lang="ts">
import { ref, computed } from 'vue'
import { Icon } from '@iconify/vue'

import { ddlPrecheckAPI, type DDLPrecheckRequest } from '@/api/ddl-precheck'
import { useDdlPrecheckStore } from '@/store/modules/ddlPrecheck'

const emit = defineEmits(['result'])

const formRef = ref()
const loading = ref(false)
const store = useDdlPrecheckStore()
const formData = store

const inputItems = [
  {
    key: 'createDatabase',
    label: '建库语句',
    icon: 'mdi:database-plus',
    iconClass: 'icon-base icon-db',
    tip: '必须以 <b>CREATE DATABASE</b> 开头，必须以分号结尾，只能输入一个语句',
    placeholder: '例如：CREATE DATABASE testdb DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;',
    rows: 4,
    validator: (value: string) => {
      if (!value.trim()) return true
      const trimmedValue = value.trim().toUpperCase()
      if (!trimmedValue.startsWith('CREATE DATABASE')) {
        return new Error('建库语句必须以 CREATE DATABASE 开头')
      }
      if (!value.trim().endsWith(';')) {
        return new Error('SQL语句必须以分号(;)结尾')
      }
      if (countSQLStatements(value.trim()) > 1) {
        return new Error('每个输入框只能输入一个SQL语句')
      }
      return true
    }
  },
  {
    key: 'createTable',
    label: '建表语句',
    icon: 'mdi:table-plus',
    iconClass: 'icon-base icon-table',
    tip: '必须以 <b>CREATE TABLE</b> 开头，必须包含列定义（使用括号包围），必须以分号结尾，只能输入一个语句',
    placeholder: '例如：CREATE TABLE testdb.users (id INT PRIMARY KEY AUTO_INCREMENT, name VARCHAR(50) NOT NULL);',
    rows: 8,
    validator: (value: string) => {
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
      if (countSQLStatements(value.trim()) > 1) {
        return new Error('每个输入框只能输入一个SQL语句')
      }
      return true
    }
  },
  {
    key: 'alterTable',
    label: '修改表语句',
    icon: 'mdi:table-edit',
    iconClass: 'icon-base icon-alter',
    tip: '必须以 <b>ALTER TABLE</b> 开头，必须以分号结尾，只能输入一个语句',
    placeholder: '例如：ALTER TABLE testdb.users MODIFY COLUMN name VARCHAR(100) NOT NULL;',
    rows: 6,
    validator: (value: string) => {
      if (!value.trim()) return true
      const trimmedValue = value.trim().toUpperCase()
      if (!trimmedValue.startsWith('ALTER TABLE')) {
        return new Error('修改表语句必须以 ALTER TABLE 开头')
      }
      if (!value.trim().endsWith(';')) {
        return new Error('SQL语句必须以分号(;)结尾')
      }
      if (countSQLStatements(value.trim()) > 1) {
        return new Error('每个输入框只能输入一个SQL语句')
      }
      return true
    }
  }
]

const rules = Object.fromEntries(
  inputItems.map(item => [
    item.key,
    {
      required: false,
      validator: (rule: any, value: string) => item.validator(value),
      trigger: 'blur'
    }
  ])
)

const hasAnySQLInput = computed(() => {
  return formData.createDatabase.trim() || formData.createTable.trim() || formData.alterTable.trim()
})

function countSQLStatements(sql: string): number {
  let count = 0
  let inSingleQuote = false
  let inDoubleQuote = false
  let inBacktick = false
  for (let i = 0; i < sql.length; i++) {
    const char = sql[i]
    const prevChar = i > 0 ? sql[i - 1] : ''
    if (prevChar === '\\') continue
    if (char === "'" && !inDoubleQuote && !inBacktick) inSingleQuote = !inSingleQuote
    else if (char === '"' && !inSingleQuote && !inBacktick) inDoubleQuote = !inDoubleQuote
    else if (char === '`' && !inSingleQuote && !inDoubleQuote) inBacktick = !inBacktick
    if (char === ';' && !inSingleQuote && !inDoubleQuote && !inBacktick) {
      const remaining = sql.substring(i + 1).trim()
      if (remaining.length > 0) count++
    }
  }
  return count + 1
}

function clearResults() {
  emit('result', null)
}
function clearAll() {
  store.clearForm()
  clearResults()
  window.$message.success('已清空所有内容')
}

function validateSQLStatements() {
  const configs = [
    {
      key: 'createDatabase',
      label: '建库语句',
      start: 'CREATE DATABASE',
      extra: null
    },
    {
      key: 'createTable',
      label: '建表语句',
      start: 'CREATE TABLE',
      extra: (val: string) => (!val.includes('(') || !val.includes(')')) ? '建表语句必须包含列定义，使用括号包围' : null
    },
    {
      key: 'alterTable',
      label: '修改表语句',
      start: 'ALTER TABLE',
      extra: null
    }
  ]
  const errors: string[] = []
  for (const cfg of configs) {
    const val = formData[cfg.key]?.trim()
    if (!val) continue
    const upper = val.toUpperCase()
    if (!upper.startsWith(cfg.start)) errors.push(`${cfg.label}必须以 ${cfg.start} 开头`)
    if (!val.endsWith(';')) errors.push(`${cfg.label}必须以分号结尾`)
    if (countSQLStatements(val) > 1) errors.push(`${cfg.label}输入框只能包含一个SQL语句`)
    if (cfg.extra) {
      const extraMsg = cfg.extra(val)
      if (extraMsg) errors.push(extraMsg)
    }
  }
  return errors
}

async function runPrecheck() {
  const sqls = [
    formData.createDatabase.trim(),
    formData.createTable.trim(),
    formData.alterTable.trim()
  ].filter(sql => sql.length > 0)
  const sqlToCheck = sqls.join('\n\n')
  if (!sqlToCheck) {
    window.$message.warning('请至少输入一个 SQL 语句')
    return
  }
  const validationErrors = validateSQLStatements()
  if (validationErrors.length > 0) {
    window.$message.error(`SQL 格式错误：${validationErrors[0]}`)
    return
  }
  try {
    await formRef.value?.validate()
  } catch {
    return
  }
  loading.value = true
  const checkStartTime = Date.now()
  try {
    const requestData: DDLPrecheckRequest = {
      sql: sqlToCheck,
      collation_enabled: formData.collationEnabled
    }
    const response = await ddlPrecheckAPI.precheck(requestData)
    const checkDuration = Date.now() - checkStartTime
    const resultObj = {
      ...response.data,
      sql: sqlToCheck,
      checkDuration
    }
    emit('result', resultObj)
    window.$message.success('DDL 检查完成')
  } catch (error: any) {
    const checkDuration = Date.now() - checkStartTime
    emit('result', { error: error.response?.data?.message || error.message, sql: sqlToCheck, checkDuration })
    window.$message.error(`检查失败: ${error.response?.data?.message || error.message}`)
  } finally {
    loading.value = false
  }
}

</script>



<style scoped>

/* 对齐 sql editor 风格的 DDL precheck 样式 */
.form-label-inline {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  font-size: 15px;
  font-weight: 500;
}
.form-label-inline .editor-tip {
  margin-left: 10px;
  font-size: 12px;
  color: #888;
  font-weight: normal;
}
.query-editor-card.ddl-precheck-card {
  display: flex;
  flex-direction: column;
  width: 100%;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.04);
  padding-bottom: 0;
  flex: 1 1 auto;
  min-height: 0;
  height: 100%;
}
.editor-toolbar {
  display: flex;
  align-items: center;
  padding: 10px 16px;
  border-bottom: 1px solid #e0e6ed;
  background: #fff;
}
.toolbar-left {
  display: flex;
  align-items: center;
  gap: 8px;
}
.editor-title {
  font-weight: 600;
  font-size: 16px;
  margin-right: 8px;
}
.editor-tip {
  color: #888;
  font-size: 12px;
  margin-left: 8px;
}
.ddl-precheck-body {
  flex: 1 1 auto;
  display: flex;
  flex-direction: column;
  padding: 16px 16px 0 16px;
  background: #fff;
  min-height: 0;
  height: 100%;
  overflow: hidden;
}
.ddl-precheck-form {
  width: 100%;
  background: #fff;
  flex: 1 1 auto;
  min-height: 0;
  overflow-y: auto;
  overflow-x: hidden;
}

/* Unified icon style, use color modifier classes for each icon type */
.icon-base {
  font-size: 18px;
  margin-right: 4px;
}
.icon-db { color: #409eff; }
.icon-table { color: #67c23a; }
.icon-alter { color: #e6a23c; }
.icon-collation { color: #36cfc9; }

/* Unified textarea style */
.ddl-input-textarea :deep(textarea) {
  border-radius: 6px;
  background: #fff;
  resize: none;
  min-height: 60px;
}

.editor-action-bar {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  background: transparent;
}
.action-bar-item {
  margin-bottom: 0;
  padding-bottom: 0;
}

</style>