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
    /// C major scale in octave 2 (MIDI notes 36-48)
    ///
    /// Notes: C2, D2, E2, F2, G2, A2, B2, C3
    /// MIDI note numbers: 36, 38, 40, 41, 43, 45, 47, 48
    /// Frequency range: ~65.41 Hz to ~130.81 Hz
    ///
    /// C major in octave 2 provides a rich, fundamental bass register with improved definition compared to 
    /// octave 1. This scale spans the range of the low cello, bass guitar, and male bass voice. C2 is often
    /// used as a reference bass note, with many bass guitars having their lowest string tuned to this pitch.
    /// This scale offers a balanced combination of sufficient bass weight and improved definition, making it
    /// ideal for bass lines in classical orchestral works, jazz walking bass parts, and contemporary pop/rock
    /// foundations. The octave 2 C major provides clear fundamental tones with excellent resonance and body.
    pub static ref C2_MAJOR_SCALE: Scale<8> = major_scale(C2);

    /// C# major scale in octave 2 (MIDI notes 37-49)
    ///
    /// Notes: C#2, D#2, E#2, F#2, G#2, A#2, B#2, C#3
    /// MIDI note numbers: 37, 39, 41, 42, 44, 46, 48, 49
    /// Frequency range: ~69.30 Hz to ~138.59 Hz
    ///
    /// C# major in octave 2 (often notated enharmonically as D♭ major) spans a resonant bass register with
    /// good definition. This scale provides sufficient weight for bass foundations while retaining clear pitch 
    /// definition. The C#2 pitch sits between the lower range of cello, the tuba, and bass guitar. Often used 
    /// in classical piano literature, romantic orchestral works, and modern jazz compositions, this scale offers 
    /// a warm but articulate foundation. Its slightly higher position compared to C major provides improved
    /// clarity for more active bass parts while maintaining substantial low-end presence.
    pub static ref CSHARP2_MAJOR_SCALE: Scale<8> = major_scale(CSHARP2);

    /// D major scale in octave 2 (MIDI notes 38-50)
    ///
    /// Notes: D2, E2, F#2, G2, A2, B2, C#3, D3
    /// MIDI note numbers: 38, 40, 42, 43, 45, 47, 49, 50
    /// Frequency range: ~73.42 Hz to ~146.83 Hz
    ///
    /// D major in octave 2 provides a rich bass foundation with good definition. D2 corresponds to the open D
    /// string on bass guitar and the third string on cello. This scale appears prominently in Bach's D major 
    /// orchestral suites, Handel's "Messiah," and countless classical and contemporary bass lines. The D2-D3 
    /// range offers an ideal balance between fundamental bass weight and articulate definition, making it well-suited 
    /// for melodic bass parts, walking bass lines in jazz, and fundamental harmonic support in chamber and 
    /// orchestral music. The balanced nature of this register makes it versatile across many musical genres.
    pub static ref D2_MAJOR_SCALE: Scale<8> = major_scale(D2);

    /// D# major scale in octave 2 (MIDI notes 39-51)
    ///
    /// Notes: D#2, E#2, F##2, G#2, A#2, B#2, C##3, D#3
    /// MIDI note numbers: 39, 41, 43, 44, 46, 48, 50, 51
    /// Frequency range: ~77.78 Hz to ~155.56 Hz
    ///
    /// D# major in octave 2 (almost always notated as E♭ major) spans a versatile bass register with good 
    /// definition. This scale contains the famous "Heroic" key of E♭ major, used in Beethoven's "Eroica" Symphony
    /// and many brass band compositions. In this octave, the scale provides a solid foundation for brass 
    /// instruments and string bass parts. E♭ major in octave 2 offers excellent resonance in the bass register
    /// while providing sufficient clarity for more melodically active bass lines. This range is particularly 
    /// effective in orchestral writing, brass ensemble music, and jazz compositions requiring a warm but 
    /// articulate bass foundation.
    pub static ref DSHARP2_MAJOR_SCALE: Scale<8> = major_scale(DSHARP2);

    /// E major scale in octave 2 (MIDI notes 40-52)
    ///
    /// Notes: E2, F#2, G#2, A2, B2, C#3, D#3, E3
    /// MIDI note numbers: 40, 42, 44, 45, 47, 49, 51, 52
    /// Frequency range: ~82.41 Hz to ~164.81 Hz
    ///
    /// E major in octave 2 spans a balanced bass register with excellent definition. E2 corresponds to the
    /// lowest string of a standard guitar and is a central note for much guitar-oriented music. In this octave,
    /// E major balances fundamental bass resonance with improved clarity, making it ideal for melodic bass lines
    /// and harmonic bass material in classical, rock, and jazz contexts. The clear definition of this register
    /// allows for more complex harmonic motion in the bass without sacrificing depth. Many guitar-centered
    /// compositions utilize this range for its natural resonance with the instrument, while its bright character
    /// cuts through effectively in ensemble textures.
    pub static ref E2_MAJOR_SCALE: Scale<8> = major_scale(E2);

    /// F major scale in octave 2 (MIDI notes 41-53)
    ///
    /// Notes: F2, G2, A2, B♭2, C3, D3, E3, F3
    /// MIDI note numbers: 41, 43, 45, 46, 48, 50, 52, 53
    /// Frequency range: ~87.31 Hz to ~174.61 Hz
    ///
    /// F major in octave 2 provides a warm, resonant bass register with excellent definition. This scale spans
    /// the central range of contrabass, cello, and bass guitar, offering an ideal balance between fundamental
    /// depth and articulate clarity. F major in this octave appears in Beethoven's "Pastoral" Symphony,
    /// Mozart's works, and countless classical and jazz compositions. The register provides sufficient weight
    /// for solid bass foundations while allowing for articulate melodic bass lines and clear harmonic motion.
    /// This combination of depth and definition makes it particularly effective for expressive bass parts in
    /// orchestral music, chamber works, and jazz contexts.
    pub static ref F2_MAJOR_SCALE: Scale<8> = major_scale(F2);

    /// F# major scale in octave 2 (MIDI notes 42-54)
    ///
    /// Notes: F#2, G#2, A#2, B2, C#3, D#3, E#3, F#3
    /// MIDI note numbers: 42, 44, 46, 47, 49, 51, 53, 54
    /// Frequency range: ~92.50 Hz to ~185.00 Hz
    ///
    /// F# major in octave 2 (sometimes notated as G♭ major) spans an articulate bass register with excellent
    /// definition. This scale appears in advanced piano literature, orchestral works, and jazz compositions
    /// where a clear but substantial bass is required. In this register, F# major balances fundamental bass
    /// resonance with improved clarity, providing an ideal foundation for complex harmonic progressions and
    /// more intricate bass lines. The scale crosses from the bass into the lower mid-range, offering versatility
    /// for expressive bass parts that require both foundational support and melodic articulation. Particularly
    /// effective in piano literature and chamber music requiring a warm yet clearly defined bass register.
    pub static ref FSHARP2_MAJOR_SCALE: Scale<8> = major_scale(FSHARP2);

    /// G major scale in octave 2 (MIDI notes 43-55)
    ///
    /// Notes: G2, A2, B2, C3, D3, E3, F#3, G3
    /// MIDI note numbers: 43, 45, 47, 48, 50, 52, 54, 55
    /// Frequency range: ~98.00 Hz to ~196.00 Hz
    ///
    /// G major in octave 2 spans a versatile bass register with excellent definition and resonance. G2 corresponds
    /// to the open G string on cello and the third string on bass guitar. This scale appears extensively in
    /// Bach's Orchestral Suite No. 3, Haydn's "Surprise" Symphony, and is a staple in folk, bluegrass, and
    /// country music due to its compatibility with guitar and string instrument tunings. In this register,
    /// G major provides a solid foundation with sufficient clarity for melodic bass lines and harmonic motion.
    /// The scale extends into the lower midrange, creating versatility for bass parts that bridge foundational
    /// and melodic functions in orchestral music, chamber works, and contemporary compositions.
    pub static ref G2_MAJOR_SCALE: Scale<8> = major_scale(G2);

    /// G# major scale in octave 2 (MIDI notes 44-56)
    ///
    /// Notes: G#2, A#2, B#2, C#3, D#3, E#3, F##3, G#3
    /// MIDI note numbers: 44, 46, 48, 49, 51, 53, 55, 56
    /// Frequency range: ~103.83 Hz to ~207.65 Hz
    ///
    /// G# major in octave 2 (almost always notated as A♭ major) spans an articulate bass register with excellent
    /// definition. A♭ major is featured in compositions by Chopin, Schubert's Impromptu Op. 90 No. 4, and many
    /// romantic piano works. In this octave, the scale provides a warm but clearly defined bass foundation,
    /// balancing substantial low-end presence with improved articulation. The scale extends into the lower
    /// midrange, offering versatility for bass parts that require both foundational weight and melodic clarity.
    /// Particularly effective in piano literature, orchestral writing, and jazz compositions where a rich but
    /// well-defined bass register is desirable.
    pub static ref GSHARP2_MAJOR_SCALE: Scale<8> = major_scale(GSHARP2);

    /// A major scale in octave 2 (MIDI notes 45-57)
    ///
    /// Notes: A2, B2, C#3, D3, E3, F#3, G#3, A3
    /// MIDI note numbers: 45, 47, 49, 50, 52, 54, 56, 57
    /// Frequency range: 110.00 Hz to 220.00 Hz
    ///
    /// A major in octave 2 spans a versatile range from bass to lower midrange with excellent definition. A2 (110 Hz)
    /// is the pitch of the open A string on standard guitars and the second open string on bass guitar. This scale
    /// appears in Mozart's "Turkish" Piano Sonata, Beethoven's 7th Symphony, and countless classical and popular
    /// works. In this register, A major provides a clear, resonant foundation while extending into the lower midrange,
    /// offering versatility for bass parts that combine foundational support with melodic movement. The scale's bright
    /// character is balanced with sufficient bass weight, making it effective for melodic bass lines in orchestral
    /// music, chamber works, jazz, and popular genres.
    pub static ref A2_MAJOR_SCALE: Scale<8> = major_scale(A2);

    /// A# major scale in octave 2 (MIDI notes 46-58)
    ///
    /// Notes: A#2, B#2, C##3, D#3, E#3, F##3, G##3, A#3
    /// MIDI note numbers: 46, 48, 50, 51, 53, 55, 57, 58
    /// Frequency range: ~116.54 Hz to ~233.08 Hz
    ///
    /// A# major in octave 2 (almost always notated as B♭ major) spans a clear bass-to-midrange register with
    /// excellent definition. B♭ major is a common key for brass and woodwind instruments, appearing in Mozart's
    /// Horn Concertos, Beethoven's "Emperor" Concerto, and countless classical works. In this octave, the scale
    /// provides a warm, resonant bass foundation while extending into the lower midrange, offering versatility
    /// for bass parts that require both depth and melodic clarity. This register is particularly effective for
    /// expressive bass lines in orchestral music, wind band compositions, jazz walking bass patterns, and chamber
    /// works where a rich but clearly articulated bass is desired.
    pub static ref ASHARP2_MAJOR_SCALE: Scale<8> = major_scale(ASHARP2);

    /// B major scale in octave 2 (MIDI notes 47-59)
    ///
    /// Notes: B2, C#3, D#3, E3, F#3, G#3, A#3, B3
    /// MIDI note numbers: 47, 49, 51, 52, 54, 56, 58, 59
    /// Frequency range: ~123.47 Hz to ~246.94 Hz
    ///
    /// B major in octave 2 spans a bright bass-to-midrange register with excellent definition and clarity.
    /// This scale appears in Liszt's Piano Sonata in B minor, Chopin's Piano Sonata No. 3, and orchestral
    /// works by Brahms and Tchaikovsky. In this register, B major balances bass depth with exceptional
    /// clarity, providing an ideal foundation for more harmonically complex or melodically active bass parts.
    /// The scale extends well into the lower midrange, offering versatility for bass lines that move between
    /// supportive and featured roles. Particularly effective in piano literature, string chamber music, and
    /// compositions requiring a bass register with brilliant articulation while maintaining sufficient depth.
    pub static ref B2_MAJOR_SCALE: Scale<8> = major_scale(B2);
}

lazy_static! {
    /// C major scale in octave 3 (MIDI notes 48-60)
    ///
    /// Notes: C3, D3, E3, F3, G3, A3, B3, C4
    /// MIDI note numbers: 48, 50, 52, 53, 55, 57, 59, 60
    /// Frequency range: ~130.81 Hz to ~261.63 Hz
    ///
    /// C major in octave 3 spans the crucial middle-lower register that bridges bass and mid-range. C3 corresponds
    /// to the second-lowest C on the piano and the open C string on the viola. This scale culminates at middle C (C4),
    /// making it a pivotal transitional register in Western music. The C3-C4 octave encompasses the central range of
    /// cello, viola, guitar, tenor voice, baritone voice, trombone, and many other instruments. This versatile register
    /// offers excellent balance between resonance and clarity, ideal for melodic lines in Bach's cello suites, Mozart's
    /// string quartets, and foundational material in orchestral and chamber music. The scale's warm tone and clear
    /// articulation make it particularly effective for expressive melodic passages.
    pub static ref C3_MAJOR_SCALE: Scale<8> = major_scale(C3);

    /// C# major scale in octave 3 (MIDI notes 49-61)
    ///
    /// Notes: C#3, D#3, E#3, F#3, G#3, A#3, B#3, C#4
    /// MIDI note numbers: 49, 51, 53, 54, 56, 58, 60, 61
    /// Frequency range: ~138.59 Hz to ~277.18 Hz
    ///
    /// C# major in octave 3 (often notated enharmonically as D♭ major) spans the middle-lower register. This scale 
    /// bridges from the lower register through to the central range of many instruments. C#3 is found in the comfortable
    /// playing range of viola, cello, tenor saxophone, and low clarinet. The scale extends to C#4, just above 
    /// middle C. Featured in Chopin's "Raindrop" Prelude and Beethoven's "Moonlight" Sonata, D♭ major in this register
    /// offers a rich, warm sonority with good balance between depth and clarity. This range provides an excellent
    /// foundation for expressive melodic material in orchestral writing, chamber music, and jazz compositions,
    /// balancing richness with clear articulation.
    pub static ref CSHARP3_MAJOR_SCALE: Scale<8> = major_scale(CSHARP3);

    /// D major scale in octave 3 (MIDI notes 50-62)
    ///
    /// Notes: D3, E3, F#3, G3, A3, B3, C#4, D4
    /// MIDI note numbers: 50, 52, 54, 55, 57, 59, 61, 62
    /// Frequency range: ~146.83 Hz to ~293.66 Hz
    ///
    /// D major in octave 3 spans a rich middle-lower register with excellent balance between warmth and clarity.
    /// D3 corresponds to the open D string on viola and cello. This scale traverses the central range of tenor and
    /// baritone voices, viola, cello, trombone, and guitar, extending to just above middle C. Prominently featured in
    /// Bach's Brandenburg Concertos, Mozart's Symphony No. 38, and countless classical works, D major in this octave 
    /// offers an ideal combination of resonance and articulation. The scale provides a perfect foundation for
    /// melodic material in orchestral writing, string chamber music, and solo repertoire. Its balanced sonority
    /// makes it effective for both accompaniment figures and featured melodic lines across many musical genres.
    pub static ref D3_MAJOR_SCALE: Scale<8> = major_scale(D3);

    /// D# major scale in octave 3 (MIDI notes 51-63)
    ///
    /// Notes: D#3, E#3, F##3, G#3, A#3, B#3, C##4, D#4
    /// MIDI note numbers: 51, 53, 55, 56, 58, 60, 62, 63
    /// Frequency range: ~155.56 Hz to ~311.13 Hz
    ///
    /// D# major in octave 3 (almost always notated as E♭ major) spans a versatile middle-lower register. This "heroic"
    /// key, made famous by Beethoven's Eroica Symphony, occupies a sweet spot in the ranges of many instruments.
    /// E♭ major in this octave is particularly well-suited for brass instruments, showcased in Holst's "Jupiter" from
    /// The Planets and brass band literature. The range from E♭3 through E♭4 provides a warm, rich foundation with
    /// excellent clarity, making it ideal for melodic material in orchestral writing, wind ensembles, and jazz
    /// compositions. This register offers a perfect balance of depth and articulation, effective for both
    /// accompaniment patterns and featured melodic passages in chamber and orchestral contexts.
    pub static ref DSHARP3_MAJOR_SCALE: Scale<8> = major_scale(DSHARP3);

    /// E major scale in octave 3 (MIDI notes 52-64)
    ///
    /// Notes: E3, F#3, G#3, A3, B3, C#4, D#4, E4
    /// MIDI note numbers: 52, 54, 56, 57, 59, 61, 63, 64
    /// Frequency range: ~164.81 Hz to ~329.63 Hz
    ///
    /// E major in octave 3 spans an expressive middle-lower register with excellent clarity and warmth. E3 corresponds
    /// to the lowest string on the guitar and a central note in the viola range. This scale traverses the comfortable
    /// range of many instruments and vocalists, extending up through middle C to E4. Featured in Mendelssohn's Violin
    /// Concerto, Tchaikovsky's Violin Concerto, and guitar literature where it resonates with the instrument's open
    /// strings. In this register, E major offers a bright yet warm sonority with exceptional balance between resonance
    /// and articulation. The scale provides an ideal foundation for melodic material in string writing, guitar music,
    /// chamber works, and orchestral compositions, with a clarity that projects well in ensemble contexts.
    pub static ref E3_MAJOR_SCALE: Scale<8> = major_scale(E3);

    /// F major scale in octave 3 (MIDI notes 53-65)
    ///
    /// Notes: F3, G3, A3, B♭3, C4, D4, E4, F4
    /// MIDI note numbers: 53, 55, 57, 58, 60, 62, 64, 65
    /// Frequency range: ~174.61 Hz to ~349.23 Hz
    ///
    /// F major in octave 3 spans a warm, resonant register that encompasses middle C (C4) at its midpoint. F3 sits
    /// comfortably in the tenor voice range and the central register of viola, cello, and bassoon. This scale has been
    /// featured prominently in Mozart's Symphony No. 41, Beethoven's "Pastoral" Symphony, and countless chamber works.
    /// F major in this octave offers exceptional balance between warmth and clarity, providing an ideal range for
    /// melodic material in orchestral writing, keyboard works, and vocal compositions. With middle C at its center,
    /// this register serves as a crucial bridge between lower and upper ranges, making it versatile for both
    /// foundational harmonic content and featured melodic passages in many musical contexts.
    pub static ref F3_MAJOR_SCALE: Scale<8> = major_scale(F3);

    /// F# major scale in octave 3 (MIDI notes 54-66)
    ///
    /// Notes: F#3, G#3, A#3, B3, C#4, D#4, E#4, F#4
    /// MIDI note numbers: 54, 56, 58, 59, 61, 63, 65, 66
    /// Frequency range: ~185.00 Hz to ~369.99 Hz
    ///
    /// F# major in octave 3 (sometimes notated as G♭ major) spans a resonant middle register with excellent clarity.
    /// This scale bridges from F#3 through middle C to F#4, encompassing the sweet spot for viola, cello, tenor voice,
    /// and many wind instruments. Featured in Chopin's "Raindrop" Prelude, Ravel's Piano Concerto, and Liszt's Hungarian
    /// Rhapsodies, F# major in this register offers a rich sonority with exceptional balance between warmth and
    /// brilliance. The scale provides an ideal foundation for expressive melodic lines in piano literature, chamber music,
    /// and orchestral compositions. With middle C near its center, this register serves as a pivotal connection between
    /// lower and upper ranges, offering versatility for both supporting material and featured passages.
    pub static ref FSHARP3_MAJOR_SCALE: Scale<8> = major_scale(FSHARP3);

    /// G major scale in octave 3 (MIDI notes 55-67)
    ///
    /// Notes: G3, A3, B3, C4, D4, E4, F#4, G4
    /// MIDI note numbers: 55, 57, 59, 60, 62, 64, 66, 67
    /// Frequency range: ~196.00 Hz to ~392.00 Hz
    ///
    /// G major in octave 3 spans a versatile middle register, beginning at G3 (open G string on violin and viola)
    /// and extending through middle C (C4) to G4. This scale occupies the central range of violin, viola, tenor voice,
    /// and many wind instruments. Featured prominently in Mozart's "Eine kleine Nachtmusik," Haydn's "Surprise" Symphony,
    /// and countless classical, folk, and popular works. G major in this octave offers perfect balance between warmth
    /// and clarity, with exceptional resonance on string instruments due to open string relationships. The scale provides
    /// an ideal foundation for melodic material in orchestral writing, string chamber music, and solo repertoire.
    /// With middle C at its midpoint, this register serves as a crucial bridge between lower and upper ranges,
    /// making it one of the most practical and versatile scales in Western music.
    pub static ref G3_MAJOR_SCALE: Scale<8> = major_scale(G3);

    /// G# major scale in octave 3 (MIDI notes 56-68)
    ///
    /// Notes: G#3, A#3, B#3, C#4, D#4, E#4, F##4, G#4
    /// MIDI note numbers: 56, 58, 60, 61, 63, 65, 67, 68
    /// Frequency range: ~207.65 Hz to ~415.30 Hz
    ///
    /// G# major in octave 3 (almost always notated as A♭ major) spans a brilliant middle register with exceptional
    /// clarity. Beginning at G#3 and extending through middle C to G#4, this scale occupies the sweet spot for many
    /// instruments and vocalists. A♭ major is featured in Chopin's Ballade No. 3, Grieg's Piano Concerto, and
    /// Schubert's Impromptu Op. 90 No. 4. In this register, A♭ major offers a warm yet brilliant sonority with
    /// excellent projection and articulation. The scale provides an ideal foundation for expressive melodic lines
    /// in piano literature, vocal works, and orchestral compositions. With middle C near its midpoint, this register
    /// serves as a pivotal connection between lower and upper ranges, offering versatility for both supporting
    /// material and featured melodic passages across many musical genres.
    pub static ref GSHARP3_MAJOR_SCALE: Scale<8> = major_scale(GSHARP3);

    /// A major scale in octave 3 (MIDI notes 57-69)
    ///
    /// Notes: A3, B3, C#4, D4, E4, F#4, G#4, A4
    /// MIDI note numbers: 57, 59, 61, 62, 64, 66, 68, 69
    /// Frequency range: 220.00 Hz to 440.00 Hz
    ///
    /// A major in octave 3 spans a crucial middle register, beginning at A3 (open A string on violin and viola) and
    /// extending through middle C to concert pitch A4 (440 Hz). This scale encompasses the central range of violin,
    /// flute, clarinet, soprano voice, and many other instruments. A3 corresponds to the open A string on violin, viola,
    /// and cello, making it particularly resonant on string instruments. Featured in Mozart's Clarinet Concerto,
    /// Vivaldi's "Spring" Concerto, and Beethoven's 7th Symphony. A major in this octave offers ideal balance between
    /// warmth and brilliance, with exceptional projection and clarity. The scale provides a perfect foundation for
    /// melodic material in orchestral writing, string chamber music, and solo repertoire. Spanning from 220 Hz to 440 Hz,
    /// this register occupies the range where human hearing is most sensitive, making it especially effective for
    /// featured melodic passages.
    pub static ref A3_MAJOR_SCALE: Scale<8> = major_scale(A3);

    /// A# major scale in octave 3 (MIDI notes 58-70)
    ///
    /// Notes: A#3, B#3, C##4, D#4, E#4, F##4, G##4, A#4
    /// MIDI note numbers: 58, 60, 62, 63, 65, 67, 69, 70
    /// Frequency range: ~233.08 Hz to ~466.16 Hz
    ///
    /// A# major in octave 3 (almost always notated as B♭ major) spans a brilliant middle register with exceptional
    /// resonance. This scale encompasses the sweet spot for many brass and woodwind instruments, which often have
    /// B♭ as their fundamental key. Featured in Mozart's Horn Concertos, Haydn's Trumpet Concerto, and countless
    /// classical and jazz works. B♭ major in this register offers a perfect balance of warmth and brilliance, with
    /// excellent projection and clarity. Beginning at B♭3 and extending through middle C to B♭4, this scale provides
    /// an ideal foundation for melodic material in wind band music, brass ensemble literature, and jazz compositions.
    /// With middle C at its center, this register serves as a crucial bridge between lower and upper ranges, making it
    /// one of the most practical and versatile scales for wind and brass instruments.
    pub static ref ASHARP3_MAJOR_SCALE: Scale<8> = major_scale(ASHARP3);

    /// B major scale in octave 3 (MIDI notes 59-71)
    ///
    /// Notes: B3, C#4, D#4, E4, F#4, G#4, A#4, B4
    /// MIDI note numbers: 59, 61, 63, 64, 66, 68, 70, 71
    /// Frequency range: ~246.94 Hz to ~493.88 Hz
    ///
    /// B major in octave 3 spans a brilliant middle register with exceptional clarity and projection. Beginning
    /// at B3 and extending through middle C to B4, this scale occupies the sweet spot for many instruments and vocalists.
    /// Featured in Chopin's Piano Sonata No. 3, Liszt's Piano Sonata in B minor, and Brahms' B major Trio.
    /// In this register, B major offers a bright, radiant sonority with excellent articulation and brilliance.
    /// The scale provides an ideal foundation for virtuosic passages in piano literature, violin concertos, and
    /// orchestral compositions. With middle C near its lower portion, this register shifts the balance toward the upper
    /// midrange, offering exceptional clarity for melodic material while maintaining sufficient warmth. Particularly
    /// effective for passages requiring both expressive depth and technical brilliance across many musical genres.
    pub static ref B3_MAJOR_SCALE: Scale<8> = major_scale(B3);
}

lazy_static! {
    /// C major scale in octave 4 (MIDI notes 60-72)
    ///
    /// Notes: C4, D4, E4, F4, G4, A4, B4, C5
    /// MIDI note numbers: 60, 62, 64, 65, 67, 69, 71, 72
    /// Frequency range: ~261.63 Hz to ~523.25 Hz
    ///
    /// C major in octave 4 spans the central register of Western music, beginning with middle C (C4).
    /// This scale encompasses the sweet spot for soprano voice, violin, flute, clarinet, trumpet, and
    /// the right hand in piano music. Middle C marks a pivotal reference point in musical notation and
    /// instrument ranges. Featured prominently in Mozart's Piano Sonata in C Major K.545, Bach's Prelude
    /// in C from WTC Book I, and countless classical, jazz, and popular works. C major in this octave
    /// offers exceptional clarity and projection, with perfect balance between warmth and brilliance.
    /// The scale provides an optimal range for melodic material in solo repertoire, chamber music, and
    /// orchestral writing. This register sits in the range where human hearing is most sensitive,
    /// making it ideal for primary thematic material across virtually all Western musical genres.
    pub static ref C4_MAJOR_SCALE: Scale<8> = major_scale(C4);

    /// C# major scale in octave 4 (MIDI notes 61-73)
    ///
    /// Notes: C#4, D#4, E#4, F#4, G#4, A#4, B#4, C#5
    /// MIDI note numbers: 61, 63, 65, 66, 68, 70, 72, 73
    /// Frequency range: ~277.18 Hz to ~554.37 Hz
    ///
    /// C# major in octave 4 (often notated enharmonically as D♭ major) spans the brilliant central
    /// register of Western music. Beginning just above middle C, this scale encompasses the optimal
    /// range for soprano voice, violin, flute, and piano. Featured in Chopin's "Black Key" Etude,
    /// Debussy's "Clair de Lune" (as D♭), and Liszt's Consolation No. 3. In this register, C# major
    /// offers a shimmering, luminous quality with exceptional projection and clarity. The scale provides
    /// an ideal foundation for virtuosic passages in piano literature, expressive melodic lines in
    /// romantic repertoire, and colorful harmonic material in impressionist works. This register 
    /// balances warmth with brilliance, making it particularly effective for emotionally expressive
    /// melodic material in chamber music, art songs, and solo piano literature.
    pub static ref CSHARP4_MAJOR_SCALE: Scale<8> = major_scale(CSHARP4);

    /// D major scale in octave 4 (MIDI notes 62-74)
    ///
    /// Notes: D4, E4, F#4, G4, A4, B4, C#5, D5
    /// MIDI note numbers: 62, 64, 66, 67, 69, 71, 73, 74
    /// Frequency range: ~293.66 Hz to ~587.33 Hz
    ///
    /// D major in octave 4 spans the brilliant central-to-upper register of Western music. This scale 
    /// encompasses the optimal range for soprano voice, violin (including the natural resonance of
    /// its open strings), flute, and oboe. Featured prominently in Beethoven's Violin Concerto,
    /// Mozart's "Jupiter" Symphony, Handel's "Messiah," and countless orchestral works. D major in this
    /// octave offers exceptional clarity, projection, and brilliance. Beginning above middle C and
    /// extending to D5, this scale provides an ideal foundation for virtuosic passages, melodic material,
    /// and thematic statements. This register balances the warmth of the middle range with the brilliance
    /// of the upper, making it particularly effective for featured melodic lines in concertos,
    /// sonatas, and symphonic works across the Classical and Romantic periods.
    pub static ref D4_MAJOR_SCALE: Scale<8> = major_scale(D4);

    /// D# major scale in octave 4 (MIDI notes 63-75)
    ///
    /// Notes: D#4, E#4, F##4, G#4, A#4, B#4, C##5, D#5
    /// MIDI note numbers: 63, 65, 67, 68, 70, 72, 74, 75
    /// Frequency range: ~311.13 Hz to ~622.25 Hz
    ///
    /// D# major in octave 4 (almost always notated as E♭ major) spans the brilliant central-to-upper register.
    /// This "heroic" key, made famous by Beethoven's "Emperor" Concerto and "Eroica" Symphony, occupies an
    /// optimal range for clarinet, trumpet, horn, and soprano voice. E♭ major is featured prominently in
    /// Mozart's "Magic Flute," Strauss's "Ein Heldenleben," and much symphonic and wind band literature.
    /// In this register, the scale offers exceptional clarity and projection, with a bright yet warm
    /// sonority. Beginning above middle C and extending upward, this range provides an ideal foundation
    /// for virtuosic passages, heroic themes, and brilliant melodic lines. This register combines the
    /// warmth of middle range with the brilliance of upper registers, making it particularly effective
    /// for featured melodic material in concertos, symphonic works, and wind ensemble compositions.
    pub static ref DSHARP4_MAJOR_SCALE: Scale<8> = major_scale(DSHARP4);

    /// E major scale in octave 4 (MIDI notes 64-76)
    ///
    /// Notes: E4, F#4, G#4, A4, B4, C#5, D#5, E5
    /// MIDI note numbers: 64, 66, 68, 69, 71, 73, 75, 76
    /// Frequency range: ~329.63 Hz to ~659.26 Hz
    ///
    /// E major in octave 4 spans a brilliant central-to-upper register with exceptional clarity and
    /// brilliance. This scale occupies the sweet spot for soprano voice, violin, flute, and the right
    /// hand in piano music. Featured in Beethoven's "Pastoral" Symphony, Mendelssohn's Violin Concerto,
    /// and guitar literature where it resonates with the instrument's open strings. In this register,
    /// E major offers a bright, radiant sonority with exceptional projection. Beginning above middle C
    /// and extending upward, this scale provides an ideal foundation for virtuosic passages in violin
    /// concertos, brilliant melodic lines in orchestral writing, and expressive material in piano and
    /// chamber music. This register combines clarity with brilliance, making it particularly effective
    /// for featured melodic lines and climactic passages in symphonic and concerto literature.
    pub static ref E4_MAJOR_SCALE: Scale<8> = major_scale(E4);

    /// F major scale in octave 4 (MIDI notes 65-77)
    ///
    /// Notes: F4, G4, A4, B♭4, C5, D5, E5, F5
    /// MIDI note numbers: 65, 67, 69, 70, 72, 74, 76, 77
    /// Frequency range: ~349.23 Hz to ~698.46 Hz
    ///
    /// F major in octave 4 spans a brilliant middle-to-upper register with exceptional clarity and projection.
    /// This scale encompasses the optimal range for soprano voice, violin, flute, and the right hand in
    /// piano music. F major appears prominently in Beethoven's "Pastoral" Symphony, Mozart's Piano Concerto
    /// No. 19, and countless classical and romantic works. In this register, F major offers a bright yet
    /// warm sonority with excellent projection. Beginning above middle C and extending to the upper register,
    /// this scale provides an ideal foundation for lyrical melodic lines in solo repertoire, chamber music,
    /// and orchestral writing. With the standard tuning reference A4=440Hz at its center, this register
    /// occupies the most sensitive range of human hearing, making it particularly effective for expressive
    /// melodic material across virtually all Western musical genres.
    pub static ref F4_MAJOR_SCALE: Scale<8> = major_scale(F4);

    /// F# major scale in octave 4 (MIDI notes 66-78)
    ///
    /// Notes: F#4, G#4, A#4, B4, C#5, D#5, E#5, F#5
    /// MIDI note numbers: 66, 68, 70, 71, 73, 75, 77, 78
    /// Frequency range: ~369.99 Hz to ~739.99 Hz
    ///
    /// F# major in octave 4 (sometimes notated as G♭ major) spans a brilliant middle-to-upper register with
    /// exceptional clarity and resonance. This scale occupies the sweet spot for coloratura soprano, violin,
    /// flute, and high register piano writing. Featured in Chopin's Impromptu in G♭ major, Ravel's "Jeux d'eau,"
    /// and Liszt's Consolation No. 3. In this register, F# major offers a shimmering, radiant quality with
    /// excellent projection and brilliance. Beginning well above middle C and extending to the upper register,
    /// this scale provides an ideal foundation for virtuosic passages in piano literature, expressive melodic
    /// lines in romantic repertoire, and coloristic harmonic material in impressionist works. This register
    /// balances warmth with brilliance, making it effective for featured melodic material in piano solos,
    /// art songs, and chamber music requiring exceptional color and brilliance.
    pub static ref FSHARP4_MAJOR_SCALE: Scale<8> = major_scale(FSHARP4);

    /// G major scale in octave 4 (MIDI notes 67-79)
    ///
    /// Notes: G4, A4, B4, C5, D5, E5, F#5, G5
    /// MIDI note numbers: 67, 69, 71, 72, 74, 76, 78, 79
    /// Frequency range: ~392.00 Hz to ~784.00 Hz
    ///
    /// G major in octave 4 spans a brilliant middle-to-upper register, beginning at G4 (above middle C) and
    /// extending into the brilliant upper range. This scale encompasses the optimal range for soprano voice,
    /// violin, flute, and the upper register of many instruments. G major appears prominently in Haydn's
    /// "Surprise" Symphony, Mozart's "Eine kleine Nachtmusik," Bach's Brandenburg Concerto No. 3, and countless
    /// classical, folk, and popular works. In this register, G major offers exceptional brilliance and projection,
    /// with a bright, clear sonority. The scale includes A4 (440 Hz), the standard tuning reference, and provides
    /// an ideal foundation for virtuosic passages in violin concertos, brilliant melodic lines in orchestral
    /// writing, and featured material in solo repertoire. This register optimizes both clarity and brilliance,
    /// making it particularly effective for principal themes and melodic material in virtually all Western
    /// musical genres.
    pub static ref G4_MAJOR_SCALE: Scale<8> = major_scale(G4);

    /// G# major scale in octave 4 (MIDI notes 68-80)
    ///
    /// Notes: G#4, A#4, B#4, C#5, D#5, E#5, F##5, G#5
    /// MIDI note numbers: 68, 70, 72, 73, 75, 77, 79, 80
    /// Frequency range: ~415.30 Hz to ~830.61 Hz
    ///
    /// G# major in octave 4 (almost always notated as A♭ major) spans a brilliant upper-middle register with
    /// exceptional clarity and resonance. This scale occupies the optimal range for coloratura soprano, violin,
    /// flute, and piano. A♭ major is featured in Chopin's "Heroic" Polonaise, Schubert's Impromptu Op. 90 No. 4,
    /// and Grieg's Lyric Pieces. In this register, A♭ major offers a shimmering, brilliant quality with exceptional
    /// projection. Beginning well above middle C and extending to the upper register, this scale provides an ideal
    /// foundation for virtuosic passages in piano literature, expressive melodic lines in romantic repertoire, and
    /// coloristic material in impressionist works. This register emphasizes brilliance while retaining sufficient
    /// warmth, making it particularly effective for featured melodic material in piano solos, art songs, and
    /// chamber music requiring exceptional color and expressive intensity.
    pub static ref GSHARP4_MAJOR_SCALE: Scale<8> = major_scale(GSHARP4);

    /// A major scale in octave 4 (MIDI notes 69-81)
    ///
    /// Notes: A4, B4, C#5, D5, E5, F#5, G#5, A5
    /// MIDI note numbers: 69, 71, 73, 74, 76, 78, 80, 81
    /// Frequency range: 440.00 Hz to 880.00 Hz
    ///
    /// A major in octave 4 spans a brilliant upper-middle register, beginning with A4 (the standard tuning
    /// reference at 440 Hz) and extending into the brilliant upper range. This scale encompasses the optimal
    /// range for coloratura soprano, violin, flute, and piccolo. A major appears prominently in Mozart's Clarinet
    /// Concerto, Vivaldi's "Spring" Concerto, and many classical and romantic works. In this register, A major
    /// offers exceptional brilliance and projection, with a radiant, clear sonority. The scale provides an ideal
    /// foundation for virtuosic passages in violin concertos, brilliant melodic lines in orchestral writing, and
    /// featured material in solo repertoire. This register emphasizes brilliance and clarity, making it
    /// particularly effective for virtuosic passages and climactic material in concertos, symphonic works, and
    /// chamber music. The span from 440 Hz to 880 Hz represents a perfect octave at the center of human hearing
    /// sensitivity.
    pub static ref A4_MAJOR_SCALE: Scale<8> = major_scale(A4);

    /// A# major scale in octave 4 (MIDI notes 70-82)
    ///
    /// Notes: A#4, B#4, C##5, D#5, E#5, F##5, G##5, A#5
    /// MIDI note numbers: 70, 72, 74, 75, 77, 79, 81, 82
    /// Frequency range: ~466.16 Hz to ~932.33 Hz
    ///
    /// A# major in octave 4 (almost always notated as B♭ major) spans a brilliant upper register with exceptional
    /// clarity and projection. This scale occupies the sweet spot for coloratura soprano, violin, flute, and
    /// piccolo. B♭ major is featured in Mozart's Horn Concertos, Dvořák's "New World" Symphony, and is common
    /// for brass and woodwind instruments due to their construction. In this register, B♭ major offers a bright,
    /// brilliant sonority with exceptional projection. Beginning well above middle C and extending to the upper
    /// register, this scale provides an ideal foundation for virtuosic passages in concertos, brilliant melodic
    /// lines in orchestral writing, and featured material in wind band literature. This register emphasizes
    /// brilliance and clarity, making it particularly effective for virtuosic passages in instrumental concertos
    /// and climactic material in symphonic works across Classical, Romantic, and contemporary musical contexts.
    pub static ref ASHARP4_MAJOR_SCALE: Scale<8> = major_scale(ASHARP4);

    /// B major scale in octave 4 (MIDI notes 71-83)
    ///
    /// Notes: B4, C#5, D#5, E5, F#5, G#5, A#5, B5
    /// MIDI note numbers: 71, 73, 75, 76, 78, 80, 82, 83
    /// Frequency range: ~493.88 Hz to ~987.77 Hz
    ///
    /// B major in octave 4 spans a brilliant upper register with exceptional clarity and projection. This scale
    /// occupies the optimal range for coloratura soprano, violin, flute, and piccolo. Featured in Chopin's
    /// "Raindrop" Prelude, Liszt's Piano Sonata in B minor, and Brahms' B major Trio. In this register, B major
    /// offers a radiant, luminous quality with excellent projection and brilliance. Beginning well above middle C
    /// and extending to the upper register, this scale provides an ideal foundation for virtuosic passages in
    /// piano literature, brilliant melodic lines in violin concertos, and climactic material in orchestral writing.
    /// This register maximizes brilliance and clarity, making it particularly effective for virtuosic passages and
    /// climactic material in concertos, symphonic works, and chamber music requiring exceptional projection and
    /// expressive intensity. The nearly 1 kHz upper range approaches the brilliance threshold of human hearing.
    pub static ref B4_MAJOR_SCALE: Scale<8> = major_scale(B4);
}

lazy_static! {
    /// C major scale in octave 5 (MIDI notes 72-84)
    ///
    /// Notes: C5, D5, E5, F5, G5, A5, B5, C6
    /// MIDI note numbers: 72, 74, 76, 77, 79, 81, 83, 84
    /// Frequency range: ~523.25 Hz to ~1046.50 Hz
    ///
    /// C major in octave 5 spans a brilliant register that occupies the upper range of many instruments.
    /// This scale is central to the soprano vocal range, the upper register of violins, and the comfortable
    /// range of flutes. The frequencies in this register produce clear, bright tones with exceptional
    /// projection and brilliance. C5 (523.25 Hz) is commonly known as "high C" for many instruments and
    /// sits at the top of the standard tenor vocal range. This scale is prominently featured in virtuosic
    /// violin passages from concertos by Tchaikovsky, Mendelssohn, and Paganini. In orchestral writing,
    /// this register creates piercing, brilliant melodic lines that cut through dense textures. In piano
    /// literature, these notes occupy the upper middle register, featured prominently in the sparkling
    /// passages of Chopin's Études and the luminous textures of Debussy's Preludes. The brilliant yet
    /// refined quality of this register makes it ideal for expressive melodic material requiring both
    /// clarity and emotional intensity.
    pub static ref C5_MAJOR_SCALE: Scale<8> = major_scale(C5);

    /// C# major scale in octave 5 (MIDI notes 73-85)
    ///
    /// Notes: C#5, D#5, E#5, F#5, G#5, A#5, B#5, C#6
    /// MIDI note numbers: 73, 75, 77, 78, 80, 82, 84, 85
    /// Frequency range: ~554.37 Hz to ~1108.73 Hz
    ///
    /// C# major in octave 5 (often notated enharmonically as D♭ major) occupies a brilliant upper register
    /// that represents the height of expressive potential for many instruments. This scale spans the prime
    /// coloratura soprano range and the upper register of violins, flutes, and oboes. The frequencies in
    /// this range (reaching toward 1.1 kHz) produce exceptionally bright, penetrating tones with remarkable
    /// presence. D♭ major in this register features prominently in virtuosic operatic arias, including the
    /// famous "Bell Song" from Delibes' "Lakmé." In piano literature, this register creates the sparkling,
    /// crystalline sonorities found in Chopin's "Black Key" Étude (Op. 10, No. 5) and Liszt's transcendental
    /// études. For orchestral writing, these notes provide brilliant, soaring melodies with exceptional
    /// projection, cutting through even the densest textures. The combination of extreme clarity and emotional
    /// power makes this register particularly effective for expressing moments of transcendence, ecstasy, and
    /// intense lyricism in both solo and ensemble contexts.
    pub static ref CSHARP5_MAJOR_SCALE: Scale<8> = major_scale(CSHARP5);

    /// D major scale in octave 5 (MIDI notes 74-86)
    ///
    /// Notes: D5, E5, F#5, G5, A5, B5, C#6, D6
    /// MIDI note numbers: 74, 76, 78, 79, 81, 83, 85, 86
    /// Frequency range: ~587.33 Hz to ~1174.66 Hz
    ///
    /// D major in octave 5 spans a brilliant upper register that represents the pinnacle of expressive
    /// capability for many instruments and vocalists. This scale occupies the heart of the coloratura soprano
    /// range, the upper register of violins, and the brilliant register of flutes and piccolo. The frequencies
    /// in this range produce exceptionally clear, penetrating tones with remarkable projection and presence.
    /// In orchestral writing, this register creates soaring, incisive melodic lines that cut through dense
    /// textures, featured prominently in virtuosic violin passages from Sibelius' Violin Concerto and
    /// Tchaikovsky's orchestral works. For piano, these notes occupy the upper middle register, creating
    /// the crystalline brilliance found in Beethoven's "Waldstein" Sonata and Ravel's "Jeux d'eau."
    /// The exceptional clarity and brightness of this register makes it ideal for expressing moments of
    /// triumph, transcendence, and intense emotion, where projection and brilliance are paramount to the
    /// musical narrative.
    pub static ref D5_MAJOR_SCALE: Scale<8> = major_scale(D5);

    /// D# major scale in octave 5 (MIDI notes 75-87)
    ///
    /// Notes: D#5, E#5, F##5, G#5, A#5, B#5, C##6, D#6
    /// MIDI note numbers: 75, 77, 79, 80, 82, 84, 86, 87
    /// Frequency range: ~622.25 Hz to ~1244.51 Hz
    ///
    /// D# major in octave 5 (almost always notated as E♭ major) spans an extremely brilliant register at the
    /// upper limits of many instruments and vocal ranges. This scale occupies the coloratura soprano's most
    /// expressive range and the upper register of violins, flutes, and oboes. At these frequencies (approaching
    /// 1.2 kHz), tones produce exceptional brilliance and penetrating power, with a crystalline quality that
    /// cuts through orchestral textures. E♭ major in this register features prominently in virtuosic operatic
    /// writing, such as the Queen of the Night's second aria from Mozart's "Magic Flute," and in brilliant
    /// orchestral passages in Strauss' "Don Juan" and Mahler's symphonies. On piano, these notes create the
    /// radiant, sparkling sonorities found in Liszt's transcendental études and Ravel's impressionistic works.
    /// The extreme clarity and brightness of this register make it particularly effective for expressing
    /// moments of triumph, transcendence, and intense emotion, especially in passages requiring exceptional
    /// projection and brilliant sonority.
    pub static ref DSHARP5_MAJOR_SCALE: Scale<8> = major_scale(DSHARP5);

    /// E major scale in octave 5 (MIDI notes 76-88)
    ///
    /// Notes: E5, F#5, G#5, A5, B5, C#6, D#6, E6
    /// MIDI note numbers: 76, 78, 80, 81, 83, 85, 87, 88
    /// Frequency range: ~659.26 Hz to ~1318.51 Hz
    ///
    /// E major in octave 5 spans an exceptionally brilliant register that approaches the upper limits of most
    /// instruments and vocal ranges. This scale occupies the highest comfortable range of coloratura sopranos,
    /// the upper register of violins, and the brilliant register of flutes. At these frequencies (approaching
    /// 1.3 kHz), tones produce extraordinary brilliance with a piercing, crystalline quality that cuts through
    /// any musical texture. E major in this register appears in virtuosic vocal writing, including the
    /// stratospheric passages in Donizetti's "Lucia di Lammermoor" and the brilliant cadenzas in Mozart's
    /// "Queen of the Night" aria. For violin, these notes create the soaring brilliance heard in the highest
    /// passages of Paganini's Caprices and Tchaikovsky's Violin Concerto. On piano, this register produces the
    /// sparkling, luminous tones featured in Liszt's "La Campanella" and Debussy's "L'Isle Joyeuse." The
    /// exceptional clarity and penetrating power of this register make it particularly effective for expressing
    /// transcendent moments, ecstatic emotion, and brilliant virtuosity where extreme projection and
    /// crystalline sonority are essential to the musical expression.
    pub static ref E5_MAJOR_SCALE: Scale<8> = major_scale(E5);

    /// F major scale in octave 5 (MIDI notes 77-89)
    ///
    /// Notes: F5, G5, A5, B♭5, C6, D6, E6, F6
    /// MIDI note numbers: 77, 79, 81, 82, 84, 86, 88, 89
    /// Frequency range: ~698.46 Hz to ~1396.91 Hz
    ///
    /// F major in octave 5 spans an extraordinarily brilliant register that approaches the extreme upper limit
    /// for most instruments and vocalists. This scale occupies the highest range of coloratura sopranos, the
    /// upper register of violins, and the brilliant register of flutes and piccolo. At these frequencies
    /// (approaching 1.4 kHz), tones produce exceptional brilliance with a piercing, crystalline quality that
    /// cuts through any orchestral texture. F major in this register is featured in the most demanding coloratura
    /// arias, including the stratospheric passages in Bellini's "I Puritani" and the virtuosic cadenzas in
    /// Donizetti's "Daughter of the Regiment." For orchestral writing, these notes create the brilliant,
    /// soaring passages in Tchaikovsky's ballets and the climactic moments in Strauss' tone poems. On piano,
    /// this register produces the radiant, sparkling sonorities found in Liszt's "Hungarian Rhapsodies" and
    /// Ravel's "Gaspard de la Nuit." The extreme clarity and penetrating power of this register make it
    /// particularly effective for expressing transcendent moments, brilliant virtuosity, and ecstatic emotion
    /// where exceptional projection and crystalline sonority are essential to the musical narrative.
    pub static ref F5_MAJOR_SCALE: Scale<8> = major_scale(F5);

    /// F# major scale in octave 5 (MIDI notes 78-90)
    ///
    /// Notes: F#5, G#5, A#5, B5, C#6, D#6, E#6, F#6
    /// MIDI note numbers: 78, 80, 82, 83, 85, 87, 89, 90
    /// Frequency range: ~739.99 Hz to ~1479.98 Hz
    ///
    /// F# major in octave 5 (sometimes notated as G♭ major) spans an extremely brilliant register at the upper
    /// limits of most instruments and vocal capabilities. This scale occupies the highest range of coloratura
    /// sopranos, the extreme upper register of violins, and the brilliant register of flutes and piccolo. At
    /// these frequencies (approaching 1.5 kHz), tones produce extraordinary brilliance with a piercing,
    /// crystalline quality that cuts through any musical texture. F# major in this register features in the
    /// most demanding operatic coloratura passages, such as the extreme high notes in Bernstein's "Glitter and
    /// Be Gay" from "Candide." In orchestral writing, these notes create the brilliant, soaring violin passages
    /// in Mahler's symphonies and the climactic moments in Richard Strauss' "Also Sprach Zarathustra." On piano,
    /// this register produces the sparkling, radiant sonorities found in Liszt's "Transcendental Études" and
    /// Ravel's "Jeux d'eau." The extreme clarity and penetrating power of this register make it ideal for
    /// expressing transcendent moments, brilliant virtuosity, and ecstatic emotion where exceptional projection
    /// and crystalline brilliance are essential to the musical expression.
    pub static ref FSHARP5_MAJOR_SCALE: Scale<8> = major_scale(FSHARP5);

    /// G major scale in octave 5 (MIDI notes 79-91)
    ///
    /// Notes: G5, A5, B5, C6, D6, E6, F#6, G6
    /// MIDI note numbers: 79, 81, 83, 84, 86, 88, 90, 91
    /// Frequency range: ~784.00 Hz to ~1568.00 Hz
    ///
    /// G major in octave 5 spans an extremely brilliant register at the upper threshold of most instruments
    /// and vocal capabilities. This scale occupies the extreme high range of coloratura sopranos, the highest
    /// practical register of violins, and the brilliant register of flutes and piccolo. At these frequencies
    /// (approaching 1.6 kHz), tones produce extraordinary brilliance with a piercing, crystalline quality that
    /// cuts through any musical texture. G major in this register features in the most demanding operatic
    /// coloratura, such as the stratospheric passages in Strauss' "Zerbinetta's Aria" from "Ariadne auf Naxos."
    /// In orchestral writing, these notes create the brilliant, soaring violin passages in Tchaikovsky's
    /// "Violin Concerto" and the piccolo features in Shostakovich's symphonies. On piano, this register produces
    /// the radiant, sparkling sonorities found in Liszt's "La Campanella" and Ravel's "Scarbo." The extreme
    /// clarity and penetrating power of this register make it particularly effective for expressing transcendent
    /// moments, brilliant virtuosity, and ecstatic emotion where exceptional projection and crystalline sonority
    /// are essential to the musical narrative. The upper notes approach the threshold where brilliance begins to
    /// dominate over tonal complexity.
    pub static ref G5_MAJOR_SCALE: Scale<8> = major_scale(G5);

    /// G# major scale in octave 5 (MIDI notes 80-92)
    ///
    /// Notes: G#5, A#5, B#5, C#6, D#6, E#6, F##6, G#6
    /// MIDI note numbers: 80, 82, 84, 85, 87, 89, 91, 92
    /// Frequency range: ~830.61 Hz to ~1661.22 Hz
    ///
    /// G# major in octave 5 (almost always notated as A♭ major) spans an extraordinarily brilliant register
    /// at the extreme upper limit of most instruments and vocal capabilities. This scale occupies the uppermost
    /// range of coloratura sopranos, the highest register of violins, and the brilliant register of piccolo.
    /// At these frequencies (exceeding 1.6 kHz at the top), tones produce exceptional brilliance with a piercing,
    /// crystalline quality that cuts through any orchestral texture. A♭ major in this register appears in the
    /// most demanding operatic coloratura, including the extreme high passages in Mozart's "Queen of the Night"
    /// aria and Strauss' "Zerbinetta's Aria." In orchestral writing, these notes create the brilliant,
    /// stratospheric violin passages in Mahler's symphonies and the shimmering effects in Debussy's orchestral
    /// works. On piano, this register produces the sparkling, radiant sonorities found in Liszt's "Transcendental
    /// Études" and Ravel's "Gaspard de la Nuit." The extreme clarity and penetrating power of this register make
    /// it ideal for expressing transcendent moments, brilliant virtuosity, and ecstatic emotion where exceptional
    /// projection and crystalline sonority are essential to the musical expression.
    pub static ref GSHARP5_MAJOR_SCALE: Scale<8> = major_scale(GSHARP5);

    /// A major scale in octave 5 (MIDI notes 81-93)
    ///
    /// Notes: A5, B5, C#6, D6, E6, F#6, G#6, A6
    /// MIDI note numbers: 81, 83, 85, 86, 88, 90, 92, 93
    /// Frequency range: ~880.00 Hz to ~1760.00 Hz
    ///
    /// A major in octave 5 spans an extraordinarily brilliant register at the extreme upper limit of most
    /// instruments and vocal capabilities. This scale occupies the uppermost range of coloratura sopranos,
    /// the highest practical register of violins, and the brilliant register of piccolo. At these frequencies
    /// (approaching 1.8 kHz at the top), tones produce exceptional brilliance with a piercing, crystalline
    /// quality that cuts through any musical texture. A major in this register features in the most demanding
    /// coloratura passages, including the extreme high notes in the cadenzas of Donizetti's and Bellini's
    /// bel canto operas. In orchestral writing, these notes create the brilliant, stratospheric violin passages
    /// in Tchaikovsky's ballets and the climactic moments in Strauss' tone poems. On piano, this register
    /// produces the shimmering, radiant sonorities found in Liszt's "Hungarian Rhapsodies" and Debussy's
    /// "L'Isle Joyeuse." The extreme clarity and penetrating power of this register make it particularly
    /// effective for expressing transcendent moments, brilliant virtuosity, and ecstatic emotion. At these
    /// frequencies, tones begin to assume a quality where brilliance dominates over tonal complexity, approaching
    /// the threshold of pure spectral sensation.
    pub static ref A5_MAJOR_SCALE: Scale<8> = major_scale(A5);

    /// A# major scale in octave 5 (MIDI notes 82-94)
    ///
    /// Notes: A#5, B#5, C##6, D#6, E#6, F##6, G##6, A#6
    /// MIDI note numbers: 82, 84, 86, 87, 89, 91, 93, 94
    /// Frequency range: ~932.33 Hz to ~1864.66 Hz
    ///
    /// A# major in octave 5 (almost always notated as B♭ major) spans an extraordinarily brilliant register
    /// that approaches the extreme upper limit of instrumental and vocal capabilities. This scale occupies
    /// the uppermost range of coloratura sopranos, the highest register of violins, and the brilliant register
    /// of piccolo. At these frequencies (approaching 1.9 kHz at the top), tones produce exceptional brilliance
    /// with a piercing, crystalline quality that cuts through any orchestral texture. B♭ major in this register
    /// features in the most demanding coloratura passages, including the stratospheric high notes in the
    /// virtuosic arias of Strauss' "Der Rosenkavalier" and "Ariadne auf Naxos." In orchestral writing, these
    /// notes create the brilliant, soaring violin passages in Tchaikovsky's symphonies and the shimmering
    /// effects in Ravel's orchestral works. On piano, this register produces the sparkling, radiant sonorities
    /// found in Chopin's "Heroic" Polonaise and Debussy's "Feux d'artifice." The extreme clarity and penetrating
    /// power of this register make it ideal for expressing transcendent moments, brilliant virtuosity, and
    /// ecstatic emotion where exceptional projection and crystalline sonority are essential to the musical
    /// narrative.
    pub static ref ASHARP5_MAJOR_SCALE: Scale<8> = major_scale(ASHARP5);

    /// B major scale in octave 5 (MIDI notes 83-95)
    ///
    /// Notes: B5, C#6, D#6, E6, F#6, G#6, A#6, B6
    /// MIDI note numbers: 83, 85, 87, 88, 90, 92, 94, 95
    /// Frequency range: ~987.77 Hz to ~1975.53 Hz
    ///
    /// B major in octave 5 spans an extraordinarily brilliant register at the extreme upper limit of instrumental
    /// and vocal capabilities. This scale occupies the uppermost range of coloratura sopranos, the highest
    /// practical register of violins, and the brilliant register of piccolo. At these frequencies (approaching
    /// 2 kHz at the top), tones produce exceptional brilliance with a piercing, crystalline quality that cuts
    /// through any orchestral texture. B major in this register appears in the most demanding coloratura
    /// passages, including the stratospheric high notes in Strauss' "Zerbinetta's Aria" and the virtuosic
    /// cadenzas in Mozart's concert arias. In orchestral writing, these notes create the brilliant, soaring
    /// violin passages in Mahler's symphonies and the shimmering effects in Debussy's "La Mer." On piano,
    /// this register produces the radiant, sparkling sonorities found in Liszt's "Transcendental Études" and
    /// Ravel's "Gaspard de la Nuit." The extreme clarity and penetrating power of this register make it
    /// particularly effective for expressing transcendent moments, brilliant virtuosity, and ecstatic emotion.
    /// The nearly 2 kHz upper range approaches the brilliance threshold where tonal complexity gives way to
    /// pure spectral sensation.
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
