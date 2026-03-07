# tauri-plugin-macos-fps

[![Crates.io](https://img.shields.io/crates/v/tauri-plugin-macos-fps.svg)](https://crates.io/crates/tauri-plugin-macos-fps)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org)

**Unlock >60fps on macOS for Tauri v2 apps.** One line of code. 120Hz ProMotion, 144Hz+ external displays — your Tauri app finally renders at the display's native refresh rate.

---

## The problem

WKWebView on macOS **caps `requestAnimationFrame` at 60fps** — regardless of your display's actual refresh rate. Your MacBook Pro has a 120Hz ProMotion display? Your Tauri app is stuck at 60fps. Your external monitor runs at 144Hz or 240Hz? Still 60fps.

Meanwhile, native macOS apps and even Safari tabs render at the full display refresh rate.

This is tracked in:
- [tauri-apps/tauri#13978](https://github.com/tauri-apps/tauri/issues/13978) — closed as "not planned"
- [WebKit Bug #173434](https://bugs.webkit.org/show_bug.cgi?id=173434) — open since 2017

The Tauri maintainers said *"WKWebView simply does not expose settings for that."*

They're wrong.

## The fix

WebKit has an internal preference called `PreferPageRenderingUpdatesNear60FPSEnabled` (defaults to `true`). This plugin sets it to `false` via the private `_features` API on `WKPreferences` — the same mechanism Safari uses internally to control frame rate behavior.

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_macos_fps::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**That's it.** Every webview now renders at your display's native refresh rate.

## Install

Add to your Tauri app's `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-macos-fps = "0.1"
```

Minimum supported Rust version: **1.85**.

## Try it yourself

A test app is included to verify the fix on your Mac:

```sh
git clone https://github.com/userFRM/tauri-plugin-macos-fps.git
cd tauri-plugin-macos-fps/examples/fps-test

# Install Tauri CLI if you don't have it
cargo install tauri-cli --version "^2"

# Run the test app
cargo tauri dev
```

The app shows a real-time FPS counter with a large display. You should see:
- **Without plugin:** ~60 FPS (WKWebView cap)
- **With plugin (default):** 120 FPS on ProMotion, 144+ on high-refresh external displays

A toggle button lets you switch between locked (60fps) and unlocked (native) in real time.

## Configuration

Optional. Set in `tauri.conf.json` to disable the plugin without removing it:

```json
{
  "plugins": {
    "macos-fps": {
      "enabled": false
    }
  }
}
```

Default: `enabled: true` — all webviews are automatically unlocked.

## Per-webview control

For fine-grained control, use the extension trait:

```rust
use tauri_plugin_macos_fps::MacFpsExt;

// Unlock native refresh rate on a specific webview:
webview.unlock_fps()?;

// Re-lock to 60fps:
webview.lock_fps()?;
```

## How it works

```
WKWebView
  → .configuration  → WKWebViewConfiguration
    → .preferences   → WKPreferences
      → [WKPreferences _features]  → NSArray<_WKFeature *>
        → find key == "PreferPageRenderingUpdatesNear60FPSEnabled"
          → [preferences _setEnabled:NO forFeature:feature]
```

The plugin hooks into Tauri's `on_webview_ready` lifecycle event. Every webview is automatically unlocked as it's created. The private `_features` API returns all WebKit feature flags; we find our target key and toggle it off.

**Why private APIs?** Because Apple's public WKWebView API provides no way to control this. The `_features` / `_setEnabled:forFeature:` selectors have been stable across macOS releases and are used by Safari internally. Wry (Tauri's webview layer) already uses private WKPreferences APIs for other features like `fullScreenEnabled`.

## Platform behavior

| Platform | Effect |
|---|---|
| **macOS** (WKWebView) | Unlocks native refresh rate (120Hz ProMotion, 144Hz+, etc.) |
| **Windows** (WebView2) | No-op — WebView2 already renders at native refresh rate |
| **Linux** (WebKitGTK) | No-op — typically 60fps, no known toggle |
| **iOS** (WKWebView) | No-op — untested, may work in future versions |

The plugin compiles and runs on all platforms. On non-macOS, `init()` registers a plugin that does nothing.

## App Store warning

This plugin uses WebKit's **private `_features` API**. Apps submitted to the **Mac App Store will be rejected** during review.

This plugin is intended for:
- **Direct distribution** (`.dmg`, Homebrew, GitHub releases, etc.)
- **Internal / enterprise** apps
- **Development and testing**

If you need Mac App Store distribution, do not use this plugin.

## Graceful degradation

The plugin never crashes. If the private API changes in a future macOS version:

| Scenario | Behavior |
|---|---|
| `_features` returns nil | Warning logged, app continues at 60fps |
| Target preference key not found | Warning logged, app continues at 60fps |
| `WKPreferences` class missing | Warning logged, app continues at 60fps |
| WKWebView pointer is null | Warning logged, app continues at 60fps |

All failure paths log via the standard `log` crate at `WARN` level and fall back silently to the default 60fps behavior.

## Verify in any Tauri app

Paste this in your browser console or embed it in your frontend to measure actual FPS:

```javascript
let frames = 0, lastTime = performance.now();
(function measure() {
    frames++;
    const now = performance.now();
    if (now - lastTime >= 1000) {
        console.log(`${Math.round(frames * 1000 / (now - lastTime))} FPS`);
        frames = 0;
        lastTime = now;
    }
    requestAnimationFrame(measure);
})();
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

## Links

- [tauri-apps/tauri#13978](https://github.com/tauri-apps/tauri/issues/13978) — Original Tauri issue
- [WebKit Bug #173434](https://bugs.webkit.org/show_bug.cgi?id=173434) — WebKit tracking bug (open since 2017)
- [WebKit Source — PreferPageRenderingUpdatesNear60FPSEnabled](https://github.com/nicolo-ribaudo/tc39-proposal-signals-webkit/blob/main/Source/WTF/Scripts/Preferences/WebPreferences.yaml) — Preference definition in WebKit source
