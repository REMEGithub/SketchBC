// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod image_manager;
use image_manager::{
    ImageFolderState,
    set_image_folder,
    get_image_count,
    read_image,
    clear_image_folder,
};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 新增文件夹选择插件
        .plugin(tauri_plugin_dialog::init())
        // 注册图片管理全局状态
        .manage(ImageFolderState::default())
        .invoke_handler(tauri::generate_handler![
            greet,
            set_image_folder,
            get_image_count,
            read_image,
            clear_image_folder
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
