#!/usr/bin/env sh
set -e

# Anime4K GLSL Shaders & Hotkeys Installer for mpv
# Repository: https://github.com/Praveensenpai/anime4k-cli

REPO="Praveensenpai/anime4k-cli"
GITHUB_RELEASES="https://github.com/${REPO}/releases"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m'

printf "${CYAN}${BOLD}📺 Anime4K GLSL Installer & Manager for mpv${NC}\n"

# 1. Check if running inside local source directory with cargo
if [ -f "./Cargo.toml" ] && command -v cargo >/dev/null 2>&1; then
    printf "${BLUE}📂 Running from local repository via cargo...${NC}\n"
    exec cargo run --release -- "$@"
fi

# 2. Detect OS and Architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
    linux)
        TARGET_OS="linux"
        ;;
    darwin)
        TARGET_OS="darwin"
        ;;
    *)
        printf "${RED}❌ Unsupported operating system: %s${NC}\n" "$OS"
        printf "${YELLOW}💡 On Windows, run in PowerShell:${NC}\n"
        printf "   irm https://raw.githubusercontent.com/%s/main/install.ps1 | iex\n" "$REPO"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64|amd64)
        TARGET_ARCH="x86_64"
        ;;
    aarch64|arm64)
        TARGET_ARCH="aarch64"
        ;;
    *)
        printf "${RED}❌ Unsupported architecture: %s${NC}\n" "$ARCH"
        exit 1
        ;;
esac

BINARY_NAME="anime4k-${TARGET_OS}-${TARGET_ARCH}"
RELEASE_URL="${GITHUB_RELEASES}/latest/download/${BINARY_NAME}.tar.gz"

INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "$INSTALL_DIR"
TARGET_BIN="${INSTALL_DIR}/anime4k"

# 3. Download release binary or build with cargo
download_binary() {
    TMP_DIR=$(mktemp -d 2>/dev/null || mktemp -d -t 'anime4k')
    ARCHIVE="${TMP_DIR}/anime4k.tar.gz"

    printf "${BLUE}📦 Fetching pre-built binary (${TARGET_OS}-${TARGET_ARCH})...${NC}\n"

    if command -v curl >/dev/null 2>&1; then
        if curl -fsSL -o "$ARCHIVE" "$RELEASE_URL"; then
            tar -xzf "$ARCHIVE" -C "$TMP_DIR"
            mv "$TMP_DIR/anime4k" "$TARGET_BIN"
            chmod +x "$TARGET_BIN"
            rm -rf "$TMP_DIR"
            return 0
        fi
    elif command -v wget >/dev/null 2>&1; then
        if wget -qO "$ARCHIVE" "$RELEASE_URL"; then
            tar -xzf "$ARCHIVE" -C "$TMP_DIR"
            mv "$TMP_DIR/anime4k" "$TARGET_BIN"
            chmod +x "$TARGET_BIN"
            rm -rf "$TMP_DIR"
            return 0
        fi
    fi

    rm -rf "$TMP_DIR"
    return 1
}

if ! download_binary; then
    # Fallback to cargo if release binary is not yet available or failed
    if command -v cargo >/dev/null 2>&1; then
        printf "${YELLOW}ℹ️ Release binary not found, compiling from source with cargo...${NC}\n"
        cargo install --git "https://github.com/${REPO}.git" --bin anime4k --root "${HOME}/.local"
    else
        printf "${RED}❌ Failed to download pre-built binary and 'cargo' is not installed.${NC}\n"
        printf "${YELLOW}Please install Rust (https://rustup.rs) or download the binary manually from:${NC}\n"
        printf "  %s\n" "$GITHUB_RELEASES"
        exit 1
    fi
fi

# 4. Check PATH and execute
case ":$PATH:" in
    *":${INSTALL_DIR}:"*) ;;
    *)
        printf "${YELLOW}⚠️ Note: %s is not in your PATH.${NC}\n" "$INSTALL_DIR"
        printf "   Add it by running: ${CYAN}export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}\n\n"
        ;;
esac

exec "$TARGET_BIN" "$@"
