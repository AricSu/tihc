<template>
  <n-modal :show="modelValue" @update:show="() => emit('update:modelValue', false)" preset="card" :title="t('sqlEditor.databaseConnection')" style="width: 600px;">
    <n-tabs :value="props.activeTab" @update:value="val => emit('update:activeTab', val)" type="line">
      <n-tab-pane name="new" :tab="t('sqlEditor.newConnection')">
        <n-form ref="formRef" :model="form" :rules="rules" label-placement="left" label-width="120">
        <n-form-item :label="t('sqlEditor.connectionName')" path="name">
          <n-input v-model:value="form.name" :placeholder="t('sqlEditor.connectionNamePlaceholder')" @blur="generateConnectionName" />
        </n-form-item>
        <n-form-item :label="t('sqlEditor.databaseType')" path="engine">
          <n-select v-model:value="form.engine" :options="databaseTypes" @update:value="setDatabaseType" />
        </n-form-item>
        <n-form-item :label="t('sqlEditor.host')" path="host">
          <n-input v-model:value="form.host" :placeholder="t('sqlEditor.hostPlaceholder')" />
        </n-form-item>
        <n-form-item :label="t('sqlEditor.port')" path="port">
          <n-input-number v-model:value="form.port" :min="1" :max="65535" :placeholder="t('sqlEditor.portPlaceholder')" style="width: 100%;" />
        </n-form-item>
        <n-form-item :label="t('sqlEditor.username')" path="username">
          <n-input v-model:value="form.username" :placeholder="t('sqlEditor.usernamePlaceholder')" />
        </n-form-item>
        <n-form-item :label="t('sqlEditor.password')" path="password">
          <n-input v-model:value="form.password" type="password" :placeholder="t('sqlEditor.passwordPlaceholder')" show-password-on="click" />
        </n-form-item>
        <n-form-item :label="t('sqlEditor.database')" path="database">
          <n-input v-model:value="form.database" :placeholder="t('sqlEditor.databasePlaceholder')" />
        </n-form-item>
        <n-form-item :label="t('sqlEditor.useTLS')" path="use_tls">
          <n-switch v-model:value="form.use_tls" />
        </n-form-item>
        <n-form-item v-if="form.use_tls" :label="t('sqlEditor.caCertPath')" path="ca_cert_path">
          <n-input v-model:value="form.ca_cert_path" :placeholder="t('sqlEditor.caCertPathPlaceholder')" />
        </n-form-item>
        </n-form>
        <n-space justify="end" style="margin-top: 16px;">
          <n-button @click="testConnection">{{ t('sqlEditor.testConnection') }}</n-button>
          <n-button @click="() => emit('update:modelValue', false)">{{ t('sqlEditor.cancel') }}</n-button>
          <n-button type="primary" @click="saveConnection">
            {{ t('sqlEditor.saveAndConnect') }}
          </n-button>
        </n-space>
      </n-tab-pane>
      <n-tab-pane name="saved" :tab="t('sqlEditor.savedConnections')">
        <n-spin :show="loadingConnections">
          <template v-if="savedConnections.length"> 
            <div class="saved-connections">
            <n-list bordered>
              <n-list-item v-for="conn in savedConnections" :key="conn.id" class="connection-item">
                <template #prefix>
                  <div class="connection-icon">
                    <Icon :icon="'mdi:database'" :width="20" :height="20" :color="getConnectionStatusColor(conn)" />
                    <div class="connection-status-dot" :class="getConnectionStatusClass(conn)"></div>
                  </div>
                </template>
                <div class="connection-details">
                  <n-thing class="connection-thing">
                    <template #header>
                      <div class="connection-header">
                        <n-text strong>{{ conn.name }}</n-text>
                        <n-tag v-if="isCurrentConnection(conn)" type="success" size="small" style="margin-left: 8px;">{{ t('sqlEditor.connected') }}</n-tag>
                      </div>
                    </template>
                    <template #description>
                      <div class="connection-info">
                        <n-text depth="3">
                          {{ (conn.engine || '').toUpperCase() }} • {{ conn.host }}:{{ conn.port }}
                        </n-text>
                        <n-text depth="3" v-if="conn.database">
                          {{ t('sqlEditor.database') }}: {{ conn.database }}
                        </n-text>
                        <n-text depth="3">
                          {{ t('sqlEditor.user') }}: {{ conn.username }}
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
                        :disabled="!!isCurrentConnection(conn)"
                        :loading="connectingTo === (conn.id || conn.name)"
                        @click="connectToSaved(conn)"
                      >
                        <template #icon>
                          <Icon icon="mdi:link-variant" width="18" height="18" />
                        </template>
                        {{ isCurrentConnection(conn) ? t('sqlEditor.connected') : t('sqlEditor.connect') }}
                      </n-button>
                      <n-dropdown 
                        trigger="click" 
                        :options="getConnectionMenuOptions(conn)"
                        @select="key => handleConnectionMenu(key, conn)"
                      >
                        <n-button size="small" quaternary>
                          <template #icon>
                            <Icon icon="mdi:dots-horizontal" width="18" height="18" />
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
            <n-empty :description="t('sqlEditor.noSavedConnections')">
              <template #extra>
                <n-button @click="() => emit('update:activeTab', 'new')">
                  {{ t('sqlEditor.createFirstConnection') }}
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
import { ref, reactive, watch, h } from 'vue'
import { useI18n } from 'vue-i18n'
import { Icon } from '@iconify/vue'
import {
  NModal, NTabs, NTabPane, NForm, NFormItem, NInput, NInputNumber,
  NSelect, NButton, NSpace, NList, NListItem, NThing, NText,
  NTag, NDropdown, NEmpty, NSpin
} from 'naive-ui'
import { PropType } from 'vue'


// Returns dropdown menu options for a connection
function getConnectionMenuOptions(conn: Connection) {
  return [
    {
      label: 'Edit',
      key: 'edit',
      icon: () => h(Icon, { icon: 'mdi:pencil', width: 18, height: 18 })
    },
    {
      label: 'Duplicate',
      key: 'duplicate',
      icon: () => h(Icon, { icon: 'mdi:content-copy', width: 18, height: 18 })
    },
    {
      label: testingFromMenu.value === (conn.id || conn.name) ? 'Testing...' : 'Test Connection',
      key: 'test',
      icon: () => h(Icon, { icon: 'mdi:check-circle', width: 18, height: 18, color: '#18a058' }),
      disabled: testingFromMenu.value === (conn.id || conn.name)
    },
    {
      type: 'divider'
    },
    {
      label: 'Delete',
      key: 'delete',
      icon: () => h(Icon, { icon: 'mdi:delete', width: 18, height: 18 })
    }
  ];
}

interface Connection {
id?: number
  name: string
  engine: string
  host: string
  port: number
  username: string
  password?: string
  database?: string
  use_tls?: boolean
  ca_cert_path?: string
  created_at?: string
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
  'update-connection',
  'duplicate-connection',
  'delete-connection'
])

// State: tracks which connection is being tested from the menu
const testingFromMenu = ref<string | null>(null);
const formRef = ref();
const form = reactive({
  id: undefined as number | undefined,
  name: '',
  engine: 'tidb',
  host: 'localhost',
  port: 4000,
  username: 'root',
  password: '',
  database: '',
  use_tls: false,
  ca_cert_path: '',
  created_at: ''
});

// Reset form when modal opens
watch(() => props.modelValue, (val) => { if (val) resetForm(); });

const { t } = useI18n();
const databaseTypes = [
  { label: t('sqlEditor.tidb'), value: 'tidb' }
];
const rules = {
  name: { required: true, message: t('sqlEditor.connectionNameRequired'), trigger: 'blur' },
  engine: { required: true, message: t('sqlEditor.databaseTypeRequired'), trigger: 'change' },
  host: { required: true, message: t('sqlEditor.hostRequired'), trigger: 'blur' },
  port: {
    required: true,
    message: t('sqlEditor.portRange'),
    trigger: 'blur',
    validator: (_rule: any, value: number) => {
      if (!value) return new Error(t('sqlEditor.portRequired'));
      if (value < 1 || value > 65535) return new Error(t('sqlEditor.portRange'));
      return true;
    }
  },
  username: { required: true, message: t('sqlEditor.usernameRequired'), trigger: 'blur' },
  password: { required: false, trigger: 'blur' },
  use_tls: { required: false },
  ca_cert_path: { required: false },
  created_at: { required: false }
};
function generateConnectionName() {
  if (!form.name && form.host && form.engine) {
    const typeMap = { tidb: t('sqlEditor.tidb') };
    form.name = `${typeMap[form.engine] || t('sqlEditor.database')} - ${form.host}`;
  }
}
function setDatabaseType(type: string) {
  form.engine = type;
  const defaultPorts = { tidb: 4000 };
  form.port = defaultPorts[type] || 4000;
}
function setFormFromConnection(conn: Connection) {
  Object.assign(form, {
    id: conn.id,
    name: conn.name,
    engine: conn.engine,
    host: conn.host,
    port: conn.port,
    username: conn.username,
    password: conn.password || '',
    database: conn.database || '',
    use_tls: conn.use_tls ?? false,
    ca_cert_path: conn.ca_cert_path || '',
    created_at: conn.created_at || ''
  });
}
function resetForm() {
  Object.assign(form, {
    id: undefined,
    name: '',
    engine: 'tidb',
    host: 'localhost',
    port: 4000,
    username: 'root',
    password: '',
    database: '',
    use_tls: false,
    ca_cert_path: '',
    created_at: ''
  });
}
function testConnection() {
  formRef.value?.validate().then(() => {
    if (!form.created_at) form.created_at = new Date().toISOString();
    const payload = {
      ...form,
      use_tls: form.use_tls ?? false,
      ca_cert_path: form.ca_cert_path ?? ''
    };
    emit('test-connection', payload);
  });
}
function saveConnection() {
  formRef.value?.validate().then(() => {
    const exists = props.savedConnections.some(
      c => c.name === form.name && c.id !== form.id
    );
    if (exists) {
      window.$message?.error?.(t('sqlEditor.connectionNameExists'));
      return;
    }
    if (!form.created_at) form.created_at = new Date().toISOString();
    const idNum = typeof form.id === 'number' ? form.id : Number(form.id);
    const isEdit = props.savedConnections.some(c => c.id === idNum);
    const payload = {
      ...form,
      id: isEdit ? idNum : Date.now(),
      use_tls: form.use_tls ?? false,
      ca_cert_path: form.ca_cert_path ?? ''
    };
    if (isEdit) {
      emit('update-connection', payload);
    } else {
      emit('save-connection', payload);
    }
    resetForm();
    emit('update:activeTab', 'saved');
    emit('update:modelValue', false);
  });
}
function connectToSaved(conn: Connection) {
  emit('connect-to-saved', conn);
}
function handleConnectionMenu(key: string, conn: Connection) {
  switch (key) {
    case 'edit': {
      const idNum = typeof conn.id === 'string' ? Number(conn.id) : conn.id;
      setFormFromConnection({ ...conn, id: idNum });
      emit('update:activeTab', 'new');
      break;
    }
    case 'duplicate': {
      const duplicated = {
        ...conn,
        id: undefined,
        name: conn.name + ' (copy)'
      };
      setFormFromConnection(duplicated);
      emit('update:activeTab', 'new');
      break;
    }
    case 'test':
      emit('test-connection', conn);
      break;
    case 'delete':
      emit('delete-connection', conn);
      break;
  }
}
function isCurrentConnection(conn: Connection) {
  return props.currentConnection && conn.id === props.currentConnection.id;
}
function getConnectionStatusColor(conn: Connection) {
  return isCurrentConnection(conn) ? '#18a058' : '#c0c4cc';
}
function getConnectionStatusClass(conn: Connection) {
  return isCurrentConnection(conn) ? 'connected' : 'disconnected';
}


</script>