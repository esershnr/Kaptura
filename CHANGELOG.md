# Changelog
 
All notable changes to this project will be documented in this file.
 
## [1.2.0] - 2026-05-12
 
### Added
- **GPU-Accelerated Sharpening**: Implemented a 3x3 Laplacian sharpening filter directly in the WGSL fragment shader, allowing for real-time detail recovery without CPU cost.
- **Interactive Control Panel**: Upgraded value displays to interactive `DragValue` inputs, supporting both fine-tuned dragging and manual keyboard entry for all image adjustments.
- **Improved UI Layout**: Unified the technical and creative controls into an optimized dual-panel layout, resolving overflow issues on small resolutions.
- **Action Grid**: Reorganized mode toggles (Stealth, Logs, Audio Only) into a high-visibility grid with improved color contrast for better accessibility.

### Optimized
- **Minimal Latency Pipeline**: Reduced crossbeam channel capacity to `bounded(1)` and set `desired_maximum_frame_latency` to `1` in the wgpu swapchain, ensuring the fastest possible visual response.
- **Shader Uniform Management**: Implemented efficient per-frame uniform buffer updates for dynamic post-processing.

---

 
## [1.1.0] - 2026-05-04
 
### Added
- **NV12 Pixel Format Support**: Native support for NV12, optimizing performance for budget capture cards and reducing CPU overhead.
- **Win32 Ghost Mode (Stealth)**: Complete rewrite of Stealth Mode using low-level Win32 API for ultimate invisibility and click-through functionality.
- **Borderless Fullscreen Stealth**: Stealth Mode now automatically triggers borderless fullscreen for high-quality Discord streaming.
- **Global Hotkey System**: 
    - `SHIFT + ESC`: Global exit from Stealth Mode (even when not focused).
    - `SHIFT + F10`: Global UI toggle (Hide/Show overlay).
- **Manual Format Selection**: Users can now explicitly choose between Auto, MJPG, YUY2, and NV12.
 
### Fixed
- **Discord Freeze Issues**: Fixed the issue where Discord streams would freeze when the window was moved or minimized during Stealth Mode.
- **UI Restoration**: Resolved bugs where window decorations (close buttons, frames) would disappear after exiting Stealth Mode.
- **Input Lag**: Optimized the GStreamer pipeline for low-end capture cards to achieve near-zero latency.
 
### Changed
- Improved overall stability of the Win32 window management logic.
- Updated to modern `HasWindowHandle` API for better future compatibility.
 
---
 
## [1.0.0] - 2026-04-29
- Initial stable release with basic capture and GStreamer support.
