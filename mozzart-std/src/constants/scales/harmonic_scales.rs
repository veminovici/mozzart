//! Harmonic minor scale constants and collections
//!
//! This module provides predefined constants for all harmonic minor scales across the MIDI note range.
//! Each scale is represented as an array of 8 notes (one octave) starting from a specific root pitch.
//!
//! # Harmonic Minor Scale Structure
//! A harmonic minor scale follows the interval pattern:
//! Whole-Half-Whole-Whole-Half-Augmented Second-Half, creating the following steps:
//! - Between 1-2: whole step (2 semitones)
//! - Between 2-3: half step (1 semitone)
//! - Between 3-4: whole step (2 semitones)
//! - Between 4-5: whole step (2 semitones)
//! - Between 5-6: half step (1 semitone)
//! - Between 6-7: augmented second (3 semitones)
//! - Between 7-8: half step (1 semitone)
//!
//! # Musical Characteristics
//! The harmonic minor scale is characterized by:
//! - Raised 7th degree compared to the natural minor scale
//! - Distinctive augmented second interval between 6th and 7th degrees
//! - Creates a stronger dominant function due to the leading tone
//! - Common in classical music and various world music traditions
//!
//! # Scale Organization
//! Scales are organized in several ways:
//! 1. Octave-independent scales (e.g., `C_HARMONIC_SCALE`) - generic patterns
//! 2. Octave-specific scales (e.g., `C4_HARMONIC_SCALE`) - starting from specific octaves
//! 3. Collections in `HARMONIC_SCALES` - HashMap mapping root pitches to their harmonic minor scales
//!
//! # Examples
//! ```
//! use mozzart_std::{A4_HARMONIC_SCALE, HARMONIC_SCALES};
//! use mozzart_std::A4;
//!
//! // Use a predefined scale
//! let a_harmonic = A4_HARMONIC_SCALE;
//!
//! // Look up a scale by root note
//! let scale = HARMONIC_SCALES.get(&A4).unwrap();
//! assert_eq!(*scale, A4_HARMONIC_SCALE);
//! ```

use crate::constants::*;
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

/// The interval steps that define a harmonic minor scale
///
/// These intervals represent the distance between consecutive notes in the scale:
/// - Root to 2nd: whole step (TONE)
/// - 2nd to 3rd: half step (SEMITONE)
/// - 3rd to 4th: whole step (TONE)
/// - 4th to 5th: whole step (TONE)
/// - 5th to 6th: half step (SEMITONE)
/// - 6th to 7th: augmented second (3 semitones, same as MINOR_THIRD)
/// - 7th to octave: half step (SEMITONE)
pub const HARMONIC_SCALE_STEPS: [Interval; 8] = [
    UNISON,
    TONE,
    SEMITONE,
    TONE,
    TONE,
    SEMITONE,
    MINOR_THIRD, // Augmented second (3 semitones)
    SEMITONE,
];

// Canonical harmonic minor scales (octave-independent)
//
// These scales define the pattern of notes for each harmonic minor scale without
// being tied to a specific octave. The scales wrap around to the next octave
// to complete the 8-note pattern.

/// C harmonic minor scale: C, D, D#, F, G, G#, B, C
/// Relative to A minor, with raised 7th degree (B instead of Bb)
/// Characteristic sound: exotic, Middle Eastern/Eastern European flavor
pub const C_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C, D, DSHARP, F, G, GSHARP, B, C0]);

/// C# harmonic minor scale: C#, D#, E, F#, G#, A, C, C#
/// Relative to A# minor, with raised 7th degree (C instead of B)
/// Enharmonic equivalent to Db harmonic minor
pub const CSHARP_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([CSHARP, DSHARP, E, FSHARP, GSHARP, A, C0, CSHARP0]);

/// D harmonic minor scale: D, E, F, G, A, A#, C#, D
/// Relative to B minor, with raised 7th degree (C# instead of C)
/// Used in flamenco music and Eastern European folk music
pub const D_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D, E, F, G, A, ASHARP, CSHARP0, D0]);

/// D# harmonic minor scale: D#, F, F#, G#, A#, B, D, D#
/// Relative to C minor, with raised 7th degree (D instead of Db)
/// Enharmonic equivalent to Eb harmonic minor
pub const DSHARP_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([DSHARP, F, FSHARP, GSHARP, ASHARP, B, D0, DSHARP0]);

/// E harmonic minor scale: E, F#, G, A, B, C, D#, E
/// Relative to C# minor, with raised 7th degree (D# instead of D)
/// Common in classical guitar music and Spanish compositions
pub const E_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E, FSHARP, G, A, B, C0, DSHARP0, E0]);

/// F harmonic minor scale: F, G, G#, A#, C, C#, E, F
/// Relative to D minor, with raised 7th degree (E instead of Eb)
/// Creates a dramatic, tense sound used in film scores
pub const F_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F, G, GSHARP, ASHARP, C0, CSHARP0, E0, F0]);

/// F# harmonic minor scale: F#, G#, A, B, C#, D, F, F#
/// Relative to D# minor, with raised 7th degree (F instead of E)
/// Enharmonic equivalent to Gb harmonic minor
pub const FSHARP_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([FSHARP, GSHARP, A, B, CSHARP0, D0, F0, FSHARP0]);

/// G harmonic minor scale: G, A, A#, C, D, D#, F#, G
/// Relative to E minor, with raised 7th degree (F# instead of F)
/// Often used in Baroque music and Eastern European folk music
pub const G_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([G, A, ASHARP, C0, D0, DSHARP0, FSHARP0, G0]);

/// G# harmonic minor scale: G#, A#, B, C#, D#, E, G, G#
/// Relative to F minor, with raised 7th degree (G instead of Gb)
/// Enharmonic equivalent to Ab harmonic minor
pub const GSHARP_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([GSHARP, ASHARP, B, CSHARP0, DSHARP0, E0, G0, GSHARP0]);

/// A harmonic minor scale: A, B, C, D, E, F, G#, A
/// The most common harmonic minor scale
/// Characteristic of Spanish, Middle Eastern, and Eastern European music
pub const A_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A, B, C0, D0, E0, F0, GSHARP0, A0]);

/// A# harmonic minor scale: A#, C, C#, D#, F, F#, A, A#
/// Relative to F# minor, with raised 7th degree (A instead of G#)
/// Enharmonic equivalent to Bb harmonic minor
pub const ASHARP_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([ASHARP, C0, CSHARP0, DSHARP0, F0, FSHARP0, A0, ASHARP0]);

/// B harmonic minor scale: B, C#, D, E, F#, G, A#, B
/// Relative to G# minor, with raised 7th degree (A# instead of A)
/// Used in classical compositions and flamenco music
pub const B_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([B, CSHARP0, D0, E0, FSHARP0, G0, ASHARP0, B0]);

// Octave 0 harmonic minor scales
// These scales start from notes in MIDI octave 0 (notes 12-23)
// The lowest complete harmonic minor scales in the MIDI range

/// C0 harmonic minor scale: C0-D0-D#0-F0-G0-G#0-B0-C1
/// Lowest C harmonic minor scale in standard MIDI (C0 = MIDI note 12)
/// Notes in Hz: C0 (16.35) - D0 (18.35) - D#0 (19.45) - F0 (21.83) - G0 (24.50) - G#0 (25.96) - B0 (30.87) - C1 (32.70)
pub const C0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C0, D0, DSHARP0, F0, G0, GSHARP0, B0, C1]);

/// C#0 harmonic minor scale: C#0-D#0-E0-F#0-G#0-A0-C1-C#1
/// Notes span from C#0 (MIDI note 13, ~17.32 Hz) to C#1 (MIDI note 25, ~34.65 Hz)
pub const CSHARP0_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([CSHARP0, DSHARP0, E0, FSHARP0, GSHARP0, A0, C1, CSHARP1]);

/// D0 harmonic minor scale: D0-E0-F0-G0-A0-A#0-C#1-D1
/// Notes span from D0 (MIDI note 14, ~18.35 Hz) to D1 (MIDI note 26, ~36.71 Hz)
pub const D0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D0, E0, F0, G0, A0, ASHARP0, CSHARP1, D1]);

/// D#0 harmonic minor scale: D#0-F0-F#0-G#0-A#0-B0-D1-D#1
/// Notes span from D#0 (MIDI note 15, ~19.45 Hz) to D#1 (MIDI note 27, ~38.89 Hz)
pub const DSHARP0_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([DSHARP0, F0, FSHARP0, GSHARP0, ASHARP0, B0, D1, DSHARP1]);

/// E0 harmonic minor scale: E0-F#0-G0-A0-B0-C1-D#1-E1
/// Notes span from E0 (MIDI note 16, ~20.60 Hz) to E1 (MIDI note 28, ~41.20 Hz)
pub const E0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E0, FSHARP0, G0, A0, B0, C1, DSHARP1, E1]);

/// F0 harmonic minor scale: F0-G0-G#0-A#0-C1-C#1-E1-F1
/// Notes span from F0 (MIDI note 17, ~21.83 Hz) to F1 (MIDI note 29, ~43.65 Hz)
pub const F0_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([F0, G0, GSHARP0, ASHARP0, C1, CSHARP1, E1, F1]);

/// F#0 harmonic minor scale: F#0-G#0-A0-B0-C#1-D1-F1-F#1
/// Notes span from F#0 (MIDI note 18, ~23.12 Hz) to F#1 (MIDI note 30, ~46.25 Hz)
pub const FSHARP0_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([FSHARP0, GSHARP0, A0, B0, CSHARP1, D1, F1, FSHARP1]);

/// G0 harmonic minor scale: G0-A0-A#0-C1-D1-D#1-F#1-G1
/// Notes span from G0 (MIDI note 19, ~24.50 Hz) to G1 (MIDI note 31, ~49.00 Hz)
pub const G0_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([G0, A0, ASHARP0, C1, D1, DSHARP1, FSHARP1, G1]);

/// G#0 harmonic minor scale: G#0-A#0-B0-C#1-D#1-E1-G1-G#1
/// Notes span from G#0 (MIDI note 20, ~25.96 Hz) to G#1 (MIDI note 32, ~51.91 Hz)
pub const GSHARP0_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([GSHARP0, ASHARP0, B0, CSHARP1, DSHARP1, E1, G1, GSHARP1]);

/// A0 harmonic minor scale: A0-B0-C1-D1-E1-F1-G#1-A1
/// Notes span from A0 (MIDI note 21, 27.50 Hz) to A1 (MIDI note 33, 55.00 Hz)
/// A0 is the lowest note on a standard piano
pub const A0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A0, B0, C1, D1, E1, F1, GSHARP1, A1]);

/// A#0 harmonic minor scale: A#0-C1-C#1-D#1-F1-F#1-A1-A#1
/// Notes span from A#0 (MIDI note 22, ~29.14 Hz) to A#1 (MIDI note 34, ~58.27 Hz)
pub const ASHARP0_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([ASHARP0, C1, CSHARP1, DSHARP1, F1, FSHARP1, A1, ASHARP1]);

/// B0 harmonic minor scale: B0-C#1-D1-E1-F#1-G1-A#1-B1
/// Notes span from B0 (MIDI note 23, ~30.87 Hz) to B1 (MIDI note 35, ~61.74 Hz)
pub const B0_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([B0, CSHARP1, D1, E1, FSHARP1, G1, ASHARP1, B1]);

// Octave 1 harmonic minor scales
// These scales start from notes in MIDI octave 1 (notes 24-35)
// These scales are in the lower bass range, often used for bass instruments

/// C1 harmonic minor scale: C1-D1-D#1-F1-G1-G#1-B1-C2
/// Notes span from C1 (MIDI note 24, 32.70 Hz) to C2 (MIDI note 36, 65.41 Hz)
/// In the low bass range, useful for bass instruments and low piano parts
pub const C1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C1, D1, DSHARP1, F1, G1, GSHARP1, B1, C2]);

/// C#1 harmonic minor scale: C#1-D#1-E1-F#1-G#1-A1-C2-C#2
/// Notes span from C#1 (MIDI note 25, ~34.65 Hz) to C#2 (MIDI note 37, ~69.30 Hz)
/// Low bass range with a dark, mysterious quality
pub const CSHARP1_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([CSHARP1, DSHARP1, E1, FSHARP1, GSHARP1, A1, C2, CSHARP2]);

/// D1 harmonic minor scale: D1-E1-F1-G1-A1-A#1-C#2-D2
/// Notes span from D1 (MIDI note 26, ~36.71 Hz) to D2 (MIDI note 38, ~73.42 Hz)
/// Low bass range, provides a rich foundation for harmonic minor compositions
pub const D1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D1, E1, F1, G1, A1, ASHARP1, CSHARP2, D2]);

/// D#1 harmonic minor scale: D#1-F1-F#1-G#1-A#1-B1-D2-D#2
/// Notes span from D#1 (MIDI note 27, ~38.89 Hz) to D#2 (MIDI note 39, ~77.78 Hz)
/// Low bass range with a somber, dramatic character
pub const DSHARP1_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([DSHARP1, F1, FSHARP1, GSHARP1, ASHARP1, B1, D2, DSHARP2]);

/// E1 harmonic minor scale: E1-F#1-G1-A1-B1-C2-D#2-E2
/// Notes span from E1 (MIDI note 28, ~41.20 Hz) to E2 (MIDI note 40, ~82.41 Hz)
/// Low E is the lowest note on a standard bass guitar (E1)
pub const E1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E1, FSHARP1, G1, A1, B1, C2, DSHARP2, E2]);

/// F1 harmonic minor scale: F1-G1-G#1-A#1-C2-C#2-E2-F2
/// Notes span from F1 (MIDI note 29, ~43.65 Hz) to F2 (MIDI note 41, ~87.31 Hz)
/// Low bass range with a rich, resonant quality
pub const F1_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([F1, G1, GSHARP1, ASHARP1, C2, CSHARP2, E2, F2]);

/// F#1 harmonic minor scale: F#1-G#1-A1-B1-C#2-D2-F2-F#2
/// Notes span from F#1 (MIDI note 30, ~46.25 Hz) to F#2 (MIDI note 42, ~92.50 Hz)
/// Low bass range with an exotic, tense character
pub const FSHARP1_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([FSHARP1, GSHARP1, A1, B1, CSHARP2, D2, F2, FSHARP2]);

/// G1 harmonic minor scale: G1-A1-A#1-C2-D2-D#2-F#2-G2
/// Notes span from G1 (MIDI note 31, ~49.00 Hz) to G2 (MIDI note 43, ~98.00 Hz)
/// Low bass range, provides a strong foundation for harmonic minor progressions
pub const G1_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([G1, A1, ASHARP1, C2, D2, DSHARP2, FSHARP2, G2]);

/// G#1 harmonic minor scale: G#1-A#1-B1-C#2-D#2-E2-G2-G#2
/// Notes span from G#1 (MIDI note 32, ~51.91 Hz) to G#2 (MIDI note 44, ~103.83 Hz)
/// Low bass range with a dark, mysterious quality
pub const GSHARP1_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([GSHARP1, ASHARP1, B1, CSHARP2, DSHARP2, E2, G2, GSHARP2]);

/// A1 harmonic minor scale: A1-B1-C2-D2-E2-F2-G#2-A2
/// Notes span from A1 (MIDI note 33, 55.00 Hz) to A2 (MIDI note 45, 110.00 Hz)
/// A1 is a common low note for cellos and bass instruments
pub const A1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A1, B1, C2, D2, E2, F2, GSHARP2, A2]);

/// A#1 harmonic minor scale: A#1-C2-C#2-D#2-F2-F#2-A2-A#2
/// Notes span from A#1 (MIDI note 34, ~58.27 Hz) to A#2 (MIDI note 46, ~116.54 Hz)
/// Low bass range with a rich, dramatic character
pub const ASHARP1_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([ASHARP1, C2, CSHARP2, DSHARP2, F2, FSHARP2, A2, ASHARP2]);

/// B1 harmonic minor scale: B1-C#2-D2-E2-F#2-G2-A#2-B2
/// Notes span from B1 (MIDI note 35, ~61.74 Hz) to B2 (MIDI note 47, ~123.47 Hz)
/// Low bass range with an exotic, tense quality
pub const B1_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([B1, CSHARP2, D2, E2, FSHARP2, G2, ASHARP2, B2]);

// Octave 2 harmonic minor scales
// These scales start from notes in MIDI octave 2 (notes 36-47)
// These scales are in the bass range, commonly used for bass instruments and left-hand piano parts

/// C2 harmonic minor scale: C2-D2-D#2-F2-G2-G#2-B2-C3
/// Notes span from C2 (MIDI note 36, 65.41 Hz) to C3 (MIDI note 48, 130.81 Hz)
/// C2 is the lowest C on a standard 88-key piano
pub const C2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C2, D2, DSHARP2, F2, G2, GSHARP2, B2, C3]);

/// C#2 harmonic minor scale: C#2-D#2-E2-F#2-G#2-A2-C3-C#3
/// Notes span from C#2 (MIDI note 37, ~69.30 Hz) to C#3 (MIDI note 49, ~138.59 Hz)
/// Bass range with a rich, exotic quality
pub const CSHARP2_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([CSHARP2, DSHARP2, E2, FSHARP2, GSHARP2, A2, C3, CSHARP3]);

/// D2 harmonic minor scale: D2-E2-F2-G2-A2-A#2-C#3-D3
/// Notes span from D2 (MIDI note 38, ~73.42 Hz) to D3 (MIDI note 50, ~146.83 Hz)
/// Bass range, commonly used in orchestral bass parts
pub const D2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D2, E2, F2, G2, A2, ASHARP2, CSHARP3, D3]);

/// D#2 harmonic minor scale: D#2-F2-F#2-G#2-A#2-B2-D3-D#3
/// Notes span from D#2 (MIDI note 39, ~77.78 Hz) to D#3 (MIDI note 51, ~155.56 Hz)
/// Bass range with a dark, dramatic character
pub const DSHARP2_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([DSHARP2, F2, FSHARP2, GSHARP2, ASHARP2, B2, D3, DSHARP3]);

/// E2 harmonic minor scale: E2-F#2-G2-A2-B2-C3-D#3-E3
/// Notes span from E2 (MIDI note 40, ~82.41 Hz) to E3 (MIDI note 52, ~164.81 Hz)
/// E2 is the lowest string on many guitars (standard tuning)
pub const E2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E2, FSHARP2, G2, A2, B2, C3, DSHARP3, E3]);

/// F2 harmonic minor scale: F2-G2-G#2-A#2-C3-C#3-E3-F3
/// Notes span from F2 (MIDI note 41, ~87.31 Hz) to F3 (MIDI note 53, ~174.61 Hz)
/// Bass range, provides a rich foundation for harmonic minor compositions
pub const F2_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([F2, G2, GSHARP2, ASHARP2, C3, CSHARP3, E3, F3]);

/// F#2 harmonic minor scale: F#2-G#2-A2-B2-C#3-D3-F3-F#3
/// Notes span from F#2 (MIDI note 42, ~92.50 Hz) to F#3 (MIDI note 54, ~185.00 Hz)
/// Bass range with an exotic, tense character
pub const FSHARP2_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([FSHARP2, GSHARP2, A2, B2, CSHARP3, D3, F3, FSHARP3]);

/// G2 harmonic minor scale: G2-A2-A#2-C3-D3-D#3-F#3-G3
/// Notes span from G2 (MIDI note 43, ~98.00 Hz) to G3 (MIDI note 55, ~196.00 Hz)
/// Bass range, commonly used in classical and flamenco music
pub const G2_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([G2, A2, ASHARP2, C3, D3, DSHARP3, FSHARP3, G3]);

/// G#2 harmonic minor scale: G#2-A#2-B2-C#3-D#3-E3-G3-G#3
/// Notes span from G#2 (MIDI note 44, ~103.83 Hz) to G#3 (MIDI note 56, ~207.65 Hz)
/// Bass range with a dark, mysterious quality
pub const GSHARP2_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([GSHARP2, ASHARP2, B2, CSHARP3, DSHARP3, E3, G3, GSHARP3]);

/// A2 harmonic minor scale: A2-B2-C3-D3-E3-F3-G#3-A3
/// Notes span from A2 (MIDI note 45, 110.00 Hz) to A3 (MIDI note 57, 220.00 Hz)
/// A2 is the open A string on a guitar, a common range for guitar bass lines
pub const A2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A2, B2, C3, D3, E3, F3, GSHARP3, A3]);

/// A#2 harmonic minor scale: A#2-C3-C#3-D#3-F3-F#3-A3-A#3
/// Notes span from A#2 (MIDI note 46, ~116.54 Hz) to A#3 (MIDI note 58, ~233.08 Hz)
/// Bass range with a rich, dramatic character
pub const ASHARP2_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([ASHARP2, C3, CSHARP3, DSHARP3, F3, FSHARP3, A3, ASHARP3]);

/// B2 harmonic minor scale: B2-C#3-D3-E3-F#3-G3-A#3-B3
/// Notes span from B2 (MIDI note 47, ~123.47 Hz) to B3 (MIDI note 59, ~246.94 Hz)
/// Bass range with an exotic, tense quality
pub const B2_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([B2, CSHARP3, D3, E3, FSHARP3, G3, ASHARP3, B3]);

// Octave 3 harmonic minor scales
// These scales start from notes in MIDI octave 3 (notes 48-59)
pub const C3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C3, D3, DSHARP3, F3, G3, GSHARP3, B3, C4]);
pub const CSHARP3_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([CSHARP3, DSHARP3, E3, FSHARP3, GSHARP3, A3, C4, CSHARP4]);
pub const D3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D3, E3, F3, G3, A3, ASHARP3, CSHARP4, D4]);
pub const DSHARP3_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([DSHARP3, F3, FSHARP3, GSHARP3, ASHARP3, B3, D4, DSHARP4]);
pub const E3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E3, FSHARP3, G3, A3, B3, C4, DSHARP4, E4]);
pub const F3_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([F3, G3, GSHARP3, ASHARP3, C4, CSHARP4, E4, F4]);
pub const FSHARP3_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([FSHARP3, GSHARP3, A3, B3, CSHARP4, D4, F4, FSHARP4]);
pub const G3_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([G3, A3, ASHARP3, C4, D4, DSHARP4, FSHARP4, G4]);
pub const GSHARP3_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([GSHARP3, ASHARP3, B3, CSHARP4, DSHARP4, E4, G4, GSHARP4]);
pub const A3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A3, B3, C4, D4, E4, F4, GSHARP4, A4]);
pub const ASHARP3_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([ASHARP3, C4, CSHARP4, DSHARP4, F4, FSHARP4, A4, ASHARP4]);
pub const B3_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([B3, CSHARP4, D4, E4, FSHARP4, G4, ASHARP4, B4]);

// Octave 4 harmonic minor scales
// These scales start from notes in MIDI octave 4 (notes 60-71), which includes A4 (440Hz concert pitch)
pub const C4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C4, D4, DSHARP4, F4, G4, GSHARP4, B4, C5]);
pub const CSHARP4_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([CSHARP4, DSHARP4, E4, FSHARP4, GSHARP4, A4, C5, CSHARP5]);
pub const D4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D4, E4, F4, G4, A4, ASHARP4, CSHARP5, D5]);
pub const DSHARP4_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([DSHARP4, F4, FSHARP4, GSHARP4, ASHARP4, B4, D5, DSHARP5]);
pub const E4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E4, FSHARP4, G4, A4, B4, C5, DSHARP5, E5]);
pub const F4_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([F4, G4, GSHARP4, ASHARP4, C5, CSHARP5, E5, F5]);
pub const FSHARP4_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([FSHARP4, GSHARP4, A4, B4, CSHARP5, D5, F5, FSHARP5]);
pub const G4_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([G4, A4, ASHARP4, C5, D5, DSHARP5, FSHARP5, G5]);
pub const GSHARP4_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([GSHARP4, ASHARP4, B4, CSHARP5, DSHARP5, E5, G5, GSHARP5]);
pub const A4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A4, B4, C5, D5, E5, F5, GSHARP5, A5]);
pub const ASHARP4_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([ASHARP4, C5, CSHARP5, DSHARP5, F5, FSHARP5, A5, ASHARP5]);
pub const B4_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([B4, CSHARP5, D5, E5, FSHARP5, G5, ASHARP5, B5]);

// Octave 5 harmonic minor scales
// These scales start from notes in MIDI octave 5 (notes 72-83)
pub const C5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C5, D5, DSHARP5, F5, G5, GSHARP5, B5, C6]);
pub const CSHARP5_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([CSHARP5, DSHARP5, E5, FSHARP5, GSHARP5, A5, C6, CSHARP6]);
pub const D5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D5, E5, F5, G5, A5, ASHARP5, CSHARP6, D6]);
pub const DSHARP5_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([DSHARP5, F5, FSHARP5, GSHARP5, ASHARP5, B5, D6, DSHARP6]);
pub const E5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E5, FSHARP5, G5, A5, B5, C6, DSHARP6, E6]);
pub const F5_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([F5, G5, GSHARP5, ASHARP5, C6, CSHARP6, E6, F6]);
pub const FSHARP5_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([FSHARP5, GSHARP5, A5, B5, CSHARP6, D6, F6, FSHARP6]);
pub const G5_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([G5, A5, ASHARP5, C6, D6, DSHARP6, FSHARP6, G6]);
pub const GSHARP5_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([GSHARP5, ASHARP5, B5, CSHARP6, DSHARP6, E6, G6, GSHARP6]);
pub const A5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A5, B5, C6, D6, E6, F6, GSHARP6, A6]);
pub const ASHARP5_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([ASHARP5, C6, CSHARP6, DSHARP6, F6, FSHARP6, A6, ASHARP6]);
pub const B5_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([B5, CSHARP6, D6, E6, FSHARP6, G6, ASHARP6, B6]);

// Octave 6 harmonic minor scales
// These scales start from notes in MIDI octave 6 (notes 84-95)
pub const C6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C6, D6, DSHARP6, F6, G6, GSHARP6, B6, C7]);
pub const CSHARP6_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([CSHARP6, DSHARP6, E6, FSHARP6, GSHARP6, A6, C7, CSHARP7]);
pub const D6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D6, E6, F6, G6, A6, ASHARP6, CSHARP7, D7]);
pub const DSHARP6_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([DSHARP6, F6, FSHARP6, GSHARP6, ASHARP6, B6, D7, DSHARP7]);
pub const E6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E6, FSHARP6, G6, A6, B6, C7, DSHARP7, E7]);
pub const F6_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([F6, G6, GSHARP6, ASHARP6, C7, CSHARP7, E7, F7]);
pub const FSHARP6_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([FSHARP6, GSHARP6, A6, B6, CSHARP7, D7, F7, FSHARP7]);
pub const G6_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([G6, A6, ASHARP6, C7, D7, DSHARP7, FSHARP7, G7]);
pub const GSHARP6_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([GSHARP6, ASHARP6, B6, CSHARP7, DSHARP7, E7, G7, GSHARP7]);
pub const A6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A6, B6, C7, D7, E7, F7, GSHARP7, A7]);
pub const ASHARP6_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([ASHARP6, C7, CSHARP7, DSHARP7, F7, FSHARP7, A7, ASHARP7]);
pub const B6_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([B6, CSHARP7, D7, E7, FSHARP7, G7, ASHARP7, B7]);

// Octave 7 harmonic minor scales
// These scales start from notes in MIDI octave 7 (notes 96-107)
pub const C7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C7, D7, DSHARP7, F7, G7, GSHARP7, B7, C8]);
pub const CSHARP7_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([CSHARP7, DSHARP7, E7, FSHARP7, GSHARP7, A7, C8, CSHARP8]);
pub const D7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D7, E7, F7, G7, A7, ASHARP7, CSHARP8, D8]);
pub const DSHARP7_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([DSHARP7, F7, FSHARP7, GSHARP7, ASHARP7, B7, D8, DSHARP8]);
pub const E7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E7, FSHARP7, G7, A7, B7, C8, DSHARP8, E8]);
pub const F7_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([F7, G7, GSHARP7, ASHARP7, C8, CSHARP8, E8, F8]);
pub const FSHARP7_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([FSHARP7, GSHARP7, A7, B7, CSHARP8, D8, F8, FSHARP8]);
pub const G7_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([G7, A7, ASHARP7, C8, D8, DSHARP8, FSHARP8, G8]);
pub const GSHARP7_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([GSHARP7, ASHARP7, B7, CSHARP8, DSHARP8, E8, G8, GSHARP8]);
pub const A7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A7, B7, C8, D8, E8, F8, GSHARP8, A8]);
pub const ASHARP7_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([ASHARP7, C8, CSHARP8, DSHARP8, F8, FSHARP8, A8, ASHARP8]);
pub const B7_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([B7, CSHARP8, D8, E8, FSHARP8, G8, ASHARP8, B8]);

// Octave 8 harmonic minor scales
// These scales start from notes in MIDI octave 8 (notes 108-119)
pub const C8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C8, D8, DSHARP8, F8, G8, GSHARP8, B8, C9]);
pub const CSHARP8_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([CSHARP8, DSHARP8, E8, FSHARP8, GSHARP8, A8, C9, CSHARP9]);
pub const D8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D8, E8, F8, G8, A8, ASHARP8, CSHARP9, D9]);
pub const DSHARP8_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([DSHARP8, F8, FSHARP8, GSHARP8, ASHARP8, B8, D9, DSHARP9]);
pub const E8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E8, FSHARP8, G8, A8, B8, C9, DSHARP9, E9]);
pub const F8_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([F8, G8, GSHARP8, ASHARP8, C9, CSHARP9, E9, F9]);
pub const FSHARP8_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([FSHARP8, GSHARP8, A8, B8, CSHARP9, D9, F9, FSHARP9]);
pub const G8_HARMONIC_SCALE: Scale<8> =
    Scale::harmonic([G8, A8, ASHARP8, C9, D9, DSHARP9, FSHARP9, G9]);

lazy_static! {
    /// HashMap containing all harmonic minor scales indexed by their root pitch.
    /// This collection provides quick access to any harmonic minor scale given its root note.
    ///
    /// The map includes scales for all valid MIDI note numbers, organized by octave.
    /// Each scale spans exactly one octave from its root note and features the characteristic
    /// raised 7th degree of the harmonic minor scale.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::HARMONIC_SCALES;
    /// use mozzart_std::A4;
    ///
    /// if let Some(scale) = HARMONIC_SCALES.get(&A4) {
    ///     // Use the A harmonic minor scale starting from A4
    /// }
    /// ```
    pub static ref HARMONIC_SCALES: std::collections::HashMap<Pitch, Scale<8>> = HashMap::from([
        // Canonical scales (base octave)
        (C, C_HARMONIC_SCALE),
        (CSHARP, CSHARP_HARMONIC_SCALE),
        (D, D_HARMONIC_SCALE),
        (DSHARP, DSHARP_HARMONIC_SCALE),
        (E, E_HARMONIC_SCALE),
        (F, F_HARMONIC_SCALE),
        (FSHARP, FSHARP_HARMONIC_SCALE),
        (G, G_HARMONIC_SCALE),
        (GSHARP, GSHARP_HARMONIC_SCALE),
        (A, A_HARMONIC_SCALE),
        (ASHARP, ASHARP_HARMONIC_SCALE),
        (B, B_HARMONIC_SCALE),

        // Octave 0
        (C0, C0_HARMONIC_SCALE),
        (CSHARP0, CSHARP0_HARMONIC_SCALE),
        (D0, D0_HARMONIC_SCALE),
        (DSHARP0, DSHARP0_HARMONIC_SCALE),
        (E0, E0_HARMONIC_SCALE),
        (F0, F0_HARMONIC_SCALE),
        (FSHARP0, FSHARP0_HARMONIC_SCALE),
        (G0, G0_HARMONIC_SCALE),
        (GSHARP0, GSHARP0_HARMONIC_SCALE),
        (A0, A0_HARMONIC_SCALE),
        (ASHARP0, ASHARP0_HARMONIC_SCALE),
        (B0, B0_HARMONIC_SCALE),

        // Octave 1
        (C1, C1_HARMONIC_SCALE),
        (CSHARP1, CSHARP1_HARMONIC_SCALE),
        (D1, D1_HARMONIC_SCALE),
        (DSHARP1, DSHARP1_HARMONIC_SCALE),
        (E1, E1_HARMONIC_SCALE),
        (F1, F1_HARMONIC_SCALE),
        (FSHARP1, FSHARP1_HARMONIC_SCALE),
        (G1, G1_HARMONIC_SCALE),
        (GSHARP1, GSHARP1_HARMONIC_SCALE),
        (A1, A1_HARMONIC_SCALE),
        (ASHARP1, ASHARP1_HARMONIC_SCALE),
        (B1, B1_HARMONIC_SCALE),

        // Octave 2
        (C2, C2_HARMONIC_SCALE),
        (CSHARP2, CSHARP2_HARMONIC_SCALE),
        (D2, D2_HARMONIC_SCALE),
        (DSHARP2, DSHARP2_HARMONIC_SCALE),
        (E2, E2_HARMONIC_SCALE),
        (F2, F2_HARMONIC_SCALE),
        (FSHARP2, FSHARP2_HARMONIC_SCALE),
        (G2, G2_HARMONIC_SCALE),
        (GSHARP2, GSHARP2_HARMONIC_SCALE),
        (A2, A2_HARMONIC_SCALE),
        (ASHARP2, ASHARP2_HARMONIC_SCALE),
        (B2, B2_HARMONIC_SCALE),

        // Octave 3
        (C3, C3_HARMONIC_SCALE),
        (CSHARP3, CSHARP3_HARMONIC_SCALE),
        (D3, D3_HARMONIC_SCALE),
        (DSHARP3, DSHARP3_HARMONIC_SCALE),
        (E3, E3_HARMONIC_SCALE),
        (F3, F3_HARMONIC_SCALE),
        (FSHARP3, FSHARP3_HARMONIC_SCALE),
        (G3, G3_HARMONIC_SCALE),
        (GSHARP3, GSHARP3_HARMONIC_SCALE),
        (A3, A3_HARMONIC_SCALE),
        (ASHARP3, ASHARP3_HARMONIC_SCALE),
        (B3, B3_HARMONIC_SCALE),

        // Octave 4
        (C4, C4_HARMONIC_SCALE),
        (CSHARP4, CSHARP4_HARMONIC_SCALE),
        (D4, D4_HARMONIC_SCALE),
        (DSHARP4, DSHARP4_HARMONIC_SCALE),
        (E4, E4_HARMONIC_SCALE),
        (F4, F4_HARMONIC_SCALE),
        (FSHARP4, FSHARP4_HARMONIC_SCALE),
        (G4, G4_HARMONIC_SCALE),
        (GSHARP4, GSHARP4_HARMONIC_SCALE),
        (A4, A4_HARMONIC_SCALE),
        (ASHARP4, ASHARP4_HARMONIC_SCALE),
        (B4, B4_HARMONIC_SCALE),

        // Octave 5
        (C5, C5_HARMONIC_SCALE),
        (CSHARP5, CSHARP5_HARMONIC_SCALE),
        (D5, D5_HARMONIC_SCALE),
        (DSHARP5, DSHARP5_HARMONIC_SCALE),
        (E5, E5_HARMONIC_SCALE),
        (F5, F5_HARMONIC_SCALE),
        (FSHARP5, FSHARP5_HARMONIC_SCALE),
        (G5, G5_HARMONIC_SCALE),
        (GSHARP5, GSHARP5_HARMONIC_SCALE),
        (A5, A5_HARMONIC_SCALE),
        (ASHARP5, ASHARP5_HARMONIC_SCALE),
        (B5, B5_HARMONIC_SCALE),

        // Octave 6
        (C6, C6_HARMONIC_SCALE),
        (CSHARP6, CSHARP6_HARMONIC_SCALE),
        (D6, D6_HARMONIC_SCALE),
        (DSHARP6, DSHARP6_HARMONIC_SCALE),
        (E6, E6_HARMONIC_SCALE),
        (F6, F6_HARMONIC_SCALE),
        (FSHARP6, FSHARP6_HARMONIC_SCALE),
        (G6, G6_HARMONIC_SCALE),
        (GSHARP6, GSHARP6_HARMONIC_SCALE),
        (A6, A6_HARMONIC_SCALE),
        (ASHARP6, ASHARP6_HARMONIC_SCALE),
        (B6, B6_HARMONIC_SCALE),

        // Octave 7
        (C7, C7_HARMONIC_SCALE),
        (CSHARP7, CSHARP7_HARMONIC_SCALE),
        (D7, D7_HARMONIC_SCALE),
        (DSHARP7, DSHARP7_HARMONIC_SCALE),
        (E7, E7_HARMONIC_SCALE),
        (F7, F7_HARMONIC_SCALE),
        (FSHARP7, FSHARP7_HARMONIC_SCALE),
        (G7, G7_HARMONIC_SCALE),
        (GSHARP7, GSHARP7_HARMONIC_SCALE),
        (A7, A7_HARMONIC_SCALE),
        (ASHARP7, ASHARP7_HARMONIC_SCALE),
        (B7, B7_HARMONIC_SCALE),

        // Octave 8
        (C8, C8_HARMONIC_SCALE),
        (CSHARP8, CSHARP8_HARMONIC_SCALE),
        (D8, D8_HARMONIC_SCALE),
        (DSHARP8, DSHARP8_HARMONIC_SCALE),
        (E8, E8_HARMONIC_SCALE),
        (F8, F8_HARMONIC_SCALE),
        (FSHARP8, FSHARP8_HARMONIC_SCALE),
        (G8, G8_HARMONIC_SCALE),
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harmonic_scale_steps() {
        assert_eq!(
            HARMONIC_SCALE_STEPS,
            [
                UNISON,
                TONE,
                SEMITONE,
                TONE,
                TONE,
                SEMITONE,
                MINOR_THIRD,
                SEMITONE,
            ]
        );
    }

    #[test]
    fn test_harmonic_scale_canonical() {
        assert_eq!(C_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(CSHARP_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(D_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(DSHARP_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(E_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(F_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(FSHARP_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(G_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(GSHARP_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(A_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(ASHARP_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(B_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
    }

    #[test]
    fn test_harmonic_scale_octave_0() {
        assert_eq!(C0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(CSHARP0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(D0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(DSHARP0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(E0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(F0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(FSHARP0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(G0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(GSHARP0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(A0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(ASHARP0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(B0_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
    }

    #[test]
    fn test_harmonic_scale_octave_1() {
        assert_eq!(C1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(CSHARP1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(D1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(DSHARP1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(E1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(F1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(FSHARP1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(G1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(GSHARP1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(A1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(ASHARP1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(B1_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
    }

    #[test]
    fn test_harmonic_scale_octave_2() {
        assert_eq!(C2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(CSHARP2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(D2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(DSHARP2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(E2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(F2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(FSHARP2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(G2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(GSHARP2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(A2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(ASHARP2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(B2_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
    }

    #[test]
    fn test_harmonic_scale_octave_3() {
        assert_eq!(C3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(CSHARP3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(D3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(DSHARP3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(E3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(F3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(FSHARP3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(G3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(GSHARP3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(A3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(ASHARP3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(B3_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
    }

    #[test]
    fn test_harmonic_scale_octave_4() {
        assert_eq!(C4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(CSHARP4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(D4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(DSHARP4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(E4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(F4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(FSHARP4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(G4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(GSHARP4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(A4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(ASHARP4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(B4_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
    }

    #[test]
    fn test_harmonic_scale_octave_5() {
        assert_eq!(C5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(CSHARP5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(D5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(DSHARP5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(E5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(F5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(FSHARP5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(G5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(GSHARP5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(A5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(ASHARP5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(B5_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
    }

    #[test]
    fn test_harmonic_scale_octave_6() {
        assert_eq!(C6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(CSHARP6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(D6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(DSHARP6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(E6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(F6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(FSHARP6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(G6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(GSHARP6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(A6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(ASHARP6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(B6_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
    }

    #[test]
    fn test_harmonic_scale_octave_7() {
        assert_eq!(C7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(CSHARP7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(D7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(DSHARP7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(E7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(F7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(FSHARP7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(G7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(GSHARP7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(A7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(ASHARP7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(B7_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
    }

    #[test]
    fn test_harmonic_scale_octave_8() {
        assert_eq!(C8_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(CSHARP8_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(D8_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(DSHARP8_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(E8_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(F8_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(FSHARP8_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
        assert_eq!(G8_HARMONIC_SCALE.steps(), HARMONIC_SCALE_STEPS);
    }
}
