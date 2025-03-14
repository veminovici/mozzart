use crate::{constants::*, diminished_triad, major_triad, minor_triad};
use crate::{Chord, Interval, Note, Step};
use std::fmt;
use std::marker::PhantomData;

/// Trait for converting a note into a major scale
///
/// This trait provides a method to convert a note into a major scale.
/// It is implemented for the `Note` type and allows for easy conversion
/// between notes and their corresponding major scales.
pub trait IntoMajorScale {
    /// Converts the note into a major scale
    ///
    /// # Returns
    /// A `Scale<MajorScaleQuality, 8>` representing the major scale starting from this note
    fn into_major_scale(self) -> Scale<MajorScaleQuality, 8>;
}

/// Trait for converting a note into a natural minor scale
///
/// This trait provides a method to convert a note into a natural minor scale.
/// It is implemented for the `Note` type and allows for easy conversion
/// between notes and their corresponding natural minor scales.
pub trait IntoNaturalMinorScale {
    /// Converts the note into a natural minor scale
    ///
    /// # Returns
    /// A `Scale<MinorScaleQuality, 8>` representing the natural minor scale starting from this note
    fn into_natural_minor_scale(self) -> Scale<MinorScaleQuality, 8>;
}

/// Trait for converting a note into a harmonic minor scale
///
/// This trait provides a method to convert a note into a harmonic minor scale.
/// It is implemented for the `Note` type and allows for easy conversion
/// between notes and their corresponding harmonic minor scales.
pub trait IntoHarmonicMinorScale {
    /// Converts the note into a harmonic minor scale
    ///
    /// # Returns
    /// A `Scale<HarmonicMinorScaleQuality, 8>` representing the harmonic minor scale starting from this note
    fn into_harmonic_minor_scale(self) -> Scale<HarmonicMinorScaleQuality, 8>;
}

/// Trait for converting a note into a melodic minor scale
///
/// This trait provides a method to convert a note into a melodic minor scale.
/// It is implemented for the `Note` type and allows for easy conversion
/// between notes and their corresponding melodic minor scales.
pub trait IntoMelodicMinorScale {
    /// Converts the note into a melodic minor scale
    ///
    /// # Returns
    /// A `Scale<MelodicMinorScaleQuality, 8>` representing the melodic minor scale starting from this note
    fn into_melodic_minor_scale(self) -> Scale<MelodicMinorScaleQuality, 8>;
}

/// Defines the musical quality of a scale, providing its name and characteristics
///
/// This trait is implemented by various scale quality types, each representing
/// a specific scale pattern (major, minor, harmonic minor, etc.).
/// Scale qualities define the pattern of intervals that give each scale its distinct sound.
pub trait ScaleQuality {
    /// Returns the name of the scale quality
    fn name() -> &'static str;
}

/// Represents the major scale quality
///
/// The major scale follows the pattern of whole and half steps: W-W-H-W-W-W-H.
/// It is one of the most common scales in Western music and has a bright,
/// happy, or resolved sound. The major scale forms the basis of major tonality
/// and is often associated with positive emotional expressions in music.
pub struct MajorScaleQuality;

/// Represents the natural minor scale quality (also known as Aeolian mode)
///
/// The natural minor scale follows the pattern of whole and half steps: W-H-W-W-H-W-W.
/// It has a darker, more melancholic sound compared to the major scale.
/// The natural minor scale forms the basis of minor tonality and is often
/// associated with sad, serious, or contemplative emotional expressions in music.
pub struct MinorScaleQuality;

/// Represents the harmonic minor scale quality
///
/// The harmonic minor scale is based on the natural minor scale but with a raised 7th degree.
/// It follows the pattern: W-H-W-W-H-(W+H)-H, where W+H represents an augmented second (3 semitones).
///
/// The raised 7th creates a leading tone that has a stronger pull to the tonic,
/// and the augmented second between the 6th and 7th degrees gives the scale
/// its distinctive exotic sound. This scale is commonly used in classical music and
/// various world music traditions, particularly those of Eastern Europe and the Middle East.
pub struct HarmonicMinorScaleQuality;

/// Represents the melodic minor scale quality (ascending form)
///
/// The melodic minor scale is based on the natural minor scale but with raised 6th and 7th degrees.
/// It follows the pattern: W-H-W-W-W-W-H.
///
/// The raised 6th and 7th degrees create a smoother ascending melodic line.
/// Traditionally, the descending form reverts to the natural minor scale,
/// though in modern practice (especially in jazz), the ascending form is
/// often used both up and down.
///
/// This scale is commonly used in classical, jazz, and contemporary music,
/// offering a distinctive sound that is neither fully major nor minor.
pub struct MelodicMinorScaleQuality;

impl ScaleQuality for MajorScaleQuality {
    fn name() -> &'static str {
        "major"
    }
}
impl ScaleQuality for MinorScaleQuality {
    fn name() -> &'static str {
        "minor"
    }
}
impl ScaleQuality for HarmonicMinorScaleQuality {
    fn name() -> &'static str {
        "harmonic minor"
    }
}
impl ScaleQuality for MelodicMinorScaleQuality {
    fn name() -> &'static str {
        "melodic minor"
    }
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
pub struct Scale<Q, const N: usize>
where
    Q: ScaleQuality,
{
    /// The notes that make up the scale, starting with the root note
    notes: [Note; N],
    quality: PhantomData<Q>,
}

impl<Q, const N: usize> Scale<Q, N>
where
    Q: ScaleQuality,
{
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
    pub(crate) fn new(notes: impl IntoIterator<Item = Note>) -> Self {
        let mut ns = [C4; N];
        for (i, n) in notes.into_iter().enumerate() {
            ns[i] = n;
        }

        Self {
            quality: PhantomData,
            notes: ns,
        }
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
}

impl<Q, const N: usize> fmt::UpperHex for Scale<Q, N>
where
    Q: ScaleQuality,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let root = self.root();
        let suffix = Q::name();

        write!(f, "{root:X} {suffix}")
    }
}

impl<Q, const N: usize> fmt::LowerHex for Scale<Q, N>
where
    Q: ScaleQuality,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let root = self.root();
        let suffix = Q::name();

        write!(f, "{root:x} {suffix}")
    }
}

impl<Q, const N: usize> fmt::Debug for Scale<Q, N>
where
    Q: ScaleQuality,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let root = self.root();
        let suffix = Q::name();

        write!(f, "{root:?} {suffix}")
    }
}

impl<Q, const N: usize> fmt::Display for Scale<Q, N>
where
    Q: ScaleQuality,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:X}", self)
    }
}

impl<Q> Scale<Q, 8>
where
    Q: ScaleQuality,
{
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

impl Scale<MajorScaleQuality, 8> {
    /// Returns the I major chord of the scale
    ///
    /// The I major chord is the first chord in the scale, built from the root note.
    /// It is a major triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the I major chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, constants::*, major_scale};
    ///
    /// let c_major = major_scale(C4);
    /// let i_major_chord = c_major.i_major_chord();
    /// assert_eq!(i_major_chord.notes(), &[C4, E4, G4]);
    /// ```
    pub fn i_major_chord(&self) -> Chord<3> {
        let root = self.notes[0];
        major_triad(root)
    }

    /// Returns the II minor chord of the scale
    ///
    /// The II minor chord is the second chord in the scale, built from the second note.
    /// It is a minor triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the II minor chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, constants::*, major_scale};
    ///
    /// let c_major = major_scale(C4);
    /// let ii_minor_chord = c_major.ii_minor_chord();
    /// assert_eq!(ii_minor_chord.notes(), &[D4, F4, A4]);
    /// ```
    pub fn ii_minor_chord(&self) -> Chord<3> {
        let root = self.notes[1];
        minor_triad(root)
    }

    /// Returns the III minor chord of the scale
    ///
    /// The III minor chord is the third chord in the scale, built from the third note.
    /// It is a minor triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the III minor chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, constants::*, major_scale};
    ///
    /// let c_major = major_scale(C4);
    /// let iii_minor_chord = c_major.iii_minor_chord();
    /// assert_eq!(iii_minor_chord.notes(), &[E4, G4, B4]);
    /// ```
    pub fn iii_minor_chord(&self) -> Chord<3> {
        let root = self.notes[2];
        minor_triad(root)
    }

    /// Returns the IV major chord of the scale
    ///
    /// The IV major chord is the fourth chord in the scale, built from the fourth note.
    /// It is a major triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the IV major chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, constants::*, major_scale};
    ///
    /// let c_major = major_scale(C4);
    /// let iv_major_chord = c_major.iv_major_chord();
    /// assert_eq!(iv_major_chord.notes(), &[F4, A4, C5]);
    /// ```
    pub fn iv_major_chord(&self) -> Chord<3> {
        let root = self.notes[3];
        major_triad(root)
    }

    /// Returns the V major chord of the scale
    ///
    /// The V major chord is the fifth chord in the scale, built from the fifth note.
    /// It is a major triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the V major chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, constants::*, major_scale};
    ///
    /// let c_major = major_scale(C4);
    /// let v_major_chord = c_major.v_major_chord();
    /// assert_eq!(v_major_chord.notes(), &[G4, B4, D5]);
    /// ```
    pub fn v_major_chord(&self) -> Chord<3> {
        let root = self.notes[4];
        major_triad(root)
    }

    /// Returns the VI minor chord of the scale
    ///
    /// The VI minor chord is the sixth chord in the scale, built from the sixth note.
    /// It is a minor triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the VI minor chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, constants::*, major_scale};
    ///
    /// let c_major = major_scale(C4);
    /// let vi_minor_chord = c_major.vi_minor_chord();
    /// assert_eq!(vi_minor_chord.notes(), &[A4, C5, E5]);
    /// ```
    pub fn vi_minor_chord(&self) -> Chord<3> {
        let root = self.notes[5];
        minor_triad(root)
    }

    /// Returns the VII diminished chord of the scale
    ///
    /// The VII diminished chord is the seventh chord in the scale, built from the seventh note.
    /// It is a diminished triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the VII diminished chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, constants::*, major_scale};
    ///
    /// let c_major = major_scale(C4);
    /// let vii_diminished_chord = c_major.vii_diminished_chord();
    /// assert_eq!(vii_diminished_chord.notes(), &[B4, D5, F5]);
    /// ```
    pub fn vii_diminished_chord(&self) -> Chord<3> {
        let root = self.notes[6];
        diminished_triad(root)
    }
}

impl Scale<MinorScaleQuality, 8> {
    /// Returns the I minor chord of the scale
    ///
    /// The I minor chord is the first chord in the scale, built from the root note.
    /// It is a minor triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the I minor chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, natural_minor_scale};
    /// use mozzart_std::constants::*;
    ///
    /// let a_minor = natural_minor_scale(C4);
    /// let i_minor_chord = a_minor.i_minor_chord();
    /// assert_eq!(i_minor_chord.notes(), &[C4, DSHARP4, G4]);
    /// ```
    pub fn i_minor_chord(&self) -> Chord<3> {
        let root = self.notes[0];
        minor_triad(root)
    }

    /// Returns the II diminished chord of the scale
    ///
    /// The II diminished chord is the second chord in the scale, built from the second note.
    /// It is a diminished triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the II diminished chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, natural_minor_scale};
    /// use mozzart_std::constants::*;
    ///
    /// let a_minor = natural_minor_scale(C4);
    /// let ii_diminished_chord = a_minor.ii_diminished_chord();
    /// assert_eq!(ii_diminished_chord.notes(), &[D4, F4, GSHARP4]);
    /// ```
    pub fn ii_diminished_chord(&self) -> Chord<3> {
        let root = self.notes[1];
        diminished_triad(root)
    }

    /// Returns the III major chord of the scale
    ///
    /// The III major chord is the third chord in the scale, built from the third note.
    ///
    /// It is a major triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the III major chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, natural_minor_scale};
    /// use mozzart_std::constants::*;
    ///
    /// let a_minor = natural_minor_scale(C4);
    /// let iii_major_chord = a_minor.iii_major_chord();
    /// assert_eq!(iii_major_chord.notes(), &[DSHARP4, G4, BFLAT4]);
    /// ```
    pub fn iii_major_chord(&self) -> Chord<3> {
        let root = self.notes[2];
        major_triad(root)
    }

    /// Returns the IV minor chord of the scale
    ///
    /// The IV minor chord is the fourth chord in the scale, built from the fourth note.
    ///
    /// It is a minor triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the IV minor chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, natural_minor_scale};
    /// use mozzart_std::constants::*;
    ///
    /// let a_minor = natural_minor_scale(C4);
    /// let iv_minor_chord = a_minor.iv_minor_chord();
    /// assert_eq!(iv_minor_chord.notes(), &[F4, GSHARP4, C5]);
    /// ```
    pub fn iv_minor_chord(&self) -> Chord<3> {
        let root = self.notes[3];
        minor_triad(root)
    }

    /// Returns the V minor chord of the scale
    ///
    /// The V minor chord is the fifth chord in the scale, built from the fifth note.
    ///
    /// It is a minor triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the V minor chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, natural_minor_scale};
    /// use mozzart_std::constants::*;
    ///
    /// let a_minor = natural_minor_scale(C4);
    /// let v_minor_chord = a_minor.v_minor_chord();
    /// assert_eq!(v_minor_chord.notes(), &[G4, BFLAT4, D5]);
    /// ```
    pub fn v_minor_chord(&self) -> Chord<3> {
        let root = self.notes[4];
        minor_triad(root)
    }

    /// Returns the VI major chord of the scale
    ///
    /// The VI major chord is the sixth chord in the scale, built from the sixth note.
    ///
    /// It is a major triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the VI major chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, natural_minor_scale};
    /// use mozzart_std::constants::*;
    ///
    /// let a_minor = natural_minor_scale(C4);
    /// let vi_major_chord = a_minor.vi_major_chord();
    /// assert_eq!(vi_major_chord.notes(), &[GSHARP4, C5, DSHARP5]);
    /// ```
    pub fn vi_major_chord(&self) -> Chord<3> {
        let root = self.notes[5];
        major_triad(root)
    }

    /// Returns the VII major chord of the scale
    ///
    /// The VII major chord is the seventh chord in the scale, built from the seventh note.
    ///
    /// It is a major triad with the root, third, and fifth notes.
    ///
    /// # Returns
    /// A `Chord<3>` representing the VII major chord
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{Note, natural_minor_scale};
    /// use mozzart_std::constants::*;
    ///
    /// let a_minor = natural_minor_scale(C4);
    /// let vii_major_chord = a_minor.vii_major_chord();
    /// assert_eq!(vii_major_chord.notes(), &[ASHARP4, D5, F5]);
    /// ```
    pub fn vii_major_chord(&self) -> Chord<3> {
        let root = self.notes[6];
        major_triad(root)
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
/// A `Scale<MajorScale, 8>` representing the major scale
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
pub fn major_scale(root: Note) -> Scale<MajorScaleQuality, 8> {
    let notes = root.into_notes_from_steps(MAJOR_SCALE_STEPS);
    Scale::new(notes)
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
/// A `Scale<MinorScale, 8>` representing the natural minor scale
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
pub fn natural_minor_scale(root: Note) -> Scale<MinorScaleQuality, 8> {
    let notes = root.into_notes_from_steps(NATURAL_MINOR_SCALE_STEPS);
    Scale::new(notes)
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
/// A `Scale<HarmonicMinorScale, 8>` representing the harmonic minor scale
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
pub fn harmonic_minor_scale(root: Note) -> Scale<HarmonicMinorScaleQuality, 8> {
    let notes = root.into_notes_from_steps(HARMONIC_MINOR_SCALE_STEPS);
    Scale::new(notes)
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
/// A `Scale<MelodicMinorScale, 8>` representing the melodic minor scale (ascending form)
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
pub fn melodic_minor_scale(root: Note) -> Scale<MelodicMinorScaleQuality, 8> {
    let notes = root.into_notes_from_steps(MELODIC_MINOR_SCALE_STEPS);
    Scale::new(notes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_scale() {
        let c4_major = major_scale(C4);
        let notes = c4_major.notes();

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

    #[test]
    fn test_major_scale_i_chord() {
        let c_major = major_scale(C4);
        let i_chord = c_major.i_major_chord();
        assert_eq!(i_chord.notes(), &[C4, E4, G4]);
    }

    #[test]
    fn test_major_scale_ii_chord() {
        let c_major = major_scale(C4);
        let ii_chord = c_major.ii_minor_chord();
        assert_eq!(ii_chord.notes(), &[D4, F4, A4]);
    }

    #[test]
    fn test_major_scale_iii_chord() {
        let c_major = major_scale(C4);
        let iii_chord = c_major.iii_minor_chord();
        assert_eq!(iii_chord.notes(), &[E4, G4, B4]);
    }

    #[test]
    fn test_major_scale_iv_chord() {
        let c_major = major_scale(C4);
        let iv_chord = c_major.iv_major_chord();
        assert_eq!(iv_chord.notes(), &[F4, A4, C5]);
    }

    #[test]
    fn test_major_scale_v_chord() {
        let c_major = major_scale(C4);
        let v_chord = c_major.v_major_chord();
        assert_eq!(v_chord.notes(), &[G4, B4, D5]);
    }

    #[test]
    fn test_major_scale_vi_chord() {
        let c_major = major_scale(C4);
        let vi_chord = c_major.vi_minor_chord();
        assert_eq!(vi_chord.notes(), &[A4, C5, E5]);
    }

    #[test]
    fn test_major_scale_vii_chord() {
        let c_major = major_scale(C4);
        let vii_chord = c_major.vii_diminished_chord();
        assert_eq!(vii_chord.notes(), &[B4, D5, F5]);
    }

    #[test]
    fn test_minor_scale_i_chord() {
        let a_minor = natural_minor_scale(A4);
        let i_chord = a_minor.i_minor_chord();
        assert_eq!(i_chord.notes(), &[A4, C5, E5]);
    }

    #[test]
    fn test_minor_scale_ii_chord() {
        let a_minor = natural_minor_scale(A4);
        let ii_chord = a_minor.ii_diminished_chord();
        assert_eq!(ii_chord.notes(), &[B4, D5, F5]);
    }

    #[test]
    fn test_minor_scale_iii_chord() {
        let a_minor = natural_minor_scale(A4);
        let iii_chord = a_minor.iii_major_chord();
        assert_eq!(iii_chord.notes(), &[C5, E5, G5]);
    }

    #[test]
    fn test_minor_scale_iv_chord() {
        let a_minor = natural_minor_scale(A4);
        let iv_chord = a_minor.iv_minor_chord();
        assert_eq!(iv_chord.notes(), &[D5, F5, A5]);
    }

    #[test]
    fn test_minor_scale_v_chord() {
        let a_minor = natural_minor_scale(A4);
        let v_chord = a_minor.v_minor_chord();
        assert_eq!(v_chord.notes(), &[E5, G5, B5]);
    }

    #[test]
    fn test_minor_scale_vi_chord() {
        let a_minor = natural_minor_scale(A4);
        let vi_chord = a_minor.vi_major_chord();
        assert_eq!(vi_chord.notes(), &[F5, A5, C6]);
    }

    #[test]
    fn test_minor_scale_vii_chord() {
        let a_minor = natural_minor_scale(A4);
        let vii_chord = a_minor.vii_major_chord();
        assert_eq!(vii_chord.notes(), &[G5, B5, D6]);
    }
}
