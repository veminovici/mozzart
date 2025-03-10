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
}
