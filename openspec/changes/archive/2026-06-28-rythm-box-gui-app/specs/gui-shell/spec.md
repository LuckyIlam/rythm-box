## ADDED Requirements

### Requirement: Main application window
The system SHALL provide a main application window with a menu bar, toolbar, and workspace area.

#### Scenario: Application launches
- **WHEN** user launches the GUI application
- **THEN** the main window opens with menu bar, toolbar, and an empty workspace

#### Scenario: Window has default size
- **WHEN** application starts for the first time
- **THEN** the window is 1200x800 pixels and centered on screen

### Requirement: Menu bar
The system SHALL provide a menu bar with File, Edit, View, and Help menus.

#### Scenario: File menu items
- **WHEN** user clicks File menu
- **THEN** options New, Open, Save, Save As, Export, and Exit are displayed

#### Scenario: Keyboard shortcuts
- **WHEN** user presses Ctrl+N, Ctrl+O, Ctrl+S
- **THEN** the corresponding File actions are triggered

### Requirement: Toolbar
The system SHALL provide a toolbar with quick-access buttons for common actions.

#### Scenario: Toolbar buttons are functional
- **WHEN** user clicks Play, Stop, Save, or New buttons on the toolbar
- **THEN** the corresponding action is executed

### Requirement: Pattern tab management
The system SHALL allow multiple patterns to be open simultaneously in tabs.

#### Scenario: Open multiple patterns
- **WHEN** user creates or opens additional patterns
- **THEN** each pattern appears in a separate tab in the workspace

#### Scenario: Switch between pattern tabs
- **WHEN** user clicks a different pattern tab
- **THEN** the editor displays that pattern's grid
