# Plan 03-01: Core Rust Backend — Duplicate Detection Engine

## What was built

This plan implements the core Rust backend for image duplicate detection. It establishes the complete scan pipeline: file discovery → perceptual hashing → duplicate grouping → event emission.

### Files created/modified

| File | Action | Purpose |
|------|--------|---------|
| `src-tauri/Cargo.toml` | Modified | Added dependencies: `image`, `rayon`, `tokio`, `walkdir`, `tauri-plugin-shell`, `dashmap` |
| `src-tauri/src/scan.rs` | Created | File discovery module — `discover_files()` with walkdir traversal, extension filtering, system dir skipping, deduplication |
| `src-tauri/src/hash.rs` | Created | Perceptual hashing module — aHash (average), dHash (differential), pHash (DCT-based 2D DCT on 32x32), `hamming_distance()`, `triple_match()` |
| `src-tauri/src/operations.rs` | Created | File operations stub for Phase 5 (keep/delete/move) |
| `src-tauri/src/lib.rs` | Rewritten | Module declarations, `ScanState` with `ScanStatus` enum, `DashMap` session store, `scan_folders` Tauri command with tokio background task, Rayon parallel hash computation, Tauri event emission (scan:progress, scan:duplicate_found, scan:complete) |

### Requirements satisfied

- **DF-SCAN-01**: Folder path validation in scan command
- **DF-SCAN-02**: File discovery with walkdir
- **DF-SCAN-03**: Extension filtering (IMAGE_EXTENSIONS)
- **DF-SCAN-06**: Scan progress events via Tauri events
- **HASH-01**: aHash via custom implementation
- **HASH-02**: dHash via custom implementation
- **HASH-03**: pHash via 2D DCT on 32x32 grayscale (NOT pixel sampling)
- **HASH-04**: Hamming distance comparison with threshold
- **HASH-05**: Triple-agreement duplicate grouping

### Design decisions

1. **aHash and dHash implemented manually** — No `image-hash` crate exists on crates.io. Standard algorithms implemented from specification.
2. **pHash uses 2D DCT-II** — Follows HASH-03 spec: 32x32 resize → grayscale → DCT → 63 low-frequency coefficients → median thresholding.
3. **Rayon parallel per-file** — Each file's 3 hashes (aHash, dHash, pHash) computed in parallel via `par_iter()` on a 3-element array.
4. **DashMap session store** — Thread-safe concurrent session management (D-03).
5. **Hash cache (D-08)** — `Mutex<HashMap<String, (FileHash, u64)>>` prevents re-computation of same file path.
6. **Triple-agreement matching** — All three algorithms must have Hamming distance <= 5 (D-06, HASH-05).

### Verification

- `cd src-tauri && cargo check` — zero errors, all dependencies compile
- aHash: 8x8 resize → grayscale → average thresholding
- dHash: 9x8 resize → grayscale → adjacent pixel comparison
- pHash: 32x32 resize → grayscale → 2D DCT-II → 63 low-freq coefficients → median
- hamming_distance: XOR + count_ones per byte
- triple_match: all three distances <= 5
- scan:progress events emit with {scanned, total, file}
- scan:duplicate_found events emit per group
- scan:complete event emits with session_id, total_files, total_groups
