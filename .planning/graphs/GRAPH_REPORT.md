# Graph Report - utility-tools  (2026-10-25)

## Corpus Check
- 64 files · ~60,625 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 5 file(s) not represented in the graph (top: .lock 1, (none) 1, .icns 1)

## Summary
- 574 nodes · 758 edges · 49 communities (38 shown, 11 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 29 edges (avg confidence: 0.95)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8236ca0d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- dependencies
- particle-button.tsx
- PLAN.md — Utility Tools Desktop App
- compilerOptions
- components.json
- v1 Requirements
- Utility Tools
- utility-tools
- react
- ROADMAP: Utility Tools
- STATE.md — Utility Tools
- shadcn
- cn
- Communities (15 total, 2 thin omitted)
- Plan Check: Phase 2 — Duplicate Finder — Folder Input
- tauri.conf.json
- dialog.tsx
- devDependencies
- alert-dialog.tsx
- package.json
- Decisions
- sheet.tsx
- Phase 02: Duplicate Finder — Folder Input — Research
- Completed Work
- Completed Work
- lib.rs
- Completed: 02-01, 02-02, 02-03
- Code Examples
- Common Pitfalls
- sonner.tsx
- @tailwindcss/vite
- ScanOptions
- Architecture Patterns
- TODO: Phase 2 — Duplicate Finder — Folder Input
- default.json
- scripts
- Standard Stack
- 02-02-PLAN.md
- 02-03-PLAN.md
- Security Domain
- motion
- @tauri-apps/plugin-dialog
- app

## God Nodes (most connected - your core abstractions)
1. `react` - 25 edges
2. `cn()` - 22 edges
3. `cn` - 18 edges
4. `compilerOptions` - 17 edges
5. `Phase 02: Duplicate Finder — Folder Input — Research` - 17 edges
6. `radix-ui` - 16 edges
7. `PLAN.md — Utility Tools Desktop App` - 15 edges
8. `Communities (15 total, 2 thin omitted)` - 14 edges
9. `lucide-react` - 13 edges
10. `ScanOptions()` - 13 edges

## Surprising Connections (you probably didn't know these)
- `Pattern: Tauri Command Definition (Rust side)` --references--> `ScanOptions()`  [INFERRED]
  .planning/phases/02-duplicate-finder-input/02-RESEARCH.md → src/components/ScanOptions.tsx
- `Wave 1: Dependencies & Tauri Command Layer` --references--> `ScanOptions()`  [INFERRED]
  .planning/phases/02-duplicate-finder-input/02-SUMMARY.md → src/components/ScanOptions.tsx
- `File Path Alias` --references--> `cn()`  [INFERRED]
  PLAN.md → src/lib/utils.ts
- `Phase 1: Foundation` --references--> `cn()`  [INFERRED]
  PLAN.md → src/lib/utils.ts
- `Suggested Questions` --references--> `cn()`  [INFERRED]
  .planning/graphs/GRAPH_REPORT.md → src/lib/utils.ts

## Import Cycles
- None detected.

## Communities (49 total, 11 thin omitted)

### Community 0 - "dependencies"
Cohesion: 0.13
Nodes (15): dependencies, clsx@^2.1.1, cn@^0.5.0, lucide-react@^1.61.0, radix-ui@^1.8.1, react@^19.3.1, react-dom@^19.3.1, tailwind-merge@^3.7.0 (+7 more)

### Community 1 - "particle-button.tsx"
Cohesion: 0.11
Nodes (19): Burst(), burstColor(), Check(), createParticles(), EASE_OUT, isPromiseLike(), LABEL_TRANSITION, Particle (+11 more)

### Community 2 - "PLAN.md — Utility Tools Desktop App"
Cohesion: 0.05
Nodes (41): 1. Folder Selection, 2. Scan Progress, 3. Duplicate Results, Actions Per File, Algorithm, App Shell (`src/App.tsx`), Architecture, Assumptions & Risks (+33 more)

### Community 3 - "compilerOptions"
Cohesion: 0.11
Nodes (18): compilerOptions, allowImportingTsExtensions, isolatedModules, jsx, lib, module, moduleResolution, noEmit (+10 more)

### Community 4 - "components.json"
Cohesion: 0.12
Nodes (16): aliases, components, ui, utils, iconLibrary, registries, @kokonutui, rsc (+8 more)

### Community 5 - "v1 Requirements"
Cohesion: 0.14
Nodes (13): App Shell, Duplicate Detection — Matching, Duplicate Finder — Actions, Duplicate Finder — Enhancements, Duplicate Finder — Input, Duplicate Finder — Results Display, Duplicate Finder — Scanning, Future Tools (+5 more)

### Community 6 - "Utility Tools"
Cohesion: 0.15
Nodes (12): Active, Business Context, Constraints, Context, Core Value, Evolution, Key Decisions, Out of Scope (+4 more)

### Community 7 - "utility-tools"
Cohesion: 0.17
Nodes (11): Commands, Gotchas, GSD commands, GSD Workflow, Key planning docs, Key Technical Decisions, Kokonut UI, Prerequisites (+3 more)

### Community 8 - "react"
Cohesion: 0.08
Nodes (33): Custom Components (build), Surprising Connections (you probably didn't know these), cn, lucide-react, radix-ui, react, App(), DEFAULT_OPTIONS (+25 more)

### Community 9 - "ROADMAP: Utility Tools"
Cohesion: 0.20
Nodes (9): Dependencies, Phase 1: App Shell, Phase 2: Duplicate Finder — Folder Input, Phase 3: Duplicate Detection Engine, Phase 4: Duplicate Results, Phase 5: Duplicate Actions, Phase 6: Polish & Testing, Risk Mitigation (+1 more)

### Community 10 - "STATE.md — Utility Tools"
Cohesion: 0.20
Nodes (9): Active Threads, Context, Phase Progress, Project Reference, Project State, Recent Changes, Requirement Status, Session (+1 more)

### Community 11 - "shadcn"
Cohesion: 0.25
Nodes (7): mcp, shadcn, plugin, $schema, command, enabled, type

### Community 12 - "cn"
Cohesion: 0.07
Nodes (37): Component Installation, Context: Phase 1 — App Shell, Decisions, Deferred Ideas, Domain, Navigation State, Project Structure, Sidebar Navigation (+29 more)

### Community 15 - "Communities (15 total, 2 thin omitted)"
Cohesion: 0.08
Nodes (23): Communities (15 total, 2 thin omitted), Community 0 - "package.json", Community 10 - "STATE.md — Utility Tools", Community 11 - "shadcn", Community 12 - "Task Breakdown", Community 1 - "particle-button.tsx", Community 2 - "PLAN.md — Utility Tools Desktop App", Community 3 - "compilerOptions" (+15 more)

### Community 16 - "Plan Check: Phase 2 — Duplicate Finder — Folder Input"
Cohesion: 0.08
Nodes (23): ADVISORY: ScanProgress and progress listener active with no Phase 2 events, @/* alias resolution, BLOCK: Missing serde derive on SessionInfo — command return value serialization will fail, BLOCK: Missing serde derives on ScanOptions — command parameter deserialization will fail, BLOCK: Missing Tauri v2 command import — compilation will fail, BLOCK: serde snake_case / camelCase naming mismatch — scan options will silently deserialze to defaults, `cn` import pattern, files_modified vs Actual Changes (+15 more)

### Community 17 - "tauri.conf.json"
Cohesion: 0.09
Nodes (21): app, security, windows, withGlobalTauri, build, beforeBuildCommand, beforeDevCommand, devUrl (+13 more)

### Community 19 - "dialog.tsx"
Cohesion: 0.15
Nodes (9): Primary (HIGH confidence), Secondary (MEDIUM confidence), Sources, Tertiary (LOW confidence), Dialog(), DialogContent(), DialogFooter(), DialogOverlay() (+1 more)

### Community 20 - "devDependencies"
Cohesion: 0.15
Nodes (13): devDependencies, shadcn@^4.22.1, @tauri-apps/cli@^2.12.4, @types/react@^19.3.0, @types/react-dom@^19.3.0, typescript@^7.0.2, vite@^8.3.5, shadcn (+5 more)

### Community 21 - "alert-dialog.tsx"
Cohesion: 0.18
Nodes (5): AlertDialogAction(), AlertDialogCancel(), AlertDialogContent(), AlertDialogOverlay(), AlertDialogPortal()

### Community 22 - "package.json"
Cohesion: 0.18
Nodes (10): author, @radix-ui/react-slot@^1.5.0, description, keywords, license, main, name, type (+2 more)

### Community 23 - "Decisions"
Cohesion: 0.18
Nodes (10): Context: Phase 2 — Duplicate Finder — Folder Input, Decisions, Deferred Ideas, Domain, Folder Picker, Multi-Folder UX, Progress Display, Scan Options UI (+2 more)

### Community 24 - "sheet.tsx"
Cohesion: 0.22
Nodes (3): SheetContent(), SheetOverlay(), SheetPortal()

### Community 25 - "Phase 02: Duplicate Finder — Folder Input — Research"
Cohesion: 0.20
Nodes (9): Architectural Responsibility Map, Assumptions Log, Environment Availability, Metadata, Package Legitimacy Audit, Phase 02: Duplicate Finder — Folder Input — Research, Project Constraints (from AGENTS.md), State of the Art (+1 more)

### Community 26 - "Completed Work"
Cohesion: 0.22
Nodes (8): 1. Tauri v2 Initialized, 2. Project Structure Fixed, 3. UI Dependencies Installed, 4. TypeScript Fixed, Completed Work, Files Changed, Summary: Plan 01-01 — Tauri Setup & Project Structure, Verification

### Community 27 - "Completed Work"
Cohesion: 0.22
Nodes (8): 1. Navigation State Verified, 2. Tauri App Compiles and Launches, 3. All Success Criteria Verified, 4. Components Verified, Completed Work, Files Changed (Plan 01-04), Summary: Plan 01-04 — Navigation State & Verification, Verification Notes

### Community 28 - "lib.rs"
Cohesion: 0.32
Nodes (3): scan_folders(), ScanOptions, SessionInfo

### Community 29 - "Completed: 02-01, 02-02, 02-03"
Cohesion: 0.25
Nodes (7): Completed: 02-01, 02-02, 02-03, Phase 2 Summary: Duplicate Finder — Input, Requirements Met, Verification, Wave 1: Dependencies & Tauri Command Layer, Wave 2: FolderPicker + ScanOptions Components, Wave 3: DuplicateFinder Orchestrator + ScanProgress

### Community 30 - "Code Examples"
Cohesion: 0.29
Nodes (7): Code Examples, Frontend: DuplicateFinder orchestrator with view state, Frontend: Folder list with add/remove (React), Frontend: Scan options component, Tauri: Frontend event listening for progress, Tauri: Frontend invoke call to start scan, Tauri: Rust-side folder picker via command

### Community 31 - "Common Pitfalls"
Cohesion: 0.29
Nodes (7): Common Pitfalls, Pitfall 1: Dialog plugin not registered in Cargo.toml, Pitfall 2: Missing `tauri-plugin-fs` dependency, Pitfall 3: Tauri JS packages missing from package.json, Pitfall 4: Capability for dialog not in default.json, Pitfall 5: Path validation race condition, Pitfall 6: Path separator cross-platform issues

### Community 32 - "sonner.tsx"
Cohesion: 0.33
Nodes (4): next-themes@^0.4.6, sonner@^2.0.8, next-themes, sonner

### Community 33 - "@tailwindcss/vite"
Cohesion: 0.33
Nodes (4): @tailwindcss/vite@^4.3.3, @vitejs/plugin-react@^6.1.2, @tailwindcss/vite, @vitejs/plugin-react

### Community 34 - "ScanOptions"
Cohesion: 0.33
Nodes (5): Summary, Wave Dependency Check, Open Questions, Summary, ScanOptions()

### Community 35 - "Architecture Patterns"
Cohesion: 0.33
Nodes (6): Architecture Patterns, Pattern: Tauri Command Definition (Rust side), Pattern: Tauri Command → Event Flow, Pattern: Tauri Command Registration, Recommended Project Structure, System Architecture Diagram

### Community 36 - "TODO: Phase 2 — Duplicate Finder — Folder Input"
Cohesion: 0.33
Nodes (5): Final Verification, TODO: Phase 2 — Duplicate Finder — Folder Input, Wave 1: Setup (dependencies + Rust), Wave 2: Implementation (tracer + expansion), Wave 3: Integration (orchestrator + progress)

### Community 37 - "default.json"
Cohesion: 0.33
Nodes (5): description, identifier, permissions, $schema, windows

### Community 38 - "scripts"
Cohesion: 0.50
Nodes (4): scripts, build, dev, preview

### Community 39 - "Standard Stack"
Cohesion: 0.50
Nodes (4): Alternatives Considered, Core, Standard Stack, Supporting

### Community 43 - "Security Domain"
Cohesion: 0.67
Nodes (3): Applicable ASVS Categories, Known Threat Patterns for Tauri + React, Security Domain

## Knowledge Gaps
- **1 isolated node(s):** `app`
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 348 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **11 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `cn()` connect `cn` to `particle-button.tsx`, `PLAN.md — Utility Tools Desktop App`, `ScanOptions`, `react`, `ROADMAP: Utility Tools`, `Communities (15 total, 2 thin omitted)`, `Plan Check: Phase 2 — Duplicate Finder — Folder Input`, `Completed Work`?**
  _High betweenness centrality (0.200) - this node is a cross-community bridge._
- **Are the 13 inferred relationships involving `cn()` (e.g. with `File Path Alias` and `Phase 1: Foundation`) actually correct?**
  _`cn()` has 13 INFERRED edges - model-reasoned connections that need verification._
- **What connects `app` to the rest of the system?**
  _1 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `dependencies` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._
- **Why does `react` connect `react` to `dependencies`, `particle-button.tsx`, `cn`, `dropdown-menu.tsx`, `dialog.tsx`, `alert-dialog.tsx`, `package.json`, `sheet.tsx`?**
  _High betweenness centrality (0.138) - this node is a cross-community bridge._
- **Should `particle-button.tsx` be split into smaller, more focused modules?**
  _Cohesion score 0.11231884057971014 - nodes in this community are weakly interconnected._
- **Why does `ScanOptions()` connect `ScanOptions` to `react`, `Architecture Patterns`, `Completed: 02-01, 02-02, 02-03`?**
  _High betweenness centrality (0.118) - this node is a cross-community bridge._