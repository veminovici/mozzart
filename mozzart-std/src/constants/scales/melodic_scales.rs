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
use crate::{Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

// Octave-independent melodic minor scales (pitch class only)
/// C melodic minor scale (C-D-E♭-F-G-A-B-C)
pub const C_MELODIC_SCALE: Scale<8> = Scale::melodic([C, D, E, F, G, A, B, C]);
/// C♯/D♭ melodic minor scale
pub const CSHARP_MELODIC_SCALE: Scale<8> = Scale::melodic([C, D, E, F, G, A, B, C]);
/// D melodic minor scale
pub const D_MELODIC_SCALE: Scale<8> = Scale::melodic([D, E, F, G, A, B, C, D]);
/// D♯/E♭ melodic minor scale
pub const DSHARP_MELODIC_SCALE: Scale<8> = Scale::melodic([D, E, F, G, A, B, C, D]);
/// E melodic minor scale
pub const E_MELODIC_SCALE: Scale<8> = Scale::melodic([E, F, G, A, B, C, D, E]);
/// F melodic minor scale
pub const F_MELODIC_SCALE: Scale<8> = Scale::melodic([F, G, A, B, C, D, E, F]);
/// F♯/G♭ melodic minor scale
pub const FSHARP_MELODIC_SCALE: Scale<8> = Scale::melodic([F, G, A, B, C, D, E, F]);
/// G melodic minor scale
pub const G_MELODIC_SCALE: Scale<8> = Scale::melodic([G, A, B, C, D, E, F, G]);
/// G♯/A♭ melodic minor scale
pub const GSHARP_MELODIC_SCALE: Scale<8> = Scale::melodic([G, A, B, C, D, E, F, G]);
/// A melodic minor scale
pub const A_MELODIC_SCALE: Scale<8> = Scale::melodic([A, B, C, D, E, F, G, A]);
/// B melodic minor scale
pub const B_MELODIC_SCALE: Scale<8> = Scale::melodic([B, C, D, E, F, G, A, B]);

// Octave 0 melodic minor scales (C0-B0, lowest octave)
/// C melodic minor scale starting from C0 (lowest octave)
pub const C0_MELODIC_SCALE: Scale<8> = Scale::melodic([C0, D0, E0, F0, G0, A0, B0, C1]);
/// C♯/D♭ melodic minor scale starting from C♯0
pub const CSHARP0_MELODIC_SCALE: Scale<8> = Scale::melodic([C0, D0, E0, F0, G0, A0, B0, C1]);
/// D melodic minor scale starting from D0
pub const D0_MELODIC_SCALE: Scale<8> = Scale::melodic([D0, E0, F0, G0, A0, B0, C1, D1]);
/// D♯/E♭ melodic minor scale starting from D♯0
pub const DSHARP0_MELODIC_SCALE: Scale<8> = Scale::melodic([D0, E0, F0, G0, A0, B0, C1, D1]);
/// E melodic minor scale starting from E0
pub const E0_MELODIC_SCALE: Scale<8> = Scale::melodic([E0, F0, G0, A0, B0, C1, D1, E1]);
/// F melodic minor scale starting from F0
pub const F0_MELODIC_SCALE: Scale<8> = Scale::melodic([F0, G0, A0, B0, C1, D1, E1, F1]);
/// F♯/G♭ melodic minor scale starting from F♯0
pub const FSHARP0_MELODIC_SCALE: Scale<8> = Scale::melodic([F0, G0, A0, B0, C1, D1, E1, F1]);
/// G melodic minor scale starting from G0
pub const G0_MELODIC_SCALE: Scale<8> = Scale::melodic([G0, A0, B0, C1, D1, E1, F1, G1]);
/// G♯/A♭ melodic minor scale starting from G♯0
pub const GSHARP0_MELODIC_SCALE: Scale<8> = Scale::melodic([G0, A0, B0, C1, D1, E1, F1, G1]);
/// A melodic minor scale starting from A0
pub const A0_MELODIC_SCALE: Scale<8> = Scale::melodic([A0, B0, C1, D1, E1, F1, G1, A1]);
/// B melodic minor scale starting from B0
pub const B0_MELODIC_SCALE: Scale<8> = Scale::melodic([B0, C1, D1, E1, F1, G1, A1, B1]);

// Octave 1 melodic minor scales (C1-B1)
/// C melodic minor scale starting from C1
pub const C1_MELODIC_SCALE: Scale<8> = Scale::melodic([C1, D1, E1, F1, G1, A1, B1, C2]);
/// C♯/D♭ melodic minor scale starting from C♯1
pub const CSHARP1_MELODIC_SCALE: Scale<8> = Scale::melodic([C1, D1, E1, F1, G1, A1, B1, C2]);
/// D melodic minor scale starting from D1
pub const D1_MELODIC_SCALE: Scale<8> = Scale::melodic([D1, E1, F1, G1, A1, B1, C2, D2]);
/// D♯/E♭ melodic minor scale starting from D♯1
pub const DSHARP1_MELODIC_SCALE: Scale<8> = Scale::melodic([D1, E1, F1, G1, A1, B1, C2, D2]);
/// E melodic minor scale starting from E1
pub const E1_MELODIC_SCALE: Scale<8> = Scale::melodic([E1, F1, G1, A1, B1, C2, D2, E2]);
/// F melodic minor scale starting from F1
pub const F1_MELODIC_SCALE: Scale<8> = Scale::melodic([F1, G1, A1, B1, C2, D2, E2, F2]);
/// F♯/G♭ melodic minor scale starting from F♯1
pub const FSHARP1_MELODIC_SCALE: Scale<8> = Scale::melodic([F1, G1, A1, B1, C2, D2, E2, F2]);
/// G melodic minor scale starting from G1
pub const G1_MELODIC_SCALE: Scale<8> = Scale::melodic([G1, A1, B1, C2, D2, E2, F2, G2]);
/// G♯/A♭ melodic minor scale starting from G♯1
pub const GSHARP1_MELODIC_SCALE: Scale<8> = Scale::melodic([G1, A1, B1, C2, D2, E2, F2, G2]);
/// A melodic minor scale starting from A1
pub const A1_MELODIC_SCALE: Scale<8> = Scale::melodic([A1, B1, C2, D2, E2, F2, G2, A2]);
/// B melodic minor scale starting from B1
pub const B1_MELODIC_SCALE: Scale<8> = Scale::melodic([B1, C2, D2, E2, F2, G2, A2, B2]);

// Octave 2 melodic minor scales (C2-B2)
/// C melodic minor scale starting from C2
pub const C2_MELODIC_SCALE: Scale<8> = Scale::melodic([C2, D2, E2, F2, G2, A2, B2, C3]);
/// C♯/D♭ melodic minor scale starting from C♯2
pub const CSHARP2_MELODIC_SCALE: Scale<8> = Scale::melodic([C2, D2, E2, F2, G2, A2, B2, C3]);
/// D melodic minor scale starting from D2
pub const D2_MELODIC_SCALE: Scale<8> = Scale::melodic([D2, E2, F2, G2, A2, B2, C3, D3]);
/// D♯/E♭ melodic minor scale starting from D♯2
pub const DSHARP2_MELODIC_SCALE: Scale<8> = Scale::melodic([D2, E2, F2, G2, A2, B2, C3, D3]);
/// E melodic minor scale starting from E2
pub const E2_MELODIC_SCALE: Scale<8> = Scale::melodic([E2, F2, G2, A2, B2, C3, D3, E3]);
/// F melodic minor scale starting from F2
pub const F2_MELODIC_SCALE: Scale<8> = Scale::melodic([F2, G2, A2, B2, C3, D3, E3, F3]);
/// F♯/G♭ melodic minor scale starting from F♯2
pub const FSHARP2_MELODIC_SCALE: Scale<8> = Scale::melodic([F2, G2, A2, B2, C3, D3, E3, F3]);
/// G melodic minor scale starting from G2
pub const G2_MELODIC_SCALE: Scale<8> = Scale::melodic([G2, A2, B2, C3, D3, E3, F3, G3]);
/// G♯/A♭ melodic minor scale starting from G♯2
pub const GSHARP2_MELODIC_SCALE: Scale<8> = Scale::melodic([G2, A2, B2, C3, D3, E3, F3, G3]);
/// A melodic minor scale starting from A2
pub const A2_MELODIC_SCALE: Scale<8> = Scale::melodic([A2, B2, C3, D3, E3, F3, G3, A3]);
/// B melodic minor scale starting from B2
pub const B2_MELODIC_SCALE: Scale<8> = Scale::melodic([B2, C3, D3, E3, F3, G3, A3, B3]);

// Octave 3 melodic minor scales (C3-B3)
/// C melodic minor scale starting from C3
pub const C3_MELODIC_SCALE: Scale<8> = Scale::melodic([C3, D3, E3, F3, G3, A3, B3, C4]);
/// C♯/D♭ melodic minor scale starting from C♯3
pub const CSHARP3_MELODIC_SCALE: Scale<8> = Scale::melodic([C3, D3, E3, F3, G3, A3, B3, C4]);
/// D melodic minor scale starting from D3
pub const D3_MELODIC_SCALE: Scale<8> = Scale::melodic([D3, E3, F3, G3, A3, B3, C4, D4]);
/// D♯/E♭ melodic minor scale starting from D♯3
pub const DSHARP3_MELODIC_SCALE: Scale<8> = Scale::melodic([D3, E3, F3, G3, A3, B3, C4, D4]);
/// E melodic minor scale starting from E3
pub const E3_MELODIC_SCALE: Scale<8> = Scale::melodic([E3, F3, G3, A3, B3, C4, D4, E4]);
/// F melodic minor scale starting from F3
pub const F3_MELODIC_SCALE: Scale<8> = Scale::melodic([F3, G3, A3, B3, C4, D4, E4, F4]);
/// F♯/G♭ melodic minor scale starting from F♯3
pub const FSHARP3_MELODIC_SCALE: Scale<8> = Scale::melodic([F3, G3, A3, B3, C4, D4, E4, F4]);
/// G melodic minor scale starting from G3
pub const G3_MELODIC_SCALE: Scale<8> = Scale::melodic([G3, A3, B3, C4, D4, E4, F4, G4]);
/// G♯/A♭ melodic minor scale starting from G♯3
pub const GSHARP3_MELODIC_SCALE: Scale<8> = Scale::melodic([G3, A3, B3, C4, D4, E4, F4, G4]);
/// A melodic minor scale starting from A3
pub const A3_MELODIC_SCALE: Scale<8> = Scale::melodic([A3, B3, C4, D4, E4, F4, G4, A4]);
/// B melodic minor scale starting from B3
pub const B3_MELODIC_SCALE: Scale<8> = Scale::melodic([B3, C4, D4, E4, F4, G4, A4, B4]);

// Octave 4 melodic minor scales (C4-B4, including middle C)
/// C melodic minor scale starting from middle C (C4)
pub const C4_MELODIC_SCALE: Scale<8> = Scale::melodic([C4, D4, E4, F4, G4, A4, B4, C5]);
/// C♯/D♭ melodic minor scale starting from C♯4
pub const CSHARP4_MELODIC_SCALE: Scale<8> = Scale::melodic([C4, D4, E4, F4, G4, A4, B4, C5]);
/// D melodic minor scale starting from D4
pub const D4_MELODIC_SCALE: Scale<8> = Scale::melodic([D4, E4, F4, G4, A4, B4, C5, D5]);
/// D♯/E♭ melodic minor scale starting from D♯4
pub const DSHARP4_MELODIC_SCALE: Scale<8> = Scale::melodic([D4, E4, F4, G4, A4, B4, C5, D5]);
/// E melodic minor scale starting from E4
pub const E4_MELODIC_SCALE: Scale<8> = Scale::melodic([E4, F4, G4, A4, B4, C5, D5, E5]);
/// F melodic minor scale starting from F4
pub const F4_MELODIC_SCALE: Scale<8> = Scale::melodic([F4, G4, A4, B4, C5, D5, E5, F5]);
/// F♯/G♭ melodic minor scale starting from F♯4
pub const FSHARP4_MELODIC_SCALE: Scale<8> = Scale::melodic([F4, G4, A4, B4, C5, D5, E5, F5]);
/// G melodic minor scale starting from G4
pub const G4_MELODIC_SCALE: Scale<8> = Scale::melodic([G4, A4, B4, C5, D5, E5, F5, G5]);
/// G♯/A♭ melodic minor scale starting from G♯4
pub const GSHARP4_MELODIC_SCALE: Scale<8> = Scale::melodic([G4, A4, B4, C5, D5, E5, F5, G5]);
/// A melodic minor scale starting from A4 (A440)
pub const A4_MELODIC_SCALE: Scale<8> = Scale::melodic([A4, B4, C5, D5, E5, F5, G5, A5]);
/// B melodic minor scale starting from B4
pub const B4_MELODIC_SCALE: Scale<8> = Scale::melodic([B4, C5, D5, E5, F5, G5, A5, B5]);

// Octave 5 melodic minor scales (C5-B5)
/// C melodic minor scale starting from C5
pub const C5_MELODIC_SCALE: Scale<8> = Scale::melodic([C5, D5, E5, F5, G5, A5, B5, C6]);
/// C♯/D♭ melodic minor scale starting from C♯5
pub const CSHARP5_MELODIC_SCALE: Scale<8> = Scale::melodic([C5, D5, E5, F5, G5, A5, B5, C6]);
/// D melodic minor scale starting from D5
pub const D5_MELODIC_SCALE: Scale<8> = Scale::melodic([D5, E5, F5, G5, A5, B5, C6, D6]);
/// D♯/E♭ melodic minor scale starting from D♯5
pub const DSHARP5_MELODIC_SCALE: Scale<8> = Scale::melodic([D5, E5, F5, G5, A5, B5, C6, D6]);
/// E melodic minor scale starting from E5
pub const E5_MELODIC_SCALE: Scale<8> = Scale::melodic([E5, F5, G5, A5, B5, C6, D6, E6]);
/// F melodic minor scale starting from F5
pub const F5_MELODIC_SCALE: Scale<8> = Scale::melodic([F5, G5, A5, B5, C6, D6, E6, F6]);
/// F♯/G♭ melodic minor scale starting from F♯5
pub const FSHARP5_MELODIC_SCALE: Scale<8> = Scale::melodic([F5, G5, A5, B5, C6, D6, E6, F6]);
/// G melodic minor scale starting from G5
pub const G5_MELODIC_SCALE: Scale<8> = Scale::melodic([G5, A5, B5, C6, D6, E6, F6, G6]);
/// G♯/A♭ melodic minor scale starting from G♯5
pub const GSHARP5_MELODIC_SCALE: Scale<8> = Scale::melodic([G5, A5, B5, C6, D6, E6, F6, G6]);
/// A melodic minor scale starting from A5
pub const A5_MELODIC_SCALE: Scale<8> = Scale::melodic([A5, B5, C6, D6, E6, F6, G6, A6]);
/// B melodic minor scale starting from B5
pub const B5_MELODIC_SCALE: Scale<8> = Scale::melodic([B5, C6, D6, E6, F6, G6, A6, B6]);

// Octave 6 melodic minor scales (C6-B6)
/// C melodic minor scale starting from C6
pub const C6_MELODIC_SCALE: Scale<8> = Scale::melodic([C6, D6, E6, F6, G6, A6, B6, C7]);
/// C♯/D♭ melodic minor scale starting from C♯6
pub const CSHARP6_MELODIC_SCALE: Scale<8> = Scale::melodic([C6, D6, E6, F6, G6, A6, B6, C7]);
/// D melodic minor scale starting from D6
pub const D6_MELODIC_SCALE: Scale<8> = Scale::melodic([D6, E6, F6, G6, A6, B6, C7, D7]);
/// D♯/E♭ melodic minor scale starting from D♯6
pub const DSHARP6_MELODIC_SCALE: Scale<8> = Scale::melodic([D6, E6, F6, G6, A6, B6, C7, D7]);
/// E melodic minor scale starting from E6
pub const E6_MELODIC_SCALE: Scale<8> = Scale::melodic([E6, F6, G6, A6, B6, C7, D7, E7]);
/// F melodic minor scale starting from F6
pub const F6_MELODIC_SCALE: Scale<8> = Scale::melodic([F6, G6, A6, B6, C7, D7, E7, F7]);
/// F♯/G♭ melodic minor scale starting from F♯6
pub const FSHARP6_MELODIC_SCALE: Scale<8> = Scale::melodic([F6, G6, A6, B6, C7, D7, E7, F7]);
/// G melodic minor scale starting from G6
pub const G6_MELODIC_SCALE: Scale<8> = Scale::melodic([G6, A6, B6, C7, D7, E7, F7, G7]);
/// G♯/A♭ melodic minor scale starting from G♯6
pub const GSHARP6_MELODIC_SCALE: Scale<8> = Scale::melodic([G6, A6, B6, C7, D7, E7, F7, G7]);
/// A melodic minor scale starting from A6
pub const A6_MELODIC_SCALE: Scale<8> = Scale::melodic([A6, B6, C7, D7, E7, F7, G7, A7]);
/// B melodic minor scale starting from B6
pub const B6_MELODIC_SCALE: Scale<8> = Scale::melodic([B6, C7, D7, E7, F7, G7, A7, B7]);

// Octave 7 melodic minor scales (C7-B7)
/// C melodic minor scale starting from C7
pub const C7_MELODIC_SCALE: Scale<8> = Scale::melodic([C7, D7, E7, F7, G7, A7, B7, C8]);
/// C♯/D♭ melodic minor scale starting from C♯7
pub const CSHARP7_MELODIC_SCALE: Scale<8> = Scale::melodic([C7, D7, E7, F7, G7, A7, B7, C8]);
/// D melodic minor scale starting from D7
pub const D7_MELODIC_SCALE: Scale<8> = Scale::melodic([D7, E7, F7, G7, A7, B7, C8, D8]);
/// D♯/E♭ melodic minor scale starting from D♯7
pub const DSHARP7_MELODIC_SCALE: Scale<8> = Scale::melodic([D7, E7, F7, G7, A7, B7, C8, D8]);
/// E melodic minor scale starting from E7
pub const E7_MELODIC_SCALE: Scale<8> = Scale::melodic([E7, F7, G7, A7, B7, C8, D8, E8]);
/// F melodic minor scale starting from F7
pub const F7_MELODIC_SCALE: Scale<8> = Scale::melodic([F7, G7, A7, B7, C8, D8, E8, F8]);
/// F♯/G♭ melodic minor scale starting from F♯7
pub const FSHARP7_MELODIC_SCALE: Scale<8> = Scale::melodic([F7, G7, A7, B7, C8, D8, E8, F8]);
/// G melodic minor scale starting from G7
pub const G7_MELODIC_SCALE: Scale<8> = Scale::melodic([G7, A7, B7, C8, D8, E8, F8, G8]);
/// G♯/A♭ melodic minor scale starting from G♯7
pub const GSHARP7_MELODIC_SCALE: Scale<8> = Scale::melodic([G7, A7, B7, C8, D8, E8, F8, G8]);
/// A melodic minor scale starting from A7
pub const A7_MELODIC_SCALE: Scale<8> = Scale::melodic([A7, B7, C8, D8, E8, F8, G8, A8]);
/// B melodic minor scale starting from B7
pub const B7_MELODIC_SCALE: Scale<8> = Scale::melodic([B7, C8, D8, E8, F8, G8, A8, B8]);

// Octave 8 melodic minor scales (C8-G#8)
/// C melodic minor scale starting from C8 (highest octave)
pub const C8_MELODIC_SCALE: Scale<8> = Scale::melodic([C8, D8, E8, F8, G8, A8, B8, C9]);
/// C♯/D♭ melodic minor scale starting from C♯8
pub const CSHARP8_MELODIC_SCALE: Scale<8> = Scale::melodic([C8, D8, E8, F8, G8, A8, B8, C9]);
/// D melodic minor scale starting from D8
pub const D8_MELODIC_SCALE: Scale<8> = Scale::melodic([D8, E8, F8, G8, A8, B8, C9, D9]);
/// D♯/E♭ melodic minor scale starting from D♯8
pub const DSHARP8_MELODIC_SCALE: Scale<8> = Scale::melodic([D8, E8, F8, G8, A8, B8, C9, D9]);
/// E melodic minor scale starting from E8
pub const E8_MELODIC_SCALE: Scale<8> = Scale::melodic([E8, F8, G8, A8, B8, C9, D9, E9]);
/// F melodic minor scale starting from F8
pub const F8_MELODIC_SCALE: Scale<8> = Scale::melodic([F8, G8, A8, B8, C9, D9, E9, F9]);
/// F♯/G♭ melodic minor scale starting from F♯8
pub const FSHARP8_MELODIC_SCALE: Scale<8> = Scale::melodic([F8, G8, A8, B8, C9, D9, E9, F9]);
/// G melodic minor scale starting from G8
pub const G8_MELODIC_SCALE: Scale<8> = Scale::melodic([G8, A8, B8, C9, D9, E9, F9, G9]);
/// G♯/A♭ melodic minor scale starting from G♯8 (highest available melodic minor scale)
pub const GSHARP8_MELODIC_SCALE: Scale<8> = Scale::melodic([G8, A8, B8, C9, D9, E9, F9, G9]);

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
    pub static ref MELODIC_SCALES: HashMap<Pitch, Scale<8>> = {
        let mut scales = HashMap::new();

        // Add octave-independent scales (based on pitch class only)
        scales.insert(C, C_MELODIC_SCALE);
        scales.insert(CSHARP, CSHARP_MELODIC_SCALE);
        scales.insert(D, D_MELODIC_SCALE);
        scales.insert(DSHARP, DSHARP_MELODIC_SCALE);
        scales.insert(E, E_MELODIC_SCALE);
        scales.insert(F, F_MELODIC_SCALE);
        scales.insert(FSHARP, FSHARP_MELODIC_SCALE);
        scales.insert(G, G_MELODIC_SCALE);
        scales.insert(GSHARP, GSHARP_MELODIC_SCALE);
        scales.insert(A, A_MELODIC_SCALE);
        scales.insert(B, B_MELODIC_SCALE);

        // Add octave 0 scales (lowest octave)
        scales.insert(C0, C0_MELODIC_SCALE);
        scales.insert(CSHARP0, CSHARP0_MELODIC_SCALE);
        scales.insert(D0, D0_MELODIC_SCALE);
        scales.insert(DSHARP0, DSHARP0_MELODIC_SCALE);
        scales.insert(E0, E0_MELODIC_SCALE);
        scales.insert(F0, F0_MELODIC_SCALE);
        scales.insert(FSHARP0, FSHARP0_MELODIC_SCALE);
        scales.insert(G0, G0_MELODIC_SCALE);
        scales.insert(GSHARP0, GSHARP0_MELODIC_SCALE);
        scales.insert(A0, A0_MELODIC_SCALE);
        scales.insert(B0, B0_MELODIC_SCALE);

        // Add octave 1 scales (C1-B1)
        scales.insert(C1, C1_MELODIC_SCALE);
        scales.insert(CSHARP1, CSHARP1_MELODIC_SCALE);
        scales.insert(D1, D1_MELODIC_SCALE);
        scales.insert(DSHARP1, DSHARP1_MELODIC_SCALE);
        scales.insert(E1, E1_MELODIC_SCALE);
        scales.insert(F1, F1_MELODIC_SCALE);
        scales.insert(FSHARP1, FSHARP1_MELODIC_SCALE);
        scales.insert(G1, G1_MELODIC_SCALE);
        scales.insert(GSHARP1, GSHARP1_MELODIC_SCALE);
        scales.insert(A1, A1_MELODIC_SCALE);
        scales.insert(B1, B1_MELODIC_SCALE);

        // Add octave 2 scales (C2-B2)
        scales.insert(C2, C2_MELODIC_SCALE);
        scales.insert(CSHARP2, CSHARP2_MELODIC_SCALE);
        scales.insert(D2, D2_MELODIC_SCALE);
        scales.insert(DSHARP2, DSHARP2_MELODIC_SCALE);
        scales.insert(E2, E2_MELODIC_SCALE);
        scales.insert(F2, F2_MELODIC_SCALE);
        scales.insert(FSHARP2, FSHARP2_MELODIC_SCALE);
        scales.insert(G2, G2_MELODIC_SCALE);
        scales.insert(GSHARP2, GSHARP2_MELODIC_SCALE);
        scales.insert(A2, A2_MELODIC_SCALE);
        scales.insert(B2, B2_MELODIC_SCALE);

        // Add octave 3 scales (C3-B3)
        scales.insert(C3, C3_MELODIC_SCALE);
        scales.insert(CSHARP3, CSHARP3_MELODIC_SCALE);
        scales.insert(D3, D3_MELODIC_SCALE);
        scales.insert(DSHARP3, DSHARP3_MELODIC_SCALE);
        scales.insert(E3, E3_MELODIC_SCALE);
        scales.insert(F3, F3_MELODIC_SCALE);
        scales.insert(FSHARP3, FSHARP3_MELODIC_SCALE);
        scales.insert(G3, G3_MELODIC_SCALE);
        scales.insert(GSHARP3, GSHARP3_MELODIC_SCALE);
        scales.insert(A3, A3_MELODIC_SCALE);
        scales.insert(B3, B3_MELODIC_SCALE);

        // Add octave 4 scales (C4-B4, including middle C)
        scales.insert(C4, C4_MELODIC_SCALE);  // Middle C
        scales.insert(CSHARP4, CSHARP4_MELODIC_SCALE);
        scales.insert(D4, D4_MELODIC_SCALE);
        scales.insert(DSHARP4, DSHARP4_MELODIC_SCALE);
        scales.insert(E4, E4_MELODIC_SCALE);
        scales.insert(F4, F4_MELODIC_SCALE);
        scales.insert(FSHARP4, FSHARP4_MELODIC_SCALE);
        scales.insert(G4, G4_MELODIC_SCALE);
        scales.insert(GSHARP4, GSHARP4_MELODIC_SCALE);
        scales.insert(A4, A4_MELODIC_SCALE);  // A440 reference pitch
        scales.insert(B4, B4_MELODIC_SCALE);

        // Add octave 5 scales (C5-B5)
        scales.insert(C5, C5_MELODIC_SCALE);
        scales.insert(CSHARP5, CSHARP5_MELODIC_SCALE);
        scales.insert(D5, D5_MELODIC_SCALE);
        scales.insert(DSHARP5, DSHARP5_MELODIC_SCALE);
        scales.insert(E5, E5_MELODIC_SCALE);
        scales.insert(F5, F5_MELODIC_SCALE);
        scales.insert(FSHARP5, FSHARP5_MELODIC_SCALE);
        scales.insert(G5, G5_MELODIC_SCALE);
        scales.insert(GSHARP5, GSHARP5_MELODIC_SCALE);
        scales.insert(A5, A5_MELODIC_SCALE);
        scales.insert(B5, B5_MELODIC_SCALE);

        // Add octave 6 scales (C6-B6)
        scales.insert(C6, C6_MELODIC_SCALE);
        scales.insert(CSHARP6, CSHARP6_MELODIC_SCALE);
        scales.insert(D6, D6_MELODIC_SCALE);
        scales.insert(DSHARP6, DSHARP6_MELODIC_SCALE);
        scales.insert(E6, E6_MELODIC_SCALE);
        scales.insert(F6, F6_MELODIC_SCALE);
        scales.insert(FSHARP6, FSHARP6_MELODIC_SCALE);
        scales.insert(G6, G6_MELODIC_SCALE);
        scales.insert(GSHARP6, GSHARP6_MELODIC_SCALE);
        scales.insert(A6, A6_MELODIC_SCALE);
        scales.insert(B6, B6_MELODIC_SCALE);

        // Add octave 7 scales (C7-B7)
        scales.insert(C7, C7_MELODIC_SCALE);
        scales.insert(CSHARP7, CSHARP7_MELODIC_SCALE);
        scales.insert(D7, D7_MELODIC_SCALE);
        scales.insert(DSHARP7, DSHARP7_MELODIC_SCALE);
        scales.insert(E7, E7_MELODIC_SCALE);
        scales.insert(F7, F7_MELODIC_SCALE);
        scales.insert(FSHARP7, FSHARP7_MELODIC_SCALE);
        scales.insert(G7, G7_MELODIC_SCALE);
        scales.insert(GSHARP7, GSHARP7_MELODIC_SCALE);
        scales.insert(A7, A7_MELODIC_SCALE);
        scales.insert(B7, B7_MELODIC_SCALE);

        // Add octave 8 scales (C8-G#8, highest octave)
        scales.insert(C8, C8_MELODIC_SCALE);
        scales.insert(CSHARP8, CSHARP8_MELODIC_SCALE);
        scales.insert(D8, D8_MELODIC_SCALE);
        scales.insert(DSHARP8, DSHARP8_MELODIC_SCALE);
        scales.insert(E8, E8_MELODIC_SCALE);
        scales.insert(F8, F8_MELODIC_SCALE);
        scales.insert(FSHARP8, FSHARP8_MELODIC_SCALE);
        scales.insert(G8, G8_MELODIC_SCALE);
        scales.insert(GSHARP8, GSHARP8_MELODIC_SCALE);  // Highest available melodic minor scale

        scales
    };
}
