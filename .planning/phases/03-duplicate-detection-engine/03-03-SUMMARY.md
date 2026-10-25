# Plan 03-03: Video File Discovery — Duplicate Detection Engine

## What was built

This plan adds video file discovery support to the scanning module. It extends discover_files to recognize video files, adds the ffmpeg-next Cargo dependency, and introduces the FileType enum for video/image distinction.

### Files created/modified

| File | Action | Purpose |
|------|--------|---------|
| `src-tauri/Cargo.toml` | Modified | Added `ffmpeg-next = "9"` dependency (version 9 for FFmpeg 9.x compatibility) |
| `src-tauri/src/scan.rs` | Modified | Added VIDEO_EXTENSIONS, SUPPORTED_EXTENSIONS, FileType enum, is_video_file(), is_image_file(); extended ScanFile with file_type field; discover_files() now filters by SUPPORTED_EXTENSIONS and sets FileType |

### Requirements satisfied

- **DF-SCAN-05**: Video file discovery alongside images

### Design decisions

1. **ffmpeg-next 9.x** — System has FFmpeg 9.0.2, so `ffmpeg-next = "9"` was used instead of the planned "7". A stub `avfft.h` was created for FFmpeg 9 compatibility (header was removed in FFmpeg 5.0+).
2. **FileType enum** — `Image` and `Video` variants with `Copy, PartialEq, Eq` derives for efficient comparison.
3. **SUPPORTED_EXTENSIONS** — Combined list of both image and video extensions for unified filtering.
4. **File type detection** — Based on extension matching against VIDEO_EXTENSIONS first, then IMAGE_EXTENSIONS.

### Verification

- `cd src-tauri && cargo check` — passes with zero errors (1 dead code warning for is_image_extension, unused until lib.rs routing)
- ScanFile struct has file_type: FileType field
- discover_files() filters by SUPPORTED_EXTENSIONS (not just images)
- File type correctly set in discover_files loop
