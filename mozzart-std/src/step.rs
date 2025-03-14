use crate::Interval;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Step(u8);

impl Step {
    /// Creates a new `Step` from the specified number of semitones
    ///
    /// # Arguments
    /// * `semitones` - The number of semitones in the step
    ///
    /// # Returns
    /// A new `Step` instance
    ///
    /// # Examples
    /// ```ignore
    /// // Creating common step (typically done via constants):
    /// let semitone = Step::new(1);
    /// let octave = Step::new(12);
    /// ```
    #[inline]
    pub(crate) const fn new(semitones: u8) -> Self {
        Self(semitones)
    }

    /// Returns the number of semitones in this step
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

/// Conversion from `Step` to `u8` (number of semitones)
///
/// This allows extracting the raw semitone count from an step.
impl From<Step> for u8 {
    #[inline]
    fn from(step: Step) -> Self {
        step.0
    }
}

/// Conversion from a reference to `Step` to `u8` (number of semitones)
///
/// This allows extracting the raw semitone count without consuming the step.
impl From<&Step> for u8 {
    #[inline]
    fn from(step: &Step) -> Self {
        step.0
    }
}

/// Conversion from `Interval` to `Step`
///
/// This allows converting an interval to a step.
///
/// # Examples
/// ```
/// use mozzart_std::{Interval, Step};
/// use mozzart_std::constants::*;
///
/// let interval = MAJOR_THIRD;
/// let step = Step::from(interval);
/// assert_eq!(step.semitones(), 4);
/// ```
impl From<Interval> for Step {
    #[inline]
    fn from(interval: Interval) -> Self {
        Step::new(interval.semitones())
    }
}

/// Conversion from a reference to `Interval` to `Step`
///
/// This allows converting an interval to a step without consuming it.
///
/// # Examples
/// ```
/// use mozzart_std::{Interval, Step};
/// use mozzart_std::constants::*;
///
/// let interval = MAJOR_THIRD;
/// let step = Step::from(&interval);
/// assert_eq!(step.semitones(), 4);
/// ```
impl From<&Interval> for Step {
    #[inline]
    fn from(interval: &Interval) -> Self {
        Step::new(interval.semitones())
    }
}
