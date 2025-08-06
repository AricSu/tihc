#!/bin/bash
# TiDB Health Check (tihc) - One-Click Install Script
# Author: Aric <ask.aric.su@gmail.com>
# Usage: curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/install.sh | bash

set -euo pipefail

# 配置
readonly GITHUB_REPO="AricSu/tihc"
readonly BINARY_NAME="tihc"
readonly INSTALL_DIR="${HOME}/.local/bin"
readonly CONFIG_DIR="${HOME}/.config/tihc"

# 颜色定义
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m'

# 临时目录
readonly TEMP_DIR=$(mktemp -d)
trap 'rm -rf "${TEMP_DIR}"' EXIT

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
    echo "=============================================="
    echo "  TiDB Health Check (tihc) Installer"
    echo "  GitHub: https://github.com/${GITHUB_REPO}"
    echo "=============================================="
    echo -e "${NC}"
}

detect_system() {
    local os=""
    local arch=""
    local package_name=""
    
    # 检测操作系统
    case "$(uname -s)" in
        Darwin) os="macos" ;;
        Linux) os="linux" ;;
        *)
            log_error "Unsupported operating system: $(uname -s)"
            log_info "Supported systems: macOS, Linux"
            exit 1
            ;;
    esac
    
    # 检测架构
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="arm64" ;;
        *)
            log_error "Unsupported architecture: $(uname -m)"
            log_info "Supported architectures: x86_64, arm64"
            exit 1
            ;;
    esac
    
    # macOS 特殊处理
    if [[ "${os}" == "macos" && "${arch}" == "arm64" ]]; then
        package_name="${BINARY_NAME}-*-macos-arm64.tar.gz"
    else
        package_name="${BINARY_NAME}-*-${os}-${arch}.tar.gz"
    fi
    
    echo "${os}:${arch}:${package_name}"
}

get_latest_version() {
    log_info "Fetching latest version information..."
    
    local api_url="https://api.github.com/repos/${GITHUB_REPO}/releases/latest"
    local version=""
    
    if command -v curl &> /dev/null; then
        version=$(curl -fsSL "${api_url}" | grep '"tag_name"' | head -1 | cut -d'"' -f4)
    elif command -v wget &> /dev/null; then
        version=$(wget -qO- "${api_url}" | grep '"tag_name"' | head -1 | cut -d'"' -f4)
    else
        log_error "Neither curl nor wget is available"
        exit 1
    fi
    
    if [[ -z "${version}" ]]; then
        log_error "Failed to get latest version"
        exit 1
    fi
    
    # 移除 'v' 前缀（如果存在）
    version=${version#v}
    echo "${version}"
}

download_release() {
    local system_info="$1"
    local version="$2"
    
    IFS=':' read -r os arch package_pattern <<< "${system_info}"
    
    log_info "Downloading ${BINARY_NAME} v${version} for ${os}-${arch}..."
    
    # 构建下载 URL
    local base_url="https://github.com/${GITHUB_REPO}/releases/download/v${version}"
    local actual_package_name=""
    
    # 根据系统确定实际的包名
    if [[ "${os}" == "macos" && "${arch}" == "arm64" ]]; then
        actual_package_name="${BINARY_NAME}-v${version}-macos-arm64.tar.gz"
    elif [[ "${os}" == "macos" && "${arch}" == "x86_64" ]]; then
        actual_package_name="${BINARY_NAME}-v${version}-macos-x86_64.tar.gz"
    elif [[ "${os}" == "linux" && "${arch}" == "arm64" ]]; then
        actual_package_name="${BINARY_NAME}-v${version}-linux-arm64.tar.gz"
    elif [[ "${os}" == "linux" && "${arch}" == "x86_64" ]]; then
        actual_package_name="${BINARY_NAME}-v${version}-linux-x86_64.tar.gz"
    else
        log_error "No pre-built binary available for ${os}-${arch}"
        log_info "Please build from source: https://github.com/${GITHUB_REPO}#building-from-source"
        exit 1
    fi
    
    local download_url="${base_url}/${actual_package_name}"
    local checksum_url="${base_url}/${actual_package_name}.sha256"
    local package_file="${TEMP_DIR}/${actual_package_name}"
    local checksum_file="${TEMP_DIR}/${actual_package_name}.sha256"
    
    # 下载包文件
    if command -v curl &> /dev/null; then
        if ! curl -fsSL -o "${package_file}" "${download_url}"; then
            log_error "Failed to download from ${download_url}"
            log_warning "The release might not be available yet or the URL might be incorrect"
            exit 1
        fi
    elif command -v wget &> /dev/null; then
        if ! wget -q -O "${package_file}" "${download_url}"; then
            log_error "Failed to download from ${download_url}"
            log_warning "The release might not be available yet or the URL might be incorrect"
            exit 1
        fi
    fi
    
    # 下载校验和文件（可选）
    if command -v curl &> /dev/null; then
        curl -fsSL -o "${checksum_file}" "${checksum_url}" 2>/dev/null || log_warning "Could not download checksum file"
    elif command -v wget &> /dev/null; then
        wget -q -O "${checksum_file}" "${checksum_url}" 2>/dev/null || log_warning "Could not download checksum file"
    fi
    
    echo "${package_file}:${checksum_file}"
}

verify_checksum() {
    local package_file="$1"
    local checksum_file="$2"
    
    if [[ ! -f "${checksum_file}" ]]; then
        log_warning "Checksum file not found, skipping verification"
        return 0
    fi
    
    log_info "Verifying package integrity..."
    
    local expected_checksum=$(cat "${checksum_file}" | cut -d' ' -f1)
    local actual_checksum=""
    
    # 计算实际校验和
    if command -v shasum &> /dev/null; then
        actual_checksum=$(shasum -a 256 "${package_file}" | cut -d' ' -f1)
    elif command -v sha256sum &> /dev/null; then
        actual_checksum=$(sha256sum "${package_file}" | cut -d' ' -f1)
    else
        log_warning "No SHA256 utility found, skipping checksum verification"
        return 0
    fi
    
    if [[ "${expected_checksum}" != "${actual_checksum}" ]]; then
        log_error "Checksum mismatch!"
        log_error "Expected: ${expected_checksum}"
        log_error "Actual:   ${actual_checksum}"
        exit 1
    fi
    
    log_success "Checksum verification passed"
}

install_binary() {
    local package_file="$1"
    
    log_info "Installing ${BINARY_NAME}..."
    
    # 创建安装目录
    mkdir -p "${INSTALL_DIR}"
    mkdir -p "${CONFIG_DIR}"
    
    # 提取包
    cd "${TEMP_DIR}"
    if ! tar -xzf "${package_file}"; then
        log_error "Failed to extract package"
        exit 1
    fi
    
    # 查找提取的目录
    local extracted_dir=""
    for dir in */; do
        if [[ -f "${dir}${BINARY_NAME}" ]]; then
            extracted_dir="${dir}"
            break
        fi
    done
    
    if [[ -z "${extracted_dir}" ]]; then
        log_error "Could not find binary in extracted package"
        exit 1
    fi
    
    # 复制二进制文件
    cp "${extracted_dir}${BINARY_NAME}" "${INSTALL_DIR}/"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
    
    # 复制配置文件示例
    if [[ -f "${extracted_dir}config.toml.example" ]]; then
        cp "${extracted_dir}config.toml.example" "${CONFIG_DIR}/"
        log_info "Config example copied to ${CONFIG_DIR}/config.toml.example"
    fi
    
    log_success "Binary installed to ${INSTALL_DIR}/${BINARY_NAME}"
}

setup_shell_integration() {
    log_info "Setting up shell integration..."
    
    # 检查 PATH 中是否包含安装目录
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        log_warning "${INSTALL_DIR} is not in your PATH"
        
        # 尝试添加到 shell 配置文件
        local shell_config=""
        local shell_name=$(basename "${SHELL:-}")
        
        case "${shell_name}" in
            bash)
                shell_config="${HOME}/.bashrc"
                [[ -f "${HOME}/.bash_profile" ]] && shell_config="${HOME}/.bash_profile"
                ;;
            zsh)
                shell_config="${HOME}/.zshrc"
                ;;
            fish)
                shell_config="${HOME}/.config/fish/config.fish"
                ;;
            *)
                log_warning "Unknown shell: ${shell_name}"
                ;;
        esac
        
        if [[ -n "${shell_config}" ]]; then
            local path_line="export PATH=\"\$PATH:${INSTALL_DIR}\""
            if [[ -f "${shell_config}" ]] && ! grep -q "${INSTALL_DIR}" "${shell_config}"; then
                echo "" >> "${shell_config}"
                echo "# Added by tihc installer" >> "${shell_config}"
                echo "${path_line}" >> "${shell_config}"
                log_info "Added ${INSTALL_DIR} to PATH in ${shell_config}"
                log_warning "Please restart your shell or run: source ${shell_config}"
            fi
        fi
    fi
}

verify_installation() {
    log_info "Verifying installation..."
    
    local binary_path="${INSTALL_DIR}/${BINARY_NAME}"
    
    if [[ ! -f "${binary_path}" ]]; then
        log_error "Binary not found at ${binary_path}"
        exit 1
    fi
    
    if [[ ! -x "${binary_path}" ]]; then
        log_error "Binary is not executable"
        exit 1
    fi
    
    # 测试运行
    local version_output=""
    if version_output=$("${binary_path}" --version 2>/dev/null); then
        log_success "Installation verified: ${version_output}"
    else
        log_warning "Binary installed but version check failed"
        log_info "This might be normal if dependencies are missing"
    fi
}

print_usage_info() {
    echo -e "${GREEN}"
    echo "=============================================="
    echo "  Installation Completed Successfully!"
    echo "=============================================="
    echo -e "${NC}"
    echo "📍 Binary location: ${INSTALL_DIR}/${BINARY_NAME}"
    echo "⚙️  Config directory: ${CONFIG_DIR}"
    echo ""
    echo "🚀 Quick start:"
    echo "   ${BINARY_NAME} --help                    # Show help"
    echo "   ${BINARY_NAME} server --port 5000       # Start web server"
    echo "   ${BINARY_NAME} tools slowlog --help     # Show slowlog help"
    echo ""
    echo "📖 Documentation: https://www.askaric.com/en/tihc"
    echo "🐛 Issues: https://github.com/${GITHUB_REPO}/issues"
    echo ""
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        echo "⚠️  Note: Please add ${INSTALL_DIR} to your PATH or restart your shell"
        echo "   Or run directly: ${INSTALL_DIR}/${BINARY_NAME}"
    fi
}

check_prerequisites() {
    log_info "Checking system prerequisites..."
    
    # 检查网络连接
    if ! command -v curl &> /dev/null && ! command -v wget &> /dev/null; then
        log_error "Neither curl nor wget is available"
        log_info "Please install curl or wget to download the binary"
        exit 1
    fi
    
    # 检查 tar
    if ! command -v tar &> /dev/null; then
        log_error "tar command not found"
        log_info "Please install tar to extract the package"
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

main() {
    print_banner
    check_prerequisites
    
    local system_info=$(detect_system)
    local version=$(get_latest_version)
    local download_result=$(download_release "${system_info}" "${version}")
    
    IFS=':' read -r package_file checksum_file <<< "${download_result}"
    
    verify_checksum "${package_file}" "${checksum_file}"
    install_binary "${package_file}"
    setup_shell_integration
    verify_installation
    print_usage_info
}

# 捕获中断信号
trap 'log_error "Installation interrupted"; exit 1' INT

# 运行主函数
main "$@"
