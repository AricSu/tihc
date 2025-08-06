# TiDB Health Check (tihc) - 发布和安装脚本使用指南

这个项目包含了完整的发布和一键安装系统，支持 macOS ARM64 和其他平台的自动化构建。

## 📁 脚本文件说明

### 1. `scripts/release-macos-arm.sh` - macOS ARM64 专用发布脚本
专门针对 Apple Silicon (M1/M2/M3) 芯片的 macOS 设备构建发布包。

**功能特性:**
- ✅ 自动检测系统和依赖
- ✅ 构建前端 (Vue.js) 和后端 (Rust)
- ✅ 生成 macOS ARM64 原生二进制
- ✅ 创建完整的发布包（包含配置示例和文档）
- ✅ 生成 SHA256 校验和
- ✅ 输出详细的构建信息

### 2. `scripts/build-release.sh` - 多平台发布脚本
支持为多个平台同时构建发布包。

**支持的平台:**
- macOS ARM64 (Apple Silicon)
- macOS x86_64 (Intel)
- Linux x86_64
- Linux ARM64

### 3. `scripts/install.sh` - 一键安装脚本
用户友好的一键安装脚本，支持自动检测系统并下载适合的版本。

**特性:**
- 🎯 自动检测操作系统和架构
- 📥 从 GitHub Releases 自动下载最新版本
- 🔐 自动验证文件完整性 (SHA256)
- ⚙️ 自动配置环境变量和 PATH
- 📋 创建配置文件示例

### 4. `.github/workflows/release.yml` - GitHub Actions 自动发布
当推送 tag 时自动触发多平台构建和发布。

## 🚀 使用方法

### 对于开发者 - 创建发布

#### 方法 1: 仅构建 macOS ARM64
```bash
# 运行 macOS ARM 专用构建脚本
./scripts/release-macos-arm.sh
```

#### 方法 2: 构建所有平台
```bash
# 构建所有支持的平台
./scripts/build-release.sh --all

# 或者构建特定平台
./scripts/build-release.sh --target macos-arm64
```

#### 方法 3: GitHub Actions 自动发布
```bash
# 创建并推送 tag
git tag v1.2.3
git push origin v1.2.3

# GitHub Actions 会自动构建并发布到 GitHub Releases
```

### 对于用户 - 安装软件

#### 一键安装（推荐）
```bash
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/install.sh | bash
```

#### 手动安装
1. 访问 [GitHub Releases](https://github.com/AricSu/tihc/releases)
2. 下载适合您系统的包
3. 解压并复制到系统 PATH 中

## 🛠️ 构建要求

### 开发环境要求
- **Rust**: 最新稳定版 (推荐使用 rustup)
- **Node.js**: v18+ (用于前端构建)  
- **Yarn**: 包管理器
- **Git**: 版本控制

### macOS 特定要求
- **macOS**: 11.0 (Big Sur) 或更高版本
- **Xcode Command Line Tools**: 提供 clang 编译器

### 安装开发依赖
```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 Node.js 和 Yarn (使用 Homebrew)
brew install node yarn

# 安装 Rust 目标平台
rustup target add aarch64-apple-darwin
rustup target add x86_64-apple-darwin
```

## 📋 发布流程

### 自动发布（推荐）
1. 更新版本号在 `cli/src/main.rs` 中
2. 提交更改到 main 分支
3. 创建并推送 tag:
   ```bash
   git tag v1.2.3
   git push origin v1.2.3
   ```
4. GitHub Actions 会自动构建所有平台并创建 Release

### 手动发布
1. 运行构建脚本生成发布包
2. 在 GitHub 上创建新的 Release
3. 上传生成的 `.tar.gz` 文件和 `.sha256` 文件
4. 更新 Release 说明

## 📁 发布包结构

每个发布包包含以下文件:
```
tihc-v1.2.2-macos-arm64/
├── tihc                    # 主要可执行文件
├── config.toml.example     # 配置文件示例
├── README.md              # 使用说明
└── LICENSE                # 许可证文件
```

## 🔍 验证安装

安装完成后，您可以通过以下命令验证:

```bash
# 检查版本
tihc --version

# 查看帮助
tihc --help

# 启动 Web 服务器测试
tihc server --port 5000
```

## 🛟 故障排除

### 常见问题

#### 1. 权限错误
```bash
chmod +x /path/to/tihc
```

#### 2. PATH 未设置
```bash
export PATH="$PATH:$HOME/.local/bin"
```

#### 3. 依赖缺失
确保系统安装了必要的依赖库。在 macOS 上，可能需要:
```bash
xcode-select --install
```

### 获取帮助
- **GitHub Issues**: https://github.com/AricSu/tihc/issues
- **文档**: https://www.askaric.com/en/tihc
- **邮件**: ask.aric.su@gmail.com

## 📊 构建统计

### 当前版本构建信息
- **版本**: v1.2.2
- **支持平台**: macOS ARM64, macOS x86_64, Linux x86_64, Linux ARM64
- **包大小**: ~8.5MB (压缩后)
- **构建时间**: ~2-3 分钟 (单平台)

## 🚀 下一步计划

- [ ] 添加 Windows 支持
- [ ] 添加 Docker 镜像
- [ ] 集成自动更新机制
- [ ] 添加包管理器支持 (Homebrew, apt, etc.)

---

> **注意**: 这些脚本假设项目已经正确配置了 Rust 和前端环境。请确保在运行脚本前已经成功执行过 `make all`。
