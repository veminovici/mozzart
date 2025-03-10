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
use crate::{Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

// Octave-independent harmonic minor scales (pitch class only)
/// C harmonic minor scale (C-D-E♭-F-G-A♭-B-C)
pub const C_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C, D, E, F, G, A, B, C]);
/// C♯/D♭ harmonic minor scale
pub const CSHARP_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C, D, E, F, G, A, B, C]);
/// D harmonic minor scale
pub const D_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D, E, F, G, A, B, C, D]);
/// D♯/E♭ harmonic minor scale
pub const DSHARP_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D, E, F, G, A, B, C, D]);
/// E harmonic minor scale
pub const E_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E, F, G, A, B, C, D, E]);
/// F harmonic minor scale
pub const F_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F, G, A, B, C, D, E, F]);
/// F♯/G♭ harmonic minor scale
pub const FSHARP_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F, G, A, B, C, D, E, F]);
/// G harmonic minor scale
pub const G_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G, A, B, C, D, E, F, G]);
/// G♯/A♭ harmonic minor scale
pub const GSHARP_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G, A, B, C, D, E, F, G]);
/// A harmonic minor scale
pub const A_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A, B, C, D, E, F, G, A]);
/// B harmonic minor scale
pub const B_HARMONIC_SCALE: Scale<8> = Scale::harmonic([B, C, D, E, F, G, A, B]);

// Octave 0 harmonic minor scales (C0-B0)
/// C harmonic minor scale starting from C0 (lowest octave)
pub const C0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C0, D0, E0, F0, G0, A0, B0, C1]);
/// C♯/D♭ harmonic minor scale starting from C♯0
pub const CSHARP0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C0, D0, E0, F0, G0, A0, B0, C1]);
/// D harmonic minor scale starting from D0
pub const D0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D0, E0, F0, G0, A0, B0, C1, D1]);
/// D♯/E♭ harmonic minor scale starting from D♯0
pub const DSHARP0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D0, E0, F0, G0, A0, B0, C1, D1]);
/// E harmonic minor scale starting from E0
pub const E0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E0, F0, G0, A0, B0, C1, D1, E1]);
/// F harmonic minor scale starting from F0
pub const F0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F0, G0, A0, B0, C1, D1, E1, F1]);
/// F♯/G♭ harmonic minor scale starting from F♯0
pub const FSHARP0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F0, G0, A0, B0, C1, D1, E1, F1]);
/// G harmonic minor scale starting from G0
pub const G0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G0, A0, B0, C1, D1, E1, F1, G1]);
/// G♯/A♭ harmonic minor scale starting from G♯0
pub const GSHARP0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G0, A0, B0, C1, D1, E1, F1, G1]);
/// A harmonic minor scale starting from A0
pub const A0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A0, B0, C1, D1, E1, F1, G1, A1]);
/// B harmonic minor scale starting from B0
pub const B0_HARMONIC_SCALE: Scale<8> = Scale::harmonic([B0, C1, D1, E1, F1, G1, A1, B1]);

// Octave 1 harmonic minor scales (C1-B1)
/// C harmonic minor scale starting from C1
pub const C1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C1, D1, E1, F1, G1, A1, B1, C2]);
/// C♯/D♭ harmonic minor scale starting from C♯1
pub const CSHARP1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C1, D1, E1, F1, G1, A1, B1, C2]);
/// D harmonic minor scale starting from D1
pub const D1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D1, E1, F1, G1, A1, B1, C2, D2]);
/// D♯/E♭ harmonic minor scale starting from D♯1
pub const DSHARP1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D1, E1, F1, G1, A1, B1, C2, D2]);
/// E harmonic minor scale starting from E1
pub const E1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E1, F1, G1, A1, B1, C2, D2, E2]);
/// F harmonic minor scale starting from F1
pub const F1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F1, G1, A1, B1, C2, D2, E2, F2]);
/// F♯/G♭ harmonic minor scale starting from F♯1
pub const FSHARP1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F1, G1, A1, B1, C2, D2, E2, F2]);
/// G harmonic minor scale starting from G1
pub const G1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G1, A1, B1, C2, D2, E2, F2, G2]);
/// G♯/A♭ harmonic minor scale starting from G♯1
pub const GSHARP1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G1, A1, B1, C2, D2, E2, F2, G2]);
/// A harmonic minor scale starting from A1
pub const A1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A1, B1, C2, D2, E2, F2, G2, A2]);
/// B harmonic minor scale starting from B1
pub const B1_HARMONIC_SCALE: Scale<8> = Scale::harmonic([B1, C2, D2, E2, F2, G2, A2, B2]);

// Octave 2 harmonic minor scales (C2-B2)
/// C harmonic minor scale starting from C2
pub const C2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C2, D2, E2, F2, G2, A2, B2, C3]);
/// C♯/D♭ harmonic minor scale starting from C♯2
pub const CSHARP2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C2, D2, E2, F2, G2, A2, B2, C3]);
/// D harmonic minor scale starting from D2
pub const D2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D2, E2, F2, G2, A2, B2, C3, D3]);
/// D♯/E♭ harmonic minor scale starting from D♯2
pub const DSHARP2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D2, E2, F2, G2, A2, B2, C3, D3]);
/// E harmonic minor scale starting from E2
pub const E2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E2, F2, G2, A2, B2, C3, D3, E3]);
/// F harmonic minor scale starting from F2
pub const F2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F2, G2, A2, B2, C3, D3, E3, F3]);
/// F♯/G♭ harmonic minor scale starting from F♯2
pub const FSHARP2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F2, G2, A2, B2, C3, D3, E3, F3]);
/// G harmonic minor scale starting from G2
pub const G2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G2, A2, B2, C3, D3, E3, F3, G3]);
/// G♯/A♭ harmonic minor scale starting from G♯2
pub const GSHARP2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G2, A2, B2, C3, D3, E3, F3, G3]);
/// A harmonic minor scale starting from A2
pub const A2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A2, B2, C3, D3, E3, F3, G3, A3]);
/// B harmonic minor scale starting from B2
pub const B2_HARMONIC_SCALE: Scale<8> = Scale::harmonic([B2, C3, D3, E3, F3, G3, A3, B3]);

// Octave 3 harmonic minor scales (C3-B3)
/// C harmonic minor scale starting from C3
pub const C3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C3, D3, E3, F3, G3, A3, B3, C4]);
/// C♯/D♭ harmonic minor scale starting from C♯3
pub const CSHARP3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C3, D3, E3, F3, G3, A3, B3, C4]);
/// D harmonic minor scale starting from D3
pub const D3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D3, E3, F3, G3, A3, B3, C4, D4]);
/// D♯/E♭ harmonic minor scale starting from D♯3
pub const DSHARP3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D3, E3, F3, G3, A3, B3, C4, D4]);
/// E harmonic minor scale starting from E3
pub const E3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E3, F3, G3, A3, B3, C4, D4, E4]);
/// F harmonic minor scale starting from F3
pub const F3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F3, G3, A3, B3, C4, D4, E4, F4]);
/// F♯/G♭ harmonic minor scale starting from F♯3
pub const FSHARP3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F3, G3, A3, B3, C4, D4, E4, F4]);
/// G harmonic minor scale starting from G3
pub const G3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G3, A3, B3, C4, D4, E4, F4, G4]);
/// G♯/A♭ harmonic minor scale starting from G♯3
pub const GSHARP3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G3, A3, B3, C4, D4, E4, F4, G4]);
/// A harmonic minor scale starting from A3
pub const A3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A3, B3, C4, D4, E4, F4, G4, A4]);
/// B harmonic minor scale starting from B3
pub const B3_HARMONIC_SCALE: Scale<8> = Scale::harmonic([B3, C4, D4, E4, F4, G4, A4, B4]);

// Octave 4 harmonic minor scales (C4-B4, including middle C)
/// C harmonic minor scale starting from middle C (C4)
pub const C4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C4, D4, E4, F4, G4, A4, B4, C5]);
/// C♯/D♭ harmonic minor scale starting from C♯4
pub const CSHARP4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C4, D4, E4, F4, G4, A4, B4, C5]);
/// D harmonic minor scale starting from D4
pub const D4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D4, E4, F4, G4, A4, B4, C5, D5]);
/// D♯/E♭ harmonic minor scale starting from D♯4
pub const DSHARP4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D4, E4, F4, G4, A4, B4, C5, D5]);
/// E harmonic minor scale starting from E4
pub const E4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E4, F4, G4, A4, B4, C5, D5, E5]);
/// F harmonic minor scale starting from F4
pub const F4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F4, G4, A4, B4, C5, D5, E5, F5]);
/// F♯/G♭ harmonic minor scale starting from F♯4
pub const FSHARP4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F4, G4, A4, B4, C5, D5, E5, F5]);
/// G harmonic minor scale starting from G4
pub const G4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G4, A4, B4, C5, D5, E5, F5, G5]);
/// G♯/A♭ harmonic minor scale starting from G♯4
pub const GSHARP4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G4, A4, B4, C5, D5, E5, F5, G5]);
/// A harmonic minor scale starting from A4 (A440)
pub const A4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A4, B4, C5, D5, E5, F5, G5, A5]);
/// B harmonic minor scale starting from B4
pub const B4_HARMONIC_SCALE: Scale<8> = Scale::harmonic([B4, C5, D5, E5, F5, G5, A5, B5]);

// Octave 5 harmonic minor scales (C5-B5)
/// C harmonic minor scale starting from C5
pub const C5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C5, D5, E5, F5, G5, A5, B5, C6]);
/// C♯/D♭ harmonic minor scale starting from C♯5
pub const CSHARP5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C5, D5, E5, F5, G5, A5, B5, C6]);
/// D harmonic minor scale starting from D5
pub const D5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D5, E5, F5, G5, A5, B5, C6, D6]);
/// D♯/E♭ harmonic minor scale starting from D♯5
pub const DSHARP5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D5, E5, F5, G5, A5, B5, C6, D6]);
/// E harmonic minor scale starting from E5
pub const E5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E5, F5, G5, A5, B5, C6, D6, E6]);
/// F harmonic minor scale starting from F5
pub const F5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F5, G5, A5, B5, C6, D6, E6, F6]);
/// F♯/G♭ harmonic minor scale starting from F♯5
pub const FSHARP5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F5, G5, A5, B5, C6, D6, E6, F6]);
/// G harmonic minor scale starting from G5
pub const G5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G5, A5, B5, C6, D6, E6, F6, G6]);
/// G♯/A♭ harmonic minor scale starting from G♯5
pub const GSHARP5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G5, A5, B5, C6, D6, E6, F6, G6]);
/// A harmonic minor scale starting from A5
pub const A5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A5, B5, C6, D6, E6, F6, G6, A6]);
/// B harmonic minor scale starting from B5
pub const B5_HARMONIC_SCALE: Scale<8> = Scale::harmonic([B5, C6, D6, E6, F6, G6, A6, B6]);

// Octave 6 harmonic minor scales (C6-B6)
/// C harmonic minor scale starting from C6
pub const C6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C6, D6, E6, F6, G6, A6, B6, C7]);
/// C♯/D♭ harmonic minor scale starting from C♯6
pub const CSHARP6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C6, D6, E6, F6, G6, A6, B6, C7]);
/// D harmonic minor scale starting from D6
pub const D6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D6, E6, F6, G6, A6, B6, C7, D7]);
/// D♯/E♭ harmonic minor scale starting from D♯6
pub const DSHARP6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D6, E6, F6, G6, A6, B6, C7, D7]);
/// E harmonic minor scale starting from E6
pub const E6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E6, F6, G6, A6, B6, C7, D7, E7]);
/// F harmonic minor scale starting from F6
pub const F6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F6, G6, A6, B6, C7, D7, E7, F7]);
/// F♯/G♭ harmonic minor scale starting from F♯6
pub const FSHARP6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F6, G6, A6, B6, C7, D7, E7, F7]);
/// G harmonic minor scale starting from G6
pub const G6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G6, A6, B6, C7, D7, E7, F7, G7]);
/// G♯/A♭ harmonic minor scale starting from G♯6
pub const GSHARP6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G6, A6, B6, C7, D7, E7, F7, G7]);
/// A harmonic minor scale starting from A6
pub const A6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A6, B6, C7, D7, E7, F7, G7, A7]);
/// B harmonic minor scale starting from B6
pub const B6_HARMONIC_SCALE: Scale<8> = Scale::harmonic([B6, C7, D7, E7, F7, G7, A7, B7]);

// Octave 7 harmonic minor scales (C7-B7)
/// C harmonic minor scale starting from C7
pub const C7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C7, D7, E7, F7, G7, A7, B7, C8]);
/// C♯/D♭ harmonic minor scale starting from C♯7
pub const CSHARP7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C7, D7, E7, F7, G7, A7, B7, C8]);
/// D harmonic minor scale starting from D7
pub const D7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D7, E7, F7, G7, A7, B7, C8, D8]);
/// D♯/E♭ harmonic minor scale starting from D♯7
pub const DSHARP7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D7, E7, F7, G7, A7, B7, C8, D8]);
/// E harmonic minor scale starting from E7
pub const E7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E7, F7, G7, A7, B7, C8, D8, E8]);
/// F harmonic minor scale starting from F7
pub const F7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F7, G7, A7, B7, C8, D8, E8, F8]);
/// F♯/G♭ harmonic minor scale starting from F♯7
pub const FSHARP7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F7, G7, A7, B7, C8, D8, E8, F8]);
/// G harmonic minor scale starting from G7
pub const G7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G7, A7, B7, C8, D8, E8, F8, G8]);
/// G♯/A♭ harmonic minor scale starting from G♯7
pub const GSHARP7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G7, A7, B7, C8, D8, E8, F8, G8]);
/// A harmonic minor scale starting from A7
pub const A7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([A7, B7, C8, D8, E8, F8, G8, A8]);
/// B harmonic minor scale starting from B7
pub const B7_HARMONIC_SCALE: Scale<8> = Scale::harmonic([B7, C8, D8, E8, F8, G8, A8, B8]);

// Octave 8 harmonic minor scales (C8-B8)
/// C harmonic minor scale starting from C8
pub const C8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C8, D8, E8, F8, G8, A8, B8, C9]);
/// C♯/D♭ harmonic minor scale starting from C♯8
pub const CSHARP8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([C8, D8, E8, F8, G8, A8, B8, C9]);
/// D harmonic minor scale starting from D8
pub const D8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D8, E8, F8, G8, A8, B8, C9, D9]);
/// D♯/E♭ harmonic minor scale starting from D♯8
pub const DSHARP8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([D8, E8, F8, G8, A8, B8, C9, D9]);
/// E harmonic minor scale starting from E8
pub const E8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([E8, F8, G8, A8, B8, C9, D9, E9]);
/// F harmonic minor scale starting from F8
pub const F8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F8, G8, A8, B8, C9, D9, E9, F9]);
/// F♯/G♭ harmonic minor scale starting from F♯8
pub const FSHARP8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([F8, G8, A8, B8, C9, D9, E9, F9]);
/// G harmonic minor scale starting from G8
pub const G8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G8, A8, B8, C9, D9, E9, F9, G9]);
/// G♯/A♭ harmonic minor scale starting from G♯8
pub const GSHARP8_HARMONIC_SCALE: Scale<8> = Scale::harmonic([G8, A8, B8, C9, D9, E9, F9, G9]);

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
    pub static ref HARMONIC_SCALES: std::collections::HashMap<Pitch, Scale<8>> = {
        let mut scales = HashMap::new();

        // Add octave-independent scales (pitch class only)
        scales.insert(C, C_HARMONIC_SCALE);
        scales.insert(CSHARP, CSHARP_HARMONIC_SCALE);
        scales.insert(D, D_HARMONIC_SCALE);
        scales.insert(DSHARP, DSHARP_HARMONIC_SCALE);
        scales.insert(E, E_HARMONIC_SCALE);
        scales.insert(F, F_HARMONIC_SCALE);
        scales.insert(FSHARP, FSHARP_HARMONIC_SCALE);
        scales.insert(G, G_HARMONIC_SCALE);
        scales.insert(GSHARP, GSHARP_HARMONIC_SCALE);
        scales.insert(A, A_HARMONIC_SCALE);
        scales.insert(B, B_HARMONIC_SCALE);

        // Add octave 0 scales (lowest octave)
        scales.insert(C0, C0_HARMONIC_SCALE);
        scales.insert(CSHARP0, CSHARP0_HARMONIC_SCALE);
        scales.insert(D0, D0_HARMONIC_SCALE);
        scales.insert(DSHARP0, DSHARP0_HARMONIC_SCALE);
        scales.insert(E0, E0_HARMONIC_SCALE);
        scales.insert(F0, F0_HARMONIC_SCALE);
        scales.insert(FSHARP0, FSHARP0_HARMONIC_SCALE);
        scales.insert(G0, G0_HARMONIC_SCALE);
        scales.insert(GSHARP0, GSHARP0_HARMONIC_SCALE);
        scales.insert(A0, A0_HARMONIC_SCALE);
        scales.insert(B0, B0_HARMONIC_SCALE);

        // Add octave 1 scales (C1-B1)
        scales.insert(C1, C1_HARMONIC_SCALE);
        scales.insert(CSHARP1, CSHARP1_HARMONIC_SCALE);
        scales.insert(D1, D1_HARMONIC_SCALE);
        scales.insert(DSHARP1, DSHARP1_HARMONIC_SCALE);
        scales.insert(E1, E1_HARMONIC_SCALE);
        scales.insert(F1, F1_HARMONIC_SCALE);
        scales.insert(FSHARP1, FSHARP1_HARMONIC_SCALE);
        scales.insert(G1, G1_HARMONIC_SCALE);
        scales.insert(GSHARP1, GSHARP1_HARMONIC_SCALE);
        scales.insert(A1, A1_HARMONIC_SCALE);
        scales.insert(B1, B1_HARMONIC_SCALE);

        // Add octave 2 scales (C2-B2)
        scales.insert(C2, C2_HARMONIC_SCALE);
        scales.insert(CSHARP2, CSHARP2_HARMONIC_SCALE);
        scales.insert(D2, D2_HARMONIC_SCALE);
        scales.insert(DSHARP2, DSHARP2_HARMONIC_SCALE);
        scales.insert(E2, E2_HARMONIC_SCALE);
        scales.insert(F2, F2_HARMONIC_SCALE);
        scales.insert(FSHARP2, FSHARP2_HARMONIC_SCALE);
        scales.insert(G2, G2_HARMONIC_SCALE);
        scales.insert(GSHARP2, GSHARP2_HARMONIC_SCALE);
        scales.insert(A2, A2_HARMONIC_SCALE);
        scales.insert(B2, B2_HARMONIC_SCALE);

        // Add octave 3 scales (C3-B3)
        scales.insert(C3, C3_HARMONIC_SCALE);
        scales.insert(CSHARP3, CSHARP3_HARMONIC_SCALE);
        scales.insert(D3, D3_HARMONIC_SCALE);
        scales.insert(DSHARP3, DSHARP3_HARMONIC_SCALE);
        scales.insert(E3, E3_HARMONIC_SCALE);
        scales.insert(F3, F3_HARMONIC_SCALE);
        scales.insert(FSHARP3, FSHARP3_HARMONIC_SCALE);
        scales.insert(G3, G3_HARMONIC_SCALE);
        scales.insert(GSHARP3, GSHARP3_HARMONIC_SCALE);
        scales.insert(A3, A3_HARMONIC_SCALE);
        scales.insert(B3, B3_HARMONIC_SCALE);

        // Add octave 4 scales (middle octave, including middle C)
        scales.insert(C4, C4_HARMONIC_SCALE);
        scales.insert(CSHARP4, CSHARP4_HARMONIC_SCALE);
        scales.insert(D4, D4_HARMONIC_SCALE);
        scales.insert(DSHARP4, DSHARP4_HARMONIC_SCALE);
        scales.insert(E4, E4_HARMONIC_SCALE);
        scales.insert(F4, F4_HARMONIC_SCALE);
        scales.insert(FSHARP4, FSHARP4_HARMONIC_SCALE);
        scales.insert(G4, G4_HARMONIC_SCALE);
        scales.insert(GSHARP4, GSHARP4_HARMONIC_SCALE);
        scales.insert(A4, A4_HARMONIC_SCALE); // A440
        scales.insert(B4, B4_HARMONIC_SCALE);

        // Add octave 5 scales (C5-B5)
        scales.insert(C5, C5_HARMONIC_SCALE);
        scales.insert(CSHARP5, CSHARP5_HARMONIC_SCALE);
        scales.insert(D5, D5_HARMONIC_SCALE);
        scales.insert(DSHARP5, DSHARP5_HARMONIC_SCALE);
        scales.insert(E5, E5_HARMONIC_SCALE);
        scales.insert(F5, F5_HARMONIC_SCALE);
        scales.insert(FSHARP5, FSHARP5_HARMONIC_SCALE);
        scales.insert(G5, G5_HARMONIC_SCALE);
        scales.insert(GSHARP5, GSHARP5_HARMONIC_SCALE);
        scales.insert(A5, A5_HARMONIC_SCALE);
        scales.insert(B5, B5_HARMONIC_SCALE);

        // Add octave 6 scales (C6-B6)
        scales.insert(C6, C6_HARMONIC_SCALE);
        scales.insert(CSHARP6, CSHARP6_HARMONIC_SCALE);
        scales.insert(D6, D6_HARMONIC_SCALE);
        scales.insert(DSHARP6, DSHARP6_HARMONIC_SCALE);
        scales.insert(E6, E6_HARMONIC_SCALE);
        scales.insert(F6, F6_HARMONIC_SCALE);
        scales.insert(FSHARP6, FSHARP6_HARMONIC_SCALE);
        scales.insert(G6, G6_HARMONIC_SCALE);
        scales.insert(GSHARP6, GSHARP6_HARMONIC_SCALE);
        scales.insert(A6, A6_HARMONIC_SCALE);
        scales.insert(B6, B6_HARMONIC_SCALE);

        // Add octave 7 scales (C7-B7)
        scales.insert(C7, C7_HARMONIC_SCALE);
        scales.insert(CSHARP7, CSHARP7_HARMONIC_SCALE);
        scales.insert(D7, D7_HARMONIC_SCALE);
        scales.insert(DSHARP7, DSHARP7_HARMONIC_SCALE);
        scales.insert(E7, E7_HARMONIC_SCALE);
        scales.insert(F7, F7_HARMONIC_SCALE);
        scales.insert(FSHARP7, FSHARP7_HARMONIC_SCALE);
        scales.insert(G7, G7_HARMONIC_SCALE);
        scales.insert(GSHARP7, GSHARP7_HARMONIC_SCALE);
        scales.insert(A7, A7_HARMONIC_SCALE);
        scales.insert(B7, B7_HARMONIC_SCALE);

        // Add octave 8 scales (C8-G#8, highest octave)
        scales.insert(C8, C8_HARMONIC_SCALE);
        scales.insert(CSHARP8, CSHARP8_HARMONIC_SCALE);
        scales.insert(D8, D8_HARMONIC_SCALE);
        scales.insert(DSHARP8, DSHARP8_HARMONIC_SCALE);
        scales.insert(E8, E8_HARMONIC_SCALE);
        scales.insert(F8, F8_HARMONIC_SCALE);
        scales.insert(FSHARP8, FSHARP8_HARMONIC_SCALE);
        scales.insert(G8, G8_HARMONIC_SCALE);
        scales.insert(GSHARP8, GSHARP8_HARMONIC_SCALE);

        scales
    };
}
