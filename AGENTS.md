# Rythm Box — AI Agent Guide

## Commands

```bash
# Browser-only dev (no audio)
npm run dev

# Desktop app with audio
cargo tauri dev

# Build frontend (tsc -b then vite build)
npm run build

# Production desktop build
cargo tauri build

# Lint (oxlint, not eslint)
npm run lint

# CI equivalent — run before PR
cargo check --workspace && cargo clippy --workspace -- -D warnings && cargo fmt --check
npm run lint && tsc -b
```

## Architecture

- **Cargo workspace**: `rhythm-core/` (Rust library — models, audio engine, project serialization), `src-tauri/` (Tauri v2 app — commands, window shell)
- **Frontend**: `src/` — React 19 + TypeScript 6 + Tailwind CSS v4 (via `@tailwindcss/vite` plugin, no config file)
- **Entrypoints**: `src/main.tsx` (React), `src-tauri/src/main.rs` (Tauri), `rhythm-core/src/lib.rs` (library)
- **Vite** on port 5173 (strict), `esnext` target. `TAURI_DEBUG` env controls minification/sourcemaps.

## Conventions

- **TypeScript**: `verbatimModuleSyntax`, `noUnusedLocals`, `noUnusedParameters`, `erasableSyntaxOnly` — enforce these via `tsc -b`
- **No tests exist yet** (zero Rust `#[test]`, zero JS test files)
- **No pre-commit hooks** configured
- CI runs `cargo build --workspace`, `cargo test --workspace`, clippy, fmt — plus `oxlint` on frontend
