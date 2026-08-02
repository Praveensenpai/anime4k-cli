#!/usr/bin/env bash

# Anime4K GLSL Shaders & Hotkeys Installer for mpv

CYAN='\033[0;36m'
GREEN='\033[0;32m'
PURPLE='\033[0;35m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

TARGET="$HOME/.config/mpv"

# Interactive Arrow Key Selector
select_option() {
    local prompt="$1"
    shift
    local options=("$@")
    local selected=0

    exec 3< /dev/tty 2>/dev/null || true

    printf "\033[?25l" >&2
    cleanup_cursor() { printf "\033[?25h" >&2; stty echo icanon 2>/dev/null || true; }
    trap cleanup_cursor EXIT INT TERM

    while true; do
        echo -e "\033[1;36m? \033[1;37m${prompt}\033[0;33m (Use arrow keys & Enter)\033[0m" >&2

        for i in "${!options[@]}"; do
            if [ "$i" -eq "$selected" ]; then
                echo -e "  \033[1;32m❯ \033[7m ${options[$i]} \033[0m" >&2
            else
                echo -e "    \033[2m${options[$i]}\033[0m" >&2
            fi
        done

        stty -echo -icanon min 1 time 0 2>/dev/null || true
        IFS= read -r -n 1 -u 3 key 2>/dev/null || true
        if [ "$key" = $'\x1b' ]; then
            read -r -n 2 -u 3 rest 2>/dev/null || true
            key="$key$rest"
        fi
        stty echo icanon 2>/dev/null || true

        case "$key" in
            $'\x1b[A'|$'\x1bOA') # Up
                ((selected--))
                [ "$selected" -lt 0 ] && selected=$((${#options[@]} - 1))
                ;;
            $'\x1b[B'|$'\x1bOB') # Down
                ((selected++))
                [ "$selected" -ge "${#options[@]}" ] && selected=0
                ;;
            ""|$'\n'|$'\r') # Enter
                break
                ;;
        esac

        local total_lines=$((${#options[@]} + 1))
        for ((l=0; l<total_lines; l++)); do
            printf "\033[1A\033[2K" >&2
        done
    done

    cleanup_cursor
    trap - EXIT INT TERM
    exec 3<&- 2>/dev/null || true

    echo "$selected"
}

echo -e "${PURPLE}📺  Installing Anime4K shaders for mpv...${NC}\n"

if ! command -v unzip &> /dev/null; then
    echo -e "${RED}❌ Error: unzip is not installed!${NC}"
    exit 1
fi

PRESET="${ANIME4K_PRESET:-}"

if [ -z "$PRESET" ]; then
    HAS_TTY=0
    if [ -t 0 ] || [ -c /dev/tty ]; then
        HAS_TTY=1
    fi

    if [ "$HAS_TTY" -eq 1 ]; then
        CHOICE_INDEX=$(select_option "Select Anime4K GPU preset:" \
            "Higher-end GPU (GTX 1080, RTX 2070/3060+, RX 590/5700XT+) [HQ - VL Shaders]" \
            "Lower-end GPU (GTX 980, GTX 1060, RX 570, Integrated GPUs) [Fast - M/S Shaders]")
        case "$CHOICE_INDEX" in
            1) PRESET="low" ;;
            *) PRESET="high" ;;
        esac
    else
        PRESET="high"
    fi
fi

if [ "$PRESET" = "low" ]; then
    URL="https://github.com/Tama47/Anime4K/releases/download/v4.0.1/GLSL_Mac_Linux_Low-end.zip"
    PRESET_NAME="Lower-end GPU (Fast)"
    MPV_GLSL_LINE='glsl-shaders="~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_M.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_S.glsl"'
    HEADER_COMMENT="# Optimized shaders for lower-end GPU: Mode A (Fast)"
else
    URL="https://github.com/Tama47/Anime4K/releases/download/v4.0.1/GLSL_Mac_Linux_High-end.zip"
    PRESET_NAME="Higher-end GPU (HQ)"
    MPV_GLSL_LINE='glsl-shaders="~~/shaders/Anime4K_Clamp_Highlights.glsl:~~/shaders/Anime4K_Restore_CNN_VL.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_VL.glsl:~~/shaders/Anime4K_AutoDownscalePre_x2.glsl:~~/shaders/Anime4K_AutoDownscalePre_x4.glsl:~~/shaders/Anime4K_Upscale_CNN_x2_M.glsl"'
    HEADER_COMMENT="# Optimized shaders for higher-end GPU: Mode A (HQ)"
fi

echo -e "${GREEN}✨ Selected preset: ${PRESET_NAME}${NC}"
echo -e "${BLUE}📂 Ensuring ${TARGET} exists...${NC}"
mkdir -p "$TARGET"

TMP_DIR=$(mktemp -d)
TARGET_FILE="$TMP_DIR/Anime4K.zip"

echo -e "${BLUE}📦 Downloading Anime4K GLSL package...${NC}"

python3 - "$URL" "$TARGET_FILE" << 'PYEOF'
import sys, urllib.request, time

url, output_file = sys.argv[1], sys.argv[2]

def format_size(bytes_num):
    if bytes_num >= 1024**2:
        return f"{bytes_num / (1024**2):.1f} MB"
    elif bytes_num >= 1024:
        return f"{bytes_num / 1024:.0f} KB"
    return f"{bytes_num} B"

req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})
with urllib.request.urlopen(req) as response, open(output_file, 'wb') as out:
    total_size = int(response.headers.get('Content-Length', 0))
    downloaded = 0
    import shutil
    start_time = time.time()
    last_update = 0
    color_cyan, color_green, color_dim, color_bold, color_reset = "\033[36m", "\033[32m", "\033[2m", "\033[1m", "\033[0m"

    while True:
        chunk = response.read(65536)
        if not chunk:
            break
        out.write(chunk)
        downloaded += len(chunk)
        now = time.time()
        if now - last_update > 0.08 or downloaded == total_size:
            last_update = now
            elapsed = now - start_time
            speed = downloaded / elapsed if elapsed > 0 else 0
            percent = (downloaded / total_size) * 100 if total_size > 0 else 0
            
            term_cols = shutil.get_terminal_size((80, 24)).columns
            d_str, t_str, s_str = format_size(downloaded), format_size(total_size), f"{format_size(speed)}/s"
            overhead = 35 + len(d_str) + len(t_str) + len(s_str)
            bar_length = max(10, term_cols - overhead)
            
            filled_len = int(bar_length * downloaded // total_size) if total_size > 0 else 0
            bar = '━' * filled_len + color_dim + '━' * (bar_length - filled_len) + color_reset
            sys.stdout.write(f"\r\033[K  {color_green}⠋{color_reset} [{color_cyan}{bar}{color_reset}] {color_bold}{percent:5.1f}%{color_reset}  ({d_str} / {t_str})  {color_cyan}{s_str}{color_reset}")
            sys.stdout.flush()

sys.stdout.write("\n")
PYEOF

set -eo pipefail

if [ ! -f "$TARGET_FILE" ] || [ ! -s "$TARGET_FILE" ]; then
    echo -e "${RED}❌ Download failed or archive is empty!${NC}"
    rm -rf "$TMP_DIR"
    exit 1
fi

unzip -q -o "$TARGET_FILE" -d "$TMP_DIR"

echo -e "${BLUE}📂 Copying Anime4K shaders to ${TARGET}/shaders...${NC}"
mkdir -p "$TARGET/shaders"
cp -rf "$TMP_DIR"/shaders/* "$TARGET/shaders/" 2>/dev/null || true

# Handle input.conf safely: preserve existing custom keybindings
if [ -f "$TARGET/input.conf" ]; then
    if ! grep -q "Anime4K" "$TARGET/input.conf" 2>/dev/null; then
        echo -e "${BLUE}📝 Appending Anime4K hotkeys to existing ${TARGET}/input.conf...${NC}"
        echo "" >> "$TARGET/input.conf"
        cat "$TMP_DIR/input.conf" >> "$TARGET/input.conf"
    else
        echo -e "${BLUE}ℹ️ Anime4K hotkeys already present in ${TARGET}/input.conf${NC}"
    fi
else
    echo -e "${BLUE}📝 Creating ${TARGET}/input.conf with Anime4K hotkeys...${NC}"
    cp "$TMP_DIR/input.conf" "$TARGET/input.conf"
fi

# Configure Anime4K default shader preset in mpv.conf
if [ -f "$TARGET/mpv.conf" ] && grep -q "glsl-shaders=" "$TARGET/mpv.conf" 2>/dev/null; then
    echo -e "${BLUE}⚙️ Updating glsl-shaders preset in ${TARGET}/mpv.conf...${NC}"
    sed -i "s|^#* *glsl-shaders=.*|${MPV_GLSL_LINE}|" "$TARGET/mpv.conf"
    sed -i "s|^# Optimized shaders.*|${HEADER_COMMENT}|" "$TARGET/mpv.conf"
else
    echo -e "${BLUE}⚙️ Appending glsl-shaders preset to ${TARGET}/mpv.conf...${NC}"
    cat << INNER_EOF >> "$TARGET/mpv.conf"

${HEADER_COMMENT}
${MPV_GLSL_LINE}
INNER_EOF
fi

rm -rf "$TMP_DIR"
echo -e "${GREEN}🎉 Anime4K shaders (${PRESET_NAME}) installed and configured for mpv!${NC}"
