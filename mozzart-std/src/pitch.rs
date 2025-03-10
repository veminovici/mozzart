use crate::Interval;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

/// Represents a musical pitch using MIDI note numbers (0-127)
/// where middle C is 60.
///
/// # Examples
/// ```
/// use mozzart_std::{C4, A4};
///
/// let middle_c = C4;
/// let middle_a = A4;  // A4 = 440Hz
/// assert!(middle_a > middle_c);
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

    /// Adds an interval to a pitch, returning a new pitch
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C4, E4, MAJOR_THIRD};
    ///
    /// assert_eq!(C4 + MAJOR_THIRD, E4);
    /// ```
    #[inline(always)]
    fn add(self, interval: Interval) -> Self::Output {
        let pitch = self.0 + u8::from(interval);
        Pitch(pitch)
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
        let pitch = Pitch::new(60); // Middle C
        let interval = MAJOR_THIRD; // Major third
        assert_eq!(pitch + interval, Pitch::new(64));
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
}
