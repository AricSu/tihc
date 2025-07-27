### **《TiDB Intelligent Health Check (tihc) — Core 设计文档》**

---

## 1️⃣ 项目目标

`tihc` 的 **Core** 模块是整个系统的核心，负责管理插件生命周期、服务注册与发现、事件处理、以及基础服务的提供。`Core` 模块采用 **微内核架构**，核心目标是将复杂的业务逻辑与插件的实现解耦，并提供一个可扩展、灵活的平台来支持 TiDB 集群巡检、性能分析、告警推送等功能。

---

## 2️⃣ 核心架构理念

| 层次          | 描述                                                        |
| ----------- | --------------------------------------------------------- |
| **插件管理**    | **插件生命周期管理**，支持插件发现、加载、卸载、热更新等操作。                         |
| **服务注册与发现** | **ServiceRegistry** 用于管理服务及其接口注册，插件可以注册自己的服务并访问其他插件提供的服务。 |
| **事件驱动**    | 通过事件总线（**EventBus**）实现插件之间的松耦合通信，插件发布和订阅事件进行异步处理。         |
| **核心服务**    | 提供基础功能，如配置管理、日志追踪、数据库访问等，支持插件进行调用。                        |

---

## 3️⃣ 核心模块组件

### 3.1 微内核 (Microkernel)

微内核是整个系统的基础平台，负责插件的生命周期管理和插件之间的协调。`Microkernel` 处理插件的注册、调度、事件管理，并提供统一的插件接口。

```rust
/// The Microkernel is the central orchestrator of the system.
/// It manages the plugin lifecycle, service registration, and event dispatching.
pub struct Microkernel {
    pub service_registry: ServiceRegistry,
    pub event_bus: EventBus,
    pub plugin_manager: PluginManager,
}

impl Microkernel {
    pub fn new() -> Self {
        // Initialize core services and structures
        Microkernel {
            service_registry: ServiceRegistry::new(),
            event_bus: EventBus::new(),
            plugin_manager: PluginManager::new(),
        }
    }
}
```

### 3.2 插件管理 (PluginManager)

`PluginManager` 负责插件的加载、卸载以及生命周期管理。它提供插件的注册、发现与启动功能。

```rust
/// The PluginManager handles the loading and lifecycle of plugins.
/// It ensures that plugins are registered correctly and can be invoked by other parts of the system.
pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager {
            plugins: HashMap::new(),
        }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.insert(plugin.name().to_string(), plugin);
    }
    
    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.get(name)
    }
}
```

### 3.3 服务注册与发现 (ServiceRegistry)

`ServiceRegistry` 负责管理所有插件提供的服务。每个插件在注册时，将其实现的服务接口注册到 `ServiceRegistry` 中，其他插件可以通过该注册中心发现并调用所需服务。

```rust
/// The ServiceRegistry is a central registry for services exposed by plugins.
/// It allows plugins to register their services and resolve dependencies from other plugins.
pub struct ServiceRegistry {
    services: HashMap<String, Box<dyn Any>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        ServiceRegistry {
            services: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self, service: Box<T>) {
        self.services.insert(type_name::<T>().to_string(), service);
    }

    pub fn resolve<T: 'static>(&self) -> Option<&Box<T>> {
        self.services.get(&type_name::<T>().to_string()).map(|s| s.downcast_ref::<T>().unwrap())
    }
}
```

### 3.4 事件总线 (EventBus)

`EventBus` 是一个事件调度中心，负责管理系统中所有事件的发布与订阅。插件通过事件总线进行异步通信。当某个插件需要发布事件时，它可以通过 `EventBus` 将事件发送到订阅该事件的插件。

```rust
/// The EventBus handles the publishing and subscribing of events between plugins.
/// Plugins can publish events to notify other plugins of state changes or actions.
pub struct EventBus {
    subscribers: HashMap<String, Vec<Box<dyn Fn(&dyn Any)>>>,
}

impl EventBus {
    pub fn new() -> Self {
        EventBus {
            subscribers: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, event_name: &str, subscriber: Box<dyn Fn(&dyn Any)>) {
        self.subscribers.entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push(subscriber);
    }

    pub fn publish(&self, event_name: &str, event: &dyn Any) {
        if let Some(subscribers) = self.subscribers.get(event_name) {
            for subscriber in subscribers {
                subscriber(event);
            }
        }
    }
}
```

### 3.5 基础服务 (Core Services)

核心服务模块提供了多个基础功能，支持插件与核心系统的交互。包括配置管理、日志追踪、数据库连接等。

```rust
/// Core services provided by the system, such as logging, database connection, and configuration management.
pub struct CoreServices {
    pub config_service: ConfigService,
    pub logging_service: LoggingService,
    pub database_service: DatabaseService,
}

impl CoreServices {
    pub fn new() -> Self {
        CoreServices {
            config_service: ConfigService::new(),
            logging_service: LoggingService::new(),
            database_service: DatabaseService::new(),
        }
    }
}
```

---

## 4️⃣ 插件通信机制

### 4.1 插件注册与依赖注入

插件在初始化时通过 `PluginManager` 注册到系统中，同时将其提供的服务接口通过 `ServiceRegistry` 注册。在插件运行时，它可以依赖系统或其他插件提供的服务。

### 4.2 事件传播与处理

插件可以通过 `EventBus` 发布事件，其他插件可以订阅这些事件，并根据事件触发相应的操作。插件之间通过事件进行解耦，保证了系统的灵活性和扩展性。

---

## 5️⃣ 插件生命周期

插件的生命周期包括加载、初始化、运行和销毁。插件在 `PluginManager` 中注册后，可以通过事件总线与其他插件进行交互，直到生命周期结束时被销毁。

### 插件生命周期流程：

1. **插件加载：** 插件通过 `PluginManager` 被加载到系统中。
2. **插件注册：** 插件将自身提供的服务注册到 `ServiceRegistry` 中。
3. **插件运行：** 插件开始处理事件和执行其核心任务。
4. **插件销毁：** 当插件被卸载时，所有的资源都会被清理，服务将被从 `ServiceRegistry` 中移除。

---

## 6️⃣ 核心服务实现

### 6.1 配置管理服务 (ConfigService)

配置管理服务负责加载和管理系统的配置信息，支持从不同的源（如文件、环境变量等）加载配置。

```rust
/// Manages the configuration settings of the system.
/// Supports loading configurations from files or environment variables.
pub struct ConfigService {
    config: HashMap<String, String>,
}

impl ConfigService {
    pub fn new() -> Self {
        ConfigService {
            config: HashMap::new(),
        }
    }

    pub fn load_config(&mut self, path: &str) {
        // Load configuration from a file or environment variable
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
}
```

### 6.2 日志服务 (LoggingService)

日志服务负责记录系统运行中的重要信息，包括错误日志、调试日志等。它提供不同级别的日志记录功能。

```rust
/// Provides logging functionality for the system, supporting various log levels.
pub struct LoggingService;

impl LoggingService {
    pub fn new() -> Self {
        LoggingService
    }

    pub fn log_info(&self, message: &str) {
        println!("[INFO] {}", message);
    }

    pub fn log_error(&self, message: &str) {
        println!("[ERROR] {}", message);
    }
}
```

---

## 7️⃣ 未来扩展与优化

1. **插件热更新支持：** 目前的插件管理和生命周期系统支持基本的插件注册与卸载，未来可以扩展支持插件的热更新和动态加载。
2. **更丰富的事件系统：** 当前的事件总线系统基本支持插件间的通信，未来可以扩展支持事件的优先级、定时发布、事件过滤等功能。
3. **可插拔的服务支持：** 系统可以支持更灵活的服务注册机制，允许外部服务进行插件注册和发现。

---

## 8️⃣ 总结

Core 模块在 tihc 系统中扮演着至关重要的角色，负责管理插件生命周期、事件驱动通信、服务注册与发现等核心功能。通过微内核架构的设计，系统具有极高的灵活性和可扩展性，能够在未来轻松添加新功能或优化现有功能。