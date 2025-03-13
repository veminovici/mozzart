//! Musical constants for the mazzart-ply library
//!
//! This module provides a comprehensive set of musical constants including:
//! - Intervals (semitones, whole tones, thirds, fifths, etc.)
//! - Notes (predefined MIDI note values)
//! - Fundamental musical values (like semitones in an octave)
//! - Steps (semitones, whole tones, etc.)
//!
//! These constants serve as building blocks for more complex musical structures
//! and calculations throughout the library.

mod chords;
mod intervals;
mod notes;
mod scales;
mod steps;

pub use chords::*;
pub use intervals::*;
pub use notes::*;
pub use scales::*;
pub use steps::*;

/// Number of semitones in an octave in the standard Western equal temperament system
///
/// In Western music, the octave is divided into 12 equal semitones.
/// This constant is fundamental to many musical calculations:
/// - Converting between note numbers and frequencies
/// - Calculating intervals across octaves
/// - Defining scales and chords
///
/// This value (12) has been the standard division of the octave in Western music
/// since the adoption of equal temperament in the 18th century.
pub(crate) const SEMITONES_IN_OCTAVE: u8 = 12;
