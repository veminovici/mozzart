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
/// use mozzart_std::Interval;
///
/// let major_third = Interval::new(4);
/// let minor_third = Interval::new(3);
/// assert!(major_third > minor_third);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval(pub(crate) u8);

impl Interval {
    /// Creates a new interval from a number of semitones.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::Interval;
    ///
    /// let perfect_fifth = Interval::new(7);
    /// let octave = Interval::new(12);
    /// ```
    pub fn new(semitones: u8) -> Self {
        Interval(semitones)
    }
}

impl From<Interval> for u8 {
    /// Converts an interval to its semitone count
    #[inline]
    fn from(interval: Interval) -> Self {
        interval.0
    }
}

impl From<&Interval> for u8 {
    /// Converts a reference to an interval to its semitone count
    #[inline]
    fn from(interval: &Interval) -> Self {
        interval.0
    }
}

impl From<u8> for Interval {
    /// Creates an interval from a semitone count
    #[inline]
    fn from(value: u8) -> Self {
        Interval(value)
    }
}

/// A slice of intervals that can be converted into pitches
pub struct Intervals<'a>(&'a [Interval]);

impl<'a> Intervals<'a> {
    /// Creates a new `Intervals` from a slice of intervals
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, Intervals};
    ///
    /// let major_triad = [Interval::new(4), Interval::new(3)];  // Major third, minor third
    /// let intervals = Intervals::new(&major_triad);
    /// ```
    pub fn new(intervals: &'a [Interval]) -> Self {
        Intervals(intervals)
    }

    /// Converts a sequence of intervals into pitches, starting from a root pitch
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, Intervals, Pitch};
    ///
    /// let c4 = Pitch::from(60u8);  // Middle C
    /// let major_triad = [Interval::from(4u8), Interval::from(3u8)];  // Major third, minor third
    /// let pitches = Intervals::new(&major_triad).into_pitches(c4);
    /// assert_eq!(pitches, vec![Pitch::from(60u8), Pitch::from(64u8), Pitch::from(67u8)]);  // C-E-G
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
        let c4 = Pitch::new(60);
        let intervals = [Interval::new(4), Interval::new(3)]; // Major third, minor third
        let pitches = Intervals::new(&intervals).into_pitches(c4);
        assert_eq!(
            pitches,
            vec![Pitch::new(60), Pitch::new(64), Pitch::new(67)]
        ); // C-E-G
    }

    #[test]
    fn test_intervals_into_pitches_empty() {
        let c4 = Pitch::new(60);
        let intervals: [Interval; 0] = [];
        let pitches = Intervals::new(&intervals).into_pitches(c4);
        assert_eq!(pitches, vec![Pitch::new(60)]); // Just the root
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
        let c4 = Pitch::new(60);
        let intervals = [
            Interval::new(4), // Major third
            Interval::new(3), // Minor third
            Interval::new(5), // Perfect fourth
        ];
        let pitches = Intervals::new(&intervals).into_pitches(c4);
        assert_eq!(
            pitches,
            vec![
                Pitch::new(60), // C4
                Pitch::new(64), // E4
                Pitch::new(67), // G4
                Pitch::new(72), // C5
            ]
        );
    }
}
