# Phase 03: Duplicate Detection Engine - Context

**Gathered:** 2026-10-25
**Status:** Ready for planning

<domain>
## Phase Boundary

Implement the Rust backend that scans user-selected directories for image/video files, computes perceptual hashes (aHash + dHash + pHash) for each file, groups duplicates using triple-agreement matching with Hamming distance threshold ≤ 5, emits real-time Tauri events for progress and duplicate detection, supports cancellation, and maintains an in-memory session store.

**In scope (from REQUIREMENTS.md):** DF-SCAN-01 through DF-SCAN-09, HASH-01 through HASH-05
**Out of scope:** Results display (Phase 4), file operations (Phase 5), UI components (all frontend phases)

</domain>

<decisions>
## Implementation Decisions

### Video file handling
- **D-01:** Use `image` crate primary with `ffmpeg-next` bindings for full format support — MP4, MOV, AVI, MKV, WebM all supported via FFmpeg frame extraction — **Reversibility:** reversible — Adding FFmpeg support now is easy; removing it later would require stripping the dependency and all frame extraction code. The ~50MB binary size increase is accepted as the tradeoff for complete format coverage.

### Scan architecture
- **D-02:** Modular file structure: `scan.rs` (file discovery), `hash.rs` (perceptual hashing), `operations.rs` (file operations stubs) — **Reversibility:** reversible — Files can be merged later if needed, but separation aids testing and matches the existing codebase structure from phases 1-2.

### Session management
- **D-03:** Use `DashMap` crate for concurrent session store — supports multiple concurrent scan sessions, thread-safe hash map access without locking — **Reversibility:** reversible — Can downgrade to `Arc<RwLock<ScanState>>` if the dependency proves unnecessary, but DashMap is well-tested and the architecture is already designed for multi-session use.

### Performance — parallelism
- **D-04:** Use Rayon for parallel hash computation — scan files sequentially but compute each file's three hashes (aHash, dHash, pHash) in parallel using Rayon's thread pool — **Reversibility:** reversible — Rayon adds one dependency and is safe/easy to remove. Good throughput with minimal complexity.

### Hashing approach
- **D-05:** Three hash algorithms with triple-agreement matching: aHash (8x8 resize, mean brightness), dHash (9x8 resize, adjacent pixel pairs), pHash (32x32 DCT, low-frequency components) — locked from PROJECT.md — **Reversibility:** costly — Changing the algorithm mix would require reworking the entire hash.rs module and re-tuning thresholds.
- **D-06:** Hamming distance threshold ≤ 5 out of 64 bits per algorithm — locked from REQUIREMENTS.md — **Reversibility:** reversible — Thresholds are constants that can be adjusted without architectural changes.

### Video frame extraction
- **D-07:** Extract frames at regular intervals (every 5 seconds), hash each frame, compare frame sets between videos. If N frames match across two videos where N ≥ 3 and ≥ 50% of analyzed frames match, consider them duplicates. Fallback: hash first frame only if video has fewer than 3 extractable frames — per PLAN.md — **Reversibility:** reversible — Frame extraction interval and matching thresholds are configurable constants.

### In-memory hash cache
- **D-08:** Maintain an in-memory hash cache per file path during a scan session (no re-computation) — stored as part of the session state in DashMap — **Reversibility:** reversible — Cache is session-scoped and discarded after scan completes.

### Event emission
- **D-09:** Tauri events: `scan:progress` (scanned/total/file), `scan:duplicate_found` (group_id, file_count, files), `scan:complete` (total_files, total_groups), `scan:cancelled` — **Reversibility:** costly — Event structure is consumed by the frontend's DuplicateFinder.tsx component.
- **D-10:** Use `tokio::spawn` for the background scan task, communicate progress via channels to Tauri event emission — **Reversibility:** reversible — Task lifecycle can be changed; the event system remains the same.

### The agent's Discretion
- Choice of specific Rust crates for image processing (beyond `image` and `ffmpeg-next` — e.g., `image-hash` for pre-built hash implementations vs. custom implementations)
- Exact file scanning filter patterns (what constitutes "hidden files" or "system directories" beyond the obvious `.git`, `node_modules`)
- Hash cache data structure internals (e.g., whether to pre-compute all three hashes per file or compute on-demand)
- Error handling granularity during scan (per-file errors vs. group-level errors in events)

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Requirements & Roadmap
- `.planning/ROADMAP.md` §3 — Phase 3 goal, success criteria, dependencies
- `.planning/REQUIREMENTS.md` — DF-SCAN-01 through DF-SCAN-09, HASH-01 through HASH-05 (13 requirements)
- `.planning/PROJECT.md` — Tech stack (Tauri v2, Rust), key decisions (three hash algorithms)

### Technical Specification
- `PLAN.md` — Full technical spec: architecture diagram, Tauri commands, hash algorithm details, video handling approach, Rust dependencies

### Existing Code
- `src-tauri/src/lib.rs` — Current stub: `scan_folders` command, `ScanOptions` struct, `SessionInfo` struct, Tauri plugin setup
- `src-tauri/Cargo.toml` — Current dependencies (no `image` or `image-hash` yet)
- `src-tauri/tauri.conf.json` — App config, window settings, CSP
- `src-tauri/build.rs` — Tauri build script
- `src/components/DuplicateFinder.tsx` — Frontend scan orchestrator, event listener setup (`scan:progress`), `invoke` calls

### Rust/Tauri
- `src-tauri/src/main.rs` — Tauri entry point
- [Rust documentation](https://doc.rust-lang.org/rust-by-example/) — Language reference
- [Tauri v2 documentation](https://v2.tauri.app/) — Framework reference
- [Rayon documentation](https://docs.rs/rayon) — Parallel iterator library
- [DashMap documentation](https://docs.rs/dashmap) — Concurrent hash map
- [image crate documentation](https://docs.rs/image) — Image processing
- [ffmpeg-next documentation](https://docs.rs/ffmpeg-next) — Video frame extraction

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `ScanOptions` struct in `src-tauri/src/lib.rs` — Already defined with `includeSubdirectories`, `includeVideoFiles`, `minFileSizeBytes` fields. Reusable as-is for the scan implementation.
- `SessionInfo` struct in `src-tauri/src/lib.rs` — Return type for `scan_folders`. May need extension to carry more data.
- `uuid` crate already in Cargo.toml — Session ID generation already available.
- `tauri::AppHandle` pattern — Used in existing `scan_folders` command for accessing Tauri context and emitting events.
- `serde` derive macros — Already used for `ScanOptions` and `SessionInfo`. New structs (hash results, duplicate groups) should follow same pattern.

### Established Patterns
- **Tauri command structure:** `#[tauri::command]` async fn with `AppHandle`, arguments, and `Result<SuccessType, String>` return. All new commands should follow this pattern.
- **Plugin registration:** Plugins initialized in `.setup()` closure, then exposed via `invoke_handler`.
- **Error handling:** Commands return `Result<T, String>` with descriptive error messages.
- **Frontend event listening:** `listen<T>("scan:progress", callback)` pattern in `DuplicateFinder.tsx` using `@tauri-apps/api/event`.

### Integration Points
- **Frontend ↔ Backend:** `invoke("scan_folders", { folderPaths, options })` triggers backend scan. Tauri events flow Rust → React via event system.
- **`scan_folders` command in lib.rs:** Current stub validates paths and returns session ID. Must be expanded to kick off the background scan task.
- **Event naming convention:** `scan:progress`, `scan:duplicate_found`, `scan:complete` — already expected by frontend.
- **Session lifecycle:** Frontend expects a `session_id` back from `scan_folders`. Backend must track session state using this ID.

</code_context>

<specifics>
## Specific Ideas

No specific ideas — open to standard approaches for Rust/Tauri implementation.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 03-Duplicate Detection Engine*
*Context gathered: 2026-10-25*
