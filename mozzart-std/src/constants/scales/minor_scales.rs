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
use crate::{Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

// Octave-independent minor scales (pitch class only)
/// C minor scale (C-D-E♭-F-G-A♭-B♭-C)
pub const C_MINOR_SCALE: Scale<8> = Scale::minor([C, D, E, F, G, A, B, C]);
/// C♯/D♭ minor scale
pub const CSHARP_MINOR_SCALE: Scale<8> = Scale::minor([C, D, E, F, G, A, B, C]);
/// D minor scale
pub const D_MINOR_SCALE: Scale<8> = Scale::minor([D, E, F, G, A, B, C, D]);
/// D♯/E♭ minor scale
pub const DSHARP_MINOR_SCALE: Scale<8> = Scale::minor([D, E, F, G, A, B, C, D]);
/// E minor scale (relative to G major)
pub const E_MINOR_SCALE: Scale<8> = Scale::minor([E, F, G, A, B, C, D, E]);
/// F minor scale
pub const F_MINOR_SCALE: Scale<8> = Scale::minor([F, G, A, B, C, D, E, F]);
/// F♯/G♭ minor scale
pub const FSHARP_MINOR_SCALE: Scale<8> = Scale::minor([F, G, A, B, C, D, E, F]);
/// G minor scale
pub const G_MINOR_SCALE: Scale<8> = Scale::minor([G, A, B, C, D, E, F, G]);
/// G♯/A♭ minor scale
pub const GSHARP_MINOR_SCALE: Scale<8> = Scale::minor([G, A, B, C, D, E, F, G]);
/// A minor scale (relative to C major)
pub const A_MINOR_SCALE: Scale<8> = Scale::minor([A, B, C, D, E, F, G, A]);
/// B minor scale (relative to D major)
pub const B_MINOR_SCALE: Scale<8> = Scale::minor([B, C, D, E, F, G, A, B]);

// Octave 0 minor scales (C0-B0)
/// C minor scale starting from C0 (lowest octave)
pub const C0_MINOR_SCALE: Scale<8> = Scale::minor([C0, D0, E0, F0, G0, A0, B0, C1]);
pub const CSHARP0_MINOR_SCALE: Scale<8> = Scale::minor([C0, D0, E0, F0, G0, A0, B0, C1]);
pub const D0_MINOR_SCALE: Scale<8> = Scale::minor([D0, E0, F0, G0, A0, B0, C1, D1]);
pub const DSHARP0_MINOR_SCALE: Scale<8> = Scale::minor([D0, E0, F0, G0, A0, B0, C1, D1]);
pub const E0_MINOR_SCALE: Scale<8> = Scale::minor([E0, F0, G0, A0, B0, C1, D1, E1]);
pub const F0_MINOR_SCALE: Scale<8> = Scale::minor([F0, G0, A0, B0, C1, D1, E1, F1]);
pub const FSHARP0_MINOR_SCALE: Scale<8> = Scale::minor([F0, G0, A0, B0, C1, D1, E1, F1]);
pub const G0_MINOR_SCALE: Scale<8> = Scale::minor([G0, A0, B0, C1, D1, E1, F1, G1]);
pub const GSHARP0_MINOR_SCALE: Scale<8> = Scale::minor([G0, A0, B0, C1, D1, E1, F1, G1]);
pub const A0_MINOR_SCALE: Scale<8> = Scale::minor([A0, B0, C1, D1, E1, F1, G1, A1]);
pub const B0_MINOR_SCALE: Scale<8> = Scale::minor([B0, C1, D1, E1, F1, G1, A1, B1]);

// Octave 1 minor scales (C1-B1)
pub const C1_MINOR_SCALE: Scale<8> = Scale::minor([C1, D1, E1, F1, G1, A1, B1, C2]);
pub const CSHARP1_MINOR_SCALE: Scale<8> = Scale::minor([C1, D1, E1, F1, G1, A1, B1, C2]);
pub const D1_MINOR_SCALE: Scale<8> = Scale::minor([D1, E1, F1, G1, A1, B1, C2, D2]);
pub const DSHARP1_MINOR_SCALE: Scale<8> = Scale::minor([D1, E1, F1, G1, A1, B1, C2, D2]);
pub const E1_MINOR_SCALE: Scale<8> = Scale::minor([E1, F1, G1, A1, B1, C2, D2, E2]);
pub const F1_MINOR_SCALE: Scale<8> = Scale::minor([F1, G1, A1, B1, C2, D2, E2, F2]);
pub const FSHARP1_MINOR_SCALE: Scale<8> = Scale::minor([F1, G1, A1, B1, C2, D2, E2, F2]);
pub const G1_MINOR_SCALE: Scale<8> = Scale::minor([G1, A1, B1, C2, D2, E2, F2, G2]);
pub const GSHARP1_MINOR_SCALE: Scale<8> = Scale::minor([G1, A1, B1, C2, D2, E2, F2, G2]);
pub const A1_MINOR_SCALE: Scale<8> = Scale::minor([A1, B1, C2, D2, E2, F2, G2, A2]);
pub const B1_MINOR_SCALE: Scale<8> = Scale::minor([B1, C2, D2, E2, F2, G2, A2, B2]);

// Octave 2 minor scales (C2-B2)
pub const C2_MINOR_SCALE: Scale<8> = Scale::minor([C2, D2, E2, F2, G2, A2, B2, C3]);
pub const CSHARP2_MINOR_SCALE: Scale<8> = Scale::minor([C2, D2, E2, F2, G2, A2, B2, C3]);
pub const D2_MINOR_SCALE: Scale<8> = Scale::minor([D2, E2, F2, G2, A2, B2, C3, D3]);
pub const DSHARP2_MINOR_SCALE: Scale<8> = Scale::minor([D2, E2, F2, G2, A2, B2, C3, D3]);
pub const E2_MINOR_SCALE: Scale<8> = Scale::minor([E2, F2, G2, A2, B2, C3, D3, E3]);
pub const F2_MINOR_SCALE: Scale<8> = Scale::minor([F2, G2, A2, B2, C3, D3, E3, F3]);
pub const FSHARP2_MINOR_SCALE: Scale<8> = Scale::minor([F2, G2, A2, B2, C3, D3, E3, F3]);
pub const G2_MINOR_SCALE: Scale<8> = Scale::minor([G2, A2, B2, C3, D3, E3, F3, G3]);
pub const GSHARP2_MINOR_SCALE: Scale<8> = Scale::minor([G2, A2, B2, C3, D3, E3, F3, G3]);
pub const A2_MINOR_SCALE: Scale<8> = Scale::minor([A2, B2, C3, D3, E3, F3, G3, A3]);
pub const B2_MINOR_SCALE: Scale<8> = Scale::minor([B2, C3, D3, E3, F3, G3, A3, B3]);

// Octave 3 minor scales (C3-B3)
pub const C3_MINOR_SCALE: Scale<8> = Scale::minor([C3, D3, E3, F3, G3, A3, B3, C4]);
pub const CSHARP3_MINOR_SCALE: Scale<8> = Scale::minor([C3, D3, E3, F3, G3, A3, B3, C4]);
pub const D3_MINOR_SCALE: Scale<8> = Scale::minor([D3, E3, F3, G3, A3, B3, C4, D4]);
pub const DSHARP3_MINOR_SCALE: Scale<8> = Scale::minor([D3, E3, F3, G3, A3, B3, C4, D4]);
pub const E3_MINOR_SCALE: Scale<8> = Scale::minor([E3, F3, G3, A3, B3, C4, D4, E4]);
pub const F3_MINOR_SCALE: Scale<8> = Scale::minor([F3, G3, A3, B3, C4, D4, E4, F4]);
pub const FSHARP3_MINOR_SCALE: Scale<8> = Scale::minor([F3, G3, A3, B3, C4, D4, E4, F4]);
pub const G3_MINOR_SCALE: Scale<8> = Scale::minor([G3, A3, B3, C4, D4, E4, F4, G4]);
pub const GSHARP3_MINOR_SCALE: Scale<8> = Scale::minor([G3, A3, B3, C4, D4, E4, F4, G4]);
pub const A3_MINOR_SCALE: Scale<8> = Scale::minor([A3, B3, C4, D4, E4, F4, G4, A4]);
pub const B3_MINOR_SCALE: Scale<8> = Scale::minor([B3, C4, D4, E4, F4, G4, A4, B4]);

// Octave 4 minor scales (C4-B4, including middle C)
/// C minor scale starting from middle C (C4)
pub const C4_MINOR_SCALE: Scale<8> = Scale::minor([C4, D4, E4, F4, G4, A4, B4, C5]);
pub const CSHARP4_MINOR_SCALE: Scale<8> = Scale::minor([C4, D4, E4, F4, G4, A4, B4, C5]);
pub const D4_MINOR_SCALE: Scale<8> = Scale::minor([D4, E4, F4, G4, A4, B4, C5, D5]);
pub const DSHARP4_MINOR_SCALE: Scale<8> = Scale::minor([D4, E4, F4, G4, A4, B4, C5, D5]);
pub const E4_MINOR_SCALE: Scale<8> = Scale::minor([E4, F4, G4, A4, B4, C5, D5, E5]);
pub const F4_MINOR_SCALE: Scale<8> = Scale::minor([F4, G4, A4, B4, C5, D5, E5, F5]);
pub const FSHARP4_MINOR_SCALE: Scale<8> = Scale::minor([F4, G4, A4, B4, C5, D5, E5, F5]);
pub const G4_MINOR_SCALE: Scale<8> = Scale::minor([G4, A4, B4, C5, D5, E5, F5, G5]);
pub const GSHARP4_MINOR_SCALE: Scale<8> = Scale::minor([G4, A4, B4, C5, D5, E5, F5, G5]);
pub const A4_MINOR_SCALE: Scale<8> = Scale::minor([A4, B4, C5, D5, E5, F5, G5, A5]);
pub const B4_MINOR_SCALE: Scale<8> = Scale::minor([B4, C5, D5, E5, F5, G5, A5, B5]);

// Octave 5 minor scales (C5-B5)
pub const C5_MINOR_SCALE: Scale<8> = Scale::minor([C5, D5, E5, F5, G5, A5, B5, C6]);
pub const CSHARP5_MINOR_SCALE: Scale<8> = Scale::minor([C5, D5, E5, F5, G5, A5, B5, C6]);
pub const D5_MINOR_SCALE: Scale<8> = Scale::minor([D5, E5, F5, G5, A5, B5, C6, D6]);
pub const DSHARP5_MINOR_SCALE: Scale<8> = Scale::minor([D5, E5, F5, G5, A5, B5, C6, D6]);
pub const E5_MINOR_SCALE: Scale<8> = Scale::minor([E5, F5, G5, A5, B5, C6, D6, E6]);
pub const F5_MINOR_SCALE: Scale<8> = Scale::minor([F5, G5, A5, B5, C6, D6, E6, F6]);
pub const FSHARP5_MINOR_SCALE: Scale<8> = Scale::minor([F5, G5, A5, B5, C6, D6, E6, F6]);
pub const G5_MINOR_SCALE: Scale<8> = Scale::minor([G5, A5, B5, C6, D6, E6, F6, G6]);
pub const GSHARP5_MINOR_SCALE: Scale<8> = Scale::minor([G5, A5, B5, C6, D6, E6, F6, G6]);
pub const A5_MINOR_SCALE: Scale<8> = Scale::minor([A5, B5, C6, D6, E6, F6, G6, A6]);
pub const B5_MINOR_SCALE: Scale<8> = Scale::minor([B5, C6, D6, E6, F6, G6, A6, B6]);

// Octave 6 minor scales (C6-B6)
pub const C6_MINOR_SCALE: Scale<8> = Scale::minor([C6, D6, E6, F6, G6, A6, B6, C7]);
pub const CSHARP6_MINOR_SCALE: Scale<8> = Scale::minor([C6, D6, E6, F6, G6, A6, B6, C7]);
pub const D6_MINOR_SCALE: Scale<8> = Scale::minor([D6, E6, F6, G6, A6, B6, C7, D7]);
pub const DSHARP6_MINOR_SCALE: Scale<8> = Scale::minor([D6, E6, F6, G6, A6, B6, C7, D7]);
pub const E6_MINOR_SCALE: Scale<8> = Scale::minor([E6, F6, G6, A6, B6, C7, D7, E7]);
pub const F6_MINOR_SCALE: Scale<8> = Scale::minor([F6, G6, A6, B6, C7, D7, E7, F7]);
pub const FSHARP6_MINOR_SCALE: Scale<8> = Scale::minor([F6, G6, A6, B6, C7, D7, E7, F7]);
pub const G6_MINOR_SCALE: Scale<8> = Scale::minor([G6, A6, B6, C7, D7, E7, F7, G7]);
pub const GSHARP6_MINOR_SCALE: Scale<8> = Scale::minor([G6, A6, B6, C7, D7, E7, F7, G7]);
pub const A6_MINOR_SCALE: Scale<8> = Scale::minor([A6, B6, C7, D7, E7, F7, G7, A7]);
pub const B6_MINOR_SCALE: Scale<8> = Scale::minor([B6, C7, D7, E7, F7, G7, A7, B7]);

// Octave 7 minor scales (C7-B7)
pub const C7_MINOR_SCALE: Scale<8> = Scale::minor([C7, D7, E7, F7, G7, A7, B7, C8]);
pub const CSHARP7_MINOR_SCALE: Scale<8> = Scale::minor([C7, D7, E7, F7, G7, A7, B7, C8]);
pub const D7_MINOR_SCALE: Scale<8> = Scale::minor([D7, E7, F7, G7, A7, B7, C8, D8]);
pub const DSHARP7_MINOR_SCALE: Scale<8> = Scale::minor([D7, E7, F7, G7, A7, B7, C8, D8]);
pub const E7_MINOR_SCALE: Scale<8> = Scale::minor([E7, F7, G7, A7, B7, C8, D8, E8]);
pub const F7_MINOR_SCALE: Scale<8> = Scale::minor([F7, G7, A7, B7, C8, D8, E8, F8]);
pub const FSHARP7_MINOR_SCALE: Scale<8> = Scale::minor([F7, G7, A7, B7, C8, D8, E8, F8]);
pub const G7_MINOR_SCALE: Scale<8> = Scale::minor([G7, A7, B7, C8, D8, E8, F8, G8]);
pub const GSHARP7_MINOR_SCALE: Scale<8> = Scale::minor([G7, A7, B7, C8, D8, E8, F8, G8]);
pub const A7_MINOR_SCALE: Scale<8> = Scale::minor([A7, B7, C8, D8, E8, F8, G8, A8]);
pub const B7_MINOR_SCALE: Scale<8> = Scale::minor([B7, C8, D8, E8, F8, G8, A8, B8]);

// Octave 8 minor scales (C8-B8)
pub const C8_MINOR_SCALE: Scale<8> = Scale::minor([C8, D8, E8, F8, G8, A8, B8, C9]);
pub const CSHARP8_MINOR_SCALE: Scale<8> = Scale::minor([C8, D8, E8, F8, G8, A8, B8, C9]);
pub const D8_MINOR_SCALE: Scale<8> = Scale::minor([D8, E8, F8, G8, A8, B8, C9, D9]);
pub const DSHARP8_MINOR_SCALE: Scale<8> = Scale::minor([D8, E8, F8, G8, A8, B8, C9, D9]);
pub const E8_MINOR_SCALE: Scale<8> = Scale::minor([E8, F8, G8, A8, B8, C9, D9, E9]);
pub const F8_MINOR_SCALE: Scale<8> = Scale::minor([F8, G8, A8, B8, C9, D9, E9, F9]);
pub const FSHARP8_MINOR_SCALE: Scale<8> = Scale::minor([F8, G8, A8, B8, C9, D9, E9, F9]);
pub const G8_MINOR_SCALE: Scale<8> = Scale::minor([G8, A8, B8, C9, D9, E9, F9, G9]);
pub const GSHARP8_MINOR_SCALE: Scale<8> = Scale::minor([G8, A8, B8, C9, D9, E9, F9, G9]);

lazy_static! {
    /// HashMap containing all natural minor scales indexed by their root pitch.
    /// This collection provides quick access to any minor scale given its root note.
    ///
    /// The map includes scales for all valid MIDI note numbers, organized by octave.
    /// Each scale spans exactly one octave from its root note.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::MINOR_SCALES;
    /// use mozzart_std::A4;
    ///
    /// if let Some(scale) = MINOR_SCALES.get(&A4) {
    ///     // Use the A minor scale starting from A4
    /// }
    /// ```
    pub static ref MINOR_SCALES: std::collections::HashMap<Pitch, Scale<8>> = {
        let mut scales = HashMap::new();

        // Add octave-independent scales
        scales.insert(C, C_MINOR_SCALE);
        scales.insert(CSHARP, CSHARP_MINOR_SCALE);
        scales.insert(D, D_MINOR_SCALE);
        scales.insert(DSHARP, DSHARP_MINOR_SCALE);
        scales.insert(E, E_MINOR_SCALE);
        scales.insert(F, F_MINOR_SCALE);
        scales.insert(FSHARP, FSHARP_MINOR_SCALE);
        scales.insert(G, G_MINOR_SCALE);
        scales.insert(GSHARP, GSHARP_MINOR_SCALE);
        scales.insert(A, A_MINOR_SCALE);
        scales.insert(B, B_MINOR_SCALE);

        // Add octave 0 scales (lowest octave)
        scales.insert(C0, C0_MINOR_SCALE);
        scales.insert(CSHARP0, CSHARP0_MINOR_SCALE);
        scales.insert(D0, D0_MINOR_SCALE);
        scales.insert(DSHARP0, DSHARP0_MINOR_SCALE);
        scales.insert(E0, E0_MINOR_SCALE);
        scales.insert(F0, F0_MINOR_SCALE);
        scales.insert(FSHARP0, FSHARP0_MINOR_SCALE);
        scales.insert(G0, G0_MINOR_SCALE);
        scales.insert(GSHARP0, GSHARP0_MINOR_SCALE);
        scales.insert(A0, A0_MINOR_SCALE);
        scales.insert(B0, B0_MINOR_SCALE);

        // Add octave 4 scales (middle octave)
        scales.insert(C4, C4_MINOR_SCALE);
        scales.insert(F4, F4_MINOR_SCALE);
        scales.insert(CSHARP4, CSHARP4_MINOR_SCALE);
        scales.insert(D4, D4_MINOR_SCALE);
        scales.insert(DSHARP4, DSHARP4_MINOR_SCALE);
        scales.insert(E4, E4_MINOR_SCALE);
        scales.insert(FSHARP4, FSHARP4_MINOR_SCALE);
        scales.insert(G4, G4_MINOR_SCALE);
        scales.insert(GSHARP4, GSHARP4_MINOR_SCALE);
        scales.insert(A4, A4_MINOR_SCALE);
        scales.insert(B4, B4_MINOR_SCALE);

        // Add octave 1 scales (C1-B1)
        scales.insert(C1, C1_MINOR_SCALE);
        scales.insert(CSHARP1, CSHARP1_MINOR_SCALE);
        scales.insert(D1, D1_MINOR_SCALE);
        scales.insert(DSHARP1, DSHARP1_MINOR_SCALE);
        scales.insert(E1, E1_MINOR_SCALE);
        scales.insert(F1, F1_MINOR_SCALE);
        scales.insert(FSHARP1, FSHARP1_MINOR_SCALE);
        scales.insert(G1, G1_MINOR_SCALE);
        scales.insert(GSHARP1, GSHARP1_MINOR_SCALE);
        scales.insert(A1, A1_MINOR_SCALE);
        scales.insert(B1, B1_MINOR_SCALE);

        // Add octave 2 scales (C2-B2)
        scales.insert(C2, C2_MINOR_SCALE);
        scales.insert(CSHARP2, CSHARP2_MINOR_SCALE);
        scales.insert(D2, D2_MINOR_SCALE);
        scales.insert(DSHARP2, DSHARP2_MINOR_SCALE);
        scales.insert(E2, E2_MINOR_SCALE);
        scales.insert(F2, F2_MINOR_SCALE);
        scales.insert(FSHARP2, FSHARP2_MINOR_SCALE);
        scales.insert(G2, G2_MINOR_SCALE);
        scales.insert(GSHARP2, GSHARP2_MINOR_SCALE);
        scales.insert(A2, A2_MINOR_SCALE);
        scales.insert(B2, B2_MINOR_SCALE);

        // Add octave 3 scales (C3-B3)
        scales.insert(C3, C3_MINOR_SCALE);
        scales.insert(CSHARP3, CSHARP3_MINOR_SCALE);
        scales.insert(D3, D3_MINOR_SCALE);
        scales.insert(DSHARP3, DSHARP3_MINOR_SCALE);
        scales.insert(E3, E3_MINOR_SCALE);
        scales.insert(F3, F3_MINOR_SCALE);
        scales.insert(FSHARP3, FSHARP3_MINOR_SCALE);
        scales.insert(G3, G3_MINOR_SCALE);
        scales.insert(GSHARP3, GSHARP3_MINOR_SCALE);
        scales.insert(A3, A3_MINOR_SCALE);
        scales.insert(B3, B3_MINOR_SCALE);

        // Add octave 5 scales (C5-B5)
        scales.insert(C5, C5_MINOR_SCALE);
        scales.insert(CSHARP5, CSHARP5_MINOR_SCALE);
        scales.insert(D5, D5_MINOR_SCALE);
        scales.insert(DSHARP5, DSHARP5_MINOR_SCALE);
        scales.insert(E5, E5_MINOR_SCALE);
        scales.insert(F5, F5_MINOR_SCALE);
        scales.insert(FSHARP5, FSHARP5_MINOR_SCALE);
        scales.insert(G5, G5_MINOR_SCALE);
        scales.insert(GSHARP5, GSHARP5_MINOR_SCALE);
        scales.insert(A5, A5_MINOR_SCALE);
        scales.insert(B5, B5_MINOR_SCALE);

        // Add octave 6 scales (C6-B6)
        scales.insert(C6, C6_MINOR_SCALE);
        scales.insert(CSHARP6, CSHARP6_MINOR_SCALE);
        scales.insert(D6, D6_MINOR_SCALE);
        scales.insert(DSHARP6, DSHARP6_MINOR_SCALE);
        scales.insert(E6, E6_MINOR_SCALE);
        scales.insert(F6, F6_MINOR_SCALE);
        scales.insert(FSHARP6, FSHARP6_MINOR_SCALE);
        scales.insert(G6, G6_MINOR_SCALE);
        scales.insert(GSHARP6, GSHARP6_MINOR_SCALE);
        scales.insert(A6, A6_MINOR_SCALE);
        scales.insert(B6, B6_MINOR_SCALE);

        // Add octave 7 scales (C7-B7)
        scales.insert(C7, C7_MINOR_SCALE);
        scales.insert(CSHARP7, CSHARP7_MINOR_SCALE);
        scales.insert(D7, D7_MINOR_SCALE);
        scales.insert(DSHARP7, DSHARP7_MINOR_SCALE);
        scales.insert(E7, E7_MINOR_SCALE);
        scales.insert(F7, F7_MINOR_SCALE);
        scales.insert(FSHARP7, FSHARP7_MINOR_SCALE);
        scales.insert(G7, G7_MINOR_SCALE);
        scales.insert(GSHARP7, GSHARP7_MINOR_SCALE);
        scales.insert(A7, A7_MINOR_SCALE);
        scales.insert(B7, B7_MINOR_SCALE);

        // Add octave 8 scales (C8-B8)
        scales.insert(C8, C8_MINOR_SCALE);
        scales.insert(CSHARP8, CSHARP8_MINOR_SCALE);
        scales.insert(D8, D8_MINOR_SCALE);
        scales.insert(DSHARP8, DSHARP8_MINOR_SCALE);
        scales.insert(E8, E8_MINOR_SCALE);
        scales.insert(F8, F8_MINOR_SCALE);
        scales.insert(FSHARP8, FSHARP8_MINOR_SCALE);
        scales.insert(G8, G8_MINOR_SCALE);
        scales.insert(GSHARP8, GSHARP8_MINOR_SCALE);

        scales
    };
}
