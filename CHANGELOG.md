# Changelog
 
All notable changes to this project will be documented in this file.
 
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
