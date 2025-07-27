<template>
  <div class="sql-template-sidebar-col">
    <div style="font-weight:600;font-size:15px;color:#222;margin-bottom:10px;">SQL模板 / SQL Templates</div>
    <n-button block type="primary" ghost size="small" style="margin-bottom: 12px;" @click="showTemplateModal = true">浏览全部模板</n-button>
    <div style="margin-bottom: 12px;">
      <div style="font-size:13px;color:#888;margin-bottom:6px;">快速插入</div>
      <n-space vertical size="small">
        <n-button v-for="tpl in quickTemplates" :key="tpl.key" block size="small" secondary style="text-align:left;" @click="insertTemplate(tpl.key)">
          {{ tpl['zh-label'] }}
        </n-button>
      </n-space>
    </div>
    <n-modal v-model:show="showTemplateModal" preset="card" title="SQL模板 / SQL Templates" style="width: 900px; max-height: 80vh;">
      <div style="display:flex;gap:18px;min-height:400px;">
        <div style="min-width:180px;border-right:1px solid #eee;padding-right:12px;">
          <n-space vertical size="small">
            <n-button
              v-for="cat in templateCategories"
              :key="cat.key"
              :type="activeCategory === cat.key ? 'primary' : 'default'"
              size="small"
              block
              style="text-align:left;"
              @click="activeCategory = cat.key"
            >
              {{ cat.label }}
            </n-button>
          </n-space>
        </div>
        <div style="flex:1;overflow-y:auto;max-height:60vh;">
          <n-space vertical size="medium">
            <div v-for="tpl in templatesByCategory[activeCategory]" :key="tpl.key" style="background:#f7f7fa;border-radius:6px;padding:12px 14px 10px 14px;box-shadow:0 1px 2px 0 rgba(60,60,60,0.03);margin-bottom:8px;">
              <div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:6px;">
                <span style="font-weight:500;font-size:14px;color:#333;">{{ tpl['zh-label'] }} / {{ tpl['en-label'] }}</span>
                <div style="display: flex; gap: 8px;">
                  <n-button @click="insertTemplateFromModal(tpl.template)" size="tiny" type="primary">插入</n-button>
                  <n-button @click="copyToClipboard(tpl.template)" size="tiny">复制</n-button>
                </div>
              </div>
              <n-code :code="tpl.template" language="sql" style="max-width:100%;white-space:pre-wrap;margin-top:4px;" />
            </div>
          </n-space>
        </div>
      </div>
      <template #action>
        <n-button @click="showTemplateModal = false">关闭</n-button>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
// @ts-expect-error: TypeScript JSON import
import sqlTemplatesRaw from '@/store/sqlTemplate.json' with { type: 'json' }

const props = defineProps<{ showSidebar: boolean }>()
const emit = defineEmits(['insert-template', 'update:showSidebar'])
const showTemplateModal = ref(false)

// 分类抽取
const templateCategories = computed(() => {
  const cats: Record<string, { key: string; label: string }> = {}
  for (const tpl of sqlTemplatesRaw) {
    const key = tpl['en-category']
    if (!cats[key]) {
      cats[key] = {
        key,
        label: `${tpl['zh-category']} / ${tpl['en-category']}`
      }
    }
  }
  return Object.values(cats)
})

const activeCategory = ref(templateCategories.value[0]?.key || '')

// 按分类分组
const templatesByCategory = computed(() => {
  const map: Record<string, any[]> = {}
  for (const cat of templateCategories.value) map[cat.key] = []
  for (const tpl of sqlTemplatesRaw) {
    if (map[tpl['en-category']]) map[tpl['en-category']].push(tpl)
  }
  return map
})

// 快速插入（取前3个）
const quickTemplates = computed(() => sqlTemplatesRaw.slice(0, 3))

function insertTemplate(key: string) {
  const tpl = sqlTemplatesRaw.find(t => t.key === key)
  if (tpl) {
    emit('insert-template', tpl.template)
  window.$message.success('模板已插入')
  } else {
    window.$message.warning('未找到模板')
  }
}
function insertTemplateFromModal(sql: string) {
  if (sql) {
    emit('insert-template', sql)
    showTemplateModal.value = false
    window.$message.success('模板已插入')
  }
}
function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text)
    .then(() => window.$message.success('已复制到剪贴板'))
    .catch(() => window.$message.error('复制失败'))
}
</script>

<style scoped>
.sql-template-sidebar-col {
  display: flex;
  flex-direction: column;
  padding: 18px 18px 0 18px;
}
</style>
