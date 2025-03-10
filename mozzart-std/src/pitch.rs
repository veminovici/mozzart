use crate::Interval;
use crate::interval::PERFECT_OCTAVE;
use std::fmt;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
};

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
    pub const fn new(midi_note: u8) -> Self {
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

/// Pitch class constants (octave-independent)
pub const C: Pitch = Pitch::new(0);
pub const CSHARP: Pitch = Pitch::new(1);
pub const DFLAT: Pitch = CSHARP;
pub const D: Pitch = Pitch::new(2);
pub const DSHARP: Pitch = Pitch::new(3);
pub const EFLAT: Pitch = DSHARP;
pub const E: Pitch = Pitch::new(4);
pub const F: Pitch = Pitch::new(5);
pub const FSHARP: Pitch = Pitch::new(6);
pub const GFLAT: Pitch = FSHARP;
pub const G: Pitch = Pitch::new(7);
pub const GSHARP: Pitch = Pitch::new(8);
pub const AFLAT: Pitch = GSHARP;
pub const A: Pitch = Pitch::new(9);
pub const ASHARP: Pitch = Pitch::new(10);
pub const BFLAT: Pitch = ASHARP;
pub const B: Pitch = Pitch::new(11);

/// Pitches in octave 0 (16.35 Hz to 30.87 Hz)
pub const C0: Pitch = Pitch::new(12);
pub const CSHARP0: Pitch = Pitch::new(13);
pub const DFLAT0: Pitch = CSHARP0;
pub const D0: Pitch = Pitch::new(14);
pub const DSHARP0: Pitch = Pitch::new(15);
pub const EFLAT0: Pitch = DSHARP0;
pub const E0: Pitch = Pitch::new(16);
pub const F0: Pitch = Pitch::new(17);
pub const FSHARP0: Pitch = Pitch::new(18);
pub const GFLAT0: Pitch = FSHARP0;
pub const G0: Pitch = Pitch::new(19);
pub const GSHARP0: Pitch = Pitch::new(20);
pub const AFLAT0: Pitch = GSHARP0;
pub const A0: Pitch = Pitch::new(21);
pub const ASHARP0: Pitch = Pitch::new(22);
pub const BFLAT0: Pitch = ASHARP0;
pub const B0: Pitch = Pitch::new(23);

/// Pitches in octave 1 (32.70 Hz to 61.74 Hz)
pub const C1: Pitch = Pitch::new(24);
pub const CSHARP1: Pitch = Pitch::new(25);
pub const DFLAT1: Pitch = CSHARP1;
pub const D1: Pitch = Pitch::new(26);
pub const DSHARP1: Pitch = Pitch::new(27);
pub const EFLAT1: Pitch = DSHARP1;
pub const E1: Pitch = Pitch::new(28);
pub const F1: Pitch = Pitch::new(29);
pub const FSHARP1: Pitch = Pitch::new(30);
pub const GFLAT1: Pitch = FSHARP1;
pub const G1: Pitch = Pitch::new(31);
pub const GSHARP1: Pitch = Pitch::new(32);
pub const AFLAT1: Pitch = GSHARP1;
pub const A1: Pitch = Pitch::new(33);
pub const ASHARP1: Pitch = Pitch::new(34);
pub const BFLAT1: Pitch = ASHARP1;
pub const B1: Pitch = Pitch::new(35);

/// Pitches in octave 2 (65.41 Hz to 123.47 Hz)
pub const C2: Pitch = Pitch::new(36);
pub const CSHARP2: Pitch = Pitch::new(37);
pub const DFLAT2: Pitch = CSHARP2;
pub const D2: Pitch = Pitch::new(38);
pub const DSHARP2: Pitch = Pitch::new(39);
pub const EFLAT2: Pitch = DSHARP2;
pub const E2: Pitch = Pitch::new(40);
pub const F2: Pitch = Pitch::new(41);
pub const FSHARP2: Pitch = Pitch::new(42);
pub const GFLAT2: Pitch = FSHARP2;
pub const G2: Pitch = Pitch::new(43);
pub const GSHARP2: Pitch = Pitch::new(44);
pub const AFLAT2: Pitch = GSHARP2;
pub const A2: Pitch = Pitch::new(45);
pub const ASHARP2: Pitch = Pitch::new(46);
pub const BFLAT2: Pitch = ASHARP2;
pub const B2: Pitch = Pitch::new(47);

/// Pitches in octave 3 (130.81 Hz to 246.94 Hz)
pub const C3: Pitch = Pitch::new(48);
pub const CSHARP3: Pitch = Pitch::new(49);
pub const DFLAT3: Pitch = CSHARP3;
pub const D3: Pitch = Pitch::new(50);
pub const DSHARP3: Pitch = Pitch::new(51);
pub const EFLAT3: Pitch = DSHARP3;
pub const E3: Pitch = Pitch::new(52);
pub const F3: Pitch = Pitch::new(53);
pub const FSHARP3: Pitch = Pitch::new(54);
pub const GFLAT3: Pitch = FSHARP3;
pub const G3: Pitch = Pitch::new(55);
pub const GSHARP3: Pitch = Pitch::new(56);
pub const AFLAT3: Pitch = GSHARP3;
pub const A3: Pitch = Pitch::new(57);
pub const ASHARP3: Pitch = Pitch::new(58);
pub const BFLAT3: Pitch = ASHARP3;
pub const B3: Pitch = Pitch::new(59);

/// Pitches in octave 4 (261.63 Hz to 493.88 Hz)
/// This octave contains middle C (C4 = 261.63 Hz)
pub const C4: Pitch = Pitch::new(60);
pub const CSHARP4: Pitch = Pitch::new(61);
pub const DFLAT4: Pitch = CSHARP4;
pub const D4: Pitch = Pitch::new(62);
pub const DSHARP4: Pitch = Pitch::new(63);
pub const EFLAT4: Pitch = DSHARP4;
pub const E4: Pitch = Pitch::new(64);
pub const F4: Pitch = Pitch::new(65);
pub const FSHARP4: Pitch = Pitch::new(66);
pub const GFLAT4: Pitch = FSHARP4;
pub const G4: Pitch = Pitch::new(67);
pub const GSHARP4: Pitch = Pitch::new(68);
pub const AFLAT4: Pitch = GSHARP4;
pub const A4: Pitch = Pitch::new(69);
pub const ASHARP4: Pitch = Pitch::new(70);
pub const BFLAT4: Pitch = ASHARP4;
pub const B4: Pitch = Pitch::new(71);

/// Pitches in octave 5 (523.25 Hz to 987.77 Hz)
pub const C5: Pitch = Pitch::new(72);
pub const CSHARP5: Pitch = Pitch::new(73);
pub const DFLAT5: Pitch = CSHARP5;
pub const D5: Pitch = Pitch::new(74);
pub const DSHARP5: Pitch = Pitch::new(75);
pub const EFLAT5: Pitch = DSHARP5;
pub const E5: Pitch = Pitch::new(76);
pub const F5: Pitch = Pitch::new(77);
pub const FSHARP5: Pitch = Pitch::new(78);
pub const GFLAT5: Pitch = FSHARP5;
pub const G5: Pitch = Pitch::new(79);
pub const GSHARP5: Pitch = Pitch::new(80);
pub const AFLAT5: Pitch = GSHARP5;
pub const A5: Pitch = Pitch::new(81);
pub const ASHARP5: Pitch = Pitch::new(82);
pub const BFLAT5: Pitch = ASHARP5;
pub const B5: Pitch = Pitch::new(83);

/// Pitches in octave 6 (1046.50 Hz to 1975.53 Hz)
pub const C6: Pitch = Pitch::new(84);
pub const CSHARP6: Pitch = Pitch::new(85);
pub const DFLAT6: Pitch = CSHARP6;
pub const D6: Pitch = Pitch::new(86);
pub const DSHARP6: Pitch = Pitch::new(87);
pub const EFLAT6: Pitch = DSHARP6;
pub const E6: Pitch = Pitch::new(88);
pub const F6: Pitch = Pitch::new(89);
pub const FSHARP6: Pitch = Pitch::new(90);
pub const GFLAT6: Pitch = FSHARP6;
pub const G6: Pitch = Pitch::new(91);
pub const GSHARP6: Pitch = Pitch::new(92);
pub const AFLAT6: Pitch = GSHARP6;
pub const A6: Pitch = Pitch::new(93);
pub const ASHARP6: Pitch = Pitch::new(94);
pub const BFLAT6: Pitch = ASHARP6;
pub const B6: Pitch = Pitch::new(95);

/// Pitches in octave 7 (2093.00 Hz to 3951.07 Hz)
pub const C7: Pitch = Pitch::new(96);
pub const CSHARP7: Pitch = Pitch::new(97);
pub const DFLAT7: Pitch = CSHARP7;
pub const D7: Pitch = Pitch::new(98);
pub const DSHARP7: Pitch = Pitch::new(99);
pub const EFLAT7: Pitch = DSHARP7;
pub const E7: Pitch = Pitch::new(100);
pub const F7: Pitch = Pitch::new(101);
pub const FSHARP7: Pitch = Pitch::new(102);
pub const GFLAT7: Pitch = FSHARP7;
pub const G7: Pitch = Pitch::new(103);
pub const GSHARP7: Pitch = Pitch::new(104);
pub const AFLAT7: Pitch = GSHARP7;
pub const A7: Pitch = Pitch::new(105);
pub const ASHARP7: Pitch = Pitch::new(106);
pub const BFLAT7: Pitch = ASHARP7;
pub const B7: Pitch = Pitch::new(107);

/// Pitches in octave 8 (4186.01 Hz to 7902.13 Hz)
pub const C8: Pitch = Pitch::new(108);
pub const CSHARP8: Pitch = Pitch::new(109);
pub const DFLAT8: Pitch = CSHARP8;
pub const D8: Pitch = Pitch::new(110);
pub const DSHARP8: Pitch = Pitch::new(111);
pub const EFLAT8: Pitch = DSHARP8;
pub const E8: Pitch = Pitch::new(112);
pub const F8: Pitch = Pitch::new(113);
pub const FSHARP8: Pitch = Pitch::new(114);
pub const GFLAT8: Pitch = FSHARP8;
pub const G8: Pitch = Pitch::new(115);
pub const GSHARP8: Pitch = Pitch::new(116);
pub const AFLAT8: Pitch = GSHARP8;
pub const A8: Pitch = Pitch::new(117);
pub const ASHARP8: Pitch = Pitch::new(118);
pub const BFLAT8: Pitch = ASHARP8;
pub const B8: Pitch = Pitch::new(119);

/// Pitches in octave 9 (8372.02 Hz to 15804.27 Hz)
pub const C9: Pitch = Pitch::new(120);
pub const CSHARP9: Pitch = Pitch::new(121);
pub const DFLAT9: Pitch = CSHARP9;
pub const D9: Pitch = Pitch::new(122);
pub const DSHARP9: Pitch = Pitch::new(123);
pub const EFLAT9: Pitch = DSHARP9;
pub const E9: Pitch = Pitch::new(124);
pub const F9: Pitch = Pitch::new(125);
pub const FSHARP9: Pitch = Pitch::new(126);
pub const GFLAT9: Pitch = FSHARP9;
pub const G9: Pitch = Pitch::new(127);

/// A slice of pitches that can be converted into intervals
#[derive(Clone, Copy, PartialEq, Eq)]
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

/// Formats the pitches as a comma-separated list within `Pitches([...])`
///
/// # Examples
/// ```
/// use mozzart_std::{Pitch, Pitches};
///
/// let c_major = [Pitch::new(60), Pitch::new(64), Pitch::new(67)];  // C-E-G
/// let pitches = Pitches::new(&c_major);
/// assert_eq!(format!("{:?}", pitches), "Pitches([Pitch(60), Pitch(64), Pitch(67)])");
/// ```
impl Debug for Pitches<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pitches = self
            .0
            .iter()
            .map(|p| format!("{:?}", p))
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "Pitches([{pitches}])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interval::PERFECT_OCTAVE;

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

    #[test]
    fn test_pitches_debug() {
        let pitches = [Pitch::new(60), Pitch::new(64), Pitch::new(67)]; // C-E-G
        let formatted = format!("{:?}", Pitches::new(&pitches));
        assert_eq!(formatted, "Pitches([Pitch(60), Pitch(64), Pitch(67)])");
    }

    #[test]
    fn test_pitches_debug_empty() {
        let pitches: [Pitch; 0] = [];
        let formatted = format!("{:?}", Pitches::new(&pitches));
        assert_eq!(formatted, "Pitches([])");
    }

    #[test]
    fn test_pitches_debug_single() {
        let pitches = [Pitch::new(60)]; // C4
        let formatted = format!("{:?}", Pitches::new(&pitches));
        assert_eq!(formatted, "Pitches([Pitch(60)])");
    }

    #[test]
    fn test_pitch_classes() {
        assert_eq!(u8::from(C), 0);
        assert_eq!(u8::from(CSHARP), 1);
        assert_eq!(DFLAT, CSHARP);  // Enharmonic equivalence
        assert_eq!(u8::from(D), 2);
        assert_eq!(u8::from(DSHARP), 3);
        assert_eq!(EFLAT, DSHARP);  // Enharmonic equivalence
        assert_eq!(u8::from(E), 4);
        assert_eq!(u8::from(F), 5);
        assert_eq!(u8::from(FSHARP), 6);
        assert_eq!(GFLAT, FSHARP);  // Enharmonic equivalence
        assert_eq!(u8::from(G), 7);
        assert_eq!(u8::from(GSHARP), 8);
        assert_eq!(AFLAT, GSHARP);  // Enharmonic equivalence
        assert_eq!(u8::from(A), 9);
        assert_eq!(u8::from(ASHARP), 10);
        assert_eq!(BFLAT, ASHARP);  // Enharmonic equivalence
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
        assert_eq!(u8::from(E2), 40);  // Low E
        assert_eq!(u8::from(A2), 45);  // A
        assert_eq!(u8::from(D3), 50);  // D
        assert_eq!(u8::from(G3), 55);  // G
        assert_eq!(u8::from(B3), 59);  // B
        assert_eq!(u8::from(E4), 64);  // High E
    }

    #[test]
    fn test_midi_range() {
        // Test MIDI note number range (0-127)
        assert_eq!(u8::from(C0), 12);   // Lowest C
        assert_eq!(u8::from(G9), 127);  // Highest note
    }
}
