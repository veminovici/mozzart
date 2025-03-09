use crate::Interval;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Represents a musical pitch using MIDI note numbers (0-127)
/// where middle C is 60.
///
/// # Examples
/// ```
/// use mozzart_std::Pitch;
///
/// let middle_c = Pitch::new(60);
/// let middle_a = Pitch::new(69);  // A4 = 440Hz
/// assert!(middle_a > middle_c);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pitch(u8);

impl Pitch {
    /// Creates a new pitch from a MIDI note number (0-127).
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::Pitch;
    ///
    /// let middle_c = Pitch::new(60);
    /// let high_c = Pitch::new(72);  // C5, one octave above middle C
    /// ```
    pub fn new(midi_note: u8) -> Self {
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

impl From<u8> for Pitch {
    /// Creates a pitch from a MIDI note number
    #[inline]
    fn from(value: u8) -> Self {
        Pitch(value)
    }
}

impl Add<Interval> for Pitch {
    type Output = Pitch;

    /// Adds an interval to a pitch, returning a new pitch
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, Pitch};
    ///
    /// let c4 = Pitch::from(60u8);  // Middle C
    /// let major_third = Interval::from(4u8);
    /// assert_eq!(c4 + major_third, Pitch::from(64u8));  // E4
    /// ```
    #[inline]
    fn add(self, interval: Interval) -> Self::Output {
        let pitch = self.0 + u8::from(interval);
        Pitch(pitch)
    }
}

impl AddAssign<&Interval> for Pitch {
    /// Adds an interval to a pitch in place
    #[inline]
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
    /// use mozzart_std::{Interval, Pitch};
    ///
    /// let g4 = Pitch::from(67u8);  // G4
    /// let c4 = Pitch::from(60u8);  // C4
    /// assert_eq!(g4 - c4, Interval::from(7u8));  // Perfect fifth
    /// ```
    #[inline]
    fn sub(self, other: Pitch) -> Self::Output {
        Interval(self.0 - other.0)
    }
}

impl SubAssign<Pitch> for Pitch {
    /// Subtracts a pitch from another pitch in place
    #[inline]
    fn sub_assign(&mut self, other: Pitch) {
        self.0 -= other.0;
    }
}

/// A slice of pitches that can be converted into intervals
pub struct Pitches<'a>(&'a [Pitch]);

impl<'a> Pitches<'a> {
    /// Creates a new `Pitches` from a slice of pitches
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Pitch, Pitches};
    ///
    /// let c_major = [Pitch::new(60), Pitch::new(64), Pitch::new(67)];  // C-E-G
    /// let pitches = Pitches::new(&c_major);
    /// ```
    #[inline]
    pub const fn new(pitches: &'a [Pitch]) -> Self {
        Self(pitches)
    }

    /// Returns the first pitch in the sequence
    ///
    /// # Panics
    /// Panics if the sequence is empty
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Pitch, Pitches};
    ///
    /// let c_major = [Pitch::new(60), Pitch::new(64), Pitch::new(67)];  // C-E-G
    /// let pitches = Pitches::new(&c_major);
    /// assert_eq!(pitches.root(), Pitch::new(60));  // C4
    /// ```
    #[inline]
    pub const fn root(&self) -> Pitch {
        self.0[0]
    }

    /// Converts a sequence of pitches into the intervals between consecutive pitches
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, Pitches, Pitch};
    /// let c_major = [Pitch::from(60u8), Pitch::from(64u8), Pitch::from(67u8)];  // C-E-G
    /// let intervals = Pitches::new(&c_major).into_intervals();
    /// assert_eq!(intervals, vec![Interval::from(4u8), Interval::from(3u8)]);  // Major third, minor third
    /// ```
    pub fn into_intervals(self) -> Vec<Interval> {
        self.0
            .windows(2)
            .map(|pitches| pitches[1] - pitches[0])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(Pitch::from(60u8), pitch);
    }

    #[test]
    fn test_pitch_add_interval() {
        let pitch = Pitch::new(60); // Middle C
        let interval = Interval::new(4); // Major third
        assert_eq!(pitch + interval, Pitch::new(64));
    }

    #[test]
    fn test_pitch_add_assign() {
        let mut pitch = Pitch::new(60); // Middle C
        let interval = Interval::new(4); // Major third
        pitch += &interval;
        assert_eq!(pitch, Pitch::new(64));
    }

    #[test]
    fn test_pitch_subtraction() {
        let pitch1 = Pitch::new(67); // G
        let pitch2 = Pitch::new(60); // C
        assert_eq!(pitch1 - pitch2, Interval::new(7)); // Perfect fifth
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
    fn test_pitches_into_intervals() {
        let pitches = [Pitch::new(60), Pitch::new(64), Pitch::new(67)]; // C-E-G
        let intervals = Pitches::new(&pitches).into_intervals();
        assert_eq!(intervals, vec![Interval::new(4), Interval::new(3)]); // Major third, minor third
    }

    #[test]
    fn test_pitches_into_intervals_empty() {
        let pitches: [Pitch; 0] = [];
        let intervals = Pitches::new(&pitches).into_intervals();
        assert!(intervals.is_empty());
    }

    #[test]
    fn test_pitches_into_intervals_single_pitch() {
        let pitches = [Pitch::new(60)];
        let intervals = Pitches::new(&pitches).into_intervals();
        assert!(intervals.is_empty());
    }

    #[test]
    fn test_pitch_octave_arithmetic() {
        let c4 = Pitch::new(60);
        let c5 = Pitch::new(72);
        assert_eq!(c5 - c4, Interval::new(12)); // One octave
    }

    #[test]
    fn test_pitches_root() {
        let pitches = [Pitch::new(60), Pitch::new(64), Pitch::new(67)]; // C-E-G
        assert_eq!(Pitches::new(&pitches).root(), Pitch::new(60)); // C4
    }

    #[test]
    #[should_panic]
    fn test_pitches_root_empty() {
        let pitches: [Pitch; 0] = [];
        let _ = Pitches::new(&pitches).root(); // Should panic
    }
}
