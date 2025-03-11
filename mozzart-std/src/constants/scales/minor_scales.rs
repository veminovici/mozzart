//! Natural minor scale constants and collections
//!
//! This module provides predefined constants for all natural minor scales across the MIDI note range.
//! Each scale is represented as an array of 8 notes (one octave) starting from a specific root pitch.
//!
//! # Natural Minor Scale Structure
//! A natural minor scale (also called Aeolian mode) follows the interval pattern:
//! Whole-Half-Whole-Whole-Half-Whole-Whole, creating the following steps:
//! - Between 1-2: whole step (2 semitones)
//! - Between 2-3: half step (1 semitone)
//! - Between 3-4: whole step (2 semitones)
//! - Between 4-5: whole step (2 semitones)
//! - Between 5-6: half step (1 semitone)
//! - Between 6-7: whole step (2 semitones)
//! - Between 7-8: whole step (2 semitones)
//!
//! # Relationship to Major Scales
//! Each natural minor scale shares the same notes as a major scale starting
//! three semitones higher (or a minor third up). For example:
//! - A minor uses the same notes as C major
//! - E minor uses the same notes as G major
//! - B minor uses the same notes as D major
//!
//! # Scale Organization
//! Scales are organized in several ways:
//! 1. Octave-independent scales (e.g., `A_MINOR_SCALE`) - generic patterns
//! 2. Octave-specific scales (e.g., `A4_MINOR_SCALE`) - starting from specific octaves
//! 3. Collections in `MINOR_SCALES` - HashMap mapping root pitches to their minor scales
//!
//! # Examples
//! ```
//! use mozzart_std::{A4_MINOR_SCALE, MINOR_SCALES};
//! use mozzart_std::A4;
//!
//! // Use a predefined scale
//! let a_minor = A4_MINOR_SCALE;
//!
//! // Look up a scale by root note
//! let scale = MINOR_SCALES.get(&A4).unwrap();
//! assert_eq!(*scale, A4_MINOR_SCALE);
//! ```

use crate::constants::*;
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

/// The interval steps that define a natural minor scale (Aeolian mode)
///
/// These intervals represent the distance between consecutive notes in the scale:
/// - Root to 2nd: whole step (TONE)
/// - 2nd to 3rd: half step (SEMITONE)
/// - 3rd to 4th: whole step (TONE)
/// - 4th to 5th: whole step (TONE)
/// - 5th to 6th: half step (SEMITONE)
/// - 6th to 7th: whole step (TONE)
/// - 7th to octave: whole step (TONE)
pub const MINOR_SCALE_STEPS: [Interval; 8] =
    [UNISON, TONE, SEMITONE, TONE, TONE, SEMITONE, TONE, TONE];

// Canonical minor scales (octave-independent)
//
// These scales define the pattern of notes for each minor scale without
// being tied to a specific octave. The scales wrap around to the next octave
// to complete the 8-note pattern.

/// C minor scale: C, D, D#, F, G, G#, A#, C
pub const C_MINOR_SCALE: Scale<8> = Scale::minor([C, D, DSHARP, F, G, GSHARP, ASHARP, C0]);
/// C# minor scale: C#, D#, E, F#, G#, A, B, C#
pub const CSHARP_MINOR_SCALE: Scale<8> =
    Scale::minor([CSHARP, DSHARP, E, FSHARP, GSHARP, A, B, CSHARP0]);
/// D minor scale: D, E, F, G, A, A#, C, D
pub const D_MINOR_SCALE: Scale<8> = Scale::minor([D, E, F, G, A, ASHARP, C0, D0]);
/// D# minor scale: D#, F, F#, G#, A#, B, C#, D#
pub const DSHARP_MINOR_SCALE: Scale<8> =
    Scale::minor([DSHARP, F, FSHARP, GSHARP, ASHARP, B, CSHARP0, DSHARP0]);
/// E minor scale: E, F#, G, A, B, C, D, E
pub const E_MINOR_SCALE: Scale<8> = Scale::minor([E, FSHARP, G, A, B, C0, D0, E0]);
/// F minor scale: F, G, G#, A#, C, C#, D#, F
pub const F_MINOR_SCALE: Scale<8> = Scale::minor([F, G, GSHARP, ASHARP, C0, CSHARP0, DSHARP0, F0]);
/// F# minor scale: F#, G#, A, B, C#, D, E, F#
pub const FSHARP_MINOR_SCALE: Scale<8> =
    Scale::minor([FSHARP, GSHARP, A, B, CSHARP0, D0, E0, FSHARP0]);
/// G minor scale: G, A, A#, C, D, D#, F, G
pub const G_MINOR_SCALE: Scale<8> = Scale::minor([G, A, ASHARP, C0, D0, DSHARP0, F0, G0]);
/// G# minor scale: G#, A#, B, C#, D#, E, F#, G#
pub const GSHARP_MINOR_SCALE: Scale<8> =
    Scale::minor([GSHARP, ASHARP, B, CSHARP0, DSHARP0, E0, FSHARP0, GSHARP0]);
/// A minor scale: A, B, C, D, E, F, G, A
pub const A_MINOR_SCALE: Scale<8> = Scale::minor([A, B, C0, D0, E0, F0, G0, A0]);
/// A# minor scale: A#, C, C#, D#, F, F#, G#, A#
pub const ASHARP_MINOR_SCALE: Scale<8> =
    Scale::minor([ASHARP, C0, CSHARP0, DSHARP0, F0, FSHARP0, GSHARP0, ASHARP0]);
/// B minor scale: B, C#, D, E, F#, G, A, B
pub const B_MINOR_SCALE: Scale<8> = Scale::minor([B, CSHARP0, D0, E0, FSHARP0, G0, A0, B0]);

// Octave 0 minor scales
// These scales start from notes in MIDI octave 0 (notes 12-23)
pub const C0_MINOR_SCALE: Scale<8> = Scale::minor([C0, D0, DSHARP0, F0, G0, GSHARP0, ASHARP0, C1]);
pub const CSHARP0_MINOR_SCALE: Scale<8> =
    Scale::minor([CSHARP0, DSHARP0, E0, FSHARP0, GSHARP0, A0, B0, CSHARP1]);
pub const D0_MINOR_SCALE: Scale<8> = Scale::minor([D0, E0, F0, G0, A0, ASHARP0, C1, D1]);
pub const DSHARP0_MINOR_SCALE: Scale<8> =
    Scale::minor([DSHARP0, F0, FSHARP0, GSHARP0, ASHARP0, B0, CSHARP1, DSHARP1]);
pub const E0_MINOR_SCALE: Scale<8> = Scale::minor([E0, FSHARP0, G0, A0, B0, C1, D1, E1]);
pub const F0_MINOR_SCALE: Scale<8> =
    Scale::minor([F0, G0, GSHARP0, ASHARP0, C1, CSHARP1, DSHARP1, F1]);
pub const FSHARP0_MINOR_SCALE: Scale<8> =
    Scale::minor([FSHARP0, GSHARP0, A0, B0, CSHARP1, D1, E1, FSHARP1]);
pub const G0_MINOR_SCALE: Scale<8> = Scale::minor([G0, A0, ASHARP0, C1, D1, DSHARP1, F1, G1]);
pub const GSHARP0_MINOR_SCALE: Scale<8> =
    Scale::minor([GSHARP0, ASHARP0, B0, CSHARP1, DSHARP1, E1, FSHARP1, GSHARP1]);
pub const A0_MINOR_SCALE: Scale<8> = Scale::minor([A0, B0, C1, D1, E1, F1, G1, A1]);
pub const ASHARP0_MINOR_SCALE: Scale<8> =
    Scale::minor([ASHARP0, C1, CSHARP1, DSHARP1, F1, FSHARP1, GSHARP1, ASHARP1]);
pub const B0_MINOR_SCALE: Scale<8> = Scale::minor([B0, CSHARP1, D1, E1, FSHARP1, G1, A1, B1]);

// Octave 1 minor scales
// These scales start from notes in MIDI octave 1 (notes 24-35)
pub const C1_MINOR_SCALE: Scale<8> = Scale::minor([C1, D1, DSHARP1, F1, G1, GSHARP1, ASHARP1, C2]);
pub const CSHARP1_MINOR_SCALE: Scale<8> =
    Scale::minor([CSHARP1, DSHARP1, E1, FSHARP1, GSHARP1, A1, B1, CSHARP2]);
pub const D1_MINOR_SCALE: Scale<8> = Scale::minor([D1, E1, F1, G1, A1, ASHARP1, C2, D2]);
pub const DSHARP1_MINOR_SCALE: Scale<8> =
    Scale::minor([DSHARP1, F1, FSHARP1, GSHARP1, ASHARP1, B1, CSHARP2, DSHARP2]);
pub const E1_MINOR_SCALE: Scale<8> = Scale::minor([E1, FSHARP1, G1, A1, B1, C2, D2, E2]);
pub const F1_MINOR_SCALE: Scale<8> =
    Scale::minor([F1, G1, GSHARP1, ASHARP1, C2, CSHARP2, DSHARP2, F2]);
pub const FSHARP1_MINOR_SCALE: Scale<8> =
    Scale::minor([FSHARP1, GSHARP1, A1, B1, CSHARP2, D2, E2, FSHARP2]);
pub const G1_MINOR_SCALE: Scale<8> = Scale::minor([G1, A1, ASHARP1, C2, D2, DSHARP2, F2, G2]);
pub const GSHARP1_MINOR_SCALE: Scale<8> =
    Scale::minor([GSHARP1, ASHARP1, B1, CSHARP2, DSHARP2, E2, FSHARP2, GSHARP2]);
pub const A1_MINOR_SCALE: Scale<8> = Scale::minor([A1, B1, C2, D2, E2, F2, G2, A2]);
pub const ASHARP1_MINOR_SCALE: Scale<8> =
    Scale::minor([ASHARP1, C2, CSHARP2, DSHARP2, F2, FSHARP2, GSHARP2, ASHARP2]);
pub const B1_MINOR_SCALE: Scale<8> = Scale::minor([B1, CSHARP2, D2, E2, FSHARP2, G2, A2, B2]);

// Octave 2 minor scales
// These scales start from notes in MIDI octave 2 (notes 36-47)
pub const C2_MINOR_SCALE: Scale<8> = Scale::minor([C2, D2, DSHARP2, F2, G2, GSHARP2, ASHARP2, C3]);
pub const CSHARP2_MINOR_SCALE: Scale<8> =
    Scale::minor([CSHARP2, DSHARP2, E2, FSHARP2, GSHARP2, A2, B2, CSHARP3]);
pub const D2_MINOR_SCALE: Scale<8> = Scale::minor([D2, E2, F2, G2, A2, ASHARP2, C3, D3]);
pub const DSHARP2_MINOR_SCALE: Scale<8> =
    Scale::minor([DSHARP2, F2, FSHARP2, GSHARP2, ASHARP2, B2, CSHARP3, DSHARP3]);
pub const E2_MINOR_SCALE: Scale<8> = Scale::minor([E2, FSHARP2, G2, A2, B2, C3, D3, E3]);
pub const F2_MINOR_SCALE: Scale<8> =
    Scale::minor([F2, G2, GSHARP2, ASHARP2, C3, CSHARP3, DSHARP3, F3]);
pub const FSHARP2_MINOR_SCALE: Scale<8> =
    Scale::minor([FSHARP2, GSHARP2, A2, B2, CSHARP3, D3, E3, FSHARP3]);
pub const G2_MINOR_SCALE: Scale<8> = Scale::minor([G2, A2, ASHARP2, C3, D3, DSHARP3, F3, G3]);
pub const GSHARP2_MINOR_SCALE: Scale<8> =
    Scale::minor([GSHARP2, ASHARP2, B2, CSHARP3, DSHARP3, E3, FSHARP3, GSHARP3]);
pub const A2_MINOR_SCALE: Scale<8> = Scale::minor([A2, B2, C3, D3, E3, F3, G3, A3]);
pub const ASHARP2_MINOR_SCALE: Scale<8> =
    Scale::minor([ASHARP2, C3, CSHARP3, DSHARP3, F3, FSHARP3, GSHARP3, ASHARP3]);
pub const B2_MINOR_SCALE: Scale<8> = Scale::minor([B2, CSHARP3, D3, E3, FSHARP3, G3, A3, B3]);

// Octave 3 minor scales
// These scales start from notes in MIDI octave 3 (notes 48-59)
pub const C3_MINOR_SCALE: Scale<8> = Scale::minor([C3, D3, DSHARP3, F3, G3, GSHARP3, ASHARP3, C4]);
pub const CSHARP3_MINOR_SCALE: Scale<8> =
    Scale::minor([CSHARP3, DSHARP3, E3, FSHARP3, GSHARP3, A3, B3, CSHARP4]);
pub const D3_MINOR_SCALE: Scale<8> = Scale::minor([D3, E3, F3, G3, A3, ASHARP3, C4, D4]);
pub const DSHARP3_MINOR_SCALE: Scale<8> =
    Scale::minor([DSHARP3, F3, FSHARP3, GSHARP3, ASHARP3, B3, CSHARP4, DSHARP4]);
pub const E3_MINOR_SCALE: Scale<8> = Scale::minor([E3, FSHARP3, G3, A3, B3, C4, D4, E4]);
pub const F3_MINOR_SCALE: Scale<8> =
    Scale::minor([F3, G3, GSHARP3, ASHARP3, C4, CSHARP4, DSHARP4, F4]);
pub const FSHARP3_MINOR_SCALE: Scale<8> =
    Scale::minor([FSHARP3, GSHARP3, A3, B3, CSHARP4, D4, E4, FSHARP4]);
pub const G3_MINOR_SCALE: Scale<8> = Scale::minor([G3, A3, ASHARP3, C4, D4, DSHARP4, F4, G4]);
pub const GSHARP3_MINOR_SCALE: Scale<8> =
    Scale::minor([GSHARP3, ASHARP3, B3, CSHARP4, DSHARP4, E4, FSHARP4, GSHARP4]);
pub const A3_MINOR_SCALE: Scale<8> = Scale::minor([A3, B3, C4, D4, E4, F4, G4, A4]);
pub const ASHARP3_MINOR_SCALE: Scale<8> =
    Scale::minor([ASHARP3, C4, CSHARP4, DSHARP4, F4, FSHARP4, GSHARP4, ASHARP4]);
pub const B3_MINOR_SCALE: Scale<8> = Scale::minor([B3, CSHARP4, D4, E4, FSHARP4, G4, A4, B4]);

// Octave 4 minor scales
// These scales start from notes in MIDI octave 4 (notes 60-71), which includes A4 (440Hz concert pitch)
pub const C4_MINOR_SCALE: Scale<8> = Scale::minor([C4, D4, DSHARP4, F4, G4, GSHARP4, ASHARP4, C5]);
pub const CSHARP4_MINOR_SCALE: Scale<8> =
    Scale::minor([CSHARP4, DSHARP4, E4, FSHARP4, GSHARP4, A4, B4, CSHARP5]);
pub const D4_MINOR_SCALE: Scale<8> = Scale::minor([D4, E4, F4, G4, A4, ASHARP4, C5, D5]);
pub const DSHARP4_MINOR_SCALE: Scale<8> =
    Scale::minor([DSHARP4, F4, FSHARP4, GSHARP4, ASHARP4, B4, CSHARP5, DSHARP5]);
pub const E4_MINOR_SCALE: Scale<8> = Scale::minor([E4, FSHARP4, G4, A4, B4, C5, D5, E5]);
pub const F4_MINOR_SCALE: Scale<8> =
    Scale::minor([F4, G4, GSHARP4, ASHARP4, C5, CSHARP5, DSHARP5, F5]);
pub const FSHARP4_MINOR_SCALE: Scale<8> =
    Scale::minor([FSHARP4, GSHARP4, A4, B4, CSHARP5, D5, E5, FSHARP5]);
pub const G4_MINOR_SCALE: Scale<8> = Scale::minor([G4, A4, ASHARP4, C5, D5, DSHARP5, F5, G5]);
pub const GSHARP4_MINOR_SCALE: Scale<8> =
    Scale::minor([GSHARP4, ASHARP4, B4, CSHARP5, DSHARP5, E5, FSHARP5, GSHARP5]);
pub const A4_MINOR_SCALE: Scale<8> = Scale::minor([A4, B4, C5, D5, E5, F5, G5, A5]);
pub const ASHARP4_MINOR_SCALE: Scale<8> =
    Scale::minor([ASHARP4, C5, CSHARP5, DSHARP5, F5, FSHARP5, GSHARP5, ASHARP5]);
pub const B4_MINOR_SCALE: Scale<8> = Scale::minor([B4, CSHARP5, D5, E5, FSHARP5, G5, A5, B5]);

// Octave 5 minor scales
// These scales start from notes in MIDI octave 5 (notes 72-83)
pub const C5_MINOR_SCALE: Scale<8> = Scale::minor([C5, D5, DSHARP5, F5, G5, GSHARP5, ASHARP5, C6]);
pub const CSHARP5_MINOR_SCALE: Scale<8> =
    Scale::minor([CSHARP5, DSHARP5, E5, FSHARP5, GSHARP5, A5, B5, CSHARP6]);
pub const D5_MINOR_SCALE: Scale<8> = Scale::minor([D5, E5, F5, G5, A5, ASHARP5, C6, D6]);
pub const DSHARP5_MINOR_SCALE: Scale<8> =
    Scale::minor([DSHARP5, F5, FSHARP5, GSHARP5, ASHARP5, B5, CSHARP6, DSHARP6]);
pub const E5_MINOR_SCALE: Scale<8> = Scale::minor([E5, FSHARP5, G5, A5, B5, C6, D6, E6]);
pub const F5_MINOR_SCALE: Scale<8> =
    Scale::minor([F5, G5, GSHARP5, ASHARP5, C6, CSHARP6, DSHARP6, F6]);
pub const FSHARP5_MINOR_SCALE: Scale<8> =
    Scale::minor([FSHARP5, GSHARP5, A5, B5, CSHARP6, D6, E6, FSHARP6]);
pub const G5_MINOR_SCALE: Scale<8> = Scale::minor([G5, A5, ASHARP5, C6, D6, DSHARP6, F6, G6]);
pub const GSHARP5_MINOR_SCALE: Scale<8> =
    Scale::minor([GSHARP5, ASHARP5, B5, CSHARP6, DSHARP6, E6, FSHARP6, GSHARP6]);
pub const A5_MINOR_SCALE: Scale<8> = Scale::minor([A5, B5, C6, D6, E6, F6, G6, A6]);
pub const ASHARP5_MINOR_SCALE: Scale<8> =
    Scale::minor([ASHARP5, C6, CSHARP6, DSHARP6, F6, FSHARP6, GSHARP6, ASHARP6]);
pub const B5_MINOR_SCALE: Scale<8> = Scale::minor([B5, CSHARP6, D6, E6, FSHARP6, G6, A6, B6]);

// Octave 6 minor scales
// These scales start from notes in MIDI octave 6 (notes 84-95)
pub const C6_MINOR_SCALE: Scale<8> = Scale::minor([C6, D6, DSHARP6, F6, G6, GSHARP6, ASHARP6, C7]);
pub const CSHARP6_MINOR_SCALE: Scale<8> =
    Scale::minor([CSHARP6, DSHARP6, E6, FSHARP6, GSHARP6, A6, B6, CSHARP7]);
pub const D6_MINOR_SCALE: Scale<8> = Scale::minor([D6, E6, F6, G6, A6, ASHARP6, C7, D7]);
pub const DSHARP6_MINOR_SCALE: Scale<8> =
    Scale::minor([DSHARP6, F6, FSHARP6, GSHARP6, ASHARP6, B6, CSHARP7, DSHARP7]);
pub const E6_MINOR_SCALE: Scale<8> = Scale::minor([E6, FSHARP6, G6, A6, B6, C7, D7, E7]);
pub const F6_MINOR_SCALE: Scale<8> =
    Scale::minor([F6, G6, GSHARP6, ASHARP6, C7, CSHARP7, DSHARP7, F7]);
pub const FSHARP6_MINOR_SCALE: Scale<8> =
    Scale::minor([FSHARP6, GSHARP6, A6, B6, CSHARP7, D7, E7, FSHARP7]);
pub const G6_MINOR_SCALE: Scale<8> = Scale::minor([G6, A6, ASHARP6, C7, D7, DSHARP7, F7, G7]);
pub const GSHARP6_MINOR_SCALE: Scale<8> =
    Scale::minor([GSHARP6, ASHARP6, B6, CSHARP7, DSHARP7, E7, FSHARP7, GSHARP7]);
pub const A6_MINOR_SCALE: Scale<8> = Scale::minor([A6, B6, C7, D7, E7, F7, G7, A7]);
pub const ASHARP6_MINOR_SCALE: Scale<8> =
    Scale::minor([ASHARP6, C7, CSHARP7, DSHARP7, F7, FSHARP7, GSHARP7, ASHARP7]);
pub const B6_MINOR_SCALE: Scale<8> = Scale::minor([B6, CSHARP7, D7, E7, FSHARP7, G7, A7, B7]);

// Octave 7 minor scales
// These scales start from notes in MIDI octave 7 (notes 96-107)
pub const C7_MINOR_SCALE: Scale<8> = Scale::minor([C7, D7, DSHARP7, F7, G7, GSHARP7, ASHARP7, C8]);
pub const CSHARP7_MINOR_SCALE: Scale<8> =
    Scale::minor([CSHARP7, DSHARP7, E7, FSHARP7, GSHARP7, A7, B7, CSHARP8]);
pub const D7_MINOR_SCALE: Scale<8> = Scale::minor([D7, E7, F7, G7, A7, ASHARP7, C8, D8]);
pub const DSHARP7_MINOR_SCALE: Scale<8> =
    Scale::minor([DSHARP7, F7, FSHARP7, GSHARP7, ASHARP7, B7, CSHARP8, DSHARP8]);
pub const E7_MINOR_SCALE: Scale<8> = Scale::minor([E7, FSHARP7, G7, A7, B7, C8, D8, E8]);
pub const F7_MINOR_SCALE: Scale<8> =
    Scale::minor([F7, G7, GSHARP7, ASHARP7, C8, CSHARP8, DSHARP8, F8]);
pub const FSHARP7_MINOR_SCALE: Scale<8> =
    Scale::minor([FSHARP7, GSHARP7, A7, B7, CSHARP8, D8, E8, FSHARP8]);
pub const G7_MINOR_SCALE: Scale<8> = Scale::minor([G7, A7, ASHARP7, C8, D8, DSHARP8, F8, G8]);
pub const GSHARP7_MINOR_SCALE: Scale<8> =
    Scale::minor([GSHARP7, ASHARP7, B7, CSHARP8, DSHARP8, E8, FSHARP8, GSHARP8]);
pub const A7_MINOR_SCALE: Scale<8> = Scale::minor([A7, B7, C8, D8, E8, F8, G8, A8]);
pub const ASHARP7_MINOR_SCALE: Scale<8> =
    Scale::minor([ASHARP7, C8, CSHARP8, DSHARP8, F8, FSHARP8, GSHARP8, ASHARP8]);
pub const B7_MINOR_SCALE: Scale<8> = Scale::minor([B7, CSHARP8, D8, E8, FSHARP8, G8, A8, B8]);

// Octave 8 minor scales
// These scales start from notes in MIDI octave 8 (notes 108-119)
// Note: Only scales up to G8 are defined, as G8 is the highest note in the standard MIDI range
pub const C8_MINOR_SCALE: Scale<8> = Scale::minor([C8, D8, DSHARP8, F8, G8, GSHARP8, ASHARP8, C9]);
pub const CSHARP8_MINOR_SCALE: Scale<8> =
    Scale::minor([CSHARP8, DSHARP8, E8, FSHARP8, GSHARP8, A8, B8, CSHARP9]);
pub const D8_MINOR_SCALE: Scale<8> = Scale::minor([D8, E8, F8, G8, A8, ASHARP8, C9, D9]);
pub const DSHARP8_MINOR_SCALE: Scale<8> =
    Scale::minor([DSHARP8, F8, FSHARP8, GSHARP8, ASHARP8, B8, CSHARP9, DSHARP9]);
pub const E8_MINOR_SCALE: Scale<8> = Scale::minor([E8, FSHARP8, G8, A8, B8, C9, D9, E9]);
pub const F8_MINOR_SCALE: Scale<8> =
    Scale::minor([F8, G8, GSHARP8, ASHARP8, C9, CSHARP9, DSHARP9, F9]);
pub const FSHARP8_MINOR_SCALE: Scale<8> =
    Scale::minor([FSHARP8, GSHARP8, A8, B8, CSHARP9, D9, E9, FSHARP9]);
pub const G8_MINOR_SCALE: Scale<8> = Scale::minor([G8, A8, ASHARP8, C9, D9, DSHARP9, F9, G9]);

lazy_static! {
    /// A HashMap that maps each pitch to its corresponding minor scale
    ///
    /// This collection provides a convenient way to look up a minor scale by its root note.
    /// It contains all minor scales from all octaves (canonical scales and octaves 0-8).
    ///
    /// # Example
    /// ```
    /// use mozzart_std::{MINOR_SCALES, A4, A4_MINOR_SCALE};
    ///
    /// let scale = MINOR_SCALES.get(&A4).unwrap();
    /// assert_eq!(*scale, A4_MINOR_SCALE);
    /// ```
    pub static ref MINOR_SCALES: HashMap<Pitch, Scale<8>> = HashMap::from([
        // Canonical scales (base octave)
        (C, C_MINOR_SCALE),
        (CSHARP, CSHARP_MINOR_SCALE),
        (D, D_MINOR_SCALE),
        (DSHARP, DSHARP_MINOR_SCALE),
        (E, E_MINOR_SCALE),
        (F, F_MINOR_SCALE),
        (FSHARP, FSHARP_MINOR_SCALE),
        (G, G_MINOR_SCALE),
        (GSHARP, GSHARP_MINOR_SCALE),
        (A, A_MINOR_SCALE),
        (ASHARP, ASHARP_MINOR_SCALE),
        (B, B_MINOR_SCALE),
        // Octave 0
        (C0, C0_MINOR_SCALE),
        (CSHARP0, CSHARP0_MINOR_SCALE),
        (D0, D0_MINOR_SCALE),
        (DSHARP0, DSHARP0_MINOR_SCALE),
        (E0, E0_MINOR_SCALE),
        (F0, F0_MINOR_SCALE),
        (FSHARP0, FSHARP0_MINOR_SCALE),
        (G0, G0_MINOR_SCALE),
        (GSHARP0, GSHARP0_MINOR_SCALE),
        (A0, A0_MINOR_SCALE),
        (ASHARP0, ASHARP0_MINOR_SCALE),
        (B0, B0_MINOR_SCALE),
        // Octave 1
        (C1, C1_MINOR_SCALE),
        (CSHARP1, CSHARP1_MINOR_SCALE),
        (D1, D1_MINOR_SCALE),
        (DSHARP1, DSHARP1_MINOR_SCALE),
        (E1, E1_MINOR_SCALE),
        (F1, F1_MINOR_SCALE),
        (FSHARP1, FSHARP1_MINOR_SCALE),
        (G1, G1_MINOR_SCALE),
        (GSHARP1, GSHARP1_MINOR_SCALE),
        (A1, A1_MINOR_SCALE),
        (ASHARP1, ASHARP1_MINOR_SCALE),
        (B1, B1_MINOR_SCALE),
        // Octave 2
        (C2, C2_MINOR_SCALE),
        (CSHARP2, CSHARP2_MINOR_SCALE),
        (D2, D2_MINOR_SCALE),
        (DSHARP2, DSHARP2_MINOR_SCALE),
        (E2, E2_MINOR_SCALE),
        (F2, F2_MINOR_SCALE),
        (FSHARP2, FSHARP2_MINOR_SCALE),
        (G2, G2_MINOR_SCALE),
        (GSHARP2, GSHARP2_MINOR_SCALE),
        (A2, A2_MINOR_SCALE),
        (ASHARP2, ASHARP2_MINOR_SCALE),
        (B2, B2_MINOR_SCALE),
        // Octave 3
        (C3, C3_MINOR_SCALE),
        (CSHARP3, CSHARP3_MINOR_SCALE),
        (D3, D3_MINOR_SCALE),
        (DSHARP3, DSHARP3_MINOR_SCALE),
        (E3, E3_MINOR_SCALE),
        (F3, F3_MINOR_SCALE),
        (FSHARP3, FSHARP3_MINOR_SCALE),
        (G3, G3_MINOR_SCALE),
        (GSHARP3, GSHARP3_MINOR_SCALE),
        (A3, A3_MINOR_SCALE),
        (ASHARP3, ASHARP3_MINOR_SCALE),
        (B3, B3_MINOR_SCALE),
        // Octave 4
        (C4, C4_MINOR_SCALE),
        (CSHARP4, CSHARP4_MINOR_SCALE),
        (D4, D4_MINOR_SCALE),
        (DSHARP4, DSHARP4_MINOR_SCALE),
        (E4, E4_MINOR_SCALE),
        (F4, F4_MINOR_SCALE),
        (FSHARP4, FSHARP4_MINOR_SCALE),
        (G4, G4_MINOR_SCALE),
        (GSHARP4, GSHARP4_MINOR_SCALE),
        (A4, A4_MINOR_SCALE),
        (ASHARP4, ASHARP4_MINOR_SCALE),
        (B4, B4_MINOR_SCALE),
        // Octave 5
        (C5, C5_MINOR_SCALE),
        (CSHARP5, CSHARP5_MINOR_SCALE),
        (D5, D5_MINOR_SCALE),
        (DSHARP5, DSHARP5_MINOR_SCALE),
        (E5, E5_MINOR_SCALE),
        (F5, F5_MINOR_SCALE),
        (FSHARP5, FSHARP5_MINOR_SCALE),
        (G5, G5_MINOR_SCALE),
        (GSHARP5, GSHARP5_MINOR_SCALE),
        (A5, A5_MINOR_SCALE),
        (ASHARP5, ASHARP5_MINOR_SCALE),
        (B5, B5_MINOR_SCALE),
        // Octave 6
        (C6, C6_MINOR_SCALE),
        (CSHARP6, CSHARP6_MINOR_SCALE),
        (D6, D6_MINOR_SCALE),
        (DSHARP6, DSHARP6_MINOR_SCALE),
        (E6, E6_MINOR_SCALE),
        (F6, F6_MINOR_SCALE),
        (FSHARP6, FSHARP6_MINOR_SCALE),
        (G6, G6_MINOR_SCALE),
        (GSHARP6, GSHARP6_MINOR_SCALE),
        (A6, A6_MINOR_SCALE),
        (ASHARP6, ASHARP6_MINOR_SCALE),
        (B6, B6_MINOR_SCALE),
        // Octave 7
        (C7, C7_MINOR_SCALE),
        (CSHARP7, CSHARP7_MINOR_SCALE),
        (D7, D7_MINOR_SCALE),
        (DSHARP7, DSHARP7_MINOR_SCALE),
        (E7, E7_MINOR_SCALE),
        (F7, F7_MINOR_SCALE),
        (FSHARP7, FSHARP7_MINOR_SCALE),
        (G7, G7_MINOR_SCALE),
        (GSHARP7, GSHARP7_MINOR_SCALE),
        (A7, A7_MINOR_SCALE),
        (ASHARP7, ASHARP7_MINOR_SCALE),
        (B7, B7_MINOR_SCALE),
        // Octave 8
        (C8, C8_MINOR_SCALE),
        (CSHARP8, CSHARP8_MINOR_SCALE),
        (D8, D8_MINOR_SCALE),
        (DSHARP8, DSHARP8_MINOR_SCALE),
        (E8, E8_MINOR_SCALE),
        (F8, F8_MINOR_SCALE),
        (FSHARP8, FSHARP8_MINOR_SCALE),
        (G8, G8_MINOR_SCALE),
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that the minor scale steps are correctly defined
    #[test]
    fn test_minor_scale_steps() {
        assert_eq!(
            MINOR_SCALE_STEPS,
            [UNISON, TONE, SEMITONE, TONE, TONE, SEMITONE, TONE, TONE]
        );
    }

    /// Test that all octave 0 minor scales follow the correct interval pattern
    #[test]
    fn test_minor_scales_octave_0() {
        assert_eq!(C0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
    }

    /// Test that all canonical minor scales follow the correct interval pattern
    #[test]
    fn test_minor_scales_canonical() {
        assert_eq!(C_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
    }

    /// Test that all canonical minor scales can be retrieved from the MINOR_SCALES HashMap
    #[test]
    fn test_scales_contains_canonical() {
        assert_eq!(MINOR_SCALES.get(&C), Some(&C_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&CSHARP), Some(&CSHARP_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&D), Some(&D_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&DSHARP), Some(&DSHARP_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&E), Some(&E_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&F), Some(&F_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&FSHARP), Some(&FSHARP_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&G), Some(&G_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&GSHARP), Some(&GSHARP_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&A), Some(&A_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&ASHARP), Some(&ASHARP_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&B), Some(&B_MINOR_SCALE));
    }

    /// Test that all octave 0 minor scales can be retrieved from the MINOR_SCALES HashMap
    #[test]
    fn test_scales_contains_octave_0() {
        assert_eq!(MINOR_SCALES.get(&C0), Some(&C0_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&CSHARP0), Some(&CSHARP0_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&D0), Some(&D0_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&DSHARP0), Some(&DSHARP0_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&E0), Some(&E0_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&F0), Some(&F0_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&FSHARP0), Some(&FSHARP0_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&G0), Some(&G0_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&GSHARP0), Some(&GSHARP0_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&A0), Some(&A0_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&ASHARP0), Some(&ASHARP0_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&B0), Some(&B0_MINOR_SCALE));
    }

    #[test]
    fn test_minor_scales_octave_1() {
        assert_eq!(C1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_1() {
        assert_eq!(MINOR_SCALES.get(&C1), Some(&C1_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&CSHARP1), Some(&CSHARP1_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&D1), Some(&D1_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&DSHARP1), Some(&DSHARP1_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&E1), Some(&E1_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&F1), Some(&F1_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&FSHARP1), Some(&FSHARP1_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&G1), Some(&G1_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&GSHARP1), Some(&GSHARP1_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&A1), Some(&A1_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&ASHARP1), Some(&ASHARP1_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&B1), Some(&B1_MINOR_SCALE));
    }

    #[test]
    fn test_minor_scales_octave_2() {
        assert_eq!(C2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B2_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_2() {
        assert_eq!(MINOR_SCALES.get(&C2), Some(&C2_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&CSHARP2), Some(&CSHARP2_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&D2), Some(&D2_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&DSHARP2), Some(&DSHARP2_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&E2), Some(&E2_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&F2), Some(&F2_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&FSHARP2), Some(&FSHARP2_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&G2), Some(&G2_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&GSHARP2), Some(&GSHARP2_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&A2), Some(&A2_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&ASHARP2), Some(&ASHARP2_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&B2), Some(&B2_MINOR_SCALE));
    }

    #[test]
    fn test_minor_scales_octave_3() {
        assert_eq!(C3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B3_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_3() {
        assert_eq!(MINOR_SCALES.get(&C3), Some(&C3_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&CSHARP3), Some(&CSHARP3_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&D3), Some(&D3_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&DSHARP3), Some(&DSHARP3_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&E3), Some(&E3_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&F3), Some(&F3_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&FSHARP3), Some(&FSHARP3_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&G3), Some(&G3_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&GSHARP3), Some(&GSHARP3_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&A3), Some(&A3_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&ASHARP3), Some(&ASHARP3_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&B3), Some(&B3_MINOR_SCALE));
    }

    #[test]
    fn test_minor_scales_octave_4() {
        assert_eq!(C4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B4_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_4() {
        assert_eq!(MINOR_SCALES.get(&C4), Some(&C4_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&CSHARP4), Some(&CSHARP4_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&D4), Some(&D4_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&DSHARP4), Some(&DSHARP4_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&E4), Some(&E4_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&F4), Some(&F4_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&FSHARP4), Some(&FSHARP4_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&G4), Some(&G4_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&GSHARP4), Some(&GSHARP4_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&A4), Some(&A4_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&ASHARP4), Some(&ASHARP4_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&B4), Some(&B4_MINOR_SCALE));
    }

    #[test]
    fn test_minor_scales_octave_5() {
        assert_eq!(C5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B5_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_5() {
        assert_eq!(MINOR_SCALES.get(&C5), Some(&C5_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&CSHARP5), Some(&CSHARP5_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&D5), Some(&D5_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&DSHARP5), Some(&DSHARP5_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&E5), Some(&E5_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&F5), Some(&F5_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&FSHARP5), Some(&FSHARP5_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&G5), Some(&G5_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&GSHARP5), Some(&GSHARP5_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&A5), Some(&A5_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&ASHARP5), Some(&ASHARP5_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&B5), Some(&B5_MINOR_SCALE));
    }

    #[test]
    fn test_minor_scales_octave_6() {
        assert_eq!(C6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B6_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_6() {
        assert_eq!(MINOR_SCALES.get(&C6), Some(&C6_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&CSHARP6), Some(&CSHARP6_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&D6), Some(&D6_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&DSHARP6), Some(&DSHARP6_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&E6), Some(&E6_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&F6), Some(&F6_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&FSHARP6), Some(&FSHARP6_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&G6), Some(&G6_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&GSHARP6), Some(&GSHARP6_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&A6), Some(&A6_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&ASHARP6), Some(&ASHARP6_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&B6), Some(&B6_MINOR_SCALE));
    }

    #[test]
    fn test_minor_scales_octave_7() {
        assert_eq!(C7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B7_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_7() {
        assert_eq!(MINOR_SCALES.get(&C7), Some(&C7_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&CSHARP7), Some(&CSHARP7_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&D7), Some(&D7_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&DSHARP7), Some(&DSHARP7_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&E7), Some(&E7_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&F7), Some(&F7_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&FSHARP7), Some(&FSHARP7_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&G7), Some(&G7_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&GSHARP7), Some(&GSHARP7_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&A7), Some(&A7_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&ASHARP7), Some(&ASHARP7_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&B7), Some(&B7_MINOR_SCALE));
    }

    #[test]
    fn test_minor_scales_octave_8() {
        assert_eq!(C8_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP8_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D8_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP8_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E8_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F8_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP8_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G8_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_octave_8() {
        assert_eq!(MINOR_SCALES.get(&C8), Some(&C8_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&CSHARP8), Some(&CSHARP8_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&D8), Some(&D8_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&DSHARP8), Some(&DSHARP8_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&E8), Some(&E8_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&F8), Some(&F8_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&FSHARP8), Some(&FSHARP8_MINOR_SCALE));
        assert_eq!(MINOR_SCALES.get(&G8), Some(&G8_MINOR_SCALE));
    }
}
