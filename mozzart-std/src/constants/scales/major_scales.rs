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
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

/// The interval pattern defining a major scale.
///
/// This array represents the intervals between consecutive notes in a major scale,
/// following the classic whole-whole-half-whole-whole-whole-half step pattern:
/// - UNISON: Starting position (0 semitones)
/// - TONE: From 1st to 2nd degree (C to D, 2 semitones)
/// - TONE: From 2nd to 3rd degree (D to E, 2 semitones)
/// - SEMITONE: From 3rd to 4th degree (E to F, 1 semitone)
/// - TONE: From 4th to 5th degree (F to G, 2 semitones)
/// - TONE: From 5th to 6th degree (G to A, 2 semitones)
/// - TONE: From 6th to 7th degree (A to B, 2 semitones)
/// - SEMITONE: From 7th to 8th degree (B to C, 1 semitone)
///
/// This pattern creates the characteristic sound of the major scale and is
/// the foundation for the diatonic system in Western music theory.
///
/// # Musical Theory
/// The major scale pattern has been central to Western music since the 17th century.
/// Its pattern of whole and half steps creates specific tonal relationships that
/// establish a strong sense of key center and harmonic function.
///
/// # Example Usage
/// ```
/// use mozzart_std::{MAJOR_SCALE_STEPS, C4_MAJOR_SCALE};
///
/// // Verify that a C major scale follows the standard major scale pattern
/// assert_eq!(C4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
/// ```
pub const MAJOR_SCALE_STEPS: [Interval; 8] =
    [UNISON, TONE, TONE, SEMITONE, TONE, TONE, TONE, SEMITONE];

// Octave-independent major scales (pitch class only)
/// C major scale: C-D-E-F-G-A-B-C
/// No sharps or flats (key signature: 0)
/// This is the most basic major scale, often used as a reference for teaching and theory.
pub const C_MAJOR_SCALE: Scale<8> = Scale::major([C, D, E, F, G, A, B, C0]);
/// C♯/D♭ major scale: C♯-D♯-E♯(F)-F♯-G♯-A♯-B♯(C)-C♯
/// As C♯: 7 sharps (C♯,D♯,E♯,F♯,G♯,A♯,B♯)
/// As D♭: 5 flats (D♭,E♭,G♭,A♭,B♭)
/// Enharmonically equivalent to D♭ major, though D♭ is more commonly used.
pub const CSHARP_MAJOR_SCALE: Scale<8> =
    Scale::major([CSHARP, DSHARP, F, FSHARP, GSHARP, ASHARP, C0, CSHARP0]);
/// D major scale: D-E-F♯-G-A-B-C♯-D
/// Key signature: 2 sharps (F♯,C♯)
/// Common key for violin music due to the instrument's tuning.
pub const D_MAJOR_SCALE: Scale<8> = Scale::major([D, E, FSHARP, G, A, B, CSHARP0, D0]);
/// D♯/E♭ major scale: D♯-E♯(F)-F♯♯(G)-G♯-A♯-B♯(C)-C♯♯(D)-D♯
/// As E♭: 3 flats (E♭,A♭,B♭)
/// Almost always written as E♭ major rather than D♯ for practical reasons.
pub const DSHARP_MAJOR_SCALE: Scale<8> =
    Scale::major([DSHARP, F, G, GSHARP, ASHARP, C0, D0, DSHARP0]);
/// E major scale: E-F♯-G♯-A-B-C♯-D♯-E
/// Key signature: 4 sharps (F♯,G♯,C♯,D♯)
/// Often used in guitar music due to the open strings.
pub const E_MAJOR_SCALE: Scale<8> = Scale::major([E, FSHARP, GSHARP, A, B, CSHARP0, DSHARP0, E0]);
/// F major scale: F-G-A-B♭-C-D-E-F
/// Key signature: 1 flat (B♭)
/// One of the earliest keys used in keyboard music.
pub const F_MAJOR_SCALE: Scale<8> = Scale::major([F, G, A, ASHARP, C0, D0, E0, F0]);
/// F♯/G♭ major scale: F♯-G♯-A♯-B-C♯-D♯-E♯(F)-F♯
/// As F♯: 6 sharps (F♯,G♯,A♯,C♯,D♯,E♯)
/// As G♭: 6 flats (G♭,A♭,B♭,C♭,D♭,E♭)
/// Used in more complex compositions; G♭ is typically preferred in flat-heavy contexts.
pub const FSHARP_MAJOR_SCALE: Scale<8> =
    Scale::major([FSHARP, GSHARP, ASHARP, B, CSHARP0, DSHARP0, F0, FSHARP0]);
/// G major scale: G-A-B-C-D-E-F♯-G
/// Key signature: 1 sharp (F♯)
/// Very common in folk and popular music.
pub const G_MAJOR_SCALE: Scale<8> = Scale::major([G, A, B, C0, D0, E0, FSHARP0, G0]);
/// G♯/A♭ major scale: G♯-A♯-B♯(C)-C♯-D♯-E♯(F)-F♯♯(G)-G♯
/// As A♭: 4 flats (A♭,B♭,D♭,E♭)
/// Almost always written as A♭ major rather than G♯ for practical reasons.
pub const GSHARP_MAJOR_SCALE: Scale<8> =
    Scale::major([GSHARP, ASHARP, C0, CSHARP0, DSHARP0, F0, G0, GSHARP0]);
/// A major scale: A-B-C♯-D-E-F♯-G♯-A
/// Key signature: 3 sharps (F♯,C♯,G♯)
/// Often used for vocal music as it sits in a comfortable range.
pub const A_MAJOR_SCALE: Scale<8> = Scale::major([A, B, CSHARP0, D0, E0, FSHARP0, GSHARP0, A0]);
/// A♯/B♭ major scale: A♯-B♯(C)-C♯♯(D)-D♯-E♯(F)-F♯♯(G)-G♯♯(A)-A♯
/// As B♭: 2 flats (B♭,E♭)
/// Common key for brass and woodwind instruments.
pub const ASHARP_MAJOR_SCALE: Scale<8> =
    Scale::major([ASHARP, C0, D0, DSHARP0, F0, G0, A0, ASHARP0]);
/// B major scale: B-C♯-D♯-E-F♯-G♯-A♯-B
/// Key signature: 5 sharps (F♯,C♯,G♯,D♯,A♯)
/// One of the more remote keys in common practice.
pub const B_MAJOR_SCALE: Scale<8> =
    Scale::major([B, CSHARP0, DSHARP0, E0, FSHARP0, GSHARP0, ASHARP0, B0]);

// Octave 0 major scales (C0-B0)
// These are the lowest complete major scales in the MIDI range.

/// C0 major scale: C0-D0-E0-F0-G0-A0-B0-C1
/// Lowest C major scale in standard MIDI (C0 = MIDI note 12)
/// Notes in Hz: C0 (16.35) - D0 (18.35) - E0 (20.60) - F0 (21.83) - G0 (24.50) - A0 (27.50) - B0 (30.87) - C1 (32.70)
pub const C0_MAJOR_SCALE: Scale<8> = Scale::major([C0, D0, E0, F0, G0, A0, B0, C1]);
/// C♯0/D♭0 major scale: C♯0-D♯0-F0-F♯0-G♯0-A♯0-C1-C♯1
/// Notes span from C♯0 (MIDI note 13, ~17.32 Hz) to C♯1 (MIDI note 25, ~34.65 Hz)
pub const CSHARP0_MAJOR_SCALE: Scale<8> =
    Scale::major([CSHARP0, DSHARP0, F0, FSHARP0, GSHARP0, ASHARP0, C1, CSHARP1]);
/// D0 major scale: D0-E0-F♯0-G0-A0-B0-C♯1-D1
/// Notes span from D0 (MIDI note 14, ~18.35 Hz) to D1 (MIDI note 26, ~36.71 Hz)
pub const D0_MAJOR_SCALE: Scale<8> = Scale::major([D0, E0, FSHARP0, G0, A0, B0, CSHARP1, D1]);
/// D♯0/E♭0 major scale: D♯0-F0-G0-G♯0-A♯0-C1-D1-D♯1
/// Notes span from D♯0 (MIDI note 15, ~19.45 Hz) to D♯1 (MIDI note 27, ~38.89 Hz)
pub const DSHARP0_MAJOR_SCALE: Scale<8> =
    Scale::major([DSHARP0, F0, G0, GSHARP0, ASHARP0, C1, D1, DSHARP1]);
/// E0 major scale: E0-F♯0-G♯0-A0-B0-C♯1-D♯1-E1
/// Notes span from E0 (MIDI note 16, ~20.60 Hz) to E1 (MIDI note 28, ~41.20 Hz)
pub const E0_MAJOR_SCALE: Scale<8> =
    Scale::major([E0, FSHARP0, GSHARP0, A0, B0, CSHARP1, DSHARP1, E1]);
/// F0 major scale: F0-G0-A0-B♭0-C1-D1-E1-F1
/// Notes span from F0 (MIDI note 17, ~21.83 Hz) to F1 (MIDI note 29, ~43.65 Hz)
pub const F0_MAJOR_SCALE: Scale<8> = Scale::major([F0, G0, A0, ASHARP0, C1, D1, E1, F1]);
/// F♯0/G♭0 major scale: F♯0-G♯0-A♯0-B0-C♯1-D♯1-F1-F♯1
/// Notes span from F♯0 (MIDI note 18, ~23.12 Hz) to F♯1 (MIDI note 30, ~46.25 Hz)
pub const FSHARP0_MAJOR_SCALE: Scale<8> =
    Scale::major([FSHARP0, GSHARP0, ASHARP0, B0, CSHARP1, DSHARP1, F1, FSHARP1]);
/// G0 major scale: G0-A0-B0-C1-D1-E1-F♯1-G1
/// Notes span from G0 (MIDI note 19, ~24.50 Hz) to G1 (MIDI note 31, ~49.00 Hz)
pub const G0_MAJOR_SCALE: Scale<8> = Scale::major([G0, A0, B0, C1, D1, E1, FSHARP1, G1]);
/// G♯0/A♭0 major scale: G♯0-A♯0-C1-C♯1-D♯1-F1-G1-G♯1
/// Notes span from G♯0 (MIDI note 20, ~25.96 Hz) to G♯1 (MIDI note 32, ~51.91 Hz)
pub const GSHARP0_MAJOR_SCALE: Scale<8> =
    Scale::major([GSHARP0, ASHARP0, C1, CSHARP1, DSHARP1, F1, G1, GSHARP1]);
/// A0 major scale: A0-B0-C♯1-D1-E1-F♯1-G♯1-A1
/// Notes span from A0 (MIDI note 21, ~27.50 Hz) - the lowest note on a standard piano - to A1 (MIDI note 33, ~55.00 Hz)
pub const A0_MAJOR_SCALE: Scale<8> = Scale::major([A0, B0, CSHARP1, D1, E1, FSHARP1, GSHARP1, A1]);
/// A♯0/B♭0 major scale: A♯0-C1-D1-D♯1-F1-G1-A1-A♯1
/// Notes span from A♯0 (MIDI note 22, ~29.14 Hz) to A♯1 (MIDI note 34, ~58.27 Hz)
pub const ASHARP0_MAJOR_SCALE: Scale<8> =
    Scale::major([ASHARP0, C1, D1, DSHARP1, F1, G1, A1, ASHARP1]);
/// B0 major scale: B0-C♯1-D♯1-E1-F♯1-G♯1-A♯1-B1
/// Notes span from B0 (MIDI note 23, ~30.87 Hz) to B1 (MIDI note 35, ~61.74 Hz)
pub const B0_MAJOR_SCALE: Scale<8> =
    Scale::major([B0, CSHARP1, DSHARP1, E1, FSHARP1, GSHARP1, ASHARP1, B1]);

/// C1 major scale: C1-D1-E1-F1-G1-A1-B1-C2
/// Notes span from C1 (MIDI note 24, 32.70 Hz) to C2 (MIDI note 36, 65.41 Hz)
/// These notes are in the range of the lowest octave on a standard piano
pub const C1_MAJOR_SCALE: Scale<8> = Scale::major([C1, D1, E1, F1, G1, A1, B1, C2]);
/// C♯1/D♭1 major scale: C♯1-D♯1-F1-F♯1-G♯1-A♯1-C2-C♯2
/// Notes span from C♯1 (MIDI note 25, ~34.65 Hz) to C♯2 (MIDI note 37, ~69.30 Hz)
pub const CSHARP1_MAJOR_SCALE: Scale<8> =
    Scale::major([CSHARP1, DSHARP1, F1, FSHARP1, GSHARP1, ASHARP1, C2, CSHARP2]);
/// D1 major scale: D1-E1-F♯1-G1-A1-B1-C♯2-D2
/// Notes span from D1 (MIDI note 26, ~36.71 Hz) to D2 (MIDI note 38, ~73.42 Hz)
pub const D1_MAJOR_SCALE: Scale<8> = Scale::major([D1, E1, FSHARP1, G1, A1, B1, CSHARP2, D2]);
/// D♯1/E♭1 major scale: D♯1-F1-G1-G♯1-A♯1-C2-D2-D♯2
/// Notes span from D♯1 (MIDI note 27, ~38.89 Hz) to D♯2 (MIDI note 39, ~77.78 Hz)
pub const DSHARP1_MAJOR_SCALE: Scale<8> =
    Scale::major([DSHARP1, F1, G1, GSHARP1, ASHARP1, C2, D2, DSHARP2]);
/// E1 major scale: E1-F♯1-G♯1-A1-B1-C♯2-D♯2-E2
/// Notes span from E1 (MIDI note 28, ~41.20 Hz) to E2 (MIDI note 40, ~82.41 Hz)
/// This is the same pitch as the lowest string (E) on a standard bass guitar
pub const E1_MAJOR_SCALE: Scale<8> =
    Scale::major([E1, FSHARP1, GSHARP1, A1, B1, CSHARP2, DSHARP2, E2]);
/// F1 major scale: F1-G1-A1-B♭1-C2-D2-E2-F2
/// Notes span from F1 (MIDI note 29, ~43.65 Hz) to F2 (MIDI note 41, ~87.31 Hz)
pub const F1_MAJOR_SCALE: Scale<8> = Scale::major([F1, G1, A1, ASHARP1, C2, D2, E2, F2]);
/// F♯1/G♭1 major scale: F♯1-G♯1-A♯1-B1-C♯2-D♯2-F2-F♯2
/// Notes span from F♯1 (MIDI note 30, ~46.25 Hz) to F♯2 (MIDI note 42, ~92.50 Hz)
pub const FSHARP1_MAJOR_SCALE: Scale<8> =
    Scale::major([FSHARP1, GSHARP1, ASHARP1, B1, CSHARP2, DSHARP2, F2, FSHARP2]);
/// G1 major scale: G1-A1-B1-C2-D2-E2-F♯2-G2
/// Notes span from G1 (MIDI note 31, ~49.00 Hz) to G2 (MIDI note 43, ~98.00 Hz)
pub const G1_MAJOR_SCALE: Scale<8> = Scale::major([G1, A1, B1, C2, D2, E2, FSHARP2, G2]);
/// G♯1/A♭1 major scale: G♯1-A♯1-C2-C♯2-D♯2-F2-G2-G♯2
/// Notes span from G♯1 (MIDI note 32, ~51.91 Hz) to G♯2 (MIDI note 44, ~103.83 Hz)
pub const GSHARP1_MAJOR_SCALE: Scale<8> =
    Scale::major([GSHARP1, ASHARP1, C2, CSHARP2, DSHARP2, F2, G2, GSHARP2]);
/// A1 major scale: A1-B1-C♯2-D2-E2-F♯2-G♯2-A2
/// Notes span from A1 (MIDI note 33, 55.00 Hz) to A2 (MIDI note 45, 110.00 Hz)
/// This scale includes the pitch of the lowest string (A) on a standard guitar
pub const A1_MAJOR_SCALE: Scale<8> = Scale::major([A1, B1, CSHARP2, D2, E2, FSHARP2, GSHARP2, A2]);
/// A♯1/B♭1 major scale: A♯1-C2-D2-D♯2-F2-G2-A2-A♯2
/// Notes span from A♯1 (MIDI note 34, ~58.27 Hz) to A♯2 (MIDI note 46, ~116.54 Hz)
pub const ASHARP1_MAJOR_SCALE: Scale<8> =
    Scale::major([ASHARP1, C2, D2, DSHARP2, F2, G2, A2, ASHARP2]);
/// B1 major scale: B1-C♯2-D♯2-E2-F♯2-G♯2-A♯2-B2
/// Notes span from B1 (MIDI note 35, ~61.74 Hz) to B2 (MIDI note 47, ~123.47 Hz)
pub const B1_MAJOR_SCALE: Scale<8> =
    Scale::major([B1, CSHARP2, DSHARP2, E2, FSHARP2, GSHARP2, ASHARP2, B2]);

/// C2 major scale: C2-D2-E2-F2-G2-A2-B2-C3
/// Notes span from C2 (MIDI note 36, 65.41 Hz) to C3 (MIDI note 48, 130.81 Hz)
/// This is the range of the C two octaves below middle C
pub const C2_MAJOR_SCALE: Scale<8> = Scale::major([C2, D2, E2, F2, G2, A2, B2, C3]);
/// C♯2/D♭2 major scale: C♯2-D♯2-F2-F♯2-G♯2-A♯2-C3-C♯3
/// Notes span from C♯2 (MIDI note 37, ~69.30 Hz) to C♯3 (MIDI note 49, ~138.59 Hz)
pub const CSHARP2_MAJOR_SCALE: Scale<8> =
    Scale::major([CSHARP2, DSHARP2, F2, FSHARP2, GSHARP2, ASHARP2, C3, CSHARP3]);
/// D2 major scale: D2-E2-F♯2-G2-A2-B2-C♯3-D3
/// Notes span from D2 (MIDI note 38, ~73.42 Hz) to D3 (MIDI note 50, ~146.83 Hz)
/// This range includes the D string on a standard guitar
pub const D2_MAJOR_SCALE: Scale<8> = Scale::major([D2, E2, FSHARP2, G2, A2, B2, CSHARP3, D3]);
/// D♯2/E♭2 major scale: D♯2-F2-G2-G♯2-A♯2-C3-D3-D♯3
/// Notes span from D♯2 (MIDI note 39, ~77.78 Hz) to D♯3 (MIDI note 51, ~155.56 Hz)
pub const DSHARP2_MAJOR_SCALE: Scale<8> =
    Scale::major([DSHARP2, F2, G2, GSHARP2, ASHARP2, C3, D3, DSHARP3]);
/// E2 major scale: E2-F♯2-G♯2-A2-B2-C♯3-D♯3-E3
/// Notes span from E2 (MIDI note 40, ~82.41 Hz) to E3 (MIDI note 52, ~164.81 Hz)
/// This scale includes the pitch of the E string (2nd lowest) on a standard guitar
pub const E2_MAJOR_SCALE: Scale<8> =
    Scale::major([E2, FSHARP2, GSHARP2, A2, B2, CSHARP3, DSHARP3, E3]);
/// F2 major scale: F2-G2-A2-B♭2-C3-D3-E3-F3
/// Notes span from F2 (MIDI note 41, ~87.31 Hz) to F3 (MIDI note 53, ~174.61 Hz)
pub const F2_MAJOR_SCALE: Scale<8> = Scale::major([F2, G2, A2, ASHARP2, C3, D3, E3, F3]);
/// F♯2/G♭2 major scale: F♯2-G♯2-A♯2-B2-C♯3-D♯3-F3-F♯3
/// Notes span from F♯2 (MIDI note 42, ~92.50 Hz) to F♯3 (MIDI note 54, ~185.00 Hz)
pub const FSHARP2_MAJOR_SCALE: Scale<8> =
    Scale::major([FSHARP2, GSHARP2, ASHARP2, B2, CSHARP3, DSHARP3, F3, FSHARP3]);
/// G2 major scale: G2-A2-B2-C3-D3-E3-F♯3-G3
/// Notes span from G2 (MIDI note 43, ~98.00 Hz) to G3 (MIDI note 55, ~196.00 Hz)
/// This scale includes the pitch of the G string (3rd string) on a standard guitar
pub const G2_MAJOR_SCALE: Scale<8> = Scale::major([G2, A2, B2, C3, D3, E3, FSHARP3, G3]);
/// G♯2/A♭2 major scale: G♯2-A♯2-C3-C♯3-D♯3-F3-G3-G♯3
/// Notes span from G♯2 (MIDI note 44, ~103.83 Hz) to G♯3 (MIDI note 56, ~207.65 Hz)
pub const GSHARP2_MAJOR_SCALE: Scale<8> =
    Scale::major([GSHARP2, ASHARP2, C3, CSHARP3, DSHARP3, F3, G3, GSHARP3]);
/// A2 major scale: A2-B2-C♯3-D3-E3-F♯3-G♯3-A3
/// Notes span from A2 (MIDI note 45, 110.00 Hz) to A3 (MIDI note 57, 220.00 Hz)
/// This scale includes the pitch of the A string (5th string) on a standard guitar
pub const A2_MAJOR_SCALE: Scale<8> = Scale::major([A2, B2, CSHARP3, D3, E3, FSHARP3, GSHARP3, A3]);
/// A♯2/B♭2 major scale: A♯2-C3-D3-D♯3-F3-G3-A3-A♯3
/// Notes span from A♯2 (MIDI note 46, ~116.54 Hz) to A♯3 (MIDI note 58, ~233.08 Hz)
pub const ASHARP2_MAJOR_SCALE: Scale<8> =
    Scale::major([ASHARP2, C3, D3, DSHARP3, F3, G3, A3, ASHARP3]);
/// B2 major scale: B2-C♯3-D♯3-E3-F♯3-G♯3-A♯3-B3
/// Notes span from B2 (MIDI note 47, ~123.47 Hz) to B3 (MIDI note 59, ~246.94 Hz)
/// This scale includes the pitch of the B string (2nd highest) on a standard guitar
pub const B2_MAJOR_SCALE: Scale<8> =
    Scale::major([B2, CSHARP3, DSHARP3, E3, FSHARP3, GSHARP3, ASHARP3, B3]);

/// C3 major scale: C3-D3-E3-F3-G3-A3-B3-C4
/// Notes span from C3 (MIDI note 48, 130.81 Hz) to C4 (MIDI note 60, 261.63 Hz)
/// C3 is in the bass clef and spans up to middle C (C4)
pub const C3_MAJOR_SCALE: Scale<8> = Scale::major([C3, D3, E3, F3, G3, A3, B3, C4]);
/// C♯3/D♭3 major scale: C♯3-D♯3-F3-F♯3-G♯3-A♯3-C4-C♯4
/// Notes span from C♯3 (MIDI note 49, ~138.59 Hz) to C♯4 (MIDI note 61, ~277.18 Hz)
pub const CSHARP3_MAJOR_SCALE: Scale<8> =
    Scale::major([CSHARP3, DSHARP3, F3, FSHARP3, GSHARP3, ASHARP3, C4, CSHARP4]);
/// D3 major scale: D3-E3-F♯3-G3-A3-B3-C♯4-D4
/// Notes span from D3 (MIDI note 50, ~146.83 Hz) to D4 (MIDI note 62, ~293.66 Hz)
pub const D3_MAJOR_SCALE: Scale<8> = Scale::major([D3, E3, FSHARP3, G3, A3, B3, CSHARP4, D4]);
/// D♯3/E♭3 major scale: D♯3-F3-G3-G♯3-A♯3-C4-D4-D♯4
/// Notes span from D♯3 (MIDI note 51, ~155.56 Hz) to D♯4 (MIDI note 63, ~311.13 Hz)
pub const DSHARP3_MAJOR_SCALE: Scale<8> =
    Scale::major([DSHARP3, F3, G3, GSHARP3, ASHARP3, C4, D4, DSHARP4]);
/// E3 major scale: E3-F♯3-G♯3-A3-B3-C♯4-D♯4-E4
/// Notes span from E3 (MIDI note 52, ~164.81 Hz) to E4 (MIDI note 64, ~329.63 Hz)
/// This scale includes the pitch of the highest string (E) on a standard guitar
pub const E3_MAJOR_SCALE: Scale<8> =
    Scale::major([E3, FSHARP3, GSHARP3, A3, B3, CSHARP4, DSHARP4, E4]);
/// F3 major scale: F3-G3-A3-B♭3-C4-D4-E4-F4
/// Notes span from F3 (MIDI note 53, ~174.61 Hz) to F4 (MIDI note 65, ~349.23 Hz)
/// This scale spans from the bass clef into the treble clef, crossing middle C
pub const F3_MAJOR_SCALE: Scale<8> = Scale::major([F3, G3, A3, ASHARP3, C4, D4, E4, F4]);
/// F♯3/G♭3 major scale: F♯3-G♯3-A♯3-B3-C♯4-D♯4-F4-F♯4
/// Notes span from F♯3 (MIDI note 54, ~185.00 Hz) to F♯4 (MIDI note 66, ~369.99 Hz)
pub const FSHARP3_MAJOR_SCALE: Scale<8> =
    Scale::major([FSHARP3, GSHARP3, ASHARP3, B3, CSHARP4, DSHARP4, F4, FSHARP4]);
/// G3 major scale: G3-A3-B3-C4-D4-E4-F♯4-G4
/// Notes span from G3 (MIDI note 55, ~196.00 Hz) to G4 (MIDI note 67, ~392.00 Hz)
/// This scale spans the break between bass and treble clefs
pub const G3_MAJOR_SCALE: Scale<8> = Scale::major([G3, A3, B3, C4, D4, E4, FSHARP4, G4]);
/// G♯3/A♭3 major scale: G♯3-A♯3-C4-C♯4-D♯4-F4-G4-G♯4
/// Notes span from G♯3 (MIDI note 56, ~207.65 Hz) to G♯4 (MIDI note 68, ~415.30 Hz)
pub const GSHARP3_MAJOR_SCALE: Scale<8> =
    Scale::major([GSHARP3, ASHARP3, C4, CSHARP4, DSHARP4, F4, G4, GSHARP4]);
/// A3 major scale: A3-B3-C♯4-D4-E4-F♯4-G♯4-A4
/// Notes span from A3 (MIDI note 57, 220.00 Hz) to A4 (MIDI note 69, 440.00 Hz)
/// This scale spans to concert pitch A4 (440 Hz standard)
pub const A3_MAJOR_SCALE: Scale<8> = Scale::major([A3, B3, CSHARP4, D4, E4, FSHARP4, GSHARP4, A4]);
/// A♯3/B♭3 major scale: A♯3-C4-D4-D♯4-F4-G4-A4-A♯4
/// Notes span from A♯3 (MIDI note 58, ~233.08 Hz) to A♯4 (MIDI note 70, ~466.16 Hz)
pub const ASHARP3_MAJOR_SCALE: Scale<8> =
    Scale::major([ASHARP3, C4, D4, DSHARP4, F4, G4, A4, ASHARP4]);
/// B3 major scale: B3-C♯4-D♯4-E4-F♯4-G♯4-A♯4-B4
/// Notes span from B3 (MIDI note 59, ~246.94 Hz) to B4 (MIDI note 71, ~493.88 Hz)
/// This scale starts just below middle C and spans into the treble clef
pub const B3_MAJOR_SCALE: Scale<8> =
    Scale::major([B3, CSHARP4, DSHARP4, E4, FSHARP4, GSHARP4, ASHARP4, B4]);

/// C4 major scale: C4-D4-E4-F4-G4-A4-B4-C5
/// Middle C major scale - the central reference scale
/// C4 is "middle C" (MIDI note 60, 261.63 Hz), a critical reference point in music
pub const C4_MAJOR_SCALE: Scale<8> = Scale::major([C4, D4, E4, F4, G4, A4, B4, C5]);
/// C♯4/D♭4 major scale: C♯4-D♯4-F4-F♯4-G♯4-A♯4-C5-C♯5
/// Notes span from C♯4 (MIDI note 61, ~277.18 Hz) to C♯5 (MIDI note 73, ~554.37 Hz)
pub const CSHARP4_MAJOR_SCALE: Scale<8> =
    Scale::major([CSHARP4, DSHARP4, F4, FSHARP4, GSHARP4, ASHARP4, C5, CSHARP5]);
/// D4 major scale: D4-E4-F♯4-G4-A4-B4-C♯5-D5
/// Notes span from D4 (MIDI note 62, ~293.66 Hz) to D5 (MIDI note 74, ~587.33 Hz)
pub const D4_MAJOR_SCALE: Scale<8> = Scale::major([D4, E4, FSHARP4, G4, A4, B4, CSHARP5, D5]);
/// D♯4/E♭4 major scale: D♯4-F4-G4-G♯4-A♯4-C5-D5-D♯5
/// Notes span from D♯4 (MIDI note 63, ~311.13 Hz) to D♯5 (MIDI note 75, ~622.25 Hz)
pub const DSHARP4_MAJOR_SCALE: Scale<8> =
    Scale::major([DSHARP4, F4, G4, GSHARP4, ASHARP4, C5, D5, DSHARP5]);
/// E4 major scale: E4-F♯4-G♯4-A4-B4-C♯5-D♯5-E5
/// Notes span from E4 (MIDI note 64, ~329.63 Hz) to E5 (MIDI note 76, ~659.26 Hz)
pub const E4_MAJOR_SCALE: Scale<8> =
    Scale::major([E4, FSHARP4, GSHARP4, A4, B4, CSHARP5, DSHARP5, E5]);
/// F4 major scale: F4-G4-A4-B♭4-C5-D5-E5-F5
/// Notes span from F4 (MIDI note 65, ~349.23 Hz) to F5 (MIDI note 77, ~698.46 Hz)
pub const F4_MAJOR_SCALE: Scale<8> = Scale::major([F4, G4, A4, ASHARP4, C5, D5, E5, F5]);
/// F♯4/G♭4 major scale: F♯4-G♯4-A♯4-B4-C♯5-D♯5-F5-F♯5
/// Notes span from F♯4 (MIDI note 66, ~369.99 Hz) to F♯5 (MIDI note 78, ~739.99 Hz)
pub const FSHARP4_MAJOR_SCALE: Scale<8> =
    Scale::major([FSHARP4, GSHARP4, ASHARP4, B4, CSHARP5, DSHARP5, F5, FSHARP5]);
/// G4 major scale: G4-A4-B4-C5-D5-E5-F♯5-G5
/// Notes span from G4 (MIDI note 67, ~392.00 Hz) to G5 (MIDI note 79, ~783.99 Hz)
pub const G4_MAJOR_SCALE: Scale<8> = Scale::major([G4, A4, B4, C5, D5, E5, FSHARP5, G5]);
/// G♯4/A♭4 major scale: G♯4-A♯4-C5-C♯5-D♯5-F5-G5-G♯5
/// Notes span from G♯4 (MIDI note 68, ~415.30 Hz) to G♯5 (MIDI note 80, ~830.61 Hz)
pub const GSHARP4_MAJOR_SCALE: Scale<8> =
    Scale::major([GSHARP4, ASHARP4, C5, CSHARP5, DSHARP5, F5, G5, GSHARP5]);
/// A4 major scale: A4-B4-C♯5-D5-E5-F♯5-G♯5-A5
/// Notes span from A4 (MIDI note 69, 440.00 Hz - international tuning standard) to A5 (MIDI note 81, 880.00 Hz)
pub const A4_MAJOR_SCALE: Scale<8> = Scale::major([A4, B4, CSHARP5, D5, E5, FSHARP5, GSHARP5, A5]);
/// A♯4/B♭4 major scale: A♯4-C5-D5-D♯5-F5-G5-A5-A♯5
/// Notes span from A♯4 (MIDI note 70, ~466.16 Hz) to A♯5 (MIDI note 82, ~932.33 Hz)
pub const ASHARP4_MAJOR_SCALE: Scale<8> =
    Scale::major([ASHARP4, C5, D5, DSHARP5, F5, G5, A5, ASHARP5]);
/// B4 major scale: B4-C♯5-D♯5-E5-F♯5-G♯5-A♯5-B5
/// Notes span from B4 (MIDI note 71, ~493.88 Hz) to B5 (MIDI note 83, ~987.77 Hz)
pub const B4_MAJOR_SCALE: Scale<8> =
    Scale::major([B4, CSHARP5, DSHARP5, E5, FSHARP5, GSHARP5, ASHARP5, B5]);

/// C5 major scale: C5-D5-E5-F5-G5-A5-B5-C6
/// Notes span from C5 (MIDI note 72, 523.25 Hz) to C6 (MIDI note 84, 1046.50 Hz)
/// This range is in the upper treble clef, commonly used for soprano voices
pub const C5_MAJOR_SCALE: Scale<8> = Scale::major([C5, D5, E5, F5, G5, A5, B5, C6]);
/// C♯5/D♭5 major scale: C♯5-D♯5-F5-F♯5-G♯5-A♯5-C6-C♯6
/// Notes span from C♯5 (MIDI note 73, ~554.37 Hz) to C♯6 (MIDI note 85, ~1108.73 Hz)
pub const CSHARP5_MAJOR_SCALE: Scale<8> =
    Scale::major([CSHARP5, DSHARP5, F5, FSHARP5, GSHARP5, ASHARP5, C6, CSHARP6]);
/// D5 major scale: D5-E5-F♯5-G5-A5-B5-C♯6-D6
/// Notes span from D5 (MIDI note 74, ~587.33 Hz) to D6 (MIDI note 86, ~1174.66 Hz)
pub const D5_MAJOR_SCALE: Scale<8> = Scale::major([D5, E5, FSHARP5, G5, A5, B5, CSHARP6, D6]);
/// D♯5/E♭5 major scale: D♯5-F5-G5-G♯5-A♯5-C6-D6-D♯6
/// Notes span from D♯5 (MIDI note 75, ~622.25 Hz) to D♯6 (MIDI note 87, ~1244.51 Hz)
pub const DSHARP5_MAJOR_SCALE: Scale<8> =
    Scale::major([DSHARP5, F5, G5, GSHARP5, ASHARP5, C6, D6, DSHARP6]);
/// E5 major scale: E5-F♯5-G♯5-A5-B5-C♯6-D♯6-E6
/// Notes span from E5 (MIDI note 76, ~659.26 Hz) to E6 (MIDI note 88, ~1318.51 Hz)
pub const E5_MAJOR_SCALE: Scale<8> =
    Scale::major([E5, FSHARP5, GSHARP5, A5, B5, CSHARP6, DSHARP6, E6]);
/// F5 major scale: F5-G5-A5-B♭5-C6-D6-E6-F6
/// Notes span from F5 (MIDI note 77, ~698.46 Hz) to F6 (MIDI note 89, ~1396.91 Hz)
/// This range is commonly used in flute music
pub const F5_MAJOR_SCALE: Scale<8> = Scale::major([F5, G5, A5, ASHARP5, C6, D6, E6, F6]);
/// F♯5/G♭5 major scale: F♯5-G♯5-A♯5-B5-C♯6-D♯6-F6-F♯6
/// Notes span from F♯5 (MIDI note 78, ~739.99 Hz) to F♯6 (MIDI note 90, ~1479.98 Hz)
pub const FSHARP5_MAJOR_SCALE: Scale<8> =
    Scale::major([FSHARP5, GSHARP5, ASHARP5, B5, CSHARP6, DSHARP6, F6, FSHARP6]);
/// G5 major scale: G5-A5-B5-C6-D6-E6-F♯6-G6
/// Notes span from G5 (MIDI note 79, ~783.99 Hz) to G6 (MIDI note 91, ~1567.98 Hz)
pub const G5_MAJOR_SCALE: Scale<8> = Scale::major([G5, A5, B5, C6, D6, E6, FSHARP6, G6]);
/// G♯5/A♭5 major scale: G♯5-A♯5-C6-C♯6-D♯6-F6-G6-G♯6
/// Notes span from G♯5 (MIDI note 80, ~830.61 Hz) to G♯6 (MIDI note 92, ~1661.22 Hz)
pub const GSHARP5_MAJOR_SCALE: Scale<8> =
    Scale::major([GSHARP5, ASHARP5, C6, CSHARP6, DSHARP6, F6, G6, GSHARP6]);
/// A5 major scale: A5-B5-C♯6-D6-E6-F♯6-G♯6-A6
/// Notes span from A5 (MIDI note 81, 880.00 Hz) to A6 (MIDI note 93, 1760.00 Hz)
/// This range is at the upper end of most soprano vocal ranges
pub const A5_MAJOR_SCALE: Scale<8> = Scale::major([A5, B5, CSHARP6, D6, E6, FSHARP6, GSHARP6, A6]);
/// A♯5/B♭5 major scale: A♯5-C6-D6-D♯6-F6-G6-A6-A♯6
/// Notes span from A♯5 (MIDI note 82, ~932.33 Hz) to A♯6 (MIDI note 94, ~1864.66 Hz)
pub const ASHARP5_MAJOR_SCALE: Scale<8> =
    Scale::major([ASHARP5, C6, D6, DSHARP6, F6, G6, A6, ASHARP6]);
/// B5 major scale: B5-C♯6-D♯6-E6-F♯6-G♯6-A♯6-B6
/// Notes span from B5 (MIDI note 83, ~987.77 Hz) to B6 (MIDI note 95, ~1975.53 Hz)
pub const B5_MAJOR_SCALE: Scale<8> =
    Scale::major([B5, CSHARP6, DSHARP6, E6, FSHARP6, GSHARP6, ASHARP6, B6]);
/// C6 major scale: C6-D6-E6-F6-G6-A6-B6-C7
/// Notes span from C6 (MIDI note 84, 1046.50 Hz) to C7 (MIDI note 96, 2093.00 Hz)
/// This is the highest C on a standard 88-key piano
pub const C6_MAJOR_SCALE: Scale<8> = Scale::major([C6, D6, E6, F6, G6, A6, B6, C7]);
/// C♯6/D♭6 major scale: C♯6-D♯6-F6-F♯6-G♯6-A♯6-C7-C♯7
/// Notes span from C♯6 (MIDI note 85, ~1108.73 Hz) to C♯7 (MIDI note 97, ~2217.46 Hz)
pub const CSHARP6_MAJOR_SCALE: Scale<8> =
    Scale::major([CSHARP6, DSHARP6, F6, FSHARP6, GSHARP6, ASHARP6, C7, CSHARP7]);
/// D6 major scale: D6-E6-F♯6-G6-A6-B6-C♯7-D7
/// Notes span from D6 (MIDI note 86, ~1174.66 Hz) to D7 (MIDI note 98, ~2349.32 Hz)
pub const D6_MAJOR_SCALE: Scale<8> = Scale::major([D6, E6, FSHARP6, G6, A6, B6, CSHARP7, D7]);
/// D♯6/E♭6 major scale: D♯6-F6-G6-G♯6-A♯6-C7-D7-D♯7
/// Notes span from D♯6 (MIDI note 87, ~1244.51 Hz) to D♯7 (MIDI note 99, ~2489.02 Hz)
pub const DSHARP6_MAJOR_SCALE: Scale<8> =
    Scale::major([DSHARP6, F6, G6, GSHARP6, ASHARP6, C7, D7, DSHARP7]);
/// E6 major scale: E6-F♯6-G♯6-A6-B6-C♯7-D♯7-E7
/// Notes span from E6 (MIDI note 88, ~1318.51 Hz) to E7 (MIDI note 100, ~2637.02 Hz)
pub const E6_MAJOR_SCALE: Scale<8> =
    Scale::major([E6, FSHARP6, GSHARP6, A6, B6, CSHARP7, DSHARP7, E7]);
/// F6 major scale: F6-G6-A6-B♭6-C7-D7-E7-F7
/// Notes span from F6 (MIDI note 89, ~1396.91 Hz) to F7 (MIDI note 101, ~2793.83 Hz)
pub const F6_MAJOR_SCALE: Scale<8> = Scale::major([F6, G6, A6, ASHARP6, C7, D7, E7, F7]);
/// F♯6/G♭6 major scale: F♯6-G♯6-A♯6-B6-C♯7-D♯7-F7-F♯7
/// Notes span from F♯6 (MIDI note 90, ~1479.98 Hz) to F♯7 (MIDI note 102, ~2959.96 Hz)
pub const FSHARP6_MAJOR_SCALE: Scale<8> =
    Scale::major([FSHARP6, GSHARP6, ASHARP6, B6, CSHARP7, DSHARP7, F7, FSHARP7]);
/// G6 major scale: G6-A6-B6-C7-D7-E7-F♯7-G7
/// Notes span from G6 (MIDI note 91, ~1567.98 Hz) to G7 (MIDI note 103, ~3135.96 Hz)
/// At the upper range of piccolo and some synthesizer sounds
pub const G6_MAJOR_SCALE: Scale<8> = Scale::major([G6, A6, B6, C7, D7, E7, FSHARP7, G7]);
/// G♯6/A♭6 major scale: G♯6-A♯6-C7-C♯7-D♯7-F7-G7-G♯7
/// Notes span from G♯6 (MIDI note 92, ~1661.22 Hz) to G♯7 (MIDI note 104, ~3322.44 Hz)
pub const GSHARP6_MAJOR_SCALE: Scale<8> =
    Scale::major([GSHARP6, ASHARP6, C7, CSHARP7, DSHARP7, F7, G7, GSHARP7]);
/// A6 major scale: A6-B6-C♯7-D7-E7-F♯7-G♯7-A7
/// Notes span from A6 (MIDI note 93, 1760.00 Hz) to A7 (MIDI note 105, 3520.00 Hz)
pub const A6_MAJOR_SCALE: Scale<8> = Scale::major([A6, B6, CSHARP7, D7, E7, FSHARP7, GSHARP7, A7]);
/// A♯6/B♭6 major scale: A♯6-C7-D7-D♯7-F7-G7-A7-A♯7
/// Notes span from A♯6 (MIDI note 94, ~1864.66 Hz) to A♯7 (MIDI note 106, ~3729.31 Hz)
pub const ASHARP6_MAJOR_SCALE: Scale<8> =
    Scale::major([ASHARP6, C7, D7, DSHARP7, F7, G7, A7, ASHARP7]);
/// B6 major scale: B6-C♯7-D♯7-E7-F♯7-G♯7-A♯7-B7
/// Notes span from B6 (MIDI note 95, ~1975.53 Hz) to B7 (MIDI note 107, ~3951.07 Hz)
pub const B6_MAJOR_SCALE: Scale<8> =
    Scale::major([B6, CSHARP7, DSHARP7, E7, FSHARP7, GSHARP7, ASHARP7, B7]);

/// C7 major scale: C7-D7-E7-F7-G7-A7-B7-C8
/// Notes span from C7 (MIDI note 96, 2093.00 Hz) to C8 (MIDI note 108, 4186.01 Hz)
/// This scale reaches the highest C on a standard piano
pub const C7_MAJOR_SCALE: Scale<8> = Scale::major([C7, D7, E7, F7, G7, A7, B7, C8]);
/// C♯7/D♭7 major scale: C♯7-D♯7-F7-F♯7-G♯7-A♯7-C8-C♯8
/// Notes span from C♯7 (MIDI note 97, ~2217.46 Hz) to C♯8 (MIDI note 109, ~4434.92 Hz)
pub const CSHARP7_MAJOR_SCALE: Scale<8> =
    Scale::major([CSHARP7, DSHARP7, F7, FSHARP7, GSHARP7, ASHARP7, C8, CSHARP8]);
/// D7 major scale: D7-E7-F♯7-G7-A7-B7-C♯8-D8
/// Notes span from D7 (MIDI note 98, ~2349.32 Hz) to D8 (MIDI note 110, ~4698.64 Hz)
pub const D7_MAJOR_SCALE: Scale<8> = Scale::major([D7, E7, FSHARP7, G7, A7, B7, CSHARP8, D8]);
/// D♯7/E♭7 major scale: D♯7-F7-G7-G♯7-A♯7-C8-D8-D♯8
/// Notes span from D♯7 (MIDI note 99, ~2489.02 Hz) to D♯8 (MIDI note 111, ~4978.03 Hz)
pub const DSHARP7_MAJOR_SCALE: Scale<8> =
    Scale::major([DSHARP7, F7, G7, GSHARP7, ASHARP7, C8, D8, DSHARP8]);
/// E7 major scale: E7-F♯7-G♯7-A7-B7-C♯8-D♯8-E8
/// Notes span from E7 (MIDI note 100, ~2637.02 Hz) to E8 (MIDI note 112, ~5274.04 Hz)
pub const E7_MAJOR_SCALE: Scale<8> =
    Scale::major([E7, FSHARP7, GSHARP7, A7, B7, CSHARP8, DSHARP8, E8]);
/// F7 major scale: F7-G7-A7-B♭7-C8-D8-E8-F8
/// Notes span from F7 (MIDI note 101, ~2793.83 Hz) to F8 (MIDI note 113, ~5587.65 Hz)
pub const F7_MAJOR_SCALE: Scale<8> = Scale::major([F7, G7, A7, ASHARP7, C8, D8, E8, F8]);
/// F♯7/G♭7 major scale: F♯7-G♯7-A♯7-B7-C♯8-D♯8-F8-F♯8
/// Notes span from F♯7 (MIDI note 102, ~2959.96 Hz) to F♯8 (MIDI note 114, ~5919.91 Hz)
pub const FSHARP7_MAJOR_SCALE: Scale<8> =
    Scale::major([FSHARP7, GSHARP7, ASHARP7, B7, CSHARP8, DSHARP8, F8, FSHARP8]);
/// G7 major scale: G7-A7-B7-C8-D8-E8-F♯8-G8
/// Notes span from G7 (MIDI note 103, ~3135.96 Hz) to G8 (MIDI note 115, ~6271.93 Hz)
/// At the upper end of human hearing range
pub const G7_MAJOR_SCALE: Scale<8> = Scale::major([G7, A7, B7, C8, D8, E8, FSHARP8, G8]);
/// G♯7/A♭7 major scale: G♯7-A♯7-C8-C♯8-D♯8-F8-G8-G♯8
/// Notes span from G♯7 (MIDI note 104, ~3322.44 Hz) to G♯8 (MIDI note 116, ~6644.88 Hz)
pub const GSHARP7_MAJOR_SCALE: Scale<8> =
    Scale::major([GSHARP7, ASHARP7, C8, CSHARP8, DSHARP8, F8, G8, GSHARP8]);
/// A7 major scale: A7-B7-C♯8-D8-E8-F♯8-G♯8-A8
/// Notes span from A7 (MIDI note 105, 3520.00 Hz) to A8 (MIDI note 117, 7040.00 Hz)
/// Approaching the upper limits of human hearing
pub const A7_MAJOR_SCALE: Scale<8> = Scale::major([A7, B7, CSHARP8, D8, E8, FSHARP8, GSHARP8, A8]);
/// A♯7/B♭7 major scale: A♯7-C8-D8-D♯8-F8-G8-A8-A♯8
/// Notes span from A♯7 (MIDI note 106, ~3729.31 Hz) to A♯8 (MIDI note 118, ~7458.62 Hz)
pub const ASHARP7_MAJOR_SCALE: Scale<8> =
    Scale::major([ASHARP7, C8, D8, DSHARP8, F8, G8, A8, ASHARP8]);
/// B7 major scale: B7-C♯8-D♯8-E8-F♯8-G♯8-A♯8-B8
/// Notes span from B7 (MIDI note 107, ~3951.07 Hz) to B8 (MIDI note 119, ~7902.13 Hz)
/// At the upper extremes of human hearing
pub const B7_MAJOR_SCALE: Scale<8> =
    Scale::major([B7, CSHARP8, DSHARP8, E8, FSHARP8, GSHARP8, ASHARP8, B8]);

/// C8 major scale: C8-D8-E8-F8-G8-A8-B8-C9
/// Notes span from C8 (MIDI note 108, 4186.01 Hz) to C9 (MIDI note 120, 8372.02 Hz)
/// The highest complete C major scale in standard MIDI
pub const C8_MAJOR_SCALE: Scale<8> = Scale::major([C8, D8, E8, F8, G8, A8, B8, C9]);
/// C♯8/D♭8 major scale: C♯8-D♯8-F8-F♯8-G♯8-A♯8-C9-C♯9
/// Notes span from C♯8 (MIDI note 109, ~4434.92 Hz) to C♯9 (MIDI note 121, ~8869.84 Hz)
/// Notes in this range are beyond the upper threshold of many people's hearing
pub const CSHARP8_MAJOR_SCALE: Scale<8> =
    Scale::major([CSHARP8, DSHARP8, F8, FSHARP8, GSHARP8, ASHARP8, C9, CSHARP9]);
/// D8 major scale: D8-E8-F♯8-G8-A8-B8-C♯9-D9
/// Notes span from D8 (MIDI note 110, ~4698.64 Hz) to D9 (MIDI note 122, ~9397.27 Hz)
/// At the upper limits of human hearing perception
pub const D8_MAJOR_SCALE: Scale<8> = Scale::major([D8, E8, FSHARP8, G8, A8, B8, CSHARP9, D9]);
/// D♯8/E♭8 major scale: D♯8-F8-G8-G♯8-A♯8-C9-D9-D♯9
/// Notes span from D♯8 (MIDI note 111, ~4978.03 Hz) to D♯9 (MIDI note 123, ~9956.06 Hz)
pub const DSHARP8_MAJOR_SCALE: Scale<8> =
    Scale::major([DSHARP8, F8, G8, GSHARP8, ASHARP8, C9, D9, DSHARP9]);
/// E8 major scale: E8-F♯8-G♯8-A8-B8-C♯9-D♯9-E9
/// Notes span from E8 (MIDI note 112, ~5274.04 Hz) to E9 (MIDI note 124, ~10548.08 Hz)
pub const E8_MAJOR_SCALE: Scale<8> =
    Scale::major([E8, FSHARP8, GSHARP8, A8, B8, CSHARP9, DSHARP9, E9]);
/// F8 major scale: F8-G8-A8-B♭8-C9-D9-E9-F9
/// Notes span from F8 (MIDI note 113, ~5587.65 Hz) to F9 (MIDI note 125, ~11175.30 Hz)
pub const F8_MAJOR_SCALE: Scale<8> = Scale::major([F8, G8, A8, ASHARP8, C9, D9, E9, F9]);
/// F♯8/G♭8 major scale: F♯8-G♯8-A♯8-B8-C♯9-D♯9-F9-F♯9
/// Notes span from F♯8 (MIDI note 114, ~5919.91 Hz) to F♯9 (MIDI note 126, ~11839.82 Hz)
/// Approaching the upper limit of the MIDI range
pub const FSHARP8_MAJOR_SCALE: Scale<8> =
    Scale::major([FSHARP8, GSHARP8, ASHARP8, B8, CSHARP9, DSHARP9, F9, FSHARP9]);
/// G8 major scale: G8-A8-B8-C9-D9-E9-F♯9-G9
/// Notes span from G8 (MIDI note 115, ~6271.93 Hz) to G9 (MIDI note 127, ~12543.85 Hz)
/// The highest complete major scale in standard MIDI
pub const G8_MAJOR_SCALE: Scale<8> = Scale::major([G8, A8, B8, C9, D9, E9, FSHARP9, G9]);

lazy_static! {
    /// HashMap containing all major scales indexed by their root pitch.
    /// This collection provides quick access to any major scale given its root note.
    ///
    /// The map includes scales for all valid MIDI note numbers, organized by octave.
    /// Each scale spans exactly one octave from its root note.
    ///
    /// # Organization
    /// - **Octave-Independent Scales**: Theoretical scales without specific octave references
    /// - **Octave-Specific Scales**: Scales starting from specific octaves (0-8)
    ///
    /// # Usage
    /// This HashMap allows you to quickly look up the major scale starting from any pitch:
    /// ```
    /// use mozzart_std::{MAJOR_SCALES, C4};
    ///
    /// // Get the C major scale starting from middle C
    /// if let Some(scale) = MAJOR_SCALES.get(&C4) {
    ///     // Use the scale for composition, analysis, etc.
    ///     let root = scale.root(); // C4
    ///     let pitches = scale.pitches(); // [C4, D4, E4, F4, G4, A4, B4, C5]
    /// }
    /// ```
    ///
    /// # Musical Context
    /// Major scales are fundamental building blocks in Western music theory,
    /// used for creating melodies, harmonies, and understanding key centers.
    /// Having access to all possible major scales enables musical operations
    /// in any key across the entire MIDI range.
    pub static ref MAJOR_SCALES: std::collections::HashMap<Pitch, Scale<8>> = HashMap::from([
        (C, C_MAJOR_SCALE),
        (CSHARP, CSHARP_MAJOR_SCALE),
        (D, D_MAJOR_SCALE),
        (DSHARP, DSHARP_MAJOR_SCALE),
        (E, E_MAJOR_SCALE),
        (F, F_MAJOR_SCALE),
        (FSHARP, FSHARP_MAJOR_SCALE),
        (G, G_MAJOR_SCALE),
        (GSHARP, GSHARP_MAJOR_SCALE),
        (A, A_MAJOR_SCALE),
        (ASHARP, ASHARP_MAJOR_SCALE),
        (B, B_MAJOR_SCALE),
        (C0, C0_MAJOR_SCALE),
        (CSHARP0, CSHARP0_MAJOR_SCALE),
        (D0, D0_MAJOR_SCALE),
        (DSHARP0, DSHARP0_MAJOR_SCALE),
        (E0, E0_MAJOR_SCALE),
        (F0, F0_MAJOR_SCALE),
        (FSHARP0, FSHARP0_MAJOR_SCALE),
        (G0, G0_MAJOR_SCALE),
        (GSHARP0, GSHARP0_MAJOR_SCALE),
        (A0, A0_MAJOR_SCALE),
        (ASHARP0, ASHARP0_MAJOR_SCALE),
        (B0, B0_MAJOR_SCALE),
        (C1, C1_MAJOR_SCALE),
        (CSHARP1, CSHARP1_MAJOR_SCALE),
        (D1, D1_MAJOR_SCALE),
        (DSHARP1, DSHARP1_MAJOR_SCALE),
        (E1, E1_MAJOR_SCALE),
        (F1, F1_MAJOR_SCALE),
        (FSHARP1, FSHARP1_MAJOR_SCALE),
        (G1, G1_MAJOR_SCALE),
        (GSHARP1, GSHARP1_MAJOR_SCALE),
        (A1, A1_MAJOR_SCALE),
        (ASHARP1, ASHARP1_MAJOR_SCALE),
        (B1, B1_MAJOR_SCALE),
        (C2, C2_MAJOR_SCALE),
        (CSHARP2, CSHARP2_MAJOR_SCALE),
        (D2, D2_MAJOR_SCALE),
        (DSHARP2, DSHARP2_MAJOR_SCALE),
        (E2, E2_MAJOR_SCALE),
        (F2, F2_MAJOR_SCALE),
        (FSHARP2, FSHARP2_MAJOR_SCALE),
        (G2, G2_MAJOR_SCALE),
        (GSHARP2, GSHARP2_MAJOR_SCALE),
        (A2, A2_MAJOR_SCALE),
        (ASHARP2, ASHARP2_MAJOR_SCALE),
        (B2, B2_MAJOR_SCALE),
        (C3, C3_MAJOR_SCALE),
        (CSHARP3, CSHARP3_MAJOR_SCALE),
        (D3, D3_MAJOR_SCALE),
        (DSHARP3, DSHARP3_MAJOR_SCALE),
        (E3, E3_MAJOR_SCALE),
        (F3, F3_MAJOR_SCALE),
        (FSHARP3, FSHARP3_MAJOR_SCALE),
        (G3, G3_MAJOR_SCALE),
        (GSHARP3, GSHARP3_MAJOR_SCALE),
        (A3, A3_MAJOR_SCALE),
        (ASHARP3, ASHARP3_MAJOR_SCALE),
        (B3, B3_MAJOR_SCALE),
        (C4, C4_MAJOR_SCALE),
        (CSHARP4, CSHARP4_MAJOR_SCALE),
        (D4, D4_MAJOR_SCALE),
        (DSHARP4, DSHARP4_MAJOR_SCALE),
        (E4, E4_MAJOR_SCALE),
        (F4, F4_MAJOR_SCALE),
        (FSHARP4, FSHARP4_MAJOR_SCALE),
        (G4, G4_MAJOR_SCALE),
        (GSHARP4, GSHARP4_MAJOR_SCALE),
        (A4, A4_MAJOR_SCALE),
        (ASHARP4, ASHARP4_MAJOR_SCALE),
        (B4, B4_MAJOR_SCALE),
        (C5, C5_MAJOR_SCALE),
        (CSHARP5, CSHARP5_MAJOR_SCALE),
        (D5, D5_MAJOR_SCALE),
        (DSHARP5, DSHARP5_MAJOR_SCALE),
        (E5, E5_MAJOR_SCALE),
        (F5, F5_MAJOR_SCALE),
        (FSHARP5, FSHARP5_MAJOR_SCALE),
        (G5, G5_MAJOR_SCALE),
        (GSHARP5, GSHARP5_MAJOR_SCALE),
        (A5, A5_MAJOR_SCALE),
        (ASHARP5, ASHARP5_MAJOR_SCALE),
        (B5, B5_MAJOR_SCALE),
        (C6, C6_MAJOR_SCALE),
        (CSHARP6, CSHARP6_MAJOR_SCALE),
        (D6, D6_MAJOR_SCALE),
        (DSHARP6, DSHARP6_MAJOR_SCALE),
        (E6, E6_MAJOR_SCALE),
        (F6, F6_MAJOR_SCALE),
        (FSHARP6, FSHARP6_MAJOR_SCALE),
        (G6, G6_MAJOR_SCALE),
        (GSHARP6, GSHARP6_MAJOR_SCALE),
        (A6, A6_MAJOR_SCALE),
        (ASHARP6, ASHARP6_MAJOR_SCALE),
        (B6, B6_MAJOR_SCALE),
        (C7, C7_MAJOR_SCALE),
        (CSHARP7, CSHARP7_MAJOR_SCALE),
        (D7, D7_MAJOR_SCALE),
        (DSHARP7, DSHARP7_MAJOR_SCALE),
        (E7, E7_MAJOR_SCALE),
        (F7, F7_MAJOR_SCALE),
        (FSHARP7, FSHARP7_MAJOR_SCALE),
        (G7, G7_MAJOR_SCALE),
        (GSHARP7, GSHARP7_MAJOR_SCALE),
        (A7, A7_MAJOR_SCALE),
        (ASHARP7, ASHARP7_MAJOR_SCALE),
        (B7, B7_MAJOR_SCALE),
        (C8, C8_MAJOR_SCALE),
        (CSHARP8, CSHARP8_MAJOR_SCALE),
        (D8, D8_MAJOR_SCALE),
        (DSHARP8, DSHARP8_MAJOR_SCALE),
        (E8, E8_MAJOR_SCALE),
        (F8, F8_MAJOR_SCALE),
        (FSHARP8, FSHARP8_MAJOR_SCALE),
        (G8, G8_MAJOR_SCALE),
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_scale_steps() {
        assert_eq!(
            MAJOR_SCALE_STEPS,
            [UNISON, TONE, TONE, SEMITONE, TONE, TONE, TONE, SEMITONE]
        );
    }

    #[test]
    fn test_major_scales_canonical() {
        let steps = C_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        let steps = CSHARP_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        let steps = D_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        let steps = DSHARP_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        let steps = E_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        let steps = F_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        let steps = FSHARP_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        let steps = G_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        let steps = GSHARP_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        let steps = A_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        let steps = ASHARP_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);

        let steps = B_MAJOR_SCALE.steps();
        assert_eq!(steps, MAJOR_SCALE_STEPS);
    }

    #[test]
    fn test_major_scales_octave_0() {
        assert_eq!(C0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(CSHARP0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(DSHARP0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(FSHARP0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(GSHARP0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(ASHARP0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(B0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
    }

    #[test]
    fn test_major_scales_octave_1() {
        assert_eq!(C1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(CSHARP1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(DSHARP1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(FSHARP1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(GSHARP1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(ASHARP1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(B1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
    }

    #[test]
    fn test_major_scales_octave_2() {
        assert_eq!(C2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(CSHARP2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(DSHARP2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(FSHARP2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(GSHARP2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(ASHARP2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(B2_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
    }

    #[test]
    fn test_major_scales_octave_3() {
        assert_eq!(C3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(CSHARP3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(DSHARP3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(FSHARP3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(GSHARP3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(ASHARP3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(B3_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
    }

    #[test]
    fn test_major_scales_octave_4() {
        assert_eq!(C4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(CSHARP4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(DSHARP4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(FSHARP4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(GSHARP4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(ASHARP4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(B4_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
    }

    #[test]
    fn test_major_scales_octave_5() {
        assert_eq!(C5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(CSHARP5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(DSHARP5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(FSHARP5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(GSHARP5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(ASHARP5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(B5_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
    }

    #[test]
    fn test_major_scales_octave_6() {
        assert_eq!(C6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(CSHARP6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(DSHARP6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(FSHARP6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(GSHARP6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(ASHARP6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(B6_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
    }

    #[test]
    fn test_major_scales_octave_7() {
        assert_eq!(C7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(CSHARP7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(DSHARP7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(FSHARP7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(GSHARP7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(ASHARP7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(B7_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
    }

    #[test]
    fn test_major_scales_octave_8() {
        assert_eq!(C8_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(CSHARP8_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D8_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(DSHARP8_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E8_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F8_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(FSHARP8_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G8_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
    }

    #[test]
    fn test_scales_contains_canonical() {
        assert_eq!(MAJOR_SCALES.get(&C), Some(&C_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&CSHARP), Some(&CSHARP_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&D), Some(&D_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&DSHARP), Some(&DSHARP_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&E), Some(&E_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&F), Some(&F_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&FSHARP), Some(&FSHARP_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&G), Some(&G_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&GSHARP), Some(&GSHARP_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&A), Some(&A_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&ASHARP), Some(&ASHARP_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&B), Some(&B_MAJOR_SCALE));
    }

    #[test]
    fn test_scales_contains_octave_0() {
        assert_eq!(MAJOR_SCALES.get(&C0), Some(&C0_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&CSHARP0), Some(&CSHARP0_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&D0), Some(&D0_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&DSHARP0), Some(&DSHARP0_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&E0), Some(&E0_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&F0), Some(&F0_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&FSHARP0), Some(&FSHARP0_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&G0), Some(&G0_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&GSHARP0), Some(&GSHARP0_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&A0), Some(&A0_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&ASHARP0), Some(&ASHARP0_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&B0), Some(&B0_MAJOR_SCALE));
    }

    #[test]
    fn test_scales_contains_octave_1() {
        assert_eq!(MAJOR_SCALES.get(&C1), Some(&C1_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&CSHARP1), Some(&CSHARP1_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&D1), Some(&D1_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&DSHARP1), Some(&DSHARP1_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&E1), Some(&E1_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&F1), Some(&F1_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&FSHARP1), Some(&FSHARP1_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&G1), Some(&G1_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&GSHARP1), Some(&GSHARP1_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&A1), Some(&A1_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&ASHARP1), Some(&ASHARP1_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&B1), Some(&B1_MAJOR_SCALE));
    }

    #[test]
    fn test_scales_contains_octave_2() {
        assert_eq!(MAJOR_SCALES.get(&C2), Some(&C2_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&CSHARP2), Some(&CSHARP2_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&D2), Some(&D2_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&DSHARP2), Some(&DSHARP2_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&E2), Some(&E2_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&F2), Some(&F2_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&FSHARP2), Some(&FSHARP2_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&G2), Some(&G2_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&GSHARP2), Some(&GSHARP2_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&A2), Some(&A2_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&ASHARP2), Some(&ASHARP2_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&B2), Some(&B2_MAJOR_SCALE));
    }

    #[test]
    fn test_scales_contains_octave_3() {
        assert_eq!(MAJOR_SCALES.get(&C3), Some(&C3_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&CSHARP3), Some(&CSHARP3_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&D3), Some(&D3_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&DSHARP3), Some(&DSHARP3_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&E3), Some(&E3_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&F3), Some(&F3_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&FSHARP3), Some(&FSHARP3_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&G3), Some(&G3_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&GSHARP3), Some(&GSHARP3_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&A3), Some(&A3_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&ASHARP3), Some(&ASHARP3_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&B3), Some(&B3_MAJOR_SCALE));
    }

    #[test]
    fn test_scales_contains_octave_4() {
        assert_eq!(MAJOR_SCALES.get(&C4), Some(&C4_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&CSHARP4), Some(&CSHARP4_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&D4), Some(&D4_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&DSHARP4), Some(&DSHARP4_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&E4), Some(&E4_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&F4), Some(&F4_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&FSHARP4), Some(&FSHARP4_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&G4), Some(&G4_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&GSHARP4), Some(&GSHARP4_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&A4), Some(&A4_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&ASHARP4), Some(&ASHARP4_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&B4), Some(&B4_MAJOR_SCALE));
    }

    #[test]
    fn test_scales_contains_octave_5() {
        assert_eq!(MAJOR_SCALES.get(&C5), Some(&C5_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&CSHARP5), Some(&CSHARP5_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&D5), Some(&D5_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&DSHARP5), Some(&DSHARP5_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&E5), Some(&E5_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&F5), Some(&F5_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&FSHARP5), Some(&FSHARP5_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&G5), Some(&G5_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&GSHARP5), Some(&GSHARP5_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&A5), Some(&A5_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&ASHARP5), Some(&ASHARP5_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&B5), Some(&B5_MAJOR_SCALE));
    }

    #[test]
    fn test_scales_contains_octave_6() {
        assert_eq!(MAJOR_SCALES.get(&C6), Some(&C6_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&CSHARP6), Some(&CSHARP6_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&D6), Some(&D6_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&DSHARP6), Some(&DSHARP6_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&E6), Some(&E6_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&F6), Some(&F6_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&FSHARP6), Some(&FSHARP6_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&G6), Some(&G6_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&GSHARP6), Some(&GSHARP6_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&A6), Some(&A6_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&ASHARP6), Some(&ASHARP6_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&B6), Some(&B6_MAJOR_SCALE));
    }

    #[test]
    fn test_scales_contains_octave_7() {
        assert_eq!(MAJOR_SCALES.get(&C7), Some(&C7_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&CSHARP7), Some(&CSHARP7_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&D7), Some(&D7_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&DSHARP7), Some(&DSHARP7_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&E7), Some(&E7_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&F7), Some(&F7_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&FSHARP7), Some(&FSHARP7_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&G7), Some(&G7_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&GSHARP7), Some(&GSHARP7_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&A7), Some(&A7_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&ASHARP7), Some(&ASHARP7_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&B7), Some(&B7_MAJOR_SCALE));
    }

    #[test]
    fn test_scales_contains_octave_8() {
        assert_eq!(MAJOR_SCALES.get(&C8), Some(&C8_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&CSHARP8), Some(&CSHARP8_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&D8), Some(&D8_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&DSHARP8), Some(&DSHARP8_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&E8), Some(&E8_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&F8), Some(&F8_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&FSHARP8), Some(&FSHARP8_MAJOR_SCALE));
        assert_eq!(MAJOR_SCALES.get(&G8), Some(&G8_MAJOR_SCALE));
    }
}
