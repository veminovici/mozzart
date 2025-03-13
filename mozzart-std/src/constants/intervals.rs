use crate::Interval;

// Simple intervals (within one octave)
/// Perfect unison (0 semitones) - same pitch, no interval distance
pub const PERFECT_UNISON: Interval = Interval::new(0);
/// Minor second (1 semitone) - dissonant interval creating tension, common in passing tones
pub const MINOR_SECOND: Interval = Interval::new(1);
/// Major second (2 semitones) - basic step in diatonic scales, moderately dissonant
pub const MAJOR_SECOND: Interval = Interval::new(2);
/// Minor third (3 semitones) - fundamental building block of minor chords, creates melancholy sound
pub const MINOR_THIRD: Interval = Interval::new(3);
/// Major third (4 semitones) - fundamental building block of major chords, creates bright sound
pub const MAJOR_THIRD: Interval = Interval::new(4);
/// Perfect fourth (5 semitones) - considered perfect consonance in most contexts
pub const PERFECT_FOURTH: Interval = Interval::new(5);
/// Augmented fourth (6 semitones) - also known as tritone, historically called "diabolus in musica"
pub const AUGMENTED_FOURTH: Interval = Interval::new(6);
/// Diminished fifth (6 semitones) - enharmonic equivalent of augmented fourth, creates tension
pub const DIMINISHED_FIFTH: Interval = Interval::new(6);
/// Perfect fifth (7 semitones) - strong consonance, foundational in harmony
pub const PERFECT_FIFTH: Interval = Interval::new(7);
/// Augmented fifth (8 semitones) - creates tension, common in augmented chords
pub const AUGMENTED_FIFTH: Interval = Interval::new(8);
/// Diminished sixth (8 semitones) - enharmonic equivalent of augmented fifth
pub const DIMINISHED_SIXTH: Interval = Interval::new(8);
/// Minor sixth (9 semitones) - creates gentle tension, common in minor keys
pub const MINOR_SIXTH: Interval = Interval::new(9);
/// Major sixth (10 semitones) - consonant interval common in major keys
pub const MAJOR_SIXTH: Interval = Interval::new(10);
/// Minor seventh (10 semitones) - creates tension seeking resolution, fundamental in dominant seventh chords
pub const MINOR_SEVENTH: Interval = Interval::new(10);
/// Major seventh (11 semitones) - creates bright tension, common in major seventh chords
pub const MAJOR_SEVENTH: Interval = Interval::new(11);
/// Perfect octave (12 semitones) - same note in a higher register, complete consonance
pub const PERFECT_OCTAVE: Interval = Interval::new(12);

// Compound intervals (beyond one octave)
/// Minor ninth (13 semitones) - octave plus minor second, creates rich dissonance in jazz chords
pub const MINOR_NINTH: Interval = Interval::new(13);
/// Major ninth (14 semitones) - octave plus major second, adds color in extended chords
pub const MAJOR_NINTH: Interval = Interval::new(14);
/// Minor tenth (15 semitones) - octave plus minor third, creates expanded minor sonority
pub const MINOR_TENTH: Interval = Interval::new(15);
/// Major tenth (16 semitones) - octave plus major third, creates expanded major sonority
pub const MAJOR_TENTH: Interval = Interval::new(16);
/// Perfect eleventh (17 semitones) - octave plus perfect fourth, common suspension in jazz
pub const PERFECT_ELEVENTH: Interval = Interval::new(17);
/// Augmented eleventh (18 semitones) - octave plus augmented fourth, adds tension in extended harmony
pub const AUGMENTED_ELEVENTH: Interval = Interval::new(18);
/// Diminished twelfth (18 semitones) - enharmonic equivalent of augmented eleventh
pub const DIMINISHED_TWELFTH: Interval = Interval::new(18);
/// Perfect twelfth (19 semitones) - octave plus perfect fifth, creates strong expanded harmony
pub const PERFECT_TWELFTH: Interval = Interval::new(19);
/// Augmented twelfth (20 semitones) - octave plus augmented fifth, expanded augmented harmony
pub const AUGMENTED_TWELFTH: Interval = Interval::new(20);
/// Diminished thirteenth (20 semitones) - enharmonic equivalent of augmented twelfth
pub const DIMINISHED_THIRTEENTH: Interval = Interval::new(20);
/// Minor thirteenth (21 semitones) - octave plus minor sixth, expressive interval in jazz harmony
pub const MINOR_THIRTEENTH: Interval = Interval::new(21);
/// Major thirteenth (22 semitones) - octave plus major sixth, highest standard extension in jazz chords
pub const MAJOR_THIRTEENTH: Interval = Interval::new(22);
/// Minor fourteenth (22 semitones) - enharmonic equivalent of major thirteenth
pub const MINOR_FOURTEENTH: Interval = Interval::new(22);
/// Major fourteenth (23 semitones) - octave plus major seventh, creates extreme tension
pub const MAJOR_FOURTEENTH: Interval = Interval::new(23);
/// Double octave (24 semitones) - spans two octaves, creates expanded range
pub const DOUBLE_OCTAVE: Interval = Interval::new(24);

// Extended compound intervals (beyond double octave)
/// Minor sixteenth (25 semitones) - two octaves plus minor second, extended dissonance
pub const MINOR_SIXTEENTH: Interval = Interval::new(25);
/// Major sixteenth (26 semitones) - two octaves plus major second, extended harmony element
pub const MAJOR_SIXTEENTH: Interval = Interval::new(26);
/// Minor seventeenth (27 semitones) - two octaves plus minor third, extended minor sonority
pub const MINOR_SEVENTEENTH: Interval = Interval::new(27);
/// Major seventeenth (28 semitones) - two octaves plus major third, extended major sonority
pub const MAJOR_SEVENTEENTH: Interval = Interval::new(28);
/// Perfect eighteenth (29 semitones) - two octaves plus perfect fourth
pub const PERFECT_EIGHTEENTH: Interval = Interval::new(29);
/// Augmented eighteenth (30 semitones) - two octaves plus augmented fourth (tritone)
pub const AUGMENTED_EIGHTEENTH: Interval = Interval::new(30);
/// Diminished nineteenth (30 semitones) - enharmonic equivalent of augmented eighteenth
pub const DIMINISHED_NINETEENTH: Interval = Interval::new(30);
/// Perfect nineteenth (31 semitones) - two octaves plus perfect fifth, expanded harmonic anchor
pub const PERFECT_NINETEENTH: Interval = Interval::new(31);
/// Minor twentieth (33 semitones) - two octaves plus minor sixth
pub const MINOR_TWENTIETH: Interval = Interval::new(33);
/// Major twentieth (34 semitones) - two octaves plus major sixth
pub const MAJOR_TWENTIETH: Interval = Interval::new(34);
/// Perfect twenty-first (35 semitones) - two octaves plus major seventh
pub const PERFECT_TWENTY_FIRST: Interval = Interval::new(35);
/// Triple octave (36 semitones) - spans three octaves, extreme range expansion
pub const TRIPLE_OCTAVE: Interval = Interval::new(36);
