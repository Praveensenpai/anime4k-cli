# 📺 anime4k-mpv-installer

Automated Anime4K GLSL shader and keybinding installer for **mpv**.

---

## ✨ Features

- 🎮 **GPU Preset Selector**: Interactively choose between High-end (HQ VL shaders) and Low-end (Fast M/S shaders).
- 📦 **Automated Shader Download**: Downloads official Anime4K v4.0.1 GLSL shaders.
- 📝 **Hotkeys & Config**: Safely appends Anime4K hotkeys to `input.conf` and default shader preset to `mpv.conf`.

---

## 📦 Quick Start

Run this one-liner in your terminal:

```bash
curl -sSL https://raw.githubusercontent.com/Praveensenpai/anime4k-mpv-installer/main/install.sh | bash
```

---

## ⚡ Usage

```bash
# Interactive GPU preset menu
anime4k-mpv-installer

# Non-interactive low-end GPU preset
ANIME4K_PRESET=low anime4k-mpv-installer

# Non-interactive high-end GPU preset
ANIME4K_PRESET=high anime4k-mpv-installer
```

---

## 📜 License

MIT
