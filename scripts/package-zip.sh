#!/bin/bash

# TiHC 扩展 ZIP 打包脚本（用于 Chrome Web Store 发布）
# 使用方法: ./package-zip.sh

set -e

EXTENSION_DIR="/Users/aric/Database/tihc/tihc-extension"
OUTPUT_DIR="/Users/aric/Database/tihc"
EXTENSION_NAME="tihc-extension"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")

echo "📦 开始创建 ZIP 包用于 Chrome Web Store 发布..."

# 检查扩展目录
if [[ ! -d "$EXTENSION_DIR" ]]; then
    echo "❌ 错误: 扩展目录不存在: $EXTENSION_DIR"
    exit 1
fi

# 切换到扩展目录
cd "$EXTENSION_DIR"

# 检查必要文件
required_files=("manifest.json" "background.js" "content.js" "popup.html" "popup.js" "icon.png")
for file in "${required_files[@]}"; do
    if [[ ! -f "$file" ]]; then
        echo "❌ 错误: 缺少必要文件: $file"
        exit 1
    fi
done

echo "✅ 文件检查完成"

# 创建 ZIP 包
ZIP_FILE="$OUTPUT_DIR/${EXTENSION_NAME}_v1.0_${TIMESTAMP}.zip"
echo "📦 正在创建 ZIP 包: $ZIP_FILE"

# 排除不需要的文件
zip -r "$ZIP_FILE" . \
    -x "*.DS_Store" \
    -x "*.git*" \
    -x "*.md" \
    -x "package*.sh" \
    -x "node_modules/*" \
    -x "*.log"

if [[ -f "$ZIP_FILE" ]]; then
    echo "✅ ZIP 包创建成功!"
    echo "📁 输出文件: $ZIP_FILE"
    echo ""
    echo "🌐 Chrome Web Store 发布步骤:"
    echo "   1. 访问: https://chrome.google.com/webstore/devconsole"
    echo "   2. 登录开发者账户"
    echo "   3. 点击"新增项目""
    echo "   4. 上传刚创建的 ZIP 文件"
    echo "   5. 填写扩展信息并提交审核"
    echo ""
    echo "📊 文件大小: $(du -h "$ZIP_FILE" | cut -f1)"
else
    echo "❌ ZIP 包创建失败"
    exit 1
fi