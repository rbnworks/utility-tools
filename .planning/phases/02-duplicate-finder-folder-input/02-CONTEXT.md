# Context: Phase 2 — Duplicate Finder — Folder Input

**Date:** 2026-10-25
**Discussed:** 2026-10-25

## Domain

Folder picker with scan options and scan trigger UI. User selects folders to scan for duplicates, configures scan parameters, and initiates the scan.

## Technical Approach

- Tauri native folder picker dialog (`dialog::open`) for native OS integration
- Manual text input for paths as alternative picker method
- Drag-and-drop for adding folders to list
- Inline validation on folder path entry
- Real-time progress via Tauri events

## Decisions

### Folder Picker
- Both native OS dialog AND manual text input available
- Drag-and-drop enabled (overrode PROJECT.md out-of-scope decision)
- Max 5 folders added at once
- No priority ordering between folders
- No folder reorder capability needed

### Scan Options UI
- Checkboxes for subdirectory scanning and video file inclusion
- Min file size input: numeric value in MB (not KB or GB)
- Clean, grouped layout for scan options

### Validation
- Folder paths validated immediately on input/change (not on scan)
- Inline error display (red text below input field)
- Invalid paths blocked from being added to the list

### Progress Display
- Inline progress shown within input form (scan options remain visible)
- Header section shows small animated spinner icon with hover tooltip displaying "Scanning: X/Y files"
- Progress overlay modal available as alternative view (dismissed by clicking outside)
- Smooth transition from input view to progress view

### Multi-Folder UX
- Visual list of added folders with name/path and remove button (×)
- Up to 5 folders displayed
- Sorted alphabetically by default
- No drag-to-reorder needed

## Deferred Ideas
- Export results to CSV/JSON (DF-V2-02)
- Exclude specific file patterns (DF-V2-01)
- Drag-and-drop on results view (future phase)
- Cloud storage integration (out of scope)

---
*Created: 2026-10-25*
