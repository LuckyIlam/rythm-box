# Rhythm Box

A cross-platform desktop rhythm pattern editor and sequencer built with [Tauri](https://v2.tauri.app/) (Rust backend + React/TypeScript frontend).

Create and edit rhythm patterns with a grid-based timeline, control BPM, assign sounds to instruments, and play back patterns in real time.

## Features

- **Grid Pattern Editor** - click cells to toggle steps on/off per instrument
- **Real-time Playback** - play patterns with real acoustic drum samples (kick, snare, hi-hat, open hat, crash) at configurable BPM
- **Multi-pattern Projects** - create and switch between multiple patterns via tabs
- **Project Save/Load** - save and load projects as `.rythm` JSON files (native Tauri dialog or browser download)
- **Export Patterns** - export individual patterns as standalone JSON
- **Keyboard Shortcuts** - Ctrl+N (new), Ctrl+O (open), Ctrl+S (save), Space (play/stop)
- **Auto-save** - project state auto-saves to localStorage every 2 seconds and restores on launch

## Quick Start

### Prerequisites

- [Node.js](https://nodejs.org/) 22+
- [Rust](https://rustup.rs/) (latest stable)
- System dependencies for [Tauri v2](https://v2.tauri.app/start/prerequisites/)

### Install

```bash
npm install
```

### Run (browser only, no audio)

```bash
npm run dev
```

Opens at `http://localhost:5173`. The full UI works but **no audio** — only `cargo tauri dev` plays sound.

### Run (desktop app with audio)

```bash
cargo tauri dev
```

Launches a native window with the Rust audio engine (cpal/rodio).

### Build for production

```bash
cargo tauri build
```

Produces platform-specific installers in `src-tauri/target/release/bundle/`.

## Project Structure

```
rythm-box/
├── src/                    # Frontend (React + TypeScript + Tailwind)
│   ├── App.tsx             # Main app component
│   ├── main.tsx            # Entry point
│   ├── index.css           # Global styles with Tailwind
│   ├── components/
│   │   ├── MenuBar.tsx     # File/Help menus
│   │   ├── Toolbar.tsx     # Play, Stop, Save, BPM, steps controls
│   │   ├── PatternTabs.tsx # Pattern tab management
│   │   ├── PatternGrid.tsx # Grid editor with step cells
│   │   └── InstrumentRow.tsx # Instrument row
│   ├── hooks/
│   │   └── useProject.ts   # Project state management
│   └── types/
│       └── index.ts        # Shared TypeScript types
├── rhythm-core/            # Rust core library
│   ├── src/
│   │   ├── lib.rs
│   │   ├── audio.rs        # Audio engine (cpal/rodio)
│   │   ├── models/         # Pattern, Instrument data models
│   │   ├── project.rs      # Project serialization
│   │   └── validation.rs   # Project validation
│   └── Cargo.toml
├── src-tauri/              # Tauri desktop shell
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   └── lib.rs          # Tauri commands (save/load)
│   ├── tauri.conf.json     # Window config, bundle settings
│   └── Cargo.toml
├── .github/workflows/      # CI configuration
├── package.json
├── Cargo.toml              # Workspace root
└── vite.config.ts
```

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+N | New project |
| Ctrl+O | Open project |
| Ctrl+S | Save project |
| Space | Play / Stop |

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Shell | [Tauri v2](https://v2.tauri.app/) |
| Frontend | [React 19](https://react.dev/), [TypeScript](https://www.typescriptlang.org/), [Tailwind CSS v4](https://tailwindcss.com/) |
| Build | [Vite 8](https://vite.dev/) |
| Backend | [Rust](https://www.rust-lang.org/) |
| Audio | [cpal](https://github.com/RustAudio/cpal) + [rodio](https://github.com/RustAudio/rodio) |
| Serialization | [serde](https://serde.rs/) + [serde_json](https://github.com/serde-rs/json) |
| CI | GitHub Actions |

## License

MIT
