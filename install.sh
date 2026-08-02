#!/bin/bash

CYAN='\033[0;36m'
GREEN='\033[0;32m'
PURPLE='\033[0;35m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BOLD='\033[1m'
NC='\033[0m'

echo -e "${PURPLE}🚀 Installing anime4k-mpv-installer...${NC}\n"

BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

RAW_URL="https://raw.githubusercontent.com/Praveensenpai/anime4k-mpv-installer/main/bin/anime4k-mpv-installer"

# Check if running directly from a local repository clone
LOCAL_DIR=""
if [ -n "${BASH_SOURCE[0]}" ] && [ -f "${BASH_SOURCE[0]}" ]; then
    LOCAL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" 2>/dev/null && pwd)"
fi

if [ -n "$LOCAL_DIR" ] && [ -f "$LOCAL_DIR/PKGBUILD" ] && [ -f "$LOCAL_DIR/bin/anime4k-mpv-installer" ]; then
    cp "$LOCAL_DIR/bin/anime4k-mpv-installer" "$BIN_DIR/anime4k-mpv-installer"
else
    echo -e "${BLUE}📦 Downloading anime4k-mpv-installer binary from GitHub...${NC}"
    curl -sSL -H 'Cache-Control: no-cache' "$RAW_URL" -o "$BIN_DIR/anime4k-mpv-installer"
fi

if [ ! -f "$BIN_DIR/anime4k-mpv-installer" ] || [ ! -s "$BIN_DIR/anime4k-mpv-installer" ]; then
    echo -e "${RED}❌ Error: Failed to download anime4k-mpv-installer binary!${NC}"
    exit 1
fi

chmod +x "$BIN_DIR/anime4k-mpv-installer"
echo -e "${GREEN}✔ Installed anime4k-mpv-installer to ${BIN_DIR}/anime4k-mpv-installer${NC}"

# Shell alias setup
SHELL_CONFIGS=("$HOME/.bashrc" "$HOME/.zshrc")
ALIAS_LINE="alias anime4k-mpv-installer='$HOME/.local/bin/anime4k-mpv-installer'"

for config in "${SHELL_CONFIGS[@]}"; do
    if [ -f "$config" ]; then
        if ! grep -q "alias anime4k-mpv-installer=" "$config" 2>/dev/null; then
            echo "" >> "$config"
            echo "$ALIAS_LINE" >> "$config"
            echo -e "${BLUE}📝 Added anime4k-mpv-installer alias to $config${NC}"
        fi
    fi
done

echo -e "\n${GREEN}${BOLD}🎉 anime4k-mpv-installer setup completed!${NC}"
echo -e "Run it anytime with: ${CYAN}anime4k-mpv-installer${NC}"
