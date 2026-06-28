## ADDED Requirements

### Requirement: Audio playback engine
The system SHALL provide real-time audio playback of rhythm patterns using synthesized or sample-based sounds for each instrument.

#### Scenario: Play a pattern with audio
- **WHEN** user triggers play on a completed pattern
- **THEN** audio plays each active step in sequence at the configured BPM

#### Scenario: Stop playback
- **WHEN** user clicks the Stop button
- **THEN** audio playback stops and the playhead resets to the beginning

#### Scenario: Change BPM during playback
- **WHEN** user adjusts the BPM slider/input while playing
- **THEN** playback tempo adjusts in real-time without restarting

### Requirement: Visual beat indicator
The system SHALL highlight the current playhead position on the pattern grid during playback.

#### Scenario: Playhead advances through pattern
- **WHEN** pattern plays
- **THEN** the current step column is highlighted, advancing each beat

#### Scenario: Step highlight stops on pause
- **WHEN** user pauses playback
- **THEN** the playhead highlight freezes at the current position

### Requirement: Sound configuration per instrument
The system SHALL allow configuring which sound/sample is assigned to each instrument row.

#### Scenario: Assign sound to instrument
- **WHEN** user selects a sound from a dropdown on an instrument row
- **THEN** that sound is used when the instrument's active steps play
