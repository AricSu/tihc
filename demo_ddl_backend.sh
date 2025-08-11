#!/usr/bin/env bash
# DDL Backend API 演示脚本

echo "=== DDL Backend API 演示 ==="
echo

# 构建项目
echo "1. 构建 backend 项目..."
cd "$(dirname "$0")" && cargo build --package backend --release

if [ $? -eq 0 ]; then
    echo "✅ Backend 构建成功！"
else
    echo "❌ Backend 构建失败！"
    exit 1
fi

echo
echo "2. DDL 预检查功能已集成到 backend："
echo "   - API 路径: /ddl/precheck"
echo "   - 方法: POST"
echo "   - 支持 TiDB 引擎分析"
echo "   - 风险评估：Safe/High"
echo

echo "3. 请求示例："
echo '   POST /ddl/precheck'
echo '   Content-Type: application/json'
echo '   {'
echo '     "sql": "DROP TABLE users",'
echo '     "collation_enabled": true'
echo '   }'
echo

echo "4. 响应示例："
echo '   {'
echo '     "is_lossy": true,'
echo '     "risk_level": "High",'
echo '     "issues": ["检测到潜在的数据丢失风险"],'
echo '     "recommendations": ["请确保已备份重要数据"]'
echo '   }'

echo
echo "=== DDL Backend 集成完成 ==="
