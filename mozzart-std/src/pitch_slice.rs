use crate::{Interval, Pitch};
use std::fmt;

/// A slice of pitches that can be converted into intervals
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PitchSlice<'a>(&'a [Pitch]);

impl<'a> PitchSlice<'a> {
    /// Creates a new `PitchSlice` from a slice of pitches
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Pitch, PitchSlice, C4, E4, G4};
    ///
    /// let c_major = [C4, E4, G4];  // C-E-G
    /// let pitches = PitchSlice::new(&c_major);
    /// ```
    #[inline(always)]
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
    /// use mozzart_std::{Pitch, PitchSlice, C4, E4, G4};
    ///
    /// let c_major = [C4, E4, G4];  // C-E-G
    /// let pitches = PitchSlice::new(&c_major);
    /// assert_eq!(pitches.root(), C4);
    /// ```
    #[inline(always)]
    pub const fn root(&self) -> Pitch {
        self.0[0]
    }

    /// Converts a sequence of pitches into the intervals between consecutive pitches
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Interval, PitchSlice, Pitch, C4, E4, G4, MAJOR_THIRD, MINOR_THIRD};
    ///
    /// let c_major = [C4, E4, G4];  // C-E-G
    /// let intervals = PitchSlice::new(&c_major).into_intervals();
    /// assert_eq!(intervals, vec![MAJOR_THIRD, MINOR_THIRD]);
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
/// use mozzart_std::{Pitch, PitchSlice, C4, E4, G4};
///
/// let c_major = [C4, E4, G4];  // C-E-G
/// let pitches = PitchSlice::new(&c_major);
/// assert_eq!(format!("{:?}", pitches), "Pitches([Pitch(60), Pitch(64), Pitch(67)])");
/// ```
impl fmt::Debug for PitchSlice<'_> {
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
    use crate::{C4, C5, E4, G4, MAJOR_THIRD, MINOR_THIRD, PERFECT_OCTAVE};

    #[test]
    fn test_pitches_into_intervals() {
        let pitches = [C4, E4, G4]; // C-E-G
        let intervals = PitchSlice::new(&pitches).into_intervals();
        assert_eq!(intervals, vec![MAJOR_THIRD, MINOR_THIRD]); // Major third, minor third
    }

    #[test]
    fn test_pitches_into_intervals_empty() {
        let pitches: [Pitch; 0] = [];
        let intervals = PitchSlice::new(&pitches).into_intervals();
        assert!(intervals.is_empty());
    }

    #[test]
    fn test_pitches_into_intervals_single_pitch() {
        let pitches = [C4];
        let intervals = PitchSlice::new(&pitches).into_intervals();
        assert!(intervals.is_empty());
    }

    #[test]
    fn test_pitch_octave_arithmetic() {
        let c4 = C4;
        let c5 = C5;
        assert_eq!(c5 - c4, PERFECT_OCTAVE); // One octave
    }

    #[test]
    fn test_pitches_root() {
        let pitches = [C4, E4, G4]; // C-E-G
        assert_eq!(PitchSlice::new(&pitches).root(), C4); // C4
    }

    #[test]
    #[should_panic]
    fn test_pitches_root_empty() {
        let pitches: [Pitch; 0] = [];
        let _ = PitchSlice::new(&pitches).root(); // Should panic
    }

    #[test]
    fn test_pitches_debug() {
        let pitches = [C4, E4, G4]; // C-E-G
        let formatted = format!("{:?}", PitchSlice::new(&pitches));
        assert_eq!(formatted, "Pitches([Pitch(60), Pitch(64), Pitch(67)])");
    }

    #[test]
    fn test_pitches_debug_empty() {
        let pitches: [Pitch; 0] = [];
        let formatted = format!("{:?}", PitchSlice::new(&pitches));
        assert_eq!(formatted, "Pitches([])");
    }

    #[test]
    fn test_pitches_debug_single() {
        let pitches = [C4]; // C4
        let formatted = format!("{:?}", PitchSlice::new(&pitches));
        assert_eq!(formatted, "Pitches([Pitch(60)])");
    }
}
