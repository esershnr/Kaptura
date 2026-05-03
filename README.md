# 🚀 Kaptura 1.0 - Video Capture Utility

Kaptura is a high-performance, low-latency **UVC (USB Video Class)** capture interface designed for Windows. It is optimized for gaming capture cards and professional webcams, providing a minimalist yet powerful tool for creators.

[🇹🇷 Türkçe README için tıklayın](README.tr.md)

![Performance](https://img.shields.io/badge/Performance-Optimized-blueviolet?style=for-the-badge)
![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=for-the-badge)
![Engine](https://img.shields.io/badge/Engine-GStreamer-green?style=for-the-badge)

---

## ✨ Key Features

### 🛡️ Parasite & Glitch Prevention
Many capture cards experience audio "crackling" or "parasites" due to USB bandwidth or clock sync issues. Kaptura solves this using a custom **100ms Leaky Downstream Buffer** architecture. This ensures audio stays fresh and smooth by dropping stale packets while maintaining minimal latency.

### 👤 Stealth Mode (Designed for Discord)
Originally developed for **Discord streaming**, Stealth Mode allows you to share or monitor content discreetly. When activated:
- Window borders and title bars are hidden.
- Taskbar presence is minimized.
- Provides a clean, borderless video-only view, perfect for screen sharing without UI clutter.

### ⚡ Intelligent Hardware Priority
Kaptura automatically scans for devices and prioritizes those with keywords like **"USB3", "Capture", or "Game"**, initializing the stream instantly upon startup.

### 🖱️ UI Toggle via Double-Click
Double-click anywhere on the video feed to instantly hide or show the control panel for an uninterrupted viewing experience.

---

## 📊 System Specs & Requirements

- **Memory Usage:** Low footprint (~150MB - 250MB RAM depending on resolution).
- **Disk Space:** Approx. **1.5GB - 2GB** (includes full GStreamer runtimes and DLL dependencies).
- **Storage Requirement:** Note that the `dist` folder is large due to necessary media libraries.
- **Hardware:** Compatible with **any UVC-compliant capture card** or camera.

---

## 📦 Build & Distribution

The `dist` folder is not included in the repository due to its size (~2GB). To build your own:

1. Install **GStreamer MSVC 1.24+** runtime.
2. Build the project: `cargo build --release`.
3. Create a `dist` folder and organize it as follows:

```text
dist/
├── kaptura.exe          # Compiled binary from target/release
├── assets/
│   └── icon.ico         # App icon
├── lib/
│   └── gstreamer-1.0/   # GStreamer plugins (copy from runtime)
└── *.dll                # Essential GStreamer DLLs (copy from runtime bin)
```

4. Use the provided `kaptura_setup.iss` with **Inno Setup** to generate the installer.

---

## 📄 License
This project is licensed under the MIT License.
