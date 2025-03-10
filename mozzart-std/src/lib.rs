//! Mozzart standard library
//!
//! A comprehensive library for working with musical concepts in Rust, providing types and utilities
//! for handling musical pitches, intervals, and their relationships.
//!
//! # Core Types
//! - [`Pitch`]: Represents a musical pitch using MIDI note numbers (0-127)
//! - [`Interval`]: Represents the distance between two pitches in semitones
//!
//! # Musical Constants
//! The library provides a rich set of musical constants through the [`constants`] module:
//! - Pitch constants (e.g., `C4`, `A4`) for common reference pitches
//! - Interval constants (e.g., `MAJOR_THIRD`, `PERFECT_FIFTH`) for standard musical intervals
//!
//! # Working with Sequences
//! The [`slices`] module provides utilities for working with sequences of musical elements:
//! - Converting between pitch sequences and interval patterns
//! - Analyzing melodic intervals
//! - Building chord voicings from interval patterns
//!
//! # Examples
//! ```
//! use mozzart_std::{Pitch, Interval, C4, E4, G4, MAJOR_THIRD, MINOR_THIRD};
//!
//! // Working with individual pitches and intervals
//! let middle_c = C4;
//! let e_above = middle_c + MAJOR_THIRD;
//! assert_eq!(e_above, E4);
//!
//! // Working with sequences
//! use mozzart_std::{into_intervals, into_pitches};
//!
//! // Analyze intervals in a C major triad
//! let c_major = [C4, E4, G4];
//! let intervals = into_intervals(&c_major);
//! assert_eq!(intervals, vec![MAJOR_THIRD, MINOR_THIRD]);
//!
//! // Build an E major triad using the same interval pattern
//! let e_major = into_pitches(E4, &intervals);
//! assert_eq!(e_major.len(), 3);  // Root + 2 intervals = 3 pitches
//! ```

mod constants;
mod interval;
mod pitch;
mod scale;
mod slices;
mod utils;

pub use constants::*;
pub use interval::*;
pub use pitch::*;
pub use scale::*;
pub use slices::*;
pub(crate) use utils::*;
