# Plan Check: Phase 2 — Duplicate Finder — Folder Input

## Verdict: PASS_WITH_FIXES

**4 blocker(s), 2 warning(s), 1 advisory** — four Rust compilation/data-serialization issues in Plan 01 will prevent the Tauri command from working. Plans 02 and 03 are well-structured; the blockers are all in the infrastructure plan.

---

## Requirements Traceability

| Requirement | Phase Goal | Covered By | Status |
|-------------|------------|------------|--------|
| DF-INPUT-01: Native OS folder picker | FolderPicker → `pickFolders()` | 02-02 Task 2, Step 4c | ✅ COVERED |
| DF-INPUT-02: Manual path entry | Input field + add button | 02-02 Task 2, Step 4d | ✅ COVERED |
| DF-INPUT-03: Remove added folders | X button → `onRemove(id)` | 02-02 Task 2, Step 4c | ✅ COVERED |
| DF-INPUT-04: Subdirectory toggle | Checkbox in ScanOptions | 02-02 Task 3, Step 4c item 1 | ✅ COVERED |
| DF-INPUT-05: Video file toggle | Checkbox in ScanOptions | 02-02 Task 3, Step 4c item 2 | ✅ COVERED |
| DF-INPUT-06: Min file size filter | Numeric input in ScanOptions | 02-02 Task 3, Step 4c item 3 | ✅ COVERED |
| DF-INPUT-07: Scan trigger | Button → `invoke("scan_folders")` | 02-03 Task 4, Step 3b | ✅ COVERED |

All 7 requirements are claimed in at least one plan's frontmatter and addressed by at least one task.

**Note:** Plan 01 claims all 7 requirements in its frontmatter despite having no user-facing tasks. The actual implementation is distributed across Plans 02 and 03. Plan 01's claim is justified because it creates the infrastructure (Tauri command, plugins, dependencies) that all requirements depend on.

---

## Findings

### BLOCK: Missing serde derives on ScanOptions — command parameter deserialization will fail

- **Location:** 02-01-PLAN.md Task 1b, Step 2c
- **Issue:** The plan defines:
  ```rust
  pub struct ScanOptions { include_subdirectories: bool, include_video_files: bool, min_file_size_bytes: u64 }
  ```
  The `scan_folders` Tauri command receives `options: ScanOptions` as a parameter. For Tauri to deserialize the incoming JSON payload (sent by `invoke()` from the frontend) into this struct, the struct must implement `serde::Deserialize`. The plan does not include `#[derive(serde::Deserialize)]` (or `#[derive(Deserialize)]` with an import) on the struct.

  The project's Cargo.toml already has `serde = { version = "1.0", features = ["derive"] }`, so the derive is available — it just needs to be applied.

- **Required property:** All Tauri command parameters that receive frontend data must derive the appropriate serde traits.
- **Fix hint (non-binding):** Add `#[derive(serde::Deserialize)]` to the ScanOptions struct definition in Task 1b Step 2c.

### BLOCK: Missing serde derive on SessionInfo — command return value serialization will fail

- **Location:** 02-01-PLAN.md Task 1b, Step 2c
- **Issue:** The plan defines:
  ```rust
  pub struct SessionInfo { session_id: String, folder_count: usize }
  ```
  The command returns `Result<SessionInfo, String>`. For Tauri to serialize the return value back to the frontend (so `invoke()` resolves with the SessionInfo data), the struct must implement `serde::Serialize`. The plan does not include `#[derive(serde::Serialize)]`.

- **Required property:** All Tauri command return types must derive the appropriate serde traits.
- **Fix hint (non-binding):** Add `#[derive(serde::Serialize)]` to the SessionInfo struct definition in Task 1b Step 2c.

### BLOCK: Missing Tauri v2 command import — compilation will fail

- **Location:** 02-01-PLAN.md Task 1b, Step 2c
- **Issue:** The plan shows:
  ```rust
  #[command] async fn scan_folders(...)
  ```
  The `#[command]` attribute requires the `use tauri::command;` import (or use the fully-qualified `#[tauri::command]` which does not require an import). The plan does not include this import in the action steps. Without it, `cargo check` will fail with an unknown attribute error.

- **Required property:** All Rust attributes used in the plan must have their imports specified.
- **Fix hint (non-binding):** Replace `#[command]` with `#[tauri::command]` (fully qualified, no import needed) in Task 1b Step 2c.

### BLOCK: serde snake_case / camelCase naming mismatch — scan options will silently deserialze to defaults

- **Location:** 02-01-PLAN.md Task 1b (Rust struct); 02-03-PLAN.md Task 4 (invoke call)
- **Issue:** The Rust struct uses snake_case field names (`include_subdirectories`, `include_video_files`, `min_file_size_bytes`) but the frontend invokes with camelCase keys (`includeSubdirectories`, `includeVideoFiles`, `minFileSizeBytes`). Without `#[serde(rename = "...")]` attributes, serde will not match the JSON keys to struct fields, and all fields will deserialize to default values: `false`, `false`, `0`.

  The command will compile (after fixes in BLOCKs 1-3) but run with incorrect scan options — subdirectories disabled, video files disabled, zero-byte minimum size. This is particularly insidious because the command won't error; it will silently produce wrong behavior.

- **Required property:** Rust struct fields must deserialize correctly from the JSON keys the frontend sends.
- **Fix hint (non-binding):** Add serde rename attributes to the ScanOptions struct:
  ```rust
  #[derive(serde::Deserialize)]
  pub struct ScanOptions {
      #[serde(rename = "includeSubdirectories")]
      pub include_subdirectories: bool,
      #[serde(rename = "includeVideoFiles")]
      pub include_video_files: bool,
      #[serde(rename = "minFileSizeBytes")]
      pub min_file_size_bytes: u64,
  }
  ```

### WARNING: Missing key_link for ScanOptions → scan_folders data flow

- **Location:** 02-02-PLAN.md must_haves.key_links
- **Issue:** Plan 02's key_links describe component imports but do not document the critical data flow from ScanOptions (options configuration) → DuplicateFinder (Wave 3) → scan_folders Tauri command (Plan 01). This data path is the core user interaction for the phase: configure options, start scan.

  Plan 03's key_links partially cover the invoke side but the ScanOptions → DuplicateFinder wiring is implicit through the props interface (Plan 02 Task 3 defines `ScanOptionsProps` with `onChange`), not explicitly linked.

- **Required property:** Dependent artifacts across waves should have their data flow explicitly linked.
- **Fix hint (non-binding):** Add to Plan 02 key_links: "ScanOptions options data flows through onChange callback to DuplicateFinder (Wave 3) which passes it to scan_folders Tauri command via invoke".

### WARNING: Path validation UX gap — no per-path feedback before scan

- **Location:** 02-02-PLAN.md Task 2, Step 4d (manual input handler); 02-01-PLAN.md Task 1b (backend validation only)
- **Issue:** The manual input handler validates only that the path string is non-empty. Paths are NOT validated for existence on disk at add time. Validation happens in the `scan_folders` backend command, which returns `Result<SessionInfo, String>`. If validation fails, the error is caught in `handleScan` and displayed generically.

  The user experience: type a path → added silently to list → click "Scan for Duplicates" → if invalid, generic error message → user doesn't know which path was invalid.

- **Required property:** User input should provide feedback at the point of interaction when feasible.
- **Fix hint (non-binding):** Consider adding a path-existence check on folder addition using a lightweight Tauri invoke (e.g., `check_path_exists`) that gives inline validation feedback. Or document this as a known limitation for Phase 2 (validated at scan time in Phase 3).

### ADVISORY: ScanProgress and progress listener active with no Phase 2 events

- **Location:** 02-03-PLAN.md Task 4 (progress listener) and Task 5 (ScanProgress component)
- **Issue:** Plan 03 creates the `scan:progress` event listener and the ScanProgress component, but Phase 2's `scan_folders` command only validates paths and returns a session ID — no events are emitted. Phase 3 emits `scan:progress` events.

  During Phase 2 execution, the progress view appears briefly (scan returns immediately), shows 0% progress, and transitions to completed. This is intentional (phase-by-phase development) but worth noting.

- **Required property:** N/A — informational only.
- **Fix hint (non-binding):** No fix needed. Consider adding to Plan 03 must_haves: "ScanProgress renders with scanned=0/total=0 in Phase 2; event-driven updates wired for Phase 3".

---

## Wave Dependency Check

| Plan | Wave | depends_on | Valid? | Notes |
|------|------|------------|--------|-------|
| 02-01 | 1 | [] | ✅ | No deps — Wave 1, correct |
| 02-02 | 2 | ["02-01"] | ✅ | Depends on Plan 01 (Tauri command scaffolding, plugins, dependencies) |
| 02-03 | 3 | ["02-02"] | ✅ | Depends on Plan 02 (component interfaces, exports) |

**Dependency graph:** 02-01 → 02-02 → 02-03 (linear, acyclic, correct wave ordering)

**Cross-plan artifact wiring:**
- Plan 01: defines `scan_folders` command, `ScanOptions` struct, `SessionInfo` struct → ✅ (compilation will pass after fixes in BLOCKs 1-4)
- Plan 02: creates `FolderPicker.tsx`, `ScanOptions.tsx` with interfaces matching Plan 01 types → ✅
- Plan 03: orchestrates both components, invokes `scan_folders` → ✅

---

## Threat Model Review

| Plan | Threat Model | Assessment |
|------|-------------|------------|
| 02-01 | `status="not_applicable"` | Acceptable — infrastructure setup, no user-facing input handling |
| 02-02 | 3 STRIDE entries (T-02-01 through T-02-03) | Partially complete — covers tampering on manual input, min file size validation, and dialog information disclosure. **Missing:** path traversal/symlink risks, path canonicalization. |
| 02-03 | 3 STRIDE entries (T-02-04 through T-02-06) | Partially complete — covers invoke tampering, event information disclosure, and capability scope. **Missing:** file operation security (deferred to Phase 5), symlink following. |

**Overall threat model assessment:** The plans identify relevant trust boundaries for the immediate surface area (user input, dialog, event system, capabilities). However, the threat model does not address:
1. **Path traversal attacks** — user-entered path like `../../../etc` could traverse outside expected directories
2. **Symlink following** — selected folders may contain symlinks to sensitive locations
3. **Path canonicalization** — paths should be resolved to absolute canonical form before validation

These are not blockers for Phase 2 (Phase 3 scanning is where they matter most), but the threat model should acknowledge them as deferred concerns.

---

## Success Criteria Coverage

| # | Success Criterion | Covered By | Assessment |
|---|-------------------|------------|------------|
| 1 | Native OS folder picker via Tauri | 02-02 Task 2 | ✅ `pickFolders()` via dynamic import from `@tauri-apps/plugin-dialog` |
| 2 | Manual type + add paths | 02-02 Task 2 | ✅ Input field + add button, validates non-empty strings |
| 3 | Remove + path validation | 02-02 Task 2 + 02-01 Task 1b | ✅ Remove button exists; validation via backend command. UX gap: no inline path validation (see WARNING #2). |
| 4 | Scan options UI (checkboxes + numeric) | 02-02 Task 3 | ✅ Subdirs checkbox, video checkbox, min file size numeric input with KB label |
| 5 | Scan trigger calls Tauri command | 02-03 Task 4 | ✅ Button → `invoke("scan_folders", ...)` with folders + options |
| 6 | Input → progress view transition | 02-03 Task 4 | ✅ viewMode state switches from "input" to "progress" |
| 7 | Form reset on new scan | 02-03 Task 4, Step 3d | ✅ `handleNewScan` resets all state (folders, options, progress, error, completed) |

---

## Tauri v2 Correctness Checklist

| Check | Status | Evidence |
|-------|--------|----------|
| Dialog plugin in Cargo.toml | ✅ | Plan 01 Task 1b: `tauri-plugin-dialog = "2"` |
| Dialog plugin in lib.rs | ✅ | Plan 01 Task 1b: `.plugin(tauri_plugin_dialog::init())` |
| Plugin registration order | ✅ | Plan 01 Task 1b Step 2a: plugins added after `.setup()` block |
| Capability permission | ✅ | Plan 01 Task 1b Step 3: `"dialog:default"` added to permissions |
| invoke_handler registration | ✅ | Plan 01 Task 1b Step 2b: `.invoke_handler(tauri::generate_handler![scan_folders])` |
| invoke() API usage | ✅ | Plan 03 Task 4: `invoke("scan_folders", {...})` — correct Tauri v2 |
| listen() API usage | ✅ | Plan 03 Task 4: `listen<ScanProgressEvent>("scan:progress", ...)` |
| listen() cleanup | ✅ | Plan 03 Task 4 Step 3c: useEffect returns `unlisten()` cleanup |
| Dynamic import for dialog | ✅ | Plan 02 Task 2: `await import("@tauri-apps/plugin-dialog")` — avoids bundling in dev |
| Tauri availability fallback | ✅ | All Tauri API calls wrapped in try/catch (Plans 02 and 03) |

---

## Import Consistency Check

### `cn` import pattern

| Component | Plan | Import used | Codebase pattern | Consistent? |
|-----------|------|-------------|------------------|-------------|
| FolderPicker (02-02 Task 2) | `import { cn } from "cn"` | ✅ | All shadcn/ui `*.tsx` use `"cn"` | ✅ |
| ScanOptions (02-02 Task 3) | `import { cn } from "cn"` | ✅ | Same pattern | ✅ |
| DuplicateFinder (02-03 Task 4) | `import { cn } from "cn"` | ✅ | Same pattern | ✅ |

All shadcn/ui components in the codebase (`button.tsx`, `input.tsx`, `checkbox.tsx`, `label.tsx`, `separator.tsx`, etc.) import from `"cn"` (the npm package, not `"@/lib/utils"`). The plan correctly matches this pattern. ✅

### shadcn/ui component imports

| Component | Import path | Exists? | Added by Plan? |
|-----------|-------------|---------|----------------|
| Button | `@/components/ui/button` | ✅ | ✅ (already exists from Phase 1) |
| Input | `@/components/ui/input` | ✅ | ✅ (already exists from Phase 1) |
| Checkbox | `@/components/ui/checkbox` | ✅ | ✅ (already exists from Phase 1) |
| Label | `@/components/ui/label` | ✅ | ✅ (already exists from Phase 1) |
| Separator | `@/components/ui/separator` | ✅ | ✅ (already exists from Phase 1) |
| Card | `@/components/ui/card` | Will be added | ✅ (Plan 01 Task 1: `npx shadcn@latest add progress card badge`) |
| Progress | `@/components/ui/progress` | Will be added | ✅ (Plan 01 Task 1) |
| Badge | `@/components/ui/badge` | Will be added | ✅ (Plan 01 Task 1) |

### @/* alias resolution

tsconfig.json: `"@/*": ["./src/*"]`. All plan imports resolve correctly:

| Import | Resolves to | Exists after execution? |
|--------|-------------|------------------------|
| `@/components/FolderPicker` | `src/components/FolderPicker.tsx` | ✅ (created in Plan 02) |
| `@/components/ScanOptions` | `src/components/ScanOptions.tsx` | ✅ (created in Plan 02) |
| `@/components/ScanProgress` | `src/components/ScanProgress.tsx` | ✅ (created in Plan 03) |
| `@/components/DuplicateFinder` | `src/components/DuplicateFinder.tsx` | ✅ (exists, modified in Plan 03) |
| `@/components/ui/button` | `src/components/ui/button.tsx` | ✅ (exists) |

### lucide-react imports

| Icon | Used in Plan | Available in codebase? |
|------|-------------|----------------------|
| FolderIcon, X, FolderOpen | Plan 02 Task 2 | ✅ (lucide-react v1.61.0, all standard icons) |
| CheckCircle2, AlertCircle, RefreshCw, Loader2, Scan | Plan 03 Task 4/5 | ✅ (all standard lucide-react icons) |

---

## files_modified vs Actual Changes

| Plan | files_modified list | Matches task output? |
|------|---------------------|---------------------|
| 02-01 | Cargo.toml, lib.rs, default.json, package.json, progress.tsx, card.tsx, badge.tsx | ✅ All accounted for (3 shadcn additions + 3 Rust/config modifications + package.json) |
| 02-02 | FolderPicker.tsx (NEW), ScanOptions.tsx (NEW) | ✅ Both are new component files, no existing files modified |
| 02-03 | DuplicateFinder.tsx (MODIFIED), ScanProgress.tsx (NEW) | ✅ Placeholder replaced + new progress component |

All files in frontmatter are created/modified by the corresponding task actions. ✅

---

## Task Structure Validation

| Plan | Task | Type | Files | Action | Verify | Done |
|------|------|------|-------|--------|--------|------|
| 02-01 | Task 1 | auto | ✅ | ✅ (3 numbered steps) | ✅ (automated) | ✅ (checklist) |
| 02-01 | Task 1b | auto | ✅ | ✅ (5 numbered steps) | ✅ (automated) | ✅ (checklist) |
| 02-02 | Task 2 | tracer | ✅ | ✅ (6 numbered steps, detailed) | ✅ (automated) | ✅ (checklist) |
| 02-02 | Task 3 | auto | ✅ | ✅ (6 numbered steps) | ✅ (automated) | ✅ (checklist) |
| 02-03 | Task 4 | auto | ✅ | ✅ (4 numbered steps) | ✅ (automated) | ✅ (checklist) |
| 02-03 | Task 5 | auto | ✅ | ✅ (6 numbered steps) | ✅ (automated) | ✅ (checklist) |

All tasks have required elements (Files, Action, Verify, Done). ✅
Plan 02 Task 2 is correctly typed as `tracer`. ✅

---

## Structured Issues (YAML)

```yaml
issues:
  - dimension: task_completeness
    severity: blocker
    required_property: "All Tauri command parameter types must derive serde::Deserialize"
    description: "Plan 01 Task 1b defines ScanOptions without #[derive(serde::Deserialize)]. The Tauri command cannot deserialize the frontend JSON payload into this struct without this derive. cargo check will fail."
    plan: "02-01"
    task: 1b

  - dimension: task_completeness
    severity: blocker
    required_property: "All Tauri command return types must derive serde::Serialize"
    description: "Plan 01 Task 1b defines SessionInfo without #[derive(serde::Serialize)]. The Tauri command cannot serialize the return value back to the frontend without this derive. cargo check will fail."
    plan: "02-01"
    task: 1b

  - dimension: task_completeness
    severity: blocker
    required_property: "Every Rust attribute used in the plan must have its import specified"
    description: "Plan 01 Task 1b Step 2c uses #[command] on scan_folders but does not include the required import (use tauri::command; or use #[tauri::command]). This will cause cargo check to fail with an unknown attribute error."
    plan: "02-01"
    task: 1b

  - dimension: task_completeness
    severity: blocker
    required_property: "Rust struct fields must deserialize correctly from frontend JSON keys"
    description: "Plan 01 Task 1b ScanOptions uses snake_case field names but the frontend in Plan 03 sends camelCase JSON keys. Without #[serde(rename)] attributes, fields deserialize to default values (false, false, 0). The command compiles but runs with incorrect scan options."
    plan: "02-01"
    task: 1b

  - dimension: key_links_planned
    severity: warning
    required_property: "Dependent artifacts are wired by a task, not merely created"
    description: "Plan 02 must_haves.key_links does not include the data flow from ScanOptions options → DuplicateFinder (Wave 3) → scan_folders Tauri command. This is critical wiring for the phase goal but is implicit through the props interface."
    plan: "02-02"

  - dimension: verification_derivation
    severity: warning
    required_property: "must_haves.truths reflect user-observable outcomes"
    description: "Plan 01 must_haves.truths include implementation-focused items ('Tauri dialog plugin compiles', 'Dialog permission is in capabilities/default.json', 'All new imports resolve'). These should be reframed as user-observable truths (e.g., 'User can open the native folder picker dialog')."
    plan: "02-01"

  - dimension: requirement_coverage
    severity: info
    required_property: "Requirements should be claimed by the plan that implements them"
    description: "Plan 01 claims all 7 requirements (DF-INPUT-01 through DF-INPUT-07) but has zero user-facing tasks. The actual implementation is in Plans 02 and 03. This is acceptable but the frontmatter claim could mislead reviewers about where each requirement is implemented."
    plan: "02-01"

  - dimension: scope_sanity
    severity: info
    required_property: "Each plan stays within the per-plan context budget"
    description: "Plan 01 estimate: 13000 calibrated tokens, 9000 raw, 2 tasks. Plan 02 estimate: 22000 calibrated, 15000 raw, 2 tasks. Plan 03 estimate: 20000 calibrated, 14000 raw, 2 tasks. All within target range (no plan exceeds 4 tasks)."
    plan: "02-01, 02-02, 02-03"
```

---

## Summary

Phase 2's three plans form a coherent progression: infrastructure (Plan 01) → components (Plan 02) → orchestration (Plan 03). All 7 requirements (DF-INPUT-01 through DF-INPUT-07) are addressed by at least one task. Wave ordering is correct, Tauri v2 patterns are followed correctly (dynamic imports, invoke/listen, capability permissions, try/catch for non-Tauri contexts), and import paths match the existing codebase conventions.

**Four blockers** must be fixed before execution — all in Plan 01 Task 1b's Rust struct definitions:

1. **Missing `#[derive(serde::Deserialize)]`** on `ScanOptions` — command parameter deserialization will fail, `cargo check` errors out.
2. **Missing `#[derive(serde::Serialize)]`** on `SessionInfo` — return value serialization will fail, `cargo check` errors out.
3. **Missing Tauri command import** — `#[command]` attribute without `use tauri::command;` (or using `#[tauri::command]`) causes compilation error.
4. **serde snake_case/camelCase mismatch** — even if the code compiles, scan options will silently deserialize to default values (false, false, 0). This is the most dangerous issue because it produces wrong behavior without any error.

These four issues are all in the same area (the Rust struct definitions in Plan 01 Task 1b Step 2c) and can be fixed together with one focused edit:
```rust
#[derive(serde::Deserialize)]
pub struct ScanOptions {
    #[serde(rename = "includeSubdirectories")]
    pub include_subdirectories: bool,
    #[serde(rename = "includeVideoFiles")]
    pub include_video_files: bool,
    #[serde(rename = "minFileSizeBytes")]
    pub min_file_size_bytes: u64,
}
#[derive(serde::Serialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub folder_count: usize,
}
```
And changing `#[command]` to `#[tauri::command]` for the function.

After fixing these four blockers, the plan set is ready for execution.
