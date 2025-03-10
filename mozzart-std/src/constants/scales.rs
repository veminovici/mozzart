//! Common musical scale constants
//!
//! This module provides predefined constants for frequently used musical scales.
//! Each scale constant is an octave-spanning scale (8 notes) starting from a specific root pitch.
//!
//! Currently available scales:
//! - C4 scales: Major, Natural Minor, Harmonic Minor, and Melodic Minor
//!
//! All scales follow standard musical theory patterns:
//! - Major: W-W-H-W-W-W-H (whole and half steps)
//! - Natural Minor: W-H-W-W-H-W-W
//! - Harmonic Minor: W-H-W-W-H-W+H-H
//! - Melodic Minor (ascending): W-H-W-W-W-W-H

use crate::constants::*;
use crate::Scale;

/// C4 major scale (C4 to C5)
///
/// Represents a C major scale starting at middle C (C4) and spanning one octave.
/// Contains the pitches: C4, D4, E4, F4, G4, A4, B4, C5
///
/// # Musical Context
/// The C major scale is often used as a reference scale due to its lack of sharps or flats.
pub const C4_MAJOR_SCALE: Scale<8> = Scale::major([C4, D4, E4, F4, G4, A4, B4, C5]);

/// C4 natural minor scale (C4 to C5)
///
/// Represents a C minor scale starting at middle C (C4) and spanning one octave.
/// Contains the pitches: C4, D4, E♭4, F4, G4, A♭4, B♭4, C5
///
/// # Musical Context
/// The natural minor scale, also known as the Aeolian mode, has a darker, more melancholic sound
/// compared to the major scale.
pub const C4_MINOR_SCALE: Scale<8> = Scale::minor([C4, D4, E4, F4, G4, A4, B4, C5]);

/// C4 harmonic minor scale (C4 to C5)
///
/// Represents a C harmonic minor scale starting at middle C (C4) and spanning one octave.
/// Contains the pitches: C4, D4, E♭4, F4, G4, A♭4, B4, C5
///
/// # Musical Context
/// The harmonic minor scale is characterized by its raised seventh degree,
/// creating an augmented second interval between the sixth and seventh degrees.
/// This gives it a distinctive sound often associated with Eastern European and Middle Eastern music.
pub const C4_HARMONIC_MINOR_SCALE: Scale<8> = Scale::harmonic([C4, D4, E4, F4, G4, A4, B4, C5]);

/// C4 melodic minor scale (C4 to C5)
///
/// Represents a C melodic minor scale starting at middle C (C4) and spanning one octave.
/// Contains the pitches (ascending): C4, D4, E♭4, F4, G4, A4, B4, C5
///
/// # Musical Context
/// The melodic minor scale traditionally has different ascending and descending forms.
/// The ascending form raises both the sixth and seventh degrees,
/// while the descending form is identical to the natural minor scale.
/// This scale is commonly used in jazz and modern classical music.
pub const C4_MELODIC_MINOR_SCALE: Scale<8> = Scale::melodic([C4, D4, E4, F4, G4, A4, B4, C5]);
