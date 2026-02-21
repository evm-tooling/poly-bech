#!/usr/bin/env bash
# Install poly-bench from the latest GitHub release.
# Usage: curl -sSL https://raw.githubusercontent.com/evm-tooling/poly-bench/main/scripts/install.sh | bash
# Or: curl -sSL ... | bash -s -- -d /usr/local/bin  # custom install dir

set -eo pipefail

POLYBENCH_INSTALLER_VERSION="1.0.0"

REPO="${POLYBENCH_REPO:-evm-tooling/poly-bench}"
INSTALL_DIR="${HOME}/.local/bin"
BINARY_NAME="poly-bench"

main() {
  need_cmd curl

  while getopts "d:hvV" opt; do
    case "$opt" in
      d) INSTALL_DIR="$OPTARG" ;;
      h) usage; exit 0 ;;
      v|V) version; exit 0 ;;
      *) exit 1 ;;
    esac
  done

  banner

  say "detecting platform..."

  detect_platform_arch

  say "detected ${BOLD}${PLATFORM}${NC} (${BOLD}${ARCHITECTURE}${NC})"

  case "$PLATFORM" in
    darwin)
      if [ "$ARCHITECTURE" = "x86_64" ]; then
        warn "pre-built binary for Intel macOS is not available"
        err "install from source with: cargo install poly-bench"
      fi
      ASSET="poly-bench-macos-aarch64"
      ;;
    linux)
      if [ "$ARCHITECTURE" != "x86_64" ]; then
        warn "pre-built binary for Linux $ARCHITECTURE is not available"
        err "install from source with: cargo install poly-bench"
      fi
      ASSET="poly-bench-linux-x86_64"
      ;;
    win32)
      ASSET="poly-bench-windows-x86_64.exe"
      BINARY_NAME="poly-bench.exe"
      ;;
    *)
      err "unsupported platform: $PLATFORM"
      ;;
  esac

  DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ASSET}"

  ensure mkdir -p "$INSTALL_DIR"
  TMP="$(mktemp -d 2>/dev/null)" || err "failed to create temp directory"
  trap 'rm -rf "$TMP"' EXIT

  say "downloading poly-bench..."

  if ! download "$DOWNLOAD_URL" "$TMP/$BINARY_NAME"; then
    err "download failed. No release asset at: $DOWNLOAD_URL
    
Ensure a release exists with asset '${ASSET}', or install from source:
    cargo install poly-bench"
  fi

  if head -c 200 "$TMP/$BINARY_NAME" | grep -qE '<!DOCTYPE|Not Found'; then
    err "download returned an error page instead of the binary.

No release asset at: $DOWNLOAD_URL

Create a release and run the release-build workflow to attach binaries,
or install from source: cargo install poly-bench"
  fi

  say "installing to ${BOLD}${INSTALL_DIR}${NC}..."

  ensure chmod +x "$TMP/$BINARY_NAME"
  ensure mv "$TMP/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"

  if ! echo ":$PATH:" | grep -q ":$INSTALL_DIR:"; then
    configure_shell_path
  fi

  printf '\n'
  printf '%b\n' "${GREEN}${BOLD}poly-bench was installed successfully!${NC}"
  printf '%b\n' "${DIM}─────────────────────────────────────────${NC}"
  printf '\n'
  
  if command -v "$INSTALL_DIR/$BINARY_NAME" >/dev/null 2>&1; then
    "$INSTALL_DIR/$BINARY_NAME" --version
  else
    say "installed: $INSTALL_DIR/$BINARY_NAME"
    printf '\n'
    warn "poly-bench is not in your PATH yet"
    say "run the following or restart your terminal:"
    printf '\n'
    printf '    %b\n' "${CYAN}source ~/.${PREF_SHELL}rc${NC}"
    printf '\n'
  fi

  printf '%b\n' "${DIM}─────────────────────────────────────────${NC}"
  printf '\n'
  printf '%b\n' "Run ${CYAN}poly-bench --help${NC} to get started"
  printf '\n'
}

usage() {
  cat 1>&2 <<EOF

${BOLD}poly-bench installer${NC}

Install poly-bench from the latest GitHub release.

${BOLD}USAGE:${NC}
    curl -sSL https://raw.githubusercontent.com/${REPO}/main/scripts/install.sh | bash

${BOLD}OPTIONS:${NC}
    -d <DIR>    Install to a custom directory (default: ~/.local/bin)
    -h          Print this help message
    -v          Print the installer version

${BOLD}EXAMPLES:${NC}
    # Install to default location
    curl -sSL .../install.sh | bash

    # Install to /usr/local/bin
    curl -sSL .../install.sh | bash -s -- -d /usr/local/bin

EOF
}

version() {
  say "poly-bench installer version ${BOLD}${POLYBENCH_INSTALLER_VERSION}${NC}"
}

banner() {
  printf '%b' "${CYAN}"
  cat <<'EOF'

.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx

    ██████╗  ██████╗ ██╗  ██╗   ██╗      ██████╗ ███████╗███╗   ██╗ ██████╗██╗  ██╗
    ██╔══██╗██╔═══██╗██║  ╚██╗ ██╔╝      ██╔══██╗██╔════╝████╗  ██║██╔════╝██║  ██║
    ██████╔╝██║   ██║██║   ╚████╔╝ █████╗██████╔╝█████╗  ██╔██╗ ██║██║     ███████║
    ██╔═══╝ ██║   ██║██║    ╚██╔╝  ╚════╝██╔══██╗██╔══╝  ██║╚██╗██║██║     ██╔══██║
    ██║     ╚██████╔╝███████╗██║         ██████╔╝███████╗██║ ╚████║╚██████╗██║  ██║
    ╚═╝      ╚═════╝ ╚══════╝╚═╝         ╚═════╝ ╚══════╝╚═╝  ╚═══╝ ╚═════╝╚═╝  ╚═╝

.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx

EOF
  printf '%b' "${NC}"
  printf '%b\n' "    ${DIM}Polyglot benchmarking for smart contracts${NC}"
  printf '\n'
  printf '%b\n' "    ${DIM}Repo${NC}  : ${BLUE}https://github.com/${REPO}${NC}"
  printf '\n'
  printf '%b' "${CYAN}"
  printf '%s\n' ".xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx.xOx"
  printf '%b' "${NC}"
  printf '\n'
}

configure_shell_path() {
  case $SHELL in
    */zsh)
      PROFILE="${ZDOTDIR:-"$HOME"}/.zshrc"
      PREF_SHELL="zsh"
      ;;
    */bash)
      PROFILE="$HOME/.bashrc"
      PREF_SHELL="bash"
      ;;
    */fish)
      PROFILE="$HOME/.config/fish/config.fish"
      PREF_SHELL="fish"
      ;;
    */ash)
      PROFILE="$HOME/.profile"
      PREF_SHELL="ash"
      ;;
    *)
      warn "could not detect shell, manually add ${INSTALL_DIR} to your PATH"
      return
      ;;
  esac

  if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
    say "adding poly-bench to PATH in ${BOLD}${PROFILE}${NC}"
    
    if [[ "$PREF_SHELL" == "fish" ]]; then
      echo >> "$PROFILE" && echo "fish_add_path -a \"$INSTALL_DIR\"" >> "$PROFILE"
    else
      echo >> "$PROFILE" && echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$PROFILE"
    fi
    
    printf '\n'
    say "added poly-bench to PATH"
    say "run ${CYAN}source ${PROFILE}${NC} or start a new terminal to use poly-bench"
  fi
}

detect_platform_arch() {
  uname_s=$(uname -s)
  PLATFORM=$(tolower "${POLYBENCH_PLATFORM:-$uname_s}")
  
  case $PLATFORM in
    linux) ;;
    darwin|mac*)
      PLATFORM="darwin"
      ;;
    mingw*|msys*|cygwin*|win*)
      PLATFORM="win32"
      ;;
    *)
      err "unsupported platform: $PLATFORM"
      ;;
  esac

  uname_m=$(uname -m)
  ARCHITECTURE=$(tolower "${POLYBENCH_ARCH:-$uname_m}")
  
  if [ "$ARCHITECTURE" = "x86_64" ]; then
    if [ "$(sysctl -n sysctl.proc_translated 2>/dev/null)" = "1" ]; then
      ARCHITECTURE="aarch64"
    fi
  elif [ "$ARCHITECTURE" = "arm64" ] || [ "$ARCHITECTURE" = "aarch64" ]; then
    ARCHITECTURE="aarch64"
  fi
}

tolower() {
  echo "$1" | awk '{print tolower($0)}'
}

say() {
  printf '%b\n' "${GREEN}info${NC}: $1"
}

warn() {
  printf '%b\n' "${YELLOW}warn${NC}: $1" >&2
}

err() {
  printf '%b\n' "${RED}error${NC}: $1" >&2
  exit 1
}

need_cmd() {
  if ! check_cmd "$1"; then
    err "need '$1' (command not found)"
  fi
}

check_cmd() {
  command -v "$1" >/dev/null 2>&1
}

ensure() {
  if ! "$@"; then
    err "command failed: $*"
  fi
}

download() {
  if check_cmd curl; then
    HTTP_CODE=$(curl -#fSL -w "%{http_code}" -o "$2" "$1" 2>&1) || {
      if [ "$HTTP_CODE" != "200" ]; then
        return 1
      fi
    }
  elif check_cmd wget; then
    wget --show-progress -qO "$2" "$1" || return 1
  else
    err "need curl or wget to download"
  fi
}

# Colors - define after functions but before main
setup_colors() {
  if [ -t 1 ] && [ -n "$(tput colors 2>/dev/null)" ] && [ "$(tput colors)" -ge 8 ]; then
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    PURPLE='\033[0;35m'
    CYAN='\033[0;36m'
    BOLD='\033[1m'
    DIM='\033[2m'
    NC='\033[0m'
  else
    RED=''
    GREEN=''
    YELLOW=''
    BLUE=''
    PURPLE=''
    CYAN=''
    BOLD=''
    DIM=''
    NC=''
  fi
}

setup_colors
main "$@"
