# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2026-03-07

### Added

- Initial release
- Automatic >60fps unlock via `on_webview_ready` lifecycle hook
- `MacFpsExt` trait for per-webview `unlock_fps()` / `lock_fps()` control
- `Config` support via `tauri.conf.json` (`plugins.macos-fps.enabled`)
- Graceful degradation — never crashes, logs warnings on failure
- No-op on non-macOS platforms
- Example test app (`examples/fps-test/`) for verification
