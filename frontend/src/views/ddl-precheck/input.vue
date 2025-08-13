
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
              <Icon :icon="item.icon" :class="item.iconClass" />
              {{ item.label }}
              <span class="editor-tip" v-if="item.tip" v-html="item.tip"></span>
            </span>
          </template>
          <div
            class="ddl-input-textarea textarea-resizable codemirror-textarea"
            :style="{
              minHeight: (item.rows * 24 + 24) + 'px',
              resize: 'vertical',
              width: '100%',
              overflow: 'visible',
              padding: 0,
              position: 'relative',
              border: '1px solid #d9d9d9',
              borderRadius: '6px',
              background: '#fff',
            }"
          >
<Codemirror
  :model-value="formData[item.key]"
  @update:modelValue="val => { formData[item.key] = val; clearResults(); }"
  :placeholder="item.placeholder"
  :extensions="[sql({ dialect: MySQL })]"
  :disabled="loading"
  :style="{
    minHeight: (item.rows * 24 + 24) + 'px',
    resize: 'vertical',
    width: '100%',
    fontSize: '15px',
    lineHeight: 1.6,
    background: 'transparent',
    border: 'none',
    boxShadow: 'none',
    padding: '8px 12px',
    color: '#222',
    boxSizing: 'border-box',
  }"
/>
          </div>
        </n-form-item>
        <n-form-item class="checkbox-item">
          <template #label>
            <span class="form-label-inline">
              <Icon icon="mdi:sort-variant" class="icon-collation" />
              {{ t('ddlCheck.collationLabel') }}
              <span class="editor-tip">{{ t('ddlCheck.collationTip') }}</span>
            </span>
          </template>
          <n-switch v-model:value="formData.collationEnabled">
            <template #checked>
              <Icon icon="mdi:sort-variant" class="checkbox-icon" /> {{ t('ddlCheck.collationOn') }}
            </template>
            <template #unchecked>
              <Icon icon="mdi:sort-variant" class="checkbox-icon" /> {{ t('ddlCheck.collationOff') }}
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
                {{ loading ? t('ddlCheck.checking') : t('ddlCheck.startCheck') }}
              </n-button>
              <n-button @click="clearAll">
                <template #icon>
                  <n-icon><Icon icon="mdi:close" /></n-icon>
                </template>
                {{ t('common.clear') }}
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
import { Codemirror } from 'vue-codemirror'
import { sql, MySQL } from '@codemirror/lang-sql'
import { ddlPrecheckAPI, type DDLPrecheckRequest } from '@/api/ddl-precheck'
import { useDdlPrecheckStore } from '@/store/modules/ddlPrecheck'
import { useI18n } from 'vue-i18n'
const { t } = useI18n()


const emit = defineEmits(['result'])
const formRef = ref()
const loading = ref(false)
const store = useDdlPrecheckStore()
const formData = store

const inputItems = computed(() => [
  {
    key: 'createDatabase',
    label: t('ddlCheck.createDatabaseLabel'),
    icon: 'mdi:database-plus',
    iconClass: 'icon-base icon-db',
    tip: t('ddlCheck.createDatabaseTip'),
    placeholder: t('ddlCheck.createDatabasePlaceholder'),
    rows: 4,
    validator: (value: string) => {
      if (!value.trim()) return true
      const trimmedValue = value.trim().toUpperCase()
      if (!trimmedValue.startsWith('CREATE DATABASE')) {
        return new Error(t('ddlCheck.createDatabaseErrorStart'))
      }
      if (!value.trim().endsWith(';')) {
        return new Error(t('ddlCheck.sqlEndWithSemicolon'))
      }
      if (countSQLStatements(value.trim()) > 1) {
        return new Error(t('ddlCheck.onlyOneSQL'))
      }
      return true
    }
  },
  {
    key: 'createTable',
    label: t('ddlCheck.createTableLabel'),
    icon: 'mdi:table-plus',
    iconClass: 'icon-base icon-table',
    tip: t('ddlCheck.createTableTip'),
    placeholder: t('ddlCheck.createTablePlaceholder'),
    rows: 8,
    validator: (value: string) => {
      if (!value.trim()) return true
      const trimmedValue = value.trim().toUpperCase()
      if (!trimmedValue.startsWith('CREATE TABLE')) {
        return new Error(t('ddlCheck.createTableErrorStart'))
      }
      if (!value.trim().endsWith(';')) {
        return new Error(t('ddlCheck.sqlEndWithSemicolon'))
      }
      if (!value.includes('(') || !value.includes(')')) {
        return new Error(t('ddlCheck.createTableErrorColumns'))
      }
      if (countSQLStatements(value.trim()) > 1) {
        return new Error(t('ddlCheck.onlyOneSQL'))
      }
      return true
    }
  },
  {
    key: 'alterTable',
    label: t('ddlCheck.alterTableLabel'),
    icon: 'mdi:table-edit',
    iconClass: 'icon-base icon-alter',
    tip: t('ddlCheck.alterTableTip'),
    placeholder: t('ddlCheck.alterTablePlaceholder'),
    rows: 6,
    validator: (value: string) => {
      if (!value.trim()) return true
      const trimmedValue = value.trim().toUpperCase()
      if (!trimmedValue.startsWith('ALTER TABLE')) {
        return new Error(t('ddlCheck.alterTableErrorStart'))
      }
      if (!value.trim().endsWith(';')) {
        return new Error(t('ddlCheck.sqlEndWithSemicolon'))
      }
      if (countSQLStatements(value.trim()) > 1) {
        return new Error(t('ddlCheck.onlyOneSQL'))
      }
      return true
    }
  }
])

const rules = Object.fromEntries(
  inputItems.value.map(item => [
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
  window.$message.success(t('ddlCheck.cleared'))
}

function validateSQLStatements() {
  const configs = [
    {
      key: 'createDatabase',
      label: t('ddlCheck.createDatabaseLabel'),
      start: 'CREATE DATABASE',
      extra: null
    },
    {
      key: 'createTable',
      label: t('ddlCheck.createTableLabel'),
      start: 'CREATE TABLE',
      extra: (val: string) => (!val.includes('(') || !val.includes(')')) ? t('ddlCheck.createTableErrorColumns') : null
    },
    {
      key: 'alterTable',
      label: t('ddlCheck.alterTableLabel'),
      start: 'ALTER TABLE',
      extra: null
    }
  ]
  const errors: string[] = []
  for (const cfg of configs) {
    const val = formData[cfg.key]?.trim()
    if (!val) continue
    const upper = val.toUpperCase()
    if (!upper.startsWith(cfg.start)) errors.push(t('ddlCheck.errorStart', { label: cfg.label, start: cfg.start }))
    if (!val.endsWith(';')) errors.push(t('ddlCheck.errorEndWithSemicolon', { label: cfg.label }))
    if (countSQLStatements(val) > 1) errors.push(t('ddlCheck.errorOnlyOneSQL', { label: cfg.label }))
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
    window.$message.warning(t('ddlCheck.inputAtLeastOneSQL'))
    return
  }
  const validationErrors = validateSQLStatements()
  if (validationErrors.length > 0) {
    window.$message.error(t('ddlCheck.sqlFormatError', { msg: validationErrors[0] }))
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
    window.$message.success(t('ddlCheck.checkSuccess'))
  } catch (error: any) {
    const checkDuration = Date.now() - checkStartTime
    emit('result', { error: error.response?.data?.message || error.message, sql: sqlToCheck, checkDuration })
    window.$message.error(t('ddlCheck.checkFailed', { msg: error.response?.data?.message || error.message }))
  } finally {
    loading.value = false
  }
}

</script>



<style scoped>
.cm-sql-placeholder {
  color: #bfbfbf !important;
  font-style: italic;
  opacity: 1 !important;
}
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
  background: transparent;
  border: none;
  resize: none;
  min-height: 60px;
}

/* Codemirror main surface border and background */
.codemirror-textarea {
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  background: #fff;
}

.codemirror-textarea :deep(.cm-editor) {
  background: transparent !important;
  border: none !important;
  box-shadow: none !important;
  outline: none !important;
}

/* 去除 Codemirror 当前行的蓝色高亮背景 */
.codemirror-textarea :deep(.cm-activeLine) {
  background: transparent !important;
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