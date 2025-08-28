# [PRD] - TiDB Intelligent Health Check (tihc)

## 1. 产品定位
TiDB 专用的智能选件与风险分析的 “高效” 运维诊断工具，支持 CLI/Web 双模式，插件化架构。

## 2. 用户场景
- DBA 定期巡检 TiDB 集群，自动生成健康报告
- 技术支持远程诊断与报告交付

## 3. 功能需求
### 3.1 核心功能
- 慢日志批量导入与分析
- DDL 风险检测（如损失性 DDL、表结构变更风险）
- SQL 质量检查与建议
- 健康巡检报告导出（HTML/Markdown/JSON）
- Web Dashboard 可视化
- CLI 命令行操作
- 插件扩展（慢日志、DDL、SQL 编辑器等）

### 3.2 非功能需求
- 性能：支持百万级慢日志分析
- 安全：敏感信息脱敏、最小权限原则
- 易用性：CLI/Web 统一体验，文档完善
- 可扩展性：插件化架构

### 3.3 兼容性
- Linux（musl 静态、glibc 2.28+）
- Windows 10/11 x64
- macOS 11+ (Intel/Apple Silicon)

## 4. 交互与界面
- CLI 命令结构与参数说明
- Web Dashboard 页面结构与主要交互流程

## 5. 交付物
- tihc CLI 可执行文件
- 用户手册与开发文档

## 6. 里程碑
- v1.0：CLI/慢日志/DDL 检查/基础插件
- v1.1：Web Dashboard、健康报告
- v1.2+：更多插件、企业特性、云原生支持
