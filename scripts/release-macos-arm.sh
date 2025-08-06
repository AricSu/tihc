#!/bin/bash
# TiDB Health Check (tihc) - macOS ARM Release Script
# Author: Aric <ask.aric.su@gmail.com>
# Description: Build and package tihc for macOS ARM (Apple Silicon)

set -euo pipefail

# 颜色定义
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m' # No Color

# 项目配置
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
readonly BIN_NAME="tihc"
readonly TARGET_ARCH="aarch64-apple-darwin"

# 版本信息 - 从 main.rs 中提取版本
readonly VERSION=$(grep -o 'version = "[^"]*"' "${PROJECT_ROOT}/cli/src/main.rs" | cut -d'"' -f2)
readonly BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
readonly GIT_COMMIT=$(git -C "${PROJECT_ROOT}" rev-parse --short HEAD 2>/dev/null || echo "unknown")

# 构建配置
readonly RELEASE_DIR="${PROJECT_ROOT}/releases"
readonly PACKAGE_NAME="${BIN_NAME}-v${VERSION}-macos-arm64"
readonly ARCHIVE_NAME="${PACKAGE_NAME}.tar.gz"

log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*"
}

print_banner() {
    echo -e "${BLUE}"
    echo "=================================================="
    echo "  TiDB Health Check (tihc) Release Builder"
    echo "  Target: macOS ARM64 (Apple Silicon)"
    echo "  Version: v${VERSION}"
    echo "  Build Date: ${BUILD_DATE}"
    echo "  Git Commit: ${GIT_COMMIT}"
    echo "=================================================="
    echo -e "${NC}"
}

check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # 检查操作系统
    if [[ "$(uname)" != "Darwin" ]]; then
        log_error "This script must be run on macOS"
        exit 1
    fi
    
    # 检查架构
    if [[ "$(uname -m)" != "arm64" ]]; then
        log_warning "Building on non-ARM64 machine, cross-compilation will be used"
    fi
    
    # 检查 Rust
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo (Rust) is not installed. Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    # 检查 Rust target
    if ! rustup target list --installed | grep -q "${TARGET_ARCH}"; then
        log_info "Installing Rust target ${TARGET_ARCH}..."
        rustup target add "${TARGET_ARCH}"
    fi
    
    # 检查 Node.js/yarn (用于前端构建)
    if ! command -v yarn &> /dev/null; then
        log_error "Yarn is not installed. Please install Node.js and Yarn"
        exit 1
    fi
    
    # 检查 Git
    if ! command -v git &> /dev/null; then
        log_warning "Git is not installed, version info may be incomplete"
    fi
    
    log_success "Prerequisites check completed"
}

clean_previous_builds() {
    log_info "Cleaning previous builds..."
    cd "${PROJECT_ROOT}"
    make clean
    rm -rf "${RELEASE_DIR}/${PACKAGE_NAME}"
    rm -f "${RELEASE_DIR}/${ARCHIVE_NAME}"
    log_success "Clean completed"
}

build_frontend() {
    log_info "Building frontend..."
    cd "${PROJECT_ROOT}"
    make dashboard
    
    # 验证前端构建
    if [[ ! -d "${PROJECT_ROOT}/frontend/dist" ]]; then
        log_error "Frontend build failed - dist directory not found"
        exit 1
    fi
    
    local dist_files=$(find "${PROJECT_ROOT}/frontend/dist" -type f | wc -l)
    log_success "Frontend build completed (${dist_files} files generated)"
}

build_backend() {
    log_info "Building backend for ${TARGET_ARCH}..."
    cd "${PROJECT_ROOT}"
    
    # 设置构建环境变量
    export CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER="clang"
    export CC_aarch64_apple_darwin="clang"
    export CXX_aarch64_apple_darwin="clang++"
    
    # 构建 Rust 项目
    cargo build --release --target "${TARGET_ARCH}"
    
    # 验证二进制文件
    local binary_path="${PROJECT_ROOT}/target/${TARGET_ARCH}/release/cli"
    if [[ ! -f "${binary_path}" ]]; then
        log_error "Backend build failed - binary not found at ${binary_path}"
        exit 1
    fi
    
    # 检查二进制架构
    local arch_info=$(file "${binary_path}")
    if [[ ! "${arch_info}" =~ "arm64" ]]; then
        log_error "Binary architecture mismatch: ${arch_info}"
        exit 1
    fi
    
    log_success "Backend build completed for ${TARGET_ARCH}"
}

create_package() {
    log_info "Creating release package..."
    
    # 创建发布目录
    mkdir -p "${RELEASE_DIR}/${PACKAGE_NAME}"
    
    # 复制二进制文件
    cp "${PROJECT_ROOT}/target/${TARGET_ARCH}/release/cli" "${RELEASE_DIR}/${PACKAGE_NAME}/${BIN_NAME}"
    
    # 赋予执行权限
    chmod +x "${RELEASE_DIR}/${PACKAGE_NAME}/${BIN_NAME}"
    
    # 创建配置文件示例
    cat > "${RELEASE_DIR}/${PACKAGE_NAME}/config.toml.example" << EOF
# TiDB Health Check (tihc) Configuration Example
# Copy this file to config.toml and modify as needed

# Logging configuration
log_level = "info"
log_file = "tihc.log"
enable_log_rotation = false

# Application settings
some_option = "default_value"
EOF
    
    # 创建 README
    cat > "${RELEASE_DIR}/${PACKAGE_NAME}/README.md" << EOF
# TiDB Health Check (tihc) v${VERSION}

## System Information
- **Version**: ${VERSION}
- **Target**: macOS ARM64 (Apple Silicon)
- **Build Date**: ${BUILD_DATE}
- **Git Commit**: ${GIT_COMMIT}

## Installation

### Quick Install (Recommended)
\`\`\`bash
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/install.sh | bash
\`\`\`

### Manual Install
1. Extract this archive to a directory in your PATH
2. Make the binary executable: \`chmod +x tihc\`
3. Run: \`./tihc --help\`

## Configuration
Copy \`config.toml.example\` to \`config.toml\` and modify as needed.

## Usage

### CLI Mode
\`\`\`bash
# Import slow log
./tihc tools slowlog /path/to/tidb-slow.log --host 127.0.0.1:4000 --user root

# Start web server
./tihc server --host 0.0.0.0 --port 5000
\`\`\`

### Web Mode
Access the web interface at: http://localhost:5000

## Documentation
Visit: https://www.askaric.com/en/tihc

## Support
- GitHub: https://github.com/AricSu/tihc
- Issues: https://github.com/AricSu/tihc/issues
- Email: ask.aric.su@gmail.com
EOF
    
    # 创建 LICENSE 文件 (如果项目根目录有的话)
    if [[ -f "${PROJECT_ROOT}/LICENSE" ]]; then
        cp "${PROJECT_ROOT}/LICENSE" "${RELEASE_DIR}/${PACKAGE_NAME}/"
    fi
    
    log_success "Package structure created"
}

create_archive() {
    log_info "Creating archive..."
    cd "${RELEASE_DIR}"
    
    tar -czf "${ARCHIVE_NAME}" "${PACKAGE_NAME}"
    
    # 验证归档文件
    if [[ ! -f "${ARCHIVE_NAME}" ]]; then
        log_error "Failed to create archive"
        exit 1
    fi
    
    # 计算校验和
    local sha256sum=$(shasum -a 256 "${ARCHIVE_NAME}" | cut -d' ' -f1)
    echo "${sha256sum}  ${ARCHIVE_NAME}" > "${ARCHIVE_NAME}.sha256"
    
    log_success "Archive created: ${ARCHIVE_NAME}"
    log_info "SHA256: ${sha256sum}"
}

generate_release_info() {
    log_info "Generating release information..."
    
    local release_info="${RELEASE_DIR}/release-info.json"
    local file_size=$(stat -f%z "${RELEASE_DIR}/${ARCHIVE_NAME}" 2>/dev/null || echo "unknown")
    local sha256sum=$(cat "${RELEASE_DIR}/${ARCHIVE_NAME}.sha256" | cut -d' ' -f1)
    
    cat > "${release_info}" << EOF
{
  "version": "${VERSION}",
  "target": "macos-arm64",
  "build_date": "${BUILD_DATE}",
  "git_commit": "${GIT_COMMIT}",
  "package": {
    "name": "${ARCHIVE_NAME}",
    "size": ${file_size},
    "sha256": "${sha256sum}"
  },
  "system_requirements": {
    "os": "macOS",
    "arch": "arm64",
    "min_version": "macOS 11.0 (Big Sur)"
  },
  "install_command": "curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/install.sh | bash"
}
EOF
    
    log_success "Release information generated: ${release_info}"
}

print_summary() {
    echo -e "${GREEN}"
    echo "=================================================="
    echo "  Release Build Completed Successfully!"
    echo "=================================================="
    echo -e "${NC}"
    echo "📦 Package: ${RELEASE_DIR}/${ARCHIVE_NAME}"
    echo "🔐 Checksum: ${RELEASE_DIR}/${ARCHIVE_NAME}.sha256"
    echo "📊 Release Info: ${RELEASE_DIR}/release-info.json"
    echo ""
    echo "🚀 Next steps:"
    echo "   1. Test the binary: ${RELEASE_DIR}/${PACKAGE_NAME}/${BIN_NAME} --help"
    echo "   2. Upload to GitHub Releases"
    echo "   3. Update install script if needed"
    echo ""
    echo "📖 Documentation: https://www.askaric.com/en/tihc"
}

main() {
    print_banner
    check_prerequisites
    clean_previous_builds
    build_frontend
    build_backend
    create_package
    create_archive
    generate_release_info
    print_summary
}

# 捕获错误并清理
trap 'log_error "Build failed at line $LINENO"' ERR

# 运行主函数
main "$@"
