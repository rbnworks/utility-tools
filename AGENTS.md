# utility-tools

> **Status:** Planning phase — Tauri desktop app for finding duplicate images/videos
> **Last updated:** 2026-10-25 after project initialization

## When to consult this file

If an agent is unsure about a convention or command that isn't obvious from the code, check here before guessing.

## GSD Workflow

This project uses GSD (Goal-Driven Development) for planning and execution.

### Key planning docs

| File | Purpose |
|------|---------|
| `.planning/PROJECT.md` | Project context, constraints, key decisions |
| `.planning/REQUIREMENTS.md` | 36 v1 requirements with REQ-IDs |
| `.planning/ROADMAP.md` | 6 phases covering all requirements |
| `.planning/STATE.md` | Project state tracking |
| `.planning/config.json` | Workflow preferences (yolo, adaptive, verifier) |
| `PLAN.md` | Technical specification (Tauri + Rust + React) |

### GSD commands

- `/gsd-discuss-phase 1` — gather context and clarify approach for Phase 1
- `/gsd-plan-phase 1` — skip discussion, plan directly
- `/gsd-progress` — check progress across all phases
- `/gsd-verify-work` — validate built features against requirements

## Commands

```bash
npm run dev        # Start Vite dev server
npm run build      # Build for production
npm run preview    # Preview production build
```

## Structure

```
utility-tools/
├── src/                          # React frontend (Vite)
│   ├── main.tsx                  # App entry point
│   ├── index.css                 # Dark theme CSS variables
│   ├── App.tsx                   # App shell (placeholder)
│   └── lib/                      # (needs creation — utils.ts is at /lib/)
│       └── utils.ts              # cn() helper — must be here for tsconfig alias
├── src-tauri/                    # (not created yet — Phase 1 setup)
│   ├── Cargo.toml
│   ├── build.rs
│   ├── tauri.conf.json
│   └── src/
│       ├── main.rs               # Tauri entry point
│       ├── lib.rs                # Tauri commands
│       ├── scan.rs               # File scanning
│       ├── hash.rs               # Perceptual hashing
│       └── operations.rs         # File operations
├── components/
│   ├── ui/                       # shadcn/ui components (not installed yet)
│   └── kokonutui/                # Kokonut UI components (not installed yet)
├── lib/utils.ts                  # cn() helper — WILL BE MOVED to src/lib/
├── .planning/                    # GSD planning docs
├── components.json               # shadcn/ui config with @kokonutui registry
├── package.json
├── tsconfig.json                 # @/* path alias → ./ (needs src/lib/ for utils)
├── vite.config.ts
├── tailwind.config.ts
└── PLAN.md                       # Technical specification
```

## Kokonut UI

This project installs components from Kokonut UI, a shadcn/ui registry.
The registry is set in components.json as "@kokonutui".

- Before building an animated or interactive component from scratch, check
  Kokonut UI: https://kokonutui.com/llms.txt lists every component with a
  one-line description.
- Install with the shadcn CLI, never by pasting code:
  `npx shadcn@latest add @kokonutui/<name>`
  It writes `components/kokonutui/<name>.tsx` and installs its dependencies.
- Installed components are project code. Edit them in place rather than wrapping them.
- Any docs page is available as Markdown, with the full source, by adding
  `.md` to its URL, e.g. `https://kokonutui.com/docs/buttons/particle-button.md`

## Key Technical Decisions

1. **Tauri v2 over Electron** — lightweight (~5MB binary), Rust backend, direct file system access
2. **Rust backend** — perceptual hashing, file scanning, file operations all in Rust
3. **Three hash algorithms** — aHash + dHash + pHash with triple-agreement matching
4. **Tauri events for real-time progress** — no polling
5. **Dark theme default** — CSS variables configured in src/index.css
6. **shadcn/ui + Kokonut UI** — base components + animated UI components

## Prerequisites

- **Node.js 18+** (currently v26.12.1) — installed
- **Rust** — NOT installed yet; install via `rustup`:
  ```
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source "$HOME/.cargo/env"
  ```
- **Tauri dependencies** (macOS):
  ```
  brew install webkitgtk
  ```

## Gotchas

- **CSS must be imported from the entry point.** `src/index.css` is the Tailwind entry — it must be `import`ed in `src/main.tsx` (or whatever the app root is). Vite does not auto-discover CSS by path proximity.
- **`lib/utils.ts` at project root won't resolve via `@/` alias.** The tsconfig only includes `src/`. Must be moved to `src/lib/utils.ts`.
- **Vite alone cannot scan file system.** Browser environment has no fs access. Tauri backend is required.
- **Tauri init must run in this project directory.** Run `npx tauri@latest init` to scaffold `src-tauri/`.
- **CORS does not apply in Tauri.** Tauri commands run in native Rust, not a web server.
- **File operations in Rust are permanent.** No trash can integration by default — confirm with user before batch delete/move.
