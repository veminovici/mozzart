use crate::constants::SEMITONES_IN_OCTAVE;
use crate::Step;

/// Represents a musical interval measured in semitones
///
/// An interval is the distance between two pitches, measured in semitones.
/// In Western music with equal temperament, semitones are the smallest unit of
/// pitch distinction.
///
/// Common intervals include:
/// - Unison (0 semitones)
/// - Minor second (1 semitone)
/// - Major second (2 semitones)
/// - Perfect fifth (7 semitones)
/// - Octave (12 semitones)
///
/// The `Interval` struct provides a type-safe way to represent these musical
/// distances and perform operations with them.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interval(u8);

impl Interval {
    /// Creates a new `Interval` from the specified number of semitones
    ///
    /// # Arguments
    /// * `semitones` - The number of semitones in the interval
    ///
    /// # Returns
    /// A new `Interval` instance
    ///
    /// # Examples
    /// ```ignore
    /// // Creating common intervals (typically done via constants):
    /// let perfect_fifth = Interval::new(7);
    /// let octave = Interval::new(12);
    /// ```
    #[inline]
    pub(crate) const fn new(semitones: u8) -> Self {
        Self(semitones)
    }

    /// Creates an `Interval` representing a number of octaves
    ///
    /// Each octave in Western equal temperament consists of 12 semitones.
    ///
    /// # Arguments
    /// * `octave` - The number of octaves
    ///
    /// # Returns
    /// An `Interval` representing the specified number of octaves
    ///
    /// # Examples
    /// ```ignore
    /// // Creating intervals of one, two, and three octaves:
    /// let octave = Interval::from_octave(1);     // 12 semitones
    /// let two_octaves = Interval::from_octave(2); // 24 semitones
    /// ```
    #[inline]
    pub(crate) const fn from_octave(octave: u8) -> Self {
        let semitones: u8 = octave * SEMITONES_IN_OCTAVE;
        Self(semitones)
    }

    /// Returns the number of semitones in this interval
    ///
    /// # Returns
    /// The number of semitones
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::constants::*;
    ///
    /// let perfect_fifth = PERFECT_FIFTH;
    /// assert_eq!(perfect_fifth.semitones(), 7);
    /// ```
    #[inline]
    pub fn semitones(&self) -> u8 {
        self.0
    }
}

/// Conversion from `Interval` to `u8` (number of semitones)
///
/// This allows extracting the raw semitone count from an interval.
impl From<Interval> for u8 {
    #[inline]
    fn from(interval: Interval) -> Self {
        interval.0
    }
}

/// Conversion from a reference to `Interval` to `u8` (number of semitones)
///
/// This allows extracting the raw semitone count without consuming the interval.
impl From<&Interval> for u8 {
    #[inline]
    fn from(interval: &Interval) -> Self {
        interval.0
    }
}

impl From<Step> for Interval {
    #[inline]
    fn from(step: Step) -> Self {
        Interval::new(step.semitones())
    }
}

impl From<&Step> for Interval {
    #[inline]
    fn from(step: &Step) -> Self {
        Interval::new(step.semitones())
    }
}
