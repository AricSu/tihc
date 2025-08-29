#!/bin/bash

# TiDB Health Check (tihc) Installation Script
# GitHub: https://github.com/AricSu/tihc

set -euo pipefail

# 配置
readonly GITHUB_REPO="AricSu/tihc"
readonly BINARY_NAME="tihc"
readonly TEMP_DIR=$(mktemp -d)
readonly INSTALL_DIR="${HOME}/.local/bin"

# 颜色定义
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly NC='\033[0m' # No Color

# 日志函数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" >&2
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" >&2
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1" >&2
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# 清理函数
cleanup() {
    if [[ -d "${TEMP_DIR}" ]]; then
        rm -rf "${TEMP_DIR}"
    fi
}
trap cleanup EXIT

# 检查系统先决条件
check_prerequisites() {
    log_info "Checking system prerequisites..."
    
    # 检查是否有 curl 或 wget
    if ! command -v curl &> /dev/null && ! command -v wget &> /dev/null; then
        log_error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi
    
    # 检查 tar
    if ! command -v tar &> /dev/null; then
        log_error "tar is not available. Please install tar."
        exit 1
    fi
    
    # 检查校验和工具
    if ! command -v shasum &> /dev/null && ! command -v sha256sum &> /dev/null; then
        log_warning "Neither shasum nor sha256sum is available. Checksum verification will be skipped."
    fi
    
    log_success "Prerequisites check passed"
}

# 获取系统信息
get_system_info() {
    local os=""
    local arch=""
    case "$(uname -s)" in
        Linux*) os="linux" ;;
        Darwin*) os="macos" ;;
        MINGW*|MSYS*|CYGWIN*) os="windows" ;;
        *)
            log_error "Unsupported operating system: $(uname -s)"
            log_info "Supported systems: Linux, macOS, Windows"
            exit 1
            ;;
    esac
    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="arm64" ;;
        *)
            log_error "Unsupported architecture: $(uname -m)"
            log_info "Supported architectures: x86_64, arm64"
            exit 1
            ;;
    esac
    echo "${os}:${arch}"
}

# 检查已安装的版本
check_installed_version() {
    local binary_path="${INSTALL_DIR}/${BINARY_NAME}"
    
    if [[ -f "${binary_path}" && -x "${binary_path}" ]]; then
        local installed_version=""
        installed_version=$(${binary_path} --version 2>/dev/null | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+' | head -1 || echo "")
        
        if [[ -n "${installed_version}" ]]; then
            echo "${installed_version}"
        else
            echo ""
        fi
    else
        echo ""
    fi
}

# 清理旧版本安装
cleanup_old_installations() {
    log_info "Checking for existing installations..."
    
    local old_locations=(
        "/usr/local/bin/${BINARY_NAME}"
        "/usr/bin/${BINARY_NAME}"
        "${HOME}/bin/${BINARY_NAME}"
    )
    
    local found_old=false
    
    for old_path in "${old_locations[@]}"; do
        if [[ -f "${old_path}" ]]; then
            log_warning "Found existing installation at ${old_path}"
            found_old=true
            
            # 尝试删除，可能需要sudo权限
            if [[ "${old_path}" == "/usr/local/bin/"* || "${old_path}" == "/usr/bin/"* ]]; then
                if command -v sudo &> /dev/null; then
                    log_info "Removing old installation at ${old_path} (requires admin privileges)..."
                    if sudo rm -f "${old_path}"; then
                        log_success "Removed old installation from ${old_path}"
                    else
                        log_warning "Failed to remove ${old_path}, you may need to remove it manually"
                    fi
                else
                    log_warning "Cannot remove ${old_path} (no sudo available), you may need to remove it manually"
                fi
            else
                log_info "Removing old installation at ${old_path}..."
                if rm -f "${old_path}"; then
                    log_success "Removed old installation from ${old_path}"
                else
                    log_warning "Failed to remove ${old_path}"
                fi
            fi
        fi
    done
    
    if ! $found_old; then
        log_info "No conflicting installations found"
    fi
}

# 比较版本号 (version1 > version2 返回 0)
version_greater_than() {
    local version1="$1"
    local version2="$2"
    
    # 使用sort -V进行版本比较
    if command -v sort &> /dev/null; then
        local highest=$(printf '%s\n%s\n' "$version1" "$version2" | sort -V | tail -1)
        [[ "$version1" == "$highest" && "$version1" != "$version2" ]]
    else
        # 如果没有sort -V，使用简单的字符串比较
        [[ "$version1" > "$version2" ]]
    fi
}

# 获取最新版本
get_latest_version() {
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

# 下载发布版本
download_release() {
    local system_info="$1"
    local version="$2"
    IFS=':' read -r os arch <<< "${system_info}"
    log_info "Downloading ${BINARY_NAME} v${version} for ${os}-${arch}..."
    local base_url="https://github.com/${GITHUB_REPO}/releases/download/v${version}"
    local package_name=""
    local checksum_name=""
    if [[ "${os}" == "macos" ]]; then
        package_name="${BINARY_NAME}-v${version}-macos.tar.gz"
        checksum_name="${BINARY_NAME}-v${version}-macos.tar.gz.sha256"
    elif [[ "${os}" == "linux" ]]; then
        package_name="${BINARY_NAME}-v${version}-linux.tar.gz"
        checksum_name="${BINARY_NAME}-v${version}-linux.tar.gz.sha256"
    elif [[ "${os}" == "windows" ]]; then
        package_name="${BINARY_NAME}-v${version}-windows.zip"
        checksum_name="${BINARY_NAME}-v${version}-windows.zip.sha256"
    else
        log_error "Unsupported OS for download: ${os}"
        exit 1
    fi
    local download_url="${base_url}/${package_name}"
    local checksum_url="${base_url}/${checksum_name}"
    local package_file="${TEMP_DIR}/${package_name}"
    local checksum_file="${TEMP_DIR}/${checksum_name}"
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
        curl -fsSL -o "${checksum_file}" "${checksum_url}" 2>/dev/null || true
    elif command -v wget &> /dev/null; then
        wget -q -O "${checksum_file}" "${checksum_url}" 2>/dev/null || true
    fi
    # 验证校验和
    if [[ -f "${checksum_file}" ]]; then
        log_info "Verifying package checksum..."
        local expected_checksum
        expected_checksum=$(cat "${checksum_file}" | cut -d' ' -f1)
        local actual_checksum=""
        if command -v shasum &> /dev/null; then
            actual_checksum=$(shasum -a 256 "${package_file}" | cut -d' ' -f1)
        elif command -v sha256sum &> /dev/null; then
            actual_checksum=$(sha256sum "${package_file}" | cut -d' ' -f1)
        fi
        if [[ -n "${actual_checksum}" ]]; then
            if [[ "${expected_checksum}" == "${actual_checksum}" ]]; then
                log_success "Checksum verification passed"
            else
                log_error "Checksum verification failed"
                log_error "Expected: ${expected_checksum}"
                log_error "Actual: ${actual_checksum}"
                exit 1
            fi
        else
            log_warning "Could not verify checksum (no checksum tool available)"
        fi
    else
        log_warning "Checksum file not found, skipping verification"
    fi
    echo "${package_file}:${os}"
}

# 安装二进制文件
install_binary() {
    local package_and_os="$1"
    local package_file="${package_and_os%%:*}"
    local os="${package_and_os##*:}"
    log_info "Installing ${BINARY_NAME}..."
    mkdir -p "${INSTALL_DIR}"
    local binary_path=""
    if [[ "${os}" == "windows" ]]; then
        if ! command -v unzip &> /dev/null; then
            log_error "unzip is required to extract Windows zip package. Please install unzip."
            exit 1
        fi
        unzip -o "${package_file}" -d "${TEMP_DIR}"
        binary_path=$(find "${TEMP_DIR}" -name "${BINARY_NAME}.exe" -type f | head -1)
        if [[ -z "${binary_path}" || ! -f "${binary_path}" ]]; then
            log_error "Binary file not found in package (expected .exe)"
            exit 1
        fi
        cp "${binary_path}" "${INSTALL_DIR}/${BINARY_NAME}.exe"
        chmod +x "${INSTALL_DIR}/${BINARY_NAME}.exe"
        log_success "Binary installed to ${INSTALL_DIR}/${BINARY_NAME}.exe"
    else
        if ! tar -xzf "${package_file}" -C "${TEMP_DIR}"; then
            log_error "Failed to extract package"
            exit 1
        fi
        binary_path=$(find "${TEMP_DIR}" -name "${BINARY_NAME}" -type f | head -1)
        if [[ -z "${binary_path}" || ! -f "${binary_path}" ]]; then
            log_error "Binary file not found in package"
            exit 1
        fi
        cp "${binary_path}" "${INSTALL_DIR}/${BINARY_NAME}"
        chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
        log_success "Binary installed to ${INSTALL_DIR}/${BINARY_NAME}"
    fi
}

# 配置 PATH
setup_path() {
    log_info "Configuring PATH..."
    
    local shell_rc=""
    local shell_name=""
    shell_name=$(basename "${SHELL}")
    
    case "${shell_name}" in
        bash)
            if [[ -f "${HOME}/.bashrc" ]]; then
                shell_rc="${HOME}/.bashrc"
            elif [[ -f "${HOME}/.bash_profile" ]]; then
                shell_rc="${HOME}/.bash_profile"
            fi
            ;;
        zsh)
            shell_rc="${HOME}/.zshrc"
            ;;
        fish)
            shell_rc="${HOME}/.config/fish/config.fish"
            ;;
    esac
    
    if [[ -n "${shell_rc}" ]]; then
        # 检查是否已经在 PATH 中
        if ! echo "${PATH}" | grep -q "${INSTALL_DIR}"; then
            # 添加到 shell 配置文件
            echo '' >> "${shell_rc}"
            echo '# TiHC installation' >> "${shell_rc}"
            if [[ "${shell_name}" == "fish" ]]; then
                echo "set -gx PATH \$PATH ${INSTALL_DIR}" >> "${shell_rc}"
            else
                echo "export PATH=\"\$PATH:${INSTALL_DIR}\"" >> "${shell_rc}"
            fi
            log_success "Added ${INSTALL_DIR} to PATH in ${shell_rc}"
        else
            log_info "${INSTALL_DIR} is already in PATH"
        fi
    else
        log_warning "Could not determine shell configuration file"
    fi
}

# 验证安装
verify_installation() {
    log_info "Verifying installation..."
    
    local binary_path="${INSTALL_DIR}/${BINARY_NAME}"
    local binary_path_win="${INSTALL_DIR}/${BINARY_NAME}.exe"
    if [[ -f "${binary_path}" && -x "${binary_path}" ]]; then
        log_success "Installation completed successfully!"
        echo
        echo "Binary location: ${binary_path}"
        if echo "${PATH}" | grep -q "${INSTALL_DIR}" || [[ -x "${binary_path}" ]]; then
            echo "Version: $(${binary_path} --version 2>/dev/null || echo 'Version info not available')"
        fi
        echo
        echo "To use ${BINARY_NAME}, either:"
        echo "  1. Restart your shell/terminal"
        echo "  2. Or run: source ~/.$(basename "${SHELL}")rc"
        echo "  3. Or use the full path: ${binary_path}"
        echo
        echo "For help, run: ${BINARY_NAME} --help"
    elif [[ -f "${binary_path_win}" && -x "${binary_path_win}" ]]; then
        log_success "Installation completed successfully!"
        echo
        echo "Binary location: ${binary_path_win}"
        if echo "${PATH}" | grep -q "${INSTALL_DIR}" || [[ -x "${binary_path_win}" ]]; then
            echo "Version: $(${binary_path_win} --version 2>/dev/null || echo 'Version info not available')"
        fi
        echo
        echo "To use ${BINARY_NAME}.exe, either:"
        echo "  1. Restart your shell/terminal"
        echo "  2. Or run: source ~/.$(basename "${SHELL}")rc"
        echo "  3. Or use the full path: ${binary_path_win}"
        echo
        echo "For help, run: ${BINARY_NAME}.exe --help"
    else
        log_error "Installation verification failed"
        exit 1
    fi
}

# 主函数
main() {
    echo "=============================================="
    echo "  TiDB Health Check (${BINARY_NAME}) Installer"
    echo "  GitHub: https://github.com/${GITHUB_REPO}"
    echo "=============================================="
    echo
    
    check_prerequisites
    
    # 清理旧版本安装
    cleanup_old_installations
    
    local system_info
    system_info=$(get_system_info)
    
    # 检查已安装的版本
    local installed_version
    installed_version=$(check_installed_version)
    
    log_info "Fetching latest version information..."
    local version
    version=$(get_latest_version)
    
    # 版本比较和决策
    if [[ -n "${installed_version}" ]]; then
        log_info "Found existing installation: v${installed_version}"
        log_info "Latest available version: v${version}"
        
        if [[ "${installed_version}" == "${version}" ]]; then
            log_info "Already running the latest version (v${version})"
            log_info "Reinstalling to ensure clean installation..."
        elif version_greater_than "${version}" "${installed_version}"; then
            log_info "Upgrading from v${installed_version} to v${version}..."
        else
            log_info "Downgrading from v${installed_version} to v${version}..."
        fi
    else
        log_info "Installing ${BINARY_NAME} v${version}..."
    fi
    
    local package_and_os
    package_and_os=$(download_release "${system_info}" "${version}")
    install_binary "${package_and_os}"
    setup_path
    verify_installation
}

# 运行主函数
main "$@"
