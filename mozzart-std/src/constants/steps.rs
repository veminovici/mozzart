use crate::Step;

/// No step (0 semitones) - used to represent the interval between the same note
pub const UNISON: Step = Step::new(0);
/// Half step (1 semitone) - smallest interval in Western music
pub const HALF: Step = Step::new(1);
/// Whole step (2 semitones) - equivalent to two half steps
pub const WHOLE: Step = Step::new(2);
/// Step and a half (3 semitones) - common in many scales including harmonic minor
pub const WHOLE_AND_HALF: Step = Step::new(3);
