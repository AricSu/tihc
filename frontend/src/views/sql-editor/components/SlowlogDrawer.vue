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
              :disabled="props.connected === false"
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
              :disabled="props.connected === false"
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
                :disabled="!connected"
                secondary
                block
              >
                Scan Files
              </n-button>
              <n-button 
                @click="processFiles" 
                :loading="processing"
                :disabled="!connected || scannedFiles.length === 0"
                type="primary"
                block
              >
                Parse & Import
              </n-button>
            </n-space>
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
            <n-list-item v-for="file in scannedFiles" :key="file.path">
              <n-thing>
                <template #header>
                  <n-text style="font-size: 12px;">{{ file.name }}</n-text>
                </template>
                <template #description>
                  <n-text depth="3" style="font-size: 11px;">
                    {{ file.size }} • {{ file.modified }}
                  </n-text>
                </template>
              </n-thing>
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
import { ref, reactive, watch, toRefs } from 'vue'
import { NDrawer, NDrawerContent, NCard, NForm, NFormItem, NInput, NButton, NSpace, NText, NAlert, NList, NListItem, NThing, NProgress, NCode } from 'naive-ui'

interface Props {
  modelValue: boolean
  connected?: boolean
  files?: any[]
}
const props = withDefaults(defineProps<Props>(), {
  connected: false,
  files: () => []
})
const emit = defineEmits(['update:modelValue', 'scan-files', 'process-files'])
const formRef = ref()
const scanning = ref(false)
const processing = ref(false)
const scanCompleted = ref(false)
const scannedFiles = ref<any[]>([])
const processStatus = ref<any>(null)
const { connected } = toRefs(props)
const form = reactive({
  logDir: '',
  pattern: ''
})
const rules = {
  logDir: [
    { required: true, message: 'Please input log directory', trigger: 'blur' }
  ],
  pattern: [
    { required: true, message: 'Please input pattern', trigger: 'blur' },
    { validator: (_rule: any, value: string) => {
      if (!value || value.trim() === '') return new Error('Pattern is required')
      try { new RegExp(value); return true } catch { return new Error('Invalid regular expression') }
    }, trigger: ['blur', 'change'] }
  ]
}
const scanFiles = async () => {
  try {
    await formRef.value?.validate()
    scanning.value = true
    scanCompleted.value = false
    scannedFiles.value = []
    emit('scan-files', { logDir: form.logDir.trim(), pattern: form.pattern.trim() })
  } catch {}
}
const processFiles = async () => {
  try {
    await formRef.value?.validate()
    processing.value = true
    processStatus.value = { progress: 0, status: 'info', message: 'Starting slowlog processing...', details: 'Initializing...' }
    emit('process-files', { logDir: form.logDir.trim(), pattern: form.pattern.trim(), files: scannedFiles.value })
  } catch {}
}
watch(() => props.files, (newFiles) => {
  scannedFiles.value = newFiles || []
  scanCompleted.value = Array.isArray(newFiles)
}, { immediate: true, deep: true })
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
