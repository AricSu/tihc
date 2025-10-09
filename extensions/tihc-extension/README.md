# TiHC 浏览器扩展

TiHC 浏览器扩展，专为高效采集 Web 页面认证信息而设计的被动数据采集工具。

## 🎯 核心功能

- 🎮 **前端完全驱动**: 由 TiHC 前端检测页面类型并发送采集指令，扩展专注执行
- 📊 **精准数据采集**: 采集 Cookies、localStorage、sessionStorage 等认证信息
- 🔄 **直接数据传输**: 数据直接发送到 TiHC 后端，实时处理
- 📱 **零配置体验**: 无需用户配置，完全由前端系统控制

## 🏗️ 设计架构

### 三个核心组件

**🖥️ TiHC 前端**: 检测页面类型 → 发送采集指令（含页面类型） → 控制采集流程
**🔌 浏览器扩展**: 接收指令 → 执行数据采集 → 发送到后端  
**⚙️ TiHC 后端**: 提供 `/api/extension/collect` 接口 → 接收数据 → 处理并记录日志

### 工作流程
```
1. 用户访问 TiHC 前端 → 自动检测扩展安装状态
2. 前端分析当前页面 → 确定页面类型（grafana/clinic/其他）
3. 前端发送采集指令 → 包含页面类型和后端地址
4. 扩展接收指令并执行采集 → 数据直接发送到后端
```

## 🚀 使用方法

### 1. 安装扩展
```bash
1. 打开 Chrome 浏览器
2. 进入 chrome://extensions/
3. 开启"开发者模式"
4. 点击"加载已解压的扩展程序"
5. 选择 tihc-extension 文件夹
```

### 2. 启动系统
```bash
# 启动 TiHC 后端
cargo run

# 启动 TiHC 前端  
yarn dev
```

### 3. 开始使用
1. **访问 TiHC 前端** - 系统自动检测扩展安装状态
2. **前端分析页面** - 检测当前页面类型（Grafana/Clinic/其他）
3. **发送采集指令** - 前端向扩展发送包含页面类型的采集指令
4. **扩展执行采集** - 根据页面类型采集对应的认证数据
5. **查看采集结果** - 后端日志或前端反馈

## 🔧 技术细节

### 前端控制页面检测
```javascript
// TiHC 前端检测页面类型
function detectPageType() {
  const domain = window.location.hostname.toLowerCase();
  const title = document.title.toLowerCase();
  const url = window.location.href.toLowerCase();
  
  // Grafana 检测逻辑
  if (domain.includes('grafana') || 
      url.includes('grafana') ||
      title.includes('grafana') ||
      document.querySelector('[data-grafana-version]')) {
    return 'grafana';
  }
  
  // TiDB Clinic 检测逻辑
  if (domain.includes('clinic') || 
      domain.includes('tidb') ||
      title.includes('clinic')) {
    return 'clinic';
  }
  
  return 'unknown';
}
```

### 前端 → 扩展通信
```javascript
// 前端发送采集指令（包含页面类型）
window.postMessage({
  type: 'TIHC_START_COLLECTION',
  pageType: 'grafana', // 由前端检测确定
  config: {
    backendUrl: 'http://localhost:3000'
  }
}, '*');

// 扩展返回采集结果
window.postMessage({
  type: 'TIHC_COLLECTION_SUCCESS',
  data: {
    domain: 'grafana.example.com',
    pageType: 'grafana',
    timestamp: Date.now()
  }
}, '*');
```

### 数据采集与发送
```javascript
// 接收前端传入的页面类型并采集数据
async function handleCollectionRequest(config, pageType) {
  // pageType 由前端传入，如 'grafana' | 'clinic'
  const collectedData = {
    cookies: document.cookie,
    page_type: pageType, // 使用前端传入的页面类型
    user_agent: navigator.userAgent,
    local_storage: {}, // 只采集认证相关的键
    session_storage: {} // 只采集认证相关的键
  };

  // 直接发送到后端
  await fetch(`${config.backendUrl}/api/collect`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      url: location.href,
      domain: location.hostname,
      timestamp: Date.now(),
      ...collectedData
    })
  });
}
```

### 后端数据处理
```rust
// 接收并处理采集数据
#[derive(Deserialize)]
struct CollectDataPayload {
    url: String,
    domain: String,
    cookies: String,
    page_type: String,
    user_agent: String,
    // ... 其他字段
}

#[post("/api/collect")]
async fn collect_data(payload: Json<CollectDataPayload>) -> Json<CollectDataResponse> {
    info!("收到 {} 页面采集数据: {}", payload.page_type, payload.domain);
    info!("Cookies 数量: {}", payload.cookies.split(';').count());
    
    // 处理和存储数据...
    
    Json(CollectDataResponse {
        success: true,
        message: "数据采集成功".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
    })
}
```

## 📋 扩展界面

扩展 popup 界面纯展示，无需用户配置：

- **当前页面信息**: 显示域名信息
- **扩展状态**: 运行状态、采集状态、最近采集时间  
- **采集统计**: 总采集次数、涉及域名数量

*注：页面类型检测完全由 TiHC 前端负责，扩展界面不再显示页面类型信息*

## 🔒 隐私与安全

- ✅ **按需采集**: 只在识别的目标页面采集数据
- ✅ **精准提取**: 仅采集认证相关的关键信息  
- ✅ **直接传输**: 数据直接发送到指定后端，不经过第三方
- ✅ **透明展示**: popup 界面显示所有采集活动

## ⚡ 特性优势

| 特性 | 传统方式 | TiHC 扩展 |
|------|----------|-----------|
| 页面检测 | 扩展内置检测逻辑 | ✅ 前端统一检测，更灵活 |
| 配置复杂度 | 需要手动配置域名 | ✅ 零配置，前端完全控制 |
| 扩展性 | 硬编码规则 | ✅ 前端驱动，动态控制 |
| 用户体验 | 多步骤配置 | ✅ 一键启用 |
| 维护成本 | 需更新扩展规则 | ✅ 前端统一管理检测逻辑 |

---
*由 TiHC 团队开发，专为高效采集和分析而设计* 🚀
