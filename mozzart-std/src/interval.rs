use std::ops::{Mul, MulAssign, Shr, ShrAssign};

/// Represents a musical interval as a number of semitones.
///
/// An interval is the distance between two pitches, measured in semitones.
/// Musical intervals are fundamental to understanding scales, chords, and melody.
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
/// Intervals beyond one octave:
/// - Ninth (M9): Major second + octave (14 semitones)
/// - Tenth (M10): Major third + octave (16 semitones)
/// - Eleventh (P11): Perfect fourth + octave (17 semitones)
/// - Twelfth (P12): Perfect fifth + octave (19 semitones)
/// - Thirteenth (M13): Major sixth + octave (21 semitones)
/// - Double octave: Two octaves (24 semitones)
///
/// # Octave Operations
/// Intervals can be shifted up or down by octaves using the shift operators:
/// - Right shift (`>>`) moves an interval up by octaves
/// - Left shift (`<<`) moves an interval down by octaves
///
/// # Examples
/// ```
/// use mozzart_std::{Interval, MAJOR_THIRD, MINOR_THIRD, PERFECT_FIFTH};
///
/// // Compare intervals
/// assert!(MAJOR_THIRD > MINOR_THIRD);
///
/// // Multiply intervals
/// let octave = PERFECT_FIFTH * 2;  // A fifth times 2 is greater than an octave
///
/// // Shift intervals by octaves
/// let compound_third = MAJOR_THIRD >> 1;  // Major third up one octave
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval(pub(crate) u8);

impl Interval {
    /// Creates a new interval from a number of semitones.
    ///
    /// # Arguments
    /// * `semitones` - The number of semitones in the interval
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, PERFECT_FIFTH, PERFECT_OCTAVE};
    ///
    /// let perfect_fifth = PERFECT_FIFTH;  // 7 semitones
    /// let octave = PERFECT_OCTAVE;       // 12 semitones
    /// assert!(octave > perfect_fifth);
    /// ```
    #[inline(always)]
    pub(crate) const fn new(semitones: u8) -> Self {
        Interval(semitones)
    }

    /// Returns the number of semitones in the interval.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, MAJOR_THIRD};
    ///
    /// let major_third = MAJOR_THIRD;
    /// assert_eq!(u8::from(major_third), 4);  // Major third is 4 semitones
    /// ```
    #[inline(always)]
    pub const fn semitones(&self) -> u8 {
        self.0
    }
}

impl From<Interval> for u8 {
    /// Converts an interval to its semitone count
    #[inline(always)]
    fn from(interval: Interval) -> Self {
        interval.0
    }
}

impl From<&Interval> for u8 {
    /// Converts a reference to an interval to its semitone count
    #[inline(always)]
    fn from(interval: &Interval) -> Self {
        interval.0
    }
}

impl Mul<u8> for Interval {
    type Output = Interval;

    /// Multiplies an interval by a scalar value.
    ///
    /// This is useful for:
    /// - Creating compound intervals (e.g., perfect fifth × 2)
    /// - Building extended intervals from basic ones
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{PERFECT_FIFTH, PERFECT_OCTAVE};
    ///
    /// let compound_fifth = PERFECT_FIFTH * 2;  // Perfect twelfth (19 semitones)
    /// assert!(compound_fifth > PERFECT_OCTAVE);
    /// ```
    fn mul(self, rhs: u8) -> Self::Output {
        Interval::new(self.0 * rhs)
    }
}

impl MulAssign<u8> for Interval {
    /// Multiplies an interval in place by a scalar value.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, PERFECT_FIFTH};
    ///
    /// let mut interval = PERFECT_FIFTH;
    /// interval *= 2;  // Convert to a perfect twelfth
    /// assert_eq!(interval.semitones(), 14);
    /// ```
    fn mul_assign(&mut self, rhs: u8) {
        self.0 *= rhs;
    }
}

/// Number of semitones in an octave
const SEMITONES_PER_OCTAVE: u8 = 12;

impl Shr<u8> for Interval {
    type Output = Interval;

    /// Shifts an interval up by the specified number of octaves.
    ///
    /// This operation adds octaves to the interval, creating compound intervals.
    /// For example, shifting a major third up one octave creates a major tenth.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{MAJOR_THIRD, MAJOR_TENTH};
    ///
    /// let compound = MAJOR_THIRD >> 1;  // Shift up one octave
    /// assert_eq!(compound, MAJOR_TENTH);
    /// ```
    fn shr(self, shift: u8) -> Self::Output {
        Interval::new(self.0 + (shift * SEMITONES_PER_OCTAVE))
    }
}

impl ShrAssign<u8> for Interval {
    /// Shifts an interval up by the specified number of octaves in place.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, MAJOR_THIRD, MAJOR_TENTH};
    ///
    /// let mut interval = MAJOR_THIRD;
    /// interval >>= 1;  // Shift up one octave
    /// assert_eq!(interval, MAJOR_TENTH);
    /// ```
    fn shr_assign(&mut self, shift: u8) {
        self.0 += shift * SEMITONES_PER_OCTAVE;
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
        compound >>= 2;  // Up two octaves
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
