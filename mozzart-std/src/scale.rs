//! Musical scales and their qualities
//!
//! This module provides types for working with musical scales, including:
//! - Common scale qualities (major, minor, harmonic minor, melodic minor)
//! - Scale construction and manipulation
//! - Access to scale degrees and pitches

use crate::Pitch;

/// Represents the quality (or type) of a musical scale.
///
/// Each quality defines a specific pattern of intervals that characterizes the scale:
/// - `Major`: The standard major scale (W-W-H-W-W-W-H)
/// - `Minor`: The natural minor scale (W-H-W-W-H-W-W)
/// - `HarmonicMinor`: Minor scale with raised 7th (W-H-W-W-H-W+H-H)
/// - `MelodicMinor`: Minor scale with raised 6th and 7th ascending (W-H-W-W-W-W-H)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScaleQuality {
    /// The major scale, characterized by its bright sound
    Major,
    /// The natural minor scale, also known as the Aeolian mode
    Minor,
    /// The harmonic minor scale, featuring an augmented second between ♭6 and ♮7
    Harmonic,
    /// The melodic minor scale, traditionally different ascending and descending
    Melodic,
}

/// A musical scale consisting of N ordered pitches with a specific quality.
///
/// The scale is defined by:
/// - A quality (major, minor, etc.)
/// - An array of N pitches representing the scale degrees
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, ScaleQuality, C4, C4_MAJOR_SCALE};
///
/// // Create a C major scale
/// let c_major = C4_MAJOR_SCALE;
///
/// assert_eq!(c_major.quality(), ScaleQuality::Major);
/// assert_eq!(c_major.pitches()[0], C4); // Root note
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Scale<const N: usize> {
    quality: ScaleQuality,
    pitches: [Pitch; N],
}

impl<const N: usize> Scale<N> {
    /// Creates a new scale with the specified quality and pitches.
    ///
    /// # Arguments
    /// * `quality` - The scale quality (major, minor, etc.)
    /// * `pitches` - An array of N pitches representing the scale degrees
    ///
    /// # Examples
    /// ```ignore
    /// use mozzart_std::{Scale, ScaleQuality, C4_MAJOR_SCALE};
    ///
    /// let c_major = C4_MAJOR_SCALE;
    /// assert_eq!(c_major.quality(), ScaleQuality::Major);
    /// ```
    pub(crate) const fn new(quality: ScaleQuality, pitches: [Pitch; N]) -> Self {
        Self { quality, pitches }
    }

    /// Creates a new major scale from the given pitches.
    ///
    /// A major scale follows the whole-whole-half-whole-whole-whole-half step pattern (W-W-H-W-W-W-H).
    ///
    /// # Arguments
    /// * `pitches` - An array of N pitches representing the scale degrees
    ///
    /// # Examples
    /// ```ignore
    /// use mozzart_std::{Scale, ScaleQuality, C4, D4, E4, F4, G4, A4, B4, C5};
    ///
    /// let c_major = Scale::major([C4, D4, E4, F4, G4, A4, B4, C5]);
    /// assert_eq!(c_major.quality(), ScaleQuality::Major);
    /// assert_eq!(c_major.root(), C4);
    /// ```
    pub(crate) const fn major(pitches: [Pitch; N]) -> Self {
        Self::new(ScaleQuality::Major, pitches)
    }

    /// Creates a new natural minor scale from the given pitches.
    ///
    /// A natural minor scale follows the whole-half-whole-whole-half-whole-whole step pattern
    /// (W-H-W-W-H-W-W). Also known as the Aeolian mode.
    ///
    /// # Arguments
    /// * `pitches` - An array of N pitches representing the scale degrees
    ///
    /// # Examples
    /// ```ignore
    /// use mozzart_std::{Scale, ScaleQuality, A4, B4, C5, D5, E5, F5, G5, A5};
    ///
    /// let a_minor = Scale::minor([A4, B4, C5, D5, E5, F5, G5, A5]);
    /// assert_eq!(a_minor.quality(), ScaleQuality::Minor);
    /// assert_eq!(a_minor.root(), A4);
    /// ```
    pub(crate) const fn minor(pitches: [Pitch; N]) -> Self {
        Self::new(ScaleQuality::Minor, pitches)
    }

    /// Creates a new harmonic minor scale from the given pitches.
    ///
    /// A harmonic minor scale is like the natural minor scale but with a raised 7th degree.
    /// It follows the whole-half-whole-whole-half-whole+half-half step pattern (W-H-W-W-H-W+H-H),
    /// creating a characteristic augmented second interval between ♭6 and ♮7.
    ///
    /// # Arguments
    /// * `pitches` - An array of N pitches representing the scale degrees
    ///
    /// # Examples
    /// ```ignore
    /// use mozzart_std::{Scale, ScaleQuality, A4, B4, C5, D5, E5, F5, G5, A5};
    ///
    /// let a_harmonic = Scale::harmonic([A4, B4, C5, D5, E5, F5, G5, A5]);
    /// assert_eq!(a_harmonic.quality(), ScaleQuality::Harmonic);
    /// assert_eq!(a_harmonic.root(), A4);
    /// ```
    pub(crate) const fn harmonic(pitches: [Pitch; N]) -> Self {
        Self::new(ScaleQuality::Harmonic, pitches)
    }

    /// Creates a new melodic minor scale from the given pitches.
    ///
    /// A melodic minor scale traditionally has different ascending and descending forms:
    /// - Ascending: whole-half-whole-whole-whole-whole-half (W-H-W-W-W-W-H)
    /// - Descending: same as natural minor
    ///
    /// This implementation represents the ascending form, which raises both the 6th and 7th degrees.
    ///
    /// # Arguments
    /// * `pitches` - An array of N pitches representing the scale degrees
    ///
    /// # Examples
    /// ```ignore
    /// use mozzart_std::{Scale, ScaleQuality, A4, B4, C5, D5, E5, F5, G5, A5};
    ///
    /// let a_melodic = Scale::melodic([A4, B4, C5, D5, E5, F5, G5, A5]);
    /// assert_eq!(a_melodic.quality(), ScaleQuality::Melodic);
    /// assert_eq!(a_melodic.root(), A4);
    /// ```
    pub(crate) const fn melodic(pitches: [Pitch; N]) -> Self {
        Self::new(ScaleQuality::Melodic, pitches)
    }

    /// Returns the quality of the scale.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Scale, ScaleQuality, C4_MAJOR_SCALE};
    ///
    /// let scale = C4_MAJOR_SCALE;
    /// assert_eq!(scale.quality(), ScaleQuality::Major);
    /// ```
    pub fn quality(&self) -> ScaleQuality {
        self.quality
    }

    /// Returns the root pitch of the scale.
    ///
    /// The root pitch is the first note of the scale, which defines its key.
    /// For example, in C major scale, C is the root pitch.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C4_MAJOR_SCALE, C4};
    ///
    /// let scale = C4_MAJOR_SCALE;
    /// assert_eq!(scale.root(), C4);  // C4 is the root of C major scale
    /// ```
    pub fn root(&self) -> Pitch {
        self.pitches[0]
    }

    /// Returns a reference to the array of pitches in the scale.
    ///
    /// The pitches are ordered from lowest to highest, starting with the root note.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Scale, ScaleQuality, C4_MAJOR_SCALE, C4, G4};
    ///
    /// let scale = C4_MAJOR_SCALE;
    /// assert_eq!(scale.pitches()[0], C4); // Root note
    /// assert_eq!(scale.pitches()[4], G4); // Fifth scale degree
    /// ```
    pub fn pitches(&self) -> &[Pitch; N] {
        &self.pitches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn test_major_scale() {
        let c_major = Scale::<7>::new(ScaleQuality::Major, [C4, D4, E4, F4, G4, A4, B4]);
        assert_eq!(c_major.quality(), ScaleQuality::Major);
        assert_eq!(c_major.pitches()[0], C4);
        assert_eq!(c_major.pitches()[4], G4);
    }

    #[test]
    fn test_minor_scale() {
        let a_minor = Scale::<7>::new(ScaleQuality::Minor, [A4, B4, C5, D5, E5, F5, G5]);
        assert_eq!(a_minor.quality(), ScaleQuality::Minor);
        assert_eq!(a_minor.pitches()[0], A4);
        assert_eq!(a_minor.pitches()[2], C5);
    }

    #[test]
    fn test_scale_quality() {
        // Test all scale qualities using basic pitch constants
        let major = Scale::<7>::new(ScaleQuality::Major, [C4, D4, E4, F4, G4, A4, B4]);
        assert_eq!(major.quality(), ScaleQuality::Major);

        let minor = Scale::<7>::new(ScaleQuality::Minor, [A4, B4, C5, D5, E5, F5, G5]);
        assert_eq!(minor.quality(), ScaleQuality::Minor);

        let harmonic = Scale::<7>::new(
            ScaleQuality::Harmonic,
            [A4, B4, C5, D5, E5, F5, G5], // Using natural notes for simplicity
        );
        assert_eq!(harmonic.quality(), ScaleQuality::Harmonic);

        let melodic = Scale::<7>::new(
            ScaleQuality::Melodic,
            [A4, B4, C5, D5, E5, F5, G5], // Using natural notes for simplicity
        );
        assert_eq!(melodic.quality(), ScaleQuality::Melodic);
    }

    #[test]
    fn test_scale_root() {
        // Test root of major scale
        let c_major = Scale::<7>::new(ScaleQuality::Major, [C4, D4, E4, F4, G4, A4, B4]);
        assert_eq!(c_major.root(), C4);

        // Test root of minor scale
        let a_minor = Scale::<7>::new(ScaleQuality::Minor, [A4, B4, C5, D5, E5, F5, G5]);
        assert_eq!(a_minor.root(), A4);

        // Test that root is always first pitch regardless of scale quality
        let harmonic_minor = Scale::<7>::new(ScaleQuality::Harmonic, [E4, F4, G4, A4, B4, C5, D5]);
        assert_eq!(harmonic_minor.root(), E4);
    }

    #[test]
    fn test_scale_constructors() {
        // Test major scale constructor
        let c_major = Scale::<8>::major([C4, D4, E4, F4, G4, A4, B4, C5]);
        assert_eq!(c_major.quality(), ScaleQuality::Major);
        assert_eq!(c_major.root(), C4);
        assert_eq!(c_major.pitches(), &[C4, D4, E4, F4, G4, A4, B4, C5]);

        // Test minor scale constructor
        let a_minor = Scale::<8>::minor([A4, B4, C5, D5, E5, F5, G5, A5]);
        assert_eq!(a_minor.quality(), ScaleQuality::Minor);
        assert_eq!(a_minor.root(), A4);
        assert_eq!(a_minor.pitches(), &[A4, B4, C5, D5, E5, F5, G5, A5]);

        // Test harmonic minor scale constructor
        let e_harmonic = Scale::<8>::harmonic([E4, F4, G4, A4, B4, C5, D5, E5]);
        assert_eq!(e_harmonic.quality(), ScaleQuality::Harmonic);
        assert_eq!(e_harmonic.root(), E4);
        assert_eq!(e_harmonic.pitches(), &[E4, F4, G4, A4, B4, C5, D5, E5]);

        // Test melodic minor scale constructor
        let d_melodic = Scale::<8>::melodic([D4, E4, F4, G4, A4, B4, C5, D5]);
        assert_eq!(d_melodic.quality(), ScaleQuality::Melodic);
        assert_eq!(d_melodic.root(), D4);
        assert_eq!(d_melodic.pitches(), &[D4, E4, F4, G4, A4, B4, C5, D5]);
    }

    #[test]
    fn test_scale_octave_span() {
        // Test that all scale constructors handle octave-spanning scales correctly
        let scales = [
            Scale::<8>::major([C4, D4, E4, F4, G4, A4, B4, C5]),
            Scale::<8>::minor([A4, B4, C5, D5, E5, F5, G5, A5]),
            Scale::<8>::harmonic([E4, F4, G4, A4, B4, C5, D5, E5]),
            Scale::<8>::melodic([D4, E4, F4, G4, A4, B4, C5, D5]),
        ];

        for scale in scales.iter() {
            // Verify that the scale spans an octave (each note higher than the previous)
            let pitches = scale.pitches();
            for i in 0..7 {
                assert!(pitches[i + 1] > pitches[i], "Scale notes should ascend");
            }
        }
    }
}
