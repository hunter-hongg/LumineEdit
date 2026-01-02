// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path)
        .map_err(|e| format!("读取文件失败: {}", e))
}

#[tauri::command]
fn save_file_dialog(path: String, content: String) -> Result<String, String> {
    let path = &path;
    std::fs::write(path, content)
        .map(|_| format!("保存到 {}", path))
        .map_err(|e| format!("保存失败: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init()) 
        .invoke_handler(tauri::generate_handler![
            read_file,
            save_file_dialog,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}