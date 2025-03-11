# Mozzart

A Rust workspace for musical composition and analysis, providing a robust foundation for working with musical concepts in Rust.

## Overview

The workspace contains:
- `mozzart-std`: A library crate providing standard musical functionality
  - MIDI-compatible pitch representation (0-127)
  - Musical intervals and pitch relationships
  - Comprehensive scale system (major, minor, harmonic, melodic)
  - Common musical constants (pitch classes, standard intervals, scales)
  - Utilities for working with sequences of pitches and intervals
- `mozzart-app`: An application that uses the library

## Features

### Core Types
- `Pitch`: Represents musical pitches using MIDI note numbers
- `Interval`: Represents the distance between pitches in semitones
- `Scale`: Represents musical scales as sequences of pitches

### Musical Constants
- Pitch constants (e.g., `C4`, `A4`) for all notes across the MIDI range (octaves 0-8)
- Interval constants (e.g., `MAJOR_THIRD`, `PERFECT_FIFTH`) for standard musical intervals
- Scale constants for major, minor, harmonic, and melodic scales in all keys and octaves

### Scale System
- Major scales with the classic diatonic pattern (TONE, TONE, SEMITONE, TONE, TONE, TONE, SEMITONE)
- Natural minor scales (Aeolian mode) with their characteristic melancholic sound
- Harmonic minor scales with the raised 7th degree
- Melodic minor scales with raised 6th and 7th degrees (ascending form)
- All scales available in all 12 keys across the entire MIDI range (octaves 0-8)

### Octave Shifting
- Shift pitches up an octave with `<<` (e.g., `C4 << 1` becomes `C5`)
- Shift pitches down an octave with `>>` (e.g., `C4 >> 1` becomes `C3`)
- Support for multi-octave shifts (e.g., `C4 << 2` becomes `C6`)
- Scale transposition across octaves

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

// Working with scales
let c_major_scale = C4.major_scale();
let a_minor_scale = A4.minor_scale();
let d_harmonic_scale = D4.harmonic_scale();
let f_melodic_scale = F4.melodic_scale();

// Octave shifting
let c5 = C4 << 1;  // Shift C4 up one octave to C5
let c3 = C4 >> 1;  // Shift C4 down one octave to C3

// Scale constants
use mozzart_std::{C4_MAJOR_SCALE, C5_MAJOR_SCALE};
assert_eq!(C4_MAJOR_SCALE.pitches()[0], C4);
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