use crate::constants::*;
use crate::Step;

/// Represents the step pattern for a major scale
///
/// The major scale is one of the most fundamental scales in Western music,
/// characterized by a bright, happy sound. It consists of 7 notes with the
/// following pattern of whole and half steps: W-W-H-W-W-W-H.
///
/// This array stores the intervals between consecutive notes in the scale:
/// - Root to 2nd: whole step (2 semitones)
/// - 2nd to 3rd: whole step (2 semitones)
/// - 3rd to 4th: half step (1 semitone)
/// - 4th to 5th: whole step (2 semitones)
/// - 5th to 6th: whole step (2 semitones)
/// - 6th to 7th: whole step (2 semitones)
/// - 7th to octave: half step (1 semitone)
///
/// The numbers in the comments represent semitones from the root:
/// - 2: second degree (whole step from root)
/// - 4: third degree (whole step from second)
/// - 5: fourth degree (half step from third)
/// - 7: fifth degree (whole step from fourth)
/// - 9: sixth degree (whole step from fifth)
/// - 11: seventh degree (whole step from sixth)
/// - 12: octave (half step from seventh)
///
/// This pattern is used to construct any major scale from any starting note.
pub const MAJOR_SCALE_STEPS: [Step; 7] = [
    WHOLE, // 2
    WHOLE, // 4
    HALF,  // 5
    WHOLE, // 7
    WHOLE, // 9
    WHOLE, // 11
    HALF,  // 12
];

/// Represents the step pattern for a natural minor scale
///
/// The natural minor scale (also called Aeolian mode) is characterized by a
/// darker, more melancholic sound compared to the major scale. It consists of
/// 7 notes with the following pattern of whole and half steps: W-H-W-W-H-W-W.
///
/// This array stores the intervals between consecutive notes in the scale:
/// - Root to 2nd: whole step (2 semitones)
/// - 2nd to 3rd: half step (1 semitone)
/// - 3rd to 4th: whole step (2 semitones)
/// - 4th to 5th: whole step (2 semitones)
/// - 5th to 6th: half step (1 semitone)
/// - 6th to 7th: whole step (2 semitones)
/// - 7th to octave: whole step (2 semitones)
///
/// The numbers in the comments represent semitones from the root:
/// - 2: second degree (whole step from root)
/// - 3: third degree (half step from second)
/// - 5: fourth degree (whole step from third)
/// - 7: fifth degree (whole step from fourth)
/// - 8: sixth degree (half step from fifth)
/// - 10: seventh degree (whole step from sixth)
/// - 12: octave (whole step from seventh)
///
/// This pattern is used to construct any natural minor scale from any starting note.
/// The natural minor scale is the most basic of the three minor scales
/// (natural, harmonic, and melodic).
pub const NATURAL_MINOR_SCALE_STEPS: [Step; 7] = [
    WHOLE, // 2
    HALF,  // 3
    WHOLE, // 5
    WHOLE, // 7
    HALF,  // 8
    WHOLE, // 10
    WHOLE, // 12
];

/// Represents the step pattern for a harmonic minor scale
///
/// The harmonic minor scale modifies the natural minor scale by raising
/// the 7th degree by a half step, creating a leading tone with a stronger
/// pull to the tonic. This results in an augmented second interval (3 semitones)
/// between the 6th and 7th degrees, giving the scale its distinctive exotic sound.
///
/// This array stores the intervals between consecutive notes in the scale:
/// - Root to 2nd: whole step (2 semitones)
/// - 2nd to 3rd: half step (1 semitone)
/// - 3rd to 4th: whole step (2 semitones)
/// - 4th to 5th: whole step (2 semitones)
/// - 5th to 6th: half step (1 semitone)
/// - 6th to 7th: augmented second (3 semitones)
/// - 7th to octave: half step (1 semitone)
///
/// The numbers in the comments represent semitones from the root:
/// - 2: second degree (whole step from root)
/// - 3: third degree (half step from second)
/// - 5: fourth degree (whole step from third)
/// - 7: fifth degree (whole step from fourth)
/// - 8: sixth degree (half step from fifth)
/// - 11: seventh degree (augmented second from sixth)
/// - 12: octave (half step from seventh)
///
/// The harmonic minor scale is particularly important in classical music
/// for creating stronger cadences. It forms the basis of many chord progressions
/// in Western classical music and is also commonly used in flamenco, Eastern European,
/// and Middle Eastern music.
pub const HARMONIC_MINOR_SCALE_STEPS: [Step; 7] = [
    WHOLE,          // 2
    HALF,           // 3
    WHOLE,          // 5
    WHOLE,          // 7
    HALF,           // 8
    WHOLE_AND_HALF, // 11
    HALF,           // 12
];

/// Represents the step pattern for a melodic minor scale (ascending form)
///
/// The melodic minor scale modifies the natural minor scale by raising both
/// the 6th and 7th degrees when ascending, making it sound smoother and more
/// melodically flexible. Traditionally, the descending form reverts to the
/// natural minor scale, though in modern practice (especially in jazz), the
/// ascending form is often used both up and down.
///
/// This array stores the intervals between consecutive notes in the ascending scale:
/// - Root to 2nd: whole step (2 semitones)
/// - 2nd to 3rd: half step (1 semitone)
/// - 3rd to 4th: whole step (2 semitones)
/// - 4th to 5th: whole step (2 semitones)
/// - 5th to 6th: whole step (2 semitones)
/// - 6th to 7th: whole step (2 semitones)
/// - 7th to octave: half step (1 semitone)
///
/// The numbers in the comments represent semitones from the root:
/// - 2: second degree (whole step from root)
/// - 3: third degree (half step from second)
/// - 5: fourth degree (whole step from third)
/// - 7: fifth degree (whole step from fourth)
/// - 9: sixth degree (whole step from fifth)
/// - 11: seventh degree (whole step from sixth)
/// - 12: octave (half step from seventh)
///
/// The melodic minor scale is widely used in classical music to create smoother
/// melodic lines. In jazz theory, it's known as the "jazz minor" scale and forms
/// the basis of many advanced jazz harmonies and improvisation approaches.
pub const MELODIC_MINOR_SCALE_STEPS: [Step; 7] = [
    WHOLE, // 2
    HALF,  // 3
    WHOLE, // 5
    WHOLE, // 7
    WHOLE, // 9
    WHOLE, // 11
    HALF,  // 12
];
