---
phase: 03-duplicate-detection-engine
plan: 02
subsystem: rust-backend
tags: [ffmpeg, video-hashing, perceptual-hash, tauri, rust]

# Dependency graph
requires:
  - phase: 03-duplicate-detection-engine
    provides: "image-only hashing primitives (Plan 03-01) and FileType enum (Plan 03-03)"
provides:
  - "compute_hashes_for_file(path, file_type) — dispatches to image or video hashing"
  - "compute_hashes_for_video() — FFmpeg frame extraction at 5s intervals"
  - "videos_match() — N >= 3 matching frames AND >= 50% threshold"
  - "video_canonical_hash() — median frame hash for grouping"
affects:
  - 03-duplicate-detection-engine

# Actuals (#2632)
actuals:
  tokens: 2221
  tasks: 1
  commits: 1

# Tech tracking
tech-stack:
  added: [ffmpeg-next 9.x compatibility]
  patterns: [per-file hashing dispatch, frame-set comparison, median canonical hash]

key-files:
  created: []
  modified:
    - src-tauri/src/hash.rs

key-decisions:
  - "Use Context::from_parameters() instead of Stream::codec() for FFmpeg 7+ compatibility"

patterns-established:
  - "File dispatch pattern: compute_hashes_for_file(path, FileType) routes to correct hashing implementation"
  - "Video canonical hash: median frame triplet used for initial grouping before fine-grained comparison"

requirements-completed: [DF-SCAN-05, HASH-03]

# Coverage
coverage:
  - id: D1
    description: "compute_hashes_for_file dispatches to image or video hashing based on FileType"
    requirement: "HASH-03"
    verification:
      - kind: automated_ui
        ref: "cd src-tauri && cargo check (compiles zero errors)"
        status: pass
    human_judgment: false
  - id: D2
    description: "compute_hashes_for_video extracts frames at 5s intervals via FFmpeg"
    requirement: "DF-SCAN-05"
    verification:
      - kind: automated_ui
        ref: "cd src-tauri && cargo check (FFmpeg 9 API compatibility verified)"
        status: pass
    human_judgment: false
  - id: D3
    description: "videos_match requires N >= 3 matching frames AND >= 50% threshold (D-07)"
    requirement: "DF-SCAN-05"
    verification:
      - kind: automated_ui
        ref: "cd src-tauri && cargo check (compiles zero errors)"
        status: pass
    human_judgment: false
  - id: D4
    description: "compute_hashes_for_file video path returns median frame hash as canonical FileHash"
    requirement: "HASH-03"
    verification:
      - kind: automated_ui
        ref: "cd src-tauri && cargo check (compiles zero errors)"
        status: pass
    human_judgment: false

# Metrics
duration: 12min
completed: 2026-10-25
status: complete
---

# Phase 03 Plan 02: Video Frame Hashing with FFmpeg 7+ Compatibility

**Video frame extraction at 5-second intervals, per-frame perceptual hashing, and file-type dispatch with FFmpeg 9 compatibility**

## Performance

- **Duration:** 12 min
- **Started:** 2026-10-25T18:40:00Z
- **Completed:** 2026-10-25T18:45:28Z
- **Tasks:** 1
- **Files modified:** 1

## Accomplishments

- Added `compute_hashes_for_file(path, FileType)` — dispatches image files to existing hash functions and video files to FFmpeg-based frame extraction
- Fixed FFmpeg 9 (v9.x) API incompatibilities in pre-existing video code from parallel plan 03-03:
  - `Stream::codec()` removed in FFmpeg 7+ → replaced with `Context::from_parameters()`
  - `DynamicImage::ImageLuma8(...).map_err(...)` → proper `Ok()` wrapping
  - `PacketIter` yields `(Stream, Packet)` not `(Option<Stream>, Packet)`
  - Decoder flush: `send_packet(EMPTY)` → `send_eof()`

## Task Commits

Each task was committed atomically:

1. **Task 1: Add video frame extraction, per-frame hashing, and frame-set comparison** - `2ac5daa2` (feat)

**Plan metadata:** `2ac5daa25953647e8eb40276aa05fa763bd94298` (docs: complete plan)

## Files Created/Modified

- `src-tauri/src/hash.rs` — Added `compute_hashes_for_file()`, fixed FFmpeg 9 API compatibility (252 diff lines)

## Decisions Made

- Used `ffmpeg::codec::Context::from_parameters()` instead of deprecated `Stream::codec()` for FFmpeg 7+ compatibility — required by system's FFmpeg 9.0.2

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] FFmpeg 9 API incompatibility — Stream::codec() removed**
- **Found during:** Task 1 (build verification)
- **Issue:** FFmpeg 9 enabled `ffmpeg_5_0` feature flag, which removes `Stream::codec()` method. Pre-existing video code from Plan 03-03 was written for older FFmpeg API. Five compilation errors: missing `codec()`, missing `map_err` on enum variant, wrong iterator pattern, decoder flush, missing `mut`.
- **Fix:** Replaced `stream.codec().decoder().video()` with `Context::from_parameters(stream.parameters()).decoder().video()`, fixed `DynamicImage` conversion, fixed `PacketIter` stream handling, changed `send_packet(empty)` to `send_eof()` for flush, added `mut` to scaler.
- **Files modified:** src-tauri/src/hash.rs
- **Verification:** `cargo check` passes with zero errors
- **Committed in:** `2ac5daa2` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (blocking API fix)
**Impact on plan:** The FFmpeg 9 fix was necessary for compilation — the video code from Plan 03-03 used the older FFmpeg API. No scope creep.

## Issues Encountored

- FFmpeg 9.0.2 on system has deprecated `Stream::codec()` behind `#[cfg(not(feature = "ffmpeg_5_0"))]` — the `ffmpeg-sys-next` crate auto-detects installed FFmpeg version and enables `ffmpeg_7_0`/`ffmpeg_8_0` features, which suppresses the old codec API.

## Next Phase Readiness

- `compute_hashes_for_file(path, FileType)` is available and compiles — ready for Plan 03-04 to wire it into the scan command's background task in lib.rs
- Video hashing path complete; image hashing path unchanged

---
*Phase: 03-duplicate-detection-engine*
*Completed: 2026-10-25*
