#!/bin/bash
# Script to update Homebrew formula after a release

set -e

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.3.0"
    exit 1
fi

REPO="say828/gh-flow"
FORMULA_PATH="homebrew/gh-flow.rb"

echo "Updating Homebrew formula for version $VERSION..."

# Download and calculate SHA256 for each platform
declare -A PLATFORMS=(
    ["MACOS_INTEL"]="x86_64-apple-darwin"
    ["MACOS_ARM"]="aarch64-apple-darwin"
    ["LINUX_INTEL"]="x86_64-unknown-linux-gnu"
    ["LINUX_ARM"]="aarch64-unknown-linux-gnu"
)

for KEY in "${!PLATFORMS[@]}"; do
    PLATFORM="${PLATFORMS[$KEY]}"
    URL="https://github.com/$REPO/releases/download/v$VERSION/gh-flow-$PLATFORM.tar.gz"
    echo "Downloading $URL..."

    SHA256=$(curl -sL "$URL" | shasum -a 256 | cut -d' ' -f1)
    echo "$KEY: $SHA256"

    # Replace placeholder in formula
    sed -i '' "s/SHA256_PLACEHOLDER_$KEY/$SHA256/g" "$FORMULA_PATH"
done

# Update version
sed -i '' "s/VERSION_PLACEHOLDER/$VERSION/g" "$FORMULA_PATH"

echo "Formula updated! Review and commit the changes."
