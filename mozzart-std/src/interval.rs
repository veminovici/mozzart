use crate::PERFECT_OCTAVE;
use std::ops::{Mul, MulAssign, Shr, ShrAssign};

/// Represents a musical interval as a number of semitones.
///
/// An interval is the distance between two pitches, measured in semitones (half steps).
/// Musical intervals are fundamental to understanding scales, chords, and melody construction.
///
/// # Musical Theory
/// Intervals are classified by:
/// 1. Quality (perfect, major, minor, augmented, diminished)
/// 2. Size (second, third, fourth, etc.)
/// 3. Direction (ascending or descending)
///
/// # Common Intervals
/// Basic intervals (within one octave):
/// - Unison (P1): 0 semitones
/// - Minor second (m2): 1 semitone
/// - Major second (M2): 2 semitones
/// - Minor third (m3): 3 semitones
/// - Major third (M3): 4 semitones
/// - Perfect fourth (P4): 5 semitones
/// - Tritone (A4/d5): 6 semitones
/// - Perfect fifth (P5): 7 semitones
/// - Minor sixth (m6): 8 semitones
/// - Major sixth (M6): 9 semitones
/// - Minor seventh (m7): 10 semitones
/// - Major seventh (M7): 11 semitones
/// - Perfect octave (P8): 12 semitones
///
/// # Extended Intervals
/// Intervals beyond one octave (compound intervals):
/// - Ninth (M9): Major second + octave (14 semitones)
/// - Tenth (M10): Major third + octave (16 semitones)
/// - Eleventh (P11): Perfect fourth + octave (17 semitones)
/// - Twelfth (P12): Perfect fifth + octave (19 semitones)
/// - Thirteenth (M13): Major sixth + octave (21 semitones)
/// - Double octave: Two octaves (24 semitones)
///
/// # Operations
/// Intervals support several operations:
/// - Comparison: Intervals can be compared using standard comparison operators
/// - Multiplication: Intervals can be multiplied by a scalar to create compound intervals
/// - Octave shifts: Intervals can be shifted up or down by octaves using shift operators
///
/// # Examples
/// ```
/// use mozzart_std::{Interval, MAJOR_THIRD, MINOR_THIRD, PERFECT_FIFTH};
///
/// // Compare intervals
/// assert!(MAJOR_THIRD > MINOR_THIRD);
///
/// // Create compound intervals through multiplication
/// let octave = PERFECT_FIFTH * 2;  // A fifth times 2 is greater than an octave
///
/// // Shift intervals by octaves
/// let compound_third = MAJOR_THIRD >> 1;  // Major third up one octave
/// ```
///
/// # Musical Context
/// Intervals are essential for:
/// - Scale construction (defining the steps between notes)
/// - Chord building (stacking thirds or other intervals)
/// - Melody analysis (understanding melodic leaps and steps)
/// - Harmony (vertical relationships between notes)
/// - Voice leading (smooth connection between chords)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval(pub(crate) u8);

impl Interval {
    /// Creates a new interval from a number of semitones.
    ///
    /// The interval is represented internally as the number of semitones between
    /// two pitches. This constructor allows creating custom intervals beyond the
    /// standard ones provided as constants.
    ///
    /// # Arguments
    /// * `semitones` - The number of semitones in the interval (0-255)
    ///
    /// # Examples
    /// ```ignore
    /// use mozzart_std::{Interval, PERFECT_FIFTH, PERFECT_OCTAVE};
    ///
    /// let perfect_fifth = PERFECT_FIFTH;  // 7 semitones
    /// let octave = PERFECT_OCTAVE;       // 12 semitones
    /// assert!(octave > perfect_fifth);
    ///
    /// // Create a custom interval (e.g., compound perfect fifth)
    /// let compound_fifth = Interval::new(19);  // Perfect fifth plus octave
    /// assert!(compound_fifth > octave);
    /// ```
    ///
    /// # Musical Context
    /// - 0 semitones: Perfect unison (same note)
    /// - 1-11 semitones: Various intervals within an octave
    /// - 12 semitones: Perfect octave
    /// - >12 semitones: Compound intervals (spanning more than one octave)
    #[inline(always)]
    pub(crate) const fn new(semitones: u8) -> Self {
        Interval(semitones)
    }

    /// Returns the number of semitones in the interval.
    ///
    /// This method provides access to the raw semitone count that defines the interval.
    /// Useful for interval arithmetic and comparison operations.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, MAJOR_THIRD};
    ///
    /// let major_third = MAJOR_THIRD;
    /// assert_eq!(major_third.semitones(), 4);  // Major third is 4 semitones
    ///
    /// // Use semitones for interval arithmetic
    /// let is_consonant = major_third.semitones() == 4 || major_third.semitones() == 7;
    /// assert!(is_consonant);  // Major thirds are consonant intervals
    /// ```
    ///
    /// # Musical Context
    /// The semitone count is fundamental to:
    /// - Determining interval quality (major, minor, perfect)
    /// - Calculating compound intervals
    /// - Analyzing consonance and dissonance
    /// - Transposition operations
    #[inline(always)]
    pub const fn semitones(&self) -> u8 {
        self.0
    }
}

/// Converts an interval to its semitone count.
///
/// This implementation allows seamless conversion from an `Interval`
/// to its underlying semitone value, enabling direct arithmetic operations.
///
/// # Examples
/// ```
/// use mozzart_std::{Interval, MAJOR_THIRD};
///
/// let semitones: u8 = MAJOR_THIRD.into();
/// assert_eq!(semitones, 4);  // Major third is 4 semitones
/// ```
impl From<Interval> for u8 {
    #[inline(always)]
    fn from(interval: Interval) -> Self {
        interval.0
    }
}

/// Converts a reference to an interval to its semitone count.
///
/// This implementation allows converting an interval reference to its
/// semitone value without taking ownership, useful in borrowed contexts.
///
/// # Examples
/// ```
/// use mozzart_std::{Interval, MAJOR_THIRD};
///
/// let major_third = &MAJOR_THIRD;
/// let semitones: u8 = major_third.into();
/// assert_eq!(semitones, 4);
/// ```
impl From<&Interval> for u8 {
    #[inline(always)]
    fn from(interval: &Interval) -> Self {
        interval.0
    }
}

/// Implements multiplication of an interval by a scalar value.
///
/// This operation is useful for:
/// - Creating compound intervals (e.g., perfect fifth × 2)
/// - Building extended intervals from basic ones
/// - Generating harmonic series intervals
///
/// # Examples
/// ```
/// use mozzart_std::{PERFECT_FIFTH, PERFECT_OCTAVE};
///
/// // Create a compound fifth (perfect twelfth)
/// let compound_fifth = PERFECT_FIFTH * 2;  // 7 semitones * 2 = 14 semitones
/// assert!(compound_fifth > PERFECT_OCTAVE);
///
/// // Create a double octave
/// let double_octave = PERFECT_OCTAVE * 2;  // 12 semitones * 2 = 24 semitones
/// ```
///
/// # Musical Context
/// Multiplication is commonly used for:
/// - Building compound intervals (intervals larger than an octave)
/// - Creating parallel harmonies (stacking similar intervals)
/// - Analyzing harmonic series relationships
impl Mul<u8> for Interval {
    type Output = Interval;

    fn mul(self, rhs: u8) -> Self::Output {
        Interval::new(self.0 * rhs)
    }
}

/// Implements in-place multiplication of an interval by a scalar value.
///
/// This operation modifies the interval in place, multiplying its semitone count
/// by the given scalar value. Useful for building compound intervals efficiently.
///
/// # Examples
/// ```
/// use mozzart_std::{Interval, PERFECT_FIFTH};
///
/// let mut interval = PERFECT_FIFTH;  // 7 semitones
/// interval *= 2;  // Convert to a perfect twelfth (14 semitones)
/// assert_eq!(interval.semitones(), 14);
///
/// // Create a triple fifth
/// interval *= 3;  // Now 42 semitones
/// ```
///
/// # Musical Context
/// In-place multiplication is useful for:
/// - Progressive interval expansion in voice leading
/// - Building complex harmonic structures
/// - Creating parallel harmony movements
impl MulAssign<u8> for Interval {
    fn mul_assign(&mut self, rhs: u8) {
        self.0 *= rhs;
    }
}

/// Implements right shift operator for intervals, shifting up by octaves.
///
/// Each octave shift adds 12 semitones to the interval, creating compound intervals.
/// This is particularly useful for voice leading and harmonic expansion.
///
/// # Examples
/// ```
/// use mozzart_std::{MAJOR_THIRD, MAJOR_TENTH};
///
/// // Create a major tenth (major third + octave)
/// let compound = MAJOR_THIRD >> 1;  // Shift up one octave
/// assert_eq!(compound, MAJOR_TENTH);
///
/// // Create a double compound third (major third + two octaves)
/// let double_compound = MAJOR_THIRD >> 2;
/// assert_eq!(double_compound.semitones(), 28);  // 4 + (12 * 2)
/// ```
///
/// # Musical Context
/// Octave shifts are essential for:
/// - Voice leading in different registers
/// - Creating compound intervals
/// - Arranging harmonies across multiple octaves
/// - Building extended chord voicings
impl Shr<u8> for Interval {
    type Output = Interval;

    fn shr(self, shift: u8) -> Self::Output {
        Interval::new(self.0 + (shift * PERFECT_OCTAVE.0))
    }
}

/// Implements in-place right shift for intervals, shifting up by octaves.
///
/// Modifies the interval in place by adding octaves, useful for progressive
/// voice leading and harmonic expansion in a single voice.
///
/// # Examples
/// ```
/// use mozzart_std::{Interval, MAJOR_THIRD, MAJOR_TENTH};
///
/// let mut interval = MAJOR_THIRD;
/// interval >>= 1;  // Shift up one octave
/// assert_eq!(interval, MAJOR_TENTH);
///
/// // Shift up another octave
/// interval >>= 1;
/// assert_eq!(interval.semitones(), 28);  // 4 + (12 * 2)
/// ```
///
/// # Musical Context
/// In-place octave shifts are useful for:
/// - Progressive voice leading
/// - Building ascending harmonic sequences
/// - Creating registral expansion effects
impl ShrAssign<u8> for Interval {
    fn shr_assign(&mut self, shift: u8) {
        self.0 += shift * PERFECT_OCTAVE.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn test_interval_new() {
        let interval = Interval::new(4);
        assert_eq!(u8::from(interval), 4);
        assert_eq!(interval.semitones(), 4);
    }

    #[test]
    fn test_interval_conversions() {
        let interval = Interval::new(4);
        assert_eq!(u8::from(interval), 4);
        assert_eq!(u8::from(&interval), 4);
        assert_eq!(MAJOR_THIRD, interval);
    }

    #[test]
    fn test_interval_multiplication() {
        let fifth = PERFECT_FIFTH;
        let compound_fifth = fifth * 2;
        assert_eq!(compound_fifth.semitones(), 14);

        let mut mutable_fifth = PERFECT_FIFTH;
        mutable_fifth *= 2;
        assert_eq!(mutable_fifth.semitones(), 14);
    }

    #[test]
    fn test_interval_ordering() {
        // Test basic interval relationships
        assert!(MINOR_SECOND < MAJOR_SECOND);
        assert!(MAJOR_SECOND < MINOR_THIRD);
        assert!(MINOR_THIRD < MAJOR_THIRD);
        assert!(MAJOR_THIRD < PERFECT_FOURTH);
        assert!(PERFECT_FOURTH < PERFECT_FIFTH);
        assert!(PERFECT_FIFTH < PERFECT_OCTAVE);

        // Test compound intervals
        let compound_fifth = PERFECT_FIFTH * 2;
        assert!(PERFECT_OCTAVE < compound_fifth);
    }

    #[test]
    fn test_standard_intervals() {
        // Test all standard intervals within an octave
        assert_eq!(UNISON.semitones(), 0);
        assert_eq!(MINOR_SECOND.semitones(), 1);
        assert_eq!(MAJOR_SECOND.semitones(), 2);
        assert_eq!(MINOR_THIRD.semitones(), 3);
        assert_eq!(MAJOR_THIRD.semitones(), 4);
        assert_eq!(PERFECT_FOURTH.semitones(), 5);
        assert_eq!(AUGMENTED_FOURTH.semitones(), 6);
        assert_eq!(DIMINISHED_FIFTH.semitones(), 6);
        assert_eq!(PERFECT_FIFTH.semitones(), 7);
        assert_eq!(MINOR_SIXTH.semitones(), 8);
        assert_eq!(MAJOR_SIXTH.semitones(), 9);
        assert_eq!(MINOR_SEVENTH.semitones(), 10);
        assert_eq!(MAJOR_SEVENTH.semitones(), 11);
        assert_eq!(PERFECT_OCTAVE.semitones(), 12);
    }

    #[test]
    fn test_extended_intervals() {
        // Test intervals beyond one octave
        assert_eq!(MINOR_NINTH.semitones(), 13);
        assert_eq!(MAJOR_NINTH.semitones(), 14);
        assert_eq!(MINOR_TENTH.semitones(), 15);
        assert_eq!(MAJOR_TENTH.semitones(), 16);
        assert_eq!(PERFECT_ELEVENTH.semitones(), 17);
        assert_eq!(AUGMENTED_ELEVENTH.semitones(), 18);
        assert_eq!(PERFECT_TWELFTH.semitones(), 19);
        assert_eq!(MINOR_THIRTEENTH.semitones(), 20);
        assert_eq!(MAJOR_THIRTEENTH.semitones(), 21);
        assert_eq!(MINOR_FOURTEENTH.semitones(), 22);
        assert_eq!(MAJOR_FOURTEENTH.semitones(), 23);
        assert_eq!(DOUBLE_OCTAVE.semitones(), 24);
    }

    #[test]
    fn test_interval_relationships() {
        // Test enharmonic equivalents
        assert_eq!(AUGMENTED_FOURTH, DIMINISHED_FIFTH); // Tritone

        // Test compound intervals
        assert_eq!(PERFECT_OCTAVE * 2, DOUBLE_OCTAVE);
        assert_eq!(MAJOR_SECOND * 7, MAJOR_NINTH);
        assert_eq!(MAJOR_THIRD * 5, MINOR_THIRTEENTH);
    }

    #[test]
    fn test_interval_arithmetic() {
        let mut interval = PERFECT_FIFTH;

        // Test multiplication
        assert_eq!(interval * 2, MAJOR_NINTH);

        // Test multiplication assignment
        interval *= 2;
        assert_eq!(interval, MAJOR_NINTH);

        // Test compound intervals
        assert_eq!(PERFECT_FOURTH * 3, MINOR_TENTH);
        assert_eq!(MAJOR_THIRD * 4, MAJOR_TENTH);
    }

    #[test]
    fn test_interval_octave_shifts() {
        // Test shifting up by octaves
        assert_eq!(MAJOR_THIRD >> 1, MAJOR_TENTH);
        assert_eq!(PERFECT_FIFTH >> 1, PERFECT_TWELFTH);
        assert_eq!(MAJOR_SIXTH >> 1, MAJOR_THIRTEENTH);

        // // Test multiple octave shifts
        assert_eq!(MAJOR_THIRD >> 2, MAJOR_TENTH >> 1);
    }

    #[test]
    fn test_interval_shift_assignments() {
        // Test shifting up
        let mut interval = MAJOR_THIRD;
        interval >>= 1;
        assert_eq!(interval, MAJOR_TENTH);

        // Test multiple shifts
        let mut compound = PERFECT_FIFTH;
        compound >>= 2; // Up two octaves
        assert_eq!(compound.semitones(), PERFECT_FIFTH.semitones() + 24);
    }

    #[test]
    fn test_compound_interval_relationships() {
        // Test relationships between simple and compound intervals
        assert_eq!(MAJOR_SECOND >> 1, MAJOR_NINTH);
        assert_eq!(MAJOR_THIRD >> 1, MAJOR_TENTH);
        assert_eq!(PERFECT_FOURTH >> 1, PERFECT_ELEVENTH);
        assert_eq!(PERFECT_FIFTH >> 1, PERFECT_TWELFTH);
        assert_eq!(MAJOR_SIXTH >> 1, MAJOR_THIRTEENTH);
        assert_eq!(MAJOR_SEVENTH >> 1, MAJOR_FOURTEENTH);
        assert_eq!(PERFECT_OCTAVE >> 1, DOUBLE_OCTAVE >> 0);
    }
}
