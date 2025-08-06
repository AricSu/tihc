#!/bin/bash
# TiDB Health Check (tihc) - Multi-platform Release Script
# Author: Aric <ask.aric.su@gmail.com>
# Description: Build and package tihc for multiple platforms

set -euo pipefail

# 颜色定义
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m'

# 项目配置
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
readonly BIN_NAME="tihc"

# 支持的目标平台
readonly TARGETS=(
    "aarch64-apple-darwin:macos-arm64" 
)

# 版本信息
readonly VERSION=$(grep -o 'version = "[^"]*"' "${PROJECT_ROOT}/cli/src/main.rs" | cut -d'"' -f2)
readonly BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
readonly GIT_COMMIT=$(git -C "${PROJECT_ROOT}" rev-parse --short HEAD 2>/dev/null || echo "unknown")

# 构建配置
readonly RELEASE_DIR="${PROJECT_ROOT}/releases"

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
    echo "  TiDB Health Check (tihc) Multi-Platform Builder"
    echo "  Version: v${VERSION}"
    echo "  Build Date: ${BUILD_DATE}"
    echo "  Git Commit: ${GIT_COMMIT}"
    echo "=================================================="
    echo -e "${NC}"
}

usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Options:
    -t, --target TARGET    Build for specific target (see list below)
    -a, --all             Build for all supported targets
    -c, --clean           Clean previous builds
    -h, --help            Show this help message

Supported targets:
EOF
    for target in "${TARGETS[@]}"; do
        IFS=':' read -r rust_target platform_name <<< "$target"
        echo "    ${platform_name} (${rust_target})"
    done
    echo ""
    echo "Examples:"
    echo "    $0 --target macos-arm64     # Build for macOS ARM64 only"
    echo "    $0 --all                    # Build for all platforms"
    echo "    $0 --clean --all           # Clean and build all"
}

check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # 检查 Rust
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo (Rust) is not installed. Please install Rust from https://rustup.rs/"
        exit 1
    fi
    
    # 检查 Node.js/yarn
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

install_rust_targets() {
    log_info "Installing required Rust targets..."
    for target in "${TARGETS[@]}"; do
        IFS=':' read -r rust_target platform_name <<< "$target"
        if ! rustup target list --installed | grep -q "${rust_target}"; then
            log_info "Installing target ${rust_target}..."
            rustup target add "${rust_target}"
        fi
    done
}

clean_builds() {
    log_info "Cleaning previous builds..."
    cd "${PROJECT_ROOT}"
    make clean
    rm -rf "${RELEASE_DIR}"
    mkdir -p "${RELEASE_DIR}"
    log_success "Clean completed"
}

build_frontend() {
    log_info "Building frontend (required for all targets)..."
    cd "${PROJECT_ROOT}"
    make dashboard
    
    if [[ ! -d "${PROJECT_ROOT}/frontend/dist" ]]; then
        log_error "Frontend build failed - dist directory not found"
        exit 1
    fi
    
    local dist_files=$(find "${PROJECT_ROOT}/frontend/dist" -type f | wc -l)
    log_success "Frontend build completed (${dist_files} files generated)"
}

build_for_target() {
    local rust_target="$1"
    local platform_name="$2"
    
    log_info "Building for ${platform_name} (${rust_target})..."
    
    # 设置交叉编译环境变量
    case "${rust_target}" in
        "x86_64-apple-darwin")
            export CC_x86_64_apple_darwin="clang"
            export CXX_x86_64_apple_darwin="clang++"
            ;;
        "aarch64-apple-darwin")
            export CC_aarch64_apple_darwin="clang"
            export CXX_aarch64_apple_darwin="clang++"
            ;;
        "x86_64-unknown-linux-gnu")
            # 可能需要安装 gcc-multilib
            export CC_x86_64_unknown_linux_gnu="gcc"
            export CXX_x86_64_unknown_linux_gnu="g++"
            ;;
        "aarch64-unknown-linux-gnu")
            # 可能需要安装交叉编译工具链
            export CC_aarch64_unknown_linux_gnu="aarch64-linux-gnu-gcc"
            export CXX_aarch64_unknown_linux_gnu="aarch64-linux-gnu-g++"
            ;;
    esac
    
    # 构建
    cd "${PROJECT_ROOT}"
    if ! cargo build --release --target "${rust_target}"; then
        log_error "Build failed for ${rust_target}"
        return 1
    fi
    
    # 验证二进制文件
    local binary_path="${PROJECT_ROOT}/target/${rust_target}/release/cli"
    if [[ ! -f "${binary_path}" ]]; then
        log_error "Binary not found at ${binary_path}"
        return 1
    fi
    
    log_success "Build completed for ${platform_name}"
    
    # 创建包
    create_package_for_target "${rust_target}" "${platform_name}" "${binary_path}"
}

create_package_for_target() {
    local rust_target="$1"
    local platform_name="$2" 
    local binary_path="$3"
    
    local package_name="${BIN_NAME}-v${VERSION}-${platform_name}"
    local package_dir="${RELEASE_DIR}/${package_name}"
    local archive_name="${package_name}.tar.gz"
    
    log_info "Creating package for ${platform_name}..."
    
    # 创建包目录
    mkdir -p "${package_dir}"
    
    # 复制二进制文件
    cp "${binary_path}" "${package_dir}/${BIN_NAME}"
    chmod +x "${package_dir}/${BIN_NAME}"
    
    # 创建配置文件示例
    create_config_example "${package_dir}"
    
    # 创建 README
    create_readme "${package_dir}" "${platform_name}"
    
    # 复制 LICENSE
    if [[ -f "${PROJECT_ROOT}/LICENSE" ]]; then
        cp "${PROJECT_ROOT}/LICENSE" "${package_dir}/"
    fi
    
    # 创建归档
    cd "${RELEASE_DIR}"
    tar -czf "${archive_name}" "${package_name}"
    
    # 计算校验和
    local sha256sum=""
    if command -v shasum &> /dev/null; then
        sha256sum=$(shasum -a 256 "${archive_name}" | cut -d' ' -f1)
    elif command -v sha256sum &> /dev/null; then
        sha256sum=$(sha256sum "${archive_name}" | cut -d' ' -f1)
    fi
    
    if [[ -n "${sha256sum}" ]]; then
        echo "${sha256sum}  ${archive_name}" > "${archive_name}.sha256"
    fi
    
    log_success "Package created: ${archive_name}"
}

create_config_example() {
    local package_dir="$1"
    cat > "${package_dir}/config.toml.example" << EOF
# TiDB Health Check (tihc) Configuration Example
# Copy this file to config.toml and modify as needed

# Logging configuration
log_level = "info"
log_file = "tihc.log"
enable_log_rotation = false

# Application settings
some_option = "default_value"
EOF
}

create_readme() {
    local package_dir="$1"
    local platform_name="$2"
    
    cat > "${package_dir}/README.md" << EOF
# TiDB Health Check (tihc) v${VERSION}

## System Information
- **Version**: ${VERSION}
- **Platform**: ${platform_name}
- **Build Date**: ${BUILD_DATE}
- **Git Commit**: ${GIT_COMMIT}

## Quick Install
\`\`\`bash
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/install.sh | bash
\`\`\`

## Manual Installation
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
}

generate_release_summary() {
    log_info "Generating release summary..."
    
    local summary_file="${RELEASE_DIR}/RELEASE_SUMMARY.md"
    cat > "${summary_file}" << EOF
# TiDB Health Check (tihc) v${VERSION} Release

**Build Date**: ${BUILD_DATE}
**Git Commit**: ${GIT_COMMIT}

## Available Packages

EOF
    
    # 列出所有包
    for archive in "${RELEASE_DIR}"/*.tar.gz; do
        if [[ -f "$archive" ]]; then
            local filename=$(basename "$archive")
            local checksum_file="${archive}.sha256"
            local filesize=$(du -h "$archive" | cut -f1)
            
            echo "### ${filename}" >> "${summary_file}"
            echo "- **Size**: ${filesize}" >> "${summary_file}"
            
            if [[ -f "$checksum_file" ]]; then
                local checksum=$(cat "$checksum_file" | cut -d' ' -f1)
                echo "- **SHA256**: \`${checksum}\`" >> "${summary_file}"
            fi
            
            echo "" >> "${summary_file}"
        fi
    done
    
    cat >> "${summary_file}" << EOF
## Installation

### Quick Install (Recommended)
\`\`\`bash
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/install.sh | bash
\`\`\`

### Manual Download
Download the appropriate package for your platform from the assets above.

## Documentation
- **Homepage**: https://www.askaric.com/en/tihc
- **GitHub**: https://github.com/AricSu/tihc
- **Issues**: https://github.com/AricSu/tihc/issues

## Support
For questions or issues, please contact: ask.aric.su@gmail.com
EOF
    
    log_success "Release summary generated: ${summary_file}"
}

main() {
    local target_filter=""
    local build_all=false
    local clean_first=false
    
    # 解析命令行参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            -t|--target)
                target_filter="$2"
                shift 2
                ;;
            -a|--all)
                build_all=true
                shift
                ;;
            -c|--clean)
                clean_first=true
                shift
                ;;
            -h|--help)
                usage
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                usage
                exit 1
                ;;
        esac
    done
    
    # 如果没有指定任何构建选项，显示帮助
    if [[ -z "${target_filter}" && "${build_all}" == false ]]; then
        usage
        exit 1
    fi
    
    print_banner
    check_prerequisites
    install_rust_targets
    
    if [[ "${clean_first}" == true ]]; then
        clean_builds
    else
        mkdir -p "${RELEASE_DIR}"
    fi
    
    build_frontend
    
    # 构建指定目标
    if [[ -n "${target_filter}" ]]; then
        local found=false
        for target in "${TARGETS[@]}"; do
            IFS=':' read -r rust_target platform_name <<< "$target"
            if [[ "${platform_name}" == "${target_filter}" ]]; then
                build_for_target "${rust_target}" "${platform_name}"
                found=true
                break
            fi
        done
        if [[ "${found}" == false ]]; then
            log_error "Target '${target_filter}' not found"
            log_info "Available targets:"
            for target in "${TARGETS[@]}"; do
                IFS=':' read -r rust_target platform_name <<< "$target"
                log_info "  ${platform_name}"
            done
            exit 1
        fi
    fi
    
    # 构建所有目标
    if [[ "${build_all}" == true ]]; then
        local failed_targets=()
        for target in "${TARGETS[@]}"; do
            IFS=':' read -r rust_target platform_name <<< "$target"
            if ! build_for_target "${rust_target}" "${platform_name}"; then
                failed_targets+=("${platform_name}")
            fi
        done
        
        if [[ ${#failed_targets[@]} -gt 0 ]]; then
            log_warning "Some targets failed to build:"
            for failed in "${failed_targets[@]}"; do
                log_warning "  - ${failed}"
            done
        fi
    fi
    
    generate_release_summary
    
    echo -e "${GREEN}"
    echo "=================================================="
    echo "  Multi-Platform Build Completed!"
    echo "=================================================="
    echo -e "${NC}"
    echo "📦 Release directory: ${RELEASE_DIR}"
    echo "📋 Summary: ${RELEASE_DIR}/RELEASE_SUMMARY.md"
    echo ""
    echo "🚀 Next steps:"
    echo "   1. Test the binaries"
    echo "   2. Upload to GitHub Releases" 
    echo "   3. Update documentation"
}

# 捕获错误
trap 'log_error "Build script failed at line $LINENO"' ERR

# 运行主函数
main "$@"
