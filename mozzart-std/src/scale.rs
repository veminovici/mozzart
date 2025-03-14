use crate::constants::*;
use crate::{Interval, Note, Step};
use std::fmt;

/// Represents the quality or type of a musical scale
///
/// In music theory, scale quality defines the character and emotional color of a scale
/// based on its unique interval pattern. Each scale quality creates a distinct sound
/// and has specific applications in different musical contexts.
///
/// The four primary scale qualities in Western music are:
/// - Major: bright, happy, resolved sound used extensively in pop, classical, and folk music
/// - Minor (Natural): darker, melancholic sound common in emotional or somber compositions
/// - Harmonic Minor: exotic sound with an augmented second interval, creating tension
/// - Melodic Minor: smoother ascending melodic line that addresses the awkward interval
///   found in the harmonic minor scale
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ScaleQuality {
    /// Major scale quality, characterized by a bright, happy sound
    Major,
    /// Natural minor scale quality (Aeolian mode), with a darker, melancholic sound
    Minor,
    /// Harmonic minor scale quality, featuring an augmented second interval for stronger resolution
    HarmonicMinor,
    /// Melodic minor scale quality (ascending form), with raised 6th and 7th degrees for smoother voice leading
    MelodicMinor,
}

/// Represents a musical scale with a specific number of notes
///
/// A `Scale` is a collection of musical notes arranged in ascending or descending order,
/// where each note has a specific relationship to the root note. Scales are fundamental
/// building blocks in music theory and composition.
///
/// The generic parameter `N` represents the number of notes in the scale:
/// - Standard diatonic scales (major, minor) typically use N=8 (including the octave)
/// - Pentatonic scales typically use N=5 or N=6 (including the octave)
/// - Other scales may have different numbers of notes
///
/// Each scale has:
/// - A root note (the first note of the scale, which establishes the key center)
/// - A quality (major, minor, etc.) that defines its interval pattern
/// - A sequence of notes following the pattern defined by the quality
pub struct Scale<const N: usize> {
    /// The quality of the scale (major, minor, etc.)
    quality: ScaleQuality,
    /// The notes that make up the scale, starting with the root note
    notes: [Note; N],
}

impl<const N: usize> Scale<N> {
    /// Creates a new `Scale` with the specified quality and notes
    ///
    /// This constructor takes a scale quality and a collection of notes, and
    /// initializes a new scale. The method is intended for internal use within
    /// the library, as scales are typically created using the specialized
    /// functions like `major_scale` or `natural_minor_scale`.
    ///
    /// # Arguments
    /// * `quality` - The quality (type) of the scale being created
    /// * `notes` - An iterable collection of notes that make up the scale
    ///
    /// # Returns
    /// A new `Scale` instance with the specified quality and notes
    pub(crate) fn new(quality: ScaleQuality, notes: impl IntoIterator<Item = Note>) -> Self {
        let mut ns = [C; N];
        for (i, n) in notes.into_iter().enumerate() {
            ns[i] = n;
        }

        Self { quality, notes: ns }
    }

    /// Returns the root note of the scale
    ///
    /// The root note is the first note of the scale and establishes the tonal center.
    /// All other notes in the scale are defined in relation to this note.
    ///
    /// # Returns
    /// The root note (first note) of the scale
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{constants::*, major_scale};
    ///
    /// let d_major = major_scale(D4);
    /// assert_eq!(d_major.root(), D4);
    /// ```
    #[inline]
    pub const fn root(&self) -> Note {
        self.notes[0]
    }

    /// Returns all notes in the scale as an array
    ///
    /// This method provides access to the complete sequence of notes that make up the scale,
    /// starting with the root note and ending with the octave (for 8-note scales).
    ///
    /// # Returns
    /// A reference to the array of notes that make up the scale
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{constants::*, major_scale};
    ///
    /// let c_major = major_scale(C4);
    /// let notes = c_major.notes();
    ///
    /// // C major scale should have these notes: C, D, E, F, G, A, B, C
    /// assert_eq!(notes[0], C4); // Root
    /// assert_eq!(notes[4], G4); // Fifth
    /// assert_eq!(notes[7], C5); // Octave
    /// ```
    #[inline]
    pub const fn notes(&self) -> &[Note; N] {
        &self.notes
    }

    /// Returns the quality of the scale
    ///
    /// The scale quality (major, minor, etc.) defines the pattern of intervals
    /// between adjacent notes, which gives the scale its characteristic sound and mood.
    ///
    /// # Returns
    /// The quality (type) of the scale
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{constants::*, major_scale, natural_minor_scale, ScaleQuality};
    ///
    /// let c_major = major_scale(C4);
    /// assert_eq!(c_major.quality(), ScaleQuality::Major);
    ///
    /// let a_minor = natural_minor_scale(A4);
    /// assert_eq!(a_minor.quality(), ScaleQuality::Minor);
    /// ```
    #[inline]
    pub const fn quality(&self) -> ScaleQuality {
        self.quality
    }
}

/// Returns the suffix for a scale
///
/// This function takes a `ScaleQuality` and returns the appropriate suffix for the scale.
///
/// # Arguments
/// * `quality` - The quality (type) of the scale
///
/// # Returns
/// The suffix for the scale
///
/// # Examples
/// ```ignore
/// use mozzart_std::{ScaleQuality, scale_suffix};
/// use mozzart_std::constants::*;
///
/// assert_eq!(scale_suffix(ScaleQuality::Major), "major");
/// assert_eq!(scale_suffix(ScaleQuality::Minor), "minor");
/// assert_eq!(scale_suffix(ScaleQuality::HarmonicMinor), "harmonic minor");
/// assert_eq!(scale_suffix(ScaleQuality::MelodicMinor), "melodic minor");
/// ```
fn scale_suffix(quality: ScaleQuality) -> &'static str {
    match quality {
        ScaleQuality::Major => "major",
        ScaleQuality::Minor => "minor",
        ScaleQuality::HarmonicMinor => "harmonic minor",
        ScaleQuality::MelodicMinor => "melodic minor",
    }
}

impl<const N: usize> fmt::UpperHex for Scale<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let root = self.root();
        let suffix = scale_suffix(self.quality());

        write!(f, "{root:X} {suffix}")
    }
}

impl<const N: usize> fmt::LowerHex for Scale<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let root = self.root();
        let suffix = scale_suffix(self.quality());

        write!(f, "{root:x} {suffix}")
    }
}

impl<const N: usize> fmt::Debug for Scale<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let root = self.root();
        let suffix = scale_suffix(self.quality());

        write!(f, "{root:?}{suffix}")
    }
}

impl<const N: usize> fmt::Display for Scale<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:X}", self)
    }
}

impl Scale<8> {
    /// Returns the steps between the notes in the scale
    ///
    /// This method calculates the interval between each pair of adjacent notes
    /// in the scale and returns an array of steps.
    ///
    /// # Returns
    /// An array of 7 steps representing the intervals between the notes
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, constants::*, major_scale};
    ///
    /// let c_major = major_scale(C4);
    /// let steps = c_major.steps();
    /// assert_eq!(steps.len(), 7);
    ///
    /// // C major scale steps: W-W-H-W-W-W-H
    /// assert_eq!(steps, [WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF]);
    /// ```
    pub fn steps(&self) -> [Step; 7] {
        let mut steps = [UNISON; 7];
        for (i, step) in steps.iter_mut().enumerate() {
            *step = self.notes[i + 1] - self.notes[i];
        }

        steps
    }

    /// Returns the intervals between the notes in the scale
    ///
    /// This method calculates the interval between each note and the root note
    /// in the scale and returns an array of intervals.
    ///
    /// # Returns
    /// An array of 7 intervals representing the intervals between the notes and the root note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, constants::*, major_scale};
    ///
    /// let c_major = major_scale(C4);
    /// let intervals = c_major.intervals();
    /// assert_eq!(intervals.len(), 7);
    ///
    /// // C major intervals: [MAJOR_SECOND, MAJOR_THIRD, PERFECT_FOURTH, PERFECT_FIFTH, MINOR_SIXTH, MAJOR_SEVENTH, PERFECT_OCTAVE]
    /// assert_eq!(intervals, [MAJOR_SECOND, MAJOR_THIRD, PERFECT_FOURTH, PERFECT_FIFTH, MINOR_SIXTH, MAJOR_SEVENTH, PERFECT_OCTAVE]);
    /// ```
    pub fn intervals(&self) -> [Interval; 7] {
        let mut intervals = [PERFECT_UNISON; 7];
        for (i, interval) in intervals.iter_mut().enumerate() {
            let step = self.notes[i + 1] - self.notes[0];
            *interval = step.into();
        }

        intervals
    }
}

/// Creates a major scale starting from the specified root note
///
/// A major scale consists of 8 notes (including the octave) and follows
/// the pattern of whole and half steps: W-W-H-W-W-W-H.
///
/// # Arguments
/// * `root` - The root note from which to build the scale
///
/// # Returns
/// A `Scale<8>` representing the major scale
///
/// # Examples
/// ```
/// use mozzart_std::{Note, constants::*, major_scale};
///
/// // Create a C major scale
/// let c_major = major_scale(C4);
/// let notes = c_major.notes();
///
/// // C major should contain C, D, E, F, G, A, B, C
/// assert_eq!(notes[0], C4);
/// assert_eq!(notes[7], C5);
/// ```
pub fn major_scale(root: Note) -> Scale<8> {
    let notes = root.into_notes_from_steps(MAJOR_SCALE_STEPS);
    Scale::new(ScaleQuality::Major, notes)
}

/// Creates a natural minor scale starting from the specified root note
///
/// A natural minor scale consists of 8 notes (including the octave) and follows
/// the pattern of whole and half steps: W-H-W-W-H-W-W.
///
/// # Arguments
/// * `root` - The root note from which to build the scale
///
/// # Returns
/// A `Scale<8>` representing the natural minor scale
///
/// # Examples
/// ```
/// use mozzart_std::{Note, natural_minor_scale};
/// use mozzart_std::constants::*;
///
/// // Create an A minor scale
/// let a_minor = natural_minor_scale(A4);
/// let notes = a_minor.notes();
///
/// // A minor should contain A, B, C, D, E, F, G, A
/// assert_eq!(notes[0], A4);
/// assert_eq!(notes[2], C5);
/// assert_eq!(notes[7], A5);
/// ```
pub fn natural_minor_scale(root: Note) -> Scale<8> {
    let notes = root.into_notes_from_steps(NATURAL_MINOR_SCALE_STEPS);
    Scale::new(ScaleQuality::Minor, notes)
}

/// Creates a harmonic minor scale starting from the specified root note
///
/// A harmonic minor scale consists of 8 notes (including the octave) and is
/// based on the natural minor scale with a raised 7th degree. It follows
/// the pattern of intervals: W-H-W-W-H-W+H-H, where W+H represents
/// an augmented second (3 semitones).
///
/// The raised 7th creates a leading tone that has a stronger pull to the tonic,
/// and the augmented second between the 6th and 7th degrees gives the scale
/// its distinctive exotic sound.
///
/// # Arguments
/// * `root` - The root note from which to build the scale
///
/// # Returns
/// A `Scale<8>` representing the harmonic minor scale
///
/// # Examples
/// ```
/// use mozzart_std::{Note, constants::*, harmonic_minor_scale};
///
/// // Create an A harmonic minor scale
/// let a_harmonic_minor = harmonic_minor_scale(A4);
/// let notes = a_harmonic_minor.notes();
///
/// // A harmonic minor should contain A, B, C, D, E, F, G#, A
/// assert_eq!(notes[0], A4);
/// assert_eq!(notes[6], GSHARP5); // The raised 7th degree
/// assert_eq!(notes[7], A5);
/// ```
pub fn harmonic_minor_scale(root: Note) -> Scale<8> {
    let notes = root.into_notes_from_steps(HARMONIC_MINOR_SCALE_STEPS);
    Scale::new(ScaleQuality::HarmonicMinor, notes)
}

/// Creates a melodic minor scale (ascending form) starting from the specified root note
///
/// A melodic minor scale consists of 8 notes (including the octave) and is
/// based on the natural minor scale with raised 6th and 7th degrees. It follows
/// the pattern of intervals: W-H-W-W-W-W-H.
///
/// The raised 6th and 7th degrees create a smoother ascending melodic line.
/// Traditionally, the descending form reverts to the natural minor scale,
/// though in modern practice (especially in jazz), the ascending form is
/// often used both up and down.
///
/// # Arguments
/// * `root` - The root note from which to build the scale
///
/// # Returns
/// A `Scale<8>` representing the melodic minor scale (ascending form)
///
/// # Examples
/// ```
/// use mozzart_std::{Note, constants::*, melodic_minor_scale};
///
/// // Create an A melodic minor scale
/// let a_melodic_minor = melodic_minor_scale(A4);
/// let notes = a_melodic_minor.notes();
///
/// // A melodic minor should contain A, B, C, D, E, F#, G#, A
/// assert_eq!(notes[0], A4);
/// assert_eq!(notes[5], FSHARP5); // The raised 6th degree
/// assert_eq!(notes[6], GSHARP5); // The raised 7th degree
/// assert_eq!(notes[7], A5);
/// ```
pub fn melodic_minor_scale(root: Note) -> Scale<8> {
    let notes = root.into_notes_from_steps(MELODIC_MINOR_SCALE_STEPS);
    Scale::new(ScaleQuality::MelodicMinor, notes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_scale() {
        let c4_major = major_scale(C4);
        let notes = c4_major.notes();

        // Verify scale quality
        assert_eq!(c4_major.quality(), ScaleQuality::Major);

        // Verify notes in C major scale
        assert_eq!(notes[0], C4); // C4 (root)
        assert_eq!(notes[1], D4); // D4
        assert_eq!(notes[2], E4); // E4
        assert_eq!(notes[3], F4); // F4
        assert_eq!(notes[4], G4); // G4
        assert_eq!(notes[5], A4); // A4
        assert_eq!(notes[6], B4); // B4
        assert_eq!(notes[7], C5); // C5 (octave)

        assert_eq!(c4_major.to_string(), "C major");
    }

    #[test]
    fn test_natural_minor_scale() {
        let a4_minor = natural_minor_scale(A4);
        let notes = a4_minor.notes();

        // Verify scale quality
        assert_eq!(a4_minor.quality(), ScaleQuality::Minor);

        // Verify notes in A minor scale
        assert_eq!(notes[0], A4); // A4 (root)
        assert_eq!(notes[1], B4); // B4
        assert_eq!(notes[2], C5); // C5
        assert_eq!(notes[3], D5); // D5
        assert_eq!(notes[4], E5); // E5
        assert_eq!(notes[5], F5); // F5
        assert_eq!(notes[6], G5); // G5
        assert_eq!(notes[7], A5); // A5 (octave)

        assert_eq!(a4_minor.to_string(), "A minor");
    }

    #[test]
    fn test_harmonic_minor_scale() {
        let a4_harmonic_minor = harmonic_minor_scale(A4);
        let notes = a4_harmonic_minor.notes();

        // Verify scale quality
        assert_eq!(a4_harmonic_minor.quality(), ScaleQuality::HarmonicMinor);

        // Verify notes in A harmonic minor scale
        assert_eq!(notes[0], A4); // A4 (root)
        assert_eq!(notes[1], B4); // B4
        assert_eq!(notes[2], C5); // C5
        assert_eq!(notes[3], D5); // D5
        assert_eq!(notes[4], E5); // E5
        assert_eq!(notes[5], F5); // F5
        assert_eq!(notes[6], GSHARP5); // G#5 (raised 7th)
        assert_eq!(notes[7], A5); // A5 (octave)

        // Confirm the difference from natural minor is at the 7th degree
        let a4_natural_minor = natural_minor_scale(A4);
        assert_eq!(a4_natural_minor.notes()[6], G5); // G5 in natural minor
        assert_eq!(harmonic_minor_scale(A4).notes()[6], GSHARP5); // G#5 in harmonic minor

        assert_eq!(a4_harmonic_minor.to_string(), "A harmonic minor");
    }

    #[test]
    fn test_melodic_minor_scale() {
        let a4_melodic_minor = melodic_minor_scale(A4);
        let notes = a4_melodic_minor.notes();

        // Verify scale quality
        assert_eq!(a4_melodic_minor.quality(), ScaleQuality::MelodicMinor);

        // Verify notes in A melodic minor scale (ascending)
        assert_eq!(notes[0], A4); // A4 (root)
        assert_eq!(notes[1], B4); // B4
        assert_eq!(notes[2], C5); // C5
        assert_eq!(notes[3], D5); // D5
        assert_eq!(notes[4], E5); // E5
        assert_eq!(notes[5], FSHARP5); // F#5 (raised 6th)
        assert_eq!(notes[6], GSHARP5); // G#5 (raised 7th)
        assert_eq!(notes[7], A5); // A5 (octave)

        // Confirm the difference from natural minor is at the 6th and 7th degrees
        let a4_natural_minor = natural_minor_scale(A4);
        assert_eq!(a4_natural_minor.notes()[5], F5); // F5 in natural minor
        assert_eq!(a4_natural_minor.notes()[6], G5); // G5 in natural minor

        assert_eq!(melodic_minor_scale(A4).notes()[5], FSHARP5); // F#5 in melodic minor
        assert_eq!(melodic_minor_scale(A4).notes()[6], GSHARP5); // G#5 in melodic minor

        assert_eq!(a4_melodic_minor.to_string(), "A melodic minor");
    }

    #[test]
    fn test_different_roots() {
        // Test with different roots to ensure scale patterns work correctly

        // D major scale
        let d4_major = major_scale(D4);
        let notes = d4_major.notes();
        assert_eq!(notes[0], D4); // D4
        assert_eq!(notes[2], FSHARP4); // F#4 (not F4)
        assert_eq!(notes[6], CSHARP5); // C#5 (not C5)

        // E harmonic minor scale
        let e4_harmonic_minor = harmonic_minor_scale(E4);
        let notes = e4_harmonic_minor.notes();
        assert_eq!(notes[0], E4); // E4
        assert_eq!(notes[2], G4); // G4
        assert_eq!(notes[6], DSHARP5); // D#5 (raised 7th)

        // G melodic minor scale
        let g4_melodic_minor = melodic_minor_scale(G4);
        let notes = g4_melodic_minor.notes();
        assert_eq!(notes[0], G4); // G4
        assert_eq!(notes[5], E5); // E5 (raised 6th)
        assert_eq!(notes[6], FSHARP5); // F#5 (raised 7th)
    }

    #[test]
    fn test_intervals() {
        let c_major = major_scale(C4);
        let intervals = c_major.intervals();
        assert_eq!(
            intervals,
            [
                MAJOR_SECOND,
                MAJOR_THIRD,
                PERFECT_FOURTH,
                PERFECT_FIFTH,
                MINOR_SIXTH,
                MAJOR_SEVENTH,
                PERFECT_OCTAVE
            ]
        );
    }

    #[test]
    fn test_steps() {
        let c_major = major_scale(C4);
        let steps = c_major.steps();
        assert_eq!(steps, [WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF]);
    }
}
