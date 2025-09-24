<template>
  <div class="extension-config">
    <n-card title="TiHC 浏览器扩展" size="small">
      <n-space vertical>
        <!-- 扩展检测状态 -->
        <div class="status-section">
          <div class="status-info">
            <div class="status-dot" :class="{ 'active': extensionInstalled }"></div>
            <span>{{ extensionInstalled ? '扩展已安装' : '扩展未安装' }}</span>
            <n-tag v-if="extensionInstalled" type="success" size="small">v{{ extensionVersion }}</n-tag>
          </div>
          <n-button @click="handleCheckExtension" :loading="checking" size="small">
            重新检测
          </n-button>
        </div>

        <!-- 目标页面配置与数据采集 -->
        <div v-if="extensionInstalled" class="url-selector-section">
          <n-divider />
          <n-text strong style="display: block; margin-bottom: 12px;">目标页面配置与数据采集</n-text>
          <div class="url-selector">
            <div class="selector-row">
              <span class="label">选择URL:</span>
              <n-select
                v-model:value="selectedUrl"
                :options="urlOptions"
                placeholder="请选择或输入URL"
                filterable
                tag
                @update:value="handleUrlChange"
                style="flex: 1; margin-left: 12px;"
              />
            </div>
            <div class="url-actions">
              <n-button 
                @click="handleNavigateToUrl" 
                type="default" 
                size="small"
                :disabled="!selectedUrl"
              >
                打开页面
              </n-button>
              <n-button 
                type="primary" 
                @click="handleStartCollection" 
                :loading="collecting"
                size="small"
                :disabled="!selectedUrl"
              >
                {{ collecting ? '采集中...' : (isCurrentPage() ? '采集当前页面' : '打开并采集') }}
              </n-button>
            </div>
          </div>
        </div>

        <!-- 采集历史 -->
        <div v-if="extensionInstalled && collectionHistory.length > 0">
          <n-divider />
          <n-text strong style="display: block; margin-bottom: 12px;">最近采集记录</n-text>
          <div class="history-list">
            <div v-for="record in collectionHistory.slice(0, 5)" :key="record.id" class="history-item">
              <div class="history-info">
                <span class="domain">{{ record.domain }}</span>
                <n-tag type="success" size="tiny">
                  已采集
                </n-tag>
              </div>
              <div class="history-time">
                {{ formatTime(record.timestamp) }}
              </div>
            </div>
          </div>
        </div>

        <!-- 未安装提示 -->
        <div v-else class="install-tip">
          <p><strong>请按以下步骤安装扩展：</strong></p>
          <ol>
            <li>打开 chrome://extensions/</li>
            <li>开启"开发者模式"</li>
            <li>点击"加载已解压的扩展程序"</li>
            <li>选择项目中的 tihc-extension 文件夹</li>
          </ol>
          <p class="note">安装完成后点击"重新检测"按钮</p>
        </div>
      </n-space>
    </n-card>
  </div>
</template>

<script setup>
import { onMounted, onUnmounted, ref } from 'vue'
import { NCard, NSpace, NButton, NDivider, NText, NTag, NAlert, NSelect } from 'naive-ui'
import { useTiHCExtension } from '@/composables/useTiHCExtension.js'

// 使用扩展管理 composable
const {
  // 状态
  checking,
  extensionInstalled,
  extensionVersion,
  collecting,
  collectionHistory,
  
  // 方法
  checkExtension,
  startCollection,
  formatTime,
  initialize,
  cleanup
} = useTiHCExtension()

// URL 选择器相关
const selectedUrl = ref('')

// 预定义的 URL 选项
const urlOptions = ref([
  {
    label: 'OP Grafana',
    value: 'http://127.0.0.1:3000/',
    group: 'OP Grafana'
  },
  {
    label: 'TiDB Clinic',
    value: 'https://clinic.pingcap.com/',
    group: '生产环境'
  }
])

// 检查是否是当前页面
function isCurrentPage() {
  if (!selectedUrl.value) return false
  try {
    const selectedDomain = new URL(selectedUrl.value).origin
    const currentDomain = window.location.origin
    return selectedDomain === currentDomain
  } catch {
    return false
  }
}

// 处理URL变化
function handleUrlChange(value) {
  selectedUrl.value = value
}

// 打开页面
function handleNavigateToUrl() {
  if (selectedUrl.value) {
    window.open(selectedUrl.value, '_blank')
    window.$message?.success('已打开新页面: ' + selectedUrl.value)
  }
}

// 开始采集（带错误处理）
async function handleStartCollection() {
  if (!selectedUrl.value) {
    window.$message?.error('请先选择目标URL')
    return
  }
  
  try {
    window.$message?.info(`开始采集目标URL: ${selectedUrl.value}`)
    
    // 直接让插件采集指定URL的数据
    const result = await startCollection(selectedUrl.value)
    window.$message?.success(result.message)
  } catch (error) {
    window.$message?.error(error.message)
  }
}

// 检测扩展（带错误处理）
async function handleCheckExtension() {
  try {
    const result = await checkExtension()
    window.$message?.[result.success ? 'success' : 'warning'](result.message)
  } catch (error) {
    window.$message?.error('检测失败: ' + error.message)
  }
}

onMounted(() => {
  initialize()
  // 默认选择 TiDB Clinic
  selectedUrl.value = 'https://clinic.pingcap.com/'
})

onUnmounted(() => {
  cleanup()
})
</script>

<style scoped>
.extension-config {
  padding: 20px;
  max-width: 700px;
  margin: 0 auto;
}

.status-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 0;
}

.status-info {
  display: flex;
  align-items: center;
  gap: 10px;
}

.status-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #d9d9d9;
  transition: all 0.3s ease;
}

.status-dot.active {
  background: #52c41a;
  box-shadow: 0 0 8px rgba(82, 196, 26, 0.4);
}

.url-selector-section {
  margin: 16px 0;
}

.url-selector {
  background: #f8f9fa;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 16px;
}

.selector-row {
  display: flex;
  align-items: center;
  margin-bottom: 12px;
}

.selector-row .label {
  font-weight: 500;
  color: #495057;
  min-width: 80px;
}

.url-actions {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.history-list {
  max-height: 300px;
  overflow-y: auto;
}

.history-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: #f8f9fa;
  border: 1px solid #dee2e6;
  border-radius: 6px;
  margin-bottom: 8px;
}

.history-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.history-info .domain {
  font-weight: 500;
  color: #495057;
}

.history-time {
  font-size: 12px;
  color: #6c757d;
}

.install-tip {
  padding: 20px;
  background: #f8f9fa;
  border-radius: 8px;
  border: 1px solid #dee2e6;
  font-size: 14px;
  line-height: 1.6;
}

.install-tip ol {
  margin: 12px 0;
  padding-left: 20px;
}

.install-tip li {
  margin-bottom: 8px;
}

.install-tip .note {
  margin-top: 16px;
  padding: 8px 12px;
  background: #e3f2fd;
  border-radius: 4px;
  font-size: 13px;
  color: #1565c0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .extension-config {
    padding: 12px;
  }
  
  .status-section {
    flex-direction: column;
    gap: 12px;
    align-items: stretch;
  }
  
  .status-info {
    justify-content: center;
  }

  .selector-row {
    flex-direction: column;
    align-items: stretch;
    gap: 8px;
  }

  .selector-row .label {
    min-width: auto;
  }

  .url-actions {
    justify-content: center;
  }
  
  .history-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 8px;
  }
  
  .history-time {
    align-self: flex-end;
  }
}
</style>