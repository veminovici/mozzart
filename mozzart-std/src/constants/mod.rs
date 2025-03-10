//! Musical constants for pitches and intervals
//!
//! This module provides a comprehensive set of musical constants:
//!
//! # Pitches
//! - Pitch classes (C, C♯/D♭, D, etc.)
//! - Full range of MIDI notes (C0 to G9)
//! - Standard reference pitches (A4 = 440Hz, C4 = middle C)
//!
//! # Intervals
//! - Basic intervals (semitone, tone)
//! - Standard intervals (unison to octave)
//! - Extended intervals (ninth to double octave)
//!
//! # Scales
//! - Major scale
//! - Minor scale
//! - Harmonic minor scale
//! - Melodic minor scale
//!
//! # Examples
//! ```
//! use mozzart_std::{C4, E4, G4, MAJOR_THIRD, MINOR_THIRD, A4_MAJOR_SCALE, C4_MINOR_SCALE, G4_HARMONIC_SCALE, D4_MELODIC_SCALE};
//!
//! // Using pitch constants
//! let middle_c = C4;  // MIDI note 60
//!
//! // Using interval constants
//! let major_triad = [C4, E4, G4];  // C-E-G
//! let intervals = [MAJOR_THIRD, MINOR_THIRD];  // Major third, minor third
//!
//! // Using scale constants
//! let a_major = A4_MAJOR_SCALE;
//! let c_minor = C4_MINOR_SCALE;
//! let g_harmonic_minor = G4_HARMONIC_SCALE;
//! let d_melodic_minor = D4_MELODIC_SCALE;
//! ```

mod intervals;
mod pitches;
mod scales;

pub use intervals::*;
pub use pitches::*;
pub use scales::*;
