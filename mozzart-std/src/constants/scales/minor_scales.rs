use crate::{constants::*, minor_scale};
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

/// The interval pattern that defines a natural minor scale.
///
/// A natural minor scale consists of 7 intervals between its 8 notes, following the pattern:
/// - Whole step (TONE)
/// - Half step (SEMITONE)
/// - Whole step (TONE)
/// - Whole step (TONE)
/// - Half step (SEMITONE)
/// - Whole step (TONE)
/// - Whole step (TONE)
///
/// This pattern is commonly remembered as "W-H-W-W-H-W-W" and produces the
/// characteristic melancholic sound of the minor scale. Also known as the Aeolian mode,
/// it is the sixth mode of the major scale.
pub const MINOR_SCALE_STEPS: [Interval; 7] = [TONE, SEMITONE, TONE, TONE, SEMITONE, TONE, TONE];

lazy_static! {
    /// A comprehensive collection of all possible minor scales indexed by their root pitch.
    ///
    /// This HashMap contains minor scales for every possible root pitch in the MIDI range
    /// (from C at MIDI note 0 to G9). Each scale is constructed using the minor scale pattern
    /// (whole-half-whole-whole-half-whole-whole) starting from the given root pitch.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{constants::scales::minor_scales::MINOR_SCALES, A4, minor_scale};
    ///
    /// // Look up the A4 minor scale
    /// let a4_scale = MINOR_SCALES.get(&A4).unwrap();
    /// assert_eq!(*a4_scale, minor_scale(A4));
    /// ```
    pub static ref MINOR_SCALES: HashMap<Pitch, Scale<8>> = {
        let mut map = HashMap::new();

        for p in C.0..G9.0 {
            let pitch = Pitch::new(p);
            map.insert(pitch, minor_scale(pitch));
        }

        map
    };
}

lazy_static! {
    /// The C minor scale (relative to Eb major).
    ///
    /// This scale contains the pitches C-D-Eb-F-G-Ab-Bb-C.
    /// The key signature for C minor has 3 flats (Bb, Eb, Ab).
    /// C minor is known for its dramatic and passionate character in classical music.
    pub static ref C_MINOR_SCALE: Scale<8> = minor_scale(C);
    
    /// The C# minor scale (relative to E major).
    ///
    /// This scale contains the pitches C#-D#-E-F#-G#-A-B-C#.
    /// The key signature for C# minor has 4 sharps (F#, C#, G#, D#).
    /// C# minor is often used to express dark, intense emotions.
    pub static ref CSHARP_MINOR_SCALE: Scale<8> = minor_scale(CSHARP);
    
    /// The D minor scale (relative to F major).
    ///
    /// This scale contains the pitches D-E-F-G-A-Bb-C-D.
    /// The key signature for D minor has 1 flat (Bb).
    /// D minor is considered to be one of the most melancholic keys,
    /// famously used in many somber classical compositions.
    pub static ref D_MINOR_SCALE: Scale<8> = minor_scale(D);
    
    /// The D# minor scale (relative to F# major).
    ///
    /// This scale contains the pitches D#-E#(F)-F#-G#-A#-B-C#-D#.
    /// This scale is more commonly written as Eb minor, with 6 flats.
    pub static ref DSHARP_MINOR_SCALE: Scale<8> = minor_scale(DSHARP);
    
    /// The E minor scale (relative to G major).
    ///
    /// This scale contains the pitches E-F#-G-A-B-C-D-E.
    /// The key signature for E minor has 1 sharp (F#).
    /// E minor is popular in guitar music due to the instrument's tuning.
    pub static ref E_MINOR_SCALE: Scale<8> = minor_scale(E);
    
    /// The F minor scale (relative to Ab major).
    ///
    /// This scale contains the pitches F-G-Ab-Bb-C-Db-Eb-F.
    /// The key signature for F minor has 4 flats (Bb, Eb, Ab, Db).
    /// F minor is often associated with passion and deep emotions.
    pub static ref F_MINOR_SCALE: Scale<8> = minor_scale(F);
    
    /// The F# minor scale (relative to A major).
    ///
    /// This scale contains the pitches F#-G#-A-B-C#-D-E-F#.
    /// The key signature for F# minor has 3 sharps (F#, C#, G#).
    /// F# minor is a commonly used key in romantic and expressive music.
    pub static ref FSHARP_MINOR_SCALE: Scale<8> = minor_scale(FSHARP);
    
    /// The G minor scale (relative to Bb major).
    ///
    /// This scale contains the pitches G-A-Bb-C-D-Eb-F-G.
    /// The key signature for G minor has 2 flats (Bb, Eb).
    /// G minor is often used to express tragedy, sadness, or grave seriousness.
    pub static ref G_MINOR_SCALE: Scale<8> = minor_scale(G);
    
    /// The G# minor scale (relative to B major).
    ///
    /// This scale contains the pitches G#-A#-B-C#-D#-E-F#-G#.
    /// The key signature for G# minor has 5 sharps (F#, C#, G#, D#, A#).
    /// This scale is sometimes written as Ab minor with 7 flats.
    pub static ref GSHARP_MINOR_SCALE: Scale<8> = minor_scale(GSHARP);
    
    /// The A minor scale (relative to C major).
    ///
    /// This scale contains the pitches A-B-C-D-E-F-G-A.
    /// A minor has no sharps or flats in its key signature.
    /// As the relative minor of C major, A minor is one of the most fundamental and
    /// commonly used minor keys in Western music.
    pub static ref A_MINOR_SCALE: Scale<8> = minor_scale(A);
    
    /// The A# minor scale (relative to C# major).
    ///
    /// This scale contains the pitches A#-B#(C)-C#-D#-E#(F)-F#-G#-A#.
    /// This scale is more commonly written as Bb minor with 5 flats.
    pub static ref ASHARP_MINOR_SCALE: Scale<8> = minor_scale(ASHARP);
    
    /// The B minor scale (relative to D major).
    ///
    /// This scale contains the pitches B-C#-D-E-F#-G-A-B.
    /// The key signature for B minor has 2 sharps (F#, C#).
    /// B minor is often associated with expressions of quiet resignation or melancholy.
    pub static ref B_MINOR_SCALE: Scale<8> = minor_scale(B);
}

lazy_static! {
    /// C minor scale in octave 0 (MIDI notes 12-24)
    ///
    /// This scale spans from C0 (MIDI note 12, ~16.35 Hz) to C1 (MIDI note 24, ~32.70 Hz).
    /// These low frequencies are below the range of a standard piano.
    pub static ref C0_MINOR_SCALE: Scale<8> = minor_scale(C0);
    
    /// C# minor scale in octave 0 (MIDI notes 13-25)
    ///
    /// This scale spans from C#0 (MIDI note 13, ~17.32 Hz) to C#1 (MIDI note 25, ~34.65 Hz).
    /// These extremely low frequencies are at the threshold of human hearing.
    pub static ref CSHARP0_MINOR_SCALE: Scale<8> = minor_scale(CSHARP0);
    
    /// D minor scale in octave 0 (MIDI notes 14-26)
    ///
    /// This scale spans from D0 (MIDI note 14, ~18.35 Hz) to D1 (MIDI note 26, ~36.71 Hz).
    /// These extremely low frequency notes are below the range of most instruments.
    pub static ref D0_MINOR_SCALE: Scale<8> = minor_scale(D0);
    
    /// D# minor scale in octave 0 (MIDI notes 15-27)
    ///
    /// This scale spans from D#0 (MIDI note 15, ~19.45 Hz) to D#1 (MIDI note 27, ~38.89 Hz).
    /// These extremely low frequency notes are below the range of most instruments.
    pub static ref DSHARP0_MINOR_SCALE: Scale<8> = minor_scale(DSHARP0);
    
    /// E minor scale in octave 0 (MIDI notes 16-28)
    ///
    /// This scale spans from E0 (MIDI note 16, ~20.60 Hz) to E1 (MIDI note 28, ~41.20 Hz).
    /// These frequencies approach the lower threshold of human hearing perception.
    pub static ref E0_MINOR_SCALE: Scale<8> = minor_scale(E0);
    
    /// F minor scale in octave 0 (MIDI notes 17-29)
    ///
    /// This scale spans from F0 (MIDI note 17, ~21.83 Hz) to F1 (MIDI note 29, ~43.65 Hz).
    /// These frequencies are at the very low end of human hearing range.
    pub static ref F0_MINOR_SCALE: Scale<8> = minor_scale(F0);
    
    /// F# minor scale in octave 0 (MIDI notes 18-30)
    ///
    /// This scale spans from F#0 (MIDI note 18, ~23.12 Hz) to F#1 (MIDI note 30, ~46.25 Hz).
    /// These frequencies are at the very low end of human hearing range.
    pub static ref FSHARP0_MINOR_SCALE: Scale<8> = minor_scale(FSHARP0);
    
    /// G minor scale in octave 0 (MIDI notes 19-31)
    ///
    /// This scale spans from G0 (MIDI note 19, ~24.50 Hz) to G1 (MIDI note 31, ~49.00 Hz).
    /// These bass frequencies are below the range of most musical instruments.
    pub static ref G0_MINOR_SCALE: Scale<8> = minor_scale(G0);
    
    /// G# minor scale in octave 0 (MIDI notes 20-32)
    ///
    /// This scale spans from G#0 (MIDI note 20, ~25.96 Hz) to G#1 (MIDI note 32, ~51.91 Hz).
    /// These bass frequencies are below the range of most musical instruments.
    pub static ref GSHARP0_MINOR_SCALE: Scale<8> = minor_scale(GSHARP0);
    
    /// A minor scale in octave 0 (MIDI notes 21-33)
    ///
    /// This scale spans from A0 (MIDI note 21, ~27.50 Hz) to A1 (MIDI note 33, ~55.00 Hz).
    /// A0 is the lowest note on a standard piano.
    pub static ref A0_MINOR_SCALE: Scale<8> = minor_scale(A0);
    
    /// A# minor scale in octave 0 (MIDI notes 22-34)
    ///
    /// This scale spans from A#0 (MIDI note 22, ~29.14 Hz) to A#1 (MIDI note 34, ~58.27 Hz).
    /// These bass frequencies begin to approach the lower range of a standard piano.
    pub static ref ASHARP0_MINOR_SCALE: Scale<8> = minor_scale(ASHARP0);
    
    /// B minor scale in octave 0 (MIDI notes 23-35)
    ///
    /// This scale spans from B0 (MIDI note 23, ~30.87 Hz) to B1 (MIDI note 35, ~61.74 Hz).
    /// These bass frequencies are near the lower end of a standard piano.
    pub static ref B0_MINOR_SCALE: Scale<8> = minor_scale(B0);
}

lazy_static! {
    /// C minor scale in octave 1 (MIDI notes 24-36)
    ///
    /// This scale spans from C1 (MIDI note 24, ~32.70 Hz) to C2 (MIDI note 36, ~65.41 Hz).
    /// These notes are in the range of the lowest octave on a standard piano.
    pub static ref C1_MINOR_SCALE: Scale<8> = minor_scale(C1);
    
    /// C# minor scale in octave 1 (MIDI notes 25-37)
    ///
    /// This scale spans from C#1 (MIDI note 25, ~34.65 Hz) to C#2 (MIDI note 37, ~69.30 Hz).
    /// These bass frequencies are in the lower range of a standard piano.
    pub static ref CSHARP1_MINOR_SCALE: Scale<8> = minor_scale(CSHARP1);
    
    /// D minor scale in octave 1 (MIDI notes 26-38)
    ///
    /// This scale spans from D1 (MIDI note 26, ~36.71 Hz) to D2 (MIDI note 38, ~73.42 Hz).
    /// These bass frequencies are in the lower range of a standard piano.
    pub static ref D1_MINOR_SCALE: Scale<8> = minor_scale(D1);
    
    /// D# minor scale in octave 1 (MIDI notes 27-39)
    ///
    /// This scale spans from D#1 (MIDI note 27, ~38.89 Hz) to D#2 (MIDI note 39, ~77.78 Hz).
    /// These bass frequencies are in the lower range of a standard piano.
    pub static ref DSHARP1_MINOR_SCALE: Scale<8> = minor_scale(DSHARP1);
    
    /// E minor scale in octave 1 (MIDI notes 28-40)
    ///
    /// This scale spans from E1 (MIDI note 28, ~41.20 Hz) to E2 (MIDI note 40, ~82.41 Hz).
    /// E1 is the same pitch as the lowest string on a standard bass guitar.
    pub static ref E1_MINOR_SCALE: Scale<8> = minor_scale(E1);
    
    /// F minor scale in octave 1 (MIDI notes 29-41)
    ///
    /// This scale spans from F1 (MIDI note 29, ~43.65 Hz) to F2 (MIDI note 41, ~87.31 Hz).
    /// These bass frequencies are used in the low bass range of orchestral and keyboard music.
    pub static ref F1_MINOR_SCALE: Scale<8> = minor_scale(F1);
    
    /// F# minor scale in octave 1 (MIDI notes 30-42)
    ///
    /// This scale spans from F#1 (MIDI note 30, ~46.25 Hz) to F#2 (MIDI note 42, ~92.50 Hz).
    /// These bass frequencies are used in the low bass range of orchestral and keyboard music.
    pub static ref FSHARP1_MINOR_SCALE: Scale<8> = minor_scale(FSHARP1);
    
    /// G minor scale in octave 1 (MIDI notes 31-43)
    ///
    /// This scale spans from G1 (MIDI note 31, ~49.00 Hz) to G2 (MIDI note 43, ~98.00 Hz).
    /// These bass frequencies are used in the low bass range of orchestral and keyboard music.
    pub static ref G1_MINOR_SCALE: Scale<8> = minor_scale(G1);
    
    /// G# minor scale in octave 1 (MIDI notes 32-44)
    ///
    /// This scale spans from G#1 (MIDI note 32, ~51.91 Hz) to G#2 (MIDI note 44, ~103.83 Hz).
    /// These bass frequencies are used in the low bass range of orchestral and keyboard music.
    pub static ref GSHARP1_MINOR_SCALE: Scale<8> = minor_scale(GSHARP1);
    
    /// A minor scale in octave 1 (MIDI notes 33-45)
    ///
    /// This scale spans from A1 (MIDI note 33, 55.00 Hz) to A2 (MIDI note 45, 110.00 Hz).
    /// This scale includes the pitch of the lowest string (A) on a standard guitar.
    pub static ref A1_MINOR_SCALE: Scale<8> = minor_scale(A1);
    
    /// A# minor scale in octave 1 (MIDI notes 34-46)
    ///
    /// This scale spans from A#1 (MIDI note 34, ~58.27 Hz) to A#2 (MIDI note 46, ~116.54 Hz).
    /// These bass frequencies are commonly used in bass lines for various music genres.
    pub static ref ASHARP1_MINOR_SCALE: Scale<8> = minor_scale(ASHARP1);
    
    /// B minor scale in octave 1 (MIDI notes 35-47)
    ///
    /// This scale spans from B1 (MIDI note 35, ~61.74 Hz) to B2 (MIDI note 47, ~123.47 Hz).
    /// These bass frequencies are commonly used in bass lines for various music genres.
    pub static ref B1_MINOR_SCALE: Scale<8> = minor_scale(B1);
}

lazy_static! {
    pub static ref C2_MINOR_SCALE: Scale<8> = minor_scale(C2);
    pub static ref CSHARP2_MINOR_SCALE: Scale<8> = minor_scale(CSHARP2);
    pub static ref D2_MINOR_SCALE: Scale<8> = minor_scale(D2);
    pub static ref DSHARP2_MINOR_SCALE: Scale<8> = minor_scale(DSHARP2);
    pub static ref E2_MINOR_SCALE: Scale<8> = minor_scale(E2);
    pub static ref F2_MINOR_SCALE: Scale<8> = minor_scale(F2);
    pub static ref FSHARP2_MINOR_SCALE: Scale<8> = minor_scale(FSHARP2);
    pub static ref G2_MINOR_SCALE: Scale<8> = minor_scale(G2);
    pub static ref GSHARP2_MINOR_SCALE: Scale<8> = minor_scale(GSHARP2);
    pub static ref A2_MINOR_SCALE: Scale<8> = minor_scale(A2);
    pub static ref ASHARP2_MINOR_SCALE: Scale<8> = minor_scale(ASHARP2);
    pub static ref B2_MINOR_SCALE: Scale<8> = minor_scale(B2);
}

lazy_static! {
    pub static ref C3_MINOR_SCALE: Scale<8> = minor_scale(C3);
    pub static ref CSHARP3_MINOR_SCALE: Scale<8> = minor_scale(CSHARP3);
    pub static ref D3_MINOR_SCALE: Scale<8> = minor_scale(D3);
    pub static ref DSHARP3_MINOR_SCALE: Scale<8> = minor_scale(DSHARP3);
    pub static ref E3_MINOR_SCALE: Scale<8> = minor_scale(E3);
    pub static ref F3_MINOR_SCALE: Scale<8> = minor_scale(F3);
    pub static ref FSHARP3_MINOR_SCALE: Scale<8> = minor_scale(FSHARP3);
    pub static ref G3_MINOR_SCALE: Scale<8> = minor_scale(G3);
    pub static ref GSHARP3_MINOR_SCALE: Scale<8> = minor_scale(GSHARP3);
    pub static ref A3_MINOR_SCALE: Scale<8> = minor_scale(A3);
    pub static ref ASHARP3_MINOR_SCALE: Scale<8> = minor_scale(ASHARP3);
    pub static ref B3_MINOR_SCALE: Scale<8> = minor_scale(B3);
}

lazy_static! {
    pub static ref C4_MINOR_SCALE: Scale<8> = minor_scale(C4);
    pub static ref CSHARP4_MINOR_SCALE: Scale<8> = minor_scale(CSHARP4);
    pub static ref D4_MINOR_SCALE: Scale<8> = minor_scale(D4);
    pub static ref DSHARP4_MINOR_SCALE: Scale<8> = minor_scale(DSHARP4);
    pub static ref E4_MINOR_SCALE: Scale<8> = minor_scale(E4);
    pub static ref F4_MINOR_SCALE: Scale<8> = minor_scale(F4);
    pub static ref FSHARP4_MINOR_SCALE: Scale<8> = minor_scale(FSHARP4);
    pub static ref G4_MINOR_SCALE: Scale<8> = minor_scale(G4);
    pub static ref GSHARP4_MINOR_SCALE: Scale<8> = minor_scale(GSHARP4);
    pub static ref A4_MINOR_SCALE: Scale<8> = minor_scale(A4);
    pub static ref ASHARP4_MINOR_SCALE: Scale<8> = minor_scale(ASHARP4);
    pub static ref B4_MINOR_SCALE: Scale<8> = minor_scale(B4);
}

lazy_static! {
    pub static ref C5_MINOR_SCALE: Scale<8> = minor_scale(C5);
    pub static ref CSHARP5_MINOR_SCALE: Scale<8> = minor_scale(CSHARP5);
    pub static ref D5_MINOR_SCALE: Scale<8> = minor_scale(D5);
    pub static ref DSHARP5_MINOR_SCALE: Scale<8> = minor_scale(DSHARP5);
    pub static ref E5_MINOR_SCALE: Scale<8> = minor_scale(E5);
    pub static ref F5_MINOR_SCALE: Scale<8> = minor_scale(F5);
    pub static ref FSHARP5_MINOR_SCALE: Scale<8> = minor_scale(FSHARP5);
    pub static ref G5_MINOR_SCALE: Scale<8> = minor_scale(G5);
    pub static ref GSHARP5_MINOR_SCALE: Scale<8> = minor_scale(GSHARP5);
    pub static ref A5_MINOR_SCALE: Scale<8> = minor_scale(A5);
    pub static ref ASHARP5_MINOR_SCALE: Scale<8> = minor_scale(ASHARP5);
    pub static ref B5_MINOR_SCALE: Scale<8> = minor_scale(B5);
}

lazy_static! {
    pub static ref C6_MINOR_SCALE: Scale<8> = minor_scale(C6);
    pub static ref CSHARP6_MINOR_SCALE: Scale<8> = minor_scale(CSHARP6);
    pub static ref D6_MINOR_SCALE: Scale<8> = minor_scale(D6);
    pub static ref DSHARP6_MINOR_SCALE: Scale<8> = minor_scale(DSHARP6);
    pub static ref E6_MINOR_SCALE: Scale<8> = minor_scale(E6);
    pub static ref F6_MINOR_SCALE: Scale<8> = minor_scale(F6);
    pub static ref FSHARP6_MINOR_SCALE: Scale<8> = minor_scale(FSHARP6);
    pub static ref G6_MINOR_SCALE: Scale<8> = minor_scale(G6);
    pub static ref GSHARP6_MINOR_SCALE: Scale<8> = minor_scale(GSHARP6);
    pub static ref A6_MINOR_SCALE: Scale<8> = minor_scale(A6);
    pub static ref ASHARP6_MINOR_SCALE: Scale<8> = minor_scale(ASHARP6);
    pub static ref B6_MINOR_SCALE: Scale<8> = minor_scale(B6);
}

lazy_static! {
    pub static ref C7_MINOR_SCALE: Scale<8> = minor_scale(C7);
    pub static ref CSHARP7_MINOR_SCALE: Scale<8> = minor_scale(CSHARP7);
    pub static ref D7_MINOR_SCALE: Scale<8> = minor_scale(D7);
    pub static ref DSHARP7_MINOR_SCALE: Scale<8> = minor_scale(DSHARP7);
    pub static ref E7_MINOR_SCALE: Scale<8> = minor_scale(E7);
    pub static ref F7_MINOR_SCALE: Scale<8> = minor_scale(F7);
    pub static ref FSHARP7_MINOR_SCALE: Scale<8> = minor_scale(FSHARP7);
    pub static ref G7_MINOR_SCALE: Scale<8> = minor_scale(G7);
    pub static ref GSHARP7_MINOR_SCALE: Scale<8> = minor_scale(GSHARP7);
    pub static ref A7_MINOR_SCALE: Scale<8> = minor_scale(A7);
    pub static ref ASHARP7_MINOR_SCALE: Scale<8> = minor_scale(ASHARP7);
    pub static ref B7_MINOR_SCALE: Scale<8> = minor_scale(B7);
}

lazy_static! {
    pub static ref C8_MINOR_SCALE: Scale<8> = minor_scale(C8);
    pub static ref CSHARP8_MINOR_SCALE: Scale<8> = minor_scale(CSHARP8);
    pub static ref D8_MINOR_SCALE: Scale<8> = minor_scale(D8);
    pub static ref DSHARP8_MINOR_SCALE: Scale<8> = minor_scale(DSHARP8);
    pub static ref E8_MINOR_SCALE: Scale<8> = minor_scale(E8);
    pub static ref F8_MINOR_SCALE: Scale<8> = minor_scale(F8);
    pub static ref FSHARP8_MINOR_SCALE: Scale<8> = minor_scale(FSHARP8);
    pub static ref G8_MINOR_SCALE: Scale<8> = minor_scale(G8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{A4, ASHARP4, B4, C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4};
    use crate::Pitch;
    use crate::ScaleQuality;

    #[test]
    fn test_minor_scale_steps_intervals() {
        // Test that MINOR_SCALE_STEPS contains the correct interval pattern
        assert_eq!(
            MINOR_SCALE_STEPS,
            [TONE, SEMITONE, TONE, TONE, SEMITONE, TONE, TONE]
        );
    }

    #[test]
    fn test_reference_minor_scales() {
        // Test that the reference scales have the correct structure
        // Each of these scales should follow the minor scale step pattern
        assert_eq!(C_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);

        // Test specific scales for correct root notes
        assert_eq!(A_MINOR_SCALE.tonic(), A);
        assert_eq!(E_MINOR_SCALE.tonic(), E);
        assert_eq!(D_MINOR_SCALE.tonic(), D);
    }

    #[test]
    fn test_octave_0_minor_scales() {
        // Test that octave 0 scales have the correct structure
        // Each of these scales should follow the minor scale step pattern
        assert_eq!(C0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B0_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);

        // Test that scale notes span the expected octave range
        // C0's MIDI value is 12, which should be in "octave 0" 
        assert_eq!(C0_MINOR_SCALE.pitches()[0].0, 12); // First note is C0 (MIDI 12)
        assert_eq!(C0_MINOR_SCALE.pitches()[7].0, 24); // Last note is C1 (MIDI 24)
    }

    #[test]
    fn test_octave_1_minor_scales() {
        // Test that octave 1 scales have the correct structure
        // Each of these scales should follow the minor scale step pattern
        assert_eq!(C1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(CSHARP1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(D1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(DSHARP1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(E1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(F1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(FSHARP1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(G1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(GSHARP1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(A1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(ASHARP1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);
        assert_eq!(B1_MINOR_SCALE.steps(), MINOR_SCALE_STEPS);

        // Test that scale notes span the expected octave range
        // C1's MIDI value is 24, which should be in "octave 1"
        assert_eq!(C1_MINOR_SCALE.pitches()[0].0, 24); // First note is C1 (MIDI 24)
        assert_eq!(C1_MINOR_SCALE.pitches()[7].0, 36); // Last note is C2 (MIDI 36)
    }

    #[test]
    fn test_a_minor_scale_notes() {
        // Test the A minor scale specifically since it's the most fundamental minor scale
        // A minor should contain the notes A, B, C, D, E, F, G, A
        let a_minor = &A4_MINOR_SCALE;
        let pitches = a_minor.pitches();
        
        assert_eq!(pitches[0], A4); // A
        assert_eq!(pitches[1], B4); // B
        assert_eq!(pitches[2], C5); // C
        assert_eq!(pitches[3], D5); // D
        assert_eq!(pitches[4], E5); // E
        assert_eq!(pitches[5], F5); // F
        assert_eq!(pitches[6], G5); // G
        assert_eq!(pitches[7], A5); // A
    }

    #[test]
    fn test_minor_scales_hashmap() {
        // Test that the MINOR_SCALES HashMap contains scales with the correct roots
        let roots = [C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4, A4, ASHARP4, B4];

        for root in roots.iter() {
            let scale = MINOR_SCALES.get(root);
            assert!(scale.is_some(), "MINOR_SCALES should contain the root pitch");
            
            let scale = scale.unwrap();
            assert_eq!(scale.tonic(), *root, "Scale tonic should match the key");
            assert_eq!(scale.steps(), MINOR_SCALE_STEPS, "Scale should follow minor scale pattern");
        }
    }

    #[test]
    fn test_minor_scales_hashmap_octave_boundaries() {
        // Test scales at the boundaries of the MIDI range
        let lowest_pitch = Pitch::new(0);  // Lowest possible MIDI note
        let highest_supported = Pitch::new(127);  // Highest possible MIDI note
        
        // For lowest pitch, ensure we have a scale
        let lowest_scale = MINOR_SCALES.get(&lowest_pitch);
        assert!(lowest_scale.is_some(), "MINOR_SCALES should contain the lowest pitch");
        
        // For highest supported pitch, ensure we have a scale
        let highest_scale = MINOR_SCALES.get(&highest_supported);
        assert!(highest_scale.is_some(), "MINOR_SCALES should contain the highest supported pitch");
    }

    #[test]
    fn test_all_scales_preserve_minor_quality() {
        // Test random sampling of scales from the HashMap to ensure they all have Minor quality
        let sample_pitches = [
            Pitch::new(12),  // C0
            Pitch::new(36),  // C2
            Pitch::new(60),  // C4 (middle C)
            Pitch::new(84),  // C6
            Pitch::new(108), // C8
            Pitch::new(127), // G9 (highest MIDI note)
        ];

        for pitch in sample_pitches.iter() {
            if let Some(scale) = MINOR_SCALES.get(pitch) {
                assert_eq!(scale.quality(), ScaleQuality::Minor);
            }
        }
    }

    #[test]
    fn test_compare_relative_major_minor() {
        // Test the relationship between relative major and minor scales
        // A minor is the relative minor of C major
        // They should have the same notes but different starting points
        use crate::constants::scales::major_scales::C_MAJOR_SCALE;
        
        let a_minor_notes: Vec<Pitch> = A_MINOR_SCALE.pitches().iter().copied().collect();
        let c_major_notes: Vec<Pitch> = C_MAJOR_SCALE.pitches().iter().copied().collect();
        
        // Adjust the octaves to compare the same pitch classes
        // A minor: A, B, C, D, E, F, G, A
        // C major: C, D, E, F, G, A, B, C
        // We need to rotate one to match the other
        let mut rotated_c_major = c_major_notes.clone();
        rotated_c_major.rotate_left(5); // Rotate to start with A
        rotated_c_major[7] = rotated_c_major[0]; // Replace the last note to match octave
        
        // Now the notes should match in pitch class (ignoring octave differences)
        for i in 0..7 {
            assert_eq!(
                a_minor_notes[i].0 % 12, 
                rotated_c_major[i].0 % 12,
                "Pitch classes should match between relative scales"
            );
        }
    }
}
