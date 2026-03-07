# Contributing to tauri-plugin-macos-fps

Contributions are welcome! Here's how to get started.

## Getting started

```sh
git clone https://github.com/userFRM/tauri-plugin-macos-fps.git
cd tauri-plugin-macos-fps
cargo check
```

The plugin compiles on all platforms. macOS-specific code is behind `#[cfg(target_os = "macos")]` and compiles as a no-op on Linux/Windows.

## Testing on macOS

The only way to verify the FPS unlock is on a real Mac with a high-refresh display:

```sh
cd examples/fps-test
cargo install tauri-cli --version "^2"
cargo tauri dev
```

The test app shows a live FPS counter and a toggle button.

## Code structure

```
src/
  lib.rs      Plugin entry point, Config, MacFpsExt trait
  macos.rs    Core unsafe objc2 code (macOS only)
examples/
  fps-test/   Minimal Tauri v2 app for verification
```

## Guidelines

- **Don't break graceful degradation.** Every ObjC call in `macos.rs` must have a null check and log warning. The plugin must never crash.
- **Don't add `objc2-web-kit`.** We use private APIs not covered by public bindings — raw `msg_send!` is intentional.
- **Keep it minimal.** This plugin does one thing. Resist scope creep.
- **Test on macOS.** CI can verify compilation, but actual FPS verification requires hardware.

## Submitting changes

1. Fork the repo
2. Create a branch (`git checkout -b my-fix`)
3. Make your changes
4. Run `cargo check` and `cargo clippy`
5. Open a pull request

## Reporting issues

Please include:
- macOS version
- Display type and refresh rate (ProMotion, external monitor Hz)
- Tauri version
- Console log output (look for `tauri-plugin-macos-fps:` prefixed messages)

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 dual license.
