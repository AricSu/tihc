# TIHC 巡检报告设计文档


## 一、背景与目标
TIHC 巡检报告用于自动化收集、分析数据库及相关系统的健康状态、性能指标、异常信息，并生成结构化报告，便于运维、开发、管理人员快速了解系统现状和优化建议。

## 二、功能设计
输入参数

1. Metrics 信息（如 Grafana 地址、Cookie）
 - 数据库信息（如 TiDB 地址、用户、密码）
 - 巡检时间范围
 - 时区、报告目录等
 - 巡检内容

2. 基础健康检查（如存活、连接数、慢查询等）
 - 性能指标采集（如 QPS、TPS、延迟、资源利用率等）
 - 异常与告警（如错误日志、告警事件）
 - 配置与安全项检查
 - 优化建议与风险提示

3. 报告生成

 - 自动生成报告名称
 - 结构化 JSON 输出
 - 支持历史报告查询与展示

# 三、巡检报告 JSON 格式定义

```
{
  "reportId": "string",                // 唯一报告ID
  "reportName": "string",              // 报告名称
  "createTime": 1694342400,            // 创建时间（Unix时间戳，秒）
  "timeRange": [1694338800, 1694342400], // 巡检时间范围（开始/结束，Unix时间戳，秒）
  "timezone": "Asia/Shanghai",         // 时区
  "metrics": {
    "grafanaUrl": "string",
    "grafanaCookie": "string"
  },
  "tidb": {
    "dbUrl": "string",
    "dbUser": "string",
    "dbPassword": "string"
  },
  "healthStatus": "Healthy|Warning|Critical", // 总体健康状态
  "summary": "string",                // 巡检摘要
  "recommendations": "string",        // 优化建议
  "sections": [                       // 各项检查详细内容
    {
      "sectionName": "string",        // 检查项名称
      "status": "Healthy|Warning|Critical",
      "details": "string",            // 详细描述
      "metrics": {                    // 相关指标（可选）
        "qps": 123,
        "tps": 456,
        "latency_ms": 12.3,
        "connections": 100,
      },
      "recommendations": "string"     // 针对本项的建议
    }
  ]
}
```