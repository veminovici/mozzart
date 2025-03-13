use crate::constants::*;
use crate::Note;

/// Represents the quality of a chord
///
/// This enum defines the different types of chords that can be created.
/// Each variant corresponds to a specific chord type, and the enum is used
/// to determine the quality of a chord.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_major = major_triad(C4);
/// assert_eq!(c_major.quality(), ChordQuality::MajorTriad);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChordQuality {
    MajorTriad,
    MinorTriad,
    DiminishedTriad,
    AugmentedTriad,
    Sus2,
    Sus4,
    MajorSixth,
    MinorSixth,
    MajorSixthNinth,
    MinorSixthNinth,
    MajorSeventh,
    DominantSeventh,
    DominantSeventhNinth,
    MinorSeventh,
    MinorSeventhNinth,
    HalfDiminishedSeventh,
    DiminishedSeventh,
}

/// Represents a chord
///
/// This struct defines a chord, which is a collection of notes with a specific quality.
/// The chord is generic over the number of notes it contains, and the type of notes
/// is constrained to be a collection of notes.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_major = major_triad(C4);
/// assert_eq!(c_major.quality(), ChordQuality::MajorTriad);
/// ```
pub struct Chord<const N: usize> {
    quality: ChordQuality,
    notes: [Note; N],
}

impl<const N: usize> Chord<N> {
    /// Creates a new `Chord` with the specified quality and notes
    ///
    /// This constructor takes a chord quality and a collection of notes, and
    /// initializes a new chord. The method is intended for internal use within
    /// the library, as chords are typically created using the specialized
    /// functions like `major_triad` or `minor_triad`.
    ///
    /// # Arguments
    /// * `quality` - The quality (type) of the chord being created
    /// * `notes` - An iterable collection of notes that make up the chord
    ///
    /// # Returns
    /// A new `Chord` instance with the specified quality and notes
    pub(crate) fn new(quality: ChordQuality, notes: impl IntoIterator<Item = Note>) -> Self {
        let mut ns = [C; N];
        for (i, n) in notes.into_iter().enumerate() {
            ns[i] = n;
        }

        Self { quality, notes: ns }
    }

    /// Returns the notes of the chord
    ///
    /// # Returns
    /// A slice of the notes in the chord
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mazzart_ply::*;
    /// use mazzart_ply::constants::*;
    ///
    /// let c_major = major_triad(C4);
    /// assert_eq!(c_major.notes(), &[C4, E4, G4]);
    /// ```
    pub const fn notes(&self) -> &[Note; N] {
        &self.notes
    }

    /// Returns the quality of the chord
    ///
    /// # Returns
    /// The quality of the chord
    ///
    /// # Examples
    ///
    /// ```rust
    /// use mazzart_ply::*;
    /// use mazzart_ply::constants::*;
    ///
    /// let c_major = major_triad(C4);
    /// assert_eq!(c_major.quality(), ChordQuality::MajorTriad);
    /// ```
    pub const fn quality(&self) -> ChordQuality {
        self.quality
    }

    /// Returns the root note of the chord
    ///
    /// # Returns
    /// The root note of the chord
    ///
    /// # Examples
    pub const fn root(&self) -> Note {
        self.notes[0]
    }
}

/// Creates a major triad chord
///
/// This function takes a root note and returns a `Chord<3>` representing a major triad.
/// The major triad is a three-note chord consisting of the root note, a major third,
/// and a perfect fifth.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_major = major_triad(C4);
/// assert_eq!(c_major.quality(), ChordQuality::MajorTriad);
/// ```
pub fn major_triad(root: Note) -> Chord<3> {
    let intervals = MAJOR_TRIAD_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MajorTriad, notes)
}

/// Creates a minor triad chord
///
/// This function takes a root note and returns a `Chord<3>` representing a minor triad.
/// The minor triad is a three-note chord consisting of the root note, a minor third,
/// and a perfect fifth.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let a_minor = minor_triad(A4);
/// assert_eq!(a_minor.quality(), ChordQuality::MinorTriad);
/// ```
pub fn minor_triad(root: Note) -> Chord<3> {
    let intervals = MINOR_TRIAD_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorTriad, notes)
}

/// Creates a diminished triad chord
///
/// This function takes a root note and returns a `Chord<3>` representing a diminished triad.
/// The diminished triad is a three-note chord consisting of the root note, a minor third,
/// and a minor third.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let b_diminished = diminished_triad(B4);
/// assert_eq!(b_diminished.quality(), ChordQuality::DiminishedTriad);
/// ```
pub fn diminished_triad(root: Note) -> Chord<3> {
    let intervals = DIMINISHED_TRIAD_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::DiminishedTriad, notes)
}

/// Creates an augmented triad chord
///
/// This function takes a root note and returns a `Chord<3>` representing an augmented triad.
/// The augmented triad is a three-note chord consisting of the root note, a major third,
/// and a major third.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_augmented = augmented_triad(C4);
/// assert_eq!(c_augmented.quality(), ChordQuality::AugmentedTriad);
/// ```
pub fn augmented_triad(root: Note) -> Chord<3> {
    let intervals = AUGMENTED_TRIAD_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::AugmentedTriad, notes)
}

/// Creates a major seventh chord
///
/// This function takes a root note and returns a `Chord<4>` representing a major seventh chord.
/// The major seventh chord is a four-note chord consisting of the root note, a major third,
/// a perfect fifth, and a major seventh.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_major_seventh = major_seventh(C4);
/// assert_eq!(c_major_seventh.quality(), ChordQuality::MajorSeventh);
/// ```
pub fn major_seventh(root: Note) -> Chord<4> {
    let intervals = MAJOR_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MajorSeventh, notes)
}

/// Creates a dominant seventh chord
///
/// This function takes a root note and returns a `Chord<4>` representing a dominant seventh chord.
/// The dominant seventh chord is a four-note chord consisting of the root note, a major third,
/// a perfect fifth, and a minor seventh.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_dominant_seventh = dominant_seventh(C4);
/// assert_eq!(c_dominant_seventh.quality(), ChordQuality::DominantSeventh);
/// ```
pub fn dominant_seventh(root: Note) -> Chord<4> {
    let intervals = DOMINANT_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::DominantSeventh, notes)
}

/// Creates a minor seventh chord
///
/// This function takes a root note and returns a `Chord<4>` representing a minor seventh chord.
/// The minor seventh chord is a four-note chord consisting of the root note, a minor third,
/// a perfect fifth, and a minor seventh.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_minor_seventh = minor_seventh(C4);
/// assert_eq!(c_minor_seventh.quality(), ChordQuality::MinorSeventh);
/// ```
pub fn minor_seventh(root: Note) -> Chord<4> {
    let intervals = MINOR_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorSeventh, notes)
}

/// Creates a half-diminished seventh chord
///
/// This function takes a root note and returns a `Chord<4>` representing a half-diminished seventh chord.
/// The half-diminished seventh chord is a four-note chord consisting of the root note, a minor third,
/// a diminished fifth, and a minor seventh.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_half_diminished_seventh = half_diminished_seventh(C4);
/// assert_eq!(c_half_diminished_seventh.quality(), ChordQuality::HalfDiminishedSeventh);
/// ```
pub fn half_diminished_seventh(root: Note) -> Chord<4> {
    let intervals = HALF_DIMINISHED_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::HalfDiminishedSeventh, notes)
}

/// Creates a diminished seventh chord
///
/// This function takes a root note and returns a `Chord<4>` representing a diminished seventh chord.
/// The diminished seventh chord is a four-note chord consisting of the root note, a minor third,
/// a diminished fifth, and a major sixth.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_diminished_seventh = diminished_seventh(C4);
/// assert_eq!(c_diminished_seventh.quality(), ChordQuality::DiminishedSeventh);
/// ```
pub fn diminished_seventh(root: Note) -> Chord<4> {
    let intervals = DIMINISHED_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::DiminishedSeventh, notes)
}

/// Creates a suspended 2 chord
///
/// This function takes a root note and returns a `Chord<3>` representing a suspended 2 chord.
/// The suspended 2 chord is a three-note chord consisting of the root note, a major second,
/// and a perfect fifth.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_suspended_2 = sus2(C4);
/// assert_eq!(c_suspended_2.quality(), ChordQuality::Sus2);
/// ```
pub fn sus2(root: Note) -> Chord<3> {
    let intervals = SUS2_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::Sus2, notes)
}

/// Creates a suspended 4 chord
///
/// This function takes a root note and returns a `Chord<3>` representing a suspended 4 chord.
/// The suspended 4 chord is a three-note chord consisting of the root note, a perfect fourth,
/// and a perfect fifth.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_suspended_4 = sus4(C4);
/// assert_eq!(c_suspended_4.quality(), ChordQuality::Sus4);
/// ```
pub fn sus4(root: Note) -> Chord<3> {
    let intervals = SUS4_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::Sus4, notes)
}

/// Creates a major sixth chord
///
/// This function takes a root note and returns a `Chord<3>` representing a major sixth chord.
/// The major sixth chord is a three-note chord consisting of the root note, a major third,
/// and a major sixth.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_major_sixth = major_sixth(C4);
/// assert_eq!(c_major_sixth.quality(), ChordQuality::MajorSixth);
/// ```
pub fn major_sixth(root: Note) -> Chord<3> {
    let intervals = MAJOR_SIXTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MajorSixth, notes)
}

/// Creates a minor sixth chord
///
/// This function takes a root note and returns a `Chord<3>` representing a minor sixth chord.
/// The minor sixth chord is a three-note chord consisting of the root note, a minor third,
/// and a major sixth.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_minor_sixth = minor_sixth(C4);
/// assert_eq!(c_minor_sixth.quality(), ChordQuality::MinorSixth);
/// ```
pub fn minor_sixth(root: Note) -> Chord<3> {
    let intervals = MINOR_SIXTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorSixth, notes)
}

/// Creates a major sixth ninth chord
///
/// This function takes a root note and returns a `Chord<4>` representing a major sixth ninth chord.
/// The major sixth ninth chord is a four-note chord consisting of the root note, a major third,
/// a major sixth, and a major ninth.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
/// let c_major_sixth_ninth = major_sixth_ninth(C4);
/// assert_eq!(c_major_sixth_ninth.quality(), ChordQuality::MajorSixthNinth);
/// ```
pub fn major_sixth_ninth(root: Note) -> Chord<4> {
    let intervals = MAJOR_SIXTH_NINTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MajorSixthNinth, notes)
}

/// Creates a minor sixth ninth chord
///
/// This function takes a root note and returns a `Chord<4>` representing a minor sixth ninth chord.
/// The minor sixth ninth chord is a four-note chord consisting of the root note, a minor third,
/// a major sixth, and a major ninth.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_minor_sixth_ninth = minor_sixth_ninth(C4);
/// assert_eq!(c_minor_sixth_ninth.quality(), ChordQuality::MinorSixthNinth);
/// ```
pub fn minor_sixth_ninth(root: Note) -> Chord<4> {
    let intervals = MINOR_SIXTH_NINTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorSixthNinth, notes)
}

/// Creates a dominant seventh ninth chord
///
/// This function takes a root note and returns a `Chord<5>` representing a dominant seventh ninth chord.
/// The dominant seventh ninth chord is a five-note chord consisting of the root note, a major third,
/// a perfect fifth, a minor seventh, and a major ninth.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_dominant_seventh_ninth = dominant_seventh_ninth(C4);
/// assert_eq!(c_dominant_seventh_ninth.quality(), ChordQuality::DominantSeventhNinth);
/// ```
pub fn dominant_seventh_ninth(root: Note) -> Chord<5> {
    let intervals = DOMINANT_SEVENTH_NINTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::DominantSeventhNinth, notes)
}

/// Creates a minor seventh ninth chord
///
/// This function takes a root note and returns a `Chord<5>` representing a minor seventh ninth chord.
/// The minor seventh ninth chord is a five-note chord consisting of the root note, a minor third,
/// a perfect fifth, a minor seventh, and a major ninth.
///
/// # Examples
///
/// ```rust
/// use mazzart_ply::*;
/// use mazzart_ply::constants::*;
///
/// let c_minor_seventh_ninth = minor_seventh_ninth(C4);
/// assert_eq!(c_minor_seventh_ninth.quality(), ChordQuality::MinorSeventhNinth);
/// ```
pub fn minor_seventh_ninth(root: Note) -> Chord<5> {
    let intervals = MINOR_SEVENTH_NINTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorSeventhNinth, notes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_triad() {
        let c_major = major_triad(C4);
        assert_eq!(c_major.quality(), ChordQuality::MajorTriad);
        assert_eq!(c_major.root(), C4);
        assert_eq!(c_major.notes()[0], C4);
        assert_eq!(c_major.notes()[1], E4);
        assert_eq!(c_major.notes()[2], G4);
    }

    #[test]
    fn test_minor_triad() {
        let a_minor = minor_triad(A4);
        assert_eq!(a_minor.quality(), ChordQuality::MinorTriad);
        assert_eq!(a_minor.root(), A4);
        assert_eq!(a_minor.notes()[0], A4);
    }
}
