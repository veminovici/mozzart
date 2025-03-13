use crate::Step;

// Basic interval steps used in scale construction
/// Half step (1 semitone) - smallest interval in Western music
pub const HALF: Step = Step::new(1);
/// Whole step (2 semitones) - equivalent to two half steps
pub const WHOLE: Step = Step::new(2);
/// Step and a half (3 semitones) - common in many scales including harmonic minor
pub const WHOLE_AND_HALF: Step = Step::new(3);
