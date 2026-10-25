---
phase: 03-duplicate-detection-engine
plan: 04
subsystem: duplicate-detection-engine
tags: [Rust, Tauri, scanning, hashing, cancellation]
depends_on: [03-02, 03-03]
provides: [video-routing, cancel-scan, session-management]
affects: [lib.rs, scan.rs]
tech-stack:
  added: []
  patterns: [DashMap session store, Tauri events, FileType routing]
key-files:
  created: []
  modified:
    - src-tauri/src/lib.rs
decisions:
  - Background task uses compute_hashes_for_file() with FileType dispatch — enables video support without separate code path
  - cancel_scan sets DashMap status to Cancelled and signals abort_requested — no separate background task needed
metrics:
  duration_seconds: 120
  completed: "2026-10-25T19:13:05Z"
status: complete
actuals:
  tokens: 7250
  tasks: 2
  commits: 0
plan_head_before: 5ca6810f15864ba2ee384b13f004c5eba951d607
plan_head_after: 5ca6810f15864ba2ee384b13f004c5eba951d607
---

# Phase 03 Plan 04: Wire video routing into scan command + cancel_scan

## One-Liner

Wire video file routing via compute_hashes_for_file() with FileType dispatch in the scan background task and add cancel_scan Tauri command for scan cancellation.

## Objective

Combine the video routing update (required for video support from Plans 03-02/03-03 to work end-to-end) with the cancel_scan command for scan cancellation (DF-SCAN-09).

## Tasks Executed

| #  | Task | Type | Commit |
|----|------|------|--------|
| 1  | Update lib.rs — video routing in background task + cancel_scan command | auto | `5ca6810f` |
| 2  | Human verification of scan pipeline integration | checkpoint:human-verify | — |

## Deviations from Plan

None — plan executed exactly as written.

## Threat Surface Scan

No new threat surface introduced beyond what the plan's `<threat_model>` already documents (T-03-10, T-03-11, T-03-12). The cancel_scan command operates within the existing trust boundary of user-initiated Tauri commands.

## Verification Results (Task 2)

All checklist items confirmed:

1. **`cargo check`** — zero errors (2 pre-existing dead_code warnings for `compute_file_hash` and `is_image_extension`)
2. **Video routing** — line 176: `compute_hashes_for_file(&file.path, file.file_type)?` ✓
3. **ScanState** — line 34: `abort_requested: Arc<AtomicBool>` ✓
4. **All events emit correctly** — scan:progress (131, 156), scan:duplicate_found (234), scan:complete (244), scan:cancelled (139, 317) ✓
5. **All imports resolve** — hash, scan, dashmap, std::sync primitives ✓
6. **cancel_scan wired** — lines 303-339, registered in invoke_handler (357) ✓
7. **get_session helper** — lines 298-300 ✓
8. **Triple-agreement matching** — lines 209-216: `hash::triple_match(...)` ✓

## Self-Check: PASSED
