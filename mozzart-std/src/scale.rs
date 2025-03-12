//! Musical scales and their qualities
//!
//! This module provides types for working with musical scales, including:
//! - Common scale qualities (major, minor, harmonic minor, melodic minor)
//! - Scale construction and manipulation
//! - Access to scale degrees and pitches

use crate::constants::*;
use crate::{Interval, Pitch, UNISON};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

/// Generates a sequence of pitches starting from a root note, following the specified interval steps.
///
/// This function creates an iterator that yields pitches of a scale, where each pitch is
/// determined by adding the corresponding interval step to the previous pitch. The first
/// yielded value is always the root note, followed by notes calculated from the interval steps.
///
/// # Arguments
/// * `root` - The starting pitch (tonic) of the scale
/// * `steps` - A slice of intervals defining the pattern between adjacent notes in the scale
///
/// # Returns
/// An iterator yielding pitches starting with the root, followed by each subsequent pitch
/// calculated by adding the interval steps to the previous pitch.
///
/// # Examples
/// ```ignore
/// use mozzart_std::{Pitch, Interval, SEMITONE, TONE, C4};
/// use mozzart_std::scale::pitches_from_steps;
///
/// // Major scale pattern: whole, whole, half, whole, whole, whole, half
/// let steps = [TONE, TONE, SEMITONE, TONE, TONE, TONE, SEMITONE];
/// let c_major_pitches: Vec<Pitch> = pitches_from_steps(C4, &steps).collect();
/// // C4, D4, E4, F4, G4, A4, B4, C5
/// ```
fn pitches_from_steps(root: Pitch, steps: &[Interval]) -> impl Iterator<Item = Pitch> + '_ {
    std::iter::once(root).chain(steps.iter().scan(root, |pitch, interval| {
        *pitch += interval;
        Some(*pitch)
    }))
}

/// Generates the sequence of pitches for a major scale starting from a given root note.
///
/// This function creates an iterator that yields the 8 pitches of a major scale (including the octave),
/// following the standard major scale pattern: Whole, Whole, Half, Whole, Whole, Whole, Half.
///
/// # Arguments
/// * `root` - The starting pitch (tonic) of the major scale
///
/// # Returns
/// An iterator yielding the 8 pitches of the major scale, starting with the root note
/// and ending with the same note an octave higher.
///
/// # Examples
/// ```
/// use mozzart_std::{Pitch, C4, D4, E4, F4, G4, A4, B4, C5, major_pitches};
///
/// let c_major: Vec<Pitch> = major_pitches(C4).collect();
/// assert_eq!(c_major, vec![C4, D4, E4, F4, G4, A4, B4, C5]);
/// ```
#[inline]
pub fn major_pitches(root: Pitch) -> impl Iterator<Item = Pitch> + 'static {
    pitches_from_steps(root, &MAJOR_SCALE_STEPS)
}

/// Generates the sequence of pitches for a natural minor scale starting from a given root note.
///
/// This function creates an iterator that yields the 8 pitches of a natural minor scale
/// (including the octave), following the minor scale pattern: Whole, Half, Whole, Whole, Half, Whole, Whole.
///
/// # Arguments
/// * `root` - The starting pitch (tonic) of the minor scale
///
/// # Returns
/// An iterator yielding the 8 pitches of the natural minor scale, starting with the root note
/// and ending with the same note an octave higher.
///
/// # Examples
/// ```
/// use mozzart_std::{Pitch, A4, B4, C5, D5, E5, F5, G5, A5, minor_pitches};
///
/// let a_minor: Vec<Pitch> = minor_pitches(A4).collect();
/// assert_eq!(a_minor, vec![A4, B4, C5, D5, E5, F5, G5, A5]);
/// ```
#[inline]
pub fn minor_pitches(root: Pitch) -> impl Iterator<Item = Pitch> + 'static {
    pitches_from_steps(root, &MINOR_SCALE_STEPS)
}

/// Generates the sequence of pitches for a harmonic minor scale starting from a given root note.
///
/// This function creates an iterator that yields the 8 pitches of a harmonic minor scale
/// (including the octave), following the harmonic minor scale pattern:
/// Whole, Half, Whole, Whole, Half, Whole+Half, Half.
///
/// The harmonic minor scale is characterized by its raised 7th degree, creating an augmented
/// second between the 6th and 7th scale degrees, which gives it its distinctive exotic sound.
///
/// # Arguments
/// * `root` - The starting pitch (tonic) of the harmonic minor scale
///
/// # Returns
/// An iterator yielding the 8 pitches of the harmonic minor scale, starting with the root note
/// and ending with the same note an octave higher.
///
/// # Examples
/// ```
/// use mozzart_std::{Pitch, E4, FSHARP4, G4, A4, B4, C5, DSHARP5, E5, harmonic_pitches};
///
/// let e_harmonic: Vec<Pitch> = harmonic_pitches(E4).collect();
/// assert_eq!(e_harmonic, vec![E4, FSHARP4, G4, A4, B4, C5, DSHARP5, E5]);
/// ```
#[inline]
pub fn harmonic_pitches(root: Pitch) -> impl Iterator<Item = Pitch> + 'static {
    pitches_from_steps(root, &HARMONIC_SCALE_STEPS)
}

/// Generates the sequence of pitches for a melodic minor scale (ascending) starting from a given root note.
///
/// This function creates an iterator that yields the 8 pitches of an ascending melodic minor scale
/// (including the octave), following the melodic minor scale pattern:
/// Whole, Half, Whole, Whole, Whole, Whole, Half.
///
/// The melodic minor scale is characterized by its raised 6th and 7th degrees when ascending,
/// which creates a smoother melodic line than the harmonic minor scale. Traditionally, the descending
/// form uses the natural minor scale, though this function returns only the ascending form.
///
/// # Arguments
/// * `root` - The starting pitch (tonic) of the melodic minor scale
///
/// # Returns
/// An iterator yielding the 8 pitches of the ascending melodic minor scale, starting with the root note
/// and ending with the same note an octave higher.
///
/// # Examples
/// ```
/// use mozzart_std::{Pitch, D4, E4, F4, G4, A4, B4, CSHARP5, D5, melodic_pitches};
///
/// let d_melodic: Vec<Pitch> = melodic_pitches(D4).collect();
/// assert_eq!(d_melodic, vec![D4, E4, F4, G4, A4, B4, CSHARP5, D5]);
/// ```
#[inline]
pub fn melodic_pitches(root: Pitch) -> impl Iterator<Item = Pitch> + 'static {
    pitches_from_steps(root, &MELODIC_SCALE_STEPS)
}

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
    /// The blues scale, characterized by its characteristic 7th degree
    Blues,
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
/// let c_major = &C4_MAJOR_SCALE;
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
    pub(crate) fn new<I>(quality: ScaleQuality, pitches: I) -> Self
    where
        I: IntoIterator<Item = Pitch>,
    {
        let mut ps = [C; N];
        for (i, p) in pitches.into_iter().take(N).enumerate() {
            ps[i] = p;
        }
        Self {
            quality,
            pitches: ps,
        }
    }

    pub fn from_steps(root: Pitch, quality: ScaleQuality, steps: &[Interval]) -> Self {
        let pitches = pitches_from_steps(root, steps);
        Self::new(quality, pitches)
    }

    /// Returns the quality of the scale.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Scale, ScaleQuality, C4_MAJOR_SCALE};
    ///
    /// let scale = &C4_MAJOR_SCALE;
    /// assert_eq!(scale.quality(), ScaleQuality::Major);
    /// ```
    #[inline]
    pub const fn quality(&self) -> ScaleQuality {
        self.quality
    }

    /// Returns the tonic pitch of the scale.
    ///
    /// The tonic pitch is the first note of the scale, which defines its key.
    /// For example, in C major scale, C is the tonic pitch.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C4_MAJOR_SCALE, C4};
    ///
    /// let scale = &C4_MAJOR_SCALE;
    /// assert_eq!(scale.tonic(), C4);  // C4 is the tonic of C major scale
    /// ```
    #[inline]
    pub const fn tonic(&self) -> Pitch {
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
    /// let scale = &C4_MAJOR_SCALE;
    /// assert_eq!(scale.pitches()[0], C4); // Root note
    /// assert_eq!(scale.pitches()[4], G4); // Fifth scale degree
    /// ```
    pub const fn pitches(&self) -> &[Pitch; N] {
        &self.pitches
    }
}

impl Scale<8> {
    /// Returns the intervals between consecutive pitches in the scale.
    ///
    /// This function calculates the interval (in semitones) between each adjacent pair
    /// of notes in the scale. For an N-note scale, this returns an array of N intervals where:
    /// - steps[0] = interval between pitches[0] and pitches[1]
    /// - steps[1] = interval between pitches[1] and pitches[2]
    /// - ...and so on
    ///
    /// This is useful for:
    /// - Analyzing the structure of scales
    /// - Identifying characteristic intervals (e.g., augmented 2nd in harmonic minor)
    /// - Comparing scale patterns
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{C4_MAJOR_SCALE, TONE, SEMITONE, MAJOR_SCALE_STEPS, UNISON};
    ///
    /// let steps = C4_MAJOR_SCALE.steps();
    /// // Major scale pattern: Whole, Whole, Half, Whole, Whole, Whole, Half
    /// assert_eq!(steps[0], TONE);      // C to D: whole step
    /// assert_eq!(steps[1], TONE);      // D to E: whole step
    /// assert_eq!(steps[2], SEMITONE);  // E to F: half step
    /// assert_eq!(steps[3], TONE);      // F to G: whole step
    /// assert_eq!(steps[4], TONE);      // G to A: whole step
    /// assert_eq!(steps[5], TONE);      // A to B: whole step
    /// assert_eq!(steps[6], SEMITONE);  // B to C: half step
    /// assert_eq!(steps, MAJOR_SCALE_STEPS);
    /// ```
    pub fn steps(&self) -> [Interval; 7] {
        let mut steps = [UNISON; 7];
        for (i, step) in steps.iter_mut().enumerate() {
            *step = self.pitches[i + 1] - self.pitches[i];
        }
        steps
    }
}

/// Implements right shift operator (`>>`) for scales, which transposes the scale up by the specified number of octaves.
///
/// Each octave shift up decreases each pitch in the scale by 12 semitones.
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, C4, C5, major_scale};
///
/// // Transpose C4 major scale up one octave to get C5 major scale
/// let c5_scale = major_scale(C4) >> 1;
/// assert_eq!(c5_scale, major_scale(C5));
/// ```
impl<const N: usize> Shr<u8> for Scale<N> {
    type Output = Self;

    fn shr(self, shift: u8) -> Self::Output {
        let pitches: [Pitch; N] = self.pitches.map(|p| p >> shift);
        Self::new(self.quality, pitches)
    }
}

/// Creates a major scale starting from the specified root pitch.
///
/// This function constructs an 8-note major scale (including the octave) with the given
/// root as the tonic, following the standard major scale pattern:
/// Whole, Whole, Half, Whole, Whole, Whole, Half.
///
/// Major scales are characterized by their bright, happy sound quality, making them
/// fundamental in Western music for expressing joy, triumph, and brightness.
///
/// # Arguments
/// * `root` - The starting pitch (tonic) of the major scale
///
/// # Returns
/// A `Scale<8>` representing the major scale, containing 8 pitches from the root to the octave above
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, ScaleQuality, C4, major_scale};
///
/// let c_major = major_scale(C4);
///
/// // Check scale quality
/// assert_eq!(c_major.quality(), ScaleQuality::Major);
///
/// // Check root note
/// assert_eq!(c_major.tonic(), C4);
/// ```
#[inline]
pub fn major_scale(root: Pitch) -> Scale<8> {
    let pitches = major_pitches(root);
    Scale::new(ScaleQuality::Major, pitches)
}

/// Creates a natural minor scale starting from the specified root pitch.
///
/// This function constructs an 8-note natural minor scale (including the octave) with the given
/// root as the tonic, following the minor scale pattern:
/// Whole, Half, Whole, Whole, Half, Whole, Whole.
///
/// Minor scales are known for their melancholic, darker sound quality, making them
/// essential in Western music for expressing sadness, introspection, and tension.
///
/// # Arguments
/// * `root` - The starting pitch (tonic) of the minor scale
///
/// # Returns
/// A `Scale<8>` representing the natural minor scale, containing 8 pitches from the root to the octave above
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, ScaleQuality, A4, minor_scale};
///
/// let a_minor = minor_scale(A4);
///
/// // Check scale quality
/// assert_eq!(a_minor.quality(), ScaleQuality::Minor);
///
/// // Check root note
/// assert_eq!(a_minor.tonic(), A4);
/// ```
#[inline]
pub fn minor_scale(root: Pitch) -> Scale<8> {
    let pitches = minor_pitches(root);
    Scale::new(ScaleQuality::Minor, pitches)
}

/// Creates a harmonic minor scale starting from the specified root pitch.
///
/// This function constructs an 8-note harmonic minor scale (including the octave) with the given
/// root as the tonic, following the harmonic minor scale pattern:
/// Whole, Half, Whole, Whole, Half, Whole+Half, Half.
///
/// The harmonic minor scale is distinguished by its raised 7th degree (compared to the natural minor),
/// creating an augmented second interval between the 6th and 7th scale degrees. This distinctive
/// interval gives harmonic minor scales their exotic, Middle Eastern sound quality that has been used
/// extensively in classical, flamenco, and metal music to create tension and drama.
///
/// # Arguments
/// * `root` - The starting pitch (tonic) of the harmonic minor scale
///
/// # Returns
/// A `Scale<8>` representing the harmonic minor scale, containing 8 pitches from the root to the octave above
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, ScaleQuality, E4, harmonic_scale};
///
/// let e_harmonic = harmonic_scale(E4);
///
/// // Check scale quality
/// assert_eq!(e_harmonic.quality(), ScaleQuality::Harmonic);
///
/// // Check root note
/// assert_eq!(e_harmonic.tonic(), E4);
/// ```
#[inline]
pub fn harmonic_scale(root: Pitch) -> Scale<8> {
    let pitches = harmonic_pitches(root);
    Scale::new(ScaleQuality::Harmonic, pitches)
}

/// Creates a melodic minor scale (ascending form) starting from the specified root pitch.
///
/// This function constructs an 8-note melodic minor scale (including the octave) with the given
/// root as the tonic, following the ascending melodic minor scale pattern:
/// Whole, Half, Whole, Whole, Whole, Whole, Half.
///
/// The melodic minor scale is characterized by both its raised 6th and 7th degrees when ascending
/// (compared to the natural minor). Traditionally, the descending form reverts to the natural minor,
/// though this function implements only the ascending form. This scale creates smoother melodic lines
/// than the harmonic minor and has become particularly important in jazz and modal composition.
///
/// # Arguments
/// * `root` - The starting pitch (tonic) of the melodic minor scale
///
/// # Returns
/// A `Scale<8>` representing the melodic minor scale (ascending form), containing 8 pitches from
/// the root to the octave above
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, ScaleQuality, D4, melodic_scale};
///
/// let d_melodic = melodic_scale(D4);
///
/// // Check scale quality
/// assert_eq!(d_melodic.quality(), ScaleQuality::Melodic);
///
/// // Check root note
/// assert_eq!(d_melodic.tonic(), D4);
/// ```
#[inline]
pub fn melodic_scale(root: Pitch) -> Scale<8> {
    let pitches = melodic_pitches(root);
    Scale::new(ScaleQuality::Melodic, pitches)
}

/// Implements right shift assignment operator (`>>=`) for scales, which transposes the scale up by the specified number of octaves in place.
///
/// Each octave shift up decreases each pitch in the scale by 12 semitones.
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, C4, C5, major_scale};
///
/// // Create a mutable C4 major scale
/// let mut scale = major_scale(C4);
///
/// // Transpose up one octave in place
/// scale >>= 1;
/// assert_eq!(scale, major_scale(C5));
/// ```
impl<const N: usize> ShrAssign<u8> for Scale<N> {
    fn shr_assign(&mut self, shift: u8) {
        self.pitches = self.pitches.map(|p| p >> shift);
    }
}

/// Implements left shift operator (`<<`) for scales, which transposes the scale down by the specified number of octaves.
///
/// Each octave shift down increases each pitch in the scale by 12 semitones.
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, major_scale, C3, C4};
///
/// // Transpose C4 major scale up one octave to get C5 major scale
/// let c3_scale = major_scale(C4) << 1;
/// assert_eq!(c3_scale, major_scale(C3));
/// ```
impl<const N: usize> Shl<u8> for Scale<N> {
    type Output = Self;

    fn shl(self, shift: u8) -> Self::Output {
        let pitches: [Pitch; N] = self.pitches.map(|p| p << shift);
        Self::new(self.quality, pitches)
    }
}

/// Implements left shift assignment operator (`<<=`) for scales, which transposes the scale down by the specified number of octaves in place.
///
/// Each octave shift udownp increases each pitch in the scale by 12 semitones.
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, major_scale, C4, C3};
///
/// // Create a mutable C4 major scale
/// let mut scale = major_scale(C4);
///
/// // Transpose up one octave in place
/// scale <<= 1;
/// assert_eq!(scale, major_scale(C3));
/// ```
impl<const N: usize> ShlAssign<u8> for Scale<N> {
    fn shl_assign(&mut self, shift: u8) {
        self.pitches = self.pitches.map(|p| p << shift);
    }
}

/// Implements right shift operator (`>>`) for scale references, which transposes the scale up by the specified number of octaves.
///
/// Each octave shift up increases each pitch in the scale by 12 semitones.
/// This implementation works on a reference to a scale and returns a new scale.
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, C4, C5, major_scale};
///
/// // Transpose C4 major scale up one octave to get C5 major scale
/// let c4_scale = major_scale(C4);
/// let c5_scale = c4_scale >> 1;
/// assert_eq!(c5_scale, major_scale(C5));
/// ```
impl<const N: usize> Shr<u8> for &Scale<N> {
    type Output = Scale<N>;

    fn shr(self, shift: u8) -> Self::Output {
        *self >> shift
    }
}

/// Implements right shift assignment operator (`>>=`) for mutable scale references, which transposes
/// the scale up by the specified number of octaves in place.
///
/// Each octave shift up increases each pitch in the scale by 12 semitones.
/// This implementation modifies the scale in place through a mutable reference.
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, C4, C5, major_scale};
///
/// // Create a mutable scale
/// let mut scale = major_scale(C4);
///
/// // Transpose up one octave in place
/// scale >>= 1;
/// assert_eq!(scale, major_scale(C5));
/// ```
impl<const N: usize> ShrAssign<u8> for &mut Scale<N> {
    fn shr_assign(&mut self, shift: u8) {
        self.pitches = self.pitches.map(|p| p >> shift);
    }
}

/// Implements left shift operator (`<<`) for scale references, which transposes the scale down by the specified number of octaves.
///
/// Each octave shift down decreases each pitch in the scale by 12 semitones.
/// This implementation works on a reference to a scale and returns a new scale.
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, C3, C4, major_scale};
///
/// // Transpose C4 major scale down one octave to get C3 major scale
/// let c4_scale = major_scale(C4);
/// let c3_scale = c4_scale << 1;
/// assert_eq!(c3_scale, major_scale(C3));
/// ```
impl<const N: usize> Shl<u8> for &Scale<N> {
    type Output = Scale<N>;

    fn shl(self, shift: u8) -> Self::Output {
        *self << shift
    }
}

/// Implements left shift assignment operator (`<<=`) for mutable scale references, which transposes
/// the scale down by the specified number of octaves in place.
///
/// Each octave shift down decreases each pitch in the scale by 12 semitones.
/// This implementation modifies the scale in place through a mutable reference.
///
/// # Examples
/// ```
/// use mozzart_std::{Scale, C3, C4, major_scale};
///
/// // Create a mutable scale
/// let mut scale = major_scale(C4);
///
/// // Transpose down one octave in place
/// scale <<= 1;
/// assert_eq!(scale, major_scale(C3));
/// ```
impl<const N: usize> ShlAssign<u8> for &mut Scale<N> {
    fn shl_assign(&mut self, shift: u8) {
        self.pitches = self.pitches.map(|p| p << shift);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_constructors() {
        // Test major scale constructor

        // C4, D4, E4, F4, G4, A4, B4, C5
        let c_major = major_scale(C4);
        assert_eq!(c_major.quality(), ScaleQuality::Major);
        assert_eq!(c_major.tonic(), C4);
        assert_eq!(c_major.pitches(), &[C4, D4, E4, F4, G4, A4, B4, C5]);
        assert_eq!(c_major.steps(), MAJOR_SCALE_STEPS);

        // Test minor scale constructor

        // A4, B4, C5, D5, E5, F5, G5, A5
        let a_minor = minor_scale(A4);
        assert_eq!(a_minor.quality(), ScaleQuality::Minor);
        assert_eq!(a_minor.tonic(), A4);
        assert_eq!(a_minor.pitches(), &[A4, B4, C5, D5, E5, F5, G5, A5]);
        assert_eq!(a_minor.steps(), MINOR_SCALE_STEPS);

        // Test harmonic minor scale constructor

        // E4, FSHARP4, G4, A4, B4, C5, DSHARP5, E5
        let e_harmonic = harmonic_scale(E4);
        assert_eq!(e_harmonic.quality(), ScaleQuality::Harmonic);
        assert_eq!(e_harmonic.tonic(), E4);
        assert_eq!(
            e_harmonic.pitches(),
            &[E4, FSHARP4, G4, A4, B4, C5, DSHARP5, E5]
        );
        assert_eq!(e_harmonic.steps(), HARMONIC_SCALE_STEPS);

        // Test melodic minor scale constructor

        // D4, E4, F4, G4, A4, B4, CSHARP5, D5
        let d_melodic = melodic_scale(D4);
        assert_eq!(d_melodic.quality(), ScaleQuality::Melodic);
        assert_eq!(d_melodic.tonic(), D4);
        assert_eq!(d_melodic.pitches(), &[D4, E4, F4, G4, A4, B4, CSHARP5, D5]);
        assert_eq!(d_melodic.steps(), MELODIC_SCALE_STEPS);
    }

    #[test]
    fn test_scale_from_steps() {
        let scale = Scale::from_steps(C0, ScaleQuality::Major, &MAJOR_SCALE_STEPS);
        assert_eq!(scale, major_scale(C0));

        let scale = Scale::from_steps(C0, ScaleQuality::Minor, &MINOR_SCALE_STEPS);
        assert_eq!(scale, minor_scale(C0));

        let scale = Scale::from_steps(C0, ScaleQuality::Harmonic, &HARMONIC_SCALE_STEPS);
        assert_eq!(scale, harmonic_scale(C0));

        let scale = Scale::from_steps(C0, ScaleQuality::Melodic, &MELODIC_SCALE_STEPS);
        assert_eq!(scale, melodic_scale(C0));
    }

    #[test]
    fn test_scale_ascends() {
        // Test that all scale constructors handle octave-spanning scales correctly
        let scales = [
            major_scale(C4),
            minor_scale(A4),
            harmonic_scale(E4),
            melodic_scale(D4),
        ];

        let _ = scales.iter().inspect(|s| {
            let pitches = s.pitches();
            assert_eq!(pitches[0] >> 1, pitches[7]);
            for i in 0..7 {
                assert!(pitches[i + 1] > pitches[i], "Scale notes should ascend");
            }
        });
    }

    #[test]
    fn test_scale_right_shift() {
        // Test shifting C4 major scale down one octave
        let c4_scale = major_scale(C4);
        let c5_scale = c4_scale >> 1;
        assert_eq!(c5_scale, major_scale(C5));

        // Test shifting C4 major scale down two octaves
        let c6_scale = c4_scale >> 2;
        assert_eq!(c6_scale, major_scale(C6));
    }

    #[test]
    fn test_scale_right_shift_assign() {
        // Test shifting C4 major scale down one octave in place
        let mut scale = major_scale(C4);
        scale >>= 1;
        assert_eq!(scale, major_scale(C5));

        // Test shifting down another octave
        scale >>= 1;
        assert_eq!(scale, major_scale(C6));
    }

    #[test]
    fn test_scale_left_shift() {
        // Test shifting C4 major scale up one octave
        let c4_scale = major_scale(C4);
        let c3_scale = c4_scale << 1;
        assert_eq!(c3_scale, major_scale(C3));

        // Test shifting C4 major scale up two octaves
        let c1_scale = c3_scale << 2;
        assert_eq!(c1_scale, major_scale(C1));
    }

    #[test]
    fn test_scale_left_shift_assign() {
        // Test shifting C4 major scale up one octave in place
        let mut scale = major_scale(C4);
        scale <<= 1;
        assert_eq!(scale, major_scale(C3));

        // Test shifting up another octave
        scale <<= 1;
        assert_eq!(scale, major_scale(C2));
    }

    #[test]
    fn test_scale_right_shift_preserves_quality() {
        let scale = major_scale(C4) >> 1;
        assert_eq!(scale.quality(), ScaleQuality::Major);

        let scale = minor_scale(C4) >> 1;
        assert_eq!(scale.quality(), ScaleQuality::Minor);

        let scale = harmonic_scale(C4) >> 1;
        assert_eq!(scale.quality(), ScaleQuality::Harmonic);

        let scale = melodic_scale(C4) >> 1;
        assert_eq!(scale.quality(), ScaleQuality::Melodic);
    }

    #[test]
    fn test_scale_left_shift_preserves_quality() {
        let scale = major_scale(C4) << 1;
        assert_eq!(scale.quality(), ScaleQuality::Major);

        let scale = minor_scale(C4) << 1;
        assert_eq!(scale.quality(), ScaleQuality::Minor);

        let scale = harmonic_scale(C4) << 1;
        assert_eq!(scale.quality(), ScaleQuality::Harmonic);

        let scale = melodic_scale(C4) << 1;
        assert_eq!(scale.quality(), ScaleQuality::Melodic);
    }

    #[test]
    fn test_scale_shift_multiple_octaves() {
        let c4_scale = major_scale(C4);

        // Test multiple octave shifts up and down
        assert_eq!(c4_scale >> 3, major_scale(C7)); // Up 3 octaves
        assert_eq!(c4_scale << 3, major_scale(C1)); // Down 3 octaves

        // Test that shifting down then up returns to original
        let shifted = (c4_scale << 2) >> 2;
        assert_eq!(shifted, c4_scale);

        // Test that shifting up then down returns to original
        let shifted = (c4_scale >> 2) << 2;
        assert_eq!(shifted, c4_scale);
    }

    #[test]
    fn test_scale_right_shift_reference() {
        // Test right-shift on a scale reference
        let c4_scale = major_scale(C4);
        let c4_ref = &c4_scale;

        // Shift up one octave
        let c5_scale = c4_ref >> 1;

        // Verify that shift worked correctly
        assert_eq!(c5_scale.tonic(), C5);
        assert_eq!(c5_scale.pitches()[0], C5);
        assert_eq!(c5_scale.pitches()[4], G5);

        // Verify that original scale is unchanged
        assert_eq!(c4_scale.tonic(), C4);

        // Test with melodic scale
        let e3_melodic = melodic_scale(E3);
        let e3_ref = &e3_melodic;
        let e4_melodic = e3_ref >> 1;

        assert_eq!(e4_melodic.quality(), ScaleQuality::Melodic);
        assert_eq!(e4_melodic.tonic(), E4);
    }

    #[test]
    fn test_scale_right_shift_assign_mut_reference() {
        // Test right-shift assignment with a separate mutable reference
        let mut scale = major_scale(C4).clone();

        // Create a mutable reference and shift it
        {
            let scale_ref: &mut Scale<8> = &mut scale;
            *scale_ref >>= 1;
        }

        // Verify that shift worked correctly
        assert_eq!(scale.tonic(), C5);
        assert_eq!(scale.pitches()[0], C5);

        // Test with multiple octaves
        let mut d_scale = harmonic_scale(D3).clone();

        // Create a mutable reference and shift it two octaves
        {
            let d_ref: &mut Scale<8> = &mut d_scale;
            *d_ref >>= 2;
        }

        // Verify results
        assert_eq!(d_scale.tonic(), D5);
        assert_eq!(d_scale.quality(), ScaleQuality::Harmonic);
    }

    #[test]
    fn test_scale_left_shift_reference() {
        // Test left-shift on a scale reference
        let c4_scale = major_scale(C4);
        let c4_ref = &c4_scale;

        // Shift down one octave
        let c3_scale = c4_ref << 1;

        // Verify that shift worked correctly
        assert_eq!(c3_scale.tonic(), C3);
        assert_eq!(c3_scale.pitches()[0], C3);
        assert_eq!(c3_scale.pitches()[4], G3);

        // Verify that original scale is unchanged
        assert_eq!(c4_scale.tonic(), C4);

        // Test with melodic scale
        let e5_melodic = melodic_scale(E5);
        let e5_ref = &e5_melodic;
        let e4_melodic = e5_ref << 1;

        assert_eq!(e4_melodic.quality(), ScaleQuality::Melodic);
        assert_eq!(e4_melodic.tonic(), E4);
    }

    #[test]
    fn test_scale_left_shift_assign_mut_reference() {
        // Test left-shift assignment with a separate mutable reference
        let mut scale = major_scale(C4).clone();

        // Create a mutable reference and shift it
        {
            let scale_ref: &mut Scale<8> = &mut scale;
            *scale_ref <<= 1;
        }

        // Verify that shift worked correctly
        assert_eq!(scale.tonic(), C3);
        assert_eq!(scale.pitches()[0], C3);

        // Test with multiple octaves
        let mut a_scale = minor_scale(A5).clone();

        // Create a mutable reference and shift it two octaves
        {
            let a_ref: &mut Scale<8> = &mut a_scale;
            *a_ref <<= 2;
        }

        // Verify results
        assert_eq!(a_scale.tonic(), A3);
        assert_eq!(a_scale.quality(), ScaleQuality::Minor);
    }

    #[test]
    fn test_scale_mixed_reference_operations() {
        // Test a combination of reference and value operations
        let c4_scale = major_scale(C4);

        // Reference shift
        let c5_scale = &c4_scale >> 1;

        // Value shift on result of reference shift
        let c6_scale = c5_scale >> 1;

        // Verify double shift
        assert_eq!(c6_scale.tonic(), C6);

        // Original remains unchanged
        assert_eq!(c4_scale.tonic(), C4);

        // Test reference shift in both directions
        let b4_scale = melodic_scale(B4);
        let b3_scale = &b4_scale << 1;
        let b5_scale = &b3_scale >> 2;

        // Verify shifts worked correctly
        assert_eq!(b3_scale.tonic(), B3);
        assert_eq!(b5_scale.tonic(), B5);
        assert_eq!(b5_scale.quality(), ScaleQuality::Melodic);
    }
}
