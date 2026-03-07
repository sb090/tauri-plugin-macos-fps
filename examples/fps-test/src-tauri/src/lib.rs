use tauri_plugin_macos_fps::MacFpsExt;

#[tauri::command]
fn unlock_fps(webview: tauri::Webview) -> Result<(), String> {
    webview.unlock_fps().map_err(|e| e.to_string())
}

#[tauri::command]
fn lock_fps(webview: tauri::Webview) -> Result<(), String> {
    webview.lock_fps().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    tauri::Builder::default()
        .plugin(tauri_plugin_macos_fps::init())
        .invoke_handler(tauri::generate_handler![unlock_fps, lock_fps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
