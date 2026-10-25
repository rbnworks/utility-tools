# Phase 03: Duplicate Detection Engine - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-10-25
**Phase:** 03-Duplicate Detection Engine
**Areas discussed:** Video file handling, Scan architecture, Session management, Performance

---

## Video file handling

| Option | Description | Selected |
|--------|-------------|----------|
| image crate only | Only formats image natively supports. Silent skip for unsupported formats. | |
| image + ffmpeg-next bindings | Full format support via FFmpeg. ~50MB binary size increase. | ✓ |
| image primary, FFmpeg optional | Cargo feature flag for FFmpeg. Small default binary. | |
| First-frame fallback | Hash only first frame. Simple but misses later video differences. | |

**User's choice:** image + ffmpeg-next bindings — full format support
**Notes:** User chose comprehensive video support despite the binary size tradeoff.

## Scan architecture

| Option | Description | Selected |
|--------|-------------|----------|
| Modular (scan.rs, hash.rs, operations.rs) | Clear responsibility boundaries, easier testing. | ✓ |
| Single lib.rs | Fewer files but grows significantly. | |
| Modular with operations.rs now | Build operations.rs now even though Phase 5 is the target. | |

**User's choice:** Modular file structure matching PLAN.md
**Notes:** Follows the existing codebase structure from phases 1-2.

## Session management

| Option | Description | Selected |
|--------|-------------|----------|
| Arc<Mutex<ScanState>> | Simple, works for single scan. Blocks async. | |
| RwLock | Better concurrent reads, minimal overhead. | |
| DashMap + session store | Thread-safe concurrent access, multi-session support. | ✓ |
| Actor-style with channels | Most idiomatic Rust, avoids locks, adds complexity. | |

**User's choice:** DashMap + session store
**Notes:** Multi-session support selected even though single-user desktop app — future-proof architecture.

## Performance — parallelism

| Option | Description | Selected |
|--------|-------------|----------|
| Sequential | One file at a time. Fine for small directories. | |
| Rayon parallel | Parallel hash computation via Rayon thread pool. | ✓ |
| Async parallel with tokio | spawn_blocking per file, complex lifecycle management. | |
| Chunked parallel | Rayon by chunks of ~50, balance throughput + responsiveness. | |

**User's choice:** Rayon parallel hash computation
**Notes:** Safe, well-tested parallelism chosen. Each file's three hashes computed concurrently.

---

## the agent's Discretion

- Choice of specific Rust crates for image processing (image-hash pre-built implementations vs. custom)
- Exact file scanning filter patterns (beyond .git, node_modules)
- Hash cache data structure internals
- Error handling granularity during scan

## Deferred Ideas

None — discussion stayed within phase scope.
