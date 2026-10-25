use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use walkdir::WalkDir;

pub const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "webp", "gif", "bmp", "tiff", "tif",
];

pub const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mov", "avi", "mkv", "webm",
];

pub const SUPPORTED_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "webp", "gif", "bmp", "tiff", "tif",
    "mp4", "mov", "avi", "mkv", "webm",
];

pub const SKIP_DIRS: &[&str] = &[
    ".git", "node_modules", ".venv", "__pycache__", ".idea", ".vscode",
    "target", "dist", "build",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Image,
    Video,
}

#[derive(Debug, Clone)]
pub struct ScanFile {
    pub path: String,
    pub size: u64,
    pub extension: String,
    pub file_type: FileType,
}

#[derive(Debug, Clone)]
pub struct FileHash {
    pub path: String,
    pub ahash: [u8; 8],
    pub dhash: [u8; 8],
    pub phash: [u8; 8],
    pub size: u64,
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct ScanProgressEvent {
    pub scanned: usize,
    pub total: usize,
    pub file: String,
}

fn is_video_extension(ext: &str) -> bool {
    VIDEO_EXTENSIONS
        .iter()
        .any(|e| e.eq_ignore_ascii_case(ext))
}

fn is_image_extension(ext: &str) -> bool {
    IMAGE_EXTENSIONS
        .iter()
        .any(|e| e.eq_ignore_ascii_case(ext))
}

fn is_allowed_extension(ext: &str) -> bool {
    SUPPORTED_EXTENSIONS
        .iter()
        .any(|e| e.eq_ignore_ascii_case(ext))
}

fn is_skip_dir(name: &str) -> bool {
    SKIP_DIRS.iter().any(|d| *d == name)
}

pub fn discover_files(
    folder_paths: Vec<String>,
    include_subdirs: bool,
    min_size: u64,
) -> Vec<ScanFile> {
    let seen_paths: Mutex<HashMap<String, ()>> = Mutex::new(HashMap::new());
    let mut files: Vec<ScanFile> = Vec::new();

    for folder_path in folder_paths {
        let path = Path::new(&folder_path);
        if !path.exists() || !path.is_dir() {
            continue;
        }

        let walker = if include_subdirs {
            WalkDir::new(path).into_iter()
        } else {
            // Top-level only: max_depth(1)
            WalkDir::new(path).max_depth(1).into_iter()
        };

        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let file_name = match entry.file_name().to_str() {
                Some(n) => n.to_string(),
                None => continue,
            };

            // Skip hidden files/directories
            if file_name.starts_with('.') {
                continue;
            }

            // Skip system directories
            if is_skip_dir(&file_name) {
                continue;
            }

            // Skip non-files
            if !entry.file_type().is_file() {
                continue;
            }

            // Check extension
            let ext = match entry.path().extension() {
                Some(e) => e.to_string_lossy().to_lowercase().to_string(),
                None => continue,
            };

            if !is_allowed_extension(&ext) {
                continue;
            }

            // Check file size
            let size = match entry.metadata() {
                Ok(m) => m.len(),
                Err(_) => continue,
            };

            if size < min_size {
                continue;
            }

            // Deduplicate by canonicalized path
            let canonical = match entry.path().canonicalize() {
                Ok(p) => p.to_string_lossy().to_string(),
                Err(_) => entry.path().to_string_lossy().to_string(),
            };

            if let Ok(mut seen) = seen_paths.lock() {
                if seen.contains_key(&canonical) {
                    continue;
                }
                seen.insert(canonical.clone(), ());
            }

            // Determine file type
            let file_type = if is_video_extension(&ext) {
                FileType::Video
            } else {
                FileType::Image
            };

            files.push(ScanFile {
                path: canonical,
                size,
                extension: ext,
                file_type,
            });
        }
    }

    files
}
