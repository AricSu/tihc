#!/bin/bash
set -e

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

# Error handling
error() {
    echo -e "${RED}${BOLD}Error:${NC} $1" >&2
    exit 1
}

# Info message
info() {
    echo -e "${BLUE}${BOLD}Info:${NC} $1"
}

# Success message
success() {
    echo -e "${GREEN}${BOLD}Success:${NC} $1"
}

# Check system requirements
[[ "$(uname)" != "Darwin" ]] && error "TIHC currently only supports MacOS"

# Check architecture
ARCH=$(uname -m)
case $ARCH in
    "x86_64"|"arm64") info "Detected architecture: $ARCH" ;;
    *) error "Unsupported architecture: $ARCH" ;;
esac

# Check network connectivity
info "Checking network connection..."
curl -s "https://api.github.com" > /dev/null || error "Unable to connect to GitHub. Please check your network"

# Create temporary directory with cleanup
TMP_DIR=$(mktemp -d) || error "Failed to create temporary directory"
trap 'rm -rf "$TMP_DIR"' EXIT

# Test connection and check latest version
echo -e "${YELLOW}Checking latest version...${NC}"
LATEST_VERSION=$(curl -sL "https://api.github.com/repos/aric-tihc/tihc/releases/latest" | grep '"tag_name":' | cut -d'"' -f4)
[[ -z "$LATEST_VERSION" ]] && error "Unable to fetch latest version information"
info "Latest version: ${BOLD}${LATEST_VERSION}${NC}"

# Download latest version
echo -e "${YELLOW}Downloading TIHC ${LATEST_VERSION}...${NC}"
DOWNLOAD_URL="https://github.com/aric-tihc/tihc/releases/download/${LATEST_VERSION}/tihc-${ARCH}"
curl -L "$DOWNLOAD_URL" -o "$TMP_DIR/tihc" --fail --progress-bar || error "Download failed"

# Verify download
info "Verifying download..."
[[ -s "$TMP_DIR/tihc" ]] || error "Downloaded file is empty"
chmod +x "$TMP_DIR/tihc" || error "Failed to set executable permissions"

# Check installation directory
if [[ ! -d "/usr/local/bin" ]]; then
    info "Creating installation directory..."
    sudo mkdir -p /usr/local/bin || error "Failed to create installation directory"
fi

# Backup existing installation
if command -v tihc &> /dev/null; then
    info "Backing up existing installation..."
    sudo mv /usr/local/bin/tihc /usr/local/bin/tihc.bak 2>/dev/null || true
fi

# Install new version
echo -e "${YELLOW}Installing new version...${NC}"
sudo mv "$TMP_DIR/tihc" /usr/local/bin/ || error "Installation failed"

# Verify installation and display version
if command -v tihc &> /dev/null; then
    success "TIHC ${LATEST_VERSION} installed successfully!"
    echo -e "\nTo get started, run: ${BOLD}tihc --help${NC}"
    echo -e "Report issues at: ${BLUE}https://github.com/aric-tihc/tihc/issues${NC}"
else
    error "Installation verification failed"
fi