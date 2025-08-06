#!/bin/bash

# Test script to simulate CI/CD workflow locally
# Usage: ./test-workflow.sh

set -e

echo "🧪 Testing TiHC CI/CD Workflow Logic"
echo "===================================="

# Simulate environment variables
export GITHUB_REF="refs/tags/v1.0.0-test"
export GITHUB_REF_NAME="v1.0.0-test"

# Test platform detection
platforms=("linux-x86_64" "macos-x86_64" "macos-arm64")

for platform in "${platforms[@]}"; do
    echo ""
    echo "📦 Testing platform: $platform"
    echo "------------------------------"
    
    # Simulate build
    echo "Building with make server..."
    make server
    
    # Test binary
    echo "Testing binary..."
    if [ -f "bin/tihc" ]; then
        echo "✅ Binary exists: bin/tihc"
        ./bin/tihc --version
        echo "✅ Binary works correctly"
    else
        echo "❌ Binary not found!"
        exit 1
    fi
    
    # Test packaging logic
    echo "Testing packaging..."
    
    # Extract version from tag
    if [[ "${GITHUB_REF}" == refs/tags/* ]]; then
        VERSION=${GITHUB_REF#refs/tags/}
    else
        VERSION="dev-$(date +%Y%m%d-%H%M%S)"
    fi
    
    echo "Version: ${VERSION}"
    echo "Platform: ${platform}"
    
    # Create package
    cd bin
    tar czf ../tihc-${platform}-${VERSION}.tar.gz tihc
    cd ..
    
    # Verify package
    if [ -f "tihc-${platform}-${VERSION}.tar.gz" ]; then
        echo "✅ Package created: tihc-${platform}-${VERSION}.tar.gz"
        echo "Package size: $(ls -lh tihc-${platform}-${VERSION}.tar.gz | awk '{print $5}')"
        echo "Package contents:"
        tar -tzf tihc-${platform}-${VERSION}.tar.gz
        
        # Clean up test package
        rm tihc-${platform}-${VERSION}.tar.gz
    else
        echo "❌ Package creation failed!"
        exit 1
    fi
    
    echo "✅ Platform $platform test passed"
done

echo ""
echo "🎉 All tests passed successfully!"
echo "Workflow logic is working correctly."

# Test installation script
echo ""
echo "🚀 Testing installation script..."
if [ -f "scripts/universal-install.sh" ]; then
    echo "✅ Universal install script exists"
    # Test script syntax
    bash -n scripts/universal-install.sh
    echo "✅ Install script syntax is valid"
else
    echo "❌ Install script not found!"
    exit 1
fi

echo ""
echo "✅ All workflow components tested successfully!"
