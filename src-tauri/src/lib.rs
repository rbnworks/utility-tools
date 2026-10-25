use dashmap::DashMap;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock, Mutex};
use tauri::Emitter;

use hash::compute_hashes_for_file;
use scan::FileHash;

pub mod scan;
pub mod hash;
pub mod operations;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ScanStatus {
    Idle,
    Scanning,
    Complete,
    Cancelled,
    Error(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanState {
    pub session_id: String,
    pub status: ScanStatus,
    pub folder_paths: Vec<String>,
    pub total_files: usize,
    pub scanned_files: usize,
    pub duplicate_groups: Vec<DuplicateGroup>,
    #[serde(skip)]
    pub abort_requested: Arc<AtomicBool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DuplicateGroup {
    pub group_id: String,
    pub file_count: usize,
    pub files: Vec<String>,
}

static SESSIONS: LazyLock<DashMap<String, ScanState>> =
    LazyLock::new(DashMap::new);

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

/// Hash cache keyed by file path: (FileHash, file_size)
/// Prevents re-computation of the same file (D-08)
type HashCache = Mutex<HashMap<String, (FileHash, u64)>>;

#[tauri::command]
async fn scan_folders(
    app: tauri::AppHandle,
    folder_paths: Vec<String>,
    options: ScanOptions,
) -> Result<SessionInfo, String> {
    // Validate each path exists and is a directory
    for path in &folder_paths {
        match std::fs::metadata(path) {
            Ok(metadata) => {
                if !metadata.is_dir() {
                    return Err(format!("Path is not a directory: {}", path));
                }
            }
            Err(e) => {
                return Err(format!("Path does not exist: {} ({})", path, e))
            }
        }
    }

    let session_id = uuid::Uuid::new_v4().to_string();
    let folder_count = folder_paths.len();

    // Create session state
    let state = ScanState {
        session_id: session_id.clone(),
        status: ScanStatus::Scanning,
        folder_paths: folder_paths.clone(),
        total_files: 0,
        scanned_files: 0,
        duplicate_groups: Vec::new(),
        abort_requested: Arc::new(AtomicBool::new(false)),
    };
    SESSIONS.insert(session_id.clone(), state);

    // Discover files
    let files = scan::discover_files(
        folder_paths,
        options.include_subdirectories,
        options.min_file_size_bytes,
    );

    let total = files.len();

    // Update total files
    if let Some(mut s) = SESSIONS.get_mut(&session_id) {
        s.total_files = total;
    }

    // Hash cache (D-08)
    let hash_cache: HashCache = Mutex::new(HashMap::new());

    // Process files and collect hashes
    let mut all_hashes: Vec<FileHash> = Vec::new();

    for (i, file) in files.iter().enumerate() {
        // Check if cancellation was requested
        {
            let mut s = SESSIONS.get_mut(&session_id).ok_or("Session not found")?;
            if s.abort_requested.load(Ordering::SeqCst) {
                s.status = ScanStatus::Cancelled;
                s.total_files = total;
                s.scanned_files = i;

                let _ = app.emit(
                    "scan:progress",
                    scan::ScanProgressEvent {
                        scanned: i,
                        total,
                        file: String::new(),
                    },
                );
                let _ = app.emit(
                    "scan:cancelled",
                    serde_json::json!({
                        "session_id": session_id,
                        "scanned": i,
                        "total": total,
                    }),
                );
                return Ok(SessionInfo {
                    session_id,
                    folder_count,
                });
            }
        }

        // Emit progress event
        let _ = app.emit(
            "scan:progress",
            scan::ScanProgressEvent {
                scanned: i + 1,
                total,
                file: file.path.clone(),
            },
        );

        // Update scanned count
        if let Some(mut s) = SESSIONS.get_mut(&session_id) {
            s.scanned_files = i + 1;
        }

        // Check hash cache (D-08)
        let file_hash = {
            if let Ok(cache) = hash_cache.lock() {
                if let Some((hashed, _)) = cache.get(&file.path) {
                    hashed.clone()
                } else {
                    drop(cache);
                    compute_hashes_for_file(&file.path, file.file_type)?
                }
            } else {
                compute_hashes_for_file(&file.path, file.file_type)?
            }
        };

        // Store in cache
        if let Ok(mut cache) = hash_cache.lock() {
            cache.insert(file.path.clone(), (file_hash.clone(), file.size));
        }

        all_hashes.push(file_hash);
    }

    // Group duplicates using triple-agreement (HASH-05)
    let mut duplicate_groups: Vec<DuplicateGroup> = Vec::new();
    let mut group_id_counter: u32 = 0;
    let mut grouped: Vec<bool> = vec![false; all_hashes.len()];

    for i in 0..all_hashes.len() {
        if grouped[i] {
            continue;
        }

        let mut group_files: Vec<String> = vec![all_hashes[i].path.clone()];
        grouped[i] = true;

        for j in (i + 1)..all_hashes.len() {
            if grouped[j] {
                continue;
            }

            if hash::triple_match(
                &all_hashes[i].ahash,
                &all_hashes[j].ahash,
                &all_hashes[i].dhash,
                &all_hashes[j].dhash,
                &all_hashes[i].phash,
                &all_hashes[j].phash,
            ) {
                group_files.push(all_hashes[j].path.clone());
                grouped[j] = true;
            }
        }

        if group_files.len() > 1 {
            group_id_counter += 1;
            duplicate_groups.push(DuplicateGroup {
                group_id: format!("group-{}", group_id_counter),
                file_count: group_files.len(),
                files: group_files,
            });
        }
    }

    // Emit duplicate found events for each group
    for group in &duplicate_groups {
        let _ = app.emit("scan:duplicate_found", group);
    }

    // Update session state
    if let Some(mut s) = SESSIONS.get_mut(&session_id) {
        s.status = ScanStatus::Complete;
        s.duplicate_groups = duplicate_groups.clone();
    }

    // Emit complete event
    let _ = app.emit(
        "scan:complete",
        serde_json::json!({
            "session_id": session_id,
            "total_files": total,
            "total_groups": duplicate_groups.len(),
        }),
    );

    Ok(SessionInfo {
        session_id,
        folder_count,
    })
}

/// Compute all three perceptual hashes for a file on disk.
/// Uses Rayon parallel computation for the three hash algorithms (D-04).
/// Only handles images; use hash::compute_hashes_for_file() for video support.
fn compute_file_hash(path: &str) -> Result<FileHash, String> {
    let bytes = std::fs::read(path).map_err(|e| {
        format!("Failed to read file {}: {}", path, e)
    })?;

    let size = bytes.len() as u64;

    // Decode image once
    let img = hash::load_image(&bytes).map_err(|e| format!("Failed to decode image: {}", e))?;

    // Rayon parallel hash computation (D-04)
    // Compute aHash, dHash, pHash in parallel via Rayon thread pool
    let imgs = [&img, &img, &img];
    let hash_results: [[u8; 8]; 3] = imgs
        .par_iter()
        .enumerate()
        .map(|(idx, img)| match idx {
            0 => hash::compute_ahash(img),
            1 => hash::compute_dhash(img),
            2 => hash::compute_phash(img),
            _ => [0u8; 8],
        })
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| "Failed to collect 3 hash results".to_string())?;

    Ok(FileHash {
        path: path.to_string(),
        ahash: hash_results[0],
        dhash: hash_results[1],
        phash: hash_results[2],
        size,
    })
}

/// Lookup a session by its ID and return a clone of its state.
pub fn get_session(session_id: &str) -> Option<ScanState> {
    SESSIONS.get(session_id).map(|s| s.clone())
}

/// Cancel an in-progress scan by session ID (DF-SCAN-09).
#[tauri::command]
async fn cancel_scan(
    app: tauri::AppHandle,
    session_id: String,
) -> Result<String, String> {
    // Find session in DashMap
    if let Some(mut entry) = SESSIONS.get_mut(&session_id) {
        match &entry.status {
            ScanStatus::Scanning => {
                entry.status = ScanStatus::Cancelled;
                entry.abort_requested.store(true, Ordering::SeqCst);
                drop(entry);

                // Emit scan:cancelled event with session_id
                let _ = app.emit(
                    "scan:cancelled",
                    serde_json::json!({
                        "session_id": session_id,
                    }),
                );

                Ok("Scan cancelled".to_string())
            }
            ScanStatus::Complete | ScanStatus::Cancelled => {
                Ok("Scan already finished".to_string())
            }
            ScanStatus::Error(_) => {
                Ok("Scan already errored".to_string())
            }
            ScanStatus::Idle => {
                Ok("No scan in progress".to_string())
            }
        }
    } else {
        Err("Session not found".to_string())
    }
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
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![scan_folders, cancel_scan])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
