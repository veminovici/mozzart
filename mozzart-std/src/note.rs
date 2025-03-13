use crate::*;
use std::ops::{Add, AddAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};

/// Represents a musical note using MIDI note numbering
///
/// The `Note` struct encapsulates a MIDI note number (0-127), where:
/// - 0 represents C-1 (the lowest note in the MIDI standard)
/// - 60 represents middle C (C4)
/// - 127 represents G9 (the highest note in the MIDI standard)
///
/// This type allows for musical operations like:
/// - Adding and subtracting intervals
/// - Shifting by octaves
/// - Comparing pitches
///
/// MIDI note numbers provide a convenient and standard way to represent
/// pitches across all octaves without dealing with the complexities
/// of frequency calculations or letter-based note naming.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct Note(u8);

impl Note {
    /// Creates a new `Note` from a MIDI note number
    ///
    /// # Arguments
    /// * `note` - A MIDI note number (0-127)
    ///
    /// # Returns
    /// A new `Note` instance
    ///
    /// # Examples
    /// ```ignore
    /// use mozzart_std::Note;
    ///
    /// let middle_c = Note::new(60);  // C4
    /// let a440 = Note::new(69);      // A4 (standard tuning reference at 440Hz)
    /// ```
    pub(crate) const fn new(note: u8) -> Self {
        Self(note)
    }

    /// Generates a sequence of notes starting from this note and following the specified interval steps
    ///
    /// This method creates an iterator that yields a sequence of notes, where:
    /// - The first note is the current note (self)
    /// - Each subsequent note is derived by adding the corresponding interval to the previous note
    ///
    /// This is particularly useful for:
    /// - Creating scales by providing scale step patterns
    /// - Building arpeggios from interval patterns
    /// - Generating melodic sequences from a starting note
    ///
    /// # Arguments
    /// * `steps` - An iterable collection of intervals defining the steps between adjacent notes
    ///
    /// # Returns
    /// An iterator yielding notes, starting with the root note (self) followed by
    /// each subsequent note derived by applying the intervals in sequence.
    ///
    /// # Examples
    /// ```ignore
    /// use mozzart_std::constants::*;
    ///
    /// // Create a C major scale using whole and half steps
    /// let c4 = C4;
    /// let major_steps = [WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF];
    /// let c_major_scale: Vec<_> = c4.from_steps(major_steps).collect();
    ///
    /// // The result should be C4, D4, E4, F4, G4, A4, B4, C5
    /// assert_eq!(c_major_scale, vec![C4, D4, E4, F4, G4, A4, B4, C5]);
    /// ```
    fn notes_from_steps<S>(&self, steps: S) -> impl Iterator<Item = Self>
    where
        S: IntoIterator<Item = Step>,
    {
        let root = *self;

        std::iter::once(root).chain(steps.into_iter().scan(root, |note, step| {
            *note += step;
            Some(*note)
        }))
    }

    /// Generates a sequence of notes by applying each interval directly to this note
    ///
    /// Unlike `notes_from_steps` which builds notes cumulatively (where each note depends on the previous one),
    /// this method applies each interval directly to the root note. The resulting sequence consists of:
    /// - The original note (self) as the first element
    /// - For each interval, a note that is that interval above the original note
    ///
    /// This is particularly useful for:
    /// - Creating chords where each note has a specific interval relationship to the root
    /// - Generating harmonic series based on fixed intervals from a fundamental
    /// - Building complex sonorities with precise interval relationships
    ///
    /// # Arguments
    /// * `intervals` - An iterable collection of intervals, each measured from the root note
    ///
    /// # Returns
    /// An iterator yielding notes, starting with the root note (self) followed by
    /// notes at specified intervals above the root.
    ///
    /// # Examples
    /// ```ignore
    /// use mozzart_std::constants::*;
    ///
    /// // Create a C major chord using fixed intervals from the root
    /// let c4 = C4;
    /// let intervals = [MAJOR_THIRD, PERFECT_FIFTH];
    /// let c_major_chord: Vec<_> = c4.notes_from_intervals(intervals).collect();
    ///
    /// // The result should be C4, E4, G4
    /// assert_eq!(c_major_chord, vec![C4, E4, G4]);
    /// ```
    fn notes_from_intervals<'a, I>(&'a self, intervals: I) -> impl Iterator<Item = Self> + 'a
    where
        I: IntoIterator<Item = Interval>,
        <I as IntoIterator>::IntoIter: 'a,
    {
        std::iter::once(*self).chain(intervals.into_iter().map(|interval| *self + interval))
    }

    /// Generates a sequence of notes starting from this note and following the specified interval steps
    ///
    /// This is a consuming version of `from_steps` that takes ownership of the note.
    ///
    /// This method creates an iterator that yields a sequence of notes, where:
    /// - The first note is the current note (self)
    /// - Each subsequent note is derived by adding the corresponding interval to the previous note
    ///
    /// This method is useful when you don't need to keep the original note around after
    /// generating the sequence.
    ///
    /// # Arguments
    /// * `steps` - An iterable collection of intervals defining the steps between adjacent notes
    ///
    /// # Returns
    /// An iterator yielding notes, starting with the root note (self) followed by
    /// each subsequent note derived by applying the intervals in sequence.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// // Create a C minor triad using intervals
    /// let c_minor_triad: Vec<_> = C4.into_notes_from_steps([MINOR_THIRD.into(), MAJOR_THIRD.into()]).collect();
    ///
    /// // The result should be C4, Eb4, G4
    /// assert_eq!(c_minor_triad, vec![C4, DSHARP4, G4]);
    /// ```
    #[inline]
    pub fn into_notes_from_steps<S>(self, steps: S) -> impl Iterator<Item = Self>
    where
        S: IntoIterator<Item = Step>,
    {
        self.notes_from_steps(steps)
    }

    /// Generates a sequence of notes by applying each interval directly to this note
    ///
    /// This method creates an iterator that yields a sequence of notes where each note
    /// has a specific interval relationship to the root note (self). The sequence consists of:
    /// - The original note (self) as the first element
    /// - For each interval, a note that is that interval above the original note
    ///
    /// Unlike `into_notes_from_steps` which builds notes by adding intervals cumulatively,
    /// this method maintains a consistent reference point (the root note) for all intervals.
    ///
    /// # Arguments
    /// * `intervals` - An iterable collection of intervals, each measured from the root note
    ///
    /// # Returns
    /// An iterator yielding notes, starting with the root note (self) followed by
    /// notes at specified intervals above the root.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::constants::*;
    ///
    /// // Create a C major chord (C, E, G)
    /// let c_major_chord: Vec<_> = C4.into_notes_from_intervals([MAJOR_THIRD, PERFECT_FIFTH]).collect();
    /// assert_eq!(c_major_chord, vec![C4, E4, G4]);
    ///
    /// // Create a C dominant seventh chord (C, E, G, Bb)
    /// let c7_chord: Vec<_> = C4.into_notes_from_intervals([
    ///     MAJOR_THIRD,      // E (major third above C)
    ///     PERFECT_FIFTH,    // G (perfect fifth above C)
    ///     MINOR_SEVENTH     // Bb (minor seventh above C)
    /// ]).collect();
    /// assert_eq!(c7_chord.len(), 4);
    /// assert_eq!(c7_chord[3], ASHARP4);  // Bb4
    /// ```
    #[inline]
    pub fn into_notes_from_intervals<'a, I>(
        &'a self,
        intervals: I,
    ) -> impl Iterator<Item = Self> + 'a
    where
        I: IntoIterator<Item = Interval>,
        <I as IntoIterator>::IntoIter: 'a,
    {
        self.notes_from_intervals(intervals)
    }

    /// Returns the MIDI note number for this note
    ///
    /// # Returns
    /// The raw MIDI note number (0-127)
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::constants::*;
    ///
    /// assert_eq!(C4.midi_number(), 60);
    /// assert_eq!(A4.midi_number(), 69);
    /// ```
    #[inline]
    pub fn midi_number(&self) -> u8 {
        self.0
    }

    /// Returns a major scale starting from this note
    ///
    /// # Returns
    /// A `Scale<8>` representing the major scale starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_major_scale = C4.major_scale();
    /// assert_eq!(c_major_scale.notes(), &[C4, D4, E4, F4, G4, A4, B4, C5]);
    /// ```
    #[inline]
    pub fn major_scale(&self) -> Scale<8> {
        major_scale(*self)
    }

    /// Returns a natural minor scale starting from this note
    ///
    /// # Returns
    /// A `Scale<8>` representing the natural minor scale starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_natural_minor_scale = C4.natural_minor_scale();
    /// assert_eq!(c_natural_minor_scale.notes(), &[C4, D4, EFLAT4, F4, G4, AFLAT4, BFLAT4, C5]);
    /// ```
    #[inline]
    pub fn natural_minor_scale(&self) -> Scale<8> {
        natural_minor_scale(*self)
    }

    /// Returns a harmonic minor scale starting from this note
    ///
    /// # Returns
    /// A `Scale<8>` representing the harmonic minor scale starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_harmonic_minor_scale = C4.harmonic_minor_scale();
    /// assert_eq!(c_harmonic_minor_scale.notes(), &[C4, D4, EFLAT4, F4, G4, AFLAT4, B4, C5]);
    /// ```
    #[inline]
    pub fn harmonic_minor_scale(&self) -> Scale<8> {
        harmonic_minor_scale(*self)
    }

    /// Returns a melodic minor scale starting from this note
    ///
    /// # Returns
    /// A `Scale<8>` representing the melodic minor scale starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_melodic_minor_scale = C4.melodic_minor_scale();
    /// assert_eq!(c_melodic_minor_scale.notes(), &[C4, D4, EFLAT4, F4, G4, A4, B4, C5]);
    /// ```
    #[inline]
    pub fn melodic_minor_scale(&self) -> Scale<8> {
        melodic_minor_scale(*self)
    }

    /// Returns a major triad chord starting from this note
    ///
    /// # Returns
    /// A `Chord<3>` representing the major triad chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_major_triad = C4.major_triad_chord();
    /// assert_eq!(c_major_triad.notes(), &[C4, E4, G4]);
    /// ```
    #[inline]
    pub fn major_triad_chord(&self) -> Chord<3> {
        major_triad(*self)
    }

    /// Returns a minor triad chord starting from this note
    ///
    /// # Returns
    /// A `Chord<3>` representing the minor triad chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_minor_triad = C4.minor_triad_chord();
    /// assert_eq!(c_minor_triad.notes(), &[C4, DSHARP4, G4]);
    /// ```
    #[inline]
    pub fn minor_triad_chord(&self) -> Chord<3> {
        minor_triad(*self)
    }

    #[inline]
    pub fn dominant_seventh_chord(&self) -> Chord<4> {
        dominant_seventh(*self)
    }

    /// Returns a dominant seventh chord starting from this note
    ///
    /// # Returns
    /// A `Chord<4>` representing the dominant seventh chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_dominant_seventh = C4.dominant_seventh_chord();
    /// assert_eq!(c_dominant_seventh.notes(), &[C4, E4, G4, BFLAT4]);
    /// ```
    #[inline]
    pub fn dominant_seventh_ninth_chord(&self) -> Chord<5> {
        dominant_seventh_ninth(*self)
    }

    /// Returns a minor seventh chord starting from this note
    ///
    /// # Returns
    /// A `Chord<4>` representing the minor seventh chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_minor_seventh = C4.minor_seventh_chord();
    /// assert_eq!(c_minor_seventh.notes(), &[C4, DSHARP4, G4, BFLAT4]);
    /// ```
    #[inline]
    pub fn minor_seventh_chord(&self) -> Chord<4> {
        minor_seventh(*self)
    }

    /// Returns a minor seventh ninth chord starting from this note
    ///
    /// # Returns
    /// A `Chord<5>` representing the minor seventh ninth chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_minor_seventh_ninth = C4.minor_seventh_ninth_chord();
    /// assert_eq!(c_minor_seventh_ninth.notes(), &[C4, EFLAT4, G4, BFLAT4, D5]);
    /// ```
    #[inline]
    pub fn minor_seventh_ninth_chord(&self) -> Chord<5> {
        minor_seventh_ninth(*self)
    }

    /// Returns a major seventh chord starting from this note
    ///
    /// # Returns
    /// A `Chord<4>` representing the major seventh chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_major_seventh = C4.major_seventh_chord();
    /// assert_eq!(c_major_seventh.notes(), &[C4, E4, G4, B4]);
    /// ```
    #[inline]
    pub fn major_seventh_chord(&self) -> Chord<4> {
        major_seventh(*self)
    }

    /// Returns a minor major seventh chord starting from this note
    ///
    /// # Returns
    /// A `Chord<4>` representing the minor major seventh chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_minor_major_seventh = C4.minor_major_seventh_chord();
    /// assert_eq!(c_minor_major_seventh.notes(), &[C4, DSHARP4, G4, B4]);
    /// ```
    #[inline]
    pub fn minor_major_seventh_chord(&self) -> Chord<4> {
        minor_major_seventh(*self)
    }

    /// Returns a major sixth chord starting from this note
    ///
    /// # Returns
    /// A `Chord<4>` representing the major sixth chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_major_sixth = C4.major_sixth_chord();
    /// assert_eq!(c_major_sixth.notes(), &[C4, E4, G4, A4]);
    /// ```
    #[inline]
    pub fn major_sixth_chord(&self) -> Chord<4> {
        major_sixth(*self)
    }

    /// Returns a minor sixth chord starting from this note
    ///
    /// # Returns
    /// A `Chord<4>` representing the minor sixth chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_minor_sixth = C4.minor_sixth_chord();
    /// assert_eq!(c_minor_sixth.notes(), &[C4, DSHARP4, G4, A4]);
    /// ```
    #[inline]
    pub fn minor_sixth_chord(&self) -> Chord<4> {
        minor_sixth(*self)
    }

    /// Returns a major sixth ninth chord starting from this note
    ///
    /// # Returns
    /// A `Chord<5>` representing the major sixth ninth chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_major_sixth_ninth = C4.major_sixth_ninth_chord();
    /// assert_eq!(c_major_sixth_ninth.notes(), &[C4, E4, G4, A4, D5]);
    /// ```
    #[inline]
    pub fn major_sixth_ninth_chord(&self) -> Chord<5> {
        major_sixth_ninth(*self)
    }

    /// Returns a minor sixth ninth chord starting from this note
    ///
    /// # Returns
    /// A `Chord<5>` representing the minor sixth ninth chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_minor_sixth_ninth = C4.minor_sixth_ninth_chord();
    /// assert_eq!(c_minor_sixth_ninth.notes(), &[C4, EFLAT4, G4, A4, D5]);
    /// ```
    #[inline]
    pub fn minor_sixth_ninth_chord(&self) -> Chord<5> {
        minor_sixth_ninth(*self)
    }

    /// Returns a sus2 chord starting from this note
    ///
    /// # Returns
    /// A `Chord<3>` representing the sus2 chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_sus2 = C4.sus2_chord();
    /// assert_eq!(c_sus2.notes(), &[C4, D4, G4]);
    /// ```
    #[inline]
    pub fn sus2_chord(&self) -> Chord<3> {
        sus2(*self)
    }

    /// Returns a sus4 chord starting from this note
    ///
    /// # Returns
    /// A `Chord<3>` representing the sus4 chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_sus4 = C4.sus4_chord();
    /// assert_eq!(c_sus4.notes(), &[C4, F4, G4]);
    /// ```
    #[inline]
    pub fn sus4_chord(&self) -> Chord<3> {
        sus4(*self)
    }

    /// Returns a diminished triad chord starting from this note
    ///
    /// # Returns
    /// A `Chord<3>` representing the diminished triad chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_diminished_triad = C4.diminished_triad_chord();
    /// assert_eq!(c_diminished_triad.notes(), &[C4, EFLAT4, GFLAT4]);
    /// ```
    #[inline]
    pub fn diminished_triad_chord(&self) -> Chord<3> {
        diminished_triad(*self)
    }

    /// Returns a diminished seventh chord starting from this note
    ///
    /// # Returns
    /// A `Chord<4>` representing the diminished seventh chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_diminished_seventh = C4.diminished_seventh_chord();
    /// assert_eq!(c_diminished_seventh.notes(), &[C4, EFLAT4, GFLAT4, A4]);
    /// ```
    #[inline]
    pub fn diminished_seventh_chord(&self) -> Chord<4> {
        diminished_seventh(*self)
    }

    /// Returns a half diminished seventh chord starting from this note
    ///
    /// # Returns
    /// A `Chord<4>` representing the half diminished seventh chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_half_diminished_seventh = C4.half_diminished_seventh_chord();
    /// assert_eq!(c_half_diminished_seventh.notes(), &[C4, EFLAT4, GFLAT4, BFLAT4]);
    /// ```
    #[inline]
    pub fn half_diminished_seventh_chord(&self) -> Chord<4> {
        half_diminished_seventh(*self)
    }

    /// Returns an augmented triad chord starting from this note
    ///
    /// # Returns
    /// A `Chord<3>` representing the augmented triad chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_augmented_triad = C4.augmented_triad_chord();
    /// assert_eq!(c_augmented_triad.notes(), &[C4, E4, GSHARP4]);
    /// ```
    #[inline]
    pub fn augmented_triad_chord(&self) -> Chord<3> {
        augmented_triad(*self)
    }

    /// Returns an augmented seventh chord starting from this note
    ///
    /// # Returns
    /// A `Chord<4>` representing the augmented seventh chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_augmented_seventh = C4.augmented_seventh_chord();
    /// assert_eq!(c_augmented_seventh.notes(), &[C4, E4, GSHARP4, BFLAT4]);
    /// ```
    #[inline]
    pub fn augmented_seventh_chord(&self) -> Chord<4> {
        augmented_seventh(*self)
    }

    /// Returns a dominant ninth chord starting from this note
    ///
    /// # Returns
    /// A `Chord<5>` representing the dominant ninth chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_dominant_ninth = C4.dominant_ninth_chord();
    /// assert_eq!(c_dominant_ninth.notes(), &[C4, E4, G4, BFLAT4, D5]);
    /// ```
    #[inline]
    pub fn dominant_ninth_chord(&self) -> Chord<5> {
        dominant_ninth(*self)
    }

    /// Returns a minor ninth chord starting from this note
    ///
    /// # Returns
    /// A `Chord<5>` representing the minor ninth chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_minor_ninth = C4.minor_ninth_chord();
    /// assert_eq!(c_minor_ninth.notes(), &[C4, EFLAT4, G4, BFLAT4, D5]);
    /// ```
    #[inline]
    pub fn minor_ninth_chord(&self) -> Chord<5> {
        minor_ninth(*self)
    }

    /// Returns a major ninth chord starting from this note
    ///
    /// # Returns
    /// A `Chord<5>` representing the major ninth chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_major_ninth = C4.major_ninth_chord();
    /// assert_eq!(c_major_ninth.notes(), &[C4, E4, G4, B4, D5]);
    /// ```
    #[inline]
    pub fn major_ninth_chord(&self) -> Chord<5> {
        major_ninth(*self)
    }

    /// Returns a dominant eleventh chord starting from this note
    ///
    /// # Returns
    /// A `Chord<6>` representing the dominant eleventh chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_dominant_eleventh = C4.dominant_eleventh_chord();
    /// assert_eq!(c_dominant_eleventh.notes(), &[C4, E4, G4, BFLAT4, D5, F5]);
    /// ```
    #[inline]
    pub fn dominant_eleventh_chord(&self) -> Chord<6> {
        dominant_eleventh(*self)
    }

    /// Returns a minor eleventh chord starting from this note
    ///
    /// # Returns
    /// A `Chord<6>` representing the minor eleventh chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_minor_eleventh = C4.minor_eleventh_chord();
    /// assert_eq!(c_minor_eleventh.notes(), &[C4, EFLAT4, G4, BFLAT4, D5, F5]);
    /// ```
    #[inline]
    pub fn minor_eleventh_chord(&self) -> Chord<6> {
        minor_eleventh(*self)
    }

    /// Returns a major eleventh chord starting from this note
    ///
    /// # Returns
    /// A `Chord<6>` representing the major eleventh chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_major_eleventh = C4.major_eleventh_chord();
    /// assert_eq!(c_major_eleventh.notes(), &[C4, E4, G4, B4, D5, F5]);
    /// ```
    #[inline]
    pub fn major_eleventh_chord(&self) -> Chord<6> {
        major_eleventh(*self)
    }

    /// Returns a dominant thirteenth chord starting from this note
    ///
    /// # Returns
    /// A `Chord<7>` representing the dominant thirteenth chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_dominant_thirteenth = C4.dominant_thirteenth_chord();
    /// assert_eq!(c_dominant_thirteenth.notes(), &[C4, E4, G4, BFLAT4, D5, F5, A5]);
    /// ```
    #[inline]
    pub fn dominant_thirteenth_chord(&self) -> Chord<7> {
        dominant_thirteenth(*self)
    }

    /// Returns a minor thirteenth chord starting from this note
    ///
    /// # Returns
    /// A `Chord<7>` representing the minor thirteenth chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_minor_thirteenth = C4.minor_thirteenth_chord();
    /// assert_eq!(c_minor_thirteenth.notes(), &[C4, EFLAT4, G4, BFLAT4, D5, F5, A5]);
    /// ```
    #[inline]
    pub fn minor_thirteenth_chord(&self) -> Chord<7> {
        minor_thirteenth(*self)
    }

    /// Returns a major thirteenth chord starting from this note
    ///
    /// # Returns
    /// A `Chord<7>` representing the major thirteenth chord starting from this note
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::*;
    /// use mozzart_std::constants::*;
    ///
    /// let c_major_thirteenth = C4.major_thirteenth_chord();
    /// assert_eq!(c_major_thirteenth.notes(), &[C4, E4, G4, B4, D5, F5, A5]);
    /// ```
    #[inline]
    pub fn major_thirteenth_chord(&self) -> Chord<7> {
        major_thirteenth(*self)
    }
}

/// Conversion from `Note` to `u8` (MIDI note number)
///
/// This allows extracting the raw MIDI note number from a `Note`.
impl From<Note> for u8 {
    #[inline]
    fn from(note: Note) -> Self {
        note.0
    }
}

/// Conversion from a reference to `Note` to `u8` (MIDI note number)
///
/// This allows extracting the raw MIDI note number without consuming the note.
impl From<&Note> for u8 {
    #[inline]
    fn from(note: &Note) -> Self {
        note.0
    }
}

/// Implements addition of an interval to a note, producing a new note
///
/// This allows for transposition of notes by adding musical intervals.
/// For example, adding a perfect fifth (7 semitones) to C4 results in G4.
impl Add<Interval> for Note {
    type Output = Note;

    #[inline]
    fn add(self, interval: Interval) -> Self::Output {
        let interval: u8 = interval.into();
        Note::new(self.0 + interval)
    }
}

/// Implements in-place addition of an interval to a note
///
/// This allows for modifying a note by adding a musical interval directly.
impl AddAssign<Interval> for Note {
    #[inline]
    fn add_assign(&mut self, interval: Interval) {
        let interval: u8 = interval.into();
        self.0 = self.0 + interval;
    }
}

/// Implements addition of an interval to a note, producing a new note
///
/// This allows for transposition of notes by adding musical intervals.
/// For example, adding a perfect fifth (7 semitones) to C4 results in G4.
impl Add<&Interval> for Note {
    type Output = Note;

    #[inline]
    fn add(self, interval: &Interval) -> Self::Output {
        let interval: u8 = interval.into();
        Note::new(self.0 + interval)
    }
}

/// Implements in-place addition of an interval to a note
///
/// This allows for modifying a note by adding a musical interval directly.
impl AddAssign<&Interval> for Note {
    #[inline]
    fn add_assign(&mut self, interval: &Interval) {
        let interval: u8 = interval.into();
        self.0 = self.0 + interval;
    }
}

/// Implements subtraction of an interval from a note, producing a new note
///
/// This allows for downward transposition of notes by musical intervals.
/// For example, subtracting a perfect fifth (7 semitones) from C5 results in F4.
impl Sub<Interval> for Note {
    type Output = Note;

    #[inline]
    fn sub(self, interval: Interval) -> Self::Output {
        let interval: u8 = interval.into();
        Note::new(self.0 - interval)
    }
}

/// Implements in-place subtraction of an interval from a note
///
/// This allows for modifying a note by subtracting a musical interval directly.
impl SubAssign<Interval> for Note {
    #[inline]
    fn sub_assign(&mut self, interval: Interval) {
        let interval: u8 = interval.into();
        self.0 = self.0 - interval;
    }
}

/// Implements addition of a step to a note, producing a new note
///
/// This allows for transposition of notes by adding musical steps.
/// For example, adding a whole step (2 semitones) to C4 results in D4.
impl Add<Step> for Note {
    type Output = Note;

    #[inline]
    fn add(self, step: Step) -> Self::Output {
        let step: u8 = step.into();
        Note::new(self.0 + step)
    }
}

/// Implements in-place addition of a step to a note
///
/// This allows for modifying a note by adding a musical step directly.
impl AddAssign<Step> for Note {
    #[inline]
    fn add_assign(&mut self, step: Step) {
        let step: u8 = step.into();
        self.0 = self.0 + step;
    }
}

/// Implements addition of a step to a note, producing a new note
///
/// This allows for transposition of notes by adding musical steps.
/// For example, adding a whole step (2 semitones) to C4 results in D4.
impl Add<&Step> for Note {
    type Output = Note;

    #[inline]
    fn add(self, step: &Step) -> Self::Output {
        let step: u8 = step.into();
        Note::new(self.0 + step)
    }
}

/// Implements in-place addition of a step to a note
///
/// This allows for modifying a note by adding a musical step directly.
impl AddAssign<&Step> for Note {
    #[inline]
    fn add_assign(&mut self, step: &Step) {
        let step: u8 = step.into();
        self.0 = self.0 + step;
    }
}

/// Implements subtraction of an interval from a note, producing a new note
///
/// This allows for downward transposition of notes by musical steps.
/// For example, subtracting a whole step (2 semitones) from C5 results in A4.
impl Sub<Step> for Note {
    type Output = Note;

    #[inline]
    fn sub(self, step: Step) -> Self::Output {
        let step: u8 = step.into();
        Note::new(self.0 - step)
    }
}

/// Implements in-place subtraction of a step from a note
///
/// This allows for modifying a note by subtracting a musical step directly.
impl SubAssign<Step> for Note {
    #[inline]
    fn sub_assign(&mut self, step: Step) {
        let step: u8 = step.into();
        self.0 = self.0 - step;
    }
}

/// Implements the right shift operator for notes, which transposes up by octaves
///
/// Each octave shift up adds 12 semitones to the note.
///
/// # Examples
/// ```
/// use mozzart_std::constants::*;
///
/// let c4 = C4;
/// let c5 = c4 >> 1;  // C5 (one octave higher)
/// let c6 = c4 >> 2;  // C6 (two octaves higher)
/// ```
impl Shr<u8> for Note {
    type Output = Note;

    #[inline]
    fn shr(self, octaves: u8) -> Self::Output {
        let interval = Interval::from_octave(octaves);
        self.add(interval)
    }
}

/// Implements in-place right shift for notes, which transposes up by octaves
///
/// Each octave shift up adds 12 semitones to the note.
impl ShrAssign<u8> for Note {
    #[inline]
    fn shr_assign(&mut self, octaves: u8) {
        let interval = Interval::from_octave(octaves);
        self.add_assign(interval);
    }
}

/// Implements the left shift operator for notes, which transposes down by octaves
///
/// Each octave shift down subtracts 12 semitones from the note.
///
/// # Examples
/// ```
/// use mozzart_std::constants::*;
///
/// let c5 = C5;
/// let c4 = c5 << 1;  // C4 (one octave lower)
/// let c3 = c5 << 2;  // C3 (two octaves lower)
/// ```
impl Shl<u8> for Note {
    type Output = Note;

    #[inline]
    fn shl(self, octaves: u8) -> Self::Output {
        let interval = Interval::from_octave(octaves);
        self.sub(interval)
    }
}

/// Implements in-place left shift for notes, which transposes down by octaves
///
/// Each octave shift down subtracts 12 semitones from the note.
impl ShlAssign<u8> for Note {
    #[inline]
    fn shl_assign(&mut self, octaves: u8) {
        let interval = Interval::from_octave(octaves);
        self.sub_assign(interval);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn test_note_new() {
        let note = Note::new(60);
        assert_eq!(60u8, note.into());

        let high_note = Note::new(127);
        assert_eq!(127u8, high_note.into());

        let low_note = Note::new(0);
        assert_eq!(0u8, low_note.into());
    }

    #[test]
    fn test_note_into_u8() {
        let note = C4;

        // Test From<Note> for u8
        let midi_num: u8 = note.into();
        assert_eq!(60, midi_num);

        // Test From<&Note> for u8
        let midi_num: u8 = (&note).into();
        assert_eq!(60, midi_num);
    }

    #[test]
    fn test_adding_interval() {
        let c4 = C4;

        // Test addition
        let d4 = c4 + WHOLE;
        assert_eq!(62u8, d4.into());
        assert_eq!(D4, d4);

        let e4 = d4 + WHOLE;
        assert_eq!(64u8, e4.into());
        assert_eq!(E4, e4);

        let f4 = e4 + HALF;
        assert_eq!(65u8, f4.into());
        assert_eq!(F4, f4);
    }

    #[test]
    fn test_adding_step() {
        let c4 = C4;
        let d4 = c4 + WHOLE;
        assert_eq!(62u8, d4.into());
        assert_eq!(D4, d4);
    }

    #[test]
    fn test_adding_interval_in_place() {
        let mut note = C4; // C4

        // Test in-place addition
        note += WHOLE;
        assert_eq!(62u8, note.into()); // D4
        assert_eq!(D4, note);
        note += HALF;
        assert_eq!(63u8, note.into()); // D#4/Eb4
        assert_eq!(DSHARP4, note);
    }

    #[test]
    fn test_adding_step_in_place() {
        let mut note = C4;
        note += WHOLE;
        assert_eq!(62u8, note.into());
        assert_eq!(D4, note);
    }

    #[test]
    fn test_subtracting_interval() {
        let a4 = A4;

        // Test subtraction
        let g4 = a4 - WHOLE;
        assert_eq!(67u8, g4.into());
        assert_eq!(G4, g4);
    }

    #[test]
    fn test_subtracting_step() {
        let a4 = A4;
        let g4 = a4 - WHOLE;
        assert_eq!(67u8, g4.into());
        assert_eq!(G4, g4);
    }

    #[test]
    fn test_subtracting_interval_in_place() {
        let mut note = A4; // A4

        // Test in-place subtraction
        note -= WHOLE;
        assert_eq!(67u8, note.into()); // G4
        assert_eq!(G4, note);

        note -= HALF;
        assert_eq!(66u8, note.into()); // F#4/Gb4
        assert_eq!(FSHARP4, note);
    }

    #[test]
    fn test_subtracting_step_in_place() {
        let mut note = A4;
        note -= WHOLE;
        assert_eq!(67u8, note.into());
        assert_eq!(G4, note);
    }

    #[test]
    fn test_octave_shifts() {
        let c4 = C4;

        // Test right shift (transposing up)
        let c5 = c4 >> 1;
        assert_eq!(72u8, c5.into());
        assert_eq!(C5, c5);

        let c6 = c4 >> 2;
        assert_eq!(84u8, c6.into());
        assert_eq!(C6, c6);

        // Test left shift (transposing down)
        let c3 = c4 << 1;
        assert_eq!(48u8, c3.into());
        assert_eq!(C3, c3);

        let c2 = c4 << 2;
        assert_eq!(36u8, c2.into());
        assert_eq!(C2, c2);
    }

    #[test]
    fn test_octave_shifts_in_place() {
        let mut note = C4;

        // Test in-place right shift
        note >>= 1;
        assert_eq!(72u8, note.into()); // C5
        assert_eq!(C5, note);

        // Test in-place left shift
        note <<= 2;
        assert_eq!(48u8, note.into()); // C3
        assert_eq!(C3, note);
    }

    #[test]
    fn test_note_comparison() {
        let c4 = C4;
        let c4_duplicate = C4;
        let d4 = D4;

        // Test equality
        assert_eq!(c4, c4_duplicate);
        assert_ne!(c4, d4);

        // Test ordering
        assert!(c4 < d4);
        assert!(d4 > c4);
    }

    #[test]
    fn test_notes_from_steps_major_scale() {
        // Test creating a major scale (whole, whole, half, whole, whole, whole, half)
        let c4 = C4;
        let major_steps = [WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF];

        let c_major: Vec<_> = c4.notes_from_steps(major_steps).collect();

        // Verify the C major scale: C4, D4, E4, F4, G4, A4, B4, C5
        assert_eq!(c_major.len(), 8);
        assert_eq!(c_major[0], C4);
        assert_eq!(c_major[1], D4);
        assert_eq!(c_major[2], E4);
        assert_eq!(c_major[3], F4);
        assert_eq!(c_major[4], G4);
        assert_eq!(c_major[5], A4);
        assert_eq!(c_major[6], B4);
        assert_eq!(c_major[7], C5);
    }

    #[test]
    fn test_from_steps_minor_scale() {
        // Test creating a natural minor scale (whole, half, whole, whole, half, whole, whole)
        let a4 = A4;
        let minor_steps = [WHOLE, HALF, WHOLE, WHOLE, HALF, WHOLE, WHOLE];

        let a_minor: Vec<_> = a4.notes_from_steps(minor_steps).collect();

        // Verify the A minor scale: A4, B4, C5, D5, E5, F5, G5, A5
        assert_eq!(a_minor.len(), 8);
        assert_eq!(a_minor[0], A4);
        assert_eq!(a_minor[1], B4);
        assert_eq!(a_minor[2], C5);
        assert_eq!(a_minor[3], D5);
        assert_eq!(a_minor[4], E5);
        assert_eq!(a_minor[5], F5);
        assert_eq!(a_minor[6], G5);
        assert_eq!(a_minor[7], A5);
    }

    #[test]
    fn test_notes_from_steps_major_triad() {
        // Test creating a major triad (major third + minor third)
        let c4 = C4;
        let major_triad_steps = [MAJOR_THIRD.into(), MINOR_THIRD.into()];

        let c_major_triad: Vec<_> = c4.notes_from_steps(major_triad_steps).collect();

        // Verify the C major triad: C4, E4, G4
        assert_eq!(c_major_triad.len(), 3);
        assert_eq!(c_major_triad[0], C4);
        assert_eq!(c_major_triad[1], E4);
        assert_eq!(c_major_triad[2], G4);
    }

    #[test]
    fn test_notes_from_steps_empty() {
        // Test with empty steps array
        let c4 = C4;
        let empty_steps: [Step; 0] = [];

        let result: Vec<_> = c4.notes_from_steps(empty_steps).collect();

        // Should just return the root note
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], C4);
    }

    #[test]
    fn test_notes_from_steps_chromatic() {
        // Test creating a chromatic scale (all half steps)
        let c4 = C4;
        let chromatic_steps = [HALF, HALF, HALF, HALF];

        let chromatic: Vec<_> = c4.notes_from_steps(chromatic_steps).collect();

        // Verify part of a chromatic scale: C4, C#4, D4, D#4, E4
        assert_eq!(chromatic.len(), 5);
        assert_eq!(chromatic[0], C4);
        assert_eq!(chromatic[1], CSHARP4);
        assert_eq!(chromatic[2], D4);
        assert_eq!(chromatic[3], DSHARP4);
        assert_eq!(chromatic[4], E4);
    }

    #[test]
    fn test_notes_from_steps_pentatonic() {
        // Test creating a major pentatonic scale (major second, major second, minor third, major second, minor third)
        let c4 = C4;
        let pentatonic_steps = [
            MAJOR_SECOND.into(),
            MAJOR_SECOND.into(),
            MINOR_THIRD.into(),
            MAJOR_SECOND.into(),
        ];

        let pentatonic: Vec<_> = c4.notes_from_steps(pentatonic_steps).collect();

        // Verify the C major pentatonic scale: C4, D4, E4, G4, A4
        assert_eq!(pentatonic.len(), 5);
        assert_eq!(pentatonic[0], C4);
        assert_eq!(pentatonic[1], D4);
        assert_eq!(pentatonic[2], E4);
        assert_eq!(pentatonic[3], G4);
        assert_eq!(pentatonic[4], A4);
    }

    #[test]
    fn test_into_notes_from_steps_minor_triad() {
        // Test creating a minor triad (minor third + major third)
        let c4 = C4;
        let minor_triad_steps = [MINOR_THIRD.into(), MAJOR_THIRD.into()];

        let c_minor_triad: Vec<_> = c4.into_notes_from_steps(minor_triad_steps).collect();

        // Verify the C minor triad: C4, Eb4, G4
        assert_eq!(c_minor_triad.len(), 3);
        assert_eq!(c_minor_triad[0], C4);
        assert_eq!(c_minor_triad[1], DSHARP4); // Eb4
        assert_eq!(c_minor_triad[2], G4);
    }

    #[test]
    fn test_into_notes_from_steps_dominant_seventh() {
        // Test creating a dominant seventh chord (major third + minor third + minor third)
        let g4 = G4;
        let dom7_steps = [MAJOR_THIRD.into(), MINOR_THIRD.into(), MINOR_THIRD.into()];

        let g7_chord: Vec<_> = g4.into_notes_from_steps(dom7_steps).collect();

        // Verify the G7 chord: G4, B4, D5, F5
        assert_eq!(g7_chord.len(), 4);
        assert_eq!(g7_chord[0], G4);
        assert_eq!(g7_chord[1], B4);
        assert_eq!(g7_chord[2], D5);
        assert_eq!(g7_chord[3], F5);
    }

    #[test]
    fn test_into_notes_from_steps_vs_from_steps() {
        // This test demonstrates the difference between into_notes and from_steps
        // in terms of ownership

        let c4 = C4;

        // Using from_steps, c4 can be used after the call
        let _scale1 = c4.notes_from_steps([WHOLE, WHOLE]).collect::<Vec<_>>();
        let c4_after = c4; // This is valid because from_steps borrows c4
        assert_eq!(c4_after, C4);

        // Using into_notes, the original note is consumed
        let c4 = C4;
        let _scale2 = c4.into_notes_from_steps([WHOLE, WHOLE]).collect::<Vec<_>>();
        // If we uncommented the line below, it would cause a compile error:
        // let c4_after = c4; // Error: c4 has been moved
    }

    #[test]
    fn test_into_notes_from_steps_method_chaining() {
        // Test that into_notes can be used in method chains
        let chord: Vec<_> = C4
            .into_notes_from_steps([MAJOR_THIRD.into(), MINOR_THIRD.into()]) // Create C major triad
            .into_iter() // This is redundant but demonstrates chainability
            .collect();

        assert_eq!(chord.len(), 3);
        assert_eq!(chord[0], C4);
        assert_eq!(chord[1], E4);
        assert_eq!(chord[2], G4);
    }

    #[test]
    fn test_notes_from_intervals_major_chord() {
        // Test creating a major chord using fixed intervals from the root
        let c4 = C4;
        let intervals = [MAJOR_THIRD.into(), PERFECT_FIFTH.into()];

        let c_major_chord: Vec<_> = c4.notes_from_intervals(intervals).collect();

        // Verify the C major chord: C4, E4, G4
        assert_eq!(c_major_chord.len(), 3);
        assert_eq!(c_major_chord[0], C4);
        assert_eq!(c_major_chord[1], E4);
        assert_eq!(c_major_chord[2], G4);
    }

    #[test]
    fn test_notes_from_intervals_seventh_chords() {
        // Test creating different seventh chords

        // Major seventh chord (major triad + major seventh)
        let intervals_maj7 = [MAJOR_THIRD, PERFECT_FIFTH, MAJOR_SEVENTH];
        let cmaj7: Vec<_> = C4.notes_from_intervals(intervals_maj7).collect();

        assert_eq!(cmaj7.len(), 4);
        assert_eq!(cmaj7[0], C4); // Root
        assert_eq!(cmaj7[1], E4); // Major third
        assert_eq!(cmaj7[2], G4); // Perfect fifth
        assert_eq!(cmaj7[3], B4); // Major seventh

        // Dominant seventh chord (major triad + minor seventh)
        let intervals_dom7 = [MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
        let c7: Vec<_> = C4.notes_from_intervals(intervals_dom7).collect();

        assert_eq!(c7.len(), 4);
        assert_eq!(c7[0], C4); // Root
        assert_eq!(c7[1], E4); // Major third
        assert_eq!(c7[2], G4); // Perfect fifth
        assert_eq!(c7[3], ASHARP4); // Minor seventh (Bb4)

        // Minor seventh chord (minor triad + minor seventh)
        let intervals_min7 = [MINOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH];
        let cmin7: Vec<_> = C4.notes_from_intervals(intervals_min7).collect();

        assert_eq!(cmin7.len(), 4);
        assert_eq!(cmin7[0], C4); // Root
        assert_eq!(cmin7[1], DSHARP4); // Minor third (Eb4)
        assert_eq!(cmin7[2], G4); // Perfect fifth
        assert_eq!(cmin7[3], ASHARP4); // Minor seventh (Bb4)
    }

    #[test]
    fn test_notes_from_intervals_extended_chords() {
        // Test creating extended chords (9th, 11th, 13th)
        let c4 = C4;

        // C9 chord (dominant 7th + major 9th)
        let c9_intervals = [MAJOR_THIRD, PERFECT_FIFTH, MINOR_SEVENTH, MAJOR_NINTH];
        let c9: Vec<_> = c4.notes_from_intervals(c9_intervals).collect();

        assert_eq!(c9.len(), 5);
        assert_eq!(c9[0], C4); // Root
        assert_eq!(c9[1], E4); // Major third
        assert_eq!(c9[2], G4); // Perfect fifth
        assert_eq!(c9[3], ASHARP4); // Minor seventh (Bb4)
        assert_eq!(c9[4], D5); // Major ninth (D5, one octave + major second)
    }

    #[test]
    fn test_steps_vs_intervals_comparison() {
        // This test demonstrates the difference between notes_from_steps and notes_from_intervals
        let c4 = C4;

        // Using steps: each interval is applied to the previous note
        // To build C-E-G-C, we need: major third + minor third + perfect fourth
        let major_triad_steps = [
            MAJOR_THIRD.into(),
            MINOR_THIRD.into(),
            PERFECT_FOURTH.into(),
        ];
        let c_major_with_steps: Vec<_> = c4.notes_from_steps(major_triad_steps).collect();

        // Using intervals: each interval is applied directly to the root
        // To build C-E-G-C, we need: major third, perfect fifth, perfect octave
        let major_triad_intervals = [MAJOR_THIRD, PERFECT_FIFTH, PERFECT_OCTAVE];
        let c_major_with_intervals: Vec<_> =
            c4.notes_from_intervals(major_triad_intervals).collect();

        // Both methods should produce the same chord (C4-E4-G4-C5), but the interval specifications differ
        assert_eq!(c_major_with_steps.len(), 4);
        assert_eq!(c_major_with_intervals.len(), 4);

        assert_eq!(c_major_with_steps[0], C4);
        assert_eq!(c_major_with_steps[1], E4);
        assert_eq!(c_major_with_steps[2], G4);
        assert_eq!(c_major_with_steps[3], C5);

        assert_eq!(c_major_with_intervals[0], C4);
        assert_eq!(c_major_with_intervals[1], E4);
        assert_eq!(c_major_with_intervals[2], G4);
        assert_eq!(c_major_with_intervals[3], C5);
    }

    #[test]
    fn test_into_notes_from_intervals() {
        // Test that into_notes_from_intervals provides the public interface
        let c_major_chord: Vec<_> = C4
            .into_notes_from_intervals([MAJOR_THIRD, PERFECT_FIFTH])
            .collect();

        assert_eq!(c_major_chord.len(), 3);
        assert_eq!(c_major_chord[0], C4);
        assert_eq!(c_major_chord[1], E4);
        assert_eq!(c_major_chord[2], G4);

        // Test using reference after call (should work since the method borrows)
        let c4 = C4;
        let _chord = c4
            .into_notes_from_intervals([MAJOR_THIRD, PERFECT_FIFTH])
            .collect::<Vec<_>>();

        // This should be valid since into_notes_from_intervals borrows c4
        let c4_after = c4;
        assert_eq!(c4_after, C4);
    }

    #[test]
    fn test_suspended_chords() {
        // Test suspended chords (sus2, sus4) which replace the third with a different interval

        // Csus2 chord (C + D + G)
        let sus2_intervals = [MAJOR_SECOND, PERFECT_FIFTH];
        let csus2: Vec<_> = C4.notes_from_intervals(sus2_intervals).collect();

        assert_eq!(csus2.len(), 3);
        assert_eq!(csus2[0], C4);
        assert_eq!(csus2[1], D4); // Major second instead of third
        assert_eq!(csus2[2], G4);

        // Csus4 chord (C + F + G)
        let sus4_intervals = [PERFECT_FOURTH, PERFECT_FIFTH];
        let csus4: Vec<_> = C4.notes_from_intervals(sus4_intervals).collect();

        assert_eq!(csus4.len(), 3);
        assert_eq!(csus4[0], C4);
        assert_eq!(csus4[1], F4); // Perfect fourth instead of third
        assert_eq!(csus4[2], G4);
    }
}
