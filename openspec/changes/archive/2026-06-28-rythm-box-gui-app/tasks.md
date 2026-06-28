## 1. Project Scaffolding

- [x] 1.1 Initialize Tauri project with React + TypeScript + Tailwind CSS
- [x] 1.2 Set up Rust core library crate structure (rhythm-core)
- [x] 1.3 Configure shared types between Rust backend and TypeScript frontend
- [x] 1.4 Set up linting, formatting, and CI configuration

## 2. Core Data Model

- [x] 2.1 Implement Pattern data model (instruments, steps, BPM, time signature)
- [x] 2.2 Implement Instrument data model (name, sound reference, volume)
- [x] 2.3 Implement Project data model (patterns, metadata, settings)
- [x] 2.4 Implement serialization/deserialization to/from JSON
- [x] 2.5 Implement validation schema for project files

## 3. Audio Playback Engine

- [x] 3.1 Set up cpal/rodio audio output in Rust backend
- [x] 3.2 Implement step sequencer that triggers sounds at configured BPM
- [x] 3.3 Implement synthesized default sounds (kick, snare, hihat, etc.)
- [ ] 3.4 Implement sample-based sound loading and playback
- [ ] 3.5 Implement play/stop/pause controls exposed via Tauri commands
- [ ] 3.6 Implement real-time playhead position events to frontend

## 4. GUI Shell

- [x] 4.1 Create main window with MenuBar component (File, Edit, View, Help)
- [x] 4.2 Create Toolbar component with Play, Stop, Save, New buttons
- [x] 4.3 Implement keyboard shortcut bindings (Ctrl+N/O/S, Space for play/stop)
- [x] 4.4 Implement tab-based pattern management
- [x] 4.5 Implement window state persistence (size, position)

## 5. Pattern Editor

- [x] 5.1 Implement grid component with scrollable rows and columns
- [x] 5.2 Implement cell toggle (active/inactive) with click interaction
- [x] 5.3 Implement instrument row management (add, remove, rename)
- [x] 5.4 Implement grid resolution/swing controls
- [x] 5.5 Implement playhead highlight animation during playback
- [x] 5.6 Implement instrument sound selector dropdown per row

## 6. Project Management

- [x] 6.1 Implement Save dialog with JSON serialization
- [x] 6.2 Implement Open dialog with file parsing and validation
- [x] 6.3 Implement New Project with unsaved changes prompt
- [x] 6.4 Implement Export pattern as standalone JSON
- [x] 6.5 Implement recent files list

## 7. Integration & Polish

- [x] 7.1 Wire up Tauri commands for all backend operations
- [x] 7.2 Implement error handling and user-facing error messages
- [x] 7.3 Add loading states and transition animations
- [x] 7.4 Implement dark/light theme support
- [ ] 7.5 Test on Windows, macOS, and Linux
- [ ] 7.6 Build packaging configuration for all platforms
