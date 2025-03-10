//! Musical slice types for working with sequences of pitches and intervals
//!
//! This module provides types and utilities for working with sequences of musical elements,
//! enabling seamless conversion between pitch sequences and their interval representations.
//!
//! # Types
//! - `PitchSlice`: A sequence of pitches that can be converted into intervals. Useful for:
//!   - Analyzing the intervals between consecutive notes in a melody
//!   - Converting chord voicings into their interval structure
//!   - Working with musical scales and their pitch content
//!
//! - `IntervalSlice`: A sequence of intervals that can be converted into pitches. Useful for:
//!   - Building chords from a root note and interval pattern
//!   - Transposing melodic patterns to different root notes
//!   - Generating scale patterns from interval structures
//!
//! # Helper Functions
//! The module also provides two convenience functions for quick conversions:
//! - `into_intervals`: Converts a slice of pitches into a vector of intervals
//! - `into_pitches`: Converts a slice of intervals into a vector of pitches from a given root
//!
//! # Examples
//! ```
//! use mozzart_std::{PitchSlice, IntervalSlice, C4, E4, G4, MAJOR_THIRD, MINOR_THIRD};
//!
//! // Converting a sequence of pitches to intervals
//! let c_major = [C4, E4, G4];  // C-E-G
//! let intervals = PitchSlice::new(&c_major).into_intervals();
//! assert_eq!(intervals, vec![MAJOR_THIRD, MINOR_THIRD]);
//!
//! // Converting a sequence of intervals to pitches
//! let major_triad = [MAJOR_THIRD, MINOR_THIRD];  // Major third, minor third
//! let pitches = IntervalSlice::new(&major_triad).into_pitches(C4);
//! assert_eq!(pitches, vec![C4, E4, G4]);
//! ```
//!
//! More complex example with helper functions:
//! ```
//! use mozzart_std::{into_intervals, into_pitches, C4, E4, G4, MAJOR_THIRD, MINOR_THIRD};
//!
//! // Using helper functions for quick conversions
//! let chord = [C4, E4, G4];
//! let intervals = into_intervals(&chord);
//! assert_eq!(intervals, vec![MAJOR_THIRD, MINOR_THIRD]);
//!
//! // Transpose the same interval pattern to a different root
//! let transposed = into_pitches(E4,&intervals);  // E major triad
//! assert_eq!(transposed.len(), 3);  // Root + 2 intervals = 3 pitches
//! ```

mod interval_slice;
mod pitch_slice;

pub use interval_slice::*;
pub use pitch_slice::*;

/// Converts a sequence of pitches into their corresponding intervals.
///
/// This helper function calculates the intervals between consecutive pitches in the sequence.
/// For example, the sequence C4-E4-G4 is converted to [M3, m3] representing the intervals
/// between C-E and E-G respectively.
///
/// # Arguments
/// * `slice` - A slice of [`Pitch`]es to convert into intervals
///
/// # Returns
/// A vector of [`Interval`]s representing the intervals between consecutive pitches.
/// The length of the returned vector will be one less than the input slice length.
///
/// # Examples
/// ```
/// use mozzart_std::{into_intervals, C4, E4, G4, MAJOR_THIRD, MINOR_THIRD};
///
/// let chord = [C4, E4, G4];  // C major triad
/// let intervals = into_intervals(&chord);
/// assert_eq!(intervals, vec![MAJOR_THIRD, MINOR_THIRD]);
/// ```
pub fn into_intervals(slice: &[crate::Pitch]) -> Vec<crate::Interval> {
    PitchSlice::new(slice).into_intervals()
}

/// Converts a sequence of intervals into pitches, starting from a given root pitch.
///
/// This helper function generates a sequence of pitches by applying each interval
/// cumulatively to the root pitch. For example, with root C4 and intervals [M3, m3],
/// it generates [C4, E4, G4] by:
/// 1. Starting with C4
/// 2. Adding M3 to get E4
/// 3. Adding m3 to get G4
///
/// # Arguments
/// * `root` - The starting [`Pitch`] from which to generate the sequence
/// * `slice` - A slice of [`Interval`]s to apply successively
///
/// # Returns
/// A vector of [`Pitch`]es starting with the root pitch, followed by pitches
/// obtained by successively applying each interval. The length of the returned
/// vector will be one more than the input slice length.
///
/// # Examples
/// ```
/// use mozzart_std::{into_pitches, C4, E4, G4, MAJOR_THIRD, MINOR_THIRD};
///
/// let intervals = [MAJOR_THIRD, MINOR_THIRD];  // Major triad pattern
/// let pitches = into_pitches(C4, &intervals);  // Build C major triad
/// assert_eq!(pitches, vec![C4, E4, G4]);
/// ```
pub fn into_pitches(root: crate::Pitch, slice: &[crate::Interval]) -> Vec<crate::Pitch> {
    IntervalSlice::new(slice).into_pitches(root)
}
