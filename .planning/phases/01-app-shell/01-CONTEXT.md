# Context: Phase 1 — App Shell

**Date:** 2026-10-25
**Discussed:** 2026-10-25

## Domain

Desktop app shell with dark-themed sidebar navigation, layout, and tool routing foundation.

## Technical Approach

- **Tauri v2**: Frameless window, 80% screen size, dark theme default
- **React 19**: Simple state-based navigation (no URL routing for v1)
- **shadcn/ui + Kokonut UI**: All components installed for v1 feature set
- **Project structure**: `src/lib/utils.ts` with `cn()` helper, `@/` path alias resolved

## Decisions

### Sidebar Navigation
- Fixed-width sidebar (not collapsible)
- Header/logo area at top of sidebar
- Tools listed below header (flat list for v1, expandable section for future tools)
- Active tool highlighted with distinct styling

### Window Configuration
- Frameless window (no OS chrome)
- Size: 80% of screen width and height
- Platform: macOS first, Tauri cross-platform ready
- Dark theme applied globally via CSS variables

### Component Installation
- All shadcn/ui base components installed (Button, Sidebar, Tabs, Separator, Dialog, etc.)
- Kokonut UI components needed for animations/interactions
- Icon library: lucide-react (configured in components.json)
- Install via shadcn CLI, not manual copying

### Navigation State
- Simple React state (`useState`) for active tool tracking
- No URL routing library (not needed for single-tool v1)
- Sidebar click → update state → render content area
- Extensible pattern for future tools (Phase 2+)

### Project Structure
- `src/lib/utils.ts` — `cn()` helper and shared utilities
- `src/components/ui/` — shadcn/ui components (auto-installed)
- `src/components/kokonutui/` — Kokonut UI components (auto-installed)
- `src/App.tsx` — App shell with sidebar + content area
- `src/index.css` — Tailwind + dark theme CSS variables

## Deferred Ideas
- URL-based routing for future tools (save for Phase 2+)
- Drag-and-drop folder selection (out of scope for v1)
- Second utility tool (placeholder only, no implementation)

---
*Created: 2026-10-25*
