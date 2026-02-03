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

# Get the installation directory
GH_INSTALL_DIR="${GH_INSTALL_DIR:-$HOME/.local/bin}"
mkdir -p "$GH_INSTALL_DIR"

# Copy the binary
echo "Installing to $GH_INSTALL_DIR..."
cp target/release/gh-flow "$GH_INSTALL_DIR/"
chmod +x "$GH_INSTALL_DIR/gh-flow"

echo "✓ gh-flow installed successfully!"
echo ""
echo "Make sure $GH_INSTALL_DIR is in your PATH:"
echo "  export PATH=\"$GH_INSTALL_DIR:\$PATH\""
echo ""
echo "You can now use: gh flow --help"
