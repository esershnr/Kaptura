# 🚀 Kaptura 1.2.0 - Video Capture Utility

Kaptura is a high-performance, low-latency **UVC (USB Video Class)** capture interface designed for Windows. It is optimized for gaming capture cards and professional webcams, providing a minimalist yet powerful tool for creators.

[🇹🇷 Türkçe README için tıklayın](README.tr.md)

![Performance](https://img.shields.io/badge/Performance-Optimized-blueviolet?style=for-the-badge)
![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange?style=for-the-badge)
![Engine](https://img.shields.io/badge/Engine-GStreamer-green?style=for-the-badge)

---

## ✨ Key Features

### 🛡️ Parasite & Glitch Prevention
Many capture cards experience audio "crackling" or "parasites" due to USB bandwidth or clock sync issues. Kaptura solves this using a custom **100ms Leaky Downstream Buffer** architecture. This ensures audio stays fresh and smooth by dropping stale packets while maintaining minimal latency.

### 👤 Stealth Mode (v1.2.0 - Win32 Ghost Mode)
Designed for **Discord streaming**, Stealth Mode now utilizes low-level Win32 API calls for the ultimate capture experience:
- **Invisible & Click-through**: The window remains active for capture but is invisible and ignores mouse clicks for the user.
- **Borderless Fullscreen**: Automatically covers the entire screen to provide high-quality, bar-free streams.
- **Discord Optimized**: Prevents stream freezing or pausing by remaining technically "visible" to the DWM.

### ⚡ Intelligent Hardware Priority & NV12
Kaptura prioritizes hardware with keywords like **"USB3", "Capture", or "Game"** and now includes native **NV12 format selection** for optimized budget capture card performance.

### 🎨 Pixel Formats & Hardware Optimization
Kaptura provides explicit control over the hardware's pixel output to help you balance visual quality and system stability:

- **NV12 (Recommended)**: The most balanced format for modern capture. It offers high performance and low latency with minimal CPU overhead. Best for 1080p/60fps on most budget cards.
- **YUY2 (High Fidelity)**: Provides the richest, uncompressed color data. However, due to its massive USB bandwidth requirements, it can cause audio "jitter" or "crackling" on low-end hardware or USB 2.0 ports. While Kaptura’s custom buffering minimizes this, it remains a hardware-bound limitation. Use this for maximum quality if your hardware allows.
- **MJPG (Compatibility)**: Uses hardware compression. Ideal for achieving high frame rates on older USB 2.0 capture cards, at the cost of slight compression artifacts.

### ⌨️ Shortcuts & Controls

| Shortcut | Action | Scope |
| :--- | :--- | :--- |
| `SHIFT + ESC` | **Exit Stealth Mode** | Global (Stealth only) |
| `SHIFT + F10` | **Toggle UI Visibility** | Global (Always) |
| `Double-Click` | **Toggle UI Visibility** | Local (Normal only) |
| `ESC` | **Close Application** | Local (Normal only) |

### 🧩 New Features in v1.2.0
- **GPU-Accelerated Sharpening**: Recover details from low-end capture cards with zero CPU overhead.
- **Interactive Controls**: Values can now be manually typed or dragged for extreme precision.
- **Optimized Latency**: Reduced buffer overhead for faster response.

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
