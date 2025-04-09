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

# Info output
info() {
    echo -e "${BLUE}${BOLD}Info:${NC} $1"
}

# Check version parameter
if [ $# -ne 1 ]; then
    error "Please provide version number, e.g.: ./release.sh v0.1.0"
fi

VERSION=$1
RELEASE_DIR="target/release"
mkdir -p $RELEASE_DIR

# Build Apple Silicon version
info "Building Apple Silicon (arm64) version..."
cargo build --release --target aarch64-apple-darwin
cp target/aarch64-apple-darwin/release/tihc "${RELEASE_DIR}/tihc"

# Package release files
info "Packaging binary files..."
cd ${RELEASE_DIR}
tar -czf "tihc-${VERSION}-arm64-darwin.tar.gz" tihc
shasum -a 256 "tihc-${VERSION}-arm64-darwin.tar.gz" > "tihc-${VERSION}-arm64-darwin.tar.gz.sha256"
cd - > /dev/null

# Create and push tag
info "Creating Git tag: ${VERSION}..."
git tag -a "$VERSION" -m "Release $VERSION"

# Display release information
echo -e "\n${GREEN}${BOLD}Build completed!${NC}"
echo -e "\nNext steps:"
echo -e "1. Push tag to GitHub:"
echo -e "   ${BOLD}git push origin $VERSION${NC}"
echo -e "\n2. Create new release on GitHub:"
echo -e "   - Visit: ${BLUE}https://github.com/aricsu/tihc/releases/new${NC}"
echo -e "   - Select tag: ${BOLD}$VERSION${NC}"
echo -e "   - Upload files:"
echo -e "     ${BOLD}${RELEASE_DIR}/tihc-${VERSION}-arm64-darwin.tar.gz${NC}"
echo -e "     ${BOLD}${RELEASE_DIR}/tihc-${VERSION}-arm64-darwin.tar.gz.sha256${NC}"

echo -e "\n${YELLOW}Note: Please test the binary before publishing the release${NC}"
