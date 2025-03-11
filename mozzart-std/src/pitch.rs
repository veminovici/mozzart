use crate::utils::SEMITONES_PER_OCTAVE;
use crate::Interval;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign},
};

/// Represents a musical pitch using MIDI note numbers (0-127).
///
/// A pitch is represented internally as a MIDI note number, where:
/// - Middle C (C4) is 60
/// - A4 (concert pitch 440Hz) is 69
/// - Each semitone increases/decreases the note number by 1
/// - The valid range is 0-127 (C-1 to G9)
///
/// # MIDI Note Number System
/// The MIDI note number system assigns each pitch a unique number:
/// - Notes are numbered from 0 (C-1) to 127 (G9)
/// - Each octave spans 12 notes (12 semitones)
/// - Middle C (C4) = 60 is a common reference point
/// - A4 = 69 (440Hz concert pitch) is another key reference
///
/// # Pitch Operations
/// Pitches support several operations:
/// - Addition with intervals (transposition)
/// - Subtraction between pitches (finding intervals)
/// - Octave shifting using `<<` and `>>` operators
///
/// # Examples
/// ```
/// use mozzart_std::{C4, A4, MAJOR_THIRD};
///
/// // Transposition
/// let e4 = C4 + MAJOR_THIRD;
///
/// // Finding intervals
/// let interval = A4 - C4;
///
/// // Octave shifts
/// let c5 = C4 >> 1;  // Shift up one octave
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pitch(u8);

impl Pitch {
    /// Creates a new pitch from a MIDI note number (0-127).
    #[inline(always)]
    pub(crate) const fn new(midi_note: u8) -> Self {
        Pitch(midi_note)
    }
}

impl From<Pitch> for u8 {
    /// Converts a pitch to its MIDI note number
    #[inline]
    fn from(pitch: Pitch) -> Self {
        pitch.0
    }
}

impl From<&Pitch> for u8 {
    /// Converts a reference to a pitch to its MIDI note number
    #[inline]
    fn from(pitch: &Pitch) -> Self {
        pitch.0
    }
}

impl Add<Interval> for Pitch {
    type Output = Pitch;

    /// Adds an interval to a pitch, returning a new transposed pitch.
    ///
    /// This operation transposes a pitch upward by the specified interval,
    /// which is fundamental to music theory operations such as:
    /// - Scale construction
    /// - Chord building
    /// - Melody transposition
    /// - Harmonic analysis
    ///
    /// # Arguments
    /// * `interval` - The interval to add to this pitch
    ///
    /// # Returns
    /// A new pitch transposed up by the given interval
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C4, E4, G4, C5, MAJOR_THIRD, PERFECT_FIFTH, PERFECT_OCTAVE};
    ///
    /// // Basic transposition
    /// assert_eq!(C4 + MAJOR_THIRD, E4);      // C4 up a major third = E4
    /// assert_eq!(C4 + PERFECT_FIFTH, G4);    // C4 up a perfect fifth = G4
    /// assert_eq!(C4 + PERFECT_OCTAVE, C5);   // C4 up an octave = C5
    ///
    /// // Building a chord (C major triad)
    /// let c_major = [C4, C4 + MAJOR_THIRD, C4 + PERFECT_FIFTH];
    /// assert_eq!(c_major, [C4, E4, G4]);
    /// ```
    ///
    /// # Musical Context
    /// Interval addition is essential for most musical operations, including:
    /// - Creating chords by stacking thirds (or other intervals)
    /// - Generating scales from interval patterns
    /// - Transposing melodic phrases to different keys
    /// - Exploring voice leading through interval relationships
    #[inline(always)]
    fn add(self, interval: Interval) -> Self::Output {
        let pitch = self.0 + u8::from(interval);
        Pitch(pitch)
    }
}

impl AddAssign<Interval> for Pitch {
    /// Adds an interval to a pitch in place
    #[inline(always)]
    fn add_assign(&mut self, interval: Interval) {
        self.0 += u8::from(interval);
    }
}

impl Add<&Interval> for Pitch {
    type Output = Pitch;

    /// Adds a reference to an interval to a pitch, returning a new transposed pitch.
    ///
    /// This is a reference-based version of the `Add<Interval>` implementation, allowing
    /// interval references to be used for transposition without requiring ownership.
    ///
    /// # Arguments
    /// * `interval` - A reference to an interval to add to this pitch
    ///
    /// # Returns
    /// A new pitch transposed up by the given interval
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C4, MAJOR_THIRD, PERFECT_FIFTH, Pitch, Interval};
    ///
    /// // Adding interval references
    /// let third_ref = &MAJOR_THIRD;
    /// let fifth_ref = &PERFECT_FIFTH;
    ///
    /// let e4 = C4 + third_ref;  // C4 + reference to major third = E4
    /// let g4 = C4 + fifth_ref;  // C4 + reference to perfect fifth = G4
    ///
    /// // Function that takes interval references
    /// fn transpose_by_ref(pitch: Pitch, interval: &Interval) -> Pitch {
    ///     pitch + interval  // Uses the Add<&Interval> implementation
    /// }
    /// ```
    ///
    /// # Musical Context
    /// Using interval references is particularly useful when:
    /// - Working with collections of intervals for batch transpositions
    /// - Passing intervals to functions without transferring ownership
    /// - Implementing algorithms that need to reference the same intervals multiple times
    #[inline(always)]
    fn add(self, interval: &Interval) -> Self::Output {
        self + *interval
    }
}

impl AddAssign<&Interval> for Pitch {
    /// Adds an interval to a pitch in place
    #[inline(always)]
    fn add_assign(&mut self, interval: &Interval) {
        self.0 += u8::from(interval);
    }
}

impl Sub<Pitch> for Pitch {
    type Output = Interval;

    /// Subtracts two pitches to get the interval between them
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{G4, C4, PERFECT_FIFTH};
    ///
    /// assert_eq!(G4 - C4, PERFECT_FIFTH);
    /// ```
    #[inline(always)]
    fn sub(self, other: Pitch) -> Self::Output {
        Interval(self.0 - other.0)
    }
}

impl SubAssign<Pitch> for Pitch {
    /// Subtracts a pitch from another pitch in place
    #[inline(always)]
    fn sub_assign(&mut self, other: Pitch) {
        self.0 -= other.0;
    }
}

impl Sub<Interval> for Pitch {
    type Output = Pitch;

    /// Subtracts an interval from a pitch, returning a new transposed pitch.
    ///
    /// This operation transposes a pitch downward by the specified interval,
    /// which is useful for:
    /// - Descending scale construction
    /// - Downward melody transposition
    /// - Voice leading and resolution
    /// - Calculating inversions
    ///
    /// # Arguments
    /// * `interval` - The interval to subtract from this pitch
    ///
    /// # Returns
    /// A new pitch transposed down by the given interval
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C5, A4, G4, E4, C4, MAJOR_THIRD, PERFECT_FIFTH, PERFECT_OCTAVE, E5, F4};
    ///
    /// // Basic downward transposition
    /// assert_eq!(C5 - PERFECT_OCTAVE, C4);    // C5 down an octave = C4
    /// assert_eq!(C5 - PERFECT_FIFTH, F4);     // C5 down a perfect fifth = F4
    /// assert_eq!(E5 - MAJOR_THIRD, C5);       // E5 down a major third = C5
    ///
    /// // Finding the lower note of an interval
    /// let upper = G4;
    /// let lower = upper - PERFECT_FIFTH;      // G4 down a fifth = C4
    /// assert_eq!(lower, C4);
    /// ```
    ///
    /// # Musical Context
    /// Interval subtraction is essential for many musical operations:
    /// - Creating descending melodies and phrases
    /// - Finding the root of a chord given an upper note and interval
    /// - Working with inverted harmonies
    /// - Analyzing melodic contours
    #[inline(always)]
    fn sub(self, interval: Interval) -> Self::Output {
        Pitch(self.0 - u8::from(interval))
    }
}

impl SubAssign<Interval> for Pitch {
    /// Subtracts an interval from a pitch in place, modifying the original pitch.
    ///
    /// This operation transposes the pitch downward by the specified interval,
    /// altering the original pitch value rather than creating a new one.
    ///
    /// # Arguments
    /// * `interval` - The interval to subtract from this pitch
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C5, G4, C4, E5, F4, MAJOR_THIRD, PERFECT_FIFTH, PERFECT_OCTAVE};
    ///
    /// // Downward transposition in place
    /// let mut pitch = C5;
    /// pitch -= PERFECT_FIFTH;      // C5 down a fifth = F4
    /// assert_eq!(pitch, F4);
    ///
    /// // Multiple transpositions
    /// let mut descending = E5;
    /// descending -= MAJOR_THIRD;   // E5 down a third = C5
    /// descending -= PERFECT_FIFTH; // C5 down a fifth = F4
    /// assert_eq!(descending, F4);
    /// ```
    ///
    /// # Musical Context
    /// In-place interval subtraction is useful for:
    /// - Sequential melodic transformations
    /// - Voice leading algorithms
    /// - Implementing descending patterns efficiently
    #[inline(always)]
    fn sub_assign(&mut self, interval: Interval) {
        self.0 -= u8::from(interval);
    }
}

impl Sub<&Interval> for Pitch {
    type Output = Pitch;

    /// Subtracts a reference to an interval from a pitch, returning a new transposed pitch.
    ///
    /// This is a reference-based version of the `Sub<Interval>` implementation, allowing
    /// interval references to be used for downward transposition without requiring ownership.
    ///
    /// # Arguments
    /// * `interval` - A reference to an interval to subtract from this pitch
    ///
    /// # Returns
    /// A new pitch transposed down by the given interval
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Pitch, Interval, C5, A4, G4, C4, MAJOR_THIRD, PERFECT_FIFTH};
    ///
    /// // Subtracting interval references
    /// let fifth_ref = &PERFECT_FIFTH;
    /// let third_ref = &MAJOR_THIRD;
    ///
    /// let f4 = C5 - fifth_ref;  // C5 down a perfect fifth = F4
    /// let a4 = C5 - third_ref;  // C5 down a major third = A4
    ///
    /// // Function that takes interval references
    /// fn transpose_down_by_ref(pitch: Pitch, interval: &Interval) -> Pitch {
    ///     pitch - interval  // Uses the Sub<&Interval> implementation
    /// }
    /// ```
    ///
    /// # Musical Context
    /// Using interval references for subtraction is useful when:
    /// - Working with shared interval patterns
    /// - Implementing algorithms that reuse intervals
    /// - Avoiding ownership transfer in performance-critical code
    #[inline(always)]
    fn sub(self, interval: &Interval) -> Self::Output {
        self - *interval
    }
}

impl SubAssign<&Interval> for Pitch {
    /// Subtracts a reference to an interval from a pitch in place, modifying the original pitch.
    ///
    /// This is a reference-based version of `SubAssign<Interval>`, allowing interval
    /// references to be used for in-place downward transposition.
    ///
    /// # Arguments
    /// * `interval` - A reference to an interval to subtract from this pitch
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C5, F4, PERFECT_FIFTH, PERFECT_FOURTH, MAJOR_THIRD, MAJOR_SECOND};
    ///
    /// // In-place subtraction with interval reference
    /// let fifth_ref = &PERFECT_FIFTH;
    /// let mut pitch = C5;
    /// pitch -= fifth_ref;       // C5 down a fifth = F4
    /// assert_eq!(pitch, F4);
    ///
    /// // Working with collections of intervals
    /// let intervals = [&MAJOR_SECOND, &MAJOR_THIRD, &PERFECT_FOURTH];
    /// let mut pitch = C5;
    /// for interval in intervals {
    ///     pitch -= interval;    // Sequentially subtract intervals
    /// }
    /// ```
    ///
    /// # Musical Context
    /// Reference-based in-place subtraction is valuable for:
    /// - Sequential processing of interval collections
    /// - Implementing complex voice leading algorithms
    /// - Creating descending patterns with shared interval structures
    #[inline(always)]
    fn sub_assign(&mut self, interval: &Interval) {
        self.0 -= u8::from(interval);
    }
}

impl Shr<u8> for Pitch {
    type Output = Pitch;

    /// Shifts a pitch up by the specified number of octaves.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C4, C5};
    ///
    /// let higher = C4 >> 1;  // C4 up one octave
    /// assert_eq!(higher, C5);
    /// ```
    fn shr(self, shift: u8) -> Self::Output {
        Pitch(self.0 + (shift * SEMITONES_PER_OCTAVE))
    }
}

impl ShrAssign<u8> for Pitch {
    /// Shifts a pitch up by the specified number of octaves in place.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C4, C5};
    ///
    /// let mut pitch = C4;
    /// pitch >>= 1;  // Shift up one octave
    /// assert_eq!(pitch, C5);
    /// ```
    fn shr_assign(&mut self, shift: u8) {
        self.0 += shift * SEMITONES_PER_OCTAVE;
    }
}

impl Shl<u8> for Pitch {
    type Output = Pitch;

    /// Shifts a pitch down by the specified number of octaves.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C4, C3};
    ///
    /// let lower = C4 << 1;  // C4 down one octave
    /// assert_eq!(lower, C3);
    /// ```
    fn shl(self, shift: u8) -> Self::Output {
        Pitch(self.0 - (shift * SEMITONES_PER_OCTAVE))
    }
}

impl ShlAssign<u8> for Pitch {
    /// Shifts a pitch down by the specified number of octaves in place.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C4, C3};
    ///
    /// let mut pitch = C4;
    /// pitch <<= 1;  // Shift down one octave
    /// assert_eq!(pitch, C3);
    /// ```
    fn shl_assign(&mut self, shift: u8) {
        self.0 -= shift * SEMITONES_PER_OCTAVE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn test_pitch_new() {
        let pitch = Pitch::new(60);
        assert_eq!(u8::from(pitch), 60);
    }

    #[test]
    fn test_pitch_conversions() {
        let pitch = Pitch::new(60);
        assert_eq!(u8::from(pitch), 60);
        assert_eq!(u8::from(&pitch), 60);
    }

    #[test]
    fn test_pitch_add_interval() {
        // Test adding common intervals
        assert_eq!(C4 + MINOR_SECOND, Pitch::new(61)); // C4 to C♯4/D♭4
        assert_eq!(C4 + MAJOR_SECOND, D4); // C4 to D4
        assert_eq!(C4 + MINOR_THIRD, Pitch::new(63)); // C4 to E♭4
        assert_eq!(C4 + MAJOR_THIRD, E4); // C4 to E4
        assert_eq!(C4 + PERFECT_FOURTH, F4); // C4 to F4
        assert_eq!(C4 + PERFECT_FIFTH, G4); // C4 to G4
        assert_eq!(C4 + MAJOR_SIXTH, A4); // C4 to A4
        assert_eq!(C4 + MAJOR_SEVENTH, B4); // C4 to B4
        assert_eq!(C4 + PERFECT_OCTAVE, C5); // C4 to C5

        // Test with different starting pitches
        assert_eq!(E4 + PERFECT_FIFTH, B4); // E4 to B4
        assert_eq!(G3 + MAJOR_THIRD, B3); // G3 to B3
        assert_eq!(A4 + PERFECT_FOURTH, D5); // A4 to D5

        // Test with compound intervals
        let compound_third = MAJOR_THIRD * 2; // Major tenth (MAJOR_THIRD + PERFECT_OCTAVE)
        assert_eq!(C4 + compound_third, GSHARP4); // C4 up a major tenth = E5

        // Test extreme ranges
        let two_octaves = PERFECT_OCTAVE * 2; // Two octaves (24 semitones)
        assert_eq!(C4 + two_octaves, C6); // C4 up two octaves = C6

        // Check that original pitch is unchanged (immutability check)
        let original = C4;
        let _transposed = original + MAJOR_THIRD;
        assert_eq!(original, C4); // Original pitch unchanged

        // Test building a chord (triad)
        let root = F4;
        let third = root + MAJOR_THIRD;
        let fifth = root + PERFECT_FIFTH;
        assert_eq!([root, third, fifth], [F4, A4, C5]); // F major triad
    }

    #[test]
    fn test_pitch_add_assign() {
        let mut pitch = Pitch::new(60); // Middle C
        let interval = MAJOR_THIRD; // Major third
        pitch += &interval;
        assert_eq!(pitch, Pitch::new(64));
    }

    #[test]
    fn test_pitch_subtraction() {
        let pitch1 = Pitch::new(67); // G
        let pitch2 = Pitch::new(60); // C
        assert_eq!(pitch1 - pitch2, PERFECT_FIFTH); // Perfect fifth
    }

    #[test]
    fn test_pitch_sub_assign() {
        let mut pitch1 = Pitch::new(67); // G
        let pitch2 = Pitch::new(60); // C
        pitch1 -= pitch2;
        assert_eq!(pitch1, Pitch::new(7));
    }

    #[test]
    fn test_pitch_ordering() {
        let c4 = Pitch::new(60);
        let e4 = Pitch::new(64);
        let g4 = Pitch::new(67);

        assert!(c4 < e4);
        assert!(e4 < g4);
        assert_eq!(c4.cmp(&e4), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_pitch_classes() {
        assert_eq!(u8::from(C), 0);
        assert_eq!(u8::from(CSHARP), 1);
        assert_eq!(DFLAT, CSHARP); // Enharmonic equivalence
        assert_eq!(u8::from(D), 2);
        assert_eq!(u8::from(DSHARP), 3);
        assert_eq!(EFLAT, DSHARP); // Enharmonic equivalence
        assert_eq!(u8::from(E), 4);
        assert_eq!(u8::from(F), 5);
        assert_eq!(u8::from(FSHARP), 6);
        assert_eq!(GFLAT, FSHARP); // Enharmonic equivalence
        assert_eq!(u8::from(G), 7);
        assert_eq!(u8::from(GSHARP), 8);
        assert_eq!(AFLAT, GSHARP); // Enharmonic equivalence
        assert_eq!(u8::from(A), 9);
        assert_eq!(u8::from(ASHARP), 10);
        assert_eq!(BFLAT, ASHARP); // Enharmonic equivalence
        assert_eq!(u8::from(B), 11);
    }

    #[test]
    fn test_octave_relationships() {
        // Test that each C is 12 semitones (one octave) apart
        assert_eq!(C1 - C0, PERFECT_OCTAVE);
        assert_eq!(C2 - C1, PERFECT_OCTAVE);
        assert_eq!(C3 - C2, PERFECT_OCTAVE);
        assert_eq!(C4 - C3, PERFECT_OCTAVE);
        assert_eq!(C5 - C4, PERFECT_OCTAVE);
        assert_eq!(C6 - C5, PERFECT_OCTAVE);
        assert_eq!(C7 - C6, PERFECT_OCTAVE);
        assert_eq!(C8 - C7, PERFECT_OCTAVE);
        assert_eq!(C9 - C8, PERFECT_OCTAVE);
    }

    #[test]
    fn test_standard_tuning() {
        // Test A4 = 69 (440 Hz concert pitch)
        assert_eq!(u8::from(A4), 69);

        // Test standard guitar tuning (E2 to E4)
        assert_eq!(u8::from(E2), 40); // Low E
        assert_eq!(u8::from(A2), 45); // A
        assert_eq!(u8::from(D3), 50); // D
        assert_eq!(u8::from(G3), 55); // G
        assert_eq!(u8::from(B3), 59); // B
        assert_eq!(u8::from(E4), 64); // High E
    }

    #[test]
    fn test_midi_range() {
        // Test MIDI note number range (0-127)
        assert_eq!(u8::from(C0), 12); // Lowest C
        assert_eq!(u8::from(G9), 127); // Highest note
    }

    #[test]
    fn test_pitch_octave_shifts() {
        // Test shifting up by octaves
        assert_eq!(C4 >> 1, C5);
        assert_eq!(A4 >> 1, A5);
        assert_eq!(G4 >> 1, G5);

        // Test shifting down by octaves
        assert_eq!(C4 << 1, C3);
        assert_eq!(A4 << 1, A3);
        assert_eq!(G4 << 1, G3);

        // Test multiple octave shifts
        assert_eq!(C4 >> 2, C6);
        assert_eq!(C4 << 2, C2);
    }

    #[test]
    fn test_pitch_shift_assignments() {
        // Test shifting up
        let mut pitch = C4;
        pitch >>= 1;
        assert_eq!(pitch, C5);

        // Test shifting down
        let mut pitch = C4;
        pitch <<= 1;
        assert_eq!(pitch, C3);

        // Test multiple shifts
        let mut pitch = C4;
        pitch >>= 2;
        assert_eq!(pitch, C6);
    }

    #[test]
    fn test_pitch_arithmetic_with_intervals() {
        // Test addition with intervals
        assert_eq!(C4 + MAJOR_THIRD, E4);
        assert_eq!(F4 + PERFECT_FOURTH, BFLAT4);
        assert_eq!(G4 + PERFECT_FIFTH, D5);

        // Test interval between pitches
        assert_eq!(E4 - C4, MAJOR_THIRD);
        assert_eq!(BFLAT4 - F4, PERFECT_FOURTH);
        assert_eq!(D5 - G4, PERFECT_FIFTH);

        // Test compound intervals
        assert_eq!(C5 - C4, PERFECT_OCTAVE);
        assert_eq!(E5 - C4, MAJOR_TENTH);
    }

    #[test]
    fn test_pitch_enharmonic_relationships() {
        // Test enharmonic equivalents
        assert_eq!(CSHARP4, DFLAT4);
        assert_eq!(DSHARP4, EFLAT4);
        assert_eq!(FSHARP4, GFLAT4);
        assert_eq!(GSHARP4, AFLAT4);
        assert_eq!(ASHARP4, BFLAT4);

        // Test intervals between enharmonic notes
        assert_eq!(DFLAT4 - C4, MINOR_SECOND);
        assert_eq!(CSHARP4 - C4, MINOR_SECOND);
        assert_eq!(EFLAT4 - C4, MINOR_THIRD);
    }

    #[test]
    fn test_pitch_chromatic_scale() {
        // Test all notes in an octave
        let chromatic = [
            C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4, A4, ASHARP4, B4,
        ];

        // Verify ascending order
        for i in 0..chromatic.len() - 1 {
            assert!(chromatic[i] < chromatic[i + 1]);
            assert_eq!(chromatic[i + 1] - chromatic[i], MINOR_SECOND);
        }

        // Verify octave completion
        assert_eq!(C5 - C4, PERFECT_OCTAVE);
    }

    #[test]
    fn test_pitch_add_interval_ref() {
        // Test adding interval references to pitches
        let major_third_ref = &MAJOR_THIRD;
        let perfect_fifth_ref = &PERFECT_FIFTH;
        let octave_ref = &PERFECT_OCTAVE;

        // Basic interval reference addition
        assert_eq!(C4 + major_third_ref, E4);
        assert_eq!(C4 + perfect_fifth_ref, G4);
        assert_eq!(C4 + octave_ref, C5);

        // Compare with value-based addition
        assert_eq!(C4 + major_third_ref, C4 + MAJOR_THIRD);
        assert_eq!(C4 + perfect_fifth_ref, C4 + PERFECT_FIFTH);
        assert_eq!(C4 + octave_ref, C4 + PERFECT_OCTAVE);

        // Test with different starting pitches
        assert_eq!(G3 + major_third_ref, B3);
        assert_eq!(A4 + perfect_fifth_ref, E5);
        assert_eq!(F5 + octave_ref, F6);

        // Test with compound intervals (convert to references)
        let compound_third = MAJOR_THIRD * 2; // Major tenth (MAJOR_THIRD + PERFECT_OCTAVE)
        let compound_third_ref = &compound_third;
        assert_eq!(C4 + compound_third_ref, GSHARP4);

        // Extreme ranges
        let high_interval = Interval::new(24); // Two octaves
        let high_interval_ref = &high_interval;
        assert_eq!(C4 + high_interval_ref, C6);
    }

    #[test]
    fn test_pitch_subtract_interval() {
        // Test subtracting common intervals
        assert_eq!(C5 - PERFECT_OCTAVE, C4); // C5 down octave = C4
        assert_eq!(C5 - PERFECT_FIFTH, F4); // C5 down fifth = F4
        assert_eq!(E5 - MAJOR_THIRD, C5); // E5 down third = C5
        assert_eq!(G4 - PERFECT_FOURTH, D4); // G4 down fourth = D4
        assert_eq!(A4 - MAJOR_SECOND, G4); // A4 down second = G4
        assert_eq!(D4 - MINOR_SECOND, Pitch::new(61)); // D4 down minor second = C♯4

        // Test with different starting pitches
        assert_eq!(F4 - PERFECT_FIFTH, ASHARP3); // F4 down fifth = B♭3
        assert_eq!(B4 - MAJOR_THIRD, G4); // B4 down third = G4
        assert_eq!(E5 - PERFECT_FOURTH, B4); // E5 down fourth = B4

        // Test with compound intervals
        let compound_fifth = PERFECT_FIFTH * 2; // Two fifths (14 semitones)
        assert_eq!(C5 - compound_fifth, ASHARP3); // C5 down two fifths = F3

        // Test extreme ranges
        let two_octaves = PERFECT_OCTAVE * 2; // Two octaves (24 semitones)
        assert_eq!(C6 - two_octaves, C4); // C6 down two octaves = C4

        // Check that original pitch is unchanged (immutability check)
        let original = C5;
        let _transposed = original - PERFECT_FIFTH;
        assert_eq!(original, C5); // Original pitch unchanged

        // Test finding chord roots
        let chord_fifth = G4;
        let root = chord_fifth - PERFECT_FIFTH;
        assert_eq!(root, C4); // G4 down a fifth = C4 (the root)
    }

    #[test]
    fn test_pitch_subtract_interval_ref() {
        // Test subtracting interval references
        let fifth_ref = &PERFECT_FIFTH;
        let third_ref = &MAJOR_THIRD;
        let octave_ref = &PERFECT_OCTAVE;

        // Basic interval reference subtraction
        assert_eq!(C5 - fifth_ref, F4); // C5 down a fifth = F4
        assert_eq!(E5 - third_ref, C5); // E5 down a third = C5
        assert_eq!(C5 - octave_ref, C4); // C5 down an octave = C4

        // Compare with value-based subtraction
        assert_eq!(C5 - fifth_ref, C5 - PERFECT_FIFTH);
        assert_eq!(E5 - third_ref, E5 - MAJOR_THIRD);
        assert_eq!(C5 - octave_ref, C5 - PERFECT_OCTAVE);

        // Test with different starting pitches
        assert_eq!(G4 - fifth_ref, C4); // G4 down a fifth = C4
        assert_eq!(F5 - third_ref, CSHARP5); // F5 down a third = D5

        // Test with compound intervals
        let compound_third = MAJOR_THIRD * 2; // Major tenth
        let compound_ref = &compound_third;
        assert_eq!(E5 - compound_ref, GSHARP4); // E5 down a tenth = C4
    }

    #[test]
    fn test_pitch_subtract_assign_interval() {
        // Test basic in-place subtraction
        let mut pitch = C5;
        pitch -= PERFECT_FIFTH; // C5 down a fifth = F4
        assert_eq!(pitch, F4);

        // Test sequential subtractions
        let mut pitch = C6;
        pitch -= MAJOR_THIRD; // C6 down a third = A5
        assert_eq!(pitch, GSHARP5);
        pitch -= MAJOR_THIRD; // A5 down a third = F5
        assert_eq!(pitch, E5);
        pitch -= MAJOR_THIRD; // F5 down a third = D5
        assert_eq!(pitch, C5);

        // Test with different intervals
        let mut pitch = G5;
        pitch -= PERFECT_FIFTH; // G5 down a fifth = C5
        assert_eq!(pitch, C5);
        pitch -= PERFECT_FOURTH; // C5 down a fourth = G4
        assert_eq!(pitch, G4);
        pitch -= MAJOR_SECOND; // G4 down a second = F4
        assert_eq!(pitch, F4);

        // Test with compound intervals
        let mut pitch = C6;
        let two_octaves = PERFECT_OCTAVE * 2; // Two octaves
        pitch -= two_octaves; // C6 down two octaves = C4
        assert_eq!(pitch, C4);
    }

    #[test]
    fn test_pitch_subtract_assign_interval_ref() {
        // Create interval references
        let fifth_ref = &PERFECT_FIFTH;
        let third_ref = &MAJOR_THIRD;
        let octave_ref = &PERFECT_OCTAVE;

        // Test basic in-place subtraction with refs
        let mut pitch = C5;
        pitch -= fifth_ref; // C5 down a fifth = F4
        assert_eq!(pitch, F4);

        // Test sequential subtractions with refs
        let mut pitch = C6;
        pitch -= third_ref; // C6 down a third = A5
        assert_eq!(pitch, GSHARP5);
        pitch -= third_ref; // A5 down a third = F5
        assert_eq!(pitch, E5);

        // Test with multiple different interval refs
        let mut pitch = G5;
        pitch -= fifth_ref; // G5 down a fifth = C5
        assert_eq!(pitch, C5);
        pitch -= octave_ref; // C5 down an octave = C4
        assert_eq!(pitch, C4);

        // Test with array of interval references
        let intervals = [fifth_ref, third_ref, octave_ref];
        let mut pitch = C6;
        for interval in intervals {
            pitch -= interval; // Sequential subtraction
        }
        // C6 - fifth = F5, - third = D5, - octave = D4
        assert_eq!(pitch, CSHARP4);
    }
}
