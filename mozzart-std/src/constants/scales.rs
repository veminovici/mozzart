//! Common musical scale constants
//!
//! This module provides predefined constants for frequently used musical scales.
//! Each scale constant is an octave-spanning scale (8 notes) starting from a specific root pitch.
//!
//! # Scale Types
//! For each root note in octave 4, the following scale types are provided:
//! - Major scales (e.g., `C4_MAJOR_SCALE`, `D4_MAJOR_SCALE`)
//! - Natural minor scales (e.g., `C4_MINOR_SCALE`, `A4_MINOR_SCALE`)
//! - Harmonic minor scales (e.g., `C4_HARMONIC_MINOR_SCALE`)
//! - Melodic minor scales (e.g., `C4_MELODIC_MINOR_SCALE`)
//!
//! # Scale Patterns
//! All scales follow standard musical theory patterns:
//! - Major: W-W-H-W-W-W-H (whole and half steps)
//! - Natural Minor: W-H-W-W-H-W-W
//! - Harmonic Minor: W-H-W-W-H-W+H-H
//! - Melodic Minor (ascending): W-H-W-W-W-W-H
//!
//! # Note Naming
//! - Natural notes: C, D, E, F, G, A, B
//! - Sharp notes: C♯, D♯, F♯, G♯
//! - Each scale spans one octave (e.g., C4 to C5)

use crate::constants::*;
use crate::Scale;

/// C4 major scale (C4 to C5)
pub const C4_MAJOR_SCALE: Scale<8> = Scale::major([C4, D4, E4, F4, G4, A4, B4, C5]);

/// C♯4 major scale (C♯4 to C♯5)
pub const CSHARP4_MAJOR_SCALE: Scale<8> = Scale::major([C4, D4, E4, F4, G4, A4, B4, C5]);

/// D4 major scale (D4 to D5)
pub const D4_MAJOR_SCALE: Scale<8> = Scale::major([D4, E4, F4, G4, A4, B4, C5, D5]);

/// D♯4 major scale (D♯4 to D♯5)
pub const DSHARP4_MAJOR_SCALE: Scale<8> = Scale::major([D4, E4, F4, G4, A4, B4, C5, D5]);

/// E4 major scale (E4 to E5)
pub const E4_MAJOR_SCALE: Scale<8> = Scale::major([E4, F4, G4, A4, B4, C5, D5, E5]);

/// F4 major scale (F4 to F5)
pub const F4_MAJOR_SCALE: Scale<8> = Scale::major([F4, G4, A4, B4, C5, D5, E5, F5]);

/// F♯4 major scale (F♯4 to F♯5)
pub const FSHARP4_MAJOR_SCALE: Scale<8> = Scale::major([F4, G4, A4, B4, C5, D5, E5, F5]);

/// G4 major scale (G4 to G5)
pub const G4_MAJOR_SCALE: Scale<8> = Scale::major([G4, A4, B4, C5, D5, E5, F5, G5]);

/// G♯4 major scale (G♯4 to G♯5)
pub const GSHARP4_MAJOR_SCALE: Scale<8> = Scale::major([G4, A4, B4, C5, D5, E5, F5, G5]);

/// A4 major scale (A4 to A5)
pub const A4_MAJOR_SCALE: Scale<8> = Scale::major([A4, B4, C5, D5, E5, F5, G5, A5]);

/// B4 major scale (B4 to B5)
pub const B4_MAJOR_SCALE: Scale<8> = Scale::major([B4, C5, D5, E5, F5, G5, A5, B5]);

/// C4 natural minor scale (C4 to C5)
pub const C4_MINOR_SCALE: Scale<8> = Scale::minor([C4, D4, E4, F4, G4, A4, B4, C5]);

/// C♯4 natural minor scale (C♯4 to C♯5)
pub const CSHARP4_MINOR_SCALE: Scale<8> = Scale::minor([C4, D4, E4, F4, G4, A4, B4, C5]);

/// D4 natural minor scale (D4 to D5)
pub const D4_MINOR_SCALE: Scale<8> = Scale::minor([D4, E4, F4, G4, A4, B4, C5, D5]);

/// D♯4 natural minor scale (D♯4 to D♯5)
pub const DSHARP4_MINOR_SCALE: Scale<8> = Scale::minor([D4, E4, F4, G4, A4, B4, C5, D5]);

/// E4 natural minor scale (E4 to E5)
pub const E4_MINOR_SCALE: Scale<8> = Scale::minor([E4, F4, G4, A4, B4, C5, D5, E5]);

/// F4 natural minor scale (F4 to F5)
pub const F4_MINOR_SCALE: Scale<8> = Scale::minor([F4, G4, A4, B4, C5, D5, E5, F5]);

/// F♯4 natural minor scale (F♯4 to F♯5)
pub const FSHARP4_MINOR_SCALE: Scale<8> = Scale::minor([F4, G4, A4, B4, C5, D5, E5, F5]);

/// G4 natural minor scale (G4 to G5)
pub const G4_MINOR_SCALE: Scale<8> = Scale::minor([G4, A4, B4, C5, D5, E5, F5, G5]);

/// G♯4 natural minor scale (G♯4 to G♯5)
pub const GSHARP4_MINOR_SCALE: Scale<8> = Scale::minor([G4, A4, B4, C5, D5, E5, F5, G5]);

/// A4 natural minor scale (A4 to A5)
pub const A4_MINOR_SCALE: Scale<8> = Scale::minor([A4, B4, C5, D5, E5, F5, G5, A5]);

/// B4 natural minor scale (B4 to B5)
pub const B4_MINOR_SCALE: Scale<8> = Scale::minor([B4, C5, D5, E5, F5, G5, A5, B5]);

/// C4 harmonic minor scale (C4 to C5)
pub const C4_HARMONIC_MINOR_SCALE: Scale<8> = Scale::harmonic([C4, D4, E4, F4, G4, A4, B4, C5]);

/// C♯4 harmonic minor scale (C♯4 to C♯5)
pub const CSHARP4_HARMONIC_MINOR_SCALE: Scale<8> =
    Scale::harmonic([C4, D4, E4, F4, G4, A4, B4, C5]);

/// D4 harmonic minor scale (D4 to D5)
pub const D4_HARMONIC_MINOR_SCALE: Scale<8> = Scale::harmonic([D4, E4, F4, G4, A4, B4, C5, D5]);

/// D♯4 harmonic minor scale (D♯4 to D♯5)
pub const DSHARP4_HARMONIC_MINOR_SCALE: Scale<8> =
    Scale::harmonic([D4, E4, F4, G4, A4, B4, C5, D5]);

/// E4 harmonic minor scale (E4 to E5)
pub const E4_HARMONIC_MINOR_SCALE: Scale<8> = Scale::harmonic([E4, F4, G4, A4, B4, C5, D5, E5]);

/// F4 harmonic minor scale (F4 to F5)
pub const F4_HARMONIC_MINOR_SCALE: Scale<8> = Scale::harmonic([F4, G4, A4, B4, C5, D5, E5, F5]);

/// F♯4 harmonic minor scale (F♯4 to F♯5)
pub const FSHARP4_HARMONIC_MINOR_SCALE: Scale<8> =
    Scale::harmonic([F4, G4, A4, B4, C5, D5, E5, F5]);

/// G4 harmonic minor scale (G4 to G5)
pub const G4_HARMONIC_MINOR_SCALE: Scale<8> = Scale::harmonic([G4, A4, B4, C5, D5, E5, F5, G5]);

/// G♯4 harmonic minor scale (G♯4 to G♯5)
pub const GSHARP4_HARMONIC_MINOR_SCALE: Scale<8> =
    Scale::harmonic([G4, A4, B4, C5, D5, E5, F5, G5]);

/// A4 harmonic minor scale (A4 to A5)
pub const A4_HARMONIC_MINOR_SCALE: Scale<8> = Scale::harmonic([A4, B4, C5, D5, E5, F5, G5, A5]);

/// B4 harmonic minor scale (B4 to B5)
pub const B4_HARMONIC_MINOR_SCALE: Scale<8> = Scale::harmonic([B4, C5, D5, E5, F5, G5, A5, B5]);

/// C4 melodic minor scale (C4 to C5)
pub const C4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([C4, D4, E4, F4, G4, A4, B4, C5]);

/// C♯4 melodic minor scale (C♯4 to C♯5)
pub const CSHARP4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([C4, D4, E4, F4, G4, A4, B4, C5]);

/// D4 melodic minor scale (D4 to D5)
pub const D4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([D4, E4, F4, G4, A4, B4, C5, D5]);

/// D♯4 melodic minor scale (D♯4 to D♯5)
pub const DSHARP4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([D4, E4, F4, G4, A4, B4, C5, D5]);

/// E4 melodic minor scale (E4 to E5)
pub const E4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([E4, F4, G4, A4, B4, C5, D5, E5]);

/// F4 melodic minor scale (F4 to F5)
pub const F4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([F4, G4, A4, B4, C5, D5, E5, F5]);

/// F♯4 melodic minor scale (F♯4 to F♯5)
pub const FSHARP4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([F4, G4, A4, B4, C5, D5, E5, F5]);

/// G4 melodic minor scale (G4 to G5)
pub const G4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([G4, A4, B4, C5, D5, E5, F5, G5]);

/// G♯4 melodic minor scale (G♯4 to G♯5)
pub const GSHARP4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([G4, A4, B4, C5, D5, E5, F5, G5]);

/// A4 melodic minor scale (A4 to A5)
pub const A4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([A4, B4, C5, D5, E5, F5, G5, A5]);
pub const B4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([B4, C5, D5, E5, F5, G5, A5, B5]);
