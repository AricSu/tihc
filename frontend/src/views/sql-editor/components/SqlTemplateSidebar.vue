<template>
  <div class="sql-template-sidebar-col">
    <n-button block type="primary" ghost size="small" class="sidebar-browse-btn" @click="showTemplateModal = true">{{ t('sqlEditor.browseTemplates') }}</n-button>
    <div class="sidebar-quick">
      <div class="sidebar-quick-label">{{ t('sqlEditor.quickInsert') }}</div>
      <n-space vertical size="small">
        <n-button v-for="tpl in quickTemplates" :key="tpl.key" block size="small" secondary class="sidebar-quick-btn" @click="handleInsertTemplateByKey(tpl.key)">
          {{ tpl[locale === 'zh' ? 'zh-label' : 'en-label'] }}
        </n-button>
      </n-space>
    </div>
    <n-modal v-model:show="showTemplateModal" preset="card" :title="t('sqlEditor.templates')" class="sidebar-modal" :style="{ width: 'min(90vw, 1350px)', maxWidth: '1400px' }">
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
                <span class="sidebar-modal-template-title">{{ tpl[locale === 'zh' ? 'zh-label' : 'en-label'] }}</span>
                <div class="sidebar-modal-template-actions">
                  <n-button @click="handleInsertTemplate(tpl.template)" size="tiny" type="primary">{{ t('sqlEditor.insert') }}</n-button>
                  <n-button @click="handleCopyToClipboard(tpl.template)" size="tiny">{{ t('common.copy') }}</n-button>
                </div>
              </div>
              <n-code :code="tpl.template" language="sql" class="sidebar-modal-template-code" />
            </div>
          </n-space>
        </div>
      </div>
      <template #action>
        <n-button @click="showTemplateModal = false">{{ t('common.close') }}</n-button>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
const { t, locale } = useI18n()
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
  if (tpl) {
    emit('insert-template', tpl.template, { append: true })
    window.$message.success(t('sqlEditor.successInsert'))
  } else {
    window.$message.warning(t('sqlEditor.templateNotFound'))
  }
}
function handleInsertTemplate(sql: string) {
  if (sql) {
    emit('insert-template', sql, { append: true })
    showTemplateModal.value = false
    window.$message.success(t('sqlEditor.successInsert'))
  }
}
function handleCopyToClipboard(text: string) {
  navigator.clipboard.writeText(text)
    .then(() => window.$message.success(t('common.copySuccess')))
    .catch(() => window.$message.error(t('common.copyFail')))
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
  width: min(90vw, 1350px) !important;
  max-width: 1400px !important;
  max-height: min(75vh, 550px) !important;
}
.sidebar-modal-main {
  display: flex;
  gap: 16px;
  min-height: min(35vh, 280px);
  max-height: min(65vh, 450px);
}
.sidebar-modal-categories {
  min-width: min(160px, 25vw);
  max-width: 200px;
  border-right: 1px solid #eee;
  padding-right: 12px;
}
.sidebar-modal-category-btn {
  text-align: left;
}
.sidebar-modal-templates {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  max-height: min(55vh, 400px);
  box-sizing: border-box;
}
.sidebar-modal-template-item {
  background: #f7f7fa;
  border-radius: 6px;
  padding: 12px 14px 10px 14px;
  box-shadow: 0 1px 2px 0 rgba(60,60,60,0.03);
  margin-bottom: 8px;
  overflow: hidden;
  word-wrap: break-word;
  max-width: 100%;
  box-sizing: border-box;
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
  width: 100%;
  white-space: pre-wrap;
  word-wrap: break-word;
  word-break: break-word;
  overflow-wrap: break-word;
  overflow-x: auto;
  overflow-y: hidden;
  margin-top: 4px;
  box-sizing: border-box;
  font-size: 12px;
  line-height: 1.4;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .sidebar-modal {
    width: min(90vw, 1000px) !important;
    max-height: min(80vh, 450px) !important;
  }
  .sidebar-modal-main {
    gap: 12px;
    min-height: min(30vh, 250px);
  }
  .sidebar-modal-categories {
    min-width: min(130px, 18vw);
  }
}

@media (max-width: 768px) {
  .sidebar-modal {
    width: 95vw !important;
    max-height: 85vh !important;
  }
  .sidebar-modal-main {
    flex-direction: column;
    gap: 12px;
    min-height: auto;
  }
  .sidebar-modal-categories {
    min-width: auto;
    max-width: none;
    border-right: none;
    border-bottom: 1px solid #eee;
    padding-right: 0;
    padding-bottom: 12px;
  }
  .sidebar-modal-templates {
    max-height: 55vh;
  }
}
</style>
