#!/bin/bash
set -e

echo "Installing gh-flow..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Build the project
echo "Building gh-flow (this may take a minute)..."
cargo build --release

# Copy binary to current directory (for gh extension)
# gh extension looks for executable in the extension directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cp target/release/gh-flow "$SCRIPT_DIR/"
chmod +x "$SCRIPT_DIR/gh-flow"

echo "✓ gh-flow installed successfully!"
echo ""
echo "You can now use: gh flow --help"
