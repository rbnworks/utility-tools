# Phase 02: Duplicate Finder — Folder Input — Research

**Researched:** 2026-10-25
**Domain:** Tauri v2 dialog plugin, Tauri event system, React form state management
**Confidence:** HIGH

## Summary

Phase 2 builds the folder selection input UI for the Duplicate Finder tool. The core technical tasks are: (1) wiring the Tauri `plugin-dialog` to open a native OS folder picker, (2) calling a Tauri command to start a scan session with collected folder paths and options, and (3) managing React component state for a dynamic folder list with add/remove/validation, scan option toggles, and UI view transitions (input → progress → results).

The Tauri v2 `plugin-dialog` (crates.io: `tauri-plugin-dialog` 2.8.3, npm: `@tauri-apps/plugin-dialog` 2.8.3, depends on `@tauri-apps/api` ^2.12.0) provides `pick_folders()` for multi-folder selection on desktop. The dialog uses the `rfd` crate internally on desktop platforms.

Backend command communication uses the Tauri `invoke` API from the frontend (`@tauri-apps/api`) and `#[tauri::command]` on the Rust side. Events for real-time progress use `emit`/`listen`.

The project currently has Phase 1 scaffolded (src-tauri exists, compiled) but the Tauri npm packages (`@tauri-apps/api`, `@tauri-apps/plugin-dialog`) are NOT yet installed in `package.json`. The Rust Cargo.toml also lacks `tauri-plugin-dialog` and `tauri-plugin-fs` dependencies. These must be added.

**Primary recommendation:** Install the Tauri npm packages first, extend the Rust Cargo.toml with dialog + fs plugins, define the `scan_folders` command with the `ScanOptions` struct, then build the frontend UI components (FolderPicker, ScanOptions, ScanProgress) with React hooks and shadcn/ui primitives.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Native OS folder picker | Browser (Tauri dialog plugin) | — | Dialog runs in native Tauri context via IPC |
| Folder path validation (exists on disk) | Browser | Backend (confirm via fs plugin) | Frontend `fs.stat` works in Tauri; backend confirms as a belt-and-suspenders check |
| Scan options UI (checkboxes, numeric input) | Browser | — | Purely frontend state |
| Scan trigger → Tauri command | Browser → Backend | — | invoke() call bridges frontend to Rust |
| Scan progress events | Backend → Browser | — | Rust emits events; React listens |
| Input form reset | Browser | — | Frontend state management only |

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `@tauri-apps/api` | 2.12.4 | Frontend bridge to Tauri backend | Official Tauri JS API, all IPC via invoke/listen/emit [VERIFIED: npm registry + crates.io] |
| `@tauri-apps/plugin-dialog` | 2.8.3 | Native folder/file picker dialogs | Depends on rfd for desktop, official Tauri plugin [VERIFIED: npm registry + crates.io] |
| `tauri-plugin-dialog` (Rust) | 2.8.3 | Rust-side folder picker API | rfd-based, builder pattern with `pick_folders` [VERIFIED: docs.rs/tauri-plugin-dialog 2.8.3] |
| `tauri-plugin-fs` (Rust) | 2.7.1+ | Rust file system operations | Used by dialog plugin for file reading on mobile [VERIFIED: docs.rs/tauri-plugin-dialog deps] |
| `shadcn/ui` components | latest | UI primitives (button, input, checkbox, progress, label, card, tooltip) | shadcn v4 (new base-ui-based system), installed via CLI [CITED: ui.shadcn.com docs] |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `sonner` | 2.0.8 | Toast notifications (already in package.json) | Validation errors, confirmation toasts |
| `lucide-react` | 1.61.0 | Icons (already in package.json) | Folder icon, remove button, scan icon |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Tauri dialog `pick_folders` | Separate `pick_folder` calls per folder | UX is worse (n dialogs); pick_folders is built-in multi-select |
| Frontend path validation only | Rust backend validation only | Frontend needs instant feedback; backend has the authoritative answer |
| shadcn `Progress` component | Kokonut `liquid-gauge` | Liquid gauge is an animated progress indicator but Progress is more functional for bar + counter layout |

**Installation:**

```bash
# Frontend (Tauri JS packages — Phase 2 prerequisite)
npm install @tauri-apps/api @tauri-apps/plugin-dialog

# Backend (Rust dependencies — add to src-tauri/Cargo.toml)
cargo add tauri-plugin-dialog
cargo add tauri-plugin-fs
```

**Version verification:**
```bash
npm view @tauri-apps/api version          # → 2.12.4
npm view @tauri-apps/plugin-dialog version # → 2.8.3
```

## Package Legitimacy Audit

| Package | Registry | Age | Downloads | Source Repo | Verdict | Disposition |
|---------|----------|-----|-----------|-------------|---------|-------------|
| `@tauri-apps/api` | npm | 5+ yrs | 1.05M/wk | github.com/tauri-apps/tauri | OK | Approved |
| `@tauri-apps/plugin-dialog` | npm | 2+ yrs | part of tauri org | github.com/tauri-apps/plugins-workspace | OK | Approved |
| `tauri-plugin-dialog` (Rust) | crates.io | 3+ yrs | Tauri core plugin | github.com/tauri-apps/plugins-workspace | OK | Approved |

**No SLOP or SUS packages detected.** All from the official Tauri organization.

*Note: The package legitimacy gate command was not available to run (`gsd_run query package-legitimacy check` requires the seam). These verdicts are based on the source-of-truth evidence: npm registry shows official Tauri org maintainers, crates.io shows Tauri-bot as owner, and source repos are the canonical Tauri repos.*

## Architecture Patterns

### System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                      Browser (React)                            │
│                                                                 │
│  ┌──────────────────┐    ┌──────────────────┐                  │
│  │ DuplicateFinder   │    │ FolderPicker      │                  │
│  │  (state machine)  │◄───│  (folder list)    │                  │
│  └────────┬─────────┘    └────────┬──────────┘                  │
│           │                       │                              │
│  ┌────────▼─────────┐    ┌───────▼──────────┐                  │
│  │ ScanOptions       │    │ scanFoldersCmd() │  invoke() call  │
│  │  (options state)  │    └────────┬─────────┘                  │
│  └──────────────────┘             │                             │
│           │                       │                             │
│  ┌────────▼─────────┐             │                             │
│  │ ScanProgress      │◄────────────┘  listen('scan:progress')  │
│  │  (view state)     │                                            │
│  └──────────────────┘                                            │
└──────────────────┬──────────────────────────────────────────────┘
                   │  IPC (Tauri webview)
┌──────────────────▼──────────────────────────────────────────────┐
│                      Rust Backend                                │
│                                                                 │
│  ┌──────────────────┐    ┌──────────────────┐                  │
│  │ scan_folders cmd  │    │ emit events      │                  │
│  │ (app dialog + fs) │───►│ (scan:progress,   │                  │
│  └──────────────────┘    │  scan:complete)    │                  │
│                          └──────────────────┘                  │
└─────────────────────────────────────────────────────────────────┘
```

### Pattern: Tauri Command → Event Flow

The standard flow for Phase 2 is:
1. Frontend collects state (folder paths + scan options)
2. Frontend calls `invoke('scan_folders', { paths, options })`
3. Rust command runs dialog picker (if triggered), builds scan session
4. Rust starts scan on a background thread (or async task)
5. Rust emits progress events via `app.emit("scan:progress", ...)`
6. Frontend listens via `listen("scan:progress", callback)`
7. When done, Rust emits `scan:complete`
8. Frontend transitions to results view

### Recommended Project Structure

```
src/
├── components/
│   ├── ui/                       # shadcn/ui (installed by planner)
│   │   ├── button.tsx
│   │   ├── input.tsx
│   │   ├── checkbox.tsx
│   │   ├── progress.tsx
│   │   ├── label.tsx
│   │   ├── card.tsx
│   │   ├── tooltip.tsx
│   │   ├── badge.tsx
│   │   └── separator.tsx
│   ├── kokonutui/                # Kokonut UI (already has particle-button.tsx)
│   │   ├── particle-button.tsx
│   │   └── ... (liquid-gauge for scan progress)
│   ├── FolderPicker.tsx          # NEW — folder path list with add/remove
│   ├── ScanOptions.tsx           # NEW — checkboxes + min file size
│   ├── ScanProgress.tsx          # NEW — progress bar + stats
│   ├── DuplicateFinder.tsx       # UPDATED — orchestrates all sub-components
│   └── Sidebar.tsx               # existing
└── lib/
    └── utils.ts                  # cn() helper (moved in Phase 1)
```

### Pattern: Tauri Command Definition (Rust side)

Use `serde` for request/response serialization. Define a `ScanOptions` struct and a `SessionId` return type.

```rust
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle};
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanOptions {
    pub include_subdirectories: bool,
    pub include_video_files: bool,
    pub min_file_size_bytes: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionInfo {
    pub session_id: String,
    pub folder_count: usize,
}

#[command]
async fn scan_folders(
    app: AppHandle,
    folder_paths: Vec<String>,
    options: ScanOptions,
) -> Result<SessionInfo, String> {
    // Validate paths exist
    for path in &folder_paths {
        let metadata = std::fs::metadata(path)
            .map_err(|e| format!("Path does not exist: {path} ({e})"))?;
        if !metadata.is_dir() {
            return Err(format!("Not a directory: {path}"));
        }
    }
    // Create scan session...
    Ok(SessionInfo {
        session_id: uuid::Uuid::new_v4().to_string(),
        folder_count: folder_paths.len(),
    })
}
```

### Pattern: Tauri Command Registration

In `src-tauri/src/lib.rs`, register the command with the Tauri builder:

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![scan_folders])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
```

## Common Pitfalls

### Pitfall 1: Dialog plugin not registered in Cargo.toml
**What goes wrong:** The `app.dialog()` method is unavailable at compile time.
**Why it happens:** The dialog plugin must be registered in both Cargo.toml (`tauri-plugin-dialog = "2"`) AND in `lib.rs` (`.plugin(tauri_plugin_dialog::init())`).
**How to avoid:** Verify the plugin is listed in both places before writing command code.
**Warning signs:** `no method named dialog found for struct AppHandle` — the plugin is not registered.

### Pitfall 2: Missing `tauri-plugin-fs` dependency
**What goes wrong:** `tauri-plugin-dialog` 2.8.3 depends on `tauri-plugin-fs` (visible in docs.rs dependencies). If not added, Cargo may fail to resolve.
**Why it happens:** Transitive dependencies are not auto-added to Cargo.toml by `cargo add`.
**How to avoid:** Explicitly add `tauri-plugin-fs` to Cargo.toml alongside `tauri-plugin-dialog`.

### Pitfall 3: Tauri JS packages missing from package.json
**What goes wrong:** `invoke`, `listen`, `emit` imports fail at build time.
**Why it happens:** `@tauri-apps/api` and `@tauri-apps/plugin-dialog` are not in the project's dependencies (they're not auto-installed by `npx tauri init`).
**How to avoid:** Add them explicitly: `npm install @tauri-apps/api @tauri-apps/plugin-dialog`.

### Pitfall 4: Capability for dialog not in default.json
**What goes wrong:** Tauri's permission system blocks the dialog API at runtime on a signed bundle.
**Why it happens:** `capabilities/default.json` only has `"core:default"` — the dialog plugin needs its permission.
**How to avoid:** Add `"dialog:default"` to the permissions array in `capabilities/default.json`. (This is handled by the Tauri plugin system when the dialog command is invoked.)

### Pitfall 5: Path validation race condition
**What goes wrong:** User types a path, frontend validates it exists, then the file is deleted before the scan starts.
**Why it happens:** File system state can change between validation and actual use.
**How to avoid:** Validate at the Rust backend during the scan command. The frontend validation is for UX feedback only — the backend must do the authoritative check.

### Pitfall 6: Path separator cross-platform issues
**What goes wrong:** On Windows, Tauri's `pick_folders` may return Windows paths that behave differently.
**Why it happens:** File path representations differ across platforms.
**How to avoid:** Use `std::path::Path` in Rust and `string` paths in JS. Tauri normalizes paths internally.

## Code Examples

### Tauri: Rust-side folder picker via command

```rust
use tauri::command;
use tauri_plugin_dialog::DialogExt;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScanOptions {
    pub include_subdirectories: bool,
    pub include_video_files: bool,
    pub min_file_size_bytes: u64,
}

#[command]
async fn select_folders(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let paths = app
        .dialog()
        .file()
        .pick_folders(|paths: Option<Vec<tauri_plugin_dialog::FilePath>>| {
            // This callback is called when dialog closes
            // For async command, we need blocking_pick_folders instead
        });

    // Use blocking version inside async command (safe because it's a UI dialog)
    let paths = app
        .dialog()
        .file()
        .blocking_pick_folders();

    Ok(paths
        .map(|paths| {
            paths
                .into_iter()
                .filter_map(|fp| fp.to_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default())
}
```

### Tauri: Frontend invoke call to start scan

```typescript
import { invoke } from "@tauri-apps/api/core";

interface ScanOptions {
  includeSubdirectories: boolean;
  includeVideoFiles: boolean;
  minFileSizeBytes: number;
}

async function startScan(
  folderPaths: string[],
  options: ScanOptions
): Promise<{ sessionId: string; folderCount: number }> {
  return await invoke("scan_folders", {
    folderPaths,
    options,
  });
}
```

### Tauri: Frontend event listening for progress

```typescript
import { listen } from "@tauri-apps/api/event";

// Listen for scan progress events from Rust backend
const unlisten = await listen<{ scanned: number; total: number; file: string }>(
  "scan:progress",
  (event) => {
    setProgress(event.payload.scanned);
    setTotal(event.payload.total);
    setCurrentFile(event.payload.file);
  }
);

// Cleanup on unmount
// unlisten();
```

### Frontend: Folder list with add/remove (React)

```typescript
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { X } from "lucide-react";

interface FolderEntry {
  id: string;
  path: string;
  valid: boolean;
}

export function FolderPicker() {
  const [folders, setFolders] = useState<FolderEntry[]>([]);

  const addFolder = (path: string) => {
    setFolders((prev) => [...prev, { id: crypto.randomUUID(), path, valid: true }]);
  };

  const removeFolder = (id: string) => {
    setFolders((prev) => prev.filter((f) => f.id !== id));
  };

  const clearFolders = () => {
    setFolders([]);
  };

  return (
    <div className="space-y-3">
      <h3 className="text-sm font-medium text-zinc-300">Folders to scan</h3>
      <div className="space-y-2">
        {folders.map((f) => (
          <div key={f.id} className="flex items-center gap-2">
            <span className="flex-1 text-sm text-zinc-300 truncate">{f.path}</span>
            <Button variant="ghost" size="sm" onClick={() => removeFolder(f.id)}>
              <X className="h-4 w-4" />
            </Button>
          </div>
        ))}
      </div>
    </div>
  );
}
```

### Frontend: Scan options component

```typescript
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { useState } from "react";

export function ScanOptions() {
  const [includeSubdirectories, setIncludeSubdirectories] = useState(true);
  const [includeVideoFiles, setIncludeVideoFiles] = useState(true);
  const [minFileSizeKB, setMinFileSizeKB] = useState(10);

  const options = {
    includeSubdirectories,
    includeVideoFiles,
    minFileSizeBytes: Math.max(0, minFileSizeKB) * 1024,
  };

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-2">
        <Checkbox
          id="subdirs"
          checked={includeSubdirectories}
          onCheckedChange={(c) => setIncludeSubdirectories(!!c)}
        />
        <Label htmlFor="subdirs" className="text-sm text-zinc-300">
          Scan subdirectories
        </Label>
      </div>
      <div className="flex items-center gap-2">
        <Checkbox
          id="videos"
          checked={includeVideoFiles}
          onCheckedChange={(c) => setIncludeVideoFiles(!!c)}
        />
        <Label htmlFor="videos" className="text-sm text-zinc-300">
          Include video files
        </Label>
      </div>
      <div className="flex items-center gap-2">
        <Label htmlFor="min-size" className="text-sm text-zinc-300">
          Min file size:
        </Label>
        <Input
          id="min-size"
          type="number"
          min={0}
          value={minFileSizeKB}
          onChange={(e) => setMinFileSizeKB(Number(e.target.value))}
          className="w-24"
        />
        <span className="text-xs text-zinc-500">KB</span>
      </div>
    </div>
  );
}
```

### Frontend: DuplicateFinder orchestrator with view state

```typescript
import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { FolderPicker } from "@/components/FolderPicker";
import { ScanOptions } from "@/components/ScanOptions";
import { ScanProgress } from "@/components/ScanProgress";
import { Button } from "@/components/ui/button";
import { FolderOpen } from "lucide-react";

type ViewMode = "input" | "progress";

export function DuplicateFinder() {
  const [viewMode, setViewMode] = useState<ViewMode>("input");

  const handleStartScan = async () => {
    // Collect folder paths + options from state...
    setViewMode("progress");
    // Listen for events, call invoke("scan_folders", ...)
  };

  return (
    <div className="flex-1 p-6 space-y-6 overflow-y-auto bg-zinc-950">
      {viewMode === "input" && (
        <>
          <FolderPicker onAdd={...} />
          <ScanOptions />
          <Button onClick={handleStartScan} className="w-full">
            Scan for Duplicates
          </Button>
        </>
      )}
      {viewMode === "progress" && <ScanProgress onReset={() => setViewMode("input")} />}
    </div>
  );
}
```

## Project Constraints (from AGENTS.md)

The following constraints from `AGENTS.md` MUST be honored by the planner:

1. **Tauri v2 over Electron** — All native operations use Tauri, never Electron. Bundle target: ~5MB.
2. **Rust backend for file operations** — File scanning, hash computation, and file operations are in Rust, not JavaScript.
3. **Tauri events for real-time progress** — No polling. Use `emit`/`listen` for progress communication.
4. **Dark theme default** — CSS variables in `src/index.css` define dark theme. All components must work in dark.
5. **shadcn/ui + Kokonut UI** — Use shadcn/ui for base components. Check Kokonut UI for animated alternatives before building from scratch.
6. **No drag-and-drop** — Keyboard + native picker only (no drag-and-drop folder selection).
7. **Tauri init must run in project directory** — `npx tauri@latest init` was already done (Phase 1).
8. **CORS does not apply in Tauri** — Tauri commands run in native Rust, no CORS concerns.
9. **File operations in Rust are permanent** — No trash can integration by default.

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| Node.js | Frontend build | ✓ | 26.12.1 | — |
| npm | Package installation | ✓ | (in Node 26) | — |
| Rust | Tauri compilation | ✓ (from Phase 1) | (checked via build) | — |
| Tauri CLI | Dev/build | ✓ (from Phase 1) | (from src-tauri/) | — |
| webkit2gtk | Tauri on macOS | ? | — | Must verify with `brew list webkit2gtk` |

**Missing dependencies to verify:**
- `webkit2gtk` — required for Tauri on macOS. Check with `brew list webkit2gtk`.

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Tauri v1 with `tauri::api::dialog` | Tauri v2 with `tauri-plugin-dialog` crate | Tauri v2.0 release | New plugin architecture, separate Cargo dependency |
| Native File System API in browser | Tauri dialog via IPC | Tauri design | Browser FS API limited; Tauri has full OS access |
| shadcn v1 (radix-based) | shadcn v4 (base-ui-based) | shadcn 4.0 release | New component structure, base-ui underpinnings |

**Deprecated/outdated:**
- `tauri::api::dialog` — Tauri v1 API, replaced by `tauri-plugin-dialog` in v2
- shadcn v1 components — project should use v4 format (Base UI)

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | `@tauri-apps/api` 2.12.x API surface matches the invoke/listen/emit pattern used in code examples | Code Examples | If API changed, the invoke/listen syntax may differ. Verify with `npm view @tauri-apps/api readme` for the specific version. |
| A2 | `pick_folders()` callback receives `Option<Vec<FilePath>>` where `FilePath` has `.to_str()` for conversion to `String` | Rust-side folder picker example | If the API surface differs, the conversion code must be adjusted. Verified on docs.rs. |
| A3 | `shadcn/ui` v4 components install with `npx shadcn@latest add <name>` and place files in `components/ui/` | Standard Stack | If the shadcn CLI version differs, the install path may vary. The project has `shadcn: "^4.22.1"` in devDependencies. |
| A4 | Tauri `withGlobalTauri: true` in `tauri.conf.json` enables the global `__TAURI__` object for backward compatibility | Tauri configuration | Already confirmed in existing `tauri.conf.json:29`. No action needed. |
| A5 | `sonner` (already installed) is the toast library used in the project | Supporting libs | The project has `sonner: "^2.0.8"` in package.json. Used for error/confirmation toasts. |

## Open Questions

1. **Does `pick_folders` return `Option<Vec<FilePath>>` or `Result<Option<Vec<FilePath>>, Error>`?** The docs.rs source shows the async `pick_folders` takes a callback with `Option<Vec<FilePath>>`, while the blocking version returns `Option<Vec<FilePath>>` directly. Need to confirm which variant to use in an `async fn` command.

2. **What is the exact `ScanOptions` JSON structure the planner should pass from the frontend to the backend?** The Rust struct fields must match the JS object keys exactly for serde deserialization to work.

3. **Does the Tauri dialog plugin need explicit permissions in `capabilities/default.json`?** The existing capability only has `"core:default"`. The dialog plugin may auto-register its permissions or may require explicit addition.

4. **Should the `scan_folders` command return a `SessionId` for future commands (cancel, get results)?** The PLAN.md shows a `scan_folders` command that returns `SessionId`. This is needed because the actual scanning (Phase 3) runs asynchronously.

## Validation Architecture

**Test Framework:** Not yet configured. Wave 0 gap: no test framework installed.
- Quick run command: `{npm test or cargo test}` — to be determined in Phase 3 when tests are added
- Full suite command: `{npm test && cd src-tauri && cargo test}` — to be determined

**Phase Requirements → Test Map:**
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| DF-INPUT-01 | Native OS folder picker opens via Tauri | manual/e2e | `npm run tauri dev` → open dialog | ❌ Wave 0 |
| DF-INPUT-02 | Manual path entry works | unit | `vitest tests/FolderPicker.test.ts` | ❌ Wave 0 |
| DF-INPUT-03 | Remove folder works | unit | `vitest tests/FolderPicker.test.ts` | ❌ Wave 0 |
| DF-INPUT-04 | Subdirectory toggle renders | unit | `vitest tests/ScanOptions.test.ts` | ❌ Wave 0 |
| DF-INPUT-05 | Video toggle renders | unit | `vitest tests/ScanOptions.test.ts` | ❌ Wave 0 |
| DF-INPUT-06 | Min file size input renders | unit | `vitest tests/ScanOptions.test.ts` | ❌ Wave 0 |
| DF-INPUT-07 | Scan triggers invoke call | unit/integration | `vitest tests/DuplicateFinder.test.ts` | ❌ Wave 0 |

**Wave 0 Gaps:**
- [ ] Test framework — vitest or jest needs to be installed
- [ ] `tests/` directory with shared fixtures
- [ ] `@tauri-apps/api` mocking for test environment (it fails outside Tauri context)

## Security Domain

> `security_enforcement` is enabled (absent in config = enabled).

### Applicable ASVS Categories

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V5 Input Validation | yes | Frontend path validation + backend fs::metadata check |
| V6 Cryptography | no | Not applicable — no crypto operations in Phase 2 |
| V9 Security Misconfiguration | yes | Tauri capabilities, CSP, plugin permissions |

### Known Threat Patterns for Tauri + React

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Arbitrary file access via folder picker | Tampering | Validate paths are directories (`fs::metadata`), filter by allowed prefixes |
| Path traversal | Elevation | Validate path is within expected scopes (user home, project dirs) |
| Unvalidated Tauri command arguments | Tampering | Serialize/deserialize via serde with explicit struct, validate in command |
| CSP bypass via `withGlobalTauri: true` | Information Disclosure | Already set in `tauri.conf.json` — review if CSP restricts to `default-src 'self'` |

## Sources

### Primary (HIGH confidence)
- docs.rs/tauri-plugin-dialog 2.8.3 - `Dialog`, `FileDialogBuilder`, `PickerMode`, `FileAccessMode` APIs
- docs.rs/tauri 2.12.0 - `AppHandle`, `Emitter`, `Listener` traits, event system
- npm registry `@tauri-apps/api` 2.12.4 - official Tauri JS API package
- npm registry `@tauri-apps/plugin-dialog` 2.8.3 - dialog plugin for frontend
- ui.shadcn.com - shadcn/ui v4 component documentation (Progress, Checkbox, Button, Input, Label, Card, Tooltip, Badge)

### Secondary (MEDIUM confidence)
- GitHub: github.com/tauri-apps/tauri (official repo) — Tauri v2 architecture and API patterns
- GitHub: github.com/tauri-apps/plugins-workspace — dialog plugin source
- PLAN.md — Technical specification with command signatures and UI wireframes

### Tertiary (LOW confidence)
- Project package.json, Cargo.toml, tauri.conf.json, capabilities/default.json — read from disk
- AGENTS.md — project constraints (read from disk)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - verified via npm registry, docs.rs, and shadcn documentation
- Architecture: HIGH - Tauri v2 invoke/listen/emit pattern is well-documented in official sources
- Pitfalls: MEDIUM - based on known Tauri v2 patterns; some edge cases may surface during implementation

**Research date:** 2026-10-25
**Valid until:** 30 days (stable Tauri v2 APIs)
