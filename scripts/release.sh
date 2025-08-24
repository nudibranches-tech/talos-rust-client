#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if version argument is provided
if [ -z "$1" ]; then
    echo -e "${RED}Error: Version argument required${NC}"
    echo "Usage: $0 <version>"
    echo "Example: $0 0.1.1"
    echo "Example: $0 0.2.0-alpha.1"
    exit 1
fi

VERSION=$1

# Validate version format
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$ ]]; then
    echo -e "${RED}Error: Invalid version format${NC}"
    echo "Version should be in format: X.Y.Z or X.Y.Z-suffix"
    exit 1
fi

echo -e "${GREEN}Preparing release v${VERSION}...${NC}"

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo -e "${YELLOW}Warning: Not on main branch (current: $CURRENT_BRANCH)${NC}"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo -e "${RED}Error: You have uncommitted changes${NC}"
    echo "Please commit or stash your changes before releasing"
    exit 1
fi

# Run tests
echo -e "${GREEN}Running tests...${NC}"
cargo test --all-features

# Create and push tag
TAG="v${VERSION}"
echo -e "${GREEN}Creating tag ${TAG}...${NC}"
git tag -a "$TAG" -m "Release $TAG"

echo -e "${GREEN}Pushing tag to origin...${NC}"
git push origin "$TAG"

echo -e "${GREEN}✓ Release tag ${TAG} pushed!${NC}"
echo ""
echo "GitHub Actions will now:"
echo "1. Update Cargo.toml to version ${VERSION}"
echo "2. Run tests"
echo "3. Create GitHub release"
echo "4. Publish to crates.io"
echo "5. Build and upload Linux artifacts"
echo ""
echo "Monitor progress at: https://github.com/nuditech/talos-rust-client/actions"