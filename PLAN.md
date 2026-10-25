# PLAN.md — Utility Tools Desktop App

> Created: 2026-10-25 | Status: Plan | Phase: discuss

## Goal

Build a **Tauri desktop app** called **Utility Tools** with a dark-themed UI featuring a left sidebar navigation and the first utility: **Duplicate Image/Video Finder** that detects visual duplicates using perceptual hashing (not file metadata).

## Architecture

```
utility-tools/
├── src/                          # React frontend (Vite)
│   ├── App.tsx                   # Shell with sidebar + main content
│   ├── index.css                 # Dark theme CSS variables
│   └── components/
│       ├── ui/                   # shadcn/ui components
│       ├── kokonutui/            # Kokonut UI components
│       ├── Sidebar.tsx           # Left navigation
│       ├── DuplicateFinder.tsx   # Main tool component
│       ├── FolderPicker.tsx      # Folder path input + picker
│       ├── ScanProgress.tsx      # Progress display
│       ├── DuplicateResults.tsx  # Results display
│       ├── DuplicateCard.tsx     # Single group card
│       └── FileThumbnail.tsx     # File preview
├── src-tauri/                    # Tauri/Rust backend (new)
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── icons/
│   └── src/
│       ├── main.rs               # Tauri entry point
│       ├── lib.rs                # Tauri commands + module exports
│       ├── scan.rs               # Recursive file scanning
│       ├── hash.rs               # Perceptual hashing
│       └── operations.rs         # File move/delete/keep
├── lib/                          # (move to src/lib/)
│   └── utils.ts                  # cn() helper
├── components.json
├── package.json
├── tsconfig.json
├── vite.config.ts                # Updated with Tauri plugin
└── tailwind.config.ts
```

**Stack:**
- **Frontend:** React 19 + Vite 8 + Tailwind CSS 4
- **Backend:** Rust via Tauri v2 (system-level file ops, image processing)
- **Real-time:** Tauri events (emit from Rust → listen in React)
- **Registry:** `@kokonutui` for animated components

## Key Design Decisions

1. **Tauri v2 (not Electron)** — Lightweight native binary (~5MB), Rust backend, bridges directly to OS file system.
2. **Hono** — Available in the project for any utility API helpers if needed later.
3. **Perceptual hashing, not metadata** — Two near-identical images with different timestamps/names will be detected as duplicates.
4. **Three hash algorithms** — aHash (average), dHash (gradient), pHash (DCT). A match requires all three to agree within tolerance, minimizing false positives/negatives.
5. **Tauri events for real-time progress** — No polling needed. Backend emits events; frontend updates reactively.

## Duplicate Detection: Perceptual Hashing

### Algorithm

For each image/video frame, compute three hashes:

| Algorithm | Method | Hash Size | What it captures |
|-----------|--------|-----------|-----------------|
| **aHash** (average hash) | Resize to 8x8, compute mean, compare each pixel to mean | 8x8 = 64 bits | Overall brightness similarity |
| **dHash** (difference hash) | Resize to 9x8, compare adjacent pixel pairs | 8x8 = 64 bits | Edge/gradient structure |
| **pHash** (DCT hash) | Resize to 32x32, DCT, compare low-frequency components | 8x8 = 64 bits | Perceptual structure (resilient to scaling) |

### Matching

- **Hamming distance** between two hashes: count differing bits.
- Match threshold: Hamming distance ≤ 5 (out of 64) for each algorithm.
- A **confirmed duplicate** requires all three hash types to match.
- This triple-agreement approach reduces false positives to near-zero while catching resized, recompressed, and converted duplicates.

### Video Handling

- Extract frames at regular intervals (e.g., every 5 seconds).
- Hash each frame. If N frames match across two videos (where N ≥ 3 and ≥ 50% of analyzed frames match), consider them duplicates.
- Fallback: hash the first frame only if video has fewer than 3 extractable frames.

### File Types Supported

| Images | Videos |
|--------|--------|
| `.jpg` `.jpeg` `.png` `.webp` `.gif` `.bmp` `.tiff` | `.mp4` `.mov` `.avi` `.mkv` `.webm` |

## Backend (Rust) Commands

### `lib.rs` — Tauri Commands

All public commands exposed to the frontend:

| Command | Args | Returns | Description |
|---------|------|---------|-------------|
| `scan_folders` | `Vec<String>` paths | `SessionId` | Start a scan session on given directories |
| `cancel_scan` | `SessionId` | `()` | Cancel an in-progress scan |
| `get_session_status` | `SessionId` | `SessionInfo` | Get current scan status (not scanning, scanning, complete) |
| `get_duplicate_groups` | `SessionId` | `Vec<DuplicateGroup>` | Retrieve all duplicate groups found |
| `keep_file` | `GroupId`, `file_index` | `Result<(), Error>` | Mark a file as the "keeper" in a group |
| `delete_file` | `FileId` | `Result<(), Error>` | Delete a file (skip keeper) |
| `move_to_folder` | `FileId`, `target_path` | `Result<(), Error>` | Move duplicate to target folder (auto-rename on conflict) |

### `scan.rs` — File Discovery

- Recursive directory traversal.
- Filter by supported image/video extensions.
- Skip hidden files and node_modules, .git, etc. (configurable).
- Emits Tauri events during scan:
  - `scan:progress` — `{ scanned: u64, total: u64, file: String }`
  - `scan:duplicate_found` — `{ group_id, file_count, files: [...] }` (sent as duplicates are identified)
  - `scan:complete` — `{ total_files, total_groups }`

### `hash.rs` — Perceptual Hashing

- **Images:** Open with `image` crate, resize, compute all three hash types.
- **Videos:** Extract frames via `image` crate (or `ffmpeg` binding if needed), hash each frame, compare frame sets.
- Cache hashes per file path (stored in-memory during scan session).
- Group duplicates using a hash-bucket approach: group by combined hash fingerprint.

### `operations.rs` — File Operations

- **Keep:** No action (file stays).
- **Delete:** Remove file with `std::fs::remove_file`.
- **Move:** Move to target folder with auto-increment suffix on name collision:
  - `photo.jpg` → `photo_1.jpg`, `photo_2.jpg`, etc.
- All operations report success/failure per file.
- **No trash** — files are permanently deleted/moved (user confirmation required before batch operations).

## Frontend Structure

### App Shell (`src/App.tsx`)

```
┌─────────────────────────────────────────────────┐
│ Header (app name, theme toggle if needed)       │
├────────┬────────────────────────────────────────┤
│ Sidebar│ Main Content Area                      │
│        │                                        │
│ 🛠️ Tools │  [Dynamic component based on selection]  │
│        │                                        │
│ ───────│ ─────────────────────────────────────  │
│        │                                        │
│ 🔍 Duplicate │  Scan UI / Progress / Results    │
│    Finder    │                                  │
│        │                                        │
│ ➕ [Next tool]│  (reserved for future)          │
│        │                                        │
└────────┴────────────────────────────────────────┘
```

### Sidebar

- Tool icons with labels.
- Active tool highlighted.
- Expandable section for future tools.
- Uses Kokonut UI for smooth transitions between sections.

### Duplicate Finder — Views

#### 1. Folder Selection

```
┌─────────────────────────────────────────────┐
│ Duplicate Image/Video Finder                 │
├─────────────────────────────────────────────┤
│                                              │
│  Add folders to scan:                        │
│  ┌──────────────────────────────────┬─────┐  │
│  │ /Users/user/Pictures/Photos     │ [+]+ │  │
│  └──────────────────────────────────┴─────┘  │
│  ┌──────────────────────────────────┬─────┐  │
│  │ /Users/user/Documents/Backup      │ [+]+ │  │
│  └──────────────────────────────────┴─────┘  │
│                                              │
│  ┌──────────────────────────────────────┐    │
│  │   [Scan for Duplicates]              │    │
│  └──────────────────────────────────────┘    │
│                                              │
│  Scan options:                               │
│  ☑ Scan subdirectories                       │
│  ☑ Include video files                       │
│  Min file size: 100 KB [____]                │
│                                              │
└─────────────────────────────────────────────┘
```

- **Folder picker:** Uses Tauri `dialog::open` multi-select for folder selection (native OS dialog).
- **Add/Remove:** Dynamically add or remove folder entries.
- **Options:** Toggle subdirectories, video inclusion, min file size filter.
- **Scan button:** Triggers `scan_folders` Tauri command.

#### 2. Scan Progress

```
┌─────────────────────────────────────────────┐
│ Duplicate Image/Video Finder                 │
├─────────────────────────────────────────────┤
│                                              │
│  Scanning 3 folders...                       │
│                                              │
│  ████████████████████░░░░░░  67%            │
│                                              │
│  Files scanned:    4,832 / 7,210            │
│  Duplicates found: 127 (34 groups)           │
│                                              │
│  Currently processing:                        │
│  /Pictures/Beach/vacation_234.jpg            │
│                                              │
│  ┌──────────────────────────────────┐        │
│  │        [Cancel Scan]             │        │
│  └──────────────────────────────────┘        │
│                                              │
└─────────────────────────────────────────────┘
```

- **Animated progress bar** using Kokonut UI `liquid-gauge` or shadcn `progress`.
- **Live counters** updating via Tauri events.
- **Current file** being processed (helps user understand what's happening).
- **Cancel button** — graceful scan cancellation.
- **Auto-expands** to show found duplicates as they appear.

#### 3. Duplicate Results

```
┌─────────────────────────────────────────────────────────────────┐
│ Duplicate Image/Video Finder                                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  34 duplicate groups | 89 affected files                         │
│  [Select all] [Keep all] [Delete all] [Move all]                 │
│                                                                  │
│  ──────────────────────────────────────────────────────────────  │
│                                                                  │
│  Group 1 (3 duplicates — 2.4 MB each)                            │
│  ┌──────────┬──────────┬──────────┐                            │
│  │ [IMG] ✅  │ [IMG]    │ [IMG]    │                            │
│  │ vacation  │ vacation │ vacation │                            │
│  │ _001.jpg │ _002.jpg │ _copy.jpg│                            │
│  │ /Photos/ │ /Photos/ │ /Backup/ │                            │
│  │ [Keep]   │ [Delete] │ [Delete] │                            │
│  └──────────┴──────────┴──────────┘                            │
│                                                                  │
│  Group 2 (2 duplicates — 1.8 MB each)                            │
│  ┌──────────┬──────────┐                                       │
│  │ [IMG] ✅  │ [IMG]    │                                       │
│  │ sunset    │ sunset   │                                       │
│  │ _v1.jpg   │ _v2.jpg  │                                       │
│  │ /Photos/  │ /Documents/│                                     │
│  │ [Keep]    │ [Delete] │                                       │
│  └──────────┴──────────┘                                       │
│                                                                  │
│  [Show all groups]  [Collapse all]                               │
│                                                                  │
│  ──────────────────────────────────────────────────────────────  │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Batch action: [Delete remaining ▼]  [Move to ▼]  [Done] │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Duplicate Card Design

Each duplicate group shows:
- **Thumbnail preview** of each file (small, e.g., 120x120px)
- **File name** and **file size**
- **File path** (truncated, hover to see full)
- **Selected action** per file (keep/delete/move)
- **Default:** first file in group is marked as "Keep" (green checkmark), rest are "Delete" (red X)

### Actions Per File

| Action | Description |
|--------|-------------|
| **Keep** (default on 1 per group) | File stays. No further action. |
| **Delete** | Permanently delete the file. |
| **Move to folder** | Move to a specified folder. Auto-rename on conflict. |
| **Rename (same folder)** | If no target folder specified, rename with `_dup_1`, `_dup_2` suffix. |

### Batch Actions

- **Per-group:** Select which file to keep, mark rest for delete/move.
- **Global:** After reviewing, apply batch actions:
  - "Delete all marked" — delete all files marked for deletion across all groups.
  - "Move all marked to..." — move all marked files to a single target folder.
  - "Keep all first" — keep the first file in each group, mark rest for deletion.
- **Confirmation dialog** before executing batch actions (irreversible).
- **Undo** option via Kokonut UI `undo-button` — user has N seconds to reverse.

### Results Summary

At the top of results view:
- Total duplicate groups
- Total affected files
- Total space recoverable
- Quick action buttons

## UI Components Required

### From shadcn/ui (install)

| Component | Usage |
|-----------|-------|
| `button` | Primary actions (Scan, Delete, Move, Keep) |
| `card` | Duplicate group containers |
| `dialog` / `alert-dialog` | Confirmation dialogs |
| `input` | Folder path entry, search |
| `progress` | Scan progress bar |
| `table` | Results table view (alternative to cards) |
| `scroll-area` | Scrollable results area |
| `checkbox` | Options toggles |
| `select` | Folder target dropdown |
| `badge` | Status indicators (group size, file type) |
| `separator` | Visual dividers |
| `tooltip` | Hover text for truncated paths |
| `tabs` | Tab between Card/List view |
| `dropdown-menu` | Per-file action menu |

### From Kokonut UI (install)

| Component | Usage |
|-----------|-------|
| `particle-button` | Animated scan/start buttons |
| `liquid-gauge` | Animated scan progress indicator |
| `loader` | Loading states |
| `status-island` | Background task notifications |
| `undo-button` | Undo recent batch operations |
| `ai-loading` | Could be repurposed for scan progress log |

### Custom Components (build)

| Component | Description |
|-----------|-------------|
| `Sidebar` | Left navigation with tool list |
| `DuplicateCard` | Single duplicate group display |
| `FileThumbnail` | Image/video preview with file info |
| `FolderPicker` | Folder path input with native picker |
| `DuplicateResults` | Full results list with batch actions |
| `ScanProgress` | Real-time scan progress display |

## File Path Alias

Note: `lib/utils.ts` is in the project root but `tsconfig.json` only includes `src/`. The `cn()` helper needs to live at `src/lib/utils.ts` for the path alias to resolve correctly. This will be moved/created during setup.

## Tauri Setup Steps

1. Install Rust (rustup):
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```
2. Initialize Tauri in the project:
   ```
   npx tauri@latest init
   ```
3. Configure `src-tauri/tauri.conf.json` for Vite integration.
4. Install Tauri frontend deps:
   ```
   npm install @tauri-apps/api @tauri-apps/plugin-dialog @tauri-apps/plugin-shell
   ```
5. Update `vite.config.ts` with Tauri plugin.

## Rust Dependencies (`src-tauri/Cargo.toml`)

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-dialog = "2"
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
image = "0.25"           # Image loading, resizing, pixel access
image-hash = "0.7"       # aHash, dHash, pHash implementations
tokio = { version = "1", features = ["full"] }
walkdir = "2"            # Recursive directory traversal
```

## Task Breakdown

### Phase 1: Foundation

1. **Install Rust + initialize Tauri**
   - Install rustup
   - Run `npx tauri@latest init`
   - Configure `tauri.conf.json` (app name, window settings, build config)
   - Set up `src-tauri/` structure

2. **Create `src/lib/utils.ts`** (move from root `lib/`)
   - Copy `cn()` helper to `src/lib/utils.ts`

3. **Install shadcn/ui components**
   - Install all required shadcn components listed above

4. **Install Kokonut UI components**
   - Install all required Kokonut components listed above

### Phase 2: App Shell

5. **Create Sidebar component**
   - Tool list with icons (lucide-react)
   - Active state styling
   - Expandable section for future tools section

6. **Create App layout**
   - Full-page flex layout: sidebar + main content
   - Dark theme styling
   - Responsive considerations

7. **Wire up navigation**
   - Click sidebar item → render corresponding content area
   - Placeholder content for future tools

### Phase 3: Duplicate Finder — Folder Selection

8. **Create FolderPicker component**
   - Text input for manual path entry
   - Native folder picker button (Tauri dialog)
   - Add/remove folder entries
   - Path validation (check if path exists)

9. **Create Scan Options**
   - Checkbox toggles (subdirs, video, min size)
   - Numeric input for min file size

10. **Wire up scan start**
    - Collect folder paths + options
    - Call Tauri `scan_folders` command
    - Transition UI to progress view

### Phase 4: Duplicate Finder — Scanning

11. **Implement Rust scan module** (`src-tauri/src/scan.rs`)
    - Recursive directory traversal
    - File filtering by extension + size
    - Skip hidden/system directories

12. **Implement Rust hash module** (`src-tauri/src/hash.rs`)
    - Image hashing (aHash + dHash + pHash)
    - Video frame extraction + hashing
    - Hamming distance comparison
    - Duplicate grouping logic

13. **Wire Tauri events**
    - Emit `scan:progress` events
    - Emit `scan:duplicate_found` as duplicates are identified
    - Emit `scan:complete` on finish

14. **Implement ScanProgress component**
    - Animated progress bar
    - Live counters (scanned, duplicates found)
    - Current file being processed
    - Cancel button

15. **Implement DuplicateResults component**
    - Group cards with thumbnails
    - File info display (name, size, path)
    - Per-file action selectors
    - Default: first file = keep, rest = delete

### Phase 5: Duplicate Finder — Actions

16. **Implement Rust operations module** (`src-tauri/src/operations.rs`)
    - Delete file
    - Move file (with auto-rename on conflict)
    - Keep file (no-op, just track selection)

17. **Wire file operations to frontend**
    - Per-file action buttons
    - Per-group action buttons (keep all, delete all)

18. **Implement batch actions**
    - Global "delete all marked"
    - Global "move all marked to..."
    - Confirmation dialog before execution

19. **Implement undo**
    - Track performed operations
    - Undo-button with time limit
    - Revert file changes on undo

### Phase 6: Polish

20. **Empty state / loading states**
    - Before scan: helpful empty state
    - During scan: progress indicator
    - No duplicates: empty state with "no duplicates found"

21. **Error handling**
    - Permission errors on inaccessible folders
    - Corrupted/unreadable files
    - Network/permission error messages

22. **Keyboard shortcuts**
    - Ctrl/Cmd + Enter to start scan
    - Escape to cancel

23. **Test with real files**
    - Verify duplicate detection accuracy
    - Test delete/move operations
    - Validate auto-rename on conflict

## File Structure (Final)

```
utility-tools/
├── src/
│   ├── main.tsx
│   ├── index.css
│   ├── App.tsx
│   ├── lib/
│   │   └── utils.ts           # cn() helper
│   └── components/
│       ├── ui/                # shadcn/ui components
│       ├── kokonutui/         # Kokonut UI components
│       ├── Sidebar.tsx        # Left navigation
│       ├── DuplicateFinder.tsx # Main tool component
│       ├── FolderPicker.tsx   # Folder path input + picker
│       ├── ScanProgress.tsx   # Progress display
│       ├── DuplicateResults.tsx # Results display
│       ├── DuplicateCard.tsx  # Single group card
│       └── FileThumbnail.tsx  # File preview
├── src-tauri/
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   ├── icons/
│   └── src/
│       ├── lib.rs             # Tauri commands + module exports
│       ├── main.rs            # Tauri entry point
│       ├── scan.rs            # File scanning
│       ├── hash.rs            # Perceptual hashing
│       └── operations.rs      # File move/delete/keep
├── components.json
├── package.json
├── tsconfig.json
├── vite.config.ts
├── tailwind.config.ts
└── AGENTS.md
```

## Assumptions & Risks

| # | Assumption / Risk | Mitigation |
|---|-------------------|-----------|
| 1 | User has (or will install) Rust | Document in README; setup script in AGENTS.md |
| 2 | Tauri v2 Rust dependencies (`image-hash`, etc.) compile on macOS (ARM) | Test early; have fallback hash implementation if crate missing |
| 3 | Video frame extraction works for all formats | Use `image` crate's built-in support; add ffmpeg binding only if needed |
| 4 | Perceptual hash triple-agreement gives good precision | Tune Hamming distance thresholds during testing |
| 5 | Tauri `dialog::open` multi-folder selection works on all platforms | Use separate dialogs per folder on platforms that don't support multi-select |
| 6 | Delete is permanent (no trash) — user might regret actions | Confirmation dialogs + undo button before session close |
| 7 | This is the only tool initially, but we want to add more | Sidebar supports expandable sections; placeholder content for future tools |

## What's Out of Scope (Future)

- Additional utility tools (the second tool is TBD)
- Drag-and-drop folder selection (keyboard only for now)
- Export results to CSV/JSON
- Scheduled/recurring scans
- Excluded file patterns (whitelist/blacklist beyond basic hidden file skip)
- Image comparison side-by-side viewer
- Move to custom per-group target folders
- Integration with cloud storage
