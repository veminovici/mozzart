//! Melodic minor scale constants and collections
//!
//! This module provides predefined constants for all melodic minor scales across the MIDI note range.
//! Each scale is represented as an array of 8 notes (one octave) starting from a specific root pitch.
//!
//! # Melodic Minor Scale Structure
//! A melodic minor scale has different ascending and descending forms:
//!
//! Ascending pattern:
//! Whole-Half-Whole-Whole-Whole-Whole-Half, creating the following steps:
//! - Between 1-2: whole step (2 semitones)
//! - Between 2-3: half step (1 semitone)
//! - Between 3-4: whole step (2 semitones)
//! - Between 4-5: whole step (2 semitones)
//! - Between 5-6: whole step (2 semitones)
//! - Between 6-7: whole step (2 semitones)
//! - Between 7-8: half step (1 semitone)
//!
//! Descending form typically uses the natural minor scale pattern.
//!
//! # Musical Characteristics
//! The melodic minor scale is characterized by:
//! - Raised 6th and 7th degrees in ascending form
//! - Natural minor form when descending
//! - Creates a smoother melodic line than harmonic minor
//! - Common in jazz and classical music
//! - Often used in melodic passages and voice leading
//!
//! # Scale Organization
//! Scales are organized in several ways:
//! 1. Octave-independent scales (e.g., `C_MELODIC_SCALE`) - generic patterns
//! 2. Octave-specific scales (e.g., `C4_MELODIC_SCALE`) - starting from specific octaves
//! 3. Collections in `MELODIC_SCALES` - HashMap mapping root pitches to their melodic minor scales
//!
//! # Examples
//! ```
//! use mozzart_std::{A4_MELODIC_SCALE, MELODIC_SCALES};
//! use mozzart_std::A4;
//!
//! // Use a predefined scale
//! let a_melodic = A4_MELODIC_SCALE;
//!
//! // Look up a scale by root note
//! let scale = MELODIC_SCALES.get(&A4).unwrap();
//! assert_eq!(*scale, A4_MELODIC_SCALE);
//! ```

use crate::constants::*;
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub const MELODIC_SCALE_STEPS: [Interval; 8] =
    [UNISON, TONE, SEMITONE, TONE, TONE, TONE, TONE, SEMITONE];

// Octave-independent melodic minor scales (pitch class only)
/// C melodic minor scale (C-D-E♭-F-G-A-B-C)
pub const C_MELODIC_SCALE: Scale<8> = Scale::melodic([C, D, DSHARP, F, G, A, B, C0]);
pub const CSHARP_MELODIC_SCALE: Scale<8> =
    Scale::melodic([CSHARP, DSHARP, E, FSHARP, GSHARP, ASHARP, C0, CSHARP0]);
pub const D_MELODIC_SCALE: Scale<8> = Scale::melodic([D, E, F, G, A, B, CSHARP0, D0]);
pub const DSHARP_MELODIC_SCALE: Scale<8> =
    Scale::melodic([DSHARP, F, FSHARP, GSHARP, ASHARP, C0, D0, DSHARP0]);
pub const E_MELODIC_SCALE: Scale<8> = Scale::melodic([E, FSHARP, G, A, B, CSHARP0, DSHARP0, E0]);
pub const F_MELODIC_SCALE: Scale<8> = Scale::melodic([F, G, GSHARP, ASHARP, C0, D0, E0, F0]);
pub const FSHARP_MELODIC_SCALE: Scale<8> =
    Scale::melodic([FSHARP, GSHARP, A, B, CSHARP0, DSHARP0, F0, FSHARP0]);
pub const G_MELODIC_SCALE: Scale<8> = Scale::melodic([G, A, ASHARP, C0, D0, E0, FSHARP0, G0]);
pub const GSHARP_MELODIC_SCALE: Scale<8> =
    Scale::melodic([GSHARP, ASHARP, B, CSHARP0, DSHARP0, F0, G0, GSHARP0]);
pub const A_MELODIC_SCALE: Scale<8> = Scale::melodic([A, B, C0, D0, E0, FSHARP0, GSHARP0, A0]);
pub const ASHARP_MELODIC_SCALE: Scale<8> =
    Scale::melodic([ASHARP, C0, CSHARP0, DSHARP0, F0, G0, A0, ASHARP0]);
pub const B_MELODIC_SCALE: Scale<8> =
    Scale::melodic([B, CSHARP0, D0, E0, FSHARP0, GSHARP0, ASHARP0, B0]);

// Octave 0 melodic minor scales
// These scales start from notes in MIDI octave 0 (notes 12-23)
// The lowest complete melodic minor scales in the MIDI range

/// C0 melodic minor scale: C0-D0-D#0-F0-G0-A0-B0-C1
/// Lowest C melodic minor scale in standard MIDI (C0 = MIDI note 12)
/// Notes span from C0 (16.35 Hz) to C1 (32.70 Hz)
pub const C0_MELODIC_SCALE: Scale<8> = Scale::melodic([C0, D0, DSHARP0, F0, G0, A0, B0, C1]);

/// C#0 melodic minor scale: C#0-D#0-E0-F#0-G#0-A#0-C1-C#1
/// Notes span from C#0 (MIDI note 13, ~17.32 Hz) to C#1 (MIDI note 25, ~34.65 Hz)
pub const CSHARP0_MELODIC_SCALE: Scale<8> =
    Scale::melodic([CSHARP0, DSHARP0, E0, FSHARP0, GSHARP0, ASHARP0, C1, CSHARP1]);

/// D0 melodic minor scale: D0-E0-F0-G0-A0-B0-C#1-D1
/// Notes span from D0 (MIDI note 14, ~18.35 Hz) to D1 (MIDI note 26, ~36.71 Hz)
pub const D0_MELODIC_SCALE: Scale<8> = Scale::melodic([D0, E0, F0, G0, A0, B0, CSHARP1, D1]);

/// D#0 melodic minor scale: D#0-F0-F#0-G#0-A#0-C1-D1-D#1
/// Notes span from D#0 (MIDI note 15, ~19.45 Hz) to D#1 (MIDI note 27, ~38.89 Hz)
pub const DSHARP0_MELODIC_SCALE: Scale<8> =
    Scale::melodic([DSHARP0, F0, FSHARP0, GSHARP0, ASHARP0, C1, D1, DSHARP1]);

/// E0 melodic minor scale: E0-F#0-G0-A0-B0-C#1-D#1-E1
/// Notes span from E0 (MIDI note 16, ~20.60 Hz) to E1 (MIDI note 28, ~41.20 Hz)
pub const E0_MELODIC_SCALE: Scale<8> =
    Scale::melodic([E0, FSHARP0, G0, A0, B0, CSHARP1, DSHARP1, E1]);

/// F0 melodic minor scale: F0-G0-G#0-A#0-C1-D1-E1-F1
/// Notes span from F0 (MIDI note 17, ~21.83 Hz) to F1 (MIDI note 29, ~43.65 Hz)
pub const F0_MELODIC_SCALE: Scale<8> = Scale::melodic([F0, G0, GSHARP0, ASHARP0, C1, D1, E1, F1]);

/// F#0 melodic minor scale: F#0-G#0-A0-B0-C#1-D#1-F1-F#1
/// Notes span from F#0 (MIDI note 18, ~23.12 Hz) to F#1 (MIDI note 30, ~46.25 Hz)
pub const FSHARP0_MELODIC_SCALE: Scale<8> =
    Scale::melodic([FSHARP0, GSHARP0, A0, B0, CSHARP1, DSHARP1, F1, FSHARP1]);

/// G0 melodic minor scale: G0-A0-A#0-C1-D1-E1-F#1-G1
/// Notes span from G0 (MIDI note 19, ~24.50 Hz) to G1 (MIDI note 31, ~49.00 Hz)
pub const G0_MELODIC_SCALE: Scale<8> = Scale::melodic([G0, A0, ASHARP0, C1, D1, E1, FSHARP1, G1]);

/// G#0 melodic minor scale: G#0-A#0-B0-C#1-D#1-F1-G1-G#1
/// Notes span from G#0 (MIDI note 20, ~25.96 Hz) to G#1 (MIDI note 32, ~51.91 Hz)
pub const GSHARP0_MELODIC_SCALE: Scale<8> =
    Scale::melodic([GSHARP0, ASHARP0, B0, CSHARP1, DSHARP1, F1, G1, GSHARP1]);

/// A0 melodic minor scale: A0-B0-C1-D1-E1-F#1-G#1-A1
/// Notes span from A0 (MIDI note 21, 27.50 Hz) to A1 (MIDI note 33, 55.00 Hz)
/// A0 is the lowest note on a standard piano
pub const A0_MELODIC_SCALE: Scale<8> = Scale::melodic([A0, B0, C1, D1, E1, FSHARP1, GSHARP1, A1]);

/// A#0 melodic minor scale: A#0-C1-C#1-D#1-F1-G1-A1-A#1
/// Notes span from A#0 (MIDI note 22, ~29.14 Hz) to A#1 (MIDI note 34, ~58.27 Hz)
pub const ASHARP0_MELODIC_SCALE: Scale<8> =
    Scale::melodic([ASHARP0, C1, CSHARP1, DSHARP1, F1, G1, A1, ASHARP1]);

/// B0 melodic minor scale: B0-C#1-D1-E1-F#1-G#1-A#1-B1
/// Notes span from B0 (MIDI note 23, ~30.87 Hz) to B1 (MIDI note 35, ~61.74 Hz)
pub const B0_MELODIC_SCALE: Scale<8> =
    Scale::melodic([B0, CSHARP1, D1, E1, FSHARP1, GSHARP1, ASHARP1, B1]);

// Octave 1 melodic minor scales
// These scales start from notes in MIDI octave 1 (notes 24-35)
// These scales are in the lower bass range, often used for bass instruments

/// C1 melodic minor scale: C1-D1-D#1-F1-G1-A1-B1-C2
/// Notes span from C1 (MIDI note 24, 32.70 Hz) to C2 (MIDI note 36, 65.41 Hz)
/// In the low bass range, useful for bass instruments and low piano parts
pub const C1_MELODIC_SCALE: Scale<8> = Scale::melodic([C1, D1, DSHARP1, F1, G1, A1, B1, C2]);

/// C#1 melodic minor scale: C#1-D#1-E1-F#1-G#1-A#1-C2-C#2
/// Notes span from C#1 (MIDI note 25, ~34.65 Hz) to C#2 (MIDI note 37, ~69.30 Hz)
/// Low bass range with a dark, mysterious quality
pub const CSHARP1_MELODIC_SCALE: Scale<8> =
    Scale::melodic([CSHARP1, DSHARP1, E1, FSHARP1, GSHARP1, ASHARP1, C2, CSHARP2]);

/// D1 melodic minor scale: D1-E1-F1-G1-A1-B1-C#2-D2
/// Notes span from D1 (MIDI note 26, ~36.71 Hz) to D2 (MIDI note 38, ~73.42 Hz)
/// Low bass range, provides a rich foundation for melodic minor compositions
pub const D1_MELODIC_SCALE: Scale<8> = Scale::melodic([D1, E1, F1, G1, A1, B1, CSHARP2, D2]);

/// D#1 melodic minor scale: D#1-F1-F#1-G#1-A#1-C2-D2-D#2
/// Notes span from D#1 (MIDI note 27, ~38.89 Hz) to D#2 (MIDI note 39, ~77.78 Hz)
/// Low bass range with a somber, dramatic character
pub const DSHARP1_MELODIC_SCALE: Scale<8> =
    Scale::melodic([DSHARP1, F1, FSHARP1, GSHARP1, ASHARP1, C2, D2, DSHARP2]);

/// E1 melodic minor scale: E1-F#1-G1-A1-B1-C#2-D#2-E2
/// Notes span from E1 (MIDI note 28, ~41.20 Hz) to E2 (MIDI note 40, ~82.41 Hz)
/// Low E is the lowest note on a standard bass guitar (E1)
pub const E1_MELODIC_SCALE: Scale<8> =
    Scale::melodic([E1, FSHARP1, G1, A1, B1, CSHARP2, DSHARP2, E2]);

/// F1 melodic minor scale: F1-G1-G#1-A#1-C2-D2-E2-F2
/// Notes span from F1 (MIDI note 29, ~43.65 Hz) to F2 (MIDI note 41, ~87.31 Hz)
/// Low bass range with a rich, resonant quality
pub const F1_MELODIC_SCALE: Scale<8> = Scale::melodic([F1, G1, GSHARP1, ASHARP1, C2, D2, E2, F2]);

/// F#1 melodic minor scale: F#1-G#1-A1-B1-C#2-D#2-F2-F#2
/// Notes span from F#1 (MIDI note 30, ~46.25 Hz) to F#2 (MIDI note 42, ~92.50 Hz)
/// Low bass range with an exotic, fluid character
pub const FSHARP1_MELODIC_SCALE: Scale<8> =
    Scale::melodic([FSHARP1, GSHARP1, A1, B1, CSHARP2, DSHARP2, F2, FSHARP2]);

/// G1 melodic minor scale: G1-A1-A#1-C2-D2-E2-F#2-G2
/// Notes span from G1 (MIDI note 31, ~49.00 Hz) to G2 (MIDI note 43, ~98.00 Hz)
/// Low bass range, provides a strong foundation for melodic minor progressions
pub const G1_MELODIC_SCALE: Scale<8> = Scale::melodic([G1, A1, ASHARP1, C2, D2, E2, FSHARP2, G2]);

/// G#1 melodic minor scale: G#1-A#1-B1-C#2-D#2-F2-G2-G#2
/// Notes span from G#1 (MIDI note 32, ~51.91 Hz) to G#2 (MIDI note 44, ~103.83 Hz)
/// Low bass range with a dark, mysterious quality
pub const GSHARP1_MELODIC_SCALE: Scale<8> =
    Scale::melodic([GSHARP1, ASHARP1, B1, CSHARP2, DSHARP2, F2, G2, GSHARP2]);

/// A1 melodic minor scale: A1-B1-C2-D2-E2-F#2-G#2-A2
/// Notes span from A1 (MIDI note 33, 55.00 Hz) to A2 (MIDI note 45, 110.00 Hz)
/// A1 is a common low note for cellos and bass instruments
pub const A1_MELODIC_SCALE: Scale<8> = Scale::melodic([A1, B1, C2, D2, E2, FSHARP2, GSHARP2, A2]);

/// A#1 melodic minor scale: A#1-C2-C#2-D#2-F2-G2-A2-A#2
/// Notes span from A#1 (MIDI note 34, ~58.27 Hz) to A#2 (MIDI note 46, ~116.54 Hz)
/// Low bass range with a rich, lyrical character
pub const ASHARP1_MELODIC_SCALE: Scale<8> =
    Scale::melodic([ASHARP1, C2, CSHARP2, DSHARP2, F2, G2, A2, ASHARP2]);

/// B1 melodic minor scale: B1-C#2-D2-E2-F#2-G#2-A#2-B2
/// Notes span from B1 (MIDI note 35, ~61.74 Hz) to B2 (MIDI note 47, ~123.47 Hz)
/// Transitional range between bass and tenor ranges
pub const B1_MELODIC_SCALE: Scale<8> =
    Scale::melodic([B1, CSHARP2, D2, E2, FSHARP2, GSHARP2, ASHARP2, B2]);

// Octave 2 melodic minor scales
// These scales start from notes in MIDI octave 2 (notes 36-47)
// These scales are in the bass range, commonly used in bass parts and lower register instruments

/// C2 melodic minor scale: C2-D2-D#2-F2-G2-A2-B2-C3
/// Notes span from C2 (MIDI note 36, 65.41 Hz) to C3 (MIDI note 48, 130.81 Hz)
/// Bass range, often used for bass lines and low piano parts
pub const C2_MELODIC_SCALE: Scale<8> = Scale::melodic([C2, D2, DSHARP2, F2, G2, A2, B2, C3]);

/// C#2 melodic minor scale: C#2-D#2-E2-F#2-G#2-A#2-C3-C#3
/// Notes span from C#2 (MIDI note 37, ~69.30 Hz) to C#3 (MIDI note 49, ~138.59 Hz)
/// Bass range with a rich, resonant quality
pub const CSHARP2_MELODIC_SCALE: Scale<8> =
    Scale::melodic([CSHARP2, DSHARP2, E2, FSHARP2, GSHARP2, ASHARP2, C3, CSHARP3]);

/// D2 melodic minor scale: D2-E2-F2-G2-A2-B2-C#3-D3
/// Notes span from D2 (MIDI note 38, ~73.42 Hz) to D3 (MIDI note 50, ~146.83 Hz)
/// Bass range, provides a solid foundation for melodic phrases
pub const D2_MELODIC_SCALE: Scale<8> = Scale::melodic([D2, E2, F2, G2, A2, B2, CSHARP3, D3]);

/// D#2 melodic minor scale: D#2-F2-F#2-G#2-A#2-C3-D3-D#3
/// Notes span from D#2 (MIDI note 39, ~77.78 Hz) to D#3 (MIDI note 51, ~155.56 Hz)
/// Bass range with a dark, somber character
pub const DSHARP2_MELODIC_SCALE: Scale<8> =
    Scale::melodic([DSHARP2, F2, FSHARP2, GSHARP2, ASHARP2, C3, D3, DSHARP3]);

/// E2 melodic minor scale: E2-F#2-G2-A2-B2-C#3-D#3-E3
/// Notes span from E2 (MIDI note 40, ~82.41 Hz) to E3 (MIDI note 52, ~164.81 Hz)
/// Bass E is a standard bass guitar open string
pub const E2_MELODIC_SCALE: Scale<8> =
    Scale::melodic([E2, FSHARP2, G2, A2, B2, CSHARP3, DSHARP3, E3]);

/// F2 melodic minor scale: F2-G2-G#2-A#2-C3-D3-E3-F3
/// Notes span from F2 (MIDI note 41, ~87.31 Hz) to F3 (MIDI note 53, ~174.61 Hz)
/// Bass range with a warm, resonant quality
pub const F2_MELODIC_SCALE: Scale<8> = Scale::melodic([F2, G2, GSHARP2, ASHARP2, C3, D3, E3, F3]);

/// F#2 melodic minor scale: F#2-G#2-A2-B2-C#3-D#3-F3-F#3
/// Notes span from F#2 (MIDI note 42, ~92.50 Hz) to F#3 (MIDI note 54, ~185.00 Hz)
/// Bass range with a rich, colorful character
pub const FSHARP2_MELODIC_SCALE: Scale<8> =
    Scale::melodic([FSHARP2, GSHARP2, A2, B2, CSHARP3, DSHARP3, F3, FSHARP3]);

/// G2 melodic minor scale: G2-A2-A#2-C3-D3-E3-F#3-G3
/// Notes span from G2 (MIDI note 43, ~98.00 Hz) to G3 (MIDI note 55, ~196.00 Hz)
/// Bass range, provides a solid foundation for melodic minor progressions
pub const G2_MELODIC_SCALE: Scale<8> = Scale::melodic([G2, A2, ASHARP2, C3, D3, E3, FSHARP3, G3]);

/// G#2 melodic minor scale: G#2-A#2-B2-C#3-D#3-F3-G3-G#3
/// Notes span from G#2 (MIDI note 44, ~103.83 Hz) to G#3 (MIDI note 56, ~207.65 Hz)
/// Bass range with a dark, mysterious quality
pub const GSHARP2_MELODIC_SCALE: Scale<8> =
    Scale::melodic([GSHARP2, ASHARP2, B2, CSHARP3, DSHARP3, F3, G3, GSHARP3]);

/// A2 melodic minor scale: A2-B2-C3-D3-E3-F#3-G#3-A3
/// Notes span from A2 (MIDI note 45, 110.00 Hz) to A3 (MIDI note 57, 220.00 Hz)
/// Bass A is a standard bass guitar and cello open string
pub const A2_MELODIC_SCALE: Scale<8> = Scale::melodic([A2, B2, C3, D3, E3, FSHARP3, GSHARP3, A3]);

/// A#2 melodic minor scale: A#2-C3-C#3-D#3-F3-G3-A3-A#3
/// Notes span from A#2 (MIDI note 46, ~116.54 Hz) to A#3 (MIDI note 58, ~233.08 Hz)
/// Bass range with a rich, lyrical character
pub const ASHARP2_MELODIC_SCALE: Scale<8> =
    Scale::melodic([ASHARP2, C3, CSHARP3, DSHARP3, F3, G3, A3, ASHARP3]);

/// B2 melodic minor scale: B2-C#3-D3-E3-F#3-G#3-A#3-B3
/// Notes span from B2 (MIDI note 47, ~123.47 Hz) to B3 (MIDI note 59, ~246.94 Hz)
/// Transitional range between bass and tenor ranges
pub const B2_MELODIC_SCALE: Scale<8> =
    Scale::melodic([B2, CSHARP3, D3, E3, FSHARP3, GSHARP3, ASHARP3, B3]);

// Octave 3 melodic minor scales
// These scales start from notes in MIDI octave 3 (notes 48-59)
// These scales are in the tenor/alto range, commonly used for mid-range instruments and vocals

/// C3 melodic minor scale: C3-D3-D#3-F3-G3-A3-B3-C4
/// Notes span from C3 (MIDI note 48, 130.81 Hz) to C4 (MIDI note 60, 261.63 Hz)
/// C3 is in the tenor range, often used for tenor vocal parts and mid-range instruments
pub const C3_MELODIC_SCALE: Scale<8> = Scale::melodic([C3, D3, DSHARP3, F3, G3, A3, B3, C4]);

/// C#3 melodic minor scale: C#3-D#3-E3-F#3-G#3-A#3-C4-C#4
/// Notes span from C#3 (MIDI note 49, ~138.59 Hz) to C#4 (MIDI note 61, ~277.18 Hz)
/// Tenor range with a rich, expressive character
pub const CSHARP3_MELODIC_SCALE: Scale<8> =
    Scale::melodic([CSHARP3, DSHARP3, E3, FSHARP3, GSHARP3, ASHARP3, C4, CSHARP4]);

/// D3 melodic minor scale: D3-E3-F3-G3-A3-B3-C#4-D4
/// Notes span from D3 (MIDI note 50, ~146.83 Hz) to D4 (MIDI note 62, ~293.66 Hz)
/// Tenor range, commonly used in vocal and instrumental music
pub const D3_MELODIC_SCALE: Scale<8> = Scale::melodic([D3, E3, F3, G3, A3, B3, CSHARP4, D4]);

/// D#3 melodic minor scale: D#3-F3-F#3-G#3-A#3-C4-D4-D#4
/// Notes span from D#3 (MIDI note 51, ~155.56 Hz) to D#4 (MIDI note 63, ~311.13 Hz)
/// Tenor range with a rich, dramatic character
pub const DSHARP3_MELODIC_SCALE: Scale<8> =
    Scale::melodic([DSHARP3, F3, FSHARP3, GSHARP3, ASHARP3, C4, D4, DSHARP4]);

/// E3 melodic minor scale: E3-F#3-G3-A3-B3-C#4-D#4-E4
/// Notes span from E3 (MIDI note 52, ~164.81 Hz) to E4 (MIDI note 64, ~329.63 Hz)
/// E3 is a guitar open string, this scale lies in a comfortable vocal range
pub const E3_MELODIC_SCALE: Scale<8> =
    Scale::melodic([E3, FSHARP3, G3, A3, B3, CSHARP4, DSHARP4, E4]);

/// F3 melodic minor scale: F3-G3-G#3-A#3-C4-D4-E4-F4
/// Notes span from F3 (MIDI note 53, ~174.61 Hz) to F4 (MIDI note 65, ~349.23 Hz)
/// Tenor/alto transitional range, versatile for many instruments
pub const F3_MELODIC_SCALE: Scale<8> = Scale::melodic([F3, G3, GSHARP3, ASHARP3, C4, D4, E4, F4]);

/// F#3 melodic minor scale: F#3-G#3-A3-B3-C#4-D#4-F4-F#4
/// Notes span from F#3 (MIDI note 54, ~185.00 Hz) to F#4 (MIDI note 66, ~369.99 Hz)
/// Alto range with a colorful, expressive character
pub const FSHARP3_MELODIC_SCALE: Scale<8> =
    Scale::melodic([FSHARP3, GSHARP3, A3, B3, CSHARP4, DSHARP4, F4, FSHARP4]);

/// G3 melodic minor scale: G3-A3-A#3-C4-D4-E4-F#4-G4
/// Notes span from G3 (MIDI note 55, ~196.00 Hz) to G4 (MIDI note 67, ~392.00 Hz)
/// G3 is a guitar open string, this scale spans the alto range
pub const G3_MELODIC_SCALE: Scale<8> = Scale::melodic([G3, A3, ASHARP3, C4, D4, E4, FSHARP4, G4]);

/// G#3 melodic minor scale: G#3-A#3-B3-C#4-D#4-F4-G4-G#4
/// Notes span from G#3 (MIDI note 56, ~207.65 Hz) to G#4 (MIDI note 68, ~415.30 Hz)
/// Alto range with a dark, exotic character
pub const GSHARP3_MELODIC_SCALE: Scale<8> =
    Scale::melodic([GSHARP3, ASHARP3, B3, CSHARP4, DSHARP4, F4, G4, GSHARP4]);

/// A3 melodic minor scale: A3-B3-C4-D4-E4-F#4-G#4-A4
/// Notes span from A3 (MIDI note 57, 220.00 Hz) to A4 (MIDI note 69, 440.00 Hz)
/// A3 is a guitar open string, this scale crosses through A4 (concert pitch)
pub const A3_MELODIC_SCALE: Scale<8> = Scale::melodic([A3, B3, C4, D4, E4, FSHARP4, GSHARP4, A4]);

/// A#3 melodic minor scale: A#3-C4-C#4-D#4-F4-G4-A4-A#4
/// Notes span from A#3 (MIDI note 58, ~233.08 Hz) to A#4 (MIDI note 70, ~466.16 Hz)
/// Alto range with a rich, lyrical quality
pub const ASHARP3_MELODIC_SCALE: Scale<8> =
    Scale::melodic([ASHARP3, C4, CSHARP4, DSHARP4, F4, G4, A4, ASHARP4]);

/// B3 melodic minor scale: B3-C#4-D4-E4-F#4-G#4-A#4-B4
/// Notes span from B3 (MIDI note 59, ~246.94 Hz) to B4 (MIDI note 71, ~493.88 Hz)
/// Alto/soprano transitional range with a bright, expressive character
pub const B3_MELODIC_SCALE: Scale<8> =
    Scale::melodic([B3, CSHARP4, D4, E4, FSHARP4, GSHARP4, ASHARP4, B4]);

// Octave 4 melodic minor scales
// These scales start from notes in MIDI octave 4 (notes 60-71)
// These scales are in the soprano/treble range, commonly used for lead melodies and high vocal parts

/// C4 melodic minor scale: C4-D4-D#4-F4-G4-A4-B4-C5
/// Notes span from C4 (MIDI note 60, 261.63 Hz) to C5 (MIDI note 72, 523.25 Hz)
/// C4 is middle C, central to most keyboard instruments and music notation
pub const C4_MELODIC_SCALE: Scale<8> = Scale::melodic([C4, D4, DSHARP4, F4, G4, A4, B4, C5]);

/// C#4 melodic minor scale: C#4-D#4-E4-F#4-G#4-A#4-C5-C#5
/// Notes span from C#4 (MIDI note 61, ~277.18 Hz) to C#5 (MIDI note 73, ~554.37 Hz)
/// Upper treble range with a bright, crystalline quality
pub const CSHARP4_MELODIC_SCALE: Scale<8> =
    Scale::melodic([CSHARP4, DSHARP4, E4, FSHARP4, GSHARP4, ASHARP4, C5, CSHARP5]);

/// D4 melodic minor scale: D4-E4-F4-G4-A4-B4-C#5-D5
/// Notes span from D4 (MIDI note 62, ~293.66 Hz) to D5 (MIDI note 74, ~587.33 Hz)
/// Treble range, well-suited for soprano vocals and lead melody instruments
pub const D4_MELODIC_SCALE: Scale<8> = Scale::melodic([D4, E4, F4, G4, A4, B4, CSHARP5, D5]);

/// D#4 melodic minor scale: D#4-F4-F#4-G#4-A#4-C5-D5-D#5
/// Notes span from D#4 (MIDI note 63, ~311.13 Hz) to D#5 (MIDI note 75, ~622.25 Hz)
/// Upper treble range with a brilliant, expressive character
pub const DSHARP4_MELODIC_SCALE: Scale<8> =
    Scale::melodic([DSHARP4, F4, FSHARP4, GSHARP4, ASHARP4, C5, D5, DSHARP5]);

/// E4 melodic minor scale: E4-F#4-G4-A4-B4-C#5-D#5-E5
/// Notes span from E4 (MIDI note 64, ~329.63 Hz) to E5 (MIDI note 76, ~659.26 Hz)
/// E4 is the highest string on a guitar, this scale spans the high soprano range
pub const E4_MELODIC_SCALE: Scale<8> =
    Scale::melodic([E4, FSHARP4, G4, A4, B4, CSHARP5, DSHARP5, E5]);

/// F4 melodic minor scale: F4-G4-G#4-A#4-C5-D5-E5-F5
/// Notes span from F4 (MIDI note 65, ~349.23 Hz) to F5 (MIDI note 77, ~698.46 Hz)
/// High treble range, excellent for flute and high woodwind melodies
pub const F4_MELODIC_SCALE: Scale<8> = Scale::melodic([F4, G4, GSHARP4, ASHARP4, C5, D5, E5, F5]);

/// F#4 melodic minor scale: F#4-G#4-A4-B4-C#5-D#5-F5-F#5
/// Notes span from F#4 (MIDI note 66, ~369.99 Hz) to F#5 (MIDI note 78, ~739.99 Hz)
/// High treble range with a bright, shimmering quality
pub const FSHARP4_MELODIC_SCALE: Scale<8> =
    Scale::melodic([FSHARP4, GSHARP4, A4, B4, CSHARP5, DSHARP5, F5, FSHARP5]);

/// G4 melodic minor scale: G4-A4-A#4-C5-D5-E5-F#5-G5
/// Notes span from G4 (MIDI note 67, ~392.00 Hz) to G5 (MIDI note 79, ~783.99 Hz)
/// High range well-suited for soprano voices and high-register instruments
pub const G4_MELODIC_SCALE: Scale<8> = Scale::melodic([G4, A4, ASHARP4, C5, D5, E5, FSHARP5, G5]);

/// G#4 melodic minor scale: G#4-A#4-B4-C#5-D#5-F5-G5-G#5
/// Notes span from G#4 (MIDI note 68, ~415.30 Hz) to G#5 (MIDI note 80, ~830.61 Hz)
/// High treble range with a brilliant, ethereal quality
pub const GSHARP4_MELODIC_SCALE: Scale<8> =
    Scale::melodic([GSHARP4, ASHARP4, B4, CSHARP5, DSHARP5, F5, G5, GSHARP5]);

/// A4 melodic minor scale: A4-B4-C5-D5-E5-F#5-G#5-A5
/// Notes span from A4 (MIDI note 69, 440.00 Hz) to A5 (MIDI note 81, 880.00 Hz)
/// A4 is concert pitch (440 Hz), this scale extends to the high soprano range
pub const A4_MELODIC_SCALE: Scale<8> = Scale::melodic([A4, B4, C5, D5, E5, FSHARP5, GSHARP5, A5]);

/// A#4 melodic minor scale: A#4-C5-C#5-D#5-F5-G5-A5-A#5
/// Notes span from A#4 (MIDI note 70, ~466.16 Hz) to A#5 (MIDI note 82, ~932.33 Hz)
/// High soprano range with brilliant, piercing quality
pub const ASHARP4_MELODIC_SCALE: Scale<8> =
    Scale::melodic([ASHARP4, C5, CSHARP5, DSHARP5, F5, G5, A5, ASHARP5]);

/// B4 melodic minor scale: B4-C#5-D5-E5-F#5-G#5-A#5-B5
/// Notes span from B4 (MIDI note 71, ~493.88 Hz) to B5 (MIDI note 83, ~987.77 Hz)
/// High soprano range, approaching the upper limits of typical vocal range
pub const B4_MELODIC_SCALE: Scale<8> =
    Scale::melodic([B4, CSHARP5, D5, E5, FSHARP5, GSHARP5, ASHARP5, B5]);

// Octave 5 melodic minor scales
// These scales start from notes in MIDI octave 5 (notes 72-83)
// These scales are in the high soprano/piccolo range, commonly used for high-register instruments

/// C5 melodic minor scale: C5-D5-D#5-F5-G5-A5-B5-C6
/// Notes span from C5 (MIDI note 72, 523.25 Hz) to C6 (MIDI note 84, 1046.50 Hz)
/// High soprano/piccolo range, at the upper limit of most soprano vocalists
pub const C5_MELODIC_SCALE: Scale<8> = Scale::melodic([C5, D5, DSHARP5, F5, G5, A5, B5, C6]);

/// C#5 melodic minor scale: C#5-D#5-E5-F#5-G#5-A#5-C6-C#6
/// Notes span from C#5 (MIDI note 73, ~554.37 Hz) to C#6 (MIDI note 85, ~1108.73 Hz)
/// Very high range with a brilliant, shimmering quality
pub const CSHARP5_MELODIC_SCALE: Scale<8> =
    Scale::melodic([CSHARP5, DSHARP5, E5, FSHARP5, GSHARP5, ASHARP5, C6, CSHARP6]);

/// D5 melodic minor scale: D5-E5-F5-G5-A5-B5-C#6-D6
/// Notes span from D5 (MIDI note 74, ~587.33 Hz) to D6 (MIDI note 86, ~1174.66 Hz)
/// High piccolo and flute range, excellent for high melodic passages
pub const D5_MELODIC_SCALE: Scale<8> = Scale::melodic([D5, E5, F5, G5, A5, B5, CSHARP6, D6]);

/// D#5 melodic minor scale: D#5-F5-F#5-G#5-A#5-C6-D6-D#6
/// Notes span from D#5 (MIDI note 75, ~622.25 Hz) to D#6 (MIDI note 87, ~1244.51 Hz)
/// Extremely high range with a brilliant, piercing quality
pub const DSHARP5_MELODIC_SCALE: Scale<8> =
    Scale::melodic([DSHARP5, F5, FSHARP5, GSHARP5, ASHARP5, C6, D6, DSHARP6]);

/// E5 melodic minor scale: E5-F#5-G5-A5-B5-C#6-D#6-E6
/// Notes span from E5 (MIDI note 76, ~659.26 Hz) to E6 (MIDI note 88, ~1318.51 Hz)
/// Very high register, above most vocal ranges, suited for piccolo and high flute
pub const E5_MELODIC_SCALE: Scale<8> =
    Scale::melodic([E5, FSHARP5, G5, A5, B5, CSHARP6, DSHARP6, E6]);

/// F5 melodic minor scale: F5-G5-G#5-A#5-C6-D6-E6-F6
/// Notes span from F5 (MIDI note 77, ~698.46 Hz) to F6 (MIDI note 89, ~1396.91 Hz)
/// Very high range used primarily by high woodwind instruments
pub const F5_MELODIC_SCALE: Scale<8> = Scale::melodic([F5, G5, GSHARP5, ASHARP5, C6, D6, E6, F6]);

/// F#5 melodic minor scale: F#5-G#5-A5-B5-C#6-D#6-F6-F#6
/// Notes span from F#5 (MIDI note 78, ~739.99 Hz) to F#6 (MIDI note 90, ~1479.98 Hz)
/// Extremely high register with a crystalline, ethereal quality
pub const FSHARP5_MELODIC_SCALE: Scale<8> =
    Scale::melodic([FSHARP5, GSHARP5, A5, B5, CSHARP6, DSHARP6, F6, FSHARP6]);

/// G5 melodic minor scale: G5-A5-A#5-C6-D6-E6-F#6-G6
/// Notes span from G5 (MIDI note 79, ~783.99 Hz) to G6 (MIDI note 91, ~1567.98 Hz)
/// Very high range, excellent for piccolo and high register instruments
pub const G5_MELODIC_SCALE: Scale<8> = Scale::melodic([G5, A5, ASHARP5, C6, D6, E6, FSHARP6, G6]);

/// G#5 melodic minor scale: G#5-A#5-B5-C#6-D#6-F6-G6-G#6
/// Notes span from G#5 (MIDI note 80, ~830.61 Hz) to G#6 (MIDI note 92, ~1661.22 Hz)
/// Extremely high register with a brilliant, whistling quality
pub const GSHARP5_MELODIC_SCALE: Scale<8> =
    Scale::melodic([GSHARP5, ASHARP5, B5, CSHARP6, DSHARP6, F6, G6, GSHARP6]);

/// A5 melodic minor scale: A5-B5-C6-D6-E6-F#6-G#6-A6
/// Notes span from A5 (MIDI note 81, 880.00 Hz) to A6 (MIDI note 93, 1760.00 Hz)
/// Very high register, one octave above concert pitch
pub const A5_MELODIC_SCALE: Scale<8> = Scale::melodic([A5, B5, C6, D6, E6, FSHARP6, GSHARP6, A6]);

/// A#5 melodic minor scale: A#5-C6-C#6-D#6-F6-G6-A6-A#6
/// Notes span from A#5 (MIDI note 82, ~932.33 Hz) to A#6 (MIDI note 94, ~1864.66 Hz)
/// Extremely high register with a penetrating, brilliant quality
pub const ASHARP5_MELODIC_SCALE: Scale<8> =
    Scale::melodic([ASHARP5, C6, CSHARP6, DSHARP6, F6, G6, A6, ASHARP6]);

/// B5 melodic minor scale: B5-C#6-D6-E6-F#6-G#6-A#6-B6
/// Notes span from B5 (MIDI note 83, ~987.77 Hz) to B6 (MIDI note 95, ~1975.53 Hz)
/// Very high register approaching the upper limit of common musical ranges
pub const B5_MELODIC_SCALE: Scale<8> =
    Scale::melodic([B5, CSHARP6, D6, E6, FSHARP6, GSHARP6, ASHARP6, B6]);

// Octave 6 melodic minor scales
// These scales start from notes in MIDI octave 6 (notes 84-95)
// These scales are in the extremely high register, beyond most vocal capabilities

/// C6 melodic minor scale: C6-D6-D#6-F6-G6-A6-B6-C7
/// Notes span from C6 (MIDI note 84, 1046.50 Hz) to C7 (MIDI note 96, 2093.00 Hz)
/// Extremely high range, often only used by piccolo and specialty instruments
pub const C6_MELODIC_SCALE: Scale<8> = Scale::melodic([C6, D6, DSHARP6, F6, G6, A6, B6, C7]);

/// C#6 melodic minor scale: C#6-D#6-E6-F#6-G#6-A#6-C7-C#7
/// Notes span from C#6 (MIDI note 85, ~1108.73 Hz) to C#7 (MIDI note 97, ~2217.46 Hz)
/// Ultra-high range with a crystalline, brilliant quality
pub const CSHARP6_MELODIC_SCALE: Scale<8> =
    Scale::melodic([CSHARP6, DSHARP6, E6, FSHARP6, GSHARP6, ASHARP6, C7, CSHARP7]);

/// D6 melodic minor scale: D6-E6-F6-G6-A6-B6-C#7-D7
/// Notes span from D6 (MIDI note 86, ~1174.66 Hz) to D7 (MIDI note 98, ~2349.32 Hz)
/// Very high piccolo range, at the upper limit of most instrument capabilities
pub const D6_MELODIC_SCALE: Scale<8> = Scale::melodic([D6, E6, F6, G6, A6, B6, CSHARP7, D7]);

/// D#6 melodic minor scale: D#6-F6-F#6-G#6-A#6-C7-D7-D#7
/// Notes span from D#6 (MIDI note 87, ~1244.51 Hz) to D#7 (MIDI note 99, ~2489.02 Hz)
/// Extremely high register rarely used in composition
pub const DSHARP6_MELODIC_SCALE: Scale<8> =
    Scale::melodic([DSHARP6, F6, FSHARP6, GSHARP6, ASHARP6, C7, D7, DSHARP7]);

/// E6 melodic minor scale: E6-F#6-G6-A6-B6-C#7-D#7-E7
/// Notes span from E6 (MIDI note 88, ~1318.51 Hz) to E7 (MIDI note 100, ~2637.02 Hz)
/// Ultra-high range at the upper limit of most instrumental ranges
pub const E6_MELODIC_SCALE: Scale<8> =
    Scale::melodic([E6, FSHARP6, G6, A6, B6, CSHARP7, DSHARP7, E7]);

/// F6 melodic minor scale: F6-G6-G#6-A#6-C7-D7-E7-F7
/// Notes span from F6 (MIDI note 89, ~1396.91 Hz) to F7 (MIDI note 101, ~2793.83 Hz)
/// Extremely high register used only by specialized instruments
pub const F6_MELODIC_SCALE: Scale<8> = Scale::melodic([F6, G6, GSHARP6, ASHARP6, C7, D7, E7, F7]);

/// F#6 melodic minor scale: F#6-G#6-A6-B6-C#7-D#7-F7-F#7
/// Notes span from F#6 (MIDI note 90, ~1479.98 Hz) to F#7 (MIDI note 102, ~2959.96 Hz)
/// Ultra-high frequency range beyond most conventional musical applications
pub const FSHARP6_MELODIC_SCALE: Scale<8> =
    Scale::melodic([FSHARP6, GSHARP6, A6, B6, CSHARP7, DSHARP7, F7, FSHARP7]);

/// G6 melodic minor scale: G6-A6-A#6-C7-D7-E7-F#7-G7
/// Notes span from G6 (MIDI note 91, ~1567.98 Hz) to G7 (MIDI note 103, ~3135.96 Hz)
/// Very high register approaching the upper limit of human hearing sensitivity
pub const G6_MELODIC_SCALE: Scale<8> = Scale::melodic([G6, A6, ASHARP6, C7, D7, E7, FSHARP7, G7]);

/// G#6 melodic minor scale: G#6-A#6-B6-C#7-D#7-F7-G7-G#7
/// Notes span from G#6 (MIDI note 92, ~1661.22 Hz) to G#7 (MIDI note 104, ~3322.44 Hz)
/// Extremely high register rarely used in conventional music
pub const GSHARP6_MELODIC_SCALE: Scale<8> =
    Scale::melodic([GSHARP6, ASHARP6, B6, CSHARP7, DSHARP7, F7, G7, GSHARP7]);

/// A6 melodic minor scale: A6-B6-C7-D7-E7-F#7-G#7-A7
/// Notes span from A6 (MIDI note 93, 1760.00 Hz) to A7 (MIDI note 105, 3520.00 Hz)
/// Ultra-high register, rarely utilized in composition
pub const A6_MELODIC_SCALE: Scale<8> = Scale::melodic([A6, B6, C7, D7, E7, FSHARP7, GSHARP7, A7]);

/// A#6 melodic minor scale: A#6-C7-C#7-D#7-F7-G7-A7-A#7
/// Notes span from A#6 (MIDI note 94, ~1864.66 Hz) to A#7 (MIDI note 106, ~3729.31 Hz)
/// Extremely high register at the limit of most synthesizers' practical range
pub const ASHARP6_MELODIC_SCALE: Scale<8> =
    Scale::melodic([ASHARP6, C7, CSHARP7, DSHARP7, F7, G7, A7, ASHARP7]);

/// B6 melodic minor scale: B6-C#7-D7-E7-F#7-G#7-A#7-B7
/// Notes span from B6 (MIDI note 95, ~1975.53 Hz) to B7 (MIDI note 107, ~3951.07 Hz)
/// Ultra-high register, used primarily in electronic music and special effects
pub const B6_MELODIC_SCALE: Scale<8> =
    Scale::melodic([B6, CSHARP7, D7, E7, FSHARP7, GSHARP7, ASHARP7, B7]);

// Octave 7 melodic minor scales
// These scales start from notes in MIDI octave 7 (notes 96-107)
// These scales are in the ultra-high register, at the upper limit of human hearing

/// C7 melodic minor scale: C7-D7-D#7-F7-G7-A7-B7-C8
/// Notes span from C7 (MIDI note 96, 2093.00 Hz) to C8 (MIDI note 108, 4186.01 Hz)
/// Extremely high range at the upper limit of human pitch discrimination
pub const C7_MELODIC_SCALE: Scale<8> = Scale::melodic([C7, D7, DSHARP7, F7, G7, A7, B7, C8]);

/// C#7 melodic minor scale: C#7-D#7-E7-F#7-G#7-A#7-C8-C#8
/// Notes span from C#7 (MIDI note 97, ~2217.46 Hz) to C#8 (MIDI note 109, ~4434.92 Hz)
/// Ultra-high range used primarily in electronic and experimental music
pub const CSHARP7_MELODIC_SCALE: Scale<8> =
    Scale::melodic([CSHARP7, DSHARP7, E7, FSHARP7, GSHARP7, ASHARP7, C8, CSHARP8]);

/// D7 melodic minor scale: D7-E7-F7-G7-A7-B7-C#8-D8
/// Notes span from D7 (MIDI note 98, ~2349.32 Hz) to D8 (MIDI note 110, ~4698.64 Hz)
/// Extremely high register at the threshold of human hearing perception
pub const D7_MELODIC_SCALE: Scale<8> = Scale::melodic([D7, E7, F7, G7, A7, B7, CSHARP8, D8]);

/// D#7 melodic minor scale: D#7-F7-F#7-G#7-A#7-C8-D8-D#8
/// Notes span from D#7 (MIDI note 99, ~2489.02 Hz) to D#8 (MIDI note 111, ~4978.03 Hz)
/// Ultra-high register approaching the upper limit of MIDI range
pub const DSHARP7_MELODIC_SCALE: Scale<8> =
    Scale::melodic([DSHARP7, F7, FSHARP7, GSHARP7, ASHARP7, C8, D8, DSHARP8]);

/// E7 melodic minor scale: E7-F#7-G7-A7-B7-C#8-D#8-E8
/// Notes span from E7 (MIDI note 100, ~2637.02 Hz) to E8 (MIDI note 112, ~5274.04 Hz)
/// Extremely high range used only in electronic and computer-generated music
pub const E7_MELODIC_SCALE: Scale<8> =
    Scale::melodic([E7, FSHARP7, G7, A7, B7, CSHARP8, DSHARP8, E8]);

/// F7 melodic minor scale: F7-G7-G#7-A#7-C8-D8-E8-F8
/// Notes span from F7 (MIDI note 101, ~2793.83 Hz) to F8 (MIDI note 113, ~5587.65 Hz)
/// Ultra-high register beyond normal hearing sensitivity for most people
pub const F7_MELODIC_SCALE: Scale<8> = Scale::melodic([F7, G7, GSHARP7, ASHARP7, C8, D8, E8, F8]);

/// F#7 melodic minor scale: F#7-G#7-A7-B7-C#8-D#8-F8-F#8
/// Notes span from F#7 (MIDI note 102, ~2959.96 Hz) to F#8 (MIDI note 114, ~5919.91 Hz)
/// Extremely high range at the limit of perception
pub const FSHARP7_MELODIC_SCALE: Scale<8> =
    Scale::melodic([FSHARP7, GSHARP7, A7, B7, CSHARP8, DSHARP8, F8, FSHARP8]);

/// G7 melodic minor scale: G7-A7-A#7-C8-D8-E8-F#8-G8
/// Notes span from G7 (MIDI note 103, ~3135.96 Hz) to G8 (MIDI note 115, ~6271.93 Hz)
/// Ultra-high register used primarily in electronic sound synthesis
pub const G7_MELODIC_SCALE: Scale<8> = Scale::melodic([G7, A7, ASHARP7, C8, D8, E8, FSHARP8, G8]);

/// G#7 melodic minor scale: G#7-A#7-B7-C#8-D#8-F8-G8-G#8
/// Notes span from G#7 (MIDI note 104, ~3322.44 Hz) to G#8 (MIDI note 116, ~6644.88 Hz)
/// Extremely high range at the upper limit of MIDI protocol
pub const GSHARP7_MELODIC_SCALE: Scale<8> =
    Scale::melodic([GSHARP7, ASHARP7, B7, CSHARP8, DSHARP8, F8, G8, GSHARP8]);

/// A7 melodic minor scale: A7-B7-C8-D8-E8-F#8-G#8-A8
/// Notes span from A7 (MIDI note 105, 3520.00 Hz) to A8 (MIDI note 117, 7040.00 Hz)
/// Ultra-high register with frequencies challenging the upper range of human hearing
pub const A7_MELODIC_SCALE: Scale<8> = Scale::melodic([A7, B7, C8, D8, E8, FSHARP8, GSHARP8, A8]);

/// A#7 melodic minor scale: A#7-C8-C#8-D#8-F8-G8-A8-A#8
/// Notes span from A#7 (MIDI note 106, ~3729.31 Hz) to A#8 (MIDI note 118, ~7458.62 Hz)
/// Extremely high register at the boundary of audible sound
pub const ASHARP7_MELODIC_SCALE: Scale<8> =
    Scale::melodic([ASHARP7, C8, CSHARP8, DSHARP8, F8, G8, A8, ASHARP8]);

/// B7 melodic minor scale: B7-C#8-D8-E8-F#8-G#8-A#8-B8
/// Notes span from B7 (MIDI note 107, ~3951.07 Hz) to B8 (MIDI note 119, ~7902.13 Hz)
/// Ultra-high register approaching the upper limit of the standard MIDI specification
pub const B7_MELODIC_SCALE: Scale<8> =
    Scale::melodic([B7, CSHARP8, D8, E8, FSHARP8, GSHARP8, ASHARP8, B8]);

// Octave 8 melodic minor scales
// This scale starts from the highest octave in the MIDI range (notes 108-120)
// This range is at the limit of human hearing perception

/// C8 melodic minor scale: C8-D8-D#8-F8-G8-A8-B8-C9
/// Notes span from C8 (MIDI note 108, 4186.01 Hz) to the hypothetical C9 (beyond standard MIDI)
/// The highest standard scale in the MIDI specification, at the upper limit of human hearing
pub const C8_MELODIC_SCALE: Scale<8> = Scale::melodic([C8, D8, DSHARP8, F8, G8, A8, B8, C9]);

/// C#8 melodic minor scale: C#8-D#8-E8-F#8-G#8-A#8-C9-C#9
/// Notes span from C#8 (MIDI note 109, ~4434.92 Hz) to the hypothetical C#9 (beyond standard MIDI)
/// Ultra-high register at the extreme upper threshold of human hearing perception
pub const CSHARP8_MELODIC_SCALE: Scale<8> =
    Scale::melodic([CSHARP8, DSHARP8, E8, FSHARP8, GSHARP8, ASHARP8, C9, CSHARP9]);

/// D8 melodic minor scale: D8-E8-F8-G8-A8-B8-C#9-D9
/// Notes span from D8 (MIDI note 110, ~4698.64 Hz) to the hypothetical D9 (beyond standard MIDI)
/// Ultra-high register at the extreme upper threshold of human hearing perception
pub const D8_MELODIC_SCALE: Scale<8> = Scale::melodic([D8, E8, F8, G8, A8, B8, CSHARP9, D9]);

/// D#8 melodic minor scale: D#8-F8-F#8-G#8-A#8-C9-D9-D#9
/// Notes span from D#8 (MIDI note 111, ~4978.03 Hz) to the hypothetical D#9 (beyond standard MIDI)
/// Ultra-high register at the extreme upper threshold of human hearing perception
pub const DSHARP8_MELODIC_SCALE: Scale<8> =
    Scale::melodic([DSHARP8, F8, FSHARP8, GSHARP8, ASHARP8, C9, D9, DSHARP9]);

/// E8 melodic minor scale: E8-F#8-G8-A8-B8-C#9-D#9-E9
/// Notes span from E8 (MIDI note 112, ~5274.04 Hz) to the hypothetical E9 (beyond standard MIDI)
/// Ultra-high register at the extreme upper threshold of human hearing perception
pub const E8_MELODIC_SCALE: Scale<8> =
    Scale::melodic([E8, FSHARP8, G8, A8, B8, CSHARP9, DSHARP9, E9]);

/// F8 melodic minor scale: F8-G8-G#8-A#8-C9-D9-E9-F9
/// Notes span from F8 (MIDI note 113, ~5587.65 Hz) to the hypothetical F9 (beyond standard MIDI)
/// Ultra-high register at the extreme upper threshold of human hearing perception
pub const F8_MELODIC_SCALE: Scale<8> = Scale::melodic([F8, G8, GSHARP8, ASHARP8, C9, D9, E9, F9]);

/// F#8 melodic minor scale: F#8-G#8-A8-B8-C#9-D#9-F9-F#9
/// Notes span from F#8 (MIDI note 114, ~5919.91 Hz) to the hypothetical F#9 (beyond standard MIDI)
/// Ultra-high register at the extreme upper threshold of human hearing perception
pub const FSHARP8_MELODIC_SCALE: Scale<8> =
    Scale::melodic([FSHARP8, GSHARP8, A8, B8, CSHARP9, DSHARP9, F9, FSHARP9]);

/// G8 melodic minor scale: G8-A8-A#8-C9-D9-E9-F#9-G9
/// Notes span from G8 (MIDI note 115, ~6271.93 Hz) to the hypothetical G9 (beyond standard MIDI)
/// Ultra-high register at the extreme upper threshold of human hearing perception
pub const G8_MELODIC_SCALE: Scale<8> = Scale::melodic([G8, A8, ASHARP8, C9, D9, E9, FSHARP9, G9]);

lazy_static! {
    /// A HashMap containing all melodic minor scales indexed by their root pitch.
    ///
    /// This collection provides quick access to melodic minor scales across all available octaves.
    /// The scales are organized by octave, from octave 0 to octave 8, with each scale being
    /// accessible by its root pitch.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::MELODIC_SCALES;
    /// use mozzart_std::C4;
    ///
    /// // Get the C melodic minor scale starting from middle C (C4)
    /// let c_melodic = MELODIC_SCALES.get(&C4).unwrap();
    /// ```
    pub static ref MELODIC_SCALES: HashMap<Pitch, Scale<8>> = HashMap::from([
        // Canonical scales (octave-independent)
        (C, C_MELODIC_SCALE),
        (CSHARP, CSHARP_MELODIC_SCALE),
        (D, D_MELODIC_SCALE),
        (DSHARP, DSHARP_MELODIC_SCALE),
        (E, E_MELODIC_SCALE),
        (F, F_MELODIC_SCALE),
        (FSHARP, FSHARP_MELODIC_SCALE),
        (G, G_MELODIC_SCALE),
        (GSHARP, GSHARP_MELODIC_SCALE),
        (A, A_MELODIC_SCALE),
        (ASHARP, ASHARP_MELODIC_SCALE),
        (B, B_MELODIC_SCALE),

        // Octave 0
        (C0, C0_MELODIC_SCALE),
        (CSHARP0, CSHARP0_MELODIC_SCALE),
        (D0, D0_MELODIC_SCALE),
        (DSHARP0, DSHARP0_MELODIC_SCALE),
        (E0, E0_MELODIC_SCALE),
        (F0, F0_MELODIC_SCALE),
        (FSHARP0, FSHARP0_MELODIC_SCALE),
        (G0, G0_MELODIC_SCALE),
        (GSHARP0, GSHARP0_MELODIC_SCALE),
        (A0, A0_MELODIC_SCALE),
        (ASHARP0, ASHARP0_MELODIC_SCALE),
        (B0, B0_MELODIC_SCALE),

        // Octave 1
        (C1, C1_MELODIC_SCALE),
        (CSHARP1, CSHARP1_MELODIC_SCALE),
        (D1, D1_MELODIC_SCALE),
        (DSHARP1, DSHARP1_MELODIC_SCALE),
        (E1, E1_MELODIC_SCALE),
        (F1, F1_MELODIC_SCALE),
        (FSHARP1, FSHARP1_MELODIC_SCALE),
        (G1, G1_MELODIC_SCALE),
        (GSHARP1, GSHARP1_MELODIC_SCALE),
        (A1, A1_MELODIC_SCALE),
        (ASHARP1, ASHARP1_MELODIC_SCALE),
        (B1, B1_MELODIC_SCALE),

        // Octave 2
        (C2, C2_MELODIC_SCALE),
        (CSHARP2, CSHARP2_MELODIC_SCALE),
        (D2, D2_MELODIC_SCALE),
        (DSHARP2, DSHARP2_MELODIC_SCALE),
        (E2, E2_MELODIC_SCALE),
        (F2, F2_MELODIC_SCALE),
        (FSHARP2, FSHARP2_MELODIC_SCALE),
        (G2, G2_MELODIC_SCALE),
        (GSHARP2, GSHARP2_MELODIC_SCALE),
        (A2, A2_MELODIC_SCALE),
        (ASHARP2, ASHARP2_MELODIC_SCALE),
        (B2, B2_MELODIC_SCALE),

        // Octave 3
        (C3, C3_MELODIC_SCALE),
        (CSHARP3, CSHARP3_MELODIC_SCALE),
        (D3, D3_MELODIC_SCALE),
        (DSHARP3, DSHARP3_MELODIC_SCALE),
        (E3, E3_MELODIC_SCALE),
        (F3, F3_MELODIC_SCALE),
        (FSHARP3, FSHARP3_MELODIC_SCALE),
        (G3, G3_MELODIC_SCALE),
        (GSHARP3, GSHARP3_MELODIC_SCALE),
        (A3, A3_MELODIC_SCALE),
        (ASHARP3, ASHARP3_MELODIC_SCALE),
        (B3, B3_MELODIC_SCALE),

        // Octave 4
        (C4, C4_MELODIC_SCALE),
        (CSHARP4, CSHARP4_MELODIC_SCALE),
        (D4, D4_MELODIC_SCALE),
        (DSHARP4, DSHARP4_MELODIC_SCALE),
        (E4, E4_MELODIC_SCALE),
        (F4, F4_MELODIC_SCALE),
        (FSHARP4, FSHARP4_MELODIC_SCALE),
        (G4, G4_MELODIC_SCALE),
        (GSHARP4, GSHARP4_MELODIC_SCALE),
        (A4, A4_MELODIC_SCALE),
        (ASHARP4, ASHARP4_MELODIC_SCALE),
        (B4, B4_MELODIC_SCALE),

        // Octave 5
        (C5, C5_MELODIC_SCALE),
        (CSHARP5, CSHARP5_MELODIC_SCALE),
        (D5, D5_MELODIC_SCALE),
        (DSHARP5, DSHARP5_MELODIC_SCALE),
        (E5, E5_MELODIC_SCALE),
        (F5, F5_MELODIC_SCALE),
        (FSHARP5, FSHARP5_MELODIC_SCALE),
        (G5, G5_MELODIC_SCALE),
        (GSHARP5, GSHARP5_MELODIC_SCALE),
        (A5, A5_MELODIC_SCALE),
        (ASHARP5, ASHARP5_MELODIC_SCALE),
        (B5, B5_MELODIC_SCALE),

        // Octave 6
        (C6, C6_MELODIC_SCALE),
        (CSHARP6, CSHARP6_MELODIC_SCALE),
        (D6, D6_MELODIC_SCALE),
        (DSHARP6, DSHARP6_MELODIC_SCALE),
        (E6, E6_MELODIC_SCALE),
        (F6, F6_MELODIC_SCALE),
        (FSHARP6, FSHARP6_MELODIC_SCALE),
        (G6, G6_MELODIC_SCALE),
        (GSHARP6, GSHARP6_MELODIC_SCALE),
        (A6, A6_MELODIC_SCALE),
        (ASHARP6, ASHARP6_MELODIC_SCALE),
        (B6, B6_MELODIC_SCALE),

        // Octave 7
        (C7, C7_MELODIC_SCALE),
        (CSHARP7, CSHARP7_MELODIC_SCALE),
        (D7, D7_MELODIC_SCALE),
        (DSHARP7, DSHARP7_MELODIC_SCALE),
        (E7, E7_MELODIC_SCALE),
        (F7, F7_MELODIC_SCALE),
        (FSHARP7, FSHARP7_MELODIC_SCALE),
        (G7, G7_MELODIC_SCALE),
        (GSHARP7, GSHARP7_MELODIC_SCALE),
        (A7, A7_MELODIC_SCALE),
        (ASHARP7, ASHARP7_MELODIC_SCALE),
        (B7, B7_MELODIC_SCALE),

        // Octave 8
        (C8, C8_MELODIC_SCALE),
        (CSHARP8, CSHARP8_MELODIC_SCALE),
        (D8, D8_MELODIC_SCALE),
        (DSHARP8, DSHARP8_MELODIC_SCALE),
        (E8, E8_MELODIC_SCALE),
        (F8, F8_MELODIC_SCALE),
        (FSHARP8, FSHARP8_MELODIC_SCALE),
        (G8, G8_MELODIC_SCALE),
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_melodic_scales_steps() {
        assert_eq!(
            MELODIC_SCALE_STEPS,
            [UNISON, TONE, SEMITONE, TONE, TONE, TONE, TONE, SEMITONE]
        );
    }

    #[test]
    fn test_melodic_scales_canonical() {
        assert_eq!(C_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(CSHARP_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(D_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(DSHARP_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(E_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(F_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(FSHARP_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(G_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(GSHARP_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(A_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(ASHARP_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(B_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_canonical() {
        assert_eq!(MELODIC_SCALES.contains_key(&C), true);
        assert_eq!(MELODIC_SCALES.contains_key(&CSHARP), true);
        assert_eq!(MELODIC_SCALES.contains_key(&D), true);
        assert_eq!(MELODIC_SCALES.contains_key(&DSHARP), true);
        assert_eq!(MELODIC_SCALES.contains_key(&E), true);
        assert_eq!(MELODIC_SCALES.contains_key(&F), true);
        assert_eq!(MELODIC_SCALES.contains_key(&FSHARP), true);
        assert_eq!(MELODIC_SCALES.contains_key(&G), true);
        assert_eq!(MELODIC_SCALES.contains_key(&GSHARP), true);
        assert_eq!(MELODIC_SCALES.contains_key(&A), true);
        assert_eq!(MELODIC_SCALES.contains_key(&ASHARP), true);
        assert_eq!(MELODIC_SCALES.contains_key(&B), true);
    }

    #[test]
    fn test_melodic_scale_octave_0() {
        assert_eq!(C0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(CSHARP0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(D0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(DSHARP0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(E0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(F0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(FSHARP0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(G0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(GSHARP0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(A0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(ASHARP0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(B0_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_0() {
        assert_eq!(MELODIC_SCALES.get(&C0), Some(&C0_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&CSHARP0), Some(&CSHARP0_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&D0), Some(&D0_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&DSHARP0), Some(&DSHARP0_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&E0), Some(&E0_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&F0), Some(&F0_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&FSHARP0), Some(&FSHARP0_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&G0), Some(&G0_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&GSHARP0), Some(&GSHARP0_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&A0), Some(&A0_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&ASHARP0), Some(&ASHARP0_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&B0), Some(&B0_MELODIC_SCALE));
    }

    #[test]
    fn test_melodic_scale_octave_1() {
        assert_eq!(C1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(CSHARP1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(D1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(DSHARP1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(E1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(F1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(FSHARP1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(G1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(GSHARP1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(A1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(ASHARP1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(B1_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_1() {
        assert_eq!(MELODIC_SCALES.get(&C1), Some(&C1_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&CSHARP1), Some(&CSHARP1_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&D1), Some(&D1_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&DSHARP1), Some(&DSHARP1_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&E1), Some(&E1_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&F1), Some(&F1_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&FSHARP1), Some(&FSHARP1_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&G1), Some(&G1_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&GSHARP1), Some(&GSHARP1_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&A1), Some(&A1_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&ASHARP1), Some(&ASHARP1_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&B1), Some(&B1_MELODIC_SCALE));
    }

    #[test]
    fn test_melodic_scale_octave_2() {
        assert_eq!(C2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(CSHARP2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(D2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(DSHARP2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(E2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(F2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(FSHARP2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(G2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(GSHARP2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(A2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(ASHARP2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(B2_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_2() {
        assert_eq!(MELODIC_SCALES.get(&C2), Some(&C2_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&CSHARP2), Some(&CSHARP2_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&D2), Some(&D2_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&DSHARP2), Some(&DSHARP2_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&E2), Some(&E2_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&F2), Some(&F2_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&FSHARP2), Some(&FSHARP2_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&G2), Some(&G2_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&GSHARP2), Some(&GSHARP2_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&A2), Some(&A2_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&ASHARP2), Some(&ASHARP2_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&B2), Some(&B2_MELODIC_SCALE));
    }

    #[test]
    fn test_melodic_scale_octave_3() {
        assert_eq!(C3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(CSHARP3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(D3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(DSHARP3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(E3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(F3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(FSHARP3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(G3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(GSHARP3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(A3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(ASHARP3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(B3_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_3() {
        assert_eq!(MELODIC_SCALES.get(&C3), Some(&C3_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&CSHARP3), Some(&CSHARP3_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&D3), Some(&D3_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&DSHARP3), Some(&DSHARP3_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&E3), Some(&E3_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&F3), Some(&F3_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&FSHARP3), Some(&FSHARP3_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&G3), Some(&G3_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&GSHARP3), Some(&GSHARP3_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&A3), Some(&A3_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&ASHARP3), Some(&ASHARP3_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&B3), Some(&B3_MELODIC_SCALE));
    }

    #[test]
    fn test_melodic_scale_octave_4() {
        assert_eq!(C4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(CSHARP4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(D4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(DSHARP4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(E4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(F4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(FSHARP4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(G4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(GSHARP4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(A4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(ASHARP4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(B4_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_4() {
        assert_eq!(MELODIC_SCALES.get(&C4), Some(&C4_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&CSHARP4), Some(&CSHARP4_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&D4), Some(&D4_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&DSHARP4), Some(&DSHARP4_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&E4), Some(&E4_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&F4), Some(&F4_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&FSHARP4), Some(&FSHARP4_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&G4), Some(&G4_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&GSHARP4), Some(&GSHARP4_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&A4), Some(&A4_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&ASHARP4), Some(&ASHARP4_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&B4), Some(&B4_MELODIC_SCALE));
    }

    #[test]
    fn test_melodic_scale_octave_5() {
        assert_eq!(C5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(CSHARP5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(D5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(DSHARP5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(E5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(F5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(FSHARP5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(G5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(GSHARP5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(A5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(ASHARP5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(B5_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_5() {
        assert_eq!(MELODIC_SCALES.get(&C5), Some(&C5_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&CSHARP5), Some(&CSHARP5_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&D5), Some(&D5_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&DSHARP5), Some(&DSHARP5_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&E5), Some(&E5_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&F5), Some(&F5_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&FSHARP5), Some(&FSHARP5_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&G5), Some(&G5_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&GSHARP5), Some(&GSHARP5_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&A5), Some(&A5_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&ASHARP5), Some(&ASHARP5_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&B5), Some(&B5_MELODIC_SCALE));
    }

    #[test]
    fn test_melodic_scale_octave_6() {
        assert_eq!(C6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(CSHARP6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(D6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(DSHARP6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(E6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(F6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(FSHARP6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(G6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(GSHARP6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(A6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(ASHARP6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(B6_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_6() {
        assert_eq!(MELODIC_SCALES.get(&C6), Some(&C6_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&CSHARP6), Some(&CSHARP6_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&D6), Some(&D6_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&DSHARP6), Some(&DSHARP6_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&E6), Some(&E6_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&F6), Some(&F6_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&FSHARP6), Some(&FSHARP6_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&G6), Some(&G6_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&GSHARP6), Some(&GSHARP6_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&A6), Some(&A6_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&ASHARP6), Some(&ASHARP6_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&B6), Some(&B6_MELODIC_SCALE));
    }

    #[test]
    fn test_melodic_scale_octave_7() {
        assert_eq!(C7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(CSHARP7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(D7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(DSHARP7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(E7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(F7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(FSHARP7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(G7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(GSHARP7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(A7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(ASHARP7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(B7_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_7() {
        assert_eq!(MELODIC_SCALES.get(&C7), Some(&C7_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&CSHARP7), Some(&CSHARP7_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&D7), Some(&D7_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&DSHARP7), Some(&DSHARP7_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&E7), Some(&E7_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&F7), Some(&F7_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&FSHARP7), Some(&FSHARP7_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&G7), Some(&G7_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&GSHARP7), Some(&GSHARP7_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&A7), Some(&A7_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&ASHARP7), Some(&ASHARP7_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&B7), Some(&B7_MELODIC_SCALE));
    }

    #[test]
    fn test_melodic_scale_octave_8() {
        assert_eq!(C8_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(CSHARP8_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(D8_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(DSHARP8_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(E8_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(F8_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(FSHARP8_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
        assert_eq!(G8_MELODIC_SCALE.steps(), MELODIC_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_8() {
        assert_eq!(MELODIC_SCALES.get(&C8), Some(&C8_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&CSHARP8), Some(&CSHARP8_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&D8), Some(&D8_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&DSHARP8), Some(&DSHARP8_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&E8), Some(&E8_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&F8), Some(&F8_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&FSHARP8), Some(&FSHARP8_MELODIC_SCALE));
        assert_eq!(MELODIC_SCALES.get(&G8), Some(&G8_MELODIC_SCALE));
    }
}
