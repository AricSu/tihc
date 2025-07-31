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
    <n-drawer-content title="Slowlog Tools" closable>
      <!-- Help -->
      <n-card title="Tips" size="small" embedded style="margin-top: 12px;margin-bottom: 12px;">
        <n-text depth="3" style="font-size: 11px;">
          <div><strong>How to use:</strong></div>
          <div>• Enter directory path in "Log Directory" field</div>
          <div>• Enter regex pattern in "File Pattern" field to match filenames</div>
          <div>• Example patterns: .*slow.*.log, .*.log, tidb-slow-.*</div>
          <div>• Large files may take time to process</div>
          <div>• Ensure server has read access to log files</div>
        </n-text>
      </n-card>
      <!-- Parse Form -->
      <n-card title="Parse Slowlog Files" size="small" embedded>
        <n-form 
          ref="formRef"
          :model="form"
          :rules="rules"
          label-placement="top"
          size="small"
        >
          <n-form-item label="Log Directory" path="logDir">
            <n-input
              v-model:value="form.logDir"
              placeholder="/Users/aric/Downloads or /var/log/tidb"
            />
            <template #feedback>
              <n-text depth="3" style="font-size: 11px;">
                Directory to search for log files
              </n-text>
            </template>
          </n-form-item>
          <n-form-item label="File Pattern (Regex)" path="pattern">
            <n-input
              v-model:value="form.pattern"
              placeholder=".*slow.*.log or cl-.*-tidb-.*slowlog.log"
              type="text"
            />
            <template #feedback>
              <n-text depth="3" style="font-size: 11px;">
                Regex pattern to match file names (not full paths)<br/>
                Examples: <code>.*slow.*.log</code>, <code>tidb-slow-.*.log</code>, <code>cl-.*slowlog.log</code>
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
                Scan Files
              </n-button>
              <n-button 
                @click="processFiles" 
                :loading="processing"
                :disabled="!hasConnection || scannedFiles.length === 0"
                type="primary"
                block
              >
                Parse & Import
              </n-button>
            </n-space>
          </n-form-item>
          <n-form-item v-if="!hasConnection">
            <n-alert type="warning" title="Not Connected" style="margin-top: 8px;">
              Please select a valid connection before using slowlog tools.
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
            <n-text>Scan Results</n-text>
          </n-space>
        </template>
        <div v-if="scannedFiles.length === 0">
          <n-alert type="warning" title="No Files Found" style="margin-bottom: 12px;">
            No files matching pattern "{{ form.pattern }}" were found in "{{ form.logDir }}".
          </n-alert>
          <n-space style="margin-bottom: 12px;">
            <n-button secondary size="small" @click="scanFiles" :loading="scanning">
              <template #icon>
                <n-text>🔄</n-text>
              </template>
              Rescan
            </n-button>
          </n-space>
          <n-text depth="3" style="font-size: 12px;">
            <strong>Suggestions:</strong>
            <ul style="margin: 8px 0; padding-left: 16px;">
              <li>Check if the directory exists and is accessible</li>
              <li>Try broader patterns like: <n-code>.*slow.*.log</n-code> or <n-code>.*.log</n-code></li>
              <li>Make sure file names match the regex pattern</li>
              <li>Verify the directory path is correct</li>
            </ul>
          </n-text>
        </div>
        <div v-else>
          <n-alert type="success" style="margin-bottom: 12px;">
            Found {{ scannedFiles.length }} matching file{{ scannedFiles.length > 1 ? 's' : '' }}
          </n-alert>
          <n-list bordered size="small">
            <n-list-item v-for="file in scannedFiles" :key="file">
              <n-text style="font-size: 13px;">{{ file }}</n-text>
            </n-list-item>
          </n-list>
        </div>
      </n-card>
      <!-- Processing Status -->
      <n-card v-if="processStatus" title="Processing Status" size="small" embedded style="margin-top: 12px;">
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
    { required: true, message: 'Please input log directory', trigger: 'blur' }
  ],
  pattern: [
    { required: true, message: 'Please input pattern', trigger: 'blur' },
    { validator: (_rule: any, value: string) => {
      if (!value || value.trim() === '') return new Error('Pattern is required')
      try { new RegExp(value); return true } catch { return new Error('Invalid regex pattern') }
    }, trigger: ['blur', 'change'] }
  ]
}

const processFiles = async () => {
  if (!hasConnection.value || scannedFiles.value.length === 0) {
    window.$message?.warning('请先选择连接并扫描到慢日志文件')
    return
  }
  processing.value = true
  processStatus.value = null
  try {
    const connectionId = sqlEditor.currentConnection?.id
    const logDir = form.logDir
    const pattern = form.pattern
    console.log('[SlowlogDrawer] processFiles called, connectionId:', connectionId, 'logDir:', logDir, 'pattern:', pattern)
    const res = await processSlowlogFiles(connectionId, logDir, pattern)
    console.log('[SlowlogDrawer] processSlowlogFiles response:', res)
    if (res?.status === 'success') {
      processStatus.value = {
        status: 'success',
        progress: 100,
        message: `慢日志导入成功`,
        details: res.processed ? `已处理文件: ${res.processed.join(', ')}` : ''
      }
      window.$message?.success('慢日志导入成功')
    } else {
      processStatus.value = {
        status: 'error',
        progress: 0,
        message: res?.error || '慢日志导入失败',
        details: res?.result ? JSON.stringify(res.result) : ''
      }
      window.$message?.error('慢日志导入失败：' + (res?.error || '未知错误'))
    }
  } catch (err) {
    processStatus.value = {
      status: 'error',
      progress: 0,
      message: err?.message || '慢日志导入异常',
      details: err ? JSON.stringify(err) : ''
    }
    window.$message?.error('慢日志导入异常：' + (err?.message || err))
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
    console.log('[SlowlogDrawer] scanFiles called, form:', { logDir: form.logDir, pattern: form.pattern })
    const res = await getSlowlogFiles({ logDir: form.logDir.trim(), pattern: form.pattern })
    console.log('[SlowlogDrawer] getSlowlogFiles response:', res)
    if (res?.code && res.code !== 200) {
      let msg = ''
      switch (res.reason) {
        case 'not_found': msg = `目录不存在：${form.logDir}`; break
        case 'permission': msg = `没有权限访问目录：${form.logDir}`; break
        case 'fs_error': msg = `文件系统错误：${res.error}`; break
        case 'internal': msg = `服务异常：${res.error}`; break
        default: msg = res.error || '未知错误';
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
    window.$message?.error('慢日志扫描失败：' + (err?.message || err))
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
