# Requirements: Utility Tools

**Defined:** 2026-10-25
**Core Value:** Find visual duplicates across folders — even when files have different names, sizes, or timestamps — and give the user full control over which copy to keep.

## v1 Requirements

### App Shell

- [ ] **SHELL-01**: Desktop app launches with a dark-themed UI
- [ ] **SHELL-02**: Left sidebar displays utility tools list with icons and labels
- [ ] **SHELL-03**: Clicking a sidebar item renders its content in the main area
- [ ] **SHELL-04**: Active tool is highlighted in the sidebar
- [ ] **SHELL-05**: Sidebar includes expandable section for future tools

### Duplicate Finder — Input

- [ ] **DF-INPUT-01**: User can add folders via native OS dialog picker
- [ ] **DF-INPUT-02**: User can manually enter folder paths
- [ ] **DF-INPUT-03**: User can remove added folders
- [ ] **DF-INPUT-04**: User can toggle subdirectory scanning
- [ ] **DF-INPUT-05**: User can toggle video file inclusion
- [ ] **DF-INPUT-06**: User can set a minimum file size filter
- [ ] **DF-INPUT-07**: Clicking "Scan for Duplicates" triggers the scan

### Duplicate Finder — Scanning

- [ ] **DF-SCAN-01**: Backend recursively scans selected directories for image/video files
- [ ] **DF-SCAN-02**: Backend skips hidden files and system directories (node_modules, .git, etc.)
- [ ] **DF-SCAN-03**: For each file, compute three perceptual hashes: aHash, dHash, pHash
- [ ] **DF-SCAN-04**: Group files by hash fingerprint to identify duplicates
- [ ] **DF-SCAN-05**: Videos are handled by extracting frames and comparing frame-level hashes
- [ ] **DF-SCAN-06**: Emit scan progress events (files scanned / total)
- [ ] **DF-SCAN-07**: Emit duplicate-found events as groups are identified
- [ ] **DF-SCAN-08**: Emit scan-complete event with final counts
- [ ] **DF-SCAN-09**: User can cancel an in-progress scan

### Duplicate Finder — Results Display

- [ ] **DF-RESULT-01**: Results display shows total duplicate groups, affected files, recoverable space
- [ ] **DF-RESULT-02**: Each group displays as a card with thumbnail previews
- [ ] **DF-RESULT-03**: Each file in a group shows name, size, and full path
- [ ] **DF-RESULT-04**: Paths are truncated in display with full path on hover (tooltip)
- [ ] **DF-RESULT-05**: Default selection: first file = keep (green check), rest = delete (red X)
- [ ] **DF-RESULT-06**: Results update in real-time as duplicates are found during scan

### Duplicate Finder — Actions

- [ ] **DF-ACTION-01**: User can change per-file action: keep, delete, or move to folder
- [ ] **DF-ACTION-02**: User can mark all files in a group for batch action
- [ ] **DF-ACTION-03**: Batch "Delete all marked" removes all deleted files across groups
- [ ] **DF-ACTION-04**: Batch "Move all marked to..." moves files to a single target folder
- [ ] **DF-ACTION-05**: Batch operations require confirmation dialog before execution
- [ ] **DF-ACTION-06**: Move operation auto-renames on filename conflict (e.g., `photo_1.jpg`)
- [ ] **DF-ACTION-07**: User has N seconds to undo a batch operation via undo button

### Duplicate Detection — Matching

- [x] **HASH-01**: aHash computed by resizing to 8x8, comparing each pixel to mean brightness
- [x] **HASH-02**: dHash computed by resizing to 9x8, comparing adjacent pixel pairs
- [x] **HASH-03**: pHash computed by resizing to 32x32, DCT, comparing low-frequency components
- [x] **HASH-04**: Match threshold: Hamming distance ≤ 5 out of 64 bits per algorithm
- [x] **HASH-05**: Confirmed duplicate requires all three hash types to match within threshold

## v2 Requirements

### Duplicate Finder — Enhancements

- [ ] **DF-V2-01**: User can exclude specific file patterns (glob, regex)
- [ ] **DF-V2-02**: User can export results to CSV or JSON
- [ ] **DF-V2-03**: User can view image side-by-side for comparison
- [ ] **DF-V2-04**: User can schedule recurring scans
- [ ] **DF-V2-05**: Per-group custom target folder for moves

### Future Tools

- [ ] **TOOL-01**: Additional utility tools can be added as sidebar items

## Out of Scope

| Feature | Reason |
|---------|--------|
| Second utility tool | TBD — sidebar placeholder only, no implementation in v1 |
| Drag-and-drop folder selection | Keyboard + native picker sufficient for v1 |
| Cloud storage integration | Not needed for personal desktop utility; adds complexity |
| Real-time collaboration | Single-user desktop app; no sync needed |
| Mobile app | Desktop-first; mobile later if demand exists |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| SHELL-01 | Phase 1 | Pending |
| SHELL-02 | Phase 1 | Pending |
| SHELL-03 | Phase 1 | Pending |
| SHELL-04 | Phase 1 | Pending |
| SHELL-05 | Phase 1 | Pending |
| DF-INPUT-01 | Phase 2 | Pending |
| DF-INPUT-02 | Phase 2 | Pending |
| DF-INPUT-03 | Phase 2 | Pending |
| DF-INPUT-04 | Phase 2 | Pending |
| DF-INPUT-05 | Phase 2 | Pending |
| DF-INPUT-06 | Phase 2 | Pending |
| DF-INPUT-07 | Phase 2 | Pending |
| DF-SCAN-01 | Phase 3 | Pending |
| DF-SCAN-02 | Phase 3 | Pending |
| DF-SCAN-03 | Phase 3 | Pending |
| DF-SCAN-04 | Phase 3 | Pending |
| DF-SCAN-05 | Phase 3 | Pending |
| DF-SCAN-06 | Phase 3 | Pending |
| DF-SCAN-07 | Phase 3 | Pending |
| DF-SCAN-08 | Phase 3 | Pending |
| DF-SCAN-09 | Phase 3 | Pending |
| DF-RESULT-01 | Phase 4 | Pending |
| DF-RESULT-02 | Phase 4 | Pending |
| DF-RESULT-03 | Phase 4 | Pending |
| DF-RESULT-04 | Phase 4 | Pending |
| DF-RESULT-05 | Phase 4 | Pending |
| DF-RESULT-06 | Phase 4 | Pending |
| DF-ACTION-01 | Phase 5 | Pending |
| DF-ACTION-02 | Phase 5 | Pending |
| DF-ACTION-03 | Phase 5 | Pending |
| DF-ACTION-04 | Phase 5 | Pending |
| DF-ACTION-05 | Phase 5 | Pending |
| DF-ACTION-06 | Phase 5 | Pending |
| DF-ACTION-07 | Phase 5 | Pending |
| HASH-01 | Phase 3 | Complete |
| HASH-02 | Phase 3 | Complete |
| HASH-03 | Phase 3 | Complete |
| HASH-04 | Phase 3 | Complete |
| HASH-05 | Phase 3 | Complete |

**Coverage:**
- v1 requirements: 36 total
- Mapped to phases: 36
- Unmapped: 0 ✓

---
*Requirements defined: 2026-10-25*
*Last updated: 2026-10-25 after initial definition*
