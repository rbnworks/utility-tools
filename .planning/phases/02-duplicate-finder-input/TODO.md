# TODO: Phase 2 — Duplicate Finder — Folder Input

## Wave 1: Setup (dependencies + Rust)
- [ ] Task 1: Install frontend dependencies — Tauri JS packages + shadcn/ui components (package.json, src/components/ui/progress.tsx, src/components/ui/card.tsx, src/components/ui/badge.tsx)
- [ ] Task 1b: Install Rust plugins and define scan command (src-tauri/Cargo.toml, src-tauri/src/lib.rs, src-tauri/capabilities/default.json, verify with `cargo check`)

## Wave 2: Implementation (tracer + expansion)
- [ ] Task 2 (Tracer): Build FolderPicker — folder list with native picker, manual input, and remove (src/components/FolderPicker.tsx)
  - Verify: `npx tsc --noEmit`
- [ ] Task 3: Build ScanOptions — subdirectory toggle, video toggle, min file size (src/components/ScanOptions.tsx)
  - Verify: `npx tsc --noEmit`

## Wave 3: Integration (orchestrator + progress)
- [ ] Task 4: Wire DuplicateFinder — orchestrator with scan trigger, view transitions, and progress events (src/components/DuplicateFinder.tsx)
  - Verify: `npx tsc --noEmit`
- [ ] Task 5: Build ScanProgress — progress bar with stats, completion state, and reset (src/components/ScanProgress.tsx)
  - Verify: `npx tsc --noEmit`

## Final Verification
- [ ] `npx tsc --noEmit` — zero TypeScript errors
- [ ] `cd src-tauri && cargo check` — zero Rust errors
- [ ] Manual test: open Tauri dev (`npm run tauri dev`), navigate to Duplicate Finder, pick folders via native dialog
- [ ] Manual test: manually type a folder path and add it
- [ ] Manual test: remove individual folders, clear all folders
- [ ] Manual test: toggle subdirectories, video files, and min file size
- [ ] Manual test: click "Scan for Duplicates" — verify view transitions to progress
- [ ] Manual test: click "New Scan" — verify form resets completely
- [ ] All 7 requirements covered (DF-INPUT-01 through DF-INPUT-07)
