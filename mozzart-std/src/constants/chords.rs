use crate::constants::*;
use crate::Interval;

/// Represents the intervals for a major triad, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
pub const MAJOR_TRIAD_INTERVALS: [Interval; 2] = [MAJOR_THIRD, PERFECT_FIFTH];

/// Represents the intervals for a minor triad, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Perfect fifth (7 semitones above root)
pub const MINOR_TRIAD_INTERVALS: [Interval; 2] = [MINOR_THIRD, PERFECT_FIFTH];

/// Represents the intervals for a dominant seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
pub const DOMINANT_SEVENTH_INTERVALS: [Interval; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];

/// Represents the intervals for a dominant seventh ninth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
/// - Major ninth (14 semitones above root)
pub const DOMINANT_SEVENTH_NINTH_INTERVALS: [Interval; 4] =
    [MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH, MAJOR_NINTH];

/// Represents the intervals for a minor seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
pub const MINOR_SEVENTH_INTERVALS: [Interval; 3] = [MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];

/// Represents the intervals for a minor seventh ninth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
/// - Major ninth (14 semitones above root)
pub const MINOR_SEVENTH_NINTH_INTERVALS: [Interval; 4] =
    [MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH, MAJOR_NINTH];

/// Represents the intervals for a major seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Major seventh (11 semitones above root)
pub const MAJOR_SEVENTH_INTERVALS: [Interval; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];

/// Represents the intervals for a minor major seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Major seventh (11 semitones above root)
pub const MINOR_MAJOR_SEVENTH_INTERVALS: [Interval; 3] =
    [MINOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];

/// Represents the intervals for a major sixth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Major sixth (9 semitones above root)
pub const MAJOR_SIXTH_INTERVALS: [Interval; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MINOR_SIXTH];

/// Represents the intervals for a minor sixth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor sixth (9 semitones above root)
pub const MINOR_SIXTH_INTERVALS: [Interval; 3] = [MINOR_THIRD, PERFECT_FIFTH, MINOR_SIXTH];

/// Represents the intervals for a major sixth ninth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Major sixth (9 semitones above root)
/// - Major ninth (14 semitones above root)
pub const MAJOR_SIXTH_NINTH_INTERVALS: [Interval; 4] =
    [MAJOR_THIRD, PERFECT_FIFTH, MINOR_SIXTH, MAJOR_NINTH];

/// Represents the intervals for a minor sixth ninth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Major sixth (9 semitones above root)
pub const MINOR_SIXTH_NINTH_INTERVALS: [Interval; 4] =
    [MINOR_THIRD, PERFECT_FIFTH, MINOR_SIXTH, MAJOR_NINTH];

/// Represents the intervals for a suspended 2nd chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major second (2 semitones above root)
/// - Perfect fifth (7 semitones above root)
pub const SUS2_INTERVALS: [Interval; 2] = [MAJOR_SECOND, PERFECT_FIFTH];

/// Represents the intervals for a suspended 4th chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Perfect fourth (5 semitones above root)
/// - Perfect fifth (7 semitones above root)
pub const SUS4_INTERVALS: [Interval; 2] = [PERFECT_FOURTH, PERFECT_FIFTH];

/// Represents the intervals for a diminished triad, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Diminished fifth (6 semitones above root)
pub const DIMINISHED_TRIAD_INTERVALS: [Interval; 2] = [MINOR_THIRD, DIMINISHED_FIFTH];

/// Represents the intervals for a fully-diminished seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Diminished fifth (6 semitones above root)
/// - Diminished seventh (9 semitones above root)
pub const DIMINISHED_SEVENTH_INTERVALS: [Interval; 3] =
    [MINOR_THIRD, DIMINISHED_FIFTH, MINOR_SIXTH];

/// Represents the intervals for a half-diminished seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Diminished fifth (6 semitones above root)
/// - Minor seventh (10 semitones above root)
pub const HALF_DIMINISHED_SEVENTH_INTERVALS: [Interval; 3] =
    [MINOR_THIRD, DIMINISHED_FIFTH, MINOR_SEVENTH];

/// Represents the intervals for an augmented triad, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Augmented fifth (8 semitones above root)
pub const AUGMENTED_TRIAD_INTERVALS: [Interval; 2] = [MAJOR_THIRD, AUGMENTED_FIFTH];

/// Represents the intervals for an augmented seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Augmented fifth (8 semitones above root)
/// - Augmented seventh (12 semitones above root)
pub const AUGMENTED_SEVENTH_INTERVALS: [Interval; 3] = [MAJOR_THIRD, AUGMENTED_FIFTH, MAJOR_SIXTH];

/// Represents the intervals for a dominant ninth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
/// - Major ninth (14 semitones above root)
pub const DOMINANT_NINTH_INTERVALS: [Interval; 4] =
    [MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH, MAJOR_NINTH];

/// Represents the intervals for a minor ninth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
/// - Major ninth (14 semitones above root)
pub const MINOR_NINTH_INTERVALS: [Interval; 4] =
    [MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH, MAJOR_NINTH];

/// Represents the intervals for a major ninth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Major seventh (11 semitones above root)
/// - Major ninth (14 semitones above root)
pub const MAJOR_NINTH_INTERVALS: [Interval; 4] =
    [MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH, MAJOR_NINTH];

/// Represents the intervals for a dominant eleventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
/// - Major ninth (14 semitones above root)
/// - Perfect eleventh (18 semitones above root)
pub const DOMINANT_ELEVENTH_INTERVALS: [Interval; 5] = [
    MAJOR_THIRD,
    PERFECT_FIFTH,
    MINOR_SEVENTH,
    MAJOR_NINTH,
    PERFECT_ELEVENTH,
];

/// Represents the intervals for a minor eleventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
/// - Major ninth (14 semitones above root)
/// - Perfect eleventh (18 semitones above root)
pub const MINOR_ELEVENTH_INTERVALS: [Interval; 5] = [
    MINOR_THIRD,
    PERFECT_FIFTH,
    MINOR_SEVENTH,
    MAJOR_NINTH,
    PERFECT_ELEVENTH,
];

/// Represents the intervals for a major eleventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Major seventh (11 semitones above root)
/// - Major ninth (14 semitones above root)
/// - Perfect eleventh (18 semitones above root)
pub const MAJOR_ELEVENTH_INTERVALS: [Interval; 5] = [
    MAJOR_THIRD,
    PERFECT_FIFTH,
    MAJOR_SEVENTH,
    MAJOR_NINTH,
    PERFECT_ELEVENTH,
];

/// Represents the intervals for a dominant thirteenth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
/// - Major ninth (14 semitones above root)
/// - Perfect eleventh (18 semitones above root)
/// - Minor thirteenth (21 semitones above root)
pub const DOMINANT_THIRTEENTH_INTERVALS: [Interval; 6] = [
    MAJOR_THIRD,
    PERFECT_FIFTH,
    MINOR_SEVENTH,
    MAJOR_NINTH,
    PERFECT_ELEVENTH,
    MINOR_THIRTEENTH,
];

/// Represents the intervals for a minor thirteenth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
/// - Major ninth (14 semitones above root)
/// - Perfect eleventh (18 semitones above root)
/// - Minor thirteenth (21 semitones above root)
pub const MINOR_THIRTEENTH_INTERVALS: [Interval; 6] = [
    MINOR_THIRD,
    PERFECT_FIFTH,
    MINOR_SEVENTH,
    MAJOR_NINTH,
    PERFECT_ELEVENTH,
    MINOR_THIRTEENTH,
];

/// Represents the intervals for a major thirteenth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Major seventh (11 semitones above root)
/// - Major ninth (14 semitones above root)
/// - Perfect eleventh (18 semitones above root)
/// - Major thirteenth (22 semitones above root)
pub const MAJOR_THIRTEENTH_INTERVALS: [Interval; 6] = [
    MAJOR_THIRD,
    PERFECT_FIFTH,
    MAJOR_SEVENTH,
    MAJOR_NINTH,
    PERFECT_ELEVENTH,
    MINOR_THIRTEENTH,
];
