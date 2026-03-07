# tauri-plugin-macos-fps — LLM Development Guide

> Tauri v2 plugin that unlocks >60fps on macOS by disabling WKWebView's frame rate cap.

## What this plugin does

WKWebView caps `requestAnimationFrame` at 60fps via the `PreferPageRenderingUpdatesNear60FPSEnabled` WebKit preference. This plugin sets it to `false` using the private `_features` API on `WKPreferences`.

On non-macOS platforms, all code compiles to no-ops.

## Architecture

```
src/
  lib.rs      Plugin init(), Config, MacFpsExt trait, PluginState
  macos.rs    Core unsafe objc2 code (cfg(target_os = "macos") only)
examples/
  fps-test/   Minimal Tauri v2 app with FPS counter + toggle button
```

## Key types

| Type | Purpose |
|---|---|
| `Config` | Deserializable from `tauri.conf.json`. Single field: `enabled: bool` (default `true`) |
| `PluginState` | Internal managed state holding the `enabled` flag |
| `MacFpsExt<R>` | Extension trait on `Webview<R>` — `unlock_fps()` / `lock_fps()` |

## How the macOS code works

1. `with_webview` gives a raw `*mut c_void` pointer to the `WKWebView`
2. `msg_send![wk_webview, configuration]` → `WKWebViewConfiguration`
3. `msg_send![config, preferences]` → `WKPreferences`
4. `msg_send![AnyClass::get("WKPreferences"), _features]` → `NSArray<_WKFeature *>`
5. Iterate array, compare each feature's `.key` (NSString) to `"PreferPageRenderingUpdatesNear60FPSEnabled"`
6. `msg_send![preferences, _setEnabled: Bool::new(false), forFeature: feature]`

All steps have null checks with log warnings. Failure is graceful.

## Private API details

- `_features` is a **class method** on `WKPreferences`
- `_setEnabled:forFeature:` is an **instance method** on `WKPreferences`
- `_WKFeature` is opaque — we never define it, just use `*mut AnyObject`
- These APIs are used by Safari internally and have been stable for years
- **Not App Store safe** — causes rejection in Mac App Store review

## Common tasks

### Test compilation
```sh
cargo check                              # Linux/Windows (no-op path)
cd examples/fps-test && cargo tauri dev  # macOS (actual verification)
```

### Change the target preference key
Edit `TARGET_KEY` in `src/macos.rs`.

## What NOT to do

- Don't add `objc2-web-kit` — the APIs we use are private and not in public bindings
- Don't remove null checks in `macos.rs` — they're the graceful degradation path
- Don't make this a Tauri command — it's a lifecycle hook, not an IPC endpoint
