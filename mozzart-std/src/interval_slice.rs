use crate::{Interval, NamedList, Pitch};
use std::fmt;

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
    /// use mozzart_std::{Interval, Intervals, Pitch, C4, E4, G4, MAJOR_THIRD, MINOR_THIRD};
    ///
    /// let c4 = C4;  // Middle C
    /// let major_triad = [MAJOR_THIRD, MINOR_THIRD];  // Major third, minor third
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

impl AsRef<[Interval]> for Intervals<'_> {
    /// Returns a reference to the underlying slice of intervals
    ///
    /// This implementation allows `Intervals` to be used in contexts that expect
    /// a reference to a slice of intervals.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval,Intervals, MAJOR_THIRD, MINOR_THIRD};
    ///
    /// let major_triad = [MAJOR_THIRD, MINOR_THIRD];  // Major third, minor third
    /// let intervals = Intervals::new(&major_triad);
    /// let slice: &[Interval] = intervals.as_ref();
    /// assert_eq!(slice, &[MAJOR_THIRD, MINOR_THIRD]);
    /// ```
    fn as_ref(&self) -> &[Interval] {
        self.0
    }
}

impl<'a> From<&'a [Interval]> for Intervals<'a> {
    /// Creates an `Intervals` from a slice of intervals
    ///
    /// This implementation allows for ergonomic conversion from an interval slice
    /// using `into()` or `from()`.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Intervals, MAJOR_THIRD, MINOR_THIRD};
    ///
    /// let major_triad = [MAJOR_THIRD, MINOR_THIRD];  // Major third, minor third
    /// let intervals: Intervals = (&major_triad[..]).into();
    /// assert_eq!(intervals.as_ref(), &[MAJOR_THIRD, MINOR_THIRD]);
    /// ```
    fn from(intervals: &'a [Interval]) -> Self {
        Intervals(intervals)
    }
}

/// Formats the intervals as a comma-separated list within `Intervals([...])`
///
/// # Examples
/// ```
/// use mozzart_std::{Intervals, MAJOR_THIRD, MINOR_THIRD};
///
/// let major_triad = [MAJOR_THIRD, MINOR_THIRD];  // Major third, minor third
/// let intervals = Intervals::new(&major_triad);
/// assert_eq!(format!("{:?}", intervals), "Intervals[Interval(4), Interval(3)]");
/// ```
impl fmt::Debug for Intervals<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let intervals = NamedList::new("Intervals", self.0);
        write!(f, "{intervals:?}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{Interval, Intervals, C4, C5, E4, G4, MAJOR_THIRD, MINOR_THIRD, PERFECT_FOURTH};

    #[test]
    fn test_intervals_compound_triad() {
        let c4 = C4;
        let intervals = [
            MAJOR_THIRD,    // Major third
            MINOR_THIRD,    // Minor third
            PERFECT_FOURTH, // Perfect fourth
        ];
        let pitches = Intervals::new(&intervals).into_pitches(c4);
        assert_eq!(pitches, vec![C4, E4, G4, C5,]);
    }

    #[test]
    fn test_intervals_into_pitches() {
        let c4 = C4;
        let intervals = [MAJOR_THIRD, MINOR_THIRD]; // Major third, minor third
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
    fn test_intervals_as_ref() {
        let intervals = [MAJOR_THIRD, MINOR_THIRD]; // Major third, minor third
        let interval_slice = Intervals::new(&intervals);
        let slice: &[Interval] = interval_slice.as_ref();
        assert_eq!(slice, &intervals);
    }

    #[test]
    fn test_intervals_as_ref_empty() {
        let intervals: [Interval; 0] = [];
        let interval_slice = Intervals::new(&intervals);
        let slice: &[Interval] = interval_slice.as_ref();
        assert!(slice.is_empty());
    }

    #[test]
    fn test_intervals_as_ref_single() {
        let intervals = [MAJOR_THIRD];
        let interval_slice = Intervals::new(&intervals);
        let slice: &[Interval] = interval_slice.as_ref();
        assert_eq!(slice, &[MAJOR_THIRD]);
    }

    #[test]
    fn test_intervals_from() {
        let intervals = [MAJOR_THIRD, MINOR_THIRD]; // Major third, minor third
        let interval_slice = Intervals::from(&intervals[..]);
        assert_eq!(interval_slice.as_ref(), &intervals[..]);
    }

    #[test]
    fn test_intervals_from_empty() {
        let intervals: [Interval; 0] = [];
        let interval_slice = Intervals::from(&intervals[..]);
        assert!(interval_slice.as_ref().is_empty());
    }

    #[test]
    fn test_intervals_into() {
        let intervals = [MAJOR_THIRD, MINOR_THIRD]; // Major third, minor third
        let interval_slice: Intervals = (&intervals[..]).into();
        assert_eq!(interval_slice.as_ref(), &intervals[..]);
    }

    #[test]
    fn test_intervals_debug() {
        let intervals = [MAJOR_THIRD, MINOR_THIRD]; // Major third, minor third
        let formatted = format!("{:?}", Intervals::new(&intervals));
        assert_eq!(formatted, "Intervals[Interval(4), Interval(3)]");
    }

    #[test]
    fn test_intervals_debug_empty() {
        let intervals: [Interval; 0] = [];
        let formatted = format!("{:?}", Intervals::new(&intervals));
        assert_eq!(formatted, "Intervals[]");
    }

    #[test]
    fn test_intervals_debug_single() {
        let intervals = [MAJOR_THIRD];
        let formatted = format!("{:?}", Intervals::new(&intervals));
        assert_eq!(formatted, "Intervals[Interval(4)]");
    }
}
