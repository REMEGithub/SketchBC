
use std::{
    fs,
    path::PathBuf,
    sync::Mutex,
};

use base64::{
    engine::general_purpose::STANDARD,
    Engine,
};

use serde::Serialize;
use tauri::State;

// ================================
// 1. 全局图片状态
// ================================

#[derive(Default)]
pub struct ImageFolderState {
    pub folder_path: Mutex<Option<PathBuf>>,
    pub images: Mutex<Vec<PathBuf>>,
}

// ================================
// 2. 返回给 Vue 的数据结构
// ================================

#[derive(Serialize)]
pub struct ImageFolderInfo {
    pub folder_name: String,
    pub image_count: usize,
}

#[derive(Serialize)]
pub struct ImageData {
    pub mime_type: String,
    pub data_base64: String,
}

// ================================
// 3. 图片格式判断
// ================================

fn is_supported_image(path: &PathBuf) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            matches!(
                ext.to_ascii_lowercase().as_str(),
                "jpeg" | "jpg" | "png" | "webp" | "avif"
            )
        })
        .unwrap_or(false)
}

// ================================
// 4. 选择文件夹并扫描
// ================================

#[tauri::command]
pub fn set_image_folder(
    folder_path: String,
    state: State<'_, ImageFolderState>,
) -> Result<ImageFolderInfo, String> {

    let folder = PathBuf::from(&folder_path);

    if !folder.is_dir() {
        return Err("无效的文件夹路径".to_string());
    }

    let entries = fs::read_dir(&folder)
        .map_err(|e| format!("无法读取文件夹: {e}"))?;

    let mut images = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_file() && is_supported_image(&path) {
            images.push(path);
        }
    }

    // 按文件名排序
    images.sort_by(|a, b| {
        a.file_name().cmp(&b.file_name())
    });

    let image_count = images.len();

    let folder_name = folder
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("图片文件夹")
        .to_string();

    // 更新全局状态
    *state.folder_path.lock()
        .map_err(|e| e.to_string())? = Some(folder);

    *state.images.lock()
        .map_err(|e| e.to_string())? = images;

    Ok(ImageFolderInfo {
        folder_name,
        image_count,
    })
}

// ================================
// 5. 获取图片数量
// ================================

#[tauri::command]
pub fn get_image_count(
    state: State<'_, ImageFolderState>,
) -> Result<usize, String> {

    let images = state.images.lock()
        .map_err(|e| e.to_string())?;

    Ok(images.len())
}

// ================================
// 6. 按索引读取图片
// ================================

#[tauri::command]
pub fn read_image(
    index: usize,
    state: State<'_, ImageFolderState>,
) -> Result<ImageData, String> {

    // 先获取路径，随后释放锁
    let path = {
        let images = state.images.lock()
            .map_err(|e| e.to_string())?;

        images.get(index)
            .cloned()
            .ok_or("图片索引不存在".to_string())?
    };

    // 读取文件内容
    let bytes = fs::read(&path)
        .map_err(|e| format!("图片读取失败: {e}"))?;

    // MIME 类型
    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let mime_type = match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "webp" => "image/webp",
        "avif" => "image/avif",
        _ => return Err("不支持的图片格式".to_string()),
    };

    Ok(ImageData {
        mime_type: mime_type.to_string(),
        data_base64: STANDARD.encode(bytes),
    })
}

#[tauri::command]
pub fn clear_image_folder(
    state: State<'_, ImageFolderState>,
) -> Result<(), String> {
    // 清空文件夹路径
    let mut folder = state.folder_path.lock()
        .map_err(|e| e.to_string())?;

    *folder = None;

    // 清空图片列表
    let mut images = state.images.lock()
        .map_err(|e| e.to_string())?;

    images.clear();

    Ok(())
}
