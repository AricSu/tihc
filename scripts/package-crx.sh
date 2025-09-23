#!/bin/bash

# TiHC 扩展 CRX 打包脚本（用于本地分发）
# 使用方法: ./package-crx.sh

set -e

EXTENSION_DIR="/Users/aric/Database/tihc/tihc-extension"
OUTPUT_DIR="/Users/aric/Database/tihc"
EXTENSION_NAME="tihc-extension"

echo "📦 开始创建 CRX 包用于本地分发..."

# 检查 Chrome 是否安装
if [[ ! -f "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" ]]; then
    echo "❌ 错误: 未找到 Google Chrome，请先安装 Chrome 浏览器"
    exit 1
fi

# 检查扩展目录
if [[ ! -d "$EXTENSION_DIR" ]]; then
    echo "❌ 错误: 扩展目录不存在: $EXTENSION_DIR"
    exit 1
fi

# 检查必要文件
required_files=("manifest.json" "background.js" "content.js" "popup.html" "popup.js" "icon.png")
for file in "${required_files[@]}"; do
    if [[ ! -f "$EXTENSION_DIR/$file" ]]; then
        echo "❌ 错误: 缺少必要文件: $file"
        exit 1
    fi
done

echo "✅ 文件检查完成"

# 切换到输出目录
cd "$OUTPUT_DIR"

# 检查是否存在私钥文件
KEY_FILE="$OUTPUT_DIR/$EXTENSION_NAME.pem"
if [[ -f "$KEY_FILE" ]]; then
    echo "🔑 使用现有私钥文件进行打包..."
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
        --pack-extension="$EXTENSION_DIR" \
        --pack-extension-key="$KEY_FILE"
else
    echo "🆕 首次打包，将生成新的私钥文件..."
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
        --pack-extension="$EXTENSION_DIR"
fi

# 检查打包结果
CRX_FILE="$OUTPUT_DIR/$EXTENSION_NAME.crx"
PEM_FILE="$OUTPUT_DIR/$EXTENSION_NAME.pem"

if [[ -f "$CRX_FILE" ]]; then
    echo "✅ CRX 打包成功!"
    echo "📁 输出文件:"
    echo "   - 扩展包: $CRX_FILE"
    echo "   - 私钥文件: $PEM_FILE"
    echo ""
    echo "🔒 重要提醒:"
    echo "   - 请妥善保管 .pem 文件，用于后续版本更新"
    echo "   - 可以将 .crx 文件分发给用户安装"
    echo ""
    echo "⚠️  安装限制说明:"
    echo "   现代 Chrome 版本对非商店 CRX 有安全限制"
    echo "   推荐使用开发者模式加载解压扩展:"
    echo "   1. 打开 chrome://extensions/"
    echo "   2. 启用"开发者模式""
    echo "   3. 点击"加载已解压的扩展程序""
    echo "   4. 选择扩展文件夹: $EXTENSION_DIR"
    echo ""
    echo "📊 文件大小: $(du -h "$CRX_FILE" | cut -f1)"
else
    echo "❌ 打包失败，请检查错误信息"
    exit 1
fi