<template>
  <div id="main-content">
    <div>
      <span>当前域名：</span>
      <span id="current-domain">{{ domain }}</span>
    </div>
    <div>
      <span>采集状态：</span>
      <span id="collection-status" :class="statusClass">{{ collectionStatus }}</span>
    </div>
    <div>
      <span>最近采集时间：</span>
      <span id="last-collection" :class="lastCollectionClass">{{ lastCollectionTime }}</span>
    </div>
    <div>
      <span>总采集次数：</span>
      <span id="total-collections">{{ totalCollections }}</span>
    </div>
    <div>
      <span>涉及域名数：</span>
      <span id="total-domains">{{ totalDomains }}</span>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';

const domain = ref('加载中...');
const collectionStatus = ref('待命中');
const statusClass = ref('inactive');
const lastCollectionTime = ref('暂无记录');
const lastCollectionClass = ref('inactive');
const totalCollections = ref(0);
const totalDomains = ref(0);

function formatTime(timestamp: number | null) {
  if (!timestamp) return '暂无记录';
  return new Date(timestamp).toLocaleString('zh-CN');
}

function updateCollectionStatus(isCollecting: boolean) {
  if (isCollecting) {
    collectionStatus.value = '采集中...';
    statusClass.value = 'warning';
  } else {
    collectionStatus.value = '待命中';
    statusClass.value = 'inactive';
  }
}

async function loadPageInfo() {
  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  if (!tab || !tab.url) {
    domain.value = '无法获取';
    return;
  }
  const url = new URL(tab.url);
  domain.value = url.hostname;
  chrome.tabs.sendMessage(tab.id!, { action: 'GET_STATUS' }, (response) => {
    if (chrome.runtime.lastError) {
      domain.value = url.hostname;
    } else if (response && response.success) {
      const data = response.data;
      domain.value = data.domain || url.hostname;
      updateCollectionStatus(data.isCollecting);
      if (data.lastCollectionTime) {
        lastCollectionTime.value = formatTime(data.lastCollectionTime);
        lastCollectionClass.value = 'active';
      }
    } else {
      domain.value = url.hostname;
    }
  });
}

async function loadExtensionState() {
  const response = await new Promise<any>((resolve) => {
    chrome.runtime.sendMessage({ type: 'GET_STATE' }, resolve);
  });
  if (response && response.success) {
    const state = response.state;
    totalCollections.value = state.collectionCount || 0;
    totalDomains.value = state.collectionDomains ? Object.keys(state.collectionDomains).length : 0;
    if (state.lastCollectionTime) {
      lastCollectionTime.value = formatTime(state.lastCollectionTime);
      lastCollectionClass.value = 'active';
    }
  }
}

onMounted(async () => {
  await Promise.all([
    loadPageInfo(),
    loadExtensionState()
  ]);
});
</script>

<style scoped>
.inactive { color: #888; }
.warning { color: #f90; }
.active { color: #28a745; }
</style>