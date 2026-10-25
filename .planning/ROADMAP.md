# ROADMAP: Utility Tools

**Created:** 2026-10-25
**Total Phases:** 6
**v1 Requirements:** 36
**All v1 requirements covered:** ✓

| # | Phase | Goal | Requirements | Success Criteria |
|---|-------|------|--------------|------------------|
| 1 | App Shell | Desktop app with dark-themed sidebar navigation | SHELL-01 to SHELL-05 | 5 |
| 2 | Duplicate Finder — Input | Folder picker, scan options, scan trigger | DF-INPUT-01 to DF-INPUT-07 | 7 |
| 3 | Duplicate Detection Engine | Rust backend: scan, hash, match, events | DF-SCAN-01 to DF-SCAN-09, HASH-01 to HASH-05 | 13 |
| 4 | Duplicate Results | Real-time results display with thumbnails and actions | DF-RESULT-01 to DF-RESULT-06 | 6 |
| 5 | Duplicate Actions | Keep/delete/move operations with batch and undo | DF-ACTION-01 to DF-ACTION-07 | 7 |
| 6 | Polish & Testing | Error handling, keyboard shortcuts, end-to-end validation | (cross-phase) | 5+ |

---

## Phase 1: App Shell

**Goal:** Desktop app with dark-themed sidebar navigation, layout, and tool routing.

**Mode:** standard

**Success Criteria:**
1. App launches as a Tauri desktop window with dark theme applied
2. Left sidebar shows utility tools list with icons (lucide-react), active state, and expandable section
3. Clicking a sidebar item renders its content area; default tool shown on launch
4. All shadcn/ui and Kokonut UI components are installed and importable
5. Project structure is correct: `src/lib/utils.ts` exists, `cn()` helper resolves via path alias

**Requirements:** SHELL-01, SHELL-02, SHELL-03, SHELL-04, SHELL-05

---

## Phase 2: Duplicate Finder — Folder Input

**Goal:** Folder picker with scan options and scan trigger UI.

**Mode:** standard

**Success Criteria:**
1. User opens native OS folder picker dialog via Tauri and selects folders
2. User can manually type folder paths and add them to the list
3. Added folders display with remove button; paths are validated (exist on disk)
4. Scan options UI: checkboxes for subdirectories, video files; numeric input for min file size
5. Clicking "Scan for Duplicates" collects paths + options and calls the Tauri command
6. UI transitions from input view to progress view after scan starts
7. Input form resets properly when a new scan is started

**Requirements:** DF-INPUT-01, DF-INPUT-02, DF-INPUT-03, DF-INPUT-04, DF-INPUT-05, DF-INPUT-06, DF-INPUT-07

**Plans:** 3 plans
- [x] 02-01-PLAN.md — Install dependencies: Tauri JS packages, Rust plugins, shadcn/ui components
- [x] 02-02-PLAN.md — Build FolderPicker (tracer) + ScanOptions components
- [x] 02-03-PLAN.md — Wire DuplicateFinder orchestrator + ScanProgress component

---

## Phase 3: Duplicate Detection Engine

**Goal:** Rust backend that scans files, computes perceptual hashes, groups duplicates, and emits real-time events.

**Mode:** standard

**Success Criteria:**
1. Rust backend recursively scans directories, filters by supported extensions and min file size, skips hidden/system dirs
2. For each image file: aHash, dHash, and pHash are computed correctly
3. For each video file: frames are extracted, each frame is hashed, frame sets are compared for duplication
4. Duplicate grouping works: files with matching hash fingerprints are grouped together
5. Hamming distance matching works: distance ≤ 5 per algorithm, all three must agree
6. Tauri events emit correctly: `scan:progress`, `scan:duplicate_found`, `scan:complete`
7. User can cancel an in-progress scan and the backend stops cleanly
8. Hash cache is maintained per file during a session (no re-computation)

**Requirements:** DF-SCAN-01, DF-SCAN-02, DF-SCAN-03, DF-SCAN-04, DF-SCAN-05, DF-SCAN-06, DF-SCAN-07, DF-SCAN-08, DF-SCAN-09, HASH-01, HASH-02, HASH-03, HASH-04, HASH-05

**Wave structure:** Wave 1 = 03-01 (core scanner), Wave 2 = 03-03 (scan.rs video support), Wave 3 = 03-02 (hash.rs video support), Wave 4 = 03-04 (combined lib.rs wiring)

Plans:
- [x] 03-01-PLAN.md — Core scanner: file discovery, image hashing (aHash/dHash/pHash), progress events, duplicate grouping with triple-agreement matching
- [x] 03-03-PLAN.md — Video scan support: ffmpeg-next dependency, video extensions, FileType enum in scan.rs
- [x] 03-02-PLAN.md — Video hash support: ffmpeg frame extraction, per-frame hashing, videos_match(), compute_hashes_for_file()
- [x] 03-04-PLAN.md — lib.rs integration: video routing in scan command, DashMap session state, cancel command, session wiring

---

## Phase 4: Duplicate Results

**Goal:** Real-time display of found duplicates with thumbnails, file info, and per-file actions.

**Mode:** standard

**Success Criteria:**
1. Results display shows summary: total groups, affected files, recoverable space
2. Each duplicate group renders as a card with thumbnail previews (loaded from file paths)
3. Each file in a group displays name, size, and truncated path (full path on hover via tooltip)
4. Default selection: first file = keep (green check), rest = delete (red X)
5. Results update in real-time as Tauri events fire during scanning (duplicates appear as they're found)
6. Group cards collapse/expand; user can show/collapse all groups
7. UI handles zero results state gracefully (scan complete, no duplicates found)

**Requirements:** DF-RESULT-01, DF-RESULT-02, DF-RESULT-03, DF-RESULT-04, DF-RESULT-05, DF-RESULT-06

---

## Phase 5: Duplicate Actions

**Goal:** Per-file and batch actions (keep, delete, move) with confirmation and undo.

**Mode:** standard

**Success Criteria:**
1. User can change any file's action to keep, delete, or move to folder
2. User can mark all files in a group for batch action at once
3. "Delete all marked" permanently removes all deleted files across all groups
4. "Move all marked to..." moves files to a single target folder specified by user
5. Batch operations show a confirmation dialog before execution (irreversible warning)
6. Move operation auto-renames on conflict: `photo.jpg` → `photo_1.jpg` → `photo_2.jpg`
7. After a batch operation, undo button appears for N seconds to reverse the action
8. Undo re-creates deleted files or moves them back from target folder
9. Each operation reports success/failure per file

**Requirements:** DF-ACTION-01, DF-ACTION-02, DF-ACTION-03, DF-ACTION-04, DF-ACTION-05, DF-ACTION-06, DF-ACTION-07

---

## Phase 6: Polish & Testing

**Goal:** Error handling, keyboard shortcuts, edge cases, and end-to-end validation with real files.

**Mode:** mvp

**Success Criteria:**
1. Permission errors on inaccessible folders display friendly error messages
2. Corrupted or unreadable files are skipped with a warning (not a crash)
3. Keyboard shortcuts work: Ctrl/Cmd+Enter to start scan, Escape to cancel
4. All shadcn/ui and Kokonut UI components render correctly in dark theme
5. Tested with real folders containing known duplicates (different names, different sizes, resized versions)
6. Perceptual hash accuracy validated: near-identical images detected, unrelated images not flagged
7. Auto-rename on move conflict tested with 10+ conflicting filenames
8. Undo tested: batch delete → undo → files restored; batch move → undo → files restored
9. Clean state after scan complete: results persist, can start new scan from input view
10. App builds successfully for the target platform with no warnings

**Requirements:** (cross-phase validation — no new requirements, validates all previous phases)

---

## Dependencies

| Phase | Depends On |
|-------|-----------|
| Phase 1 | None |
| Phase 2 | Phase 1 |
| Phase 3 | Phase 1, Rust installed |
| Phase 4 | Phase 3 (needs Tauri events) |
| Phase 5 | Phase 4 (needs results display), Phase 3 (needs Tauri commands) |
| Phase 6 | All phases |

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| Rust not installed or crates fail to compile on macOS ARM | Install Rust in Phase 1 setup; test early in Phase 1; have fallback hash implementation ready |
| Video frame extraction fails for some formats | Use `image` crate's built-in support first; add `ffmpeg` binding only if needed |
| Perceptual hash accuracy insufficient | Tune Hamming thresholds during Phase 6 testing; adjust based on real file results |
| Tauri event wiring issues between Rust and React | Test event emission and reception in Phase 3 with a simple echo test before full implementation |

---
*Roadmap created: 2026-10-25*
*Last updated: 2026-10-25 after initial creation*
