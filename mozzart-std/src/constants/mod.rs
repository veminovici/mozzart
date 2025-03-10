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
//! # Examples
//! ```
//! use mozzart_std::{C4, E4, G4, MAJOR_THIRD, MINOR_THIRD};
//!
//! // Using pitch constants
//! let middle_c = C4;  // MIDI note 60
//!
//! // Using interval constants
//! let major_triad = [C4, E4, G4];  // C-E-G
//! let intervals = [MAJOR_THIRD, MINOR_THIRD];  // Major third, minor third
//! ```

mod intervals;
mod pitches;

pub use intervals::*;
pub use pitches::*;
