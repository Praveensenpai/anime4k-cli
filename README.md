# 📺 anime4k (anime4k-cli)

> Modern, blazing-fast automated Anime4K GLSL shader and configuration manager for **mpv**.

Zero runtime dependencies. Written in Rust. Native support for Linux, macOS, and Windows.

---

## ✨ Features

- 🎮 **Interactive GPU Preset Selector**: Arrow-key menu for High-end (HQ VL) vs Low-end (Fast M/S) shaders.
- 📦 **Automated Shader Management**: Fast, secure download and extraction of Anime4K v4.0.1 GLSL shaders.
- 📝 **Safe Config Management**: Structured `# >>> anime4k-managed-start >>>` blocks that protect your custom `mpv.conf` and `input.conf` with automatic timestamped backups.
- ⚡ **Instant Preset Switcher**: Switch between HQ and Fast presets on the fly (`anime4k preset high` / `anime4k preset low`).
- 🩺 **Doctor / Health Check**: Diagnose mpv version, shader files, video output driver (`vo=gpu-next`), and keybindings with `anime4k doctor`.
- 🗑️ **Clean Uninstaller**: Safely removes managed configuration blocks and shader files with `anime4k uninstall`.
- 🪟 **Cross-Platform**: Linux (x86_64, aarch64), macOS (Intel, Apple Silicon), and native Windows (`.exe` / PowerShell).

---

## 📦 Quick Start

### 🐧 Linux & 🍎 macOS
```bash
curl -fsSL https://raw.githubusercontent.com/Praveensenpai/anime4k-cli/main/install.sh | sh
```

### 🪟 Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/Praveensenpai/anime4k-cli/main/install.ps1 | iex
```

### 🦀 Cargo
```bash
cargo install --git https://github.com/Praveensenpai/anime4k-cli.git --bin anime4k
```

---

## ⚡ Usage

```bash
# Launch interactive TUI menu
anime4k

# Direct installation with specific GPU preset
anime4k install --preset high       # Higher-end GPU (HQ - VL Shaders)
anime4k install --preset low        # Lower-end GPU / iGPU (Fast - M/S Shaders)
anime4k install --yes               # Non-interactive default install

# Switch active preset in mpv.conf instantly
anime4k preset high
anime4k preset low

# Run system diagnostic & configuration health check
anime4k doctor

# View hotkey cheatsheet & shader modes
anime4k info

# Safely uninstall Anime4K configuration and shaders
anime4k uninstall
```

---

## 🎮 MPV Hotkeys Cheat Sheet

| Keybinding | Shader Mode | Description |
| :--- | :--- | :--- |
| `CTRL+1` | **Mode A (HQ/Fast)** | Optimized for 1080p Anime (Restore + Upscale) |
| `CTRL+2` | **Mode B (HQ/Fast)** | Denoise + Deblur (great for older/grainy anime) |
| `CTRL+3` | **Mode C (HQ/Fast)** | Deblur only (for clean, low-noise anime) |
| `CTRL+4` | **Mode A+A** | Higher quality perceptual upscale for 1080p → 4K |
| `CTRL+5` | **Mode B+B** | Higher quality perceptual denoise & deblur |
| `CTRL+6` | **Mode C+A** | Higher quality deblur + upscale |
| `CTRL+0` | **Clear Shaders** | Disables Anime4K and restores standard playback |
| `Shift+I` | **mpv Stats** | Displays real-time render stats and active GLSL shaders |

---

## 🛠️ Building from Source

```bash
git clone https://github.com/Praveensenpai/anime4k-cli.git
cd anime4k-cli
cargo build --release
./target/release/anime4k --help
```

---

## 📜 License

[MIT](LICENSE)
