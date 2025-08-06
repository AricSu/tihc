<template>
  <n-drawer
    :show="modelValue"
    @update:show="$emit('update:modelValue', $event)"
    :width="400"
    placement="right"
    :show-mask="true"
    :trap-focus="false"
    :block-scroll="false"
  >
<n-drawer-content :title="t('sqlEditor.slowlogTools')" closable>
      <!-- Help -->
      <n-card :title="t('sqlEditor.tips')" size="small" embedded style="margin-top: 12px;margin-bottom: 12px;">
        <n-text depth="3" style="font-size: 11px;">
          <div><strong>{{ t('sqlEditor.howToUse') }}</strong></div>
          <div>• {{ t('sqlEditor.enterLogDir') }}</div>
          <div>• {{ t('sqlEditor.enterPattern') }}</div>
          <div>• {{ t('sqlEditor.examplePatterns') }}</div>
          <div>• {{ t('sqlEditor.largeFilesTip') }}</div>
          <div>• {{ t('sqlEditor.ensureReadAccess') }}</div>
        </n-text>
      </n-card>
      <!-- Parse Form -->
      <n-card :title="t('sqlEditor.parseSlowlogFiles')" size="small" embedded>
        <n-form 
          ref="formRef"
          :model="form"
          :rules="rules"
          label-placement="top"
          size="small"
        >
          <n-form-item :label="t('sqlEditor.logDir')" path="logDir">
            <n-input
              v-model:value="form.logDir"
              :placeholder="t('sqlEditor.logDirPlaceholder')"
            />
            <template #feedback>
              <n-text depth="3" style="font-size: 11px;">
                {{ t('sqlEditor.logDirFeedback') }}
              </n-text>
            </template>
          </n-form-item>
          <n-form-item :label="t('sqlEditor.filePattern')" path="pattern">
            <n-input
              v-model:value="form.pattern"
              :placeholder="t('sqlEditor.patternPlaceholder')"
              type="text"
            />
            <template #feedback>
              <n-text depth="3" style="font-size: 11px;">
                {{ t('sqlEditor.patternFeedback') }}
              </n-text>
            </template>
          </n-form-item>
          <n-form-item>
            <n-space vertical style="width: 100%;">
              <n-button 
                @click="scanFiles" 
                :loading="scanning"
                :disabled="!hasConnection"
                secondary
                block
              >
                {{ t('sqlEditor.scanFiles') }}
              </n-button>
              <n-button 
                @click="processFiles" 
                :loading="processing"
                :disabled="!hasConnection || scannedFiles.length === 0"
                type="primary"
                block
              >
                {{ t('sqlEditor.parseImport') }}
              </n-button>
            </n-space>
          </n-form-item>
          <n-form-item v-if="!hasConnection">
            <n-alert type="warning" :title="t('sqlEditor.notConnected')" style="margin-top: 8px;">
              {{ t('sqlEditor.selectConnectionTip') }}
            </n-alert>
          </n-form-item>
        </n-form>
      </n-card>
      <!-- Scan Results -->
      <n-card v-if="scanCompleted" size="small" embedded style="margin-top: 12px;">
        <template #header>
          <n-space align="center">
            <n-text v-if="scannedFiles.length > 0" style="color: #18a058;">✅</n-text>
            <n-text v-else style="color: #f0a020;">⚠️</n-text>
            <n-text>{{ t('sqlEditor.scanResults') }}</n-text>
          </n-space>
        </template>
        <div v-if="scannedFiles.length === 0">
          <n-alert type="warning" :title="t('sqlEditor.noFilesFound')" style="margin-bottom: 12px;">
            {{ t('sqlEditor.noFilesFoundMsg', { pattern: form.pattern, dir: form.logDir }) }}
          </n-alert>
          <n-space style="margin-bottom: 12px;">
            <n-button secondary size="small" @click="scanFiles" :loading="scanning">
              <template #icon>
                <n-text>🔄</n-text>
              </template>
              {{ t('sqlEditor.rescan') }}
            </n-button>
          </n-space>
          <n-text depth="3" style="font-size: 12px;">
            <strong>{{ t('sqlEditor.suggestions') }}</strong>
            <ul style="margin: 8px 0; padding-left: 16px;">
              <li>{{ t('sqlEditor.suggestionCheckDir') }}</li>
              <li>{{ t('sqlEditor.suggestionBroaderPattern') }} <n-code>.*slow.*.log</n-code> {{ t('sqlEditor.or') }} <n-code>.*.log</n-code></li>
              <li>{{ t('sqlEditor.suggestionMatchRegex') }}</li>
              <li>{{ t('sqlEditor.suggestionVerifyPath') }}</li>
            </ul>
          </n-text>
        </div>
        <div v-else>
          <n-alert type="success" style="margin-bottom: 12px;">
            {{ scannedFiles.length === 1 ? t('sqlEditor.foundFilesOne', { count: scannedFiles.length }) : t('sqlEditor.foundFilesMany', { count: scannedFiles.length }) }}
          </n-alert>
          <n-list bordered size="small">
            <n-list-item v-for="file in scannedFiles" :key="file">
              <n-text style="font-size: 13px;">{{ file }}</n-text>
            </n-list-item>
          </n-list>
        </div>
      </n-card>
      <!-- Processing Status -->
      <n-card v-if="processStatus" :title="t('sqlEditor.processingStatus')" size="small" embedded style="margin-top: 12px;">
        <n-progress
          type="line"
          :percentage="processStatus.progress"
          :status="processStatus.status"
          indicator-placement="inside"
        />
        <n-text style="margin-top: 8px; display: block; font-size: 12px;">
          {{ processStatus.message }}
        </n-text>
        <n-text v-if="processStatus.details" depth="3" style="display: block; font-size: 11px;">
          {{ processStatus.details }}
        </n-text>
      </n-card>
    </n-drawer-content>
  </n-drawer>
</template>

<script setup lang="ts">
import { ref, reactive, watch, computed } from 'vue'
import { useSqlEditorStore } from '@/store/modules/sqlEditor'
import { NDrawer, NDrawerContent, NCard, NForm, NFormItem, NInput, NButton, NSpace, NText, NAlert, NList, NListItem, NThing, NProgress, NCode } from 'naive-ui'
import { getSlowlogFiles, processSlowlogFiles } from '@/api/slowlog'
import { useI18n } from 'vue-i18n'
const { t } = useI18n()

interface Props {
  modelValue: boolean
  connected?: boolean
  files?: any[]
}
const props = withDefaults(defineProps<Props>(), {
  connected: false,
  files: () => []
})
const emit = defineEmits(['update:modelValue'])
const formRef = ref()
const scanning = ref(false)
const processing = ref(false)
const scanCompleted = ref(false)
const scannedFiles = ref<any[]>([])
const processStatus = ref<any>(null)
const sqlEditor = useSqlEditorStore()
const hasConnection = computed(() => !!sqlEditor.currentConnection?.id)
// 默认填充示例值，提升体验
const form = reactive({
  logDir: '/Users/aric/Downloads',
  pattern: '.*slow.*log'
})
const rules = {
  logDir: [
    { required: true, message: t('sqlEditor.inputLogDir'), trigger: 'blur' }
  ],
  pattern: [
    { required: true, message: t('sqlEditor.inputPattern'), trigger: 'blur' },
    { validator: (_rule: any, value: string) => {
      if (!value || value.trim() === '') return new Error(t('sqlEditor.patternRequired'))
      try { new RegExp(value); return true } catch { return new Error(t('sqlEditor.invalidRegex')) }
    }, trigger: ['blur', 'change'] }
  ]
}

const processFiles = async () => {
  if (!hasConnection.value || scannedFiles.value.length === 0) {
    window.$message?.warning(t('sqlEditor.selectConnectionAndScan'))
    return
  }
  processing.value = true
  processStatus.value = null
  try {
    const connectionId = sqlEditor.currentConnection?.id
    const logDir = form.logDir
    const pattern = form.pattern
    const res = await processSlowlogFiles(connectionId, logDir, pattern)
    if (res?.status === 'success') {
      processStatus.value = {
        status: 'success',
        progress: 100,
        message: t('sqlEditor.importSuccess'),
        details: res.processed ? t('sqlEditor.processedFiles', { files: res.processed.join(', ') }) : ''
      }
      window.$message?.success(t('sqlEditor.importSuccess'))
    } else {
      processStatus.value = {
        status: 'error',
        progress: 0,
        message: res?.error || t('sqlEditor.importFailed'),
        details: res?.result ? JSON.stringify(res.result) : ''
      }
      window.$message?.error(t('sqlEditor.importFailed') + '：' + (res?.error || t('sqlEditor.unknownError')))
    }
  } catch (err) {
    processStatus.value = {
      status: 'error',
      progress: 0,
      message: err?.message || t('sqlEditor.importException'),
      details: err ? JSON.stringify(err) : ''
    }
    window.$message?.error(t('sqlEditor.importException') + '：' + (err?.message || err))
  } finally {
    processing.value = false
  }
}

const scanFiles = async () => {
  try {
    await formRef.value?.validate()
    scanning.value = true
    scanCompleted.value = false
    scannedFiles.value = []
    const res = await getSlowlogFiles({ logDir: form.logDir.trim(), pattern: form.pattern })
    if (res?.code && res.code !== 200) {
      let msg = ''
      switch (res.reason) {
        case 'not_found': msg = t('sqlEditor.noFilesFoundMsg', { pattern: form.pattern, dir: form.logDir }); break
        case 'permission': msg = t('sqlEditor.suggestionCheckDir'); break
        case 'fs_error': msg = t('sqlEditor.suggestionVerifyPath') + ': ' + res.error; break
        case 'internal': msg = t('sqlEditor.unknownError') + ': ' + res.error; break
        default: msg = res.error || t('sqlEditor.unknownError');
      }
      window.$message?.error(msg)
      scannedFiles.value = []
      scanCompleted.value = true
      return
    }
    // 兼容后端返回 result.matched_files 或 files
    let files: any[] = []
    // 兼容 axios 响应结构 res.data?.result?.matched_files
    const result = (res as any)?.result || (res as any)?.data?.result
    if (Array.isArray(res?.files)) {
      files = res.files
    } else if (Array.isArray(result?.matched_files)) {
      files = result.matched_files
    }
    // 拼接目录路径，确保 scannedFiles.value 为全路径
    scannedFiles.value = files.map(f => {
      if (typeof f === 'string') {
        // 如果后端只返回文件名，则拼接目录路径
        if (!f.startsWith('/') && !f.match(/^([a-zA-Z]:\\|\\)/)) {
          return form.logDir.replace(/\/$/, '') + '/' + f
        }
        return f
      }
      return f
    })
    scanCompleted.value = true
  } catch (err) {
    console.error('[SlowlogDrawer] scanFiles error:', err)
    window.$message?.error(t('sqlEditor.scanFailed') + '：' + (err?.message || err))

    scannedFiles.value = []
    scanCompleted.value = true
  } finally {
    scanning.value = false
  }
}

const setScanResult = (files: any[]) => {
  scannedFiles.value = files
  scanCompleted.value = Array.isArray(files) && files.length > 0
  scanning.value = false
}
const setProcessStatus = (status: any) => {
  processStatus.value = status
  if (status?.progress === 100 || status?.status === 'error') processing.value = false
}
const setScanning = (value: boolean) => { scanning.value = value }
const setProcessing = (value: boolean) => { processing.value = value }
defineExpose({ setScanResult, setProcessStatus, setScanning, setProcessing })
</script>

<style scoped>
/* Add any specific styles for the slowlog tools component */
</style>
