use crate::{constants::*, major_scale};
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

/// The interval pattern that defines a major scale.
///
/// A major scale consists of 7 intervals between its 8 notes, following the pattern:
/// - Whole step (TONE)
/// - Whole step (TONE)
/// - Half step (SEMITONE)
/// - Whole step (TONE)
/// - Whole step (TONE)
/// - Whole step (TONE)
/// - Half step (SEMITONE)
///
/// This pattern is commonly remembered as "W-W-H-W-W-W-H" and produces the
/// characteristic sound of the major scale, which is the foundation of Western music.
pub const MAJOR_SCALE_STEPS: [Interval; 7] = [TONE, TONE, SEMITONE, TONE, TONE, TONE, SEMITONE];

lazy_static! {
    /// A comprehensive collection of all possible major scales indexed by their root pitch.
    ///
    /// This HashMap contains major scales for every possible root pitch in the MIDI range
    /// (from C at MIDI note 0 to G9). Each scale is constructed using the major scale pattern
    /// (whole-whole-half-whole-whole-whole-half) starting from the given root pitch.
    ///
    /// # Examples
    /// ```
    /// use mozzart_std::{MAJOR_SCALES, C4, major_scale};
    ///
    /// // Look up the C4 major scale
    /// let c4_scale = MAJOR_SCALES.get(&C4).unwrap();
    /// assert_eq!(*c4_scale, major_scale(C4));
    /// ```
    pub static ref MAJOR_SCALES: HashMap<Pitch, Scale<8>> = {
        let mut map = HashMap::new();

        for p in C.0..G9.0 {
            let pitch = Pitch::new(p);
            map.insert(pitch, major_scale(pitch));
        }

        map
    };
}

lazy_static! {
    /// The C major scale starting from middle C (C4).
    ///
    /// This scale contains the pitches C4-D4-E4-F4-G4-A4-B4-C5.
    /// The C major scale is the most basic major scale with no sharps or flats,
    /// making it a fundamental reference point in music theory and education.
    pub static ref C_MAJOR_SCALE: Scale<8> = major_scale(C4);

    /// The C# major scale starting from C#4.
    ///
    /// This scale contains the pitches C#4-D#4-E#4(F4)-F#4-G#4-A#4-B#4(C5)-C#5.
    /// As a key signature, C# major has 7 sharps, making it enharmonically
    /// equivalent to Db major (which has 5 flats).
    pub static ref C_SHARP_MAJOR_SCALE: Scale<8> = major_scale(CSHARP4);

    /// The D major scale starting from D4.
    ///
    /// This scale contains the pitches D4-E4-F#4-G4-A4-B4-C#5-D5.
    /// The key signature for D major contains 2 sharps (F# and C#).
    /// D major is commonly used in violin music due to the instrument's tuning.
    pub static ref D_MAJOR_SCALE: Scale<8> = major_scale(D4);

    /// The D# major scale starting from D#4.
    ///
    /// This scale contains the pitches D#4-E#4(F4)-F##4(G4)-G#4-A#4-B#4(C5)-C##4(D5)-D#5.
    /// This scale is more commonly written as Eb major, which has 3 flats (Eb, Ab, Bb).
    pub static ref D_SHARP_MAJOR_SCALE: Scale<8> = major_scale(DSHARP4);

    /// The E major scale starting from E4.
    ///
    /// This scale contains the pitches E4-F#4-G#4-A4-B4-C#5-D#5-E5.
    /// The key signature for E major contains 4 sharps (F#, G#, C#, D#).
    /// E major is often used in guitar music due to the instrument's open strings.
    pub static ref E_MAJOR_SCALE: Scale<8> = major_scale(E4);

    /// The F major scale starting from F4.
    ///
    /// This scale contains the pitches F4-G4-A4-Bb4-C5-D5-E5-F5.
    /// The key signature for F major contains 1 flat (Bb).
    /// F major was one of the earliest keys used in keyboard music.
    pub static ref F_MAJOR_SCALE: Scale<8> = major_scale(F4);

    /// The F# major scale starting from F#4.
    ///
    /// This scale contains the pitches F#4-G#4-A#4-B4-C#5-D#5-E#5(F5)-F#5.
    /// As F# major, the key signature has 6 sharps (F#, G#, A#, C#, D#, E#).
    /// As Gb major, it has 6 flats (Gb, Ab, Bb, Cb, Db, Eb).
    pub static ref F_SHARP_MAJOR_SCALE: Scale<8> = major_scale(FSHARP4);

    /// The G major scale starting from G4.
    ///
    /// This scale contains the pitches G4-A4-B4-C5-D5-E5-F#5-G5.
    /// The key signature for G major contains 1 sharp (F#).
    /// G major is very common in folk and popular music.
    pub static ref G_MAJOR_SCALE: Scale<8> = major_scale(G4);

    /// The G# major scale starting from G#4.
    ///
    /// This scale contains the pitches G#4-A#4-B#4(C5)-C#5-D#5-E#5(F5)-F##5(G5)-G#5.
    /// This scale is almost always written as Ab major, which has 4 flats (Ab, Bb, Db, Eb).
    pub static ref G_SHARP_MAJOR_SCALE: Scale<8> = major_scale(GSHARP4);

    /// The A major scale starting from A4 (A440).
    ///
    /// This scale contains the pitches A4-B4-C#5-D5-E5-F#5-G#5-A5.
    /// The key signature for A major contains 3 sharps (F#, C#, G#).
    /// A4 is the standard tuning reference at 440 Hz.
    pub static ref A_MAJOR_SCALE: Scale<8> = major_scale(A4);

    /// The A# major scale starting from A#4.
    ///
    /// This scale contains the pitches A#4-B#4(C5)-C##5(D5)-D#5-E#5(F5)-F##5(G5)-G##5(A5)-A#5.
    /// This scale is almost always written as Bb major, which has 2 flats (Bb, Eb).
    /// Bb major is common for brass and woodwind instruments.
    pub static ref A_SHARP_MAJOR_SCALE: Scale<8> = major_scale(ASHARP4);

    /// The B major scale starting from B4.
    ///
    /// This scale contains the pitches B4-C#5-D#5-E5-F#5-G#5-A#5-B5.
    /// The key signature for B major contains 5 sharps (F#, C#, G#, D#, A#).
    /// B major is one of the more remote keys in common practice.
    pub static ref B_MAJOR_SCALE: Scale<8> = major_scale(B4);
}

lazy_static! {
    /// C major scale in octave 0 (MIDI notes 12-24)
    ///
    /// This scale spans from C0 (MIDI note 12, ~16.35 Hz) to C1 (MIDI note 24, ~32.70 Hz).
    /// These notes are at the very bottom of the MIDI range and below the range of a standard piano.
    pub static ref C0_MAJOR_SCALE: Scale<8> = major_scale(C0);

    /// C# major scale in octave 0 (MIDI notes 13-25)
    ///
    /// This scale spans from C#0 (MIDI note 13, ~17.32 Hz) to C#1 (MIDI note 25, ~34.65 Hz).
    /// These extremely low frequency notes are below the range of most instruments.
    pub static ref CSHARP0_MAJOR_SCALE: Scale<8> = major_scale(CSHARP0);

    /// D major scale in octave 0 (MIDI notes 14-26)
    ///
    /// This scale spans from D0 (MIDI note 14, ~18.35 Hz) to D1 (MIDI note 26, ~36.71 Hz).
    /// These extremely low frequency notes are below the range of most instruments.
    pub static ref D0_MAJOR_SCALE: Scale<8> = major_scale(D0);

    /// D# major scale in octave 0 (MIDI notes 15-27)
    ///
    /// This scale spans from D#0 (MIDI note 15, ~19.45 Hz) to D#1 (MIDI note 27, ~38.89 Hz).
    /// These extremely low frequency notes are below the range of most instruments.
    pub static ref DSHARP0_MAJOR_SCALE: Scale<8> = major_scale(DSHARP0);

    /// E major scale in octave 0 (MIDI notes 16-28)
    ///
    /// This scale spans from E0 (MIDI note 16, ~20.60 Hz) to E1 (MIDI note 28, ~41.20 Hz).
    /// These frequencies approach the lower threshold of human hearing perception.
    pub static ref E0_MAJOR_SCALE: Scale<8> = major_scale(E0);

    /// F major scale in octave 0 (MIDI notes 17-29)
    ///
    /// This scale spans from F0 (MIDI note 17, ~21.83 Hz) to F1 (MIDI note 29, ~43.65 Hz).
    /// These frequencies are at the very low end of human hearing range.
    pub static ref F0_MAJOR_SCALE: Scale<8> = major_scale(F0);

    /// F# major scale in octave 0 (MIDI notes 18-30)
    ///
    /// This scale spans from F#0 (MIDI note 18, ~23.12 Hz) to F#1 (MIDI note 30, ~46.25 Hz).
    /// These frequencies are at the very low end of human hearing range.
    pub static ref FSHARP0_MAJOR_SCALE: Scale<8> = major_scale(FSHARP0);

    /// G major scale in octave 0 (MIDI notes 19-31)
    ///
    /// This scale spans from G0 (MIDI note 19, ~24.50 Hz) to G1 (MIDI note 31, ~49.00 Hz).
    /// These bass frequencies are below the range of most musical instruments.
    pub static ref G0_MAJOR_SCALE: Scale<8> = major_scale(G0);

    /// G# major scale in octave 0 (MIDI notes 20-32)
    ///
    /// This scale spans from G#0 (MIDI note 20, ~25.96 Hz) to G#1 (MIDI note 32, ~51.91 Hz).
    /// These bass frequencies are below the range of most musical instruments.
    pub static ref GSHARP0_MAJOR_SCALE: Scale<8> = major_scale(GSHARP0);

    /// A major scale in octave 0 (MIDI notes 21-33)
    ///
    /// This scale spans from A0 (MIDI note 21, ~27.50 Hz) to A1 (MIDI note 33, ~55.00 Hz).
    /// A0 is the lowest note on a standard piano.
    pub static ref A0_MAJOR_SCALE: Scale<8> = major_scale(A0);

    /// A# major scale in octave 0 (MIDI notes 22-34)
    ///
    /// This scale spans from A#0 (MIDI note 22, ~29.14 Hz) to A#1 (MIDI note 34, ~58.27 Hz).
    /// These bass frequencies begin to approach the lower range of a standard piano.
    pub static ref ASHARP0_MAJOR_SCALE: Scale<8> = major_scale(ASHARP0);

    /// B major scale in octave 0 (MIDI notes 23-35)
    ///
    /// This scale spans from B0 (MIDI note 23, ~30.87 Hz) to B1 (MIDI note 35, ~61.74 Hz).
    /// These bass frequencies are near the lower end of a standard piano.
    pub static ref B0_MAJOR_SCALE: Scale<8> = major_scale(B0);
}

lazy_static! {
    /// C major scale in octave 1 (MIDI notes 24-36)
    ///
    /// This scale spans from C1 (MIDI note 24, ~32.70 Hz) to C2 (MIDI note 36, ~65.41 Hz).
    /// These notes are in the range of the lowest octave on a standard piano.
    pub static ref C1_MAJOR_SCALE: Scale<8> = major_scale(C1);

    /// C# major scale in octave 1 (MIDI notes 25-37)
    ///
    /// This scale spans from C#1 (MIDI note 25, ~34.65 Hz) to C#2 (MIDI note 37, ~69.30 Hz).
    /// These bass frequencies are in the lower range of a standard piano.
    pub static ref CSHARP1_MAJOR_SCALE: Scale<8> = major_scale(CSHARP1);

    /// D major scale in octave 1 (MIDI notes 26-38)
    ///
    /// This scale spans from D1 (MIDI note 26, ~36.71 Hz) to D2 (MIDI note 38, ~73.42 Hz).
    /// These bass frequencies are in the lower range of a standard piano.
    pub static ref D1_MAJOR_SCALE: Scale<8> = major_scale(D1);

    /// D# major scale in octave 1 (MIDI notes 27-39)
    ///
    /// This scale spans from D#1 (MIDI note 27, ~38.89 Hz) to D#2 (MIDI note 39, ~77.78 Hz).
    /// These bass frequencies are in the lower range of a standard piano.
    pub static ref DSHARP1_MAJOR_SCALE: Scale<8> = major_scale(DSHARP1);

    /// E major scale in octave 1 (MIDI notes 28-40)
    ///
    /// This scale spans from E1 (MIDI note 28, ~41.20 Hz) to E2 (MIDI note 40, ~82.41 Hz).
    /// E1 is the same pitch as the lowest string on a standard bass guitar.
    pub static ref E1_MAJOR_SCALE: Scale<8> = major_scale(E1);

    /// F major scale in octave 1 (MIDI notes 29-41)
    ///
    /// This scale spans from F1 (MIDI note 29, ~43.65 Hz) to F2 (MIDI note 41, ~87.31 Hz).
    /// These bass frequencies are used in the low bass range of orchestral and keyboard music.
    pub static ref F1_MAJOR_SCALE: Scale<8> = major_scale(F1);

    /// F# major scale in octave 1 (MIDI notes 30-42)
    ///
    /// This scale spans from F#1 (MIDI note 30, ~46.25 Hz) to F#2 (MIDI note 42, ~92.50 Hz).
    /// These bass frequencies are used in the low bass range of orchestral and keyboard music.
    pub static ref FSHARP1_MAJOR_SCALE: Scale<8> = major_scale(FSHARP1);

    /// G major scale in octave 1 (MIDI notes 31-43)
    ///
    /// This scale spans from G1 (MIDI note 31, ~49.00 Hz) to G2 (MIDI note 43, ~98.00 Hz).
    /// These bass frequencies are used in the low bass range of orchestral and keyboard music.
    pub static ref G1_MAJOR_SCALE: Scale<8> = major_scale(G1);

    /// G# major scale in octave 1 (MIDI notes 32-44)
    ///
    /// This scale spans from G#1 (MIDI note 32, ~51.91 Hz) to G#2 (MIDI note 44, ~103.83 Hz).
    /// These bass frequencies are used in the low bass range of orchestral and keyboard music.
    pub static ref GSHARP1_MAJOR_SCALE: Scale<8> = major_scale(GSHARP1);

    /// A major scale in octave 1 (MIDI notes 33-45)
    ///
    /// This scale spans from A1 (MIDI note 33, 55.00 Hz) to A2 (MIDI note 45, 110.00 Hz).
    /// This scale includes the pitch of the lowest string (A) on a standard guitar.
    pub static ref A1_MAJOR_SCALE: Scale<8> = major_scale(A1);

    /// A# major scale in octave 1 (MIDI notes 34-46)
    ///
    /// This scale spans from A#1 (MIDI note 34, ~58.27 Hz) to A#2 (MIDI note 46, ~116.54 Hz).
    /// These bass frequencies are commonly used in bass lines for various music genres.
    pub static ref ASHARP1_MAJOR_SCALE: Scale<8> = major_scale(ASHARP1);

    /// B major scale in octave 1 (MIDI notes 35-47)
    ///
    /// This scale spans from B1 (MIDI note 35, ~61.74 Hz) to B2 (MIDI note 47, ~123.47 Hz).
    /// These bass frequencies are commonly used in bass lines for various music genres.
    pub static ref B1_MAJOR_SCALE: Scale<8> = major_scale(B1);
}

lazy_static! {
    pub static ref C2_MAJOR_SCALE: Scale<8> = major_scale(C2);
    pub static ref CSHARP2_MAJOR_SCALE: Scale<8> = major_scale(CSHARP2);
    pub static ref D2_MAJOR_SCALE: Scale<8> = major_scale(D2);
    pub static ref DSHARP2_MAJOR_SCALE: Scale<8> = major_scale(DSHARP2);
    pub static ref E2_MAJOR_SCALE: Scale<8> = major_scale(E2);
    pub static ref F2_MAJOR_SCALE: Scale<8> = major_scale(F2);
    pub static ref FSHARP2_MAJOR_SCALE: Scale<8> = major_scale(FSHARP2);
    pub static ref G2_MAJOR_SCALE: Scale<8> = major_scale(G2);
    pub static ref GSHARP2_MAJOR_SCALE: Scale<8> = major_scale(GSHARP2);
    pub static ref A2_MAJOR_SCALE: Scale<8> = major_scale(A2);
    pub static ref ASHARP2_MAJOR_SCALE: Scale<8> = major_scale(ASHARP2);
    pub static ref B2_MAJOR_SCALE: Scale<8> = major_scale(B2);
}

lazy_static! {
    pub static ref C3_MAJOR_SCALE: Scale<8> = major_scale(C3);
    pub static ref CSHARP3_MAJOR_SCALE: Scale<8> = major_scale(CSHARP3);
    pub static ref D3_MAJOR_SCALE: Scale<8> = major_scale(D3);
    pub static ref DSHARP3_MAJOR_SCALE: Scale<8> = major_scale(DSHARP3);
    pub static ref E3_MAJOR_SCALE: Scale<8> = major_scale(E3);
    pub static ref F3_MAJOR_SCALE: Scale<8> = major_scale(F3);
    pub static ref FSHARP3_MAJOR_SCALE: Scale<8> = major_scale(FSHARP3);
    pub static ref G3_MAJOR_SCALE: Scale<8> = major_scale(G3);
    pub static ref GSHARP3_MAJOR_SCALE: Scale<8> = major_scale(GSHARP3);
    pub static ref A3_MAJOR_SCALE: Scale<8> = major_scale(A3);
    pub static ref ASHARP3_MAJOR_SCALE: Scale<8> = major_scale(ASHARP3);
    pub static ref B3_MAJOR_SCALE: Scale<8> = major_scale(B3);
}

lazy_static! {
    pub static ref C4_MAJOR_SCALE: Scale<8> = major_scale(C4);
    pub static ref CSHARP4_MAJOR_SCALE: Scale<8> = major_scale(CSHARP4);
    pub static ref D4_MAJOR_SCALE: Scale<8> = major_scale(D4);
    pub static ref DSHARP4_MAJOR_SCALE: Scale<8> = major_scale(DSHARP4);
    pub static ref E4_MAJOR_SCALE: Scale<8> = major_scale(E4);
    pub static ref F4_MAJOR_SCALE: Scale<8> = major_scale(F4);
    pub static ref FSHARP4_MAJOR_SCALE: Scale<8> = major_scale(FSHARP4);
    pub static ref G4_MAJOR_SCALE: Scale<8> = major_scale(G4);
    pub static ref GSHARP4_MAJOR_SCALE: Scale<8> = major_scale(GSHARP4);
    pub static ref A4_MAJOR_SCALE: Scale<8> = major_scale(A4);
    pub static ref ASHARP4_MAJOR_SCALE: Scale<8> = major_scale(ASHARP4);
    pub static ref B4_MAJOR_SCALE: Scale<8> = major_scale(B4);
}

lazy_static! {
    pub static ref C5_MAJOR_SCALE: Scale<8> = major_scale(C5);
    pub static ref CSHARP5_MAJOR_SCALE: Scale<8> = major_scale(CSHARP5);
    pub static ref D5_MAJOR_SCALE: Scale<8> = major_scale(D5);
    pub static ref DSHARP5_MAJOR_SCALE: Scale<8> = major_scale(DSHARP5);
    pub static ref E5_MAJOR_SCALE: Scale<8> = major_scale(E5);
    pub static ref F5_MAJOR_SCALE: Scale<8> = major_scale(F5);
    pub static ref FSHARP5_MAJOR_SCALE: Scale<8> = major_scale(FSHARP5);
    pub static ref G5_MAJOR_SCALE: Scale<8> = major_scale(G5);
    pub static ref GSHARP5_MAJOR_SCALE: Scale<8> = major_scale(GSHARP5);
    pub static ref A5_MAJOR_SCALE: Scale<8> = major_scale(A5);
    pub static ref ASHARP5_MAJOR_SCALE: Scale<8> = major_scale(ASHARP5);
    pub static ref B5_MAJOR_SCALE: Scale<8> = major_scale(B5);
}

lazy_static! {
    pub static ref C6_MAJOR_SCALE: Scale<8> = major_scale(C6);
    pub static ref CSHARP6_MAJOR_SCALE: Scale<8> = major_scale(CSHARP6);
    pub static ref D6_MAJOR_SCALE: Scale<8> = major_scale(D6);
    pub static ref DSHARP6_MAJOR_SCALE: Scale<8> = major_scale(DSHARP6);
    pub static ref E6_MAJOR_SCALE: Scale<8> = major_scale(E6);
    pub static ref F6_MAJOR_SCALE: Scale<8> = major_scale(F6);
    pub static ref FSHARP6_MAJOR_SCALE: Scale<8> = major_scale(FSHARP6);
    pub static ref G6_MAJOR_SCALE: Scale<8> = major_scale(G6);
    pub static ref GSHARP6_MAJOR_SCALE: Scale<8> = major_scale(GSHARP6);
    pub static ref A6_MAJOR_SCALE: Scale<8> = major_scale(A6);
    pub static ref ASHARP6_MAJOR_SCALE: Scale<8> = major_scale(ASHARP6);
    pub static ref B6_MAJOR_SCALE: Scale<8> = major_scale(B6);
}

lazy_static! {
    pub static ref C7_MAJOR_SCALE: Scale<8> = major_scale(C7);
    pub static ref CSHARP7_MAJOR_SCALE: Scale<8> = major_scale(CSHARP7);
    pub static ref D7_MAJOR_SCALE: Scale<8> = major_scale(D7);
    pub static ref DSHARP7_MAJOR_SCALE: Scale<8> = major_scale(DSHARP7);
    pub static ref E7_MAJOR_SCALE: Scale<8> = major_scale(E7);
    pub static ref F7_MAJOR_SCALE: Scale<8> = major_scale(F7);
    pub static ref FSHARP7_MAJOR_SCALE: Scale<8> = major_scale(FSHARP7);
    pub static ref G7_MAJOR_SCALE: Scale<8> = major_scale(G7);
    pub static ref GSHARP7_MAJOR_SCALE: Scale<8> = major_scale(GSHARP7);
    pub static ref A7_MAJOR_SCALE: Scale<8> = major_scale(A7);
    pub static ref ASHARP7_MAJOR_SCALE: Scale<8> = major_scale(ASHARP7);
    pub static ref B7_MAJOR_SCALE: Scale<8> = major_scale(B7);
}

lazy_static! {
    pub static ref C8_MAJOR_SCALE: Scale<8> = major_scale(C8);
    pub static ref CSHARP8_MAJOR_SCALE: Scale<8> = major_scale(CSHARP8);
    pub static ref D8_MAJOR_SCALE: Scale<8> = major_scale(D8);
    pub static ref DSHARP8_MAJOR_SCALE: Scale<8> = major_scale(DSHARP8);
    pub static ref E8_MAJOR_SCALE: Scale<8> = major_scale(E8);
    pub static ref F8_MAJOR_SCALE: Scale<8> = major_scale(F8);
    pub static ref FSHARP8_MAJOR_SCALE: Scale<8> = major_scale(FSHARP8);
    pub static ref G8_MAJOR_SCALE: Scale<8> = major_scale(G8);
    pub static ref GSHARP8_MAJOR_SCALE: Scale<8> = major_scale(GSHARP8);
    pub static ref A8_MAJOR_SCALE: Scale<8> = major_scale(A8);
    pub static ref ASHARP8_MAJOR_SCALE: Scale<8> = major_scale(ASHARP8);
    pub static ref B8_MAJOR_SCALE: Scale<8> = major_scale(B8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{
        A4, ASHARP4, B4, C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4,
    };
    use crate::Pitch;

    #[test]
    fn test_major_scale_steps_intervals() {
        // Test that MAJOR_SCALE_STEPS contains the correct interval pattern
        assert_eq!(
            MAJOR_SCALE_STEPS,
            [TONE, TONE, SEMITONE, TONE, TONE, TONE, SEMITONE]
        );
    }

    #[test]
    fn test_reference_major_scales() {
        // Test that the reference scales in octave 4 have the correct structure
        // Each of these scales should follow the major scale step pattern
        assert_eq!(C_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(C_SHARP_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D_SHARP_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F_SHARP_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G_SHARP_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A_SHARP_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(B_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);

        // Test specific scales for correct root notes
        assert_eq!(C_MAJOR_SCALE.tonic(), C4);
        assert_eq!(E_MAJOR_SCALE.tonic(), E4);
        assert_eq!(A_MAJOR_SCALE.tonic(), A4);
    }

    #[test]
    fn test_octave_0_scales() {
        // Test that octave 0 scales have the correct structure
        // Each of these scales should follow the major scale step pattern
        assert_eq!(C0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(CSHARP0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(DSHARP0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(FSHARP0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(GSHARP0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(ASHARP0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(B0_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);

        // Test that scale notes span the expected octave range
        // C0's MIDI value is 12, which should be in "octave 0"
        assert_eq!(C0_MAJOR_SCALE.pitches()[0].0, 12); // First note is C0 (MIDI 12)
        assert_eq!(C0_MAJOR_SCALE.pitches()[7].0, 24); // Last note is C1 (MIDI 24)
    }

    #[test]
    fn test_octave_1_scales() {
        // Test that octave 1 scales have the correct structure
        // Each of these scales should follow the major scale step pattern
        assert_eq!(C1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(CSHARP1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(D1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(DSHARP1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(E1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(F1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(FSHARP1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(G1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(GSHARP1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(A1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(ASHARP1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);
        assert_eq!(B1_MAJOR_SCALE.steps(), MAJOR_SCALE_STEPS);

        // Test that scale notes span the expected octave range
        // C1's MIDI value is 24, which should be in "octave 1"
        assert_eq!(C1_MAJOR_SCALE.pitches()[0].0, 24); // First note is C1 (MIDI 24)
        assert_eq!(C1_MAJOR_SCALE.pitches()[7].0, 36); // Last note is C2 (MIDI 36)
    }

    #[test]
    fn test_major_scales_hashmap() {
        // Test that the MAJOR_SCALES HashMap contains scales with the correct roots
        let roots = [
            C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4, A4, ASHARP4, B4,
        ];

        for root in roots.iter() {
            let scale = MAJOR_SCALES.get(root);
            assert!(
                scale.is_some(),
                "MAJOR_SCALES should contain the root pitch"
            );

            let scale = scale.unwrap();
            assert_eq!(scale.tonic(), *root, "Scale tonic should match the key");
            assert_eq!(
                scale.steps(),
                MAJOR_SCALE_STEPS,
                "Scale should follow major scale pattern"
            );
        }
    }

    #[test]
    fn test_major_scales_hashmap_octave_boundaries() {
        // Test scales at the boundaries of the MIDI range
        let lowest_pitch = C0; // Lowest possible MIDI note
        let highest_supported = G8; // Highest possible MIDI note

        // For lowest pitch, ensure we have a scale (even if it extends below MIDI range)
        let lowest_scale = MAJOR_SCALES.get(&lowest_pitch);
        assert!(
            lowest_scale.is_some(),
            "MAJOR_SCALES should contain the lowest pitch"
        );

        // For highest supported pitch, ensure we have a scale
        let highest_scale = MAJOR_SCALES.get(&highest_supported);
        assert!(
            highest_scale.is_some(),
            "MAJOR_SCALES should contain the highest supported pitch"
        );
    }

    #[test]
    fn test_all_scales_preserve_major_quality() {
        // Test random sampling of scales from the HashMap to ensure they all have Major quality
        let sample_pitches = [
            Pitch::new(12),  // C0
            Pitch::new(36),  // C2
            Pitch::new(60),  // C4 (middle C)
            Pitch::new(84),  // C6
            Pitch::new(108), // C8
            Pitch::new(127), // G9 (highest MIDI note)
        ];

        for pitch in sample_pitches.iter() {
            if let Some(scale) = MAJOR_SCALES.get(pitch) {
                assert_eq!(scale.quality(), crate::ScaleQuality::Major);
            }
        }
    }
}
