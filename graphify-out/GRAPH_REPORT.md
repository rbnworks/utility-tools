# Graph Report - utility-tools  (2026-10-25)

## Corpus Check
- 19 files · ~9,404 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 1 file(s) not represented in the graph (top: .css 1)

## Summary
- 225 nodes · 262 edges · 15 communities (13 shown, 2 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 3 edges (avg confidence: 0.95)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `18e2d96d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- package.json
- particle-button.tsx
- PLAN.md — Utility Tools Desktop App
- compilerOptions
- components.json
- v1 Requirements
- Utility Tools
- utility-tools
- Frontend Structure
- ROADMAP: Utility Tools
- STATE.md — Utility Tools
- shadcn
- Task Breakdown

## God Nodes (most connected - your core abstractions)
1. `compilerOptions` - 17 edges
2. `PLAN.md — Utility Tools Desktop App` - 15 edges
3. `ParticleButton()` - 10 edges
4. `Utility Tools` - 9 edges
5. `ROADMAP: Utility Tools` - 9 edges
6. `utility-tools` - 9 edges
7. `cn()` - 8 edges
8. `STATE.md — Utility Tools` - 8 edges
9. `Frontend Structure` - 8 edges
10. `v1 Requirements` - 7 edges

## Surprising Connections (you probably didn't know these)
- `File Path Alias` --references--> `cn()`  [INFERRED]
  PLAN.md → lib/utils.ts
- `Phase 1: Foundation` --references--> `cn()`  [INFERRED]
  PLAN.md → lib/utils.ts
- `Phase 1: App Shell` --references--> `cn()`  [INFERRED]
  .planning/ROADMAP.md → lib/utils.ts
- `ParticleButton()` --calls--> `cn()`  [EXTRACTED]
  components/kokonutui/particle-button.tsx → lib/utils.ts
- `App()` --calls--> `ParticleButton()`  [EXTRACTED]
  src/App.tsx → components/kokonutui/particle-button.tsx

## Import Cycles
- None detected.

## Communities (15 total, 2 thin omitted)

### Community 0 - "package.json"
Cohesion: 0.05
Nodes (49): author, dependencies, class-variance-authority@^0.7.1, clsx@^2.1.1, cn@^0.5.0, lucide-react@^1.61.0, motion@^14.6.0, radix-ui@^1.8.1 (+41 more)

### Community 1 - "particle-button.tsx"
Cohesion: 0.15
Nodes (19): Burst(), burstColor(), Check(), createParticles(), EASE_OUT, isPromiseLike(), LABEL_TRANSITION, Particle (+11 more)

### Community 2 - "PLAN.md — Utility Tools Desktop App"
Cohesion: 0.08
Nodes (24): Algorithm, Architecture, Assumptions & Risks, Backend (Rust) Commands, Custom Components (build), Duplicate Detection: Perceptual Hashing, File Path Alias, File Structure (Final) (+16 more)

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

### Community 8 - "Frontend Structure"
Cohesion: 0.18
Nodes (11): 1. Folder Selection, 2. Scan Progress, 3. Duplicate Results, Actions Per File, App Shell (`src/App.tsx`), Batch Actions, Duplicate Card Design, Duplicate Finder — Views (+3 more)

### Community 9 - "ROADMAP: Utility Tools"
Cohesion: 0.20
Nodes (9): Dependencies, Phase 1: App Shell, Phase 2: Duplicate Finder — Folder Input, Phase 3: Duplicate Detection Engine, Phase 4: Duplicate Results, Phase 5: Duplicate Actions, Phase 6: Polish & Testing, Risk Mitigation (+1 more)

### Community 10 - "STATE.md — Utility Tools"
Cohesion: 0.22
Nodes (8): Active Threads, Context, Phase Progress, Project Reference, Project State, Recent Changes, Requirement Status, STATE.md — Utility Tools

### Community 11 - "shadcn"
Cohesion: 0.25
Nodes (7): mcp, shadcn, plugin, $schema, command, enabled, type

### Community 12 - "Task Breakdown"
Cohesion: 0.29
Nodes (7): Phase 1: Foundation, Phase 2: App Shell, Phase 3: Duplicate Finder — Folder Selection, Phase 4: Duplicate Finder — Scanning, Phase 5: Duplicate Finder — Actions, Phase 6: Polish, Task Breakdown

## Knowledge Gaps
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `cn()` connect `particle-button.tsx` to `ROADMAP: Utility Tools`, `PLAN.md — Utility Tools Desktop App`, `Task Breakdown`?**
  _High betweenness centrality (0.179) - this node is a cross-community bridge._
- **Should `package.json` be split into smaller, more focused modules?**
  _Cohesion score 0.05203619909502263 - nodes in this community are weakly interconnected._
- **Why does `PLAN.md — Utility Tools Desktop App` connect `PLAN.md — Utility Tools Desktop App` to `Frontend Structure`, `Task Breakdown`?**
  _High betweenness centrality (0.149) - this node is a cross-community bridge._
- **Should `particle-button.tsx` be split into smaller, more focused modules?**
  _Cohesion score 0.14666666666666667 - nodes in this community are weakly interconnected._
- **Why does `File Path Alias` connect `PLAN.md — Utility Tools Desktop App` to `particle-button.tsx`?**
  _High betweenness centrality (0.122) - this node is a cross-community bridge._
- **Should `PLAN.md — Utility Tools Desktop App` be split into smaller, more focused modules?**
  _Cohesion score 0.08 - nodes in this community are weakly interconnected._
- **Should `compilerOptions` be split into smaller, more focused modules?**
  _Cohesion score 0.10526315789473684 - nodes in this community are weakly interconnected._