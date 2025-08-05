import { defineStore } from 'pinia'

export const useSqlEditorStore = defineStore('sqlEditor', {
  state: () => ({
    connections: [],
    currentConnection: null,
    queryResults: [],
    activeResultTab: '',
    sqlContent: '',
    history: [],
    settings: {
      fontSize: 14,
      tabSize: 2,
      wordWrap: true,
      autoSave: false,
      queryTimeout: 30,
    },
    showConnectionModal: false,
    showUploadDrawer: false,
    showSidebar: true,
    showSlowlogPanel: false,
    slowQuerySchema: [],
    // 新增页面依赖的属性
    isExecuting: false,
    sqlTemplates: [],
    showQueryHistory: false,
    // schema 加载 loading
    loadingSchema: false,
    // 慢日志工具相关
    slowlogFiles: [],
    slowlogProcessStatus: null,
    // ConnectionManager 受控 tab/连接状态
    activeTab: 'saved',
    connectingTo: null,
  }),
  actions: {
    setConnections(list) { this.connections = list },
    setCurrentConnection(conn) { this.currentConnection = conn },
    addQueryResult(result) { this.queryResults.unshift(result) },
    setActiveResultTab(tab) { this.activeResultTab = tab },
    setSqlContent(content) { this.sqlContent = content },
    setShowConnectionModal(val) { this.showConnectionModal = val },
    setShowSettings(val) { this.showSettings = val },
    setShowUploadDrawer(val) { this.showUploadDrawer = val },
    setShowSidebar(val) { this.showSidebar = val },
    setShowSlowlogPanel(val) { this.showSlowlogPanel = val },
    setSlowQuerySchema(schema) { this.slowQuerySchema = schema },
    setShowQueryHistory(val) { this.showQueryHistory = val },
    // 页面依赖的空方法（后续可完善具体逻辑）
    setLoadingSchema(val) { this.loadingSchema = val },
    refreshSchema() {},
    executeQuery() {},
    insertTemplate() {},
    formatSQL() {},
    clearEditor() { this.sqlContent = '' },
    saveQuery() {},
    toggleSlowlogPanel() { this.showSlowlogPanel = !this.showSlowlogPanel },
    handleKeyDown() {},
    handleInput() {},
    handleScroll() {},
    handleTabClose() {},
    exportData() {},
    copyResultToClipboard() {},
    deleteResult() {},
    // ConnectionManager 相关
    setActiveTab(val) {
      this.activeTab = val
    },
  }
})
