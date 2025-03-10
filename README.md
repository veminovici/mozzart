# Mozzart

A Rust workspace for musical composition and analysis, providing a robust foundation for working with musical concepts in Rust.

## Overview

The workspace contains:
- `mozzart-std`: A library crate providing standard musical functionality
  - MIDI-compatible pitch representation (0-127)
  - Musical intervals and pitch relationships
  - Common musical constants (pitch classes, standard intervals)
  - Utilities for working with sequences of pitches and intervals
- `mozzart-app`: An application that uses the library

## Features

### Core Types
- `Pitch`: Represents musical pitches using MIDI note numbers
- `Interval`: Represents the distance between pitches in semitones

### Musical Constants
- Pitch constants (e.g., `C4`, `A4`) for common reference pitches
- Interval constants (e.g., `MAJOR_THIRD`, `PERFECT_FIFTH`) for standard musical intervals

### Sequence Operations
- Convert between pitch sequences and interval patterns
- Analyze melodic intervals
- Build chord voicings from interval patterns
- Transpose musical patterns

## Example Usage

```rust
use mozzart_std::{C4, E4, G4, MAJOR_THIRD, MINOR_THIRD, into_intervals, into_pitches};

// Analyze intervals in a C major triad
let c_major = [C4, E4, G4];
let intervals = into_intervals(&c_major);
assert_eq!(intervals, vec![MAJOR_THIRD, MINOR_THIRD]);

// Build an E major triad using the same interval pattern
let e_major = into_pitches(E4, &intervals);
```

## Building

To build all workspace members:
```bash
cargo build
```

## Running the Application

To run the application:
```bash
cargo run -p mozzart-app
```

## Running Tests

To run tests for all workspace members:
```bash
cargo test
``` 