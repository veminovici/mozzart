//! Musical interval constants
//!
//! This module provides constants for common musical intervals, from unison
//! up to a double octave. Each interval is represented by its semitone count.
//!
//! # Categories
//! - Basic intervals: fundamental building blocks (semitone, tone)
//! - Standard intervals: common intervals within an octave
//! - Extended intervals: intervals beyond one octave
//!
//! # Common Uses
//! ```
//! use mozzart_std::{C4, MAJOR_THIRD, MINOR_THIRD, PERFECT_FIFTH};
//!
//! // Building a major triad (C-E-G)
//! let root = C4;
//! let third = root + MAJOR_THIRD;    // E4
//! let fifth = root + PERFECT_FIFTH;  // G4
//! ```

use crate::Interval;

/// Basic intervals (1-2 semitones)
///
/// These are the fundamental building blocks of all other intervals:
/// - SEMITONE: The smallest interval in Western music (e.g., C to C♯)
/// - TONE: Two semitones (e.g., C to D)
pub const SEMITONE: Interval = Interval::new(1);
pub const TONE: Interval = Interval::new(2);

/// Standard intervals (within one octave)
///
/// These intervals form the basis of Western harmony:
/// - Perfect intervals: unison (0), fourth (5), fifth (7), octave (12)
/// - Major intervals: second (2), third (4), sixth (9), seventh (11)
/// - Minor intervals: second (1), third (3), sixth (8), seventh (10)
/// - Special intervals: augmented fourth/diminished fifth (6) - the tritone
pub const UNISON: Interval = Interval::new(0);
pub const MINOR_SECOND: Interval = Interval::new(1);
pub const MAJOR_SECOND: Interval = Interval::new(2);
pub const MINOR_THIRD: Interval = Interval::new(3);
pub const MAJOR_THIRD: Interval = Interval::new(4);
pub const PERFECT_FOURTH: Interval = Interval::new(5);
pub const AUGMENTED_FOURTH: Interval = Interval::new(6);
pub const DIMINISHED_FIFTH: Interval = Interval::new(6);
pub const PERFECT_FIFTH: Interval = Interval::new(7);
pub const MINOR_SIXTH: Interval = Interval::new(8);
pub const MAJOR_SIXTH: Interval = Interval::new(9);
pub const MINOR_SEVENTH: Interval = Interval::new(10);
pub const MAJOR_SEVENTH: Interval = Interval::new(11);
pub const PERFECT_OCTAVE: Interval = Interval::new(12);

/// Extended intervals (beyond one octave)
///
/// These intervals extend beyond an octave and are commonly used in:
/// - Complex chord voicings
/// - Extended harmony (9th, 11th, 13th chords)
/// - Wide melodic leaps
///
/// Note: Many of these intervals have enharmonic equivalents:
/// - Minor ninth = Octave + minor second (13 semitones)
/// - Major tenth = Octave + major third (16 semitones)
/// - Perfect twelfth = Octave + perfect fifth (19 semitones)
pub const MINOR_NINTH: Interval = Interval::new(13);
pub const MAJOR_NINTH: Interval = Interval::new(14);
pub const MINOR_TENTH: Interval = Interval::new(15);
pub const MAJOR_TENTH: Interval = Interval::new(16);
pub const PERFECT_ELEVENTH: Interval = Interval::new(17);
pub const AUGMENTED_ELEVENTH: Interval = Interval::new(18);
pub const PERFECT_TWELFTH: Interval = Interval::new(19);
pub const MINOR_THIRTEENTH: Interval = Interval::new(20);
pub const MAJOR_THIRTEENTH: Interval = Interval::new(21);
pub const MINOR_FOURTEENTH: Interval = Interval::new(22);
pub const MAJOR_FOURTEENTH: Interval = Interval::new(23);
pub const DOUBLE_OCTAVE: Interval = Interval::new(24);
