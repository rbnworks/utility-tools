# Utility Tools

## What This Is

A desktop application that ships a collection of developer-friendly utility tools, starting with a Duplicate Image/Video Finder. It scans local directories using perceptual hashing (not file metadata) to detect near-identical images and videos, then lets the user keep one and delete or move the rest.

## Core Value

Find visual duplicates across folders — even when files have different names, sizes, or timestamps — and give the user full control over which copy to keep.

## Business Context

<!-- OPTIONAL — only for monetized or customer-facing projects. Delete this section otherwise. -->

- **Customer**: The developer who owns this workspace — personal utility tool, not a commercial product
- **Revenue model**: None — internal-use tool
- **Success metric**: First tool ships as a polished, reliable desktop app that actually catches duplicates the user cares about
- **Strategy notes**: Future tools will be added as separate sidebar items; architecture should stay open-ended

## Requirements

### Validated

(None yet — ship to validate)

### Active

- [ ] Desktop app shell with dark-themed UI and left sidebar navigation
- [ ] Duplicate Image/Video Finder — first utility tool
- [ ] Perceptual hash-based duplicate detection (aHash + dHash + pHash, triple-agreement matching)
- [ ] Support for image formats: JPG, PNG, WebP, GIF, BMP, TIFF
- [ ] Support for video formats: MP4, MOV, AVI, MKV, WebM
- [ ] Folder picker (native OS dialog) and manual path input
- [ ] Real-time scan progress via Tauri events
- [ ] Results display with thumbnails, file info, per-file actions (keep/delete/move)
- [ ] Default: first file = keep, rest = delete
- [ ] Batch actions with confirmation dialog
- [ ] Undo button for recent batch operations
- [ ] Auto-rename on file move conflict

### Out of Scope

- Second utility tool (TBD — sidebar placeholder only for now)
- Drag-and-drop folder selection (keyboard + native picker only)
- Export results to CSV/JSON
- Scheduled or recurring scans
- Cloud storage integration
- Image comparison side-by-side viewer
- Per-group custom target folders for moves
- Excluded file pattern whitelist/blacklist (beyond basic hidden file skip)

## Context

- Existing project is a Vite + React 19 + Tailwind CSS 4 app with Kokonut UI registry
- `lib/utils.ts` at project root needs to move to `src/lib/utils.ts` for tsconfig path alias to resolve
- Rust is not installed — first step is `rustup` installation
- Tauri v2 chosen over Electron for lightweight native binary and direct file system access
- Hono is already in the project dependencies but will not be used for the backend (Rust is the backend)
- Dark theme as default with CSS variables already defined in `src/index.css`
- PLAN.md already exists with detailed technical specification

## Constraints

- **[Tech stack]**: Tauri v2 (Rust) + React 19 + Vite 8 + Tailwind CSS 4 — user has committed to this stack after deliberation
- **[Runtime]**: Desktop-only — browser-only approaches (Vite without Tauri) cannot access the file system for directory scanning
- **[Platform]**: macOS first (development machine), but Tauri should target cross-platform (macOS, Windows, Linux)
- **[UI framework]**: shadcn/ui base components + Kokonut UI registry — no custom UI library from scratch

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Tauri v2 over Electron | Lightweight native binary (~5MB), Rust backend, direct file system access | ✓ Good |
| Tauri v2 over Node.js backend with Electron | Smaller bundle, Rust's safety guarantees, better image processing crates | ✓ Good |
| Hono available but not used for backend | Rust provides the backend via Tauri commands; Hono kept in deps for potential future use | ✓ Good |
| Three hash algorithms (aHash + dHash + pHash) | Triple-agreement minimizes false positives while catching resized/recompressed duplicates | ✓ Good |
| Dark theme as default | User preference for polished desktop app aesthetics; CSS variables already configured | ✓ Good |
| Perceptual hashing over metadata comparison | Captures visual duplicates even with different names, timestamps, or file sizes | ✓ Good |
| Tauri events (not polling) for real-time progress | Built into Tauri architecture; no extra infrastructure needed | ✓ Good |

---
*Last updated: 2026-10-25 after initialization*

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd-complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state
