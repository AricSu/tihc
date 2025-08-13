
<template>
  <div class="ddl-root">
    <div class="ddl-header-bar header-card">
      <Icon icon="mdi:database-check" class="header-icon" />
      <div class="header-title-group">
        <div class="header-title">DDL 预检查</div>
        <div class="header-subtitle">检查 DDL 语句是否会造成 “数据” 或 “统计信息” 丢失，提供安全建议</div>
      </div>
    </div>
    <n-grid :cols="24" :x-gap="18" responsive="screen" class="ddl-main-grid">
      <n-gi :span="showResults ? 12 : 24" class="ddl-main-gi">
        <InputArea @result="onResult" />
      </n-gi>
      <n-gi v-if="showResults" :span="12" class="ddl-main-gi">
        <OutputArea :result="result" />
      </n-gi>
    </n-grid>
  </div>

</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Icon } from '@iconify/vue'
import InputArea from './input.vue'
import OutputArea from './output.vue'

const result = ref(null)
const showResults = ref(false)
function onResult(res) {
  result.value = res
  showResults.value = !!res
}
</script>

<style scoped>
.ddl-root {
  height: 100vh;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
.ddl-main-grid {
  flex: 1 1 auto;
  height: 0;
  min-height: 0;
  margin-top: 18px;
}
.ddl-main-gi {
  height: 100%;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
.ddl-header-bar {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 32px 0 12px 0;
}
.header-card {
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 2px 12px 0 rgba(0,0,0,0.04);
  margin: 0 0 18px 0;
}
.header-icon {
  font-size: 32px;
  color: #1976d2;
  margin-bottom: 6px;
}
.header-title-group {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.header-title {
  font-size: 22px;
  font-weight: 600;
  margin-bottom: 4px;
}
.header-subtitle {
  font-size: 14px;
  color: #888;
  font-weight: 400;
}
</style>