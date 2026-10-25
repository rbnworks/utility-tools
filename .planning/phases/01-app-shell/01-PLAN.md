# Phase 1 Plan: App Shell

**Created:** 2026-10-25
**Phase Goal:** Desktop app with dark-themed sidebar navigation, layout, and tool routing.
**Requirements:** SHELL-01, SHELL-02, SHELL-03, SHELL-04, SHELL-05

---

## Plans

### Plan 01-01: Tauri Setup & Window Configuration

**Objective:** Initialize Tauri v2 project, configure window settings (frameless, 80% screen size, dark theme).

**Tasks:**

1. **Install Rust and Tauri prerequisites**
   - Verify/install Rust via rustup
   - Install macOS dependencies: `brew install webkitgtk`
   - Initialize Tauri project: `npx tauri@latest init` in project root

2. **Configure `tauri.conf.json`**
   - Window: frameless, 80% screen size, centered
   - App URL: Vite dev server URL (localhost)
   - Enable Tauri events for future use
   - Set appropriate security CSP settings

3. **Verify Tauri dev mode**
   - Ensure `tauri dev` starts the app
   - Confirm window displays correctly at 80% size
   - Verify dark theme CSS variables are loaded

### Plan 01-02: Project Structure & Dependencies

**Objective:** Set up correct project structure and install all UI dependencies.

**Tasks:**

1. **Move `lib/utils.ts` to `src/lib/utils.ts`**
   - Create `src/lib/` directory
   - Move `lib/utils.ts` → `src/lib/utils.ts`
   - Ensure `@/*` path alias resolves correctly
   - Verify `cn()` helper works in imports

2. **Install shadcn/ui components**
   - Use CLI to install all required components:
     - Button, Sheet, Separator, Tabs, Dialog
     - Dropdown Menu, Tooltip, Scroll Area, Collapsible
     - Label, Input, Checkbox, AlertDialog, Toast
   - Install to `components/ui/` (already scaffolded)
   - Verify all components import correctly

3. **Install Kokonut UI components**
   - Install components relevant to v1 features:
     - ParticleButton (already installed)
     - NotificationStack (for toast notifications)
     - LiquidGauge (for scan progress)
   - Verify Kokonut components import and render

4. **Install/verify icon library**
   - lucide-react already in dependencies
   - Verify icons render correctly in sidebar

### Plan 01-03: App Shell Layout (Sidebar + Content Area)

**Objective:** Build the app shell with sidebar navigation and main content area.

**Tasks:**

1. **Create Sidebar component** (`src/components/Sidebar.tsx`)
   - Header/logo area at top
   - Fixed width (~240px)
   - Dark theme styling
   - "Tools" section label
   - First tool: "Duplicate Finder" with icon
   - Placeholder for future tools (expandable section)
   - Active tool highlighting

2. **Update App.tsx shell**
   - Layout: Sidebar (left) + Content Area (right)
   - State: `activeTool` with useState
   - Default active tool: Duplicate Finder
   - Content area renders selected tool component
   - Flexbox/grid layout ensuring sidebar stays fixed

3. **Create placeholder tool component** (`src/components/DuplicateFinder.tsx`)
   - Basic placeholder: "Duplicate Finder — Phase 2"
   - Ready to receive Phase 2 implementation
   - Demonstrates sidebar navigation works

4. **Ensure dark theme applies globally**
   - CSS variables from `src/index.css` applied to root
   - Sidebar uses dark theme colors
   - Content area uses dark theme colors
   - Verify on different content heights

### Plan 01-04: Navigation State & Verification

**Objective:** Wire up navigation state, test all interactions, ensure all success criteria met.

**Tasks:**

1. **Implement navigation state**
   - Sidebar click → updates `activeTool` state
   - Content area switches based on selection
   - Active state visual indicator (different background/border)
   - Verify keyboard navigation works (Tab through sidebar items)

2. **Verify all success criteria**
   - [ ] App launches as Tauri desktop window with dark theme
   - [ ] Left sidebar shows utility tools with icons, active state, expandable section
   - [ ] Clicking sidebar item renders content area; default tool shown on launch
   - [ ] All shadcn/ui and Kokonut UI components installable
   - [ ] `src/lib/utils.ts` exists, `cn()` resolves via alias

3. **Edge case handling**
   - Window resize (80% should adapt to screen size)
   - Long tool names in sidebar (truncation/ellipsis)
   - Empty content area (if no tool selected)

4. **Smoke test**
   - Run `tauri dev` and interact with app
   - Click sidebar items → content changes
   - Verify dark theme consistent
   - Check for console errors

---
*Plan created: 2026-10-25*
