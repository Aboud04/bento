#!/bin/bash
set -euo pipefail

REPO="Aboud04/bento"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

echo "Installing bento..."

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64) ASSET="bento-linux-x86_64.tar.gz" ;;
            *) echo "Error: Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    Darwin)
        case "$ARCH" in
            x86_64) ASSET="bento-macos-x86_64.tar.gz" ;;
            arm64)  ASSET="bento-macos-aarch64.tar.gz" ;;
            *) echo "Error: Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "Error: Unsupported OS: $OS (use the Windows .zip from GitHub Releases)"
        exit 1
        ;;
esac

URL="https://github.com/$REPO/releases/latest/download/$ASSET"

# Create install directory
mkdir -p "$INSTALL_DIR"

# Download and extract
echo "Downloading $ASSET..."
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

if command -v curl &>/dev/null; then
    curl -fsSL "$URL" -o "$TMP/$ASSET"
elif command -v wget &>/dev/null; then
    wget -q "$URL" -O "$TMP/$ASSET"
else
    echo "Error: curl or wget is required"
    exit 1
fi

tar xzf "$TMP/$ASSET" -C "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/bento" "$INSTALL_DIR/bt"

echo ""
echo "Installed bento to $INSTALL_DIR"

# Check if install dir is in PATH
if ! echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
    echo ""
    echo "Add this to your shell config (~/.bashrc or ~/.zshrc):"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
fi

echo ""
echo "Then run:"
echo "  bento init        # set up tab completion + auto-cd"
echo "  source ~/.bashrc  # activate it"
echo ""
echo "Done!"
