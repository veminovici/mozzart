//! Major scale constants and collections
//!
//! This module provides predefined constants for all major scales across the MIDI note range.
//! Each scale is represented as an array of 8 notes (one octave) starting from a specific root pitch.
//!
//! # Major Scale Structure
//! A major scale follows the interval pattern: Whole-Whole-Half-Whole-Whole-Whole-Half
//! This creates the familiar "do-re-mi-fa-so-la-ti-do" pattern, where:
//! - Between do-re: whole step (2 semitones)
//! - Between re-mi: whole step (2 semitones)
//! - Between mi-fa: half step (1 semitone)
//! - Between fa-so: whole step (2 semitones)
//! - Between so-la: whole step (2 semitones)
//! - Between la-ti: whole step (2 semitones)
//! - Between ti-do: half step (1 semitone)
//!
//! # Scale Organization
//! Scales are organized in several ways:
//! 1. Octave-independent scales (e.g., `C_MAJOR_SCALE`) - generic patterns
//! 2. Octave-specific scales (e.g., `C4_MAJOR_SCALE`) - starting from specific octaves
//! 3. Collections in `MAJOR_SCALES` - HashMap mapping root pitches to their major scales
//!
//! # Examples
//! ```
//! use mozzart_std::{C4_MAJOR_SCALE, MAJOR_SCALES};
//! use mozzart_std::C4;
//!
//! // Use a predefined scale
//! let c_major = C4_MAJOR_SCALE;
//!
//! // Look up a scale by root note
//! let scale = MAJOR_SCALES.get(&C4).unwrap();
//! assert_eq!(*scale, C4_MAJOR_SCALE);
//! ```

use crate::constants::*;
use crate::{Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

// Octave-independent major scales (pitch class only)
/// C major scale (C-D-E-F-G-A-B-C)
pub const C_MAJOR_SCALE: Scale<8> = Scale::major([C, D, E, F, G, A, B, C]);
/// C♯/D♭ major scale
pub const CSHARP_MAJOR_SCALE: Scale<8> = Scale::major([C, D, E, F, G, A, B, C]);
/// D major scale
pub const D_MAJOR_SCALE: Scale<8> = Scale::major([D, E, F, G, A, B, C, D]);
/// D♯/E♭ major scale
pub const DSHARP_MAJOR_SCALE: Scale<8> = Scale::major([D, E, F, G, A, B, C, D]);
/// E major scale
pub const E_MAJOR_SCALE: Scale<8> = Scale::major([E, F, G, A, B, C, D, E]);
/// F major scale
pub const F_MAJOR_SCALE: Scale<8> = Scale::major([F, G, A, B, C, D, E, F]);
/// F♯/G♭ major scale
pub const FSHARP_MAJOR_SCALE: Scale<8> = Scale::major([F, G, A, B, C, D, E, F]);
/// G major scale
pub const G_MAJOR_SCALE: Scale<8> = Scale::major([G, A, B, C, D, E, F, G]);
/// G♯/A♭ major scale
pub const GSHARP_MAJOR_SCALE: Scale<8> = Scale::major([G, A, B, C, D, E, F, G]);
/// A major scale
pub const A_MAJOR_SCALE: Scale<8> = Scale::major([A, B, C, D, E, F, G, A]);
/// B major scale
pub const B_MAJOR_SCALE: Scale<8> = Scale::major([B, C, D, E, F, G, A, B]);

// Octave 0 major scales (C0-B0)
/// C major scale starting from C0 (lowest octave)
pub const C0_MAJOR_SCALE: Scale<8> = Scale::major([C0, D0, E0, F0, G0, A0, B0, C1]);
pub const CSHARP0_MAJOR_SCALE: Scale<8> = Scale::major([C0, D0, E0, F0, G0, A0, B0, C1]);
pub const D0_MAJOR_SCALE: Scale<8> = Scale::major([D0, E0, F0, G0, A0, B0, C1, D1]);
pub const DSHARP0_MAJOR_SCALE: Scale<8> = Scale::major([D0, E0, F0, G0, A0, B0, C1, D1]);
pub const E0_MAJOR_SCALE: Scale<8> = Scale::major([E0, F0, G0, A0, B0, C1, D1, E1]);
pub const F0_MAJOR_SCALE: Scale<8> = Scale::major([F0, G0, A0, B0, C1, D1, E1, F1]);
pub const FSHARP0_MAJOR_SCALE: Scale<8> = Scale::major([F0, G0, A0, B0, C1, D1, E1, F1]);
pub const G0_MAJOR_SCALE: Scale<8> = Scale::major([G0, A0, B0, C1, D1, E1, F1, G1]);
pub const GSHARP0_MAJOR_SCALE: Scale<8> = Scale::major([G0, A0, B0, C1, D1, E1, F1, G1]);
pub const A0_MAJOR_SCALE: Scale<8> = Scale::major([A0, B0, C1, D1, E1, F1, G1, A1]);
pub const B0_MAJOR_SCALE: Scale<8> = Scale::major([B0, C1, D1, E1, F1, G1, A1, B1]);

pub const C1_MAJOR_SCALE: Scale<8> = Scale::major([C1, D1, E1, F1, G1, A1, B1, C2]);
pub const CSHARP1_MAJOR_SCALE: Scale<8> = Scale::major([C1, D1, E1, F1, G1, A1, B1, C2]);
pub const D1_MAJOR_SCALE: Scale<8> = Scale::major([D1, E1, F1, G1, A1, B1, C2, D2]);
pub const DSHARP1_MAJOR_SCALE: Scale<8> = Scale::major([D1, E1, F1, G1, A1, B1, C2, D2]);
pub const E1_MAJOR_SCALE: Scale<8> = Scale::major([E1, F1, G1, A1, B1, C2, D2, E2]);
pub const F1_MAJOR_SCALE: Scale<8> = Scale::major([F1, G1, A1, B1, C2, D2, E2, F2]);
pub const FSHARP1_MAJOR_SCALE: Scale<8> = Scale::major([F1, G1, A1, B1, C2, D2, E2, F2]);
pub const G1_MAJOR_SCALE: Scale<8> = Scale::major([G1, A1, B1, C2, D2, E2, F2, G2]);
pub const GSHARP1_MAJOR_SCALE: Scale<8> = Scale::major([G1, A1, B1, C2, D2, E2, F2, G2]);
pub const A1_MAJOR_SCALE: Scale<8> = Scale::major([A1, B1, C2, D2, E2, F2, G2, A2]);
pub const B1_MAJOR_SCALE: Scale<8> = Scale::major([B1, C2, D2, E2, F2, G2, A2, B2]);

pub const C2_MAJOR_SCALE: Scale<8> = Scale::major([C2, D2, E2, F2, G2, A2, B2, C3]);
pub const CSHARP2_MAJOR_SCALE: Scale<8> = Scale::major([C2, D2, E2, F2, G2, A2, B2, C3]);
pub const D2_MAJOR_SCALE: Scale<8> = Scale::major([D2, E2, F2, G2, A2, B2, C3, D3]);
pub const DSHARP2_MAJOR_SCALE: Scale<8> = Scale::major([D2, E2, F2, G2, A2, B2, C3, D3]);
pub const E2_MAJOR_SCALE: Scale<8> = Scale::major([E2, F2, G2, A2, B2, C3, D3, E3]);
pub const F2_MAJOR_SCALE: Scale<8> = Scale::major([F2, G2, A2, B2, C3, D3, E3, F3]);
pub const FSHARP2_MAJOR_SCALE: Scale<8> = Scale::major([F2, G2, A2, B2, C3, D3, E3, F3]);
pub const G2_MAJOR_SCALE: Scale<8> = Scale::major([G2, A2, B2, C3, D3, E3, F3, G3]);
pub const GSHARP2_MAJOR_SCALE: Scale<8> = Scale::major([G2, A2, B2, C3, D3, E3, F3, G3]);
pub const A2_MAJOR_SCALE: Scale<8> = Scale::major([A2, B2, C3, D3, E3, F3, G3, A3]);
pub const B2_MAJOR_SCALE: Scale<8> = Scale::major([B2, C3, D3, E3, F3, G3, A3, B3]);

pub const C3_MAJOR_SCALE: Scale<8> = Scale::major([C3, D3, E3, F3, G3, A3, B3, C4]);
pub const CSHARP3_MAJOR_SCALE: Scale<8> = Scale::major([C3, D3, E3, F3, G3, A3, B3, C4]);
pub const D3_MAJOR_SCALE: Scale<8> = Scale::major([D3, E3, F3, G3, A3, B3, C4, D4]);
pub const DSHARP3_MAJOR_SCALE: Scale<8> = Scale::major([D3, E3, F3, G3, A3, B3, C4, D4]);
pub const E3_MAJOR_SCALE: Scale<8> = Scale::major([E3, F3, G3, A3, B3, C4, D4, E4]);
pub const F3_MAJOR_SCALE: Scale<8> = Scale::major([F3, G3, A3, B3, C4, D4, E4, F4]);
pub const FSHARP3_MAJOR_SCALE: Scale<8> = Scale::major([F3, G3, A3, B3, C4, D4, E4, F4]);
pub const G3_MAJOR_SCALE: Scale<8> = Scale::major([G3, A3, B3, C4, D4, E4, F4, G4]);
pub const GSHARP3_MAJOR_SCALE: Scale<8> = Scale::major([G3, A3, B3, C4, D4, E4, F4, G4]);
pub const A3_MAJOR_SCALE: Scale<8> = Scale::major([A3, B3, C4, D4, E4, F4, G4, A4]);
pub const B3_MAJOR_SCALE: Scale<8> = Scale::major([B3, C4, D4, E4, F4, G4, A4, B4]);

// Octave 4 major scales (C4-B4, including middle C)
/// C major scale starting from middle C (C4)
pub const C4_MAJOR_SCALE: Scale<8> = Scale::major([C4, D4, E4, F4, G4, A4, B4, C5]);
pub const CSHARP4_MAJOR_SCALE: Scale<8> = Scale::major([C4, D4, E4, F4, G4, A4, B4, C5]);
pub const D4_MAJOR_SCALE: Scale<8> = Scale::major([D4, E4, F4, G4, A4, B4, C5, D5]);
pub const DSHARP4_MAJOR_SCALE: Scale<8> = Scale::major([D4, E4, F4, G4, A4, B4, C5, D5]);
pub const E4_MAJOR_SCALE: Scale<8> = Scale::major([E4, F4, G4, A4, B4, C5, D5, E5]);
pub const F4_MAJOR_SCALE: Scale<8> = Scale::major([F4, G4, A4, B4, C5, D5, E5, F5]);
pub const FSHARP4_MAJOR_SCALE: Scale<8> = Scale::major([F4, G4, A4, B4, C5, D5, E5, F5]);
pub const G4_MAJOR_SCALE: Scale<8> = Scale::major([G4, A4, B4, C5, D5, E5, F5, G5]);
pub const GSHARP4_MAJOR_SCALE: Scale<8> = Scale::major([G4, A4, B4, C5, D5, E5, F5, G5]);
pub const A4_MAJOR_SCALE: Scale<8> = Scale::major([A4, B4, C5, D5, E5, F5, G5, A5]);
pub const B4_MAJOR_SCALE: Scale<8> = Scale::major([B4, C5, D5, E5, F5, G5, A5, B5]);

pub const C5_MAJOR_SCALE: Scale<8> = Scale::major([C5, D5, E5, F5, G5, A5, B5, C6]);
pub const CSHARP5_MAJOR_SCALE: Scale<8> = Scale::major([C5, D5, E5, F5, G5, A5, B5, C6]);
pub const D5_MAJOR_SCALE: Scale<8> = Scale::major([D5, E5, F5, G5, A5, B5, C6, D6]);
pub const DSHARP5_MAJOR_SCALE: Scale<8> = Scale::major([D5, E5, F5, G5, A5, B5, C6, D6]);
pub const E5_MAJOR_SCALE: Scale<8> = Scale::major([E5, F5, G5, A5, B5, C6, D6, E6]);
pub const F5_MAJOR_SCALE: Scale<8> = Scale::major([F5, G5, A5, B5, C6, D6, E6, F6]);
pub const FSHARP5_MAJOR_SCALE: Scale<8> = Scale::major([F5, G5, A5, B5, C6, D6, E6, F6]);
pub const G5_MAJOR_SCALE: Scale<8> = Scale::major([G5, A5, B5, C6, D6, E6, F6, G6]);
pub const GSHARP5_MAJOR_SCALE: Scale<8> = Scale::major([G5, A5, B5, C6, D6, E6, F6, G6]);
pub const A5_MAJOR_SCALE: Scale<8> = Scale::major([A5, B5, C6, D6, E6, F6, G6, A6]);
pub const B5_MAJOR_SCALE: Scale<8> = Scale::major([B5, C6, D6, E6, F6, G6, A6, B6]);

pub const C6_MAJOR_SCALE: Scale<8> = Scale::major([C6, D6, E6, F6, G6, A6, B6, C7]);
pub const CSHARP6_MAJOR_SCALE: Scale<8> = Scale::major([C6, D6, E6, F6, G6, A6, B6, C7]);
pub const D6_MAJOR_SCALE: Scale<8> = Scale::major([D6, E6, F6, G6, A6, B6, C7, D7]);
pub const DSHARP6_MAJOR_SCALE: Scale<8> = Scale::major([D6, E6, F6, G6, A6, B6, C7, D7]);
pub const E6_MAJOR_SCALE: Scale<8> = Scale::major([E6, F6, G6, A6, B6, C7, D7, E7]);
pub const F6_MAJOR_SCALE: Scale<8> = Scale::major([F6, G6, A6, B6, C7, D7, E7, F7]);
pub const FSHARP6_MAJOR_SCALE: Scale<8> = Scale::major([F6, G6, A6, B6, C7, D7, E7, F7]);
pub const G6_MAJOR_SCALE: Scale<8> = Scale::major([G6, A6, B6, C7, D7, E7, F7, G7]);
pub const GSHARP6_MAJOR_SCALE: Scale<8> = Scale::major([G6, A6, B6, C7, D7, E7, F7, G7]);
pub const A6_MAJOR_SCALE: Scale<8> = Scale::major([A6, B6, C7, D7, E7, F7, G7, A7]);
pub const B6_MAJOR_SCALE: Scale<8> = Scale::major([B6, C7, D7, E7, F7, G7, A7, B7]);

pub const C7_MAJOR_SCALE: Scale<8> = Scale::major([C7, D7, E7, F7, G7, A7, B7, C8]);
pub const CSHARP7_MAJOR_SCALE: Scale<8> = Scale::major([C7, D7, E7, F7, G7, A7, B7, C8]);
pub const D7_MAJOR_SCALE: Scale<8> = Scale::major([D7, E7, F7, G7, A7, B7, C8, D8]);
pub const DSHARP7_MAJOR_SCALE: Scale<8> = Scale::major([D7, E7, F7, G7, A7, B7, C8, D8]);
pub const E7_MAJOR_SCALE: Scale<8> = Scale::major([E7, F7, G7, A7, B7, C8, D8, E8]);
pub const F7_MAJOR_SCALE: Scale<8> = Scale::major([F7, G7, A7, B7, C8, D8, E8, F8]);
pub const FSHARP7_MAJOR_SCALE: Scale<8> = Scale::major([F7, G7, A7, B7, C8, D8, E8, F8]);
pub const G7_MAJOR_SCALE: Scale<8> = Scale::major([G7, A7, B7, C8, D8, E8, F8, G8]);
pub const GSHARP7_MAJOR_SCALE: Scale<8> = Scale::major([G7, A7, B7, C8, D8, E8, F8, G8]);
pub const A7_MAJOR_SCALE: Scale<8> = Scale::major([A7, B7, C8, D8, E8, F8, G8, A8]);
pub const B7_MAJOR_SCALE: Scale<8> = Scale::major([B7, C8, D8, E8, F8, G8, A8, B8]);

pub const C8_MAJOR_SCALE: Scale<8> = Scale::major([C8, D8, E8, F8, G8, A8, B8, C9]);
pub const CSHARP8_MAJOR_SCALE: Scale<8> = Scale::major([C8, D8, E8, F8, G8, A8, B8, C9]);
pub const D8_MAJOR_SCALE: Scale<8> = Scale::major([D8, E8, F8, G8, A8, B8, C9, D9]);
pub const DSHARP8_MAJOR_SCALE: Scale<8> = Scale::major([D8, E8, F8, G8, A8, B8, C9, D9]);
pub const E8_MAJOR_SCALE: Scale<8> = Scale::major([E8, F8, G8, A8, B8, C9, D9, E9]);
pub const F8_MAJOR_SCALE: Scale<8> = Scale::major([F8, G8, A8, B8, C9, D9, E9, F9]);
pub const FSHARP8_MAJOR_SCALE: Scale<8> = Scale::major([F8, G8, A8, B8, C9, D9, E9, F9]);
pub const G8_MAJOR_SCALE: Scale<8> = Scale::major([G8, A8, B8, C9, D9, E9, F9, G9]);
pub const GSHARP8_MAJOR_SCALE: Scale<8> = Scale::major([G8, A8, B8, C9, D9, E9, F9, G9]);

lazy_static! {
    /// HashMap containing all major scales indexed by their root pitch.
    /// This collection provides quick access to any major scale given its root note.
    ///
    /// The map includes scales for all valid MIDI note numbers, organized by octave.
    /// Each scale spans exactly one octave from its root note.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::MAJOR_SCALES;
    /// use mozzart_std::C4;
    ///
    /// if let Some(scale) = MAJOR_SCALES.get(&C4) {
    ///     // Use the C major scale starting from middle C
    /// }
    /// ```
    pub static ref MAJOR_SCALES: std::collections::HashMap<Pitch, Scale<8>> = {
        let mut scales = HashMap::new();

        // Add octave-independent scales
        scales.insert(C, C_MAJOR_SCALE);
        scales.insert(CSHARP, CSHARP_MAJOR_SCALE);
        scales.insert(D, D_MAJOR_SCALE);
        scales.insert(DSHARP, DSHARP_MAJOR_SCALE);
        scales.insert(E, E_MAJOR_SCALE);
        scales.insert(F, F_MAJOR_SCALE);
        scales.insert(FSHARP, FSHARP_MAJOR_SCALE);
        scales.insert(G, G_MAJOR_SCALE);
        scales.insert(GSHARP, GSHARP_MAJOR_SCALE);
        scales.insert(A, A_MAJOR_SCALE);
        scales.insert(B, B_MAJOR_SCALE);

        // Add octave 0 scales (lowest octave)
        scales.insert(C0, C0_MAJOR_SCALE);
        scales.insert(CSHARP0, CSHARP0_MAJOR_SCALE);
        scales.insert(D0, D0_MAJOR_SCALE);
        scales.insert(DSHARP0, DSHARP0_MAJOR_SCALE);
        scales.insert(E0, E0_MAJOR_SCALE);
        scales.insert(F0, F0_MAJOR_SCALE);
        scales.insert(FSHARP0, FSHARP0_MAJOR_SCALE);
        scales.insert(G0, G0_MAJOR_SCALE);
        scales.insert(GSHARP0, GSHARP0_MAJOR_SCALE);
        scales.insert(A0, A0_MAJOR_SCALE);
        scales.insert(B0, B0_MAJOR_SCALE);

        // Add octave 4 scales (middle octave)
        scales.insert(C4, C4_MAJOR_SCALE);
        scales.insert(F4, F4_MAJOR_SCALE);
        scales.insert(FSHARP4, FSHARP4_MAJOR_SCALE);
        scales.insert(G4, G4_MAJOR_SCALE);
        scales.insert(GSHARP4, GSHARP4_MAJOR_SCALE);
        scales.insert(A4, A4_MAJOR_SCALE);
        scales.insert(B4, B4_MAJOR_SCALE);

        scales.insert(C1, C1_MAJOR_SCALE);
        scales.insert(CSHARP1, CSHARP1_MAJOR_SCALE);
        scales.insert(D1, D1_MAJOR_SCALE);
        scales.insert(DSHARP1, DSHARP1_MAJOR_SCALE);
        scales.insert(E1, E1_MAJOR_SCALE);
        scales.insert(F1, F1_MAJOR_SCALE);
        scales.insert(FSHARP1, FSHARP1_MAJOR_SCALE);
        scales.insert(G1, G1_MAJOR_SCALE);
        scales.insert(GSHARP1, GSHARP1_MAJOR_SCALE);
        scales.insert(A1, A1_MAJOR_SCALE);
        scales.insert(B1, B1_MAJOR_SCALE);

        scales.insert(C2, C2_MAJOR_SCALE);
        scales.insert(F2, F2_MAJOR_SCALE);
        scales.insert(FSHARP2, FSHARP2_MAJOR_SCALE);
        scales.insert(G2, G2_MAJOR_SCALE);
        scales.insert(GSHARP2, GSHARP2_MAJOR_SCALE);
        scales.insert(A2, A2_MAJOR_SCALE);
        scales.insert(B2, B2_MAJOR_SCALE);

        scales.insert(C3, C3_MAJOR_SCALE);
        scales.insert(CSHARP3, CSHARP3_MAJOR_SCALE);
        scales.insert(D3, D3_MAJOR_SCALE);
        scales.insert(DSHARP3, DSHARP3_MAJOR_SCALE);
        scales.insert(E3, E3_MAJOR_SCALE);
        scales.insert(F3, F3_MAJOR_SCALE);
        scales.insert(FSHARP3, FSHARP3_MAJOR_SCALE);
        scales.insert(G3, G3_MAJOR_SCALE);
        scales.insert(GSHARP3, GSHARP3_MAJOR_SCALE);
        scales.insert(A3, A3_MAJOR_SCALE);
        scales.insert(B3, B3_MAJOR_SCALE);

        scales.insert(C5, C5_MAJOR_SCALE);
        scales.insert(CSHARP5, CSHARP5_MAJOR_SCALE);
        scales.insert(D5, D5_MAJOR_SCALE);
        scales.insert(DSHARP5, DSHARP5_MAJOR_SCALE);
        scales.insert(E5, E5_MAJOR_SCALE);
        scales.insert(F5, F5_MAJOR_SCALE);
        scales.insert(FSHARP5, FSHARP5_MAJOR_SCALE);
        scales.insert(G5, G5_MAJOR_SCALE);
        scales.insert(GSHARP5, GSHARP5_MAJOR_SCALE);
        scales.insert(A5, A5_MAJOR_SCALE);
        scales.insert(B5, B5_MAJOR_SCALE);

        scales.insert(C6, C6_MAJOR_SCALE);
        scales.insert(CSHARP6, CSHARP6_MAJOR_SCALE);
        scales.insert(D6, D6_MAJOR_SCALE);
        scales.insert(DSHARP6, DSHARP6_MAJOR_SCALE);
        scales.insert(E6, E6_MAJOR_SCALE);
        scales.insert(F6, F6_MAJOR_SCALE);
        scales.insert(FSHARP6, FSHARP6_MAJOR_SCALE);
        scales.insert(G6, G6_MAJOR_SCALE);
        scales.insert(GSHARP6, GSHARP6_MAJOR_SCALE);
        scales.insert(A6, A6_MAJOR_SCALE);
        scales.insert(B6, B6_MAJOR_SCALE);

        scales.insert(C7, C7_MAJOR_SCALE);
        scales.insert(CSHARP7, CSHARP7_MAJOR_SCALE);
        scales.insert(D7, D7_MAJOR_SCALE);
        scales.insert(DSHARP7, DSHARP7_MAJOR_SCALE);
        scales.insert(E7, E7_MAJOR_SCALE);
        scales.insert(F7, F7_MAJOR_SCALE);
        scales.insert(FSHARP7, FSHARP7_MAJOR_SCALE);
        scales.insert(G7, G7_MAJOR_SCALE);
        scales.insert(GSHARP7, GSHARP7_MAJOR_SCALE);
        scales.insert(A7, A7_MAJOR_SCALE);
        scales.insert(B7, B7_MAJOR_SCALE);

        scales.insert(C8, C8_MAJOR_SCALE);
        scales.insert(CSHARP8, CSHARP8_MAJOR_SCALE);
        scales.insert(D8, D8_MAJOR_SCALE);
        scales.insert(DSHARP8, DSHARP8_MAJOR_SCALE);
        scales.insert(E8, E8_MAJOR_SCALE);
        scales.insert(F8, F8_MAJOR_SCALE);
        scales.insert(FSHARP8, FSHARP8_MAJOR_SCALE);
        scales.insert(G8, G8_MAJOR_SCALE);
        scales.insert(GSHARP8, GSHARP8_MAJOR_SCALE);

        scales
    };
}
