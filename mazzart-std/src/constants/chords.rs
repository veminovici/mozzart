use crate::constants::*;
use crate::Interval;

/// Represents the interval structure of a major triad
///
/// A major triad is one of the most fundamental chord types in Western music,
/// characterized by a bright, stable, and resolved sound. It consists of 3 notes:
/// - The root note
/// - A major third above the root (4 semitones)
/// - A minor third above the major third (3 semitones), which forms a perfect fifth (7 semitones) above the root
///
/// This array stores the intervals needed to build a major triad from a root note:
/// - First interval: major third (4 semitones)
/// - Second interval: minor third (3 semitones)
///
/// Major triads are the foundation of major keys and are typically perceived as
/// "happy" or "bright" sounding. They appear as the I, IV, and V chords in a major scale,
/// and as the III, VI, and VII chords in a minor scale.
///
/// # Examples of major triads:
/// - C major: C, E, G
/// - F major: F, A, C
/// - G major: G, B, D
pub const MAJOR_TRIAD_STEPS: [Interval; 2] = [MAJOR_THIRD, MINOR_THIRD];

/// Represents the interval structure of a minor triad
///
/// A minor triad is a fundamental chord type in Western music,
/// characterized by a darker, more melancholic sound compared to a major triad.
/// It consists of 3 notes:
/// - The root note
/// - A minor third above the root (3 semitones)
/// - A major third above the minor third (4 semitones), which forms a perfect fifth (7 semitones) above the root
///
/// This array stores the intervals needed to build a minor triad from a root note:
/// - First interval: minor third (3 semitones)
/// - Second interval: major third (4 semitones)
///
/// Minor triads are the foundation of minor keys and are typically perceived as
/// "sad" or "serious" sounding. They appear as the i, iv, and v chords in a minor scale,
/// and as the ii, iii, and vi chords in a major scale.
///
/// # Examples of minor triads:
/// - C minor: C, Eb, G
/// - A minor: A, C, E
/// - E minor: E, G, B
pub const MINOR_TRIAD_STEPS: [Interval; 2] = [MINOR_THIRD, MAJOR_THIRD];

/// Represents the interval structure of a diminished triad
///
/// A diminished triad has a tense, unstable sound characterized by two stacked minor thirds,
/// resulting in a diminished fifth (6 semitones) between the root and the fifth.
/// It consists of 3 notes:
/// - The root note
/// - A minor third above the root (3 semitones)
/// - A minor third above the second note (3 semitones), which forms a diminished fifth (6 semitones) above the root
///
/// Diminished triads have a distinctly dissonant quality and create tension that seeks resolution.
/// They appear naturally as the vii° chord in major scales and as the ii° chord in minor scales.
///
/// # Examples of diminished triads:
/// - B diminished: B, D, F
/// - D diminished: D, F, Ab
pub const DIMINISHED_TRIAD_STEPS: [Interval; 2] = [MINOR_THIRD, MINOR_THIRD];

/// Represents the interval structure of an augmented triad
///
/// An augmented triad has a bright but unstable sound characterized by two stacked major thirds,
/// resulting in an augmented fifth (8 semitones) between the root and the fifth.
/// It consists of 3 notes:
/// - The root note
/// - A major third above the root (4 semitones)
/// - A major third above the second note (4 semitones), which forms an augmented fifth (8 semitones) above the root
///
/// Augmented triads have an open, expansive, and somewhat mysterious quality.
/// They don't occur naturally in a diatonic scale, but they appear in harmonic minor scales
/// and are often used for color and tension in various musical styles.
///
/// # Examples of augmented triads:
/// - C augmented: C, E, G#
/// - F augmented: F, A, C#
pub const AUGMENTED_TRIAD_STEPS: [Interval; 2] = [MAJOR_THIRD, MAJOR_THIRD];

/// Represents the interval structure of a major seventh chord
///
/// A major seventh chord extends the major triad by adding a major seventh interval.
/// It consists of 4 notes:
/// - The root note
/// - A major third above the root (4 semitones)
/// - A perfect fifth above the root (7 semitones)
/// - A major seventh above the root (11 semitones)
///
/// Major seventh chords have a rich, sophisticated sound commonly used in jazz, pop, and classical music.
/// They create a sense of openness and resolution with a touch of complexity from the seventh.
/// They appear naturally as the I△7 and IV△7 chords in major scales.
///
/// # Examples of major seventh chords:
/// - Cmaj7: C, E, G, B
/// - Fmaj7: F, A, C, E
pub const MAJOR_SEVENTH_CHORD_STEPS: [Interval; 3] = [MAJOR_THIRD, MINOR_THIRD, MAJOR_THIRD];

// Intervals for common chord types using the from_intervals approach
// where each interval is measured from the root note

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

/// Represents the intervals for a diminished triad, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Diminished fifth (6 semitones above root)
pub const DIMINISHED_TRIAD_INTERVALS: [Interval; 2] = [MINOR_THIRD, DIMINISHED_FIFTH];

/// Represents the intervals for an augmented triad, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Augmented fifth (8 semitones above root)
pub const AUGMENTED_TRIAD_INTERVALS: [Interval; 2] = [MAJOR_THIRD, AUGMENTED_FIFTH];

/// Represents the intervals for a major seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Major seventh (11 semitones above root)
pub const MAJOR_SEVENTH_INTERVALS: [Interval; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];

/// Represents the intervals for a dominant seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
pub const DOMINANT_SEVENTH_INTERVALS: [Interval; 3] = [MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];

/// Represents the intervals for a minor seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
pub const MINOR_SEVENTH_INTERVALS: [Interval; 3] = [MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];

/// Represents the intervals for a half-diminished seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Diminished fifth (6 semitones above root)
/// - Minor seventh (10 semitones above root)
pub const HALF_DIMINISHED_SEVENTH_INTERVALS: [Interval; 3] =
    [MINOR_THIRD, DIMINISHED_FIFTH, MINOR_SEVENTH];

/// Represents the intervals for a fully-diminished seventh chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Diminished fifth (6 semitones above root)
/// - Diminished seventh (9 semitones above root)
pub const DIMINISHED_SEVENTH_INTERVALS: [Interval; 3] =
    [MINOR_THIRD, DIMINISHED_FIFTH, MAJOR_SIXTH]; // Diminished 7th is enharmonic with Major 6th

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

/// Represents the intervals for a major sixth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Major sixth (9 semitones above root)
pub const MAJOR_SIXTH_INTERVALS: [Interval; 2] = [MAJOR_THIRD, MAJOR_SIXTH];

/// Represents the intervals for a minor sixth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Major sixth (9 semitones above root)
pub const MINOR_SIXTH_INTERVALS: [Interval; 2] = [MINOR_THIRD, MAJOR_SIXTH];

/// Represents the intervals for a major sixth ninth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Major sixth (9 semitones above root)
/// - Major ninth (14 semitones above root)
pub const MAJOR_SIXTH_NINTH_INTERVALS: [Interval; 3] = [MAJOR_THIRD, MAJOR_SIXTH, MAJOR_SECOND];

/// Represents the intervals for a minor sixth ninth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Major sixth (9 semitones above root)
pub const MINOR_SIXTH_NINTH_INTERVALS: [Interval; 3] = [MINOR_THIRD, MAJOR_SIXTH, MAJOR_SECOND];

/// Represents the intervals for a dominant seventh ninth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Major third (4 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
/// - Major ninth (14 semitones above root)
pub const DOMINANT_SEVENTH_NINTH_INTERVALS: [Interval; 4] =
    [MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH, MAJOR_SECOND];

/// Represents the intervals for a minor seventh ninth chord, measured from the root note
///
/// The notes are:
/// - Root
/// - Minor third (3 semitones above root)
/// - Perfect fifth (7 semitones above root)
/// - Minor seventh (10 semitones above root)
/// - Major ninth (14 semitones above root)
pub const MINOR_SEVENTH_NINTH_INTERVALS: [Interval; 4] =
    [MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH, MAJOR_SECOND];
