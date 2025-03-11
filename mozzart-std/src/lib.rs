//! Mozzart standard library
//!
//! A comprehensive library for working with musical concepts in Rust, providing types and utilities
//! for handling musical pitches, intervals, scales, and their relationships.
//!
//! # Core Types
//! - [`Pitch`]: Represents a musical pitch using MIDI note numbers (0-127)
//! - [`Interval`]: Represents the distance between two pitches in semitones
//! - [`Scale`]: Represents musical scales as sequences of pitches
//!
//! # Musical Constants
//! The library provides a rich set of musical constants through the [`constants`] module:
//! - Pitch constants (e.g., `C4`, `A4`) for common reference pitches across all MIDI octaves (0-8)
//! - Interval constants (e.g., `MAJOR_THIRD`, `PERFECT_FIFTH`) for standard musical intervals
//! - Scale constants for major, minor, harmonic, and melodic scales in all keys and octaves
//!
//! # Scale System
//! The library offers a comprehensive scale system with:
//! - Major scales: The standard 7-note diatonic major scale (TONE, TONE, SEMITONE, TONE, TONE, TONE, SEMITONE)
//! - Natural minor scales: The Aeolian mode with its characteristic melancholy sound
//! - Harmonic minor scales: Minor scales with a raised 7th degree
//! - Melodic minor scales: Minor scales with raised 6th and 7th degrees when ascending
//!
//! Each scale type is available in all 12 keys across the entire MIDI range (octaves 0-8).
//!
//! # Octave Shifting
//! The library provides convenient octave-shifting operations through bit-shift operators:
//! - Shift a pitch up an octave with `<<` (e.g., `C4 << 1` becomes `C5`)
//! - Shift a pitch down an octave with `>>` (e.g., `C4 >> 1` becomes `C3`)
//! - Shift by multiple octaves by specifying the number (e.g., `C4 << 2` becomes `C6`)
//!
//! These operations make it easy to transpose scales and patterns across octaves while
//! maintaining their relative intervals.
//!
//! # Working with Sequences
//! The [`slices`] module provides utilities for working with sequences of musical elements:
//! - Converting between pitch sequences and interval patterns
//! - Analyzing melodic intervals
//! - Building chord voicings from interval patterns
//!
//! # Examples
//! ```
//! use mozzart_std::{Pitch, Interval, A4, C4, C5, C6, D4, F4, E4, MAJOR_THIRD, MINOR_THIRD, C5_MAJOR_SCALE, C3_MAJOR_SCALE, C4_MAJOR_SCALE};
//!
//! // Working with individual pitches and intervals
//! let middle_c = C4;
//! let e_above = middle_c + MAJOR_THIRD;
//! assert_eq!(e_above, E4);
//!
//! // Working with scales
//! let c_major = C4.major_scale();
//! let a_minor = A4.minor_scale();
//! let d_harmonic = D4.harmonic_scale();
//! let f_melodic = F4.melodic_scale();
//!
//! // Shifting pitches
//! assert_eq!(C5 << 1, C4); // octave down
//! assert_eq!(C5 >> 1, C6); // octave up
//!
//! // Shifting scales
//! let c4_major = C4_MAJOR_SCALE;
//! let c5_major = c4_major >> 1; // octave up
//! assert_eq!(c5_major, C5_MAJOR_SCALE);
//!
//! let c3_major = c4_major << 1; // octave down
//! assert_eq!(c3_major, C3_MAJOR_SCALE);
//! ```

mod constants;
mod interval;
mod pitch;
mod scale;
mod utils;

pub use constants::*;
pub use interval::*;
pub use pitch::*;
pub use scale::*;
pub use utils::*;
