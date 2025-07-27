<template>
  <n-modal :show="modelValue" @update:show="() => emit('update:modelValue', false)" preset="card" title="Database Connection" style="width: 600px;">
    <n-tabs :value="props.activeTab" @update:value="val => emit('update:activeTab', val)" type="line">
      <n-tab-pane name="new" tab="New Connection">
        <n-form ref="formRef" :model="form" :rules="rules" label-placement="left" label-width="120">
          <n-form-item label="Connection Name" path="name">
            <n-input v-model:value="form.name" placeholder="My TiDB Connection" @blur="generateConnectionName" />
          </n-form-item>
          <n-form-item label="Database Type" path="type">
            <n-select v-model:value="form.type" :options="databaseTypes" @update:value="handleDatabaseTypeChange" />
          </n-form-item>
          <n-form-item label="Host" path="host">
            <n-input v-model:value="form.host" placeholder="localhost" />
          </n-form-item>
          <n-form-item label="Port" path="port">
            <n-input-number v-model:value="form.port" :min="1" :max="65535" placeholder="3306" style="width: 100%;" />
          </n-form-item>
          <n-form-item label="Username" path="username">
            <n-input v-model:value="form.username" placeholder="root" />
          </n-form-item>
          <n-form-item label="Password" path="password">
            <n-input v-model:value="form.password" type="password" placeholder="password (optional)" show-password-on="click" />
          </n-form-item>
          <n-form-item label="Database" path="database">
            <n-input v-model:value="form.database" placeholder="test (optional)" />
          </n-form-item>
        </n-form>
        <n-space justify="end" style="margin-top: 16px;">
          <n-button @click="onTestConnection">Test Connection</n-button>
          <n-button @click="() => emit('update:modelValue', false)">Cancel</n-button>
          <n-button type="primary" @click="onSaveConnection">
            Save & Connect
          </n-button>
        </n-space>
      </n-tab-pane>
      <n-tab-pane name="saved" tab="Saved Connections">
        <n-spin :show="loadingConnections">
          <template v-if="savedConnections.length"> 
            <div class="saved-connections">
              <n-list bordered>
                <n-list-item v-for="conn in savedConnections" :key="conn.id" class="connection-item">
                  <template #prefix>
                    <div class="connection-icon">
                      <n-icon size="20" :color="getConnectionStatusColor(conn)" /><DatabaseIcon />
                      <div class="connection-status-dot" :class="getConnectionStatusClass(conn)"></div>
                    </div>
                  </template>
                  <div class="connection-details">
                    <n-thing class="connection-thing">
                      <template #header>
                        <div class="connection-header">
                          <n-text strong>{{ conn.name }}</n-text>
                          <n-tag v-if="isCurrentConnection(conn)" type="success" size="small" style="margin-left: 8px;">Connected</n-tag>
                        </div>
                      </template>
                      <template #description>
                        <div class="connection-info">
                          <n-text depth="3">
                            {{ (conn.type || '').toUpperCase() }} • {{ conn.host }}:{{ conn.port }}
                          </n-text>
                          <n-text depth="3" v-if="conn.database">
                            Database: {{ conn.database }}
                          </n-text>
                          <n-text depth="3">
                            User: {{ conn.username }}
                          </n-text>
                        </div>
                      </template>
                    </n-thing>
                  </div>
                  <template #suffix>
                    <div class="connection-actions">
                      <n-space>
                        <n-button 
                          size="small" 
                          type="primary"
                          :disabled="isCurrentConnection(conn)"
                          :loading="connectingTo === (conn.id || conn.name)"
                          @click="() => emit('connect-to-saved', conn)"
                        >
                          <template #icon>
                            <n-icon><ConnectIcon /></n-icon>
                          </template>
                          {{ isCurrentConnection(conn) ? 'Connected' : 'Connect' }}
                        </n-button>
                        <n-dropdown 
                          trigger="click" 
                          :options="getConnectionMenuOptions(conn)"
                          @select="key => onConnectionMenu(key, conn)"
                        >
                          <n-button size="small" quaternary>
                            <template #icon>
                              <n-icon><MoreIcon /></n-icon>
                            </template>
                          </n-button>
                        </n-dropdown>
                      </n-space>
                    </div>
                  </template>
                </n-list-item>
              </n-list>
            </div>
          </template>
          <template v-else>
            <n-empty description="No saved connections">
              <template #extra>
                <n-button @click="() => emit('update:activeTab', 'new')">
                  Create your first connection
                </n-button>
              </template>
            </n-empty>
          </template>
        </n-spin> 
        <!-- 删除确认弹窗已移除，由父组件控制 -->
      </n-tab-pane>
    </n-tabs>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, reactive, computed, watch } from 'vue'
import {
  NModal, NTabs, NTabPane, NForm, NFormItem, NInput, NInputNumber,
  NSelect, NButton, NSpace, NList, NListItem, NThing, NText,
  NTag, NIcon, NDropdown, NEmpty, NSpin
} from 'naive-ui'
import { PropType } from 'vue'



const DatabaseIcon = () => '🗄️'
const ConnectIcon = () => '🔗'
const MoreIcon = () => '⋯'
const EditIcon = () => '✏️'
const DeleteIcon = () => '🗑️'
const CopyIcon = () => '📋'
const CheckCircleIcon = () => '✅'



// Dropdown menu options for each connection (admin-style, English)
function getConnectionMenuOptions(conn: Connection) {
  return [
    {
      label: 'Edit',
      key: 'edit',
      icon: () => EditIcon()
    },
    {
      label: 'Duplicate',
      key: 'duplicate',
      icon: () => CopyIcon()
    },
    {
      label: testingFromMenu.value === (conn.id || conn.name) ? 'Testing...' : 'Test Connection',
      key: 'test',
      icon: () => CheckCircleIcon(),
      disabled: testingFromMenu.value === (conn.id || conn.name)
    },
    {
      type: 'divider'
    },
    {
      label: 'Delete',
      key: 'delete',
      icon: () => DeleteIcon()
    }
  ]
}

interface Connection {
  id?: string
  name: string
  type: string
  host: string
  port: number
  username: string
  password?: string
  database?: string
}
const props = defineProps({
  modelValue: Boolean,
  savedConnections: {
    type: Array as PropType<Connection[]>,
    default: () => []
  },
  currentConnection: {
    type: Object as PropType<Connection | null>,
    default: null
  },
  activeTab: {
    type: String,
    default: 'saved'
  },
  connectingTo: {
    type: String,
    default: null
  },
  loadingConnections: {
    type: Boolean,
    default: false
  }
})
const emit = defineEmits([
  'update:modelValue',
  'update:activeTab',
  'test-connection',
  'save-connection',
  'connect-to-saved',
  'edit-connection',
  'duplicate-connection',
  'delete-connection'
])

// Track which connection is being tested from the menu
const testingFromMenu = ref<string | null>(null)

const formRef = ref()

const form = reactive({
  name: '',
  type: 'mysql',
  host: 'localhost',
  port: 3306,
  username: 'root',
  password: '',
  database: ''
})

// When modal opens, just reset form
watch(
  () => props.modelValue,
  (val) => {
    if (val) {
      resetForm()
    }
  }
)

const databaseTypes = [
  { label: 'MySQL', value: 'mysql' },
  { label: 'TiDB', value: 'tidb' }
]
const rules = {
  name: { required: true, message: '请输入连接名称', trigger: 'blur' },
  host: { required: true, message: '请输入主机地址', trigger: 'blur' },
  port: {
    required: true,
    message: '请输入有效端口 (1-65535)',
    trigger: 'blur',
    validator: (_rule: any, value: number) => {
      if (!value) {
        return new Error('请输入端口')
      }
      if (value < 1 || value > 65535) {
        return new Error('端口需在 1-65535 之间')
      }
      return true
    }
  },
  username: { required: true, message: '请输入用户名', trigger: 'blur' },
  password: { required: false, trigger: 'blur' }
}
function generateConnectionName() {
  if (!form.name && form.host && form.type) {
    const typeMap = {
      mysql: 'MySQL',
      tidb: 'TiDB'
    }
    form.name = `${typeMap[form.type] || 'Database'} - ${form.host}`
  }
}
function handleDatabaseTypeChange(type: string) {
  const defaultPorts = {
    mysql: 3306,
    tidb: 4000
  }
  form.port = defaultPorts[type] || 3306
}
function fillForm(conn: Connection) {
  Object.assign(form, {
    name: conn.name,
    type: conn.type,
    host: conn.host,
    port: conn.port,
    username: conn.username,
    password: conn.password || '',
    database: conn.database || ''
  })
}
function resetForm() {
  Object.assign(form, {
    name: '',
    type: 'mysql',
    host: 'localhost',
    port: 3306,
    username: 'root',
    password: '',
    database: ''
  })
}
function onTestConnection() {
  formRef.value?.validate().then(() => {
    emit('test-connection', { ...form })
  })
}
function onSaveConnection() {
  formRef.value?.validate().then(() => {
    emit('save-connection', { ...form })
    resetForm()
    emit('update:activeTab', 'saved')
    emit('update:modelValue', false)
  })
}
function onConnectToSaved(conn) {
  emit('connect-to-saved', conn)
}
function onConnectionMenu(key, conn) {
  switch (key) {
    case 'edit':
      fillForm(conn)
      emit('update:activeTab', 'new')
      break
    case 'duplicate':
      emit('duplicate-connection', conn)
      break
    case 'test':
      emit('test-connection', conn)
      break
    case 'delete':
      emit('delete-connection', conn)
      break
  }
}
// Utility: check if a connection is当前选中
const isCurrentConnection = (conn: Connection) => {
  return props.currentConnection && conn.id === props.currentConnection.id
}
// Utility: get color for connection status icon
const getConnectionStatusColor = (conn: Connection) => {
  return isCurrentConnection(conn) ? '#18a058' : '#c0c4cc'
}
// Utility: get class for connection status dot
const getConnectionStatusClass = (conn: Connection) => {
  return isCurrentConnection(conn) ? 'connected' : 'disconnected'
}


</script>