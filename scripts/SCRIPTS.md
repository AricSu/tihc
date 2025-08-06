# TiHC Installation Scripts Summary

This directory contains multiple installation scripts for TiHC (TiDB Intelligent Health Check). Choose the one that best fits your needs.

## 🎯 Recommended: Universal Installation Script

**File:** `universal-install.sh`

**Best for:** Everyone - supports all platforms with advanced features

**Features:**
- ✅ Universal platform support (Linux, macOS, Windows WSL)
- ✅ Automatic platform detection
- ✅ Windows WSL support with guidance
- ✅ System requirements checking
- ✅ Force install option
- ✅ Platform compatibility check
- ✅ Enhanced error handling
- ✅ Colored output with better UX

**Usage:**
```bash
# One-line installation
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/universal-install.sh | bash

# Check platform compatibility first
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/universal-install.sh | bash -s -- --check

# Force reinstall
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/universal-install.sh | bash -s -- --force

# Custom directory
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/universal-install.sh | INSTALL_DIR=~/.local/bin bash
```

## 📦 Legacy: Linux/macOS Installation Script

**File:** `install.sh`

**Best for:** Linux/macOS users who prefer the original script

**Features:**
- ✅ Linux and macOS support
- ✅ Automatic platform detection
- ✅ Smart directory selection
- ✅ Clean installation process
- ✅ Installation verification

**Usage:**
```bash
# One-line installation
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/install.sh | bash

# Custom directory
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/install.sh | INSTALL_DIR=~/.local/bin bash
```

## ⚡ Quick Install Redirect Script

**File:** `quick-install.sh`

**Best for:** Users who want the shortest URL

**Features:**
- ✅ Redirects to `install.sh`
- ✅ Shorter script name
- ✅ Simple one-liner

**Usage:**
```bash
# Redirects to install.sh
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/quick-install.sh | bash
```

## 🪟 Windows PowerShell Script (Future)

**File:** `install.ps1`

**Best for:** Future Windows native support

**Features:**
- ⏳ Prepared for Windows releases
- ✅ PowerShell-based installation
- ✅ Windows platform detection
- ✅ WSL guidance

**Current Status:** Ready for future Windows releases

## Platform Support Matrix

| Script | Linux x86_64 | macOS x86_64 | macOS ARM64 | Windows WSL | Windows Native |
|--------|---------------|---------------|-------------|-------------|----------------|
| `universal-install.sh` | ✅ | ✅ | ✅ | ✅ | 🔗 (guides to WSL) |
| `install.sh` | ✅ | ✅ | ✅ | ❌ | ❌ |
| `quick-install.sh` | ✅ | ✅ | ✅ | ❌ | ❌ |
| `install.ps1` | ❌ | ❌ | ❌ | ❌ | ⏳ (future) |

## Migration Guide

### From install.sh to universal-install.sh

The universal script is backward compatible and includes all features from `install.sh` plus:

- Windows WSL support
- Enhanced error handling
- System requirements checking
- Force install option
- Platform compatibility check

**Migration is seamless** - just replace the URL:

```bash
# Old way
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/install.sh | bash

# New way (recommended)
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/universal-install.sh | bash
```

## File Organization

```
scripts/
├── universal-install.sh    # 🎯 Recommended: Universal installer
├── install.sh             # 📦 Legacy: Linux/macOS installer  
├── quick-install.sh        # ⚡ Redirect to install.sh
├── install.ps1            # 🪟 Future: Windows PowerShell
├── README.md              # 📚 Detailed documentation
└── SCRIPTS.md             # 📋 This summary file
```

## Quick Decision Guide

**Choose `universal-install.sh` if:**
- ✅ You want the best experience
- ✅ You're using Windows with WSL
- ✅ You want advanced features (force install, platform check)
- ✅ You prefer enhanced error handling

**Choose `install.sh` if:**
- ✅ You're on Linux/macOS only
- ✅ You prefer the original script
- ✅ You want a smaller script

**Choose `quick-install.sh` if:**
- ✅ You want the shortest URL
- ✅ You're okay with redirecting to `install.sh`

## Testing Installation Scripts

To test the installation scripts without affecting your system:

```bash
# Test with custom directory
INSTALL_DIR=/tmp/tihc-test curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/universal-install.sh | bash

# Check platform compatibility only
curl -fsSL https://raw.githubusercontent.com/AricSu/tihc/main/scripts/universal-install.sh | bash -s -- --check

# Clean up test installation
rm -f /tmp/tihc-test/tihc
```
