use crate::Pitch;

/// Represents a musical interval as a number of semitones.
///
/// Common intervals:
/// - Minor second: 1 semitone
/// - Major second: 2 semitones
/// - Minor third: 3 semitones
/// - Major third: 4 semitones
/// - Perfect fourth: 5 semitones
/// - Perfect fifth: 7 semitones
/// - Octave: 12 semitones
///
/// # Examples
/// ```
/// use mozzart_std::{Interval, MAJOR_THIRD, MINOR_THIRD};
///
/// let major_third = MAJOR_THIRD;
/// let minor_third = MINOR_THIRD;
/// assert!(major_third > minor_third);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval(pub(crate) u8);

impl Interval {
    /// Creates a new interval from a number of semitones.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{PERFECT_FIFTH, PERFECT_OCTAVE};
    ///
    /// let perfect_fifth = PERFECT_FIFTH;
    /// let octave = PERFECT_OCTAVE;
    /// ```
    #[inline(always)]
    const fn new(semitones: u8) -> Self {
        Interval(semitones)
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

impl From<u8> for Interval {
    /// Creates an interval from a semitone count
    #[inline(always)]
    fn from(value: u8) -> Self {
        Interval(value)
    }
}

/// Basic intervals
pub const SEMITONE: Interval = Interval::new(1);
pub const TONE: Interval = Interval::new(2);

/// Standard intervals
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

/// Extended intervals
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

/// A slice of intervals that can be converted into pitches
pub struct Intervals<'a>(&'a [Interval]);

impl<'a> Intervals<'a> {
    /// Creates a new `Intervals` from a slice of intervals
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, Intervals, MAJOR_THIRD, MINOR_THIRD};
    ///
    /// let major_triad = [MAJOR_THIRD, MINOR_THIRD];  // Major third, minor third
    /// let intervals = Intervals::new(&major_triad);
    /// ```
    #[inline(always)]
    pub fn new(intervals: &'a [Interval]) -> Self {
        Intervals(intervals)
    }

    /// Converts a sequence of intervals into pitches, starting from a root pitch
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, Intervals, Pitch, C4, E4, G4};
    ///
    /// let c4 = C4;  // Middle C
    /// let major_triad = [Interval::from(4u8), Interval::from(3u8)];  // Major third, minor third
    /// let pitches = Intervals::new(&major_triad).into_pitches(c4);
    /// assert_eq!(pitches, vec![C4, E4, G4]); // C-E-G
    /// ```
    pub fn into_pitches(self, root: Pitch) -> Vec<Pitch> {
        std::iter::once(root)
            .chain(self.0.iter().scan(root, |pitch, interval| {
                *pitch += interval;
                Some(*pitch)
            }))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{C4, C5, E4, G4};

    use super::*;

    #[test]
    fn test_interval_new() {
        let interval = Interval::new(4);
        assert_eq!(u8::from(interval), 4);
    }

    #[test]
    fn test_interval_conversions() {
        let interval = Interval::new(4);
        assert_eq!(u8::from(interval), 4);
        assert_eq!(u8::from(&interval), 4);
        assert_eq!(Interval::from(4u8), interval);
    }

    #[test]
    fn test_interval_to_u8() {
        let interval = Interval::new(4); // Major third
        let value: u8 = interval.into();
        assert_eq!(value, 4);
    }

    #[test]
    fn test_u8_to_interval() {
        let value: u8 = 7; // Perfect fifth
        let interval: Interval = value.into();
        assert_eq!(interval, Interval::new(7));
    }

    #[test]
    fn test_interval_ordering() {
        let minor_third = Interval::new(3);
        let major_third = Interval::new(4);
        let perfect_fifth = Interval::new(7);

        assert!(minor_third < major_third);
        assert!(major_third < perfect_fifth);
        assert_eq!(minor_third.cmp(&major_third), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_intervals_into_pitches() {
        let c4 = C4;
        let intervals = [Interval::new(4), Interval::new(3)]; // Major third, minor third
        let pitches = Intervals::new(&intervals).into_pitches(c4);
        assert_eq!(pitches, [C4, E4, G4]); // C-E-G
    }

    #[test]
    fn test_intervals_into_pitches_empty() {
        let c4 = C4;
        let intervals: [Interval; 0] = [];
        let pitches = Intervals::new(&intervals).into_pitches(c4);
        assert_eq!(pitches, vec![C4]); // Just the root
    }

    #[test]
    fn test_common_intervals() {
        let minor_second = Interval::new(1);
        let major_second = Interval::new(2);
        let minor_third = Interval::new(3);
        let major_third = Interval::new(4);
        let perfect_fourth = Interval::new(5);
        let perfect_fifth = Interval::new(7);
        let octave = Interval::new(12);

        assert!(minor_second < major_second);
        assert!(major_second < minor_third);
        assert!(minor_third < major_third);
        assert!(major_third < perfect_fourth);
        assert!(perfect_fourth < perfect_fifth);
        assert!(perfect_fifth < octave);
    }

    #[test]
    fn test_intervals_compound_triad() {
        let c4 = C4;
        let intervals = [
            Interval::new(4), // Major third
            Interval::new(3), // Minor third
            Interval::new(5), // Perfect fourth
        ];
        let pitches = Intervals::new(&intervals).into_pitches(c4);
        assert_eq!(pitches, vec![C4, E4, G4, C5,]);
    }

    #[test]
    fn test_standard_intervals() {
        assert_eq!(UNISON.0, 0);
        assert_eq!(MINOR_SECOND.0, 1);
        assert_eq!(MAJOR_SECOND.0, 2);
        assert_eq!(MINOR_THIRD.0, 3);
        assert_eq!(MAJOR_THIRD.0, 4);
        assert_eq!(PERFECT_FOURTH.0, 5);
        assert_eq!(AUGMENTED_FOURTH.0, 6);
        assert_eq!(DIMINISHED_FIFTH.0, 6);
        assert_eq!(PERFECT_FIFTH.0, 7);
        assert_eq!(MINOR_SIXTH.0, 8);
        assert_eq!(MAJOR_SIXTH.0, 9);
        assert_eq!(MINOR_SEVENTH.0, 10);
        assert_eq!(MAJOR_SEVENTH.0, 11);
        assert_eq!(PERFECT_OCTAVE.0, 12);
    }

    #[test]
    fn test_extended_intervals() {
        assert_eq!(MINOR_NINTH.0, 13);
        assert_eq!(MAJOR_NINTH.0, 14);
        assert_eq!(MINOR_TENTH.0, 15);
        assert_eq!(MAJOR_TENTH.0, 16);
        assert_eq!(PERFECT_ELEVENTH.0, 17);
        assert_eq!(AUGMENTED_ELEVENTH.0, 18);
        assert_eq!(PERFECT_TWELFTH.0, 19);
        assert_eq!(MINOR_THIRTEENTH.0, 20);
        assert_eq!(MAJOR_THIRTEENTH.0, 21);
        assert_eq!(MINOR_FOURTEENTH.0, 22);
        assert_eq!(MAJOR_FOURTEENTH.0, 23);
        assert_eq!(DOUBLE_OCTAVE.0, 24);
    }

    #[test]
    fn test_tritone_equivalence() {
        assert_eq!(AUGMENTED_FOURTH, DIMINISHED_FIFTH);
    }
}
