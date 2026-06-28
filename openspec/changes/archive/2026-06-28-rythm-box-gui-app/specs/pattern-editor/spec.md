## ADDED Requirements

### Requirement: Grid-based pattern timeline
The system SHALL provide a grid-based timeline editor where each row represents an instrument/sound and each column represents a time step within a measure.

#### Scenario: Create a new pattern
- **WHEN** user clicks "New Pattern"
- **THEN** a blank grid is displayed with default rows (kick, snare, hihat) and 16 columns

#### Scenario: Toggle a step on/off
- **WHEN** user clicks a grid cell
- **THEN** the cell toggles between active (filled) and inactive (empty) state

#### Scenario: Change grid resolution
- **WHEN** user selects a different step count (8, 16, 32)
- **THEN** the grid updates to the selected number of columns

### Requirement: Instrument/sound row management
The system SHALL allow users to add, remove, and rename instrument rows in the pattern editor.

#### Scenario: Add a new instrument row
- **WHEN** user clicks "Add Instrument" and enters a name
- **THEN** a new row is added to the grid with the given name

#### Scenario: Remove an instrument row
- **WHEN** user clicks "Remove" on an existing instrument row
- **THEN** the row is removed from the grid and all its steps are deleted

#### Scenario: Rename an instrument row
- **WHEN** user double-clicks the instrument name
- **THEN** the name becomes editable, and changes are saved on blur

### Requirement: Pattern playback from editor
The system SHALL allow previewing the current pattern directly from the editor.

#### Scenario: Play pattern preview
- **WHEN** user clicks the Play button in the editor toolbar
- **THEN** the pattern loops from the current position with visual step highlighting
