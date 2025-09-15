
# plugin_observability 设计文档（优化版）

## 1. 背景与目标

`plugin_observability` 是 Inspection / RCA Engine 的核心数据采集与管理模块，旨在为上层引擎提供统一、标准化的数据接口，屏蔽底层数据源差异，提升系统可观测性和可维护性。

**目标：**
- 提供统一接口访问数据库与系统相关信息
- 插件化架构，支持灵活扩展新的数据源
- 支持异步与批量数据拉取，提升性能
- 保证数据标准化、可复用、易维护

**支持数据类型：**
- 数据库相关：日志、监控指标、系统表、系统参数、进程启动参数
- Kubernetes 相关：集群 API、子系统信息（Etcd、CNI、Ingress、Kibana 等）

---

## 2. 架构总览

```text
                   +----------------------+
                   | plugin_observability |
                   |      (DataManager)   |
                   +-----------+----------+
                               |
       +-----------------------+-----------------------+
       |                       |                       |
       v                       v                       v
+------------+           +------------+           +------------+
| Metrics    |           | Logs       |           | SystemTables|
| Adapter    |           | Adapter    |           | Adapter     |
+------------+           +------------+           +------------+
       |                       |                       |
       v                       v                       v
 Prometheus/Grafana/API      Log system             DB system tables

+------------------------+
| Config / Process Adapter |
+------------------------+
| DB params, process args |
+------------------------+

+------------------------+
| K8s Adapter / Subsystem |
+------------------------+
| K8s API / Etcd / CNI   |
| Kibana / Ingress       |
+------------------------+
```

---

## 3. 核心模块设计

### 3.1 DataManager

**职责：**
- 管理 Adapter 生命周期与注册
- 提供统一数据访问接口
- 调度 Adapter 拉取数据
- 负责数据缓存、聚合和标准化

**接口定义：**
```rust
pub trait DataManager {
    fn get_metrics(&self, query: MetricQuery) -> Result<MetricsData>;
    fn get_logs(&self, filter: LogFilter) -> Result<LogData>;
    fn get_system_table(&self, query: TableQuery) -> Result<TableData>;
    fn get_config(&self, target: ConfigTarget) -> Result<ConfigData>;
    fn get_process_args(&self, target: ProcessTarget) -> Result<ProcessData>;
    fn get_k8s_resource(&self, query: K8sResourceQuery) -> Result<K8sData>;
    fn get_k8s_subsystem(&self, query: K8sSubsystemQuery) -> Result<SubsystemData>;
}
```

### 3.2 Adapter 模块

每类数据源对应一个 Adapter，负责具体的数据采集与标准化。

- **Metrics Adapter**：Prometheus / Grafana / 自定义监控系统
- **Logs Adapter**：数据库日志、应用日志、K8s 日志系统（Loki、ELK 等）
- **SystemTables Adapter**：数据库系统表
- **Config / Process Adapter**：数据库配置文件、进程启动参数
- **K8s Adapter**：Kubernetes API
- **K8s Subsystem Adapter**：Etcd / CNI / Ingress / Kibana 等

**Adapter 设计原则：**
- 标准化输出结构，便于上层统一处理
- 支持异步、批量数据拉取
- 易于扩展和注册新数据源

---

## 4. 数据流与调用流程

1. Engine 提交数据请求至 DataManager
2. DataManager 分析请求类型并调度对应 Adapter
3. Adapter 访问外部数据源并获取结果
4. Adapter 返回标准化数据给 DataManager
5. DataManager 聚合 / 缓存数据后返回给 Engine

---

## 5. 扩展性与可维护性

- **插件化架构**：新增数据源仅需实现 Adapter 接口并注册，无需修改核心逻辑
- **算法与数据解耦**：数据采集层与 RCA 分析层完全分离
- **统一数据格式**：所有 Adapter 输出标准化结构，便于聚合与分析
- **异步/批量支持**：提升大规模集群或海量数据场景下的性能

---

## 6. 性能与安全设计

- **性能优化**：支持缓存、批量请求，减少 API 调用开销；对大规模集群支持分页与异步请求
- **错误处理**：Adapter 拉取失败时返回标准化错误，DataManager 实现重试与降级机制
- **安全性**：敏感数据支持脱敏，API 调用需认证与授权
- **可观测性**：自身暴露 Prometheus 格式监控指标（API 调用次数、耗时、错误率等）

---

## 7. 未来规划

- 支持更多数据源类型（如云厂商 API、第三方监控平台）
- Adapter 热插拔与动态注册
- 更丰富的数据聚合与分析能力
- 完善的测试与文档体系
