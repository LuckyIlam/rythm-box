## Context

Rhythm Box currently operates as a CLI/headless application. Adding a GUI requires selecting an appropriate framework, defining the architecture for desktop GUI + shared core library, and establishing patterns for real-time audio visualization and user interaction.

The existing codebase is unknown; this design assumes a modular core with rhythm patterns, audio engine, and storage that the GUI will consume.

## Goals / Non-Goals

**Goals:**
- Cross-platform desktop GUI (Windows, macOS, Linux)
- Shared core library: rhythm engine, pattern data model, audio playback logic reused by both CLI and GUI
- Real-time visual feedback for beat/rhythm playback
- Intuitive pattern editor with grid-based timeline
- Project save/load using a portable format (JSON/YAML)

**Non-Goals:**
- Mobile or web versions (future consideration)
- MIDI hardware integration
- Cloud sync or multi-user collaboration
- Audio recording or sampling (playback only)

## Decisions

### GUI Framework: Tauri (Rust backend + Web frontend)
- **Rationale**: Tauri provides a small binary, strong cross-platform support, native OS integration, and uses web technologies (HTML/CSS/JS/TS) for the UI. Compared to Electron (heavier memory) or Flutter (different language stack), Tauri pairs well with an existing Rust core.
- **Alternatives considered**: Electron (too heavy), Flutter (Dart adds language overhead), Qt (C++ complexity)

### Frontend UI: React + TypeScript + Tailwind CSS
- **Rationale**: React is widely used, Tailwind enables rapid UI development, TypeScript adds type safety. The web frontend runs in Tauri's webview.
- **Alternatives considered**: Svelte (smaller community), Vue (team preference unknown)

### Audio Engine: cpal + rodio (Rust)
- **Rationale**: cpal provides cross-platform audio output; rodio is built on cpal with a simpler API for playback. If the existing engine uses something else, wrap it behind a shared trait.
- **Alternatives considered**: Web Audio API in frontend (latency concerns), PortAudio (C dependency)

### Project Format: JSON with versioned schema
- **Rationale**: Human-readable, diffable in git, easy to validate, widely supported. YAML alternative is more error-prone with indentation.

## Risks / Trade-offs

- **[Tauri learning curve]** → Mitigate with documented examples and gradual adoption; core logic stays in Rust
- **[Webview inconsistencies across platforms]** → Use standard web APIs and test on all target OS early
- **[Real-time audio latency in webview]** → Keep audio in Rust backend; use Tauri events for timing signals to frontend
- **[Frontend bundle size]** → Use code splitting and lazy loading for editor and playback views
