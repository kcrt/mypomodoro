# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

MyPomodoro is a Rust GUI application built with egui/eframe that implements a Pomodoro timer with visual progress indicators and sound notifications. The application runs as a native desktop app with cross-platform support for Windows and macOS.

## Development Commands

### Building and Running
- **Development run**: `cargo run`
- **Release build**: `cargo build --release`
- **Tests**: `cargo test`
- **Platform-specific builds**:
  - Windows: `./build.bat` or `./build.sh --windows`
  - macOS: `./build.sh --macos`
  - All platforms: `./build.sh --all`

### Dependencies
- Requires `cargo-bundle` for creating platform bundles: `cargo install cargo-bundle`
- Uses rodio for cross-platform audio playback
- Built with egui/eframe for the GUI

## Architecture

### Core Components
- **MyApp struct** (`src/lib.rs`): Main application state containing timer logic, UI rendering, and state management
- **Timer States**: `Running`, `Paused`, `Stopped`
- **Timer Phases**: `Pomodoro` (work), `ShortBreak`, `LongBreak`
- **Time calculation**: Handles pause duration tracking and remaining time calculations

### Key Features
- **Dynamic window icon**: Updates to reflect timer progress as a doughnut chart
- **Audio notifications**: Plays bell.mp3 when phases complete using embedded resources
- **Visual timer**: Doughnut-style progress indicator with phase-specific colors
- **Configurable settings**: Work/break durations, cycle counts, sound toggle

### File Structure
- `src/main.rs`: Entry point, window setup with egui
- `src/lib.rs`: Core application logic and UI rendering
- `tests/`: Unit tests for timer states, phases, and time calculations
- `resources/bell.mp3`: Embedded sound notification
- `icons/`: Application icons in multiple formats

### Testing
Tests are organized by functionality:
- `timer_state_tests.rs`: State transitions and basic functionality
- `timer_phase_tests.rs`: Phase progression and cycle logic  
- `time_calculation_tests.rs`: Time tracking and calculations

The application follows Rust best practices with proper error handling, state management, and separation of concerns between timer logic and UI rendering.