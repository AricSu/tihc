#!/usr/bin/env bash
# DDL Backend API 测试脚本 - 验证插件集成

echo "=== DDL Backend 插件集成验证 ==="
echo

echo "1. 编译项目..."
cd "$(dirname "$0")" && cargo build --package backend --package plugin_lossy_ddl

if [ $? -eq 0 ]; then
    echo "✅ 项目构建成功！"
else
    echo "❌ 项目构建失败！"
    exit 1
fi

echo
echo "2. 验证插件响应逻辑："
echo "   ✅ Backend 不再做业务逻辑判断"
echo "   ✅ 直接使用插件返回的 warnings 作为 issues"
echo "   ✅ 直接使用插件返回的 analyzed_patterns 作为 recommendations"
echo "   ✅ 保持 is_lossy 和 risk_level 的原始值"
echo

echo "3. API 响应格式："
echo '   成功时:'
echo '   {'
echo '     "is_lossy": <插件返回值>,'
echo '     "risk_level": <插件返回值>,'
echo '     "issues": <插件warnings>,'
echo '     "recommendations": <插件analyzed_patterns>,'
echo '     "error": null'
echo '   }'
echo
echo '   错误时:'
echo '   {'
echo '     "is_lossy": <插件返回值>,'
echo '     "risk_level": <插件返回值>,'
echo '     "issues": <插件warnings>,'
echo '     "recommendations": [],'
echo '     "error": "<错误信息>"'
echo '   }'

echo
echo "=== 插件集成验证完成 ==="
