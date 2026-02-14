#!/usr/bin/env bash
# Install poly-bench from the latest GitHub release.
# Usage: curl -L https://raw.githubusercontent.com/evm-tooling/poly-bech/main/scripts/install.sh | bash
# Or: curl -L ... | bash -s -- -d /usr/local/bin  # custom install dir

set -e

REPO="${POLYBENCH_REPO:-evm-tooling/poly-bech}"
INSTALL_DIR="${HOME}/.local/bin"
BINARY_NAME="poly-bench"

while getopts "d:h" opt; do
  case "$opt" in
    d) INSTALL_DIR="$OPTARG" ;;
    h) echo "Usage: curl -L .../install.sh | bash [ -d /path/to/bin ]"; exit 0 ;;
    *) exit 1 ;;
  esac
done

# Detect OS and arch
OS="$(uname -s)"
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64|amd64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) ARCH="x86_64" ;;
esac

case "$OS" in
  Darwin)
    OS="macos"
    # Only aarch64 binary is built for macOS (GitHub runners are Apple Silicon)
    if [ "$ARCH" = "x86_64" ]; then
      echo "Pre-built binary for Intel macOS is not available. Install with: cargo install poly-bench"
      exit 1
    fi
    ASSET="poly-bench-macos-aarch64"
    ;;
  Linux)
    OS="linux"
    if [ "$ARCH" != "x86_64" ]; then
      echo "Pre-built binary for Linux $ARCH is not available. Install with: cargo install poly-bench"
      exit 1
    fi
    ASSET="poly-bench-linux-x86_64"
    ;;
  MINGW*|MSYS*|CYGWIN*)
    OS="windows"
    ARCH="x86_64"
    ASSET="poly-bench-windows-x86_64.exe"
    BINARY_NAME="poly-bench.exe"
    ;;
  *)
    echo "Unsupported OS: $OS"
    exit 1
    ;;
esac

DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ASSET}"
mkdir -p "$INSTALL_DIR"
TMP="$(mktemp -d)"
trap "rm -rf '$TMP'" EXIT

echo "==> Downloading poly-bench from GitHub..."
if command -v curl >/dev/null 2>&1; then
  HTTP_CODE="$(curl -sSL -w "%{http_code}" -o "$TMP/$BINARY_NAME" "$DOWNLOAD_URL")"
  if [ "$HTTP_CODE" != "200" ]; then
    echo "Download failed (HTTP $HTTP_CODE). No release asset at: $DOWNLOAD_URL"
    echo "Ensure a release exists with asset '$ASSET', or install from source: cargo install poly-bench"
    exit 1
  fi
elif command -v wget >/dev/null 2>&1; then
  if ! wget -q -O "$TMP/$BINARY_NAME" "$DOWNLOAD_URL"; then
    echo "Download failed. No release asset at: $DOWNLOAD_URL"
    echo "Ensure a release exists with asset '$ASSET', or install from source: cargo install poly-bench"
    exit 1
  fi
else
  echo "Need curl or wget to download."
  exit 1
fi

# Refuse to install if we got HTML (e.g. 404 page)
if head -c 200 "$TMP/$BINARY_NAME" | grep -qE '<!DOCTYPE|Not Found'; then
  echo "Download returned an error page instead of the binary. No release asset at: $DOWNLOAD_URL"
  echo "Create a release and run the release-build workflow to attach binaries, or install from source: cargo install poly-bench"
  exit 1
fi

chmod +x "$TMP/$BINARY_NAME"
mv "$TMP/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

if ! echo ":$PATH:" | grep -q ":$INSTALL_DIR:"; then
  echo ""
  echo "==> Add poly-bench to your PATH:"
  echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
  echo ""
  echo "Add the line above to your shell profile (~/.bashrc, ~/.zshrc, or ~/.profile)."
fi

echo "==> Installed: $INSTALL_DIR/$BINARY_NAME"
"$INSTALL_DIR/$BINARY_NAME" --version
