## ADDED Requirements

### Requirement: Save project
The system SHALL save the current project (all patterns, instruments, and settings) to a file in JSON format.

#### Scenario: Save project to file
- **WHEN** user clicks File > Save or presses Ctrl+S
- **THEN** the project is serialized and written to the chosen file path

#### Scenario: Save with unsaved changes warning
- **WHEN** user closes the app with unsaved changes
- **THEN** a dialog prompts to save, discard, or cancel

### Requirement: Load project
The system SHALL load a previously saved project from a file.

#### Scenario: Load project from file
- **WHEN** user clicks File > Open and selects a project file
- **THEN** all patterns, instruments, and settings are restored to the editor

#### Scenario: Load with invalid file
- **WHEN** user opens a file that is not a valid project file
- **THEN** an error message is displayed and no data is loaded

### Requirement: New project
The system SHALL allow creating a new empty project.

#### Scenario: Create new project
- **WHEN** user clicks File > New
- **THEN** the editor resets to a blank default project (with prompt to save current if unsaved)

### Requirement: Export pattern
The system SHALL allow exporting individual patterns to a shareable format.

#### Scenario: Export pattern as JSON
- **WHEN** user right-clicks a pattern and selects Export
- **THEN** a JSON file containing only that pattern's data is saved
