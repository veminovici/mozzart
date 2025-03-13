# Mazzart Polyphony Library (mazzart-ply)

A Rust library for musical note manipulation, interval operations, chord construction, and scale generation within the Mozzart music system.

## Overview

This library provides a comprehensive set of tools for working with musical elements using MIDI note representation. It implements standard music theory concepts like notes, intervals, chords, and scales with an ergonomic Rust API.

## Features

- **MIDI Note Representation**: Work with notes using the standard MIDI numbering system (0-127)
- **Interval Operations**: Apply musical intervals to transpose notes
- **Chord Construction**: Generate various chord types (triads, seventh chords, extended chords)
- **Scale Generation**: Create common musical scales (major, minor, harmonic minor, melodic minor)
- **Musical Operations**: Perform operations like octave shifting, transposition, and step-wise progressions

## Core Types

- `Note`: Represents a musical note using MIDI note numbering (0-127)
- `Interval`: Represents a musical interval in semitones
- `Chord<N>`: Represents a chord with N notes of a specific quality
- `Scale<N>`: Represents a musical scale with N notes and a specific quality

## Usage

```rust
use mazzart_ply::constants::*;
use mazzart_ply::Note;

// Create a C major scale
let c_major_scale: Vec<_> = C4.into_notes_from_steps([WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF]).collect();
// Result: [C4, D4, E4, F4, G4, A4, B4, C5]

// Create a C major chord using fixed intervals from the root
let c_major_chord: Vec<_> = C4.into_notes_from_intervals([MAJOR_THIRD, PERFECT_FIFTH]).collect();
// Result: [C4, E4, G4]

// Create a C dominant seventh chord
let c7_chord: Vec<_> = C4.into_notes_from_intervals([MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH]).collect();
// Result: [C4, E4, G4, Bb4]

// Transpose a note up by a perfect fifth
let g4 = C4 + PERFECT_FIFTH;
// Result: G4

// Shift a note up by an octave
let c5 = C4 >> 1;
// Result: C5
```

## License

MIT 