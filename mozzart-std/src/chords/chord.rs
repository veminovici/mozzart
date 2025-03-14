use crate::constants::*;
use crate::Note;
use std::fmt;

/// Represents the quality of a chord
///
/// This enum defines the different types of chords that can be created.
/// Each variant corresponds to a specific chord type, and the enum is used
/// to determine the quality of a chord.
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_major = major_triad(C4);
/// assert_eq!(c_major.quality(), ChordQuality::MajorTriad);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChordQuality {
    MajorTriad,
    MinorTriad,
    DominantSeventh,
    DominantSeventhNinth,
    MinorSeventh,
    MinorSeventhNinth,
    MajorSeventh,
    MinorMajorSeventh,
    MajorSixth,
    MinorSixth,
    MajorSixthNinth,
    MinorSixthNinth,
    Sus2,
    Sus4,
    DiminishedTriad,
    DiminishedSeventh,
    HalfDiminishedSeventh,
    AugmentedTriad,
    AugmentedSeventh,
    DominantNinth,
    MinorNinth,
    MajorNinth,
    DominantEleventh,
    MinorEleventh,
    MajorEleventh,
    DominantThirteenth,
    MinorThirteenth,
    MajorThirteenth,
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
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
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
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
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let a_minor = minor_triad(A4);
/// assert_eq!(a_minor.quality(), ChordQuality::MinorTriad);
/// ```
pub fn minor_triad(root: Note) -> Chord<3> {
    let intervals = MINOR_TRIAD_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorTriad, notes)
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_dominant_seventh = dominant_seventh(C4);
/// assert_eq!(c_dominant_seventh.quality(), ChordQuality::DominantSeventh);
/// ```
pub fn dominant_seventh(root: Note) -> Chord<4> {
    let intervals = DOMINANT_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::DominantSeventh, notes)
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_dominant_seventh_ninth = dominant_seventh_ninth(C4);
/// assert_eq!(c_dominant_seventh_ninth.quality(), ChordQuality::DominantSeventhNinth);
/// ```
pub fn dominant_seventh_ninth(root: Note) -> Chord<5> {
    let intervals = DOMINANT_SEVENTH_NINTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::DominantSeventhNinth, notes)
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_major_seventh = major_seventh(C4);
/// assert_eq!(c_major_seventh.quality(), ChordQuality::MajorSeventh);
/// ```
pub fn major_seventh(root: Note) -> Chord<4> {
    let intervals = MAJOR_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MajorSeventh, notes)
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_minor_seventh = minor_seventh(C4);
/// assert_eq!(c_minor_seventh.quality(), ChordQuality::MinorSeventh);
/// ```
pub fn minor_seventh(root: Note) -> Chord<4> {
    let intervals = MINOR_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorSeventh, notes)
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_minor_seventh_ninth = minor_seventh_ninth(C4);
/// assert_eq!(c_minor_seventh_ninth.quality(), ChordQuality::MinorSeventhNinth);
/// ```
pub fn minor_seventh_ninth(root: Note) -> Chord<5> {
    let intervals = MINOR_SEVENTH_NINTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorSeventhNinth, notes)
}

/// Creates a minor major seventh chord
///
/// This function takes a root note and returns a `Chord<4>` representing a minor major seventh chord.
/// The minor major seventh chord is a four-note chord consisting of the root note, a minor third,
/// a perfect fifth, and a major seventh.
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_minor_major_seventh = minor_major_seventh(C4);
/// assert_eq!(c_minor_major_seventh.quality(), ChordQuality::MinorMajorSeventh);
/// ```
pub fn minor_major_seventh(root: Note) -> Chord<4> {
    let intervals = MINOR_MAJOR_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorMajorSeventh, notes)
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_major_sixth = major_sixth(C4);
/// assert_eq!(c_major_sixth.quality(), ChordQuality::MajorSixth);
/// ```
pub fn major_sixth(root: Note) -> Chord<4> {
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_minor_sixth = minor_sixth(C4);
/// assert_eq!(c_minor_sixth.quality(), ChordQuality::MinorSixth);
/// ```
pub fn minor_sixth(root: Note) -> Chord<4> {
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
/// let c_major_sixth_ninth = major_sixth_ninth(C4);
/// assert_eq!(c_major_sixth_ninth.quality(), ChordQuality::MajorSixthNinth);
/// ```
pub fn major_sixth_ninth(root: Note) -> Chord<5> {
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_minor_sixth_ninth = minor_sixth_ninth(C4);
/// assert_eq!(c_minor_sixth_ninth.quality(), ChordQuality::MinorSixthNinth);
/// ```
pub fn minor_sixth_ninth(root: Note) -> Chord<5> {
    let intervals = MINOR_SIXTH_NINTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorSixthNinth, notes)
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_suspended_4 = sus4(C4);
/// assert_eq!(c_suspended_4.quality(), ChordQuality::Sus4);
/// ```
pub fn sus4(root: Note) -> Chord<3> {
    let intervals = SUS4_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::Sus4, notes)
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let b_diminished = diminished_triad(B4);
/// assert_eq!(b_diminished.quality(), ChordQuality::DiminishedTriad);
/// ```
pub fn diminished_triad(root: Note) -> Chord<3> {
    let intervals = DIMINISHED_TRIAD_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::DiminishedTriad, notes)
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_diminished_seventh = diminished_seventh(C4);
/// assert_eq!(c_diminished_seventh.quality(), ChordQuality::DiminishedSeventh);
/// ```
pub fn diminished_seventh(root: Note) -> Chord<4> {
    let intervals = DIMINISHED_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::DiminishedSeventh, notes)
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_half_diminished_seventh = half_diminished_seventh(C4);
/// assert_eq!(c_half_diminished_seventh.quality(), ChordQuality::HalfDiminishedSeventh);
/// ```
pub fn half_diminished_seventh(root: Note) -> Chord<4> {
    let intervals = HALF_DIMINISHED_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::HalfDiminishedSeventh, notes)
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
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_augmented = augmented_triad(C4);
/// assert_eq!(c_augmented.quality(), ChordQuality::AugmentedTriad);
/// ```
pub fn augmented_triad(root: Note) -> Chord<3> {
    let intervals = AUGMENTED_TRIAD_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::AugmentedTriad, notes)
}

/// Creates an augmented seventh chord
///
/// This function takes a root note and returns a `Chord<4>` representing an augmented seventh chord.
/// The augmented seventh chord is a four-note chord consisting of the root note, a major third,
/// an augmented fifth, and an augmented seventh.
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
/// ```
pub fn augmented_seventh(root: Note) -> Chord<4> {
    let intervals = AUGMENTED_SEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::AugmentedSeventh, notes)
}

/// Creates a dominant ninth chord
///
/// This function takes a root note and returns a `Chord<5>` representing a dominant ninth chord.
/// The dominant ninth chord is a five-note chord consisting of the root note, a major third,
/// a perfect fifth, a minor seventh, and a major ninth.
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_dominant_ninth = dominant_ninth(C4);
/// assert_eq!(c_dominant_ninth.quality(), ChordQuality::DominantNinth);
/// ```
pub fn dominant_ninth(root: Note) -> Chord<5> {
    let intervals = DOMINANT_NINTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::DominantNinth, notes)
}

/// Creates a minor ninth chord
///
/// This function takes a root note and returns a `Chord<5>` representing a minor ninth chord.
/// The minor ninth chord is a five-note chord consisting of the root note, a minor third,
/// a perfect fifth, a minor seventh, and a major ninth.
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_minor_ninth = minor_ninth(C4);
/// assert_eq!(c_minor_ninth.quality(), ChordQuality::MinorNinth);
/// ```
pub fn minor_ninth(root: Note) -> Chord<5> {
    let intervals = MINOR_NINTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorNinth, notes)
}

/// Creates a major ninth chord
///
/// This function takes a root note and returns a `Chord<5>` representing a major ninth chord.
/// The major ninth chord is a five-note chord consisting of the root note, a major third,
/// a perfect fifth, a major seventh, and a major ninth.
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_major_ninth = major_ninth(C4);
/// assert_eq!(c_major_ninth.quality(), ChordQuality::MajorNinth);
/// ```
pub fn major_ninth(root: Note) -> Chord<5> {
    let intervals = MAJOR_NINTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MajorNinth, notes)
}

/// Creates a dominant eleventh chord
///
/// This function takes a root note and returns a `Chord<6>` representing a dominant eleventh chord.
/// The dominant eleventh chord is a six-note chord consisting of the root note, a major third,
/// a perfect fifth, a minor seventh, a major ninth, and a major eleventh.
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_dominant_eleventh = dominant_eleventh(C4);
/// assert_eq!(c_dominant_eleventh.quality(), ChordQuality::DominantEleventh);
/// ```
pub fn dominant_eleventh(root: Note) -> Chord<6> {
    let intervals = DOMINANT_ELEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::DominantEleventh, notes)
}

/// Creates a minor eleventh chord
///
/// This function takes a root note and returns a `Chord<6>` representing a minor eleventh chord.
/// The minor eleventh chord is a six-note chord consisting of the root note, a minor third,
/// a perfect fifth, a minor seventh, a major ninth, and a perfect eleventh.
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_minor_eleventh = minor_eleventh(C4);
/// assert_eq!(c_minor_eleventh.quality(), ChordQuality::MinorEleventh);
/// ```
pub fn minor_eleventh(root: Note) -> Chord<6> {
    let intervals = MINOR_ELEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorEleventh, notes)
}

/// Creates a major eleventh chord
///
/// This function takes a root note and returns a `Chord<6>` representing a major eleventh chord.
/// The major eleventh chord is a six-note chord consisting of the root note, a major third,
/// a perfect fifth, a major seventh, a major ninth, and a perfect eleventh.
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_major_eleventh = major_eleventh(C4);
/// assert_eq!(c_major_eleventh.quality(), ChordQuality::MajorEleventh);
/// ```
pub fn major_eleventh(root: Note) -> Chord<6> {
    let intervals = MAJOR_ELEVENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MajorEleventh, notes)
}

/// Creates a dominant thirteenth chord
///
/// This function takes a root note and returns a `Chord<7>` representing a dominant thirteenth chord.
/// The dominant thirteenth chord is a seven-note chord consisting of the root note, a major third,
/// a perfect fifth, a minor seventh, a major ninth, a perfect eleventh, and a major thirteenth.
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_dominant_thirteenth = dominant_thirteenth(C4);
/// assert_eq!(c_dominant_thirteenth.quality(), ChordQuality::DominantThirteenth);
/// ```
pub fn dominant_thirteenth(root: Note) -> Chord<7> {
    let intervals = DOMINANT_THIRTEENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::DominantThirteenth, notes)
}

/// Creates a minor thirteenth chord
///
/// This function takes a root note and returns a `Chord<7>` representing a minor thirteenth chord.
/// The minor thirteenth chord is a seven-note chord consisting of the root note, a minor third,
/// a perfect fifth, a minor seventh, a major ninth, a perfect eleventh, and a minor thirteenth.    
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_minor_thirteenth = minor_thirteenth(C4);
/// assert_eq!(c_minor_thirteenth.quality(), ChordQuality::MinorThirteenth);
/// ```
pub fn minor_thirteenth(root: Note) -> Chord<7> {
    let intervals = MINOR_THIRTEENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MinorThirteenth, notes)
}

/// Creates a major thirteenth chord
///
/// This function takes a root note and returns a `Chord<7>` representing a major thirteenth chord.
/// The major thirteenth chord is a seven-note chord consisting of the root note, a major third,
/// a perfect fifth, a major seventh, a major ninth, a perfect eleventh, and a major thirteenth.
///
/// # Examples
///
/// ```rust
/// use mozzart_std::*;
/// use mozzart_std::constants::*;
///
/// let c_major_thirteenth = major_thirteenth(C4);
/// assert_eq!(c_major_thirteenth.quality(), ChordQuality::MajorThirteenth);
/// ```
pub fn major_thirteenth(root: Note) -> Chord<7> {
    let intervals = MAJOR_THIRTEENTH_INTERVALS;
    let notes = root.into_notes_from_intervals(intervals);
    Chord::new(ChordQuality::MajorThirteenth, notes)
}

/// Returns the suffix for a chord
///
/// This function takes a `ChordQuality` and returns the appropriate suffix for the chord.
///
/// # Arguments
/// * `quality` - The quality (type) of the chord
///
/// # Returns
/// The suffix for the chord
///
/// # Examples
/// ```ignore
/// use mozzart_std::ChordQuality;
/// use mozzart_std::constants::*;
///
/// assert_eq!(chord_suffix(ChordQuality::MajorTriad), "");
/// assert_eq!(chord_suffix(ChordQuality::MinorTriad), "m");
/// assert_eq!(chord_suffix(ChordQuality::DominantSeventh), "7");
/// assert_eq!(chord_suffix(ChordQuality::MinorSeventh), "m7");
/// assert_eq!(chord_suffix(ChordQuality::MajorSeventh), "maj7");
/// assert_eq!(chord_suffix(ChordQuality::MinorMajorSeventh), "mM7");
/// assert_eq!(chord_suffix(ChordQuality::MajorSixth), "6");
/// assert_eq!(chord_suffix(ChordQuality::MinorSixth), "m6");
/// assert_eq!(chord_suffix(ChordQuality::MajorSixthNinth), "6/9");
/// assert_eq!(chord_suffix(ChordQuality::MinorSixthNinth), "m6/9");
/// ```
fn chord_suffix(quality: ChordQuality) -> &'static str {
    match quality {
        ChordQuality::MajorTriad => "",
        ChordQuality::MinorTriad => "m",
        ChordQuality::DominantSeventh => "7",
        ChordQuality::MinorSeventh => "m7",
        ChordQuality::MajorSeventh => "maj7",
        ChordQuality::MinorMajorSeventh => "mM7",
        ChordQuality::MajorSixth => "6",
        ChordQuality::MinorSixth => "m6",
        ChordQuality::MajorSixthNinth => "6/9",
        ChordQuality::MinorSixthNinth => "m6/9",
        ChordQuality::Sus2 => "sus2",
        ChordQuality::Sus4 => "sus4",
        ChordQuality::DiminishedTriad => "dim",
        ChordQuality::DiminishedSeventh => "dim7",
        ChordQuality::DominantSeventhNinth => "7/9",
        ChordQuality::MinorSeventhNinth => "m7/9",
        ChordQuality::HalfDiminishedSeventh => "hdim7",
        ChordQuality::AugmentedTriad => "aug",
        ChordQuality::AugmentedSeventh => "aug7",
        ChordQuality::DominantNinth => "9",
        ChordQuality::MinorNinth => "m9",
        ChordQuality::MajorNinth => "maj9",
        ChordQuality::DominantEleventh => "11",
        ChordQuality::MinorEleventh => "m11",
        ChordQuality::MajorEleventh => "maj11",
        ChordQuality::DominantThirteenth => "13",
        ChordQuality::MinorThirteenth => "m13",
        ChordQuality::MajorThirteenth => "maj13",
    }
}

impl<const N: usize> fmt::UpperHex for Chord<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let root = self.root();
        let suffix = chord_suffix(self.quality());
        write!(f, "{root:X}{suffix}")
    }
}

impl<const N: usize> fmt::LowerHex for Chord<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let root = self.root();
        let suffix = chord_suffix(self.quality());
        write!(f, "{root:x}{suffix}")
    }
}

impl<const N: usize> fmt::Display for Chord<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let root = self.root();
        let suffix = chord_suffix(self.quality());
        write!(f, "{root}{suffix}")
    }
}

impl<const N: usize> fmt::Debug for Chord<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let root = self.root();
        let suffix = chord_suffix(self.quality());
        write!(f, "{root:?}{suffix}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_triad() {
        let scale = major_triad(C4);
        assert_eq!(scale.quality(), ChordQuality::MajorTriad);
        assert_eq!(scale.notes().len(), 3);
        assert_eq!(scale.notes(), &[C4, E4, G4]);
        assert_eq!(format!("{}", scale), "C");
    }

    #[test]
    fn test_minor_triad() {
        let scale = minor_triad(C4);
        assert_eq!(scale.quality(), ChordQuality::MinorTriad);
        assert_eq!(scale.notes().len(), 3);
        assert_eq!(scale.notes(), &[C4, DSHARP4, G4]);
        assert_eq!(format!("{}", scale), "Cm");
    }

    #[test]
    fn test_dominant_seventh() {
        let scale = dominant_seventh(C4);
        assert_eq!(scale.quality(), ChordQuality::DominantSeventh);
        assert_eq!(scale.notes().len(), 4);
        assert_eq!(scale.notes(), &[C4, E4, G4, BFLAT4]);
        assert_eq!(format!("{}", scale), "C7");
    }

    #[test]
    fn test_dominant_seventh_ninth() {
        let scale = dominant_seventh_ninth(C4);
        assert_eq!(scale.quality(), ChordQuality::DominantSeventhNinth);
        assert_eq!(scale.notes().len(), 5);
        assert_eq!(scale.notes(), &[C4, E4, G4, BFLAT4, D5]);
        assert_eq!(format!("{}", scale), "C7/9");
    }

    #[test]
    fn test_minor_seventh() {
        let scale = minor_seventh(C4);
        assert_eq!(scale.quality(), ChordQuality::MinorSeventh);
        assert_eq!(scale.notes().len(), 4);
        assert_eq!(scale.notes(), &[C4, EFLAT4, G4, BFLAT4]);
        assert_eq!(format!("{}", scale), "Cm7");
    }

    #[test]
    fn test_minor_seventh_ninth() {
        let scale = minor_seventh_ninth(C4);
        assert_eq!(scale.quality(), ChordQuality::MinorSeventhNinth);
        assert_eq!(scale.notes().len(), 5);
        assert_eq!(scale.notes(), &[C4, EFLAT4, G4, BFLAT4, D5]);
        assert_eq!(format!("{}", scale), "Cm7/9");
    }

    #[test]
    fn test_major_seventh() {
        let scale = major_seventh(C4);
        assert_eq!(scale.quality(), ChordQuality::MajorSeventh);
        assert_eq!(scale.notes().len(), 4);
        assert_eq!(scale.notes(), &[C4, E4, G4, B4]);
        assert_eq!(format!("{}", scale), "Cmaj7");
    }

    #[test]
    fn test_minor_major_seventh() {
        let scale = minor_major_seventh(C4);
        assert_eq!(scale.quality(), ChordQuality::MinorMajorSeventh);
        assert_eq!(scale.notes().len(), 4);
        assert_eq!(scale.notes(), &[C4, EFLAT4, G4, B4]);
        assert_eq!(format!("{}", scale), "CmM7");
    }

    #[test]
    fn test_major_sixth() {
        let scale = major_sixth(C4);
        assert_eq!(scale.quality(), ChordQuality::MajorSixth);
        assert_eq!(scale.notes().len(), 4);
        assert_eq!(scale.notes(), &[C4, E4, G4, A4]);
        assert_eq!(format!("{}", scale), "C6");
    }

    #[test]
    fn test_minor_sixth() {
        let scale = minor_sixth(C4);
        assert_eq!(scale.quality(), ChordQuality::MinorSixth);
        assert_eq!(scale.notes().len(), 4);
        assert_eq!(scale.notes(), &[C4, EFLAT4, G4, A4]);
        assert_eq!(format!("{}", scale), "Cm6");
    }

    #[test]
    fn test_major_sixth_ninth() {
        let scale = major_sixth_ninth(C4);
        assert_eq!(scale.quality(), ChordQuality::MajorSixthNinth);
        assert_eq!(scale.notes().len(), 5);
        assert_eq!(scale.notes(), &[C4, E4, G4, A4, D5]);
        assert_eq!(format!("{}", scale), "C6/9");
    }

    #[test]
    fn test_minor_sixth_ninth() {
        let scale = minor_sixth_ninth(C4);
        assert_eq!(scale.quality(), ChordQuality::MinorSixthNinth);
        assert_eq!(scale.notes().len(), 5);
        assert_eq!(scale.notes(), &[C4, EFLAT4, G4, A4, D5]);
        assert_eq!(format!("{}", scale), "Cm6/9");
    }

    #[test]
    fn test_sus2() {
        let scale = sus2(C4);
        assert_eq!(scale.quality(), ChordQuality::Sus2);
        assert_eq!(scale.notes().len(), 3);
        assert_eq!(scale.notes(), &[C4, D4, G4]);
        assert_eq!(format!("{}", scale), "Csus2");
    }

    #[test]
    fn test_sus4() {
        let scale = sus4(C4);
        assert_eq!(scale.quality(), ChordQuality::Sus4);
        assert_eq!(scale.notes().len(), 3);
        assert_eq!(scale.notes(), &[C4, F4, G4]);
        assert_eq!(format!("{}", scale), "Csus4");
    }

    #[test]
    fn test_diminished_triad() {
        let scale = diminished_triad(C4);
        assert_eq!(scale.quality(), ChordQuality::DiminishedTriad);
        assert_eq!(scale.notes().len(), 3);
        assert_eq!(scale.notes(), &[C4, EFLAT4, GFLAT4]);
        assert_eq!(format!("{}", scale), "Cdim");
    }

    #[test]
    fn test_diminished_seventh() {
        let scale = diminished_seventh(C4);
        assert_eq!(scale.quality(), ChordQuality::DiminishedSeventh);
        assert_eq!(scale.notes().len(), 4);
        assert_eq!(scale.notes(), &[C4, EFLAT4, GFLAT4, A4]);
        assert_eq!(format!("{}", scale), "Cdim7");
    }

    #[test]
    fn test_half_diminished_seventh() {
        let scale = half_diminished_seventh(C4);
        assert_eq!(scale.quality(), ChordQuality::HalfDiminishedSeventh);
        assert_eq!(scale.notes().len(), 4);
        assert_eq!(scale.notes(), &[C4, EFLAT4, GFLAT4, BFLAT4]);
        assert_eq!(format!("{}", scale), "Chdim7");
    }

    #[test]
    fn test_augmented_triad() {
        let scale = augmented_triad(C4);
        assert_eq!(scale.quality(), ChordQuality::AugmentedTriad);
        assert_eq!(scale.notes().len(), 3);
        assert_eq!(scale.notes(), &[C4, E4, GSHARP4]);
        assert_eq!(format!("{}", scale), "Caug");
    }

    #[test]
    fn test_augmented_seventh() {
        let scale = augmented_seventh(C4);
        assert_eq!(scale.quality(), ChordQuality::AugmentedSeventh);
        assert_eq!(scale.notes().len(), 4);
        assert_eq!(scale.notes(), &[C4, E4, GSHARP4, BFLAT4]);
        assert_eq!(format!("{}", scale), "Caug7");
    }

    #[test]
    fn test_dominant_ninth() {
        let scale = dominant_ninth(C4);
        assert_eq!(scale.quality(), ChordQuality::DominantNinth);
        assert_eq!(scale.notes().len(), 5);
        assert_eq!(scale.notes(), &[C4, E4, G4, BFLAT4, D5]);
        assert_eq!(format!("{}", scale), "C9");
    }

    #[test]
    fn test_minor_ninth() {
        let scale = minor_ninth(C4);
        assert_eq!(scale.quality(), ChordQuality::MinorNinth);
        assert_eq!(scale.notes().len(), 5);
        assert_eq!(scale.notes(), &[C4, EFLAT4, G4, BFLAT4, D5]);
        assert_eq!(format!("{}", scale), "Cm9");
    }

    #[test]
    fn test_major_ninth() {
        let scale = major_ninth(C4);
        assert_eq!(scale.quality(), ChordQuality::MajorNinth);
        assert_eq!(scale.notes().len(), 5);
        assert_eq!(scale.notes(), &[C4, E4, G4, B4, D5]);
        assert_eq!(format!("{}", scale), "Cmaj9");
    }

    #[test]
    fn test_dominant_eleventh() {
        let scale = dominant_eleventh(C4);
        assert_eq!(scale.quality(), ChordQuality::DominantEleventh);
        assert_eq!(scale.notes().len(), 6);
        assert_eq!(scale.notes(), &[C4, E4, G4, BFLAT4, D5, F5]);
        assert_eq!(format!("{}", scale), "C11");
    }

    #[test]
    fn test_minor_eleventh() {
        let scale = minor_eleventh(C4);
        assert_eq!(scale.quality(), ChordQuality::MinorEleventh);
        assert_eq!(scale.notes().len(), 6);
        assert_eq!(scale.notes(), &[C4, EFLAT4, G4, BFLAT4, D5, F5]);
        assert_eq!(format!("{}", scale), "Cm11");
    }

    #[test]
    fn test_major_eleventh() {
        let scale = major_eleventh(C4);
        assert_eq!(scale.quality(), ChordQuality::MajorEleventh);
        assert_eq!(scale.notes().len(), 6);
        assert_eq!(scale.notes(), &[C4, E4, G4, B4, D5, F5]);
        assert_eq!(format!("{}", scale), "Cmaj11");
    }

    #[test]
    fn test_dominant_thirteenth() {
        let scale = dominant_thirteenth(C4);
        assert_eq!(scale.quality(), ChordQuality::DominantThirteenth);
        assert_eq!(scale.notes().len(), 7);
        assert_eq!(scale.notes(), &[C4, E4, G4, BFLAT4, D5, F5, A5]);
        assert_eq!(format!("{}", scale), "C13");
    }

    #[test]
    fn test_minor_thirteenth() {
        let scale = minor_thirteenth(C4);
        assert_eq!(scale.quality(), ChordQuality::MinorThirteenth);
        assert_eq!(scale.notes().len(), 7);
        assert_eq!(scale.notes(), &[C4, EFLAT4, G4, BFLAT4, D5, F5, A5]);
        assert_eq!(format!("{}", scale), "Cm13");
    }

    #[test]
    fn test_major_thirteenth() {
        let scale = major_thirteenth(C4);
        assert_eq!(scale.quality(), ChordQuality::MajorThirteenth);
        assert_eq!(scale.notes().len(), 7);
        assert_eq!(scale.notes(), &[C4, E4, G4, B4, D5, F5, A5]);
        assert_eq!(format!("{}", scale), "Cmaj13");
    }
}
