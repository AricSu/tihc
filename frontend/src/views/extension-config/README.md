# TiHC 前端扩展集成说明

## 概述

TiHC 前端现已完全支持浏览器扩展的页面类型检测和数据采集功能。页面类型检测逻辑从浏览器扩展移动到前端，实现了更灵活的架构设计。

## 🏗️ 新架构特性

### 前端完全驱动
- ✅ **页面类型检测**: 由前端统一检测页面类型（Grafana/Clinic）
- ✅ **扩展控制**: 前端发送包含页面类型的采集指令
- ✅ **状态管理**: 前端管理采集状态和历史记录
- ✅ **用户界面**: 完整的扩展管理和采集控制界面

### 智能页面检测
- 🔍 **多维度检测**: 域名、URL、标题、DOM元素综合判断
- 🎯 **精准识别**: 支持 Grafana 和 TiDB Clinic 页面类型
- 🔄 **实时更新**: 监听页面变化自动更新检测结果

## 📁 文件结构

```
frontend/src/
├── views/extension-config/
│   └── index.vue                    # 扩展配置页面
├── composables/
│   └── useTiHCExtension.js          # 扩展管理 Composable
├── utils/
│   └── pageDetection.js             # 页面类型检测工具
└── ...

frontend/public/
└── extension-test.html              # 扩展功能测试页面
```

## 🚀 使用方法

### 1. 在组件中使用扩展功能

```vue
<template>
  <div>
    <h2>扩展状态: {{ extensionInstalled ? '已安装' : '未安装' }}</h2>
    <p>页面类型: {{ pageInfo.type }}</p>
    <button @click="handleStartCollection" :disabled="!canCollect">
      开始采集
    </button>
  </div>
</template>

<script setup>
import { useTiHCExtension } from '@/composables/useTiHCExtension.js'

const {
  extensionInstalled,
  pageInfo,
  canCollect,
  startCollection,
  initialize,
  cleanup
} = useTiHCExtension()

async function handleStartCollection() {
  try {
    const result = await startCollection()
    console.log('采集成功:', result)
  } catch (error) {
    console.error('采集失败:', error.message)
  }
}

onMounted(() => {
  initialize()
})

onUnmounted(() => {
  cleanup()
})
</script>
```

### 2. 直接使用页面检测工具

```javascript
import { 
  detectPageType, 
  getPageCollectionInfo,
  watchPageChanges 
} from '@/utils/pageDetection.js'

// 检测当前页面类型
const pageType = detectPageType() // 'grafana' | 'clinic' | 'unknown'

// 获取完整页面信息
const pageInfo = getPageCollectionInfo()
console.log(pageInfo) // { pageType, displayName, supported, url, domain, title }

// 监听页面变化
const stopWatching = watchPageChanges((pageInfo) => {
  console.log('页面变化:', pageInfo)
})

// 停止监听
stopWatching()
```

## 🔧 技术实现

### 页面类型检测逻辑

```javascript
function detectPageType() {
  const domain = window.location.hostname.toLowerCase()
  const url = window.location.href.toLowerCase()  
  const title = document.title.toLowerCase()
  
  // Grafana 检测
  if (domain.includes('grafana') || 
      url.includes('grafana') ||
      title.includes('grafana') ||
      document.querySelector('[data-grafana-version]')) {
    return 'grafana'
  }
  
  // TiDB Clinic 检测
  if (domain.includes('clinic') || 
      domain.includes('tidb') ||
      title.includes('clinic')) {
    return 'clinic'
  }
  
  return 'unknown'
}
```

### 前端 ↔ 扩展通信协议

```javascript
// 1. 扩展检测
window.postMessage({ type: 'TIHC_EXTENSION_CHECK' }, '*')

// 2. 扩展响应
window.postMessage({
  type: 'TIHC_EXTENSION_RESPONSE',
  installed: true,
  version: '1.0'
}, '*')

// 3. 开始采集 (包含页面类型)
window.postMessage({
  type: 'TIHC_START_COLLECTION',
  pageType: 'grafana', // 前端检测的页面类型
  config: {
    backendUrl: 'http://localhost:3000'
  }
}, '*')

// 4. 采集成功
window.postMessage({
  type: 'TIHC_COLLECTION_SUCCESS',
  data: {
    pageType: 'grafana',
    domain: 'grafana.example.com',
    timestamp: Date.now()
  }
}, '*')
```

## 🧪 测试功能

### 使用测试页面
访问 `http://localhost:3000/extension-test.html` 进行完整的功能测试：

1. **扩展检测测试**: 验证扩展安装状态和通信
2. **页面类型检测**: 模拟不同页面类型进行测试  
3. **数据采集测试**: 完整的采集流程测试
4. **实时日志**: 查看详细的执行日志

### 测试步骤
1. 安装浏览器扩展
2. 启动 TiHC 前端服务
3. 访问测试页面
4. 依次执行各项测试功能

## 📋 API 参考

### useTiHCExtension Composable

#### 状态属性
- `checking` - 是否正在检测扩展
- `extensionInstalled` - 扩展是否已安装
- `extensionVersion` - 扩展版本号
- `collecting` - 是否正在采集数据
- `collectionStatus` - 采集状态描述
- `pageInfo` - 当前页面信息
- `collectionHistory` - 采集历史记录

#### 计算属性
- `isExtensionReady` - 扩展是否就绪
- `canCollect` - 是否可以执行采集
- `pageTypeDisplay` - 页面类型显示名称
- `pageTypeTag` - 页面类型标签样式

#### 方法
- `checkExtension()` - 检测扩展状态
- `startCollection()` - 开始数据采集
- `stopCollection()` - 停止数据采集
- `updatePageInfo()` - 更新页面信息
- `initialize()` - 初始化
- `cleanup()` - 清理资源

### 页面检测工具函数

- `detectPageType(url?, domain?, title?)` - 检测页面类型
- `getPageTypeDisplayName(pageType)` - 获取显示名称
- `getPageTypeTagType(pageType)` - 获取标签样式
- `isPageSupportedForCollection(pageType?)` - 是否支持采集
- `getPageCollectionInfo()` - 获取完整页面信息
- `watchPageChanges(callback)` - 监听页面变化

## 🔍 故障排除

### 常见问题

1. **扩展检测失败**
   - 确认扩展已正确安装并启用
   - 检查浏览器控制台是否有错误信息
   - 尝试手动重新检测

2. **页面类型识别错误**  
   - 检查页面是否包含特征元素
   - 查看控制台的检测日志
   - 使用测试页面验证检测逻辑

3. **采集指令无响应**
   - 确认扩展与前端的通信正常
   - 检查后端服务是否正常运行
   - 查看网络请求是否成功

### 调试技巧

- 打开浏览器开发者工具查看控制台日志
- 使用 `extension-test.html` 页面进行功能验证
- 检查扩展的 popup 界面显示的状态信息

## 📈 后续规划

- [ ] 支持更多页面类型检测
- [ ] 添加采集数据的本地预览功能
- [ ] 实现采集任务的批量管理
- [ ] 优化页面变化监听性能
- [ ] 添加采集数据的统计分析

---

*TiHC 前端扩展集成 - 让数据采集更智能、更灵活* 🚀