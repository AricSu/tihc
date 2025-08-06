#!/bin/bash

# TiHC Universal Auto Download Script
# Automatically detects platform and downloads the latest release
# Supports Linux, macOS, and Windows (via WSL)

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# GitHub repository
REPO="AricSu/tihc"
GITHUB_API="https://api.github.com/repos/${REPO}"

# Default installation directory
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Script version
SCRIPT_VERSION="1.1.0"

# Print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_highlight() {
    echo -e "${CYAN}[HIGHLIGHT]${NC} $1"
}

# Check if running in Windows environment
is_windows() {
    [[ "$(uname -s)" == CYGWIN* ]] || [[ "$(uname -s)" == MINGW* ]] || [[ "$(uname -s)" == MSYS* ]] || [[ -n "$WSL_DISTRO_NAME" ]]
}

# Detect platform
detect_platform() {
    local os arch platform

    # Check if this is Windows/WSL environment
    if is_windows; then
        print_warning "Windows environment detected."
        
        if [[ -n "$WSL_DISTRO_NAME" ]]; then
            print_status "Running in WSL (Windows Subsystem for Linux): $WSL_DISTRO_NAME"
            print_status "Using Linux binary for WSL environment."
            os="linux"
        else
            print_error "Native Windows is not currently supported."
            print_status "Supported options:"
            echo "  1. Use WSL (Windows Subsystem for Linux)"
            echo "  2. Visit https://github.com/$REPO/releases for manual download"
            echo "  3. Wait for future Windows releases"
            echo ""
            print_highlight "To install WSL and try again:"
            echo "  wsl --install"
            echo "  wsl"
            echo "  curl -fsSL https://raw.githubusercontent.com/$REPO/main/scripts/universal-install.sh | bash"
            exit 1
        fi
    else
        # Detect OS for Unix-like systems
        case "$(uname -s)" in
            Linux*)     os="linux" ;;
            Darwin*)    os="macos" ;;
            *)          
                print_error "Unsupported operating system: $(uname -s)"
                print_error "Supported platforms: Linux, macOS, Windows (via WSL)"
                exit 1
                ;;
        esac
    fi

    # Detect architecture
    case "$(uname -m)" in
        x86_64|amd64)   arch="x86_64" ;;
        arm64|aarch64)  arch="arm64" ;;
        *)
            print_error "Unsupported architecture: $(uname -m)"
            print_error "Supported architectures: x86_64, arm64"
            exit 1
            ;;
    esac

    platform="${os}-${arch}"
    echo "$platform"
}

# Get latest release info
get_latest_release() {
    print_status "Fetching latest release information..."
    
    if command -v curl >/dev/null 2>&1; then
        curl -s "${GITHUB_API}/releases/latest"
    elif command -v wget >/dev/null 2>&1; then
        wget -q -O - "${GITHUB_API}/releases/latest"
    else
        print_error "Neither curl nor wget is available. Please install one of them."
        exit 1
    fi
}

# Download file
download_file() {
    local url="$1"
    local output="$2"
    
    print_status "Downloading: $(basename "$output")"
    
    if command -v curl >/dev/null 2>&1; then
        curl -L --progress-bar "$url" -o "$output"
    elif command -v wget >/dev/null 2>&1; then
        wget --progress=bar:force:noscroll "$url" -O "$output"
    else
        print_error "Neither curl nor wget is available."
        exit 1
    fi
}

# Extract version from JSON
extract_version() {
    local json="$1"
    echo "$json" | grep '"tag_name"' | head -n1 | cut -d'"' -f4
}

# Extract download URL from JSON
extract_download_url() {
    local json="$1"
    local platform="$2"
    local pattern="tihc-${platform}-.*\.tar\.gz"
    
    echo "$json" | grep '"browser_download_url"' | grep -E "$pattern" | head -n1 | cut -d'"' -f4
}

# Check system requirements
check_requirements() {
    print_status "Checking system requirements..."
    
    # Check for required tools
    local missing_tools=()
    
    if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
        missing_tools+=("curl or wget")
    fi
    
    if ! command -v tar >/dev/null 2>&1; then
        missing_tools+=("tar")
    fi
    
    if [ ${#missing_tools[@]} -gt 0 ]; then
        print_error "Missing required tools: ${missing_tools[*]}"
        print_status "Please install the missing tools and try again."
        
        # Provide installation suggestions
        case "$(uname -s)" in
            Linux*)
                if command -v apt-get >/dev/null 2>&1; then
                    echo "  sudo apt-get update && sudo apt-get install curl tar"
                elif command -v yum >/dev/null 2>&1; then
                    echo "  sudo yum install curl tar"
                elif command -v dnf >/dev/null 2>&1; then
                    echo "  sudo dnf install curl tar"
                fi
                ;;
            Darwin*)
                echo "  brew install curl" # tar is usually pre-installed on macOS
                ;;
        esac
        exit 1
    fi
    
    print_success "All required tools are available."
}

# Main installation function
install_tihc() {
    local platform version release_info download_url temp_dir filename

    # Check system requirements
    check_requirements

    # Detect platform
    platform=$(detect_platform)
    print_status "Detected platform: $platform"

    # Get release information
    release_info=$(get_latest_release)
    if [ -z "$release_info" ]; then
        print_error "Failed to fetch release information"
        print_status "Please check your internet connection and try again."
        exit 1
    fi

    # Extract version and download URL
    version=$(extract_version "$release_info")
    download_url=$(extract_download_url "$release_info" "$platform")

    if [ -z "$version" ]; then
        print_error "Failed to extract version information"
        exit 1
    fi

    if [ -z "$download_url" ]; then
        print_error "No release found for platform: $platform"
        print_error "Available releases:"
        echo "$release_info" | grep '"browser_download_url"' | cut -d'"' -f4 | sed 's/.*\//  - /'
        echo ""
        print_status "Please visit https://github.com/$REPO/releases for manual download."
        exit 1
    fi

    print_highlight "Latest version: $version"
    print_status "Download URL: $download_url"

    # Create temporary directory
    temp_dir=$(mktemp -d)
    filename="tihc-${platform}-${version}.tar.gz"
    
    # Download the release
    download_file "$download_url" "${temp_dir}/${filename}"

    # Extract the archive
    print_status "Extracting archive..."
    cd "$temp_dir"
    tar -xzf "$filename"

    # Check if binary exists
    if [ ! -f "tihc" ]; then
        print_error "Binary 'tihc' not found in archive"
        ls -la
        exit 1
    fi

    # Make binary executable
    chmod +x tihc

    # Test binary
    print_status "Testing binary..."
    if ./tihc --help >/dev/null 2>&1 || ./tihc --version >/dev/null 2>&1; then
        print_success "Binary test passed."
    else
        print_warning "Binary test failed, but continuing with installation..."
    fi

    # Install binary
    if [ "$EUID" -eq 0 ] || [ -w "$INSTALL_DIR" ]; then
        print_status "Installing tihc to $INSTALL_DIR..."
        cp tihc "$INSTALL_DIR/"
        print_success "tihc $version installed successfully!"
    else
        print_warning "No write permission to $INSTALL_DIR"
        print_status "Trying to install with sudo..."
        sudo cp tihc "$INSTALL_DIR/"
        print_success "tihc $version installed successfully with sudo!"
    fi

    # Clean up
    cd - >/dev/null
    rm -rf "$temp_dir"

    # Verify installation
    if command -v tihc >/dev/null 2>&1; then
        print_success "Installation verified. Run 'tihc --help' to get started."
        echo
        print_highlight "Installed version:"
        tihc --version 2>/dev/null || tihc --help | head -n1 || echo "tihc $version"
        echo
        print_status "You can now use 'tihc' command from anywhere."
    else
        print_warning "tihc installed but not found in PATH."
        print_warning "You may need to add $INSTALL_DIR to your PATH or restart your shell."
        echo
        print_status "To add to PATH temporarily:"
        echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
        echo
        print_status "To add to PATH permanently:"
        case "$SHELL" in
            */bash)
                echo "  echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.bashrc"
                echo "  source ~/.bashrc"
                ;;
            */zsh)
                echo "  echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.zshrc"
                echo "  source ~/.zshrc"
                ;;
            *)
                echo "  Add 'export PATH=\"$INSTALL_DIR:\$PATH\"' to your shell's config file"
                ;;
        esac
    fi
}

# Show usage
show_usage() {
    cat << EOF
TiHC Universal Auto Download Script v$SCRIPT_VERSION

DESCRIPTION:
    Automatically detects your platform and downloads the latest TiHC release.
    Supports Linux, macOS, and Windows (via WSL).

USAGE:
    $0 [OPTIONS]

OPTIONS:
    -h, --help          Show this help message
    -d, --dir DIR       Installation directory (default: /usr/local/bin)
    -v, --version       Show script version
    -f, --force         Force installation even if already installed
    --check             Check platform compatibility without installing

EXAMPLES:
    # Install to default location (/usr/local/bin)
    $0

    # Install to custom directory
    $0 --dir ~/.local/bin

    # Install to current directory
    INSTALL_DIR=. $0

    # Force reinstall
    $0 --force

    # Check platform compatibility
    $0 --check

ENVIRONMENT VARIABLES:
    INSTALL_DIR         Installation directory (overrides -d option)

SUPPORTED PLATFORMS:
    - Linux x86_64
    - Linux arm64 (if available in releases)
    - macOS x86_64 (Intel)
    - macOS arm64 (Apple Silicon)
    - Windows via WSL (any distribution)

INSTALLATION METHODS:
    # One-line installation
    curl -fsSL https://raw.githubusercontent.com/$REPO/main/scripts/universal-install.sh | bash

    # Download and run
    curl -fsSL https://raw.githubusercontent.com/$REPO/main/scripts/universal-install.sh -o install.sh
    chmod +x install.sh
    ./install.sh

TROUBLESHOOTING:
    If installation fails:
    1. Check internet connection
    2. Ensure curl/wget and tar are installed
    3. Try custom installation directory: --dir ~/.local/bin
    4. Visit https://github.com/$REPO/releases for manual download

EOF
}

# Check platform compatibility
check_platform() {
    print_status "Checking platform compatibility..."
    
    local platform
    platform=$(detect_platform)
    
    print_success "Platform detected: $platform"
    
    # Check if releases are available
    local release_info
    release_info=$(get_latest_release)
    
    if [ -z "$release_info" ]; then
        print_error "Cannot fetch release information"
        exit 1
    fi
    
    local download_url
    download_url=$(extract_download_url "$release_info" "$platform")
    
    if [ -z "$download_url" ]; then
        print_error "No release available for platform: $platform"
        print_status "Available releases:"
        echo "$release_info" | grep '"browser_download_url"' | cut -d'"' -f4 | sed 's/.*\//  - /'
        exit 1
    else
        print_success "Release available for your platform!"
        local version
        version=$(extract_version "$release_info")
        print_highlight "Latest version: $version"
        print_status "Download URL: $download_url"
    fi
}

# Parse command line arguments
FORCE_INSTALL=false
CHECK_ONLY=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_usage
            exit 0
            ;;
        -d|--dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        -v|--version)
            echo "TiHC Universal Auto Download Script v$SCRIPT_VERSION"
            exit 0
            ;;
        -f|--force)
            FORCE_INSTALL=true
            shift
            ;;
        --check)
            CHECK_ONLY=true
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Main execution
main() {
    echo "🚀 TiHC Universal Auto Download Script v$SCRIPT_VERSION"
    echo "=================================================="
    echo

    # Check platform compatibility only
    if [ "$CHECK_ONLY" = true ]; then
        check_platform
        exit 0
    fi

    # Check if tihc is already installed
    if command -v tihc >/dev/null 2>&1 && [ "$FORCE_INSTALL" = false ]; then
        print_warning "tihc is already installed:"
        tihc --version 2>/dev/null || tihc --help | head -n1 || echo "Unknown version"
        echo
        read -p "Do you want to continue and potentially overwrite? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_status "Installation cancelled."
            print_status "Use --force to override this check."
            exit 0
        fi
        echo
    fi

    install_tihc

    echo
    print_success "🎉 Installation completed successfully!"
    echo
    print_highlight "Next steps:"
    echo "  1. Run 'tihc --help' to see available commands"
    echo "  2. Visit https://www.askaric.com/en/tihc/ for documentation"
    echo "  3. Report issues at https://github.com/$REPO/issues"
}

# Run main function
main "$@"
