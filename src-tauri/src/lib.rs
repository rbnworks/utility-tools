use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct ScanOptions {
    #[serde(rename = "includeSubdirectories")]
    pub include_subdirectories: bool,
    #[serde(rename = "includeVideoFiles")]
    pub include_video_files: bool,
    #[serde(rename = "minFileSizeBytes")]
    pub min_file_size_bytes: u64,
}

#[derive(Debug, Serialize, Clone)]
pub struct SessionInfo {
    pub session_id: String,
    pub folder_count: usize,
}

#[tauri::command]
async fn scan_folders(
    _app: tauri::AppHandle,
    folder_paths: Vec<String>,
    _options: ScanOptions,
) -> Result<SessionInfo, String> {
    // Validate each path exists and is a directory
    for path in &folder_paths {
        match std::fs::metadata(path) {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return Err(format!("Path is not a directory: {}", path));
                }
            }
            Err(e) => return Err(format!("Path does not exist: {} ({})", path, e)),
        }
    }

    let session_id = uuid::Uuid::new_v4().to_string();

    Ok(SessionInfo {
        session_id,
        folder_count: folder_paths.len(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![scan_folders])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
