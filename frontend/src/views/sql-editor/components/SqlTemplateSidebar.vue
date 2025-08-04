<template>
  <div class="sql-template-sidebar-col">
    <div class="sidebar-title">SQL模板 / SQL Templates</div>
    <n-button block type="primary" ghost size="small" class="sidebar-browse-btn" @click="showTemplateModal = true">浏览全部模板</n-button>
    <div class="sidebar-quick">
      <div class="sidebar-quick-label">快速插入</div>
      <n-space vertical size="small">
        <n-button v-for="tpl in quickTemplates" :key="tpl.key" block size="small" secondary class="sidebar-quick-btn" @click="handleInsertTemplateByKey(tpl.key)">
          {{ tpl['zh-label'] }}
        </n-button>
      </n-space>
    </div>
    <n-modal v-model:show="showTemplateModal" preset="card" title="SQL模板 / SQL Templates" class="sidebar-modal">
      <div class="sidebar-modal-main">
        <div class="sidebar-modal-categories">
          <n-space vertical size="small">
            <n-button
              v-for="cat in templateCategories"
              :key="cat.key"
              :type="activeCategory === cat.key ? 'primary' : 'default'"
              size="small"
              block
              class="sidebar-modal-category-btn"
              @click="activeCategory = cat.key"
            >
              {{ cat.label }}
            </n-button>
          </n-space>
        </div>
        <div class="sidebar-modal-templates">
          <n-space vertical size="medium">
            <div v-for="tpl in templatesByCategory[activeCategory]" :key="tpl.key" class="sidebar-modal-template-item">
              <div class="sidebar-modal-template-header">
                <span class="sidebar-modal-template-title">{{ tpl['zh-label'] }} / {{ tpl['en-label'] }}</span>
                <div class="sidebar-modal-template-actions">
                  <n-button @click="handleInsertTemplate(tpl.template)" size="tiny" type="primary">插入</n-button>
                  <n-button @click="handleCopyToClipboard(tpl.template)" size="tiny">复制</n-button>
                </div>
              </div>
              <n-code :code="tpl.template" language="sql" class="sidebar-modal-template-code" />
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

// 分类与分组逻辑
function getTemplateCategories(templates: any[]) {
  const cats: Record<string, { key: string; label: string }> = {}
  templates.forEach(tpl => {
    const key = tpl['en-category']
    if (!cats[key]) {
      cats[key] = {
        key,
        label: `${tpl['zh-category']} / ${tpl['en-category']}`
      }
    }
  })
  return Object.values(cats)
}

function groupTemplatesByCategory(templates: any[], categories: { key: string }[]) {
  const map: Record<string, any[]> = {}
  categories.forEach(cat => { map[cat.key] = [] })
  templates.forEach(tpl => {
    if (map[tpl['en-category']]) map[tpl['en-category']].push(tpl)
  })
  return map
}

const templateCategories = computed(() => getTemplateCategories(sqlTemplatesRaw))
const activeCategory = ref(templateCategories.value[0]?.key || '')
const templatesByCategory = computed(() => groupTemplatesByCategory(sqlTemplatesRaw, templateCategories.value))
const quickTemplates = computed(() => sqlTemplatesRaw.slice(0, 3))

// 插入相关逻辑
function handleInsertTemplateByKey(key: string) {
  const tpl = sqlTemplatesRaw.find(t => t.key === key)
  console.log('[SqlTemplateSidebar] handleInsertTemplateByKey', key, tpl)
  if (tpl) {
    console.log('[SqlTemplateSidebar] emit insert-template', tpl.template)
    emit('insert-template', tpl.template, { append: true })
    window.$message.success('模板已插入')
  } else {
    window.$message.warning('未找到模板')
  }
}

function handleInsertTemplate(sql: string) {
  if (sql) {
    console.log('[SqlTemplateSidebar] emit insert-template', sql)
    emit('insert-template', sql, { append: true })
    showTemplateModal.value = false
    window.$message.success('模板已插入')
  }
}

function handleCopyToClipboard(text: string) {
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
.sidebar-title {
  font-weight: 600;
  font-size: 15px;
  color: #222;
  margin-bottom: 10px;
}
.sidebar-browse-btn {
  margin-bottom: 12px;
}
.sidebar-quick {
  margin-bottom: 12px;
}
.sidebar-quick-label {
  font-size: 13px;
  color: #888;
  margin-bottom: 6px;
}
.sidebar-quick-btn {
  text-align: left;
}
.sidebar-modal {
  width: 900px;
  max-height: 80vh;
}
.sidebar-modal-main {
  display: flex;
  gap: 18px;
  min-height: 400px;
}
.sidebar-modal-categories {
  min-width: 180px;
  border-right: 1px solid #eee;
  padding-right: 12px;
}
.sidebar-modal-category-btn {
  text-align: left;
}
.sidebar-modal-templates {
  flex: 1;
  overflow-y: auto;
  max-height: 60vh;
}
.sidebar-modal-template-item {
  background: #f7f7fa;
  border-radius: 6px;
  padding: 12px 14px 10px 14px;
  box-shadow: 0 1px 2px 0 rgba(60,60,60,0.03);
  margin-bottom: 8px;
}
.sidebar-modal-template-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 6px;
}
.sidebar-modal-template-title {
  font-weight: 500;
  font-size: 14px;
  color: #333;
}
.sidebar-modal-template-actions {
  display: flex;
  gap: 8px;
}
.sidebar-modal-template-code {
  max-width: 100%;
  white-space: pre-wrap;
  margin-top: 4px;
}
</style>
