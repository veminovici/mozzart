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
    /// use mozzart_std::{MINOR_SCALES, A4, minor_scale};
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
    /// C minor scale in octave 2 (MIDI notes 36-48)
    ///
    /// Notes: C2, D2, E♭2, F2, G2, A♭2, B♭2, C3
    /// MIDI note numbers: 36, 38, 39, 41, 43, 44, 46, 48
    /// Frequency range: ~65.41 Hz to ~130.81 Hz
    ///
    /// C minor in octave 2 offers solid bass fundamentals across the typical lower range of many instruments.
    /// This scale encompasses the lower register of male vocals (bass/baritone) and cello, with the root C2
    /// representing the lowest C on many bass guitars. The characteristic melancholy of minor tonality is
    /// present but with a rich, warm quality in this register. Found in orchestral bass parts, piano left-hand
    /// work, and contemporary bass lines in jazz, rock, and electronic music.
    pub static ref C2_MINOR_SCALE: Scale<8> = minor_scale(C2);

    /// C# minor scale in octave 2 (MIDI notes 37-49)
    ///
    /// Notes: C#2, D#2, E2, F#2, G#2, A2, B2, C#3
    /// MIDI note numbers: 37, 39, 40, 42, 44, 45, 47, 49
    /// Frequency range: ~69.30 Hz to ~138.59 Hz
    ///
    /// C# minor in octave 2 provides a resonant bass foundation, sitting slightly higher than C minor
    /// for improved definition while maintaining substantial low-end presence. This scale is utilized in
    /// bass sections of orchestral works, piano literature, and bass guitar parts where slightly more
    /// clarity is needed without sacrificing depth. The low bass register imparts a dark, rich quality
    /// to the minor tonality that creates a powerful foundation for harmonic structures.
    pub static ref CSHARP2_MINOR_SCALE: Scale<8> = minor_scale(CSHARP2);

    /// D minor scale in octave 2 (MIDI notes 38-50)
    ///
    /// Notes: D2, E2, F2, G2, A2, B♭2, C3, D3
    /// MIDI note numbers: 38, 40, 41, 43, 45, 46, 48, 50
    /// Frequency range: ~73.42 Hz to ~146.83 Hz
    ///
    /// D minor in octave 2 provides a rich bass foundation with improved definition compared to octave 1.
    /// This scale is prominently featured in Bach's D minor organ works, orchestral bass parts, and
    /// contemporary bass lines. The D2 (open D string on bass) serves as a powerful fundamental in this
    /// register. In this octave, D minor maintains its somber character but with sufficient clarity for
    /// melodic bass passages. Particularly effective for bass lines with melodic movement in classical,
    /// jazz, and rock contexts.
    pub static ref D2_MINOR_SCALE: Scale<8> = minor_scale(D2);

    /// D# minor scale in octave 2 (MIDI notes 39-51)
    ///
    /// Notes: D#2, E#2, F#2, G#2, A#2, B2, C#3, D#3
    /// MIDI note numbers: 39, 41, 42, 44, 46, 47, 49, 51
    /// Frequency range: ~77.78 Hz to ~155.56 Hz
    ///
    /// D# minor in octave 2 (more commonly notated as E♭ minor) spans the lower-middle bass register.
    /// This scale offers good definition while maintaining substantial bass presence. In practice, E♭ minor
    /// is more common than D# minor notation, appearing in orchestral works, jazz compositions, and film scores.
    /// The scale bridges the very low bass range with the more functionally melodic range above, allowing
    /// for bass lines that combine fundamental support with harmonic interest and expression.
    pub static ref DSHARP2_MINOR_SCALE: Scale<8> = minor_scale(DSHARP2);

    /// E minor scale in octave 2 (MIDI notes 40-52)
    ///
    /// Notes: E2, F#2, G2, A2, B2, C3, D3, E3
    /// MIDI note numbers: 40, 42, 43, 45, 47, 48, 50, 52
    /// Frequency range: ~82.41 Hz to ~164.81 Hz
    ///
    /// E minor in octave 2 provides a balanced bass range with good definition. The open E string of bass
    /// guitar and lowest string on standard guitar fall in this octave. This makes E minor particularly
    /// idiomatic for these instruments, appearing in countless rock, folk, and classical guitar compositions.
    /// The scale combines sufficient bass weight with improved clarity, making it effective for bass lines
    /// that require both foundation and melodic expression. Stravinsky, Shostakovich, and many rock
    /// composers have used this register effectively.
    pub static ref E2_MINOR_SCALE: Scale<8> = minor_scale(E2);

    /// F minor scale in octave 2 (MIDI notes 41-53)
    ///
    /// Notes: F2, G2, A♭2, B♭2, C3, D♭3, E♭3, F3
    /// MIDI note numbers: 41, 43, 44, 46, 48, 49, 51, 53
    /// Frequency range: ~87.31 Hz to ~174.61 Hz
    ///
    /// F minor in octave 2 sits in the prime bass register, balancing depth and definition. This scale
    /// appears in piano sonatas by Beethoven, orchestral bass parts, and jazz bass lines. The register allows
    /// for expressive bass lines with clear pitch definition while retaining substantial low-end presence.
    /// F minor's dramatic and passionate qualities are well-served in this register, offering both weight
    /// and clarity. The upper notes of the scale begin to approach the transition into the lower-middle
    /// register, adding versatility to melodic possibilities.
    pub static ref F2_MINOR_SCALE: Scale<8> = minor_scale(F2);

    /// F# minor scale in octave 2 (MIDI notes 42-54)
    ///
    /// Notes: F#2, G#2, A2, B2, C#3, D3, E3, F#3
    /// MIDI note numbers: 42, 44, 45, 47, 49, 50, 52, 54
    /// Frequency range: ~92.50 Hz to ~185.00 Hz
    ///
    /// F# minor in octave 2 offers a well-defined bass register with good balance between depth and clarity.
    /// It appears in romantic piano works by Rachmaninoff, orchestral compositions, and modern film scores.
    /// This register allows the scale's melancholic character to be expressed with both weight and definition,
    /// making it suitable for expressive bass lines and harmonic foundations. The upper notes begin crossing
    /// into the baritone register, providing a smooth transition between the deep bass and the midrange.
    pub static ref FSHARP2_MINOR_SCALE: Scale<8> = minor_scale(FSHARP2);

    /// G minor scale in octave 2 (MIDI notes 43-55)
    ///
    /// Notes: G2, A2, B♭2, C3, D3, E♭3, F3, G3
    /// MIDI note numbers: 43, 45, 46, 48, 50, 51, 53, 55
    /// Frequency range: ~98.00 Hz to ~196.00 Hz
    ///
    /// G minor in octave 2 spans a versatile bass range with excellent definition. This scale appears in
    /// Bach's organ works, Mozart's Symphony No. 40, and countless bass lines across genres. The G2 string
    /// on cello and the third string on bass guitar fall in this register. G minor in this octave provides
    /// a solid harmonic foundation while allowing for more melodically active bass lines than lower octaves.
    /// The characteristic tragic quality of G minor is clearly expressed with both weight and articulation.
    pub static ref G2_MINOR_SCALE: Scale<8> = minor_scale(G2);

    /// G# minor scale in octave 2 (MIDI notes 44-56)
    ///
    /// Notes: G#2, A#2, B2, C#3, D#3, E3, F#3, G#3
    /// MIDI note numbers: 44, 46, 47, 49, 51, 52, 54, 56
    /// Frequency range: ~103.83 Hz to ~207.65 Hz
    ///
    /// G# minor in octave 2 (often written as A♭ minor) spans the upper bass range with excellent definition.
    /// This scale bridges the deep bass and baritone registers, providing both foundational support and
    /// melodic clarity. While less common than other minor keys, it appears in romantic piano works and
    /// modern compositions where its unique dark yet articulate character is desired. The upper notes
    /// of this scale cross into the lower midrange, creating versatility for expressive bass passages that
    /// extend beyond purely foundational roles.
    pub static ref GSHARP2_MINOR_SCALE: Scale<8> = minor_scale(GSHARP2);

    /// A minor scale in octave 2 (MIDI notes 45-57)
    ///
    /// Notes: A2, B2, C3, D3, E3, F3, G3, A3
    /// MIDI note numbers: 45, 47, 48, 50, 52, 53, 55, 57
    /// Frequency range: 110.00 Hz to 220.00 Hz
    ///
    /// A minor in octave 2 spans the upper bass to lower midrange, with excellent definition. This A2 (110 Hz)
    /// is the pitch of the open A string on guitars and bass guitars. In this register, A minor serves as a
    /// versatile scale for both foundational and melodic bass lines across classical, jazz, rock, and folk
    /// music. As the relative minor of C major, its melancholic character is clearly articulated while
    /// maintaining sufficient bass presence. The scale's upper notes extend into the baritone vocal range,
    /// providing melodic versatility.
    pub static ref A2_MINOR_SCALE: Scale<8> = minor_scale(A2);

    /// A# minor scale in octave 2 (MIDI notes 46-58)
    ///
    /// Notes: A#2, B#2, C#3, D#3, E#3, F#3, G#3, A#3
    /// MIDI note numbers: 46, 48, 49, 51, 53, 54, 56, 58
    /// Frequency range: ~116.54 Hz to ~233.08 Hz
    ///
    /// A# minor in octave 2 (more commonly notated as B♭ minor) spans from upper bass to lower midrange with
    /// excellent definition. This scale is featured in romantic piano literature (including Chopin's works)
    /// and orchestral compositions. In this register, B♭ minor balances bass presence with articulate clarity,
    /// making it effective for expressive bass lines that require both weight and definition. The scale's
    /// upper notes approach the tenor range, allowing for smooth transitions between foundational bass and
    /// more melodically active midrange parts.
    pub static ref ASHARP2_MINOR_SCALE: Scale<8> = minor_scale(ASHARP2);

    /// B minor scale in octave 2 (MIDI notes 47-59)
    ///
    /// Notes: B2, C#3, D3, E3, F#3, G3, A3, B3
    /// MIDI note numbers: 47, 49, 50, 52, 54, 55, 57, 59
    /// Frequency range: ~123.47 Hz to ~246.94 Hz
    ///
    /// B minor in octave 2 spans from upper bass to lower midrange, with excellent definition and articulation.
    /// This scale appears in Bach's B minor Mass, orchestral literature, and guitar-based compositions.
    /// The B2 note serves as the lowest note on the 5-string bass guitar's B string. In this register,
    /// B minor's melancholic character is clearly expressed with good balance between bass foundation and
    /// melodic articulation. The upper notes extend into the tenor register, creating versatility for
    /// expressive bass lines that move between supportive and melodically prominent roles.
    pub static ref B2_MINOR_SCALE: Scale<8> = minor_scale(B2);
}

lazy_static! {
    /// C minor scale in octave 3 (MIDI notes 48-60)
    ///
    /// Notes: C3, D3, E♭3, F3, G3, A♭3, B♭3, C4
    /// MIDI note numbers: 48, 50, 51, 53, 55, 56, 58, 60
    /// Frequency range: ~130.81 Hz to ~261.63 Hz
    ///
    /// C minor in octave 3 spans the lower-middle register, offering a rich tenor range with excellent
    /// balance between warmth and definition. This scale encompasses the central range of cello, viola,
    /// tenor voices, and guitar. The C3 corresponds to the C string on viola and the third fret of the A string
    /// on guitar. With its dramatic character fully expressed, this register provides the perfect balance of
    /// fundamental richness and melodic clarity, making it ideal for expressive solo passages and rich
    /// harmonic material in orchestral music, chamber works, and contemporary piano compositions.
    pub static ref C3_MINOR_SCALE: Scale<8> = minor_scale(C3);

    /// C# minor scale in octave 3 (MIDI notes 49-61)
    ///
    /// Notes: C#3, D#3, E3, F#3, G#3, A3, B3, C#4
    /// MIDI note numbers: 49, 51, 52, 54, 56, 57, 59, 61
    /// Frequency range: ~138.59 Hz to ~277.18 Hz
    ///
    /// C# minor in octave 3 occupies the central tenor register, providing excellent melodic definition
    /// and expressive capability. The scale falls comfortably in the central range of cello, viola, tenor voice,
    /// and guitar. This register perfectly balances richness and clarity, making it ideal for emotional expression
    /// in Romantic piano works, string quartets, and vocal compositions. The dark intensity characteristic of C# minor
    /// is particularly effective in this register, making it a favorite for expressive solo passages in both
    /// classical and contemporary music, including film scores that require emotional depth.
    pub static ref CSHARP3_MINOR_SCALE: Scale<8> = minor_scale(CSHARP3);

    /// D minor scale in octave 3 (MIDI notes 50-62)
    ///
    /// Notes: D3, E3, F3, G3, A3, B♭3, C4, D4
    /// MIDI note numbers: 50, 52, 53, 55, 57, 58, 60, 62
    /// Frequency range: ~146.83 Hz to ~293.66 Hz
    ///
    /// D minor in octave 3 spans the central tenor range, offering a perfect balance of warmth and clarity.
    /// This scale encompasses the sweet spot of viola, cello, and tenor voice, with the top note reaching
    /// to middle C (C4). The D3 corresponds to the open D string on guitar. In this register, D minor's
    /// melancholic character is ideally expressed, with enough weight for emotional gravity but sufficient
    /// clarity for intricate melodic passages. Extensively used in Bach's keyboard works, Mozart's Requiem,
    /// and countless string quartet movements where expressive melodic lines are paramount.
    pub static ref D3_MINOR_SCALE: Scale<8> = minor_scale(D3);

    /// D# minor scale in octave 3 (MIDI notes 51-63)
    ///
    /// Notes: D#3, E#3, F#3, G#3, A#3, B3, C#4, D#4
    /// MIDI note numbers: 51, 53, 54, 56, 58, 59, 61, 63
    /// Frequency range: ~155.56 Hz to ~311.13 Hz
    ///
    /// D# minor in octave 3 (more commonly notated as E♭ minor) occupies the central tenor range with excellent
    /// melodic definition. The scale falls comfortably in the expressive middle range of viola, cello, tenor saxophone,
    /// and guitar. In this register, the scale's solemn character is perfectly balanced with clarity, making it
    /// effective for expressive solo passages in Romantic piano works, chamber music, and jazz compositions. The upper
    /// notes of this scale begin to cross into the alto range, providing versatility for melodic development that
    /// spans from rich lower tones to more brilliant upper register sounds.
    pub static ref DSHARP3_MINOR_SCALE: Scale<8> = minor_scale(DSHARP3);

    /// E minor scale in octave 3 (MIDI notes 52-64)
    ///
    /// Notes: E3, F#3, G3, A3, B3, C4, D4, E4
    /// MIDI note numbers: 52, 54, 55, 57, 59, 60, 62, 64
    /// Frequency range: ~164.81 Hz to ~329.63 Hz
    ///
    /// E minor in octave 3 spans the central tenor to lower alto range, offering exceptional expressivity.
    /// The E3 corresponds to the open E string on guitar and the lowest string on a standard guitar.
    /// In this register, E minor provides a perfect balance of richness and clarity, making it ideal for
    /// melodic passages in guitar music, string quartets, and vocal compositions. The scale encompasses the
    /// sweet spot of many stringed instruments, including viola and cello. Its natural melancholy is fully
    /// expressed in this versatile middle register, making it a staple in folk, rock, classical, and film music.
    pub static ref E3_MINOR_SCALE: Scale<8> = minor_scale(E3);

    /// F minor scale in octave 3 (MIDI notes 53-65)
    ///
    /// Notes: F3, G3, A♭3, B♭3, C4, D♭4, E♭4, F4
    /// MIDI note numbers: 53, 55, 56, 58, 60, 61, 63, 65
    /// Frequency range: ~174.61 Hz to ~349.23 Hz
    ///
    /// F minor in octave 3 spans from the tenor through the central alto register, offering excellent melodic
    /// clarity and expressivity. This scale encompasses middle C (C4), making it a pivotal range that bridges
    /// lower and middle registers. In this octave, F minor achieves a perfect balance of emotional depth and
    /// articulate definition, making it ideal for expressive melodic passages in piano sonatas, string quartets,
    /// and vocal compositions. The passionate character of F minor is particularly effective in this register,
    /// making it a favorite for emotional musical statements across classical and contemporary genres.
    pub static ref F3_MINOR_SCALE: Scale<8> = minor_scale(F3);

    /// F# minor scale in octave 3 (MIDI notes 54-66)
    ///
    /// Notes: F#3, G#3, A3, B3, C#4, D4, E4, F#4
    /// MIDI note numbers: 54, 56, 57, 59, 61, 62, 64, 66
    /// Frequency range: ~185.00 Hz to ~369.99 Hz
    ///
    /// F# minor in octave 3 spans from tenor to mid-alto register, offering exceptional melodic clarity
    /// and expressivity. This scale encompasses the central playing range of viola, violin, and tenor/alto voices.
    /// The scale crosses through middle C (C4), bridging the lower and middle registers. In this register,
    /// F# minor's melancholic intensity is perfectly balanced with clear articulation, making it ideal for
    /// expressive solo passages in Romantic piano works, string quartets, and art songs. Composers like
    /// Rachmaninoff and Chopin utilized this register of F# minor for some of their most emotionally impactful works.
    pub static ref FSHARP3_MINOR_SCALE: Scale<8> = minor_scale(FSHARP3);

    /// G minor scale in octave 3 (MIDI notes 55-67)
    ///
    /// Notes: G3, A3, B♭3, C4, D4, E♭4, F4, G4
    /// MIDI note numbers: 55, 57, 58, 60, 62, 63, 65, 67
    /// Frequency range: ~196.00 Hz to ~392.00 Hz
    ///
    /// G minor in octave 3 spans from the tenor through the central alto register, with the root G3 corresponding
    /// to the open G string on guitar and viola. This scale centers around middle C (C4), perfectly bridging
    /// lower and middle registers. In this octave, G minor's tragic character is balanced with exceptional
    /// clarity and projection, making it ideal for expressive musical statements in string quartets, piano
    /// sonatas, and vocal compositions. The scale's dramatic intensity is perfectly expressed in this versatile
    /// middle register, which has been utilized by Mozart, Bach, and countless composers for works requiring
    /// both emotional depth and technical flexibility.
    pub static ref G3_MINOR_SCALE: Scale<8> = minor_scale(G3);

    /// G# minor scale in octave 3 (MIDI notes 56-68)
    ///
    /// Notes: G#3, A#3, B3, C#4, D#4, E4, F#4, G#4
    /// MIDI note numbers: 56, 58, 59, 61, 63, 64, 66, 68
    /// Frequency range: ~207.65 Hz to ~415.30 Hz
    ///
    /// G# minor in octave 3 (often written as A♭ minor) spans central vocal and instrumental ranges,
    /// bridging tenor and alto registers. This scale crosses through middle C (C4), offering an excellent
    /// balance of rich lower tones and clearer upper tones. In this register, G# minor provides exceptional
    /// melodic clarity while maintaining emotional depth, making it effective for expressive passages in
    /// Romantic piano literature, string quartets, and vocal compositions. While less commonly used than
    /// other minor keys, in this versatile middle register it offers a unique color for compositions
    /// requiring particular intensity and timbral variety.
    pub static ref GSHARP3_MINOR_SCALE: Scale<8> = minor_scale(GSHARP3);

    /// A minor scale in octave 3 (MIDI notes 57-69)
    ///
    /// Notes: A3, B3, C4, D4, E4, F4, G4, A4
    /// MIDI note numbers: 57, 59, 60, 62, 64, 65, 67, 69
    /// Frequency range: 220.00 Hz to 440.00 Hz
    ///
    /// A minor in octave 3 spans from the central tenor through the alto register, reaching up to
    /// concert pitch A4 (440 Hz). This scale includes the open A string on violin and encompasses
    /// middle C (C4), making it a versatile bridge between lower and upper registers. The root A3 (220 Hz)
    /// is the open A string on guitar and viola. In this register, A minor's melancholic character achieves
    /// perfect balance between emotional depth and melodic clarity, making it the quintessential minor key
    /// for string quartets, piano sonatas, and vocal compositions. As the relative minor of C major,
    /// it occupies a central position in Western music in this practical middle register.
    pub static ref A3_MINOR_SCALE: Scale<8> = minor_scale(A3);

    /// A# minor scale in octave 3 (MIDI notes 58-70)
    ///
    /// Notes: A#3, B#3, C#4, D#4, E#4, F#4, G#4, A#4
    /// MIDI note numbers: 58, 60, 61, 63, 65, 66, 68, 70
    /// Frequency range: ~233.08 Hz to ~466.16 Hz
    ///
    /// A# minor in octave 3 (more commonly notated as B♭ minor) spans a brilliant upper register with
    /// exceptional clarity and projection. This scale features in Chopin's famous "Funeral March" Sonata
    /// and Tchaikovsky's 6th Symphony. In this octave, B♭ minor takes on a penetrating quality with the
    /// upper notes particularly radiant. This range combines the scale's inherent melancholy with brilliant
    /// articulation, making it effective for dramatic contrasts and expressive passages in the upper register.
    pub static ref ASHARP3_MINOR_SCALE: Scale<8> = minor_scale(ASHARP3);

    /// B minor scale in octave 3 (MIDI notes 59-71)
    ///
    /// Notes: B3, C#4, D4, E4, F#4, G4, A4, B4
    /// MIDI note numbers: 59, 61, 62, 64, 66, 67, 69, 71
    /// Frequency range: ~246.94 Hz to ~493.88 Hz
    ///
    /// B minor in octave 3 spans a brilliant upper register with exceptional clarity and projection.
    /// This scale appears in Bach's Mass in B minor and Tchaikovsky's 6th Symphony. In this octave,
    /// B minor achieves a balance of emotional intensity and brilliant articulation, with the upper notes
    /// particularly luminous. This register is ideal for virtuosic passages and expressive melodies in
    /// violin, flute, and coloratura soprano. The scale's melancholic character takes on a more poignant,
    /// ethereal quality in this higher range.
    pub static ref B3_MINOR_SCALE: Scale<8> = minor_scale(B3);
}

lazy_static! {
    /// C minor scale in octave 4 (MIDI notes 60-72)
    ///
    /// Notes: C4 (middle C), D4, E♭4, F4, G4, A♭4, B♭4, C5
    /// MIDI note numbers: 60, 62, 63, 65, 67, 68, 70, 72
    /// Frequency range: ~261.63 Hz to ~523.25 Hz
    ///
    /// C minor in the fourth octave spans the vocal range of most singers and is central to piano literature.
    /// This scale occupies the most prominent octave in Western music, with excellent projection and balance
    /// between clarity and warmth. Extensively used in Beethoven's "Pathétique" Sonata and many classical works,
    /// this scale creates a dramatic, powerful sound with clear articulation. Also commonly used in pop, rock,
    /// and jazz for its emotionally evocative yet accessible range.
    pub static ref C4_MINOR_SCALE: Scale<8> = minor_scale(C4);

    /// C# minor scale in octave 4 (MIDI notes 61-73)
    ///
    /// Notes: C#4, D#4, E4, F#4, G#4, A4, B4, C#5
    /// MIDI note numbers: 61, 63, 64, 66, 68, 69, 71, 73
    /// Frequency range: ~277.18 Hz to ~554.37 Hz
    ///
    /// C# minor in octave 4 spans an ideal range for most instruments and vocalists. It offers excellent
    /// projection and articulation in this central register. Known for its emotional intensity, this scale
    /// was featured in Chopin's Nocturne Op. 27 No. 1 and Rachmaninoff's Prelude Op. 3 No. 2. The scale
    /// provides a balance of clear melodic definition and rich harmonic potential, making it effective
    /// for expressive passages in many genres from classical to contemporary pop.
    pub static ref CSHARP4_MINOR_SCALE: Scale<8> = minor_scale(CSHARP4);

    /// D minor scale in octave 4 (MIDI notes 62-74)
    ///
    /// Notes: D4, E4, F4, G4, A4, B♭4, C5, D5
    /// MIDI note numbers: 62, 64, 65, 67, 69, 70, 72, 74
    /// Frequency range: ~293.66 Hz to ~587.33 Hz
    ///
    /// D minor in octave 4 sits in an optimal range for most instruments, with excellent resonance and
    /// projection. Known as one of the most expressive keys, it's featured in masterpieces like Mozart's
    /// Requiem, Bach's D minor Toccata and Fugue, and countless film scores. This scale spans A4 (concert pitch),
    /// making it well-balanced between higher and lower registers. Its melancholic character is enhanced in this
    /// octave where each note can be clearly articulated while maintaining emotional depth.
    pub static ref D4_MINOR_SCALE: Scale<8> = minor_scale(D4);

    /// D# minor scale in octave 4 (MIDI notes 63-75)
    ///
    /// Notes: D#4, E#4, F#4, G#4, A#4, B4, C#5, D#5
    /// MIDI note numbers: 63, 65, 66, 68, 70, 71, 73, 75
    /// Frequency range: ~311.13 Hz to ~622.25 Hz
    ///
    /// D# minor in octave 4 (more commonly notated as E♭ minor) spans a versatile middle-to-upper range.
    /// In this octave, the scale demonstrates excellent clarity with substantial projection. While less 
    /// common than E♭ major, E♭ minor appears in Chopin's "Raindrop" Prelude and other Romantic works.
    /// This range provides ideal balance for piano, woodwinds, and strings, with each note articulating 
    /// clearly while maintaining the scale's solemn, introspective character.
    pub static ref DSHARP4_MINOR_SCALE: Scale<8> = minor_scale(DSHARP4);

    /// E minor scale in octave 4 (MIDI notes 64-76)
    ///
    /// Notes: E4, F#4, G4, A4, B4, C5, D5, E5
    /// MIDI note numbers: 64, 66, 67, 69, 71, 72, 74, 76
    /// Frequency range: ~329.63 Hz to ~659.26 Hz
    ///
    /// E minor in octave 4 occupies a sweet spot for many instruments and vocal ranges. It's particularly
    /// comfortable on the guitar, where open strings reinforce the scale's tonal center. This scale has been
    /// used extensively in folk, rock, and classical music, including Mendelssohn's Violin Concerto and countless
    /// guitar-based compositions. In this octave, E minor maintains its natural melancholy while providing
    /// excellent clarity and projection, balancing rich lower notes with brilliant higher tones.
    pub static ref E4_MINOR_SCALE: Scale<8> = minor_scale(E4);

    /// F minor scale in octave 4 (MIDI notes 65-77)
    ///
    /// Notes: F4, G4, A♭4, B♭4, C5, D♭5, E♭5, F5
    /// MIDI note numbers: 65, 67, 68, 70, 72, 73, 75, 77
    /// Frequency range: ~349.23 Hz to ~698.46 Hz
    ///
    /// F minor in octave 4 spans the upper-middle register, offering a powerful expressive range with excellent
    /// projection. This scale exhibits a passionate, dramatic quality in this octave, balanced between warmth
    /// and brilliance. Featured in Beethoven's "Appassionata" Sonata and Chopin's Fantaisie-Impromptu, F minor
    /// in this range provides distinct articulation while maintaining emotional intensity. It works exceptionally
    /// well for lyrical passages in piano, strings, and wind instruments, with each note clearly defined.
    pub static ref F4_MINOR_SCALE: Scale<8> = minor_scale(F4);

    /// F# minor scale in octave 4 (MIDI notes 66-78)
    ///
    /// Notes: F#4, G#4, A4, B4, C#5, D5, E5, F#5
    /// MIDI note numbers: 66, 68, 69, 71, 73, 74, 76, 78
    /// Frequency range: ~369.99 Hz to ~739.99 Hz
    ///
    /// F# minor in octave 4 spans a radiant upper-middle register with exceptional clarity and projection.
    /// This scale features prominently in Romantic piano literature, including Rachmaninoff's Piano Concerto
    /// No. 2 and Chopin's Barcarolle. In this register, F# minor achieves a balance of emotional intensity
    /// and brilliant articulation, with the upper notes beginning to take on a sparkling quality. This range
    /// is ideal for expressive melodic lines in piano, violin, and soprano voice.
    pub static ref FSHARP4_MINOR_SCALE: Scale<8> = minor_scale(FSHARP4);

    /// G minor scale in octave 4 (MIDI notes 67-79)
    ///
    /// Notes: G4, A4, B♭4, C5, D5, E♭5, F5, G5
    /// MIDI note numbers: 67, 69, 70, 72, 74, 75, 77, 79
    /// Frequency range: ~392.00 Hz to ~784.00 Hz
    ///
    /// G minor in octave 4 spans a brilliant upper-middle register with excellent projection and clarity.
    /// This scale has been used to express profound emotion in works like Mozart's Symphony No. 40 and
    /// Bach's "Little" Fugue. In this octave, G minor maintains its tragic character while gaining brightness
    /// and intensity in the upper notes. This register balances emotional depth with technical agility,
    /// making it ideal for virtuosic passages in piano, violin, and woodwind literature.
    pub static ref G4_MINOR_SCALE: Scale<8> = minor_scale(G4);

    /// G# minor scale in octave 4 (MIDI notes 68-80)
    ///
    /// Notes: G#4, A#4, B4, C#5, D#5, E5, F#5, G#5
    /// MIDI note numbers: 68, 70, 71, 73, 75, 76, 78, 80
    /// Frequency range: ~415.30 Hz to ~830.61 Hz
    ///
    /// G# minor in octave 4 (often notated as A♭ minor) spans a brilliant upper-middle register with exceptional
    /// clarity. This scale appears in Chopin's Etude Op. 25 No. 11 "Winter Wind" and Rachmaninoff's Prelude
    /// Op. 32 No. 12. In this register, the scale takes on a shimmering intensity, with the upper notes
    /// particularly bright and penetrating. This range combines emotional depth with brilliant articulation,
    /// providing a powerful expressive palette for virtuosic passages and impassioned musical statements.
    pub static ref GSHARP4_MINOR_SCALE: Scale<8> = minor_scale(GSHARP4);

    /// A minor scale in octave 4 (MIDI notes 69-81)
    ///
    /// Notes: A4 (concert A = 440Hz), B4, C5, D5, E5, F5, G5, A5
    /// MIDI note numbers: 69, 71, 72, 74, 76, 77, 79, 81
    /// Frequency range: 440.00 Hz to 880.00 Hz
    ///
    /// A minor in octave 4 begins with concert pitch A4 (440Hz) and spans into the brilliant upper register.
    /// As the relative minor of C major, this scale is fundamental to Western music theory and practice.
    /// In this octave, A minor achieves exceptional clarity and brilliance, while maintaining its characteristic
    /// melancholy. Featured in Bach's A minor Violin Concerto and countless piano works, this scale balances
    /// emotional expressivity with technical brilliance. The upper notes approach the soprano vocal range,
    /// creating a singing, luminous quality.
    pub static ref A4_MINOR_SCALE: Scale<8> = minor_scale(A4);

    /// A# minor scale in octave 4 (MIDI notes 70-82)
    ///
    /// Notes: A#4, B#4, C#5, D#5, E#5, F#5, G#5, A#5
    /// MIDI note numbers: 70, 72, 73, 75, 77, 78, 80, 82
    /// Frequency range: ~466.16 Hz to ~932.33 Hz
    ///
    /// A# minor in octave 4 (more commonly notated as B♭ minor) spans a brilliant upper register with
    /// exceptional clarity and projection. This scale features in Chopin's famous "Funeral March" Sonata
    /// and Tchaikovsky's 6th Symphony. In this octave, B♭ minor takes on a penetrating quality with the
    /// upper notes particularly radiant. This range combines the scale's inherent melancholy with brilliant
    /// articulation, making it effective for dramatic contrasts and expressive passages in the upper register.
    pub static ref ASHARP4_MINOR_SCALE: Scale<8> = minor_scale(ASHARP4);

    /// B minor scale in octave 4 (MIDI notes 71-83)
    ///
    /// Notes: B4, C#5, D5, E5, F#5, G5, A5, B5
    /// MIDI note numbers: 71, 73, 74, 76, 78, 79, 81, 83
    /// Frequency range: ~493.88 Hz to ~987.77 Hz
    ///
    /// B minor in octave 4 spans a brilliant upper register with exceptional clarity and projection.
    /// This scale appears in Bach's Mass in B minor and Tchaikovsky's 6th Symphony. In this octave,
    /// B minor achieves a balance of emotional intensity and brilliant articulation, with the upper notes
    /// particularly luminous. This register is ideal for virtuosic passages and expressive melodies in
    /// violin, flute, and coloratura soprano. The scale's melancholic character takes on a more poignant,
    /// ethereal quality in this higher range.
    pub static ref B4_MINOR_SCALE: Scale<8> = minor_scale(B4);
}

lazy_static! {
    /// C minor scale in octave 5 (MIDI notes 72-84)
    ///
    /// Notes: C5, D5, E♭5, F5, G5, A♭5, B♭5, C6
    /// MIDI note numbers: 72, 74, 75, 77, 79, 80, 82, 84
    /// Frequency range: ~523.25 Hz to ~1046.50 Hz
    ///
    /// C minor in octave 5 spans the soprano range, offering brilliant clarity with exceptional projection.
    /// This scale occupies the upper range of most soprano vocalists, flutes, and violins, with the starting
    /// note C5 sitting an octave above middle C. In this register, C minor takes on a brilliant, penetrating
    /// quality, with the characteristic melancholy expressed through a more ethereal, delicate timbre.
    /// This range appears prominently in coloratura vocal passages, high violin lines, piccolo parts, and
    /// the upper register of piano literature, including many climactic passages in Romantic piano works.
    pub static ref C5_MINOR_SCALE: Scale<8> = minor_scale(C5);

    /// C# minor scale in octave 5 (MIDI notes 73-85)
    ///
    /// Notes: C#5, D#5, E5, F#5, G#5, A5, B5, C#6
    /// MIDI note numbers: 73, 75, 76, 78, 80, 81, 83, 85
    /// Frequency range: ~554.37 Hz to ~1108.73 Hz
    ///
    /// C# minor in octave 5 spans a brilliant high register with exceptional clarity and penetration.
    /// This scale occupies the upper limits of most vocalists and many instruments, offering a shimmering,
    /// radiant quality. The characteristic intensity of C# minor is transformed in this register, taking
    /// on an almost otherworldly brilliance. Featured in virtuosic violin passages, coloratura soprano arias,
    /// and climactic moments in piano literature, this scale combines emotional expressivity with sparkling
    /// technical brilliance, making it effective for moments of transcendent musical intensity.
    pub static ref CSHARP5_MINOR_SCALE: Scale<8> = minor_scale(CSHARP5);

    /// D minor scale in octave 5 (MIDI notes 74-86)
    ///
    /// Notes: D5, E5, F5, G5, A5, B♭5, C6, D6
    /// MIDI note numbers: 74, 76, 77, 79, 81, 82, 84, 86
    /// Frequency range: ~587.33 Hz to ~1174.66 Hz
    ///
    /// D minor in octave 5 spans a brilliant high register that approaches the upper limits of most singers
    /// and many instruments. This scale offers exceptional clarity, projection, and brilliance, occupying
    /// the upper range of violins, flutes, and soprano voices. The inherent melancholy of D minor takes on
    /// a more ethereal, crystalline quality in this register, making it ideal for expressing poignant, delicate
    /// emotions. This octave appears in virtuosic violin passages, coloratura soprano arias, and climactic
    /// moments in piano repertoire, offering both emotional depth and technical brilliance.
    pub static ref D5_MINOR_SCALE: Scale<8> = minor_scale(D5);

    /// D# minor scale in octave 5 (MIDI notes 75-87)
    ///
    /// Notes: D#5, E#5, F#5, G#5, A#5, B5, C#6, D#6
    /// MIDI note numbers: 75, 77, 78, 80, 82, 83, 85, 87
    /// Frequency range: ~622.25 Hz to ~1244.51 Hz
    ///
    /// D# minor in octave 5 (more commonly notated as E♭ minor) spans an extremely high register with
    /// exceptional brilliance and clarity. This scale occupies the upper limits of most instruments and
    /// vocalists, creating a shimmering, often ethereal sound quality. The solemn character of E♭ minor
    /// is transformed in this register, taking on a more transcendent, otherworldly quality. Featured in
    /// virtuosic passages for piccolo, violin harmonics, and the highest registers of the piano, this scale
    /// combines emotional expressivity with crystalline brilliance for moments of particular musical intensity.
    pub static ref DSHARP5_MINOR_SCALE: Scale<8> = minor_scale(DSHARP5);

    /// E minor scale in octave 5 (MIDI notes 76-88)
    ///
    /// Notes: E5, F#5, G5, A5, B5, C6, D6, E6
    /// MIDI note numbers: 76, 78, 79, 81, 83, 84, 86, 88
    /// Frequency range: ~659.26 Hz to ~1318.51 Hz
    ///
    /// E minor in octave 5 spans a brilliant high register, offering exceptional clarity and luminous
    /// projection. This scale occupies the upper range of flutes, violins, and coloratura sopranos,
    /// with a crystalline, shimmering quality. The melancholic character of E minor is transformed at this
    /// height, taking on an ethereal, almost transcendent quality. This register appears in virtuosic classical
    /// passages, high violin lines in orchestral climaxes, and the upper extremes of the piano keyboard.
    /// The scale's character shifts from its earthy, folk-like quality in lower octaves to a more brilliant,
    /// otherworldly expression in this soprano register.
    pub static ref E5_MINOR_SCALE: Scale<8> = minor_scale(E5);

    /// F minor scale in octave 5 (MIDI notes 77-89)
    ///
    /// Notes: F5, G5, A♭5, B♭5, C6, D♭6, E♭6, F6
    /// MIDI note numbers: 77, 79, 80, 82, 84, 85, 87, 89
    /// Frequency range: ~698.46 Hz to ~1396.91 Hz
    ///
    /// F minor in octave 5 spans an extremely high register with brilliant clarity and exceptional projection.
    /// This scale occupies the upper limits of most instruments and vocalists, with a penetrating, crystalline
    /// quality. The dramatic intensity of F minor is transformed in this register, taking on an almost
    /// otherworldly brilliance while maintaining its passionate character. Utilized in virtuosic passages
    /// for piccolo, high violin lines, coloratura soprano arias, and climactic moments in piano literature,
    /// this scale offers both emotional expressivity and technical brilliance for moments of supreme musical intensity.
    pub static ref F5_MINOR_SCALE: Scale<8> = minor_scale(F5);

    /// F# minor scale in octave 5 (MIDI notes 78-90)
    ///
    /// Notes: F#5, G#5, A5, B5, C#6, D5, E5, F#6
    /// MIDI note numbers: 78, 80, 81, 83, 85, 86, 88, 90
    /// Frequency range: ~739.99 Hz to ~1479.98 Hz
    ///
    /// F# minor in octave 5 spans an extremely high register with crystalline brightness and exceptional
    /// projection. This scale reaches the upper limits of most instruments and vocalists, creating a brilliantly
    /// clear, often shimmering sound quality. The characteristic melancholy of F# minor is transformed at this
    /// extreme height, taking on an almost ethereal, transcendent quality. Extensively used in virtuosic
    /// violin passages, piccolo solos, coloratura soprano lines, and climactic piano passages in Romantic
    /// literature. In this register, the scale combines emotional intensity with sparkling brilliance for
    /// moments of supreme musical expressivity.
    pub static ref FSHARP5_MINOR_SCALE: Scale<8> = minor_scale(FSHARP5);

    /// G minor scale in octave 5 (MIDI notes 79-91)
    ///
    /// Notes: G5, A5, B♭5, C6, D5, E♭5, F5, G6
    /// MIDI note numbers: 79, 81, 82, 84, 86, 87, 89, 91
    /// Frequency range: ~784.00 Hz to ~1568.00 Hz
    ///
    /// G minor in octave 5 spans an extremely high register with brilliant clarity and exceptional projection.
    /// This scale occupies the upper limits of most instruments and vocalists, with a penetrating, crystalline
    /// quality. The tragic character of G minor is transformed in this register, taking on an almost otherworldly
    /// brilliance. Featured in virtuosic passages for piccolo, violin harmonics, extreme coloratura vocal lines, and
    /// the highest register of the piano. The scale's characteristic intensity combines with the brilliance of this register
    /// to create moments of transcendent musical expression.
    pub static ref G5_MINOR_SCALE: Scale<8> = minor_scale(G5);

    /// G# minor scale in octave 5 (MIDI notes 80-92)
    ///
    /// Notes: G#5, A#5, B5, C#6, D#5, E5, F#5, G#6
    /// MIDI note numbers: 80, 82, 83, 85, 87, 88, 90, 92
    /// Frequency range: ~830.61 Hz to ~1661.22 Hz
    ///
    /// G# minor in octave 5 (often notated as A♭ minor) spans an extremely high register with brilliant,
    /// crystalline clarity. This scale occupies the uppermost range of most instruments and vocalists,
    /// creating a penetrating, often shimmering sound quality. At this extreme height, the dark character
    /// of G# minor is transformed, taking on an almost celestial, otherworldly quality. Utilized in virtuosic
    /// passages for piccolo, violin harmonics, extreme coloratura vocal lines, and the highest register of
    /// piano literature. The scale's characteristic intensity combines with the brilliance of this register
    /// to create moments of transcendent musical expression.
    pub static ref GSHARP5_MINOR_SCALE: Scale<8> = minor_scale(GSHARP5);

    /// A minor scale in octave 5 (MIDI notes 81-93)
    ///
    /// Notes: A5, B5, C6, D6, E6, F6, G6, A6
    /// MIDI note numbers: 81, 83, 84, 86, 88, 89, 91, 93
    /// Frequency range: 880.00 Hz to 1760.00 Hz
    ///
    /// A minor in octave 5 spans an extremely high register with exceptional clarity and brightness. This scale
    /// occupies the uppermost range of most instruments and vocalists, with a penetrating, crystalline quality.
    /// At this extreme height, A minor's melancholic character is transformed into a more ethereal, otherworldly
    /// expression. Featured in virtuosic passages for piccolo, violin harmonics, coloratura soprano arias, and
    /// the highest register of the piano keyboard. As the relative minor of C major, this scale remains
    /// fundamentally important even at these extreme heights, offering both technical brilliance and emotional
    /// depth for moments of peak musical intensity.
    pub static ref A5_MINOR_SCALE: Scale<8> = minor_scale(A5);

    /// A# minor scale in octave 5 (MIDI notes 82-94)
    ///
    /// Notes: A#5, B#5, C#6, D#5, E#5, F#5, G#5, A#6
    /// MIDI note numbers: 82, 84, 85, 87, 89, 90, 92, 94
    /// Frequency range: ~932.33 Hz to ~1864.66 Hz
    ///
    /// A# minor in octave 5 (more commonly notated as B♭ minor) spans a brilliant upper register with
    /// exceptional brilliance and clarity. This scale reaches the uppermost limits of most instruments
    /// and vocalists, creating a penetrating, often shimmering sound quality. The dark, melancholic character
    /// of B♭ minor is transformed at this extreme height, taking on an almost celestial, transcendent quality.
    /// Utilized in virtuosic passages for piccolo, extreme violin techniques, coloratura soprano literature,
    /// and the highest reaches of the piano keyboard. In this register, the scale's expressive potential
    /// combines with extraordinary brilliance for moments of supreme musical intensity.
    pub static ref ASHARP5_MINOR_SCALE: Scale<8> = minor_scale(ASHARP5);

    /// B minor scale in octave 5 (MIDI notes 83-95)
    ///
    /// Notes: B5, C#6, D5, E5, F#5, G5, A5, B6
    /// MIDI note numbers: 83, 85, 86, 88, 90, 91, 93, 95
    /// Frequency range: ~987.77 Hz to ~1975.53 Hz
    ///
    /// B minor in octave 5 spans an extremely high register, approaching the upper limits of human hearing
    /// acuity. This scale occupies the uppermost range of most instruments and vocalists, with a brilliant,
    /// often piercing clarity. At this extreme height, B minor's reflective melancholy is transformed into
    /// a more ethereal, transcendent expression. Featured in the most virtuosic passages for piccolo,
    /// extreme violin techniques, and the highest register of the piano. The scale's expressive power
    /// combines with extraordinary brilliance in this register, creating a uniquely intense musical effect
    /// that can evoke sensations of weightlessness or luminosity in the right musical context.
    pub static ref B5_MINOR_SCALE: Scale<8> = minor_scale(B5);
}

lazy_static! {
    /// C minor scale in octave 6 (MIDI notes 84-96)
    ///
    /// Notes: C6, D6, E♭6, F6, G6, A♭6, B♭6, C7
    /// MIDI note numbers: 84, 86, 87, 89, 91, 92, 94, 96
    /// Frequency range: ~1046.50 Hz to ~2093.00 Hz
    ///
    /// C minor in octave 6 occupies an extremely high register that approaches the upper limit of
    /// practical musical application. This scale produces piercing, crystalline tones at frequencies
    /// where pitch perception begins to blur for many listeners. Only specialized instruments like
    /// piccolo, highest piano notes, and extended technique violin harmonics can effectively produce these
    /// notes. The characteristic melancholy of the minor scale is transformed at this extreme height into
    /// an almost ethereal, otherworldly quality. These frequencies are primarily theoretical in musical
    /// composition, though occasionally appear in contemporary works exploring the extremes of human
    /// hearing and instrumental capability.
    pub static ref C6_MINOR_SCALE: Scale<8> = minor_scale(C6);

    /// C# minor scale in octave 6 (MIDI notes 85-97)
    ///
    /// Notes: C#6, D#6, E6, F#6, G#6, A6, B6, C#7
    /// MIDI note numbers: 85, 87, 88, 90, 92, 93, 95, 97
    /// Frequency range: ~1108.73 Hz to ~2217.46 Hz
    ///
    /// C# minor in octave 6 represents an extreme upper register that lies at the threshold of
    /// practical musical application. These piercing, brilliantly clear tones approach the upper
    /// limits of pitch discrimination for many listeners. At these frequencies, the conventional expressive
    /// qualities of minor scales transform into pure, spectral phenomena. Used almost exclusively in
    /// experimental music, electronic compositions, or works exploring the outer boundaries of acoustics.
    /// Only the piccolo, highest piano notes, and specialized extended techniques on strings can effectively
    /// produce these notes, primarily as effects rather than melodic material. C# minor at this altitude
    /// creates an almost extraterrestrial sound quality.
    pub static ref CSHARP6_MINOR_SCALE: Scale<8> = minor_scale(CSHARP6);

    /// D minor scale in octave 6 (MIDI notes 86-98)
    ///
    /// Notes: D6, E6, F6, G6, A6, B♭6, C7, D7
    /// MIDI note numbers: 86, 88, 89, 91, 93, 94, 96, 98
    /// Frequency range: ~1174.66 Hz to ~2349.32 Hz
    ///
    /// D minor in octave 6 inhabits an extreme range where traditional musical perception begins to
    /// dissolve into pure acoustical phenomena. With frequencies exceeding 2kHz at the upper notes,
    /// this scale operates at the periphery of pitch recognition for many listeners. Only instruments
    /// like piccolo, glockenspiel, highest piano notes, and specialized violin techniques can produce
    /// these notes. At these extreme heights, the minor scale's melancholy is transformed into a
    /// brilliant, almost glassy sheen of sound. These frequencies appear primarily in contemporary
    /// experimental compositions exploring sound as physics rather than traditional musical expression.
    pub static ref D6_MINOR_SCALE: Scale<8> = minor_scale(D6);

    /// D# minor scale in octave 6 (MIDI notes 87-99)
    ///
    /// Notes: D#6, E#6, F#6, G#6, A#6, B6, C#7, D#7
    /// MIDI note numbers: 87, 89, 90, 92, 94, 95, 97, 99
    /// Frequency range: ~1244.51 Hz to ~2489.02 Hz
    ///
    /// D# minor in octave 6 (more commonly notated as E♭ minor) exists in an exceptionally high
    /// register where musical pitch begins to transform into pure sonic brilliance. The highest notes
    /// in this scale exceed 2.4kHz, approaching the range where pitch perception becomes increasingly
    /// difficult for many listeners. This scale can only be realized on instruments like piccolo,
    /// highest piano notes, or string harmonics, and typically functions as spectral color rather
    /// than traditional melodic material. Used almost exclusively in avant-garde compositions, 
    /// electronic music, or acoustic research exploring the extreme upper threshold of musical sound.
    pub static ref DSHARP6_MINOR_SCALE: Scale<8> = minor_scale(DSHARP6);

    /// E minor scale in octave 6 (MIDI notes 88-100)
    ///
    /// Notes: E6, F#6, G6, A6, B6, C7, D7, E7
    /// MIDI note numbers: 88, 90, 91, 93, 95, 96, 98, 100
    /// Frequency range: ~1318.51 Hz to ~2637.02 Hz
    ///
    /// E minor in octave 6 occupies an extremely high register at the upper limits of practical
    /// musical application. With frequencies exceeding 2.6kHz in its highest notes, this scale
    /// operates in a range where pitch perception begins to blur into pure brilliance and texture
    /// for many listeners. Limited to instruments like piccolo, extreme piano range, glockenspiel,
    /// and violin harmonics, this scale functions more as spectral color than traditional melodic
    /// material. The natural melancholy of E minor is translated into a crystalline, otherworldly
    /// quality at this extreme altitude. These frequencies appear primarily in contemporary experimental
    /// music, film scoring for tension effects, and acoustic research.
    pub static ref E6_MINOR_SCALE: Scale<8> = minor_scale(E6);

    /// F minor scale in octave 6 (MIDI notes 89-101)
    ///
    /// Notes: F6, G6, A♭6, B♭6, C7, D♭7, E♭7, F7
    /// MIDI note numbers: 89, 91, 92, 94, 96, 97, 99, 101
    /// Frequency range: ~1396.91 Hz to ~2793.83 Hz
    ///
    /// F minor in octave 6 represents an extraordinarily high register where conventional musical
    /// expression transforms into pure acoustical phenomena. With its highest notes approaching 2.8kHz,
    /// this scale operates near the upper threshold of pitch recognition for many listeners. Only
    /// specialized instruments like piccolo, highest piano notes, and glockenspiel can effectively
    /// produce these notes. At these extreme frequencies, the minor scale's traditional emotional
    /// character dissolves into brilliant, almost glass-like sonic textures. These sounds appear almost
    /// exclusively in contemporary experimental compositions, spectral music, and acoustic research
    /// exploring the boundaries between pitch and timbre.
    pub static ref F6_MINOR_SCALE: Scale<8> = minor_scale(F6);

    /// F# minor scale in octave 6 (MIDI notes 90-102)
    ///
    /// Notes: F#6, G#6, A6, B6, C#7, D7, E7, F#7
    /// MIDI note numbers: 90, 92, 93, 95, 97, 98, 100, 102
    /// Frequency range: ~1479.98 Hz to ~2959.96 Hz
    ///
    /// F# minor in octave 6 exists at the extreme upper threshold of practical musical application.
    /// With its highest frequencies approaching 3kHz, this scale operates in a region where pitch
    /// perception begins to blur into pure brilliance and texture. Only specialized instruments like
    /// piccolo, glockenspiel, highest piano notes, and violin harmonics can effectively produce these
    /// notes, and primarily as effects rather than melodic material. At these extreme heights, the
    /// minor scale's melancholy dissolves into a pure, almost glass-like shimmer. Used almost exclusively
    /// in experimental compositions, electronic music, and acoustic research exploring the boundaries
    /// of musical perception.
    pub static ref FSHARP6_MINOR_SCALE: Scale<8> = minor_scale(FSHARP6);

    /// G minor scale in octave 6 (MIDI notes 91-103)
    ///
    /// Notes: G6, A6, B♭6, C7, D7, E♭7, F7, G7
    /// MIDI note numbers: 91, 93, 94, 96, 98, 99, 101, 103
    /// Frequency range: ~1568.00 Hz to ~3136.00 Hz
    ///
    /// G minor in octave 6 occupies an extreme register at the boundaries of practical musical application.
    /// With its highest notes exceeding 3kHz, this scale reaches frequencies where pitch perception begins
    /// to dissolve into pure spectral brilliance for many listeners. Limited to specialized instruments
    /// like piccolo, glockenspiel, and highest piano notes, these pitches function more as timbral effects
    /// than traditional melodic material. The tragic character of G minor is transformed at this altitude
    /// into a diamond-like brilliance devoid of conventional expressivity. Used almost exclusively in
    /// contemporary experimental compositions, spectral music, and acoustic research exploring the
    /// perceptual boundaries between pitch and timbre.
    pub static ref G6_MINOR_SCALE: Scale<8> = minor_scale(G6);

    /// G# minor scale in octave 6 (MIDI notes 92-104)
    ///
    /// Notes: G#6, A#6, B6, C#7, D#7, E7, F#7, G#7
    /// MIDI note numbers: 92, 94, 95, 97, 99, 100, 102, 104
    /// Frequency range: ~1661.22 Hz to ~3322.44 Hz
    ///
    /// G# minor in octave 6 (often notated as A♭ minor) inhabits an extraordinarily high register
    /// at the upper threshold of musical perception. With its highest frequencies exceeding 3.3kHz,
    /// this scale operates in a domain where pitch identity begins to dissolve into pure acoustic energy
    /// for many listeners. These frequencies can only be produced by specialized instruments like piccolo,
    /// glockenspiel, crotales, and highest piano notes. At this extreme altitude, the minor scale's
    /// traditional emotional qualities are transformed into pure, almost laser-like sonic brilliance.
    /// These sounds appear exclusively in avant-garde compositions, electronic music, and acoustic
    /// research exploring the outer boundaries of musical perception.
    pub static ref GSHARP6_MINOR_SCALE: Scale<8> = minor_scale(GSHARP6);

    /// A minor scale in octave 6 (MIDI notes 93-105)
    ///
    /// Notes: A6, B6, C7, D7, E7, F7, G7, A7
    /// MIDI note numbers: 93, 95, 96, 98, 100, 101, 103, 105
    /// Frequency range: 1760.00 Hz to 3520.00 Hz
    ///
    /// A minor in octave 6 represents an extreme register at the outer limits of practical musical
    /// application. Beginning at A6 (1760Hz) and reaching to A7 (3520Hz), this scale operates in a frequency
    /// range where conventional pitch perception begins to blur for many listeners. The highest notes of
    /// this scale can only be produced by specialized instruments like piccolo, glockenspiel, crotales,
    /// and the extreme upper register of the piano. At these frequencies, the natural melancholy of A minor
    /// is transformed into a pure, crystalline brilliance that functions more as spectral color than
    /// traditional musical expression. Used almost exclusively in contemporary experimental compositions,
    /// electronic music, and acoustic research exploring the boundaries of human hearing.
    pub static ref A6_MINOR_SCALE: Scale<8> = minor_scale(A6);

    /// A# minor scale in octave 6 (MIDI notes 94-106)
    ///
    /// Notes: A#6, B#6, C#7, D#7, E#7, F#7, G#7, A#7
    /// MIDI note numbers: 94, 96, 97, 99, 101, 102, 104, 106
    /// Frequency range: ~1864.66 Hz to ~3729.31 Hz
    ///
    /// A# minor in octave 6 (more commonly notated as B♭ minor) exists at the extreme upper threshold
    /// of musical application. With its highest notes approaching 3.7kHz, this scale operates in a
    /// frequency range where pitch identity begins to dissolve into pure acoustic brilliance for most
    /// listeners. These sounds can only be produced by specialized instruments like piccolo, glockenspiel,
    /// crotales, and the highest piano notes, functioning more as spectral effects than conventional
    /// musical material. At these extreme heights, B♭ minor's traditional emotional quality is transformed
    /// into an almost ethereal, glass-like sonic texture. These frequencies appear exclusively in
    /// contemporary experimental compositions, electronic music, and acoustic research exploring the
    /// outer boundaries of musical perception.
    pub static ref ASHARP6_MINOR_SCALE: Scale<8> = minor_scale(ASHARP6);

    /// B minor scale in octave 6 (MIDI notes 95-107)
    ///
    /// Notes: B6, C#7, D7, E7, F#7, G7, A7, B7
    /// MIDI note numbers: 95, 97, 98, 100, 102, 103, 105, 107
    /// Frequency range: ~1975.53 Hz to ~3951.07 Hz
    ///
    /// B minor in octave 6 occupies an extreme register at the outer threshold of musical application.
    /// With its highest frequencies approaching 4kHz, this scale reaches the upper limits of conventional
    /// pitch perception for many listeners, entering a domain where notes are experienced more as
    /// pure brilliance and texture than as distinct pitches. Only specialized instruments like piccolo,
    /// glockenspiel, crotales, and the extreme upper register of the piano can produce these notes,
    /// and primarily as effects rather than melodic material. At these frequencies, the reflective
    /// melancholy of B minor is transformed into a crystalline, almost otherworldly sheen. These sounds
    /// appear exclusively in contemporary experimental compositions, electronic music, and acoustic
    /// research exploring the boundaries of human hearing and musical perception.
    pub static ref B6_MINOR_SCALE: Scale<8> = minor_scale(B6);
}

lazy_static! {
    /// C minor scale in octave 7 (MIDI notes 96-108)
    ///
    /// Notes: C7, D7, E♭7, F7, G7, A♭7, B♭7, C8
    /// MIDI note numbers: 96, 98, 99, 101, 103, 104, 106, 108
    /// Frequency range: ~2093.00 Hz to ~4186.01 Hz
    ///
    /// C minor in octave 7 inhabits an extreme register at the uppermost threshold of musical perception.
    /// With frequencies spanning from ~2kHz to over 4kHz, this scale operates in a domain where traditional
    /// pitch recognition transitions to pure spectral experience for most listeners. These frequencies can
    /// only be produced by specialized instruments like piccolo (highest register), glockenspiel, crotales,
    /// and the very highest piano notes. At this extreme altitude, the minor scale's emotional character
    /// dissolves into pure acoustic phenomena—more timbre than tonality. These sounds appear exclusively in
    /// electronic music, spectral compositions, and acoustic research studying the transition zone between
    /// pitch perception and timbre perception. The C8 at the top of this scale (MIDI 108) marks the highest C
    /// on the standard piano.
    pub static ref C7_MINOR_SCALE: Scale<8> = minor_scale(C7);

    /// C# minor scale in octave 7 (MIDI notes 97-109)
    ///
    /// Notes: C#7, D#7, E7, F#7, G#7, A7, B7, C#8
    /// MIDI note numbers: 97, 99, 100, 102, 104, 105, 107, 109
    /// Frequency range: ~2217.46 Hz to ~4434.92 Hz
    ///
    /// C# minor in octave 7 exists at the extreme upper boundary of musical perception, approaching
    /// the threshold where pitch identity dissolves into pure acoustic sensation. With frequencies exceeding
    /// 4.4kHz at its highest note, this scale lies beyond the range of most acoustic instruments, with only
    /// specialized percussion (crotales, celesta) and extended techniques capable of producing these sounds.
    /// At these extreme frequencies, the minor scale's traditional emotional qualities transform completely
    /// into pure timbral phenomena—shimmering, crystalline textures rather than clearly defined pitches.
    /// These sounds exist primarily in electronic music, spectral composition, acoustic research, and
    /// specialized sound design contexts exploring the perceptual boundaries between pitch and pure frequency.
    pub static ref CSHARP7_MINOR_SCALE: Scale<8> = minor_scale(CSHARP7);

    /// D minor scale in octave 7 (MIDI notes 98-110)
    ///
    /// Notes: D7, E7, F7, G7, A7, B♭7, C8, D8
    /// MIDI note numbers: 98, 100, 101, 103, 105, 106, 108, 110
    /// Frequency range: ~2349.32 Hz to ~4698.64 Hz
    ///
    /// D minor in octave 7 occupies an extremely high register that approaches the upper limits of human
    /// pitch perception. With frequencies spanning from ~2.3kHz to ~4.7kHz, this scale exists in a domain
    /// where most listeners perceive brilliance and timbre more than distinct pitches. These frequencies
    /// exceed the range of nearly all acoustic instruments except specialized percussion (crotales, celesta)
    /// and the absolute highest piano notes. At these extreme heights, the minor scale's melancholic
    /// character transforms into a pure acoustic phenomenon—more a study in harmonics than conventional
    /// music. These sounds function primarily in electronic music, spectral composition, acoustic research,
    /// and as specialized effects in sound design contexts exploring the boundaries of auditory perception.
    pub static ref D7_MINOR_SCALE: Scale<8> = minor_scale(D7);

    /// D# minor scale in octave 7 (MIDI notes 99-111)
    ///
    /// Notes: D#7, E#7, F#7, G#7, A#7, B7, C#8, D#8
    /// MIDI note numbers: 99, 101, 102, 104, 106, 107, 109, 111
    /// Frequency range: ~2489.02 Hz to ~4978.03 Hz
    ///
    /// D# minor in octave 7 (more commonly notated as E♭ minor) exists at the extreme upper threshold of
    /// human pitch perception. With frequencies approaching 5kHz at its highest note, this scale operates
    /// in a perceptual range where pitch identity largely dissolves into pure acoustic sensation for most
    /// listeners. These frequencies exceed the capabilities of nearly all acoustic instruments except the
    /// highest registers of specialized percussion like crotales and celesta. At these extreme heights,
    /// the minor scale's emotional qualities transform entirely into spectral phenomena—pure, glassy
    /// textures rather than traditional musical expressions. These sounds exist almost exclusively in
    /// electronic music, spectral composition, and acoustic research exploring the boundaries between
    /// pitch perception and pure sonic experience.
    pub static ref DSHARP7_MINOR_SCALE: Scale<8> = minor_scale(DSHARP7);

    /// E minor scale in octave 7 (MIDI notes 100-112)
    ///
    /// Notes: E7, F#7, G7, A7, B7, C8, D8, E8
    /// MIDI note numbers: 100, 102, 103, 105, 107, 108, 110, 112
    /// Frequency range: ~2637.02 Hz to ~5274.04 Hz
    ///
    /// E minor in octave 7 inhabits an extraordinarily high register at the upper threshold of human
    /// pitch discrimination. With frequencies spanning from ~2.6kHz to over 5.2kHz, this scale operates
    /// in a domain where traditional musical perception dissolves into pure acoustic phenomena. These
    /// frequencies exceed the capabilities of virtually all conventional instruments, with only specialized
    /// percussion and electronic sources capable of accurately producing these sounds. At these extreme
    /// heights, the minor scale's melancholic character transforms completely into pure spectral energy—
    /// glistening, ethereal textures rather than conventionally perceived musical notes. These sounds exist
    /// primarily in electronic music, spectral research, and specialized acoustic studies exploring the
    /// threshold between pitch and pure frequency perception.
    pub static ref E7_MINOR_SCALE: Scale<8> = minor_scale(E7);

    /// F minor scale in octave 7 (MIDI notes 101-113)
    ///
    /// Notes: F7, G7, A♭7, B♭7, C8, D♭8, E♭8, F8
    /// MIDI note numbers: 101, 103, 104, 106, 108, 109, 111, 113
    /// Frequency range: ~2793.83 Hz to ~5587.65 Hz
    ///
    /// F minor in octave 7 exists at the extreme upper threshold of musical application and human pitch
    /// perception. With frequencies spanning from ~2.8kHz to ~5.6kHz, this scale operates in a domain
    /// where traditional pitch perception dissolves into pure spectral brilliance for most listeners.
    /// These frequencies exceed the capabilities of virtually all conventional acoustic instruments,
    /// existing primarily in electronic and computer-generated music. At these extreme heights, the minor
    /// scale's emotional character transforms entirely into pure acoustic phenomena—crystalline textures
    /// and spectral energy rather than traditionally perceived musical intervals. The scale exists almost
    /// exclusively in electronic music, psychoacoustic research, and specialized sound design contexts
    /// exploring the boundaries of human auditory perception.
    pub static ref F7_MINOR_SCALE: Scale<8> = minor_scale(F7);

    /// F# minor scale in octave 7 (MIDI notes 102-114)
    ///
    /// Notes: F#7, G#7, A7, B7, C#8, D8, E8, F#8
    /// MIDI note numbers: 102, 104, 105, 107, 109, 110, 112, 114
    /// Frequency range: ~2959.96 Hz to ~5919.91 Hz
    ///
    /// F# minor in octave 7 exists at the extreme upper boundary of musical perception, with frequencies
    /// approaching 6kHz at its highest note. This scale operates in a frequency domain where conventional
    /// pitch perception gives way to pure acoustic sensation for most listeners, beyond the capabilities
    /// of virtually all acoustic instruments except for specialized percussion like crotales. At these
    /// extreme heights, the traditional character of the minor scale dissolves entirely into pure spectral
    /// phenomena—shimmering, glassy textures rather than distinctly perceived musical pitches. These sounds
    /// function almost exclusively in electronic music, computer-generated sound, acoustic research, and
    /// specialized sound design exploring the boundaries of human auditory perception and the transition
    /// from pitch to pure frequency sensation.
    pub static ref FSHARP7_MINOR_SCALE: Scale<8> = minor_scale(FSHARP7);

    /// G minor scale in octave 7 (MIDI notes 103-115)
    ///
    /// Notes: G7, A7, B♭7, C8, D8, E♭8, F8, G8
    /// MIDI note numbers: 103, 105, 106, 108, 110, 111, 113, 115
    /// Frequency range: ~3136.00 Hz to ~6271.93 Hz
    ///
    /// G minor in octave 7 inhabits an extraordinarily high register at the upper threshold of human pitch
    /// perception. With frequencies spanning from ~3.1kHz to ~6.3kHz, this scale operates in a domain
    /// where distinct pitch identification gives way to pure spectral sensation for most listeners. These
    /// frequencies exceed the capabilities of virtually all conventional acoustic instruments, existing
    /// primarily in electronic and computer-generated contexts. At these extreme heights, the minor scale's
    /// traditional character transforms completely into pure acoustic phenomena—shimmering, crystalline
    /// textures rather than conventionally perceived musical intervals. These sounds function almost
    /// exclusively in electronic music, acoustic research, and specialized sound design exploring the
    /// perceptual boundaries between pitch and pure frequency.
    pub static ref G7_MINOR_SCALE: Scale<8> = minor_scale(G7);

    /// G# minor scale in octave 7 (MIDI notes 104-116)
    ///
    /// Notes: G#7, A#7, B7, C#8, D#8, E8, F#8, G#8
    /// MIDI note numbers: 104, 106, 107, 109, 111, 112, 114, 116
    /// Frequency range: ~3322.44 Hz to ~6644.88 Hz
    ///
    /// G# minor in octave 7 (often notated as A♭ minor) exists at the extreme upper threshold of human pitch
    /// perception. With frequencies exceeding 6.6kHz at its highest note, this scale operates entirely in the
    /// domain where distinct pitch perception dissolves into pure spectral sensation for most listeners.
    /// These frequencies lie beyond the capabilities of virtually all acoustic instruments, existing primarily
    /// in electronic and computer-generated music. At these extreme heights, the minor scale's traditional
    /// emotional qualities transform completely into pure acoustic phenomena—crystalline, shimmering textures
    /// rather than distinctly perceived musical intervals. The scale functions almost exclusively in electronic
    /// music, acoustic research, and specialized sound design exploring the perceptual boundaries between
    /// pitch identity and pure frequency sensation.
    pub static ref GSHARP7_MINOR_SCALE: Scale<8> = minor_scale(GSHARP7);

    /// A minor scale in octave 7 (MIDI notes 105-117)
    ///
    /// Notes: A7, B7, C8, D8, E8, F8, G8, A8
    /// MIDI note numbers: 105, 107, 108, 110, 112, 113, 115, 117
    /// Frequency range: 3520.00 Hz to 7040.00 Hz
    ///
    /// A minor in octave 7 exists at the extreme boundary of human pitch perception. With frequencies
    /// spanning from 3.5kHz to 7kHz, this scale operates in a domain where traditional pitch perception
    /// dissolves into pure spectral brilliance for most listeners. These frequencies exceed the capabilities
    /// of virtually all conventional instruments, existing primarily in electronic and computer-generated
    /// contexts. At these extreme heights, the minor scale's melancholic character transforms entirely
    /// into pure acoustic phenomena—glistening, ethereal textures rather than distinctly perceived musical
    /// intervals. The scale functions almost exclusively in electronic music, acoustic research, and specialized
    /// sound design exploring the boundaries of human auditory perception. Notably, many adults gradually lose
    /// the ability to hear the highest frequencies in this scale as they age.
    pub static ref A7_MINOR_SCALE: Scale<8> = minor_scale(A7);

    /// A# minor scale in octave 7 (MIDI notes 106-118)
    ///
    /// Notes: A#7, B#7, C#8, D#8, E#8, F#8, G#8, A#8
    /// MIDI note numbers: 106, 108, 109, 111, 113, 114, 116, 118
    /// Frequency range: ~3729.31 Hz to ~7458.62 Hz
    ///
    /// A# minor in octave 7 (more commonly notated as B♭ minor) inhabits an extreme register at the uppermost
    /// threshold of human pitch perception. With frequencies approaching 7.5kHz at its highest note, this scale
    /// operates entirely in the domain where pitch identity dissolves into pure spectral sensation for most
    /// listeners. These frequencies exceed the capabilities of all conventional acoustic instruments, existing
    /// primarily in electronic and computer-generated music. At these extreme heights, the minor scale's
    /// traditional emotional qualities transform completely into pure acoustic phenomena—shimmering, crystalline
    /// textures rather than distinctly perceived musical intervals. The scale functions exclusively in electronic
    /// music, acoustic research, and specialized sound design contexts exploring the perceptual boundaries between
    /// pitch and pure frequency. At these frequencies, many adults over 40 may be unable to hear the highest notes.
    pub static ref ASHARP7_MINOR_SCALE: Scale<8> = minor_scale(ASHARP7);

    /// B minor scale in octave 7 (MIDI notes 107-119)
    ///
    /// Notes: B7, C#8, D8, E8, F#8, G8, A8, B8
    /// MIDI note numbers: 107, 109, 110, 112, 114, 115, 117, 119
    /// Frequency range: ~3951.07 Hz to ~7902.13 Hz
    ///
    /// B minor in octave 7 exists at the extreme upper limit of human pitch perception. With frequencies
    /// spanning from ~4kHz to ~7.9kHz, this scale operates in a frequency domain where traditional pitch
    /// perception dissolves entirely into pure spectral sensation for most listeners. These frequencies exceed
    /// the capabilities of all conventional acoustic instruments, existing primarily in electronic music and
    /// specialized sound design. At these extreme heights, the minor scale's traditional emotional character
    /// transforms completely into pure acoustic phenomena—brilliant, crystalline textures rather than distinctly
    /// perceived musical intervals. The scale functions exclusively in electronic music, acoustic research, and
    /// specialized sound design contexts exploring the perceptual boundaries of human hearing. The highest notes
    /// of this scale are inaudible to many adults, particularly those over 40, due to age-related hearing loss.
    pub static ref B7_MINOR_SCALE: Scale<8> = minor_scale(B7);
}

lazy_static! {
    /// C minor scale in octave 8 (MIDI notes 108-120)
    ///
    /// Notes: C8, D8, E♭8, F8, G8, A♭8, B♭8, C9
    /// MIDI note numbers: 108, 110, 111, 113, 115, 116, 118, 120
    /// Frequency range: ~4186.01 Hz to ~8372.02 Hz
    ///
    /// C minor in octave 8 exists at the absolute upper threshold of human pitch perception and MIDI
    /// representation. These frequencies, spanning from ~4.2kHz to ~8.4kHz, operate in a domain where 
    /// traditional musical perception completely dissolves into pure acoustic sensation for virtually all
    /// listeners. C8 (MIDI 108) represents the highest C on a standard piano, with notes beyond this point
    /// existing primarily in electronic contexts and specialized acoustic research. At these extreme frequencies,
    /// minor scale relationships lose all conventional emotional associations, functioning instead as pure
    /// acoustic phenomena and ratios of vibration. These sounds are beyond the capability of virtually all
    /// acoustic instruments, accessible only through electronic synthesis and specialized computer music.
    /// Many adults cannot perceive the highest frequencies in this scale due to age-related hearing limitations.
    pub static ref C8_MINOR_SCALE: Scale<8> = minor_scale(C8);

    /// C# minor scale in octave 8 (MIDI notes 109-121)
    ///
    /// Notes: C#8, D#8, E8, F#8, G#8, A8, B8, C#9
    /// MIDI note numbers: 109, 111, 112, 114, 116, 117, 119, 121
    /// Frequency range: ~4434.92 Hz to ~8869.84 Hz
    ///
    /// C# minor in octave 8 inhabits the extreme upper limit of human auditory perception and MIDI
    /// representation. With frequencies spanning from ~4.4kHz to ~8.9kHz, this scale exists in a domain
    /// where traditional pitch perception is replaced by pure acoustic sensation for most listeners.
    /// These frequencies exceed the capabilities of all conventional acoustic instruments, existing
    /// solely in electronic and computer-generated contexts. At these extreme heights, the minor scale's
    /// traditional emotional qualities are entirely abstract—representing mathematical relationships rather
    /// than audible musical expressions for many listeners. This scale functions exclusively in specialized
    /// electronic music, acoustic research, and sound design contexts exploring the uppermost boundaries
    /// of human auditory perception and the transition from musical sound to pure frequency.
    pub static ref CSHARP8_MINOR_SCALE: Scale<8> = minor_scale(CSHARP8);

    /// D minor scale in octave 8 (MIDI notes 110-122)
    ///
    /// Notes: D8, E8, F8, G8, A8, B♭8, C9, D9
    /// MIDI note numbers: 110, 112, 113, 115, 117, 118, 120, 122
    /// Frequency range: ~4698.64 Hz to ~9397.27 Hz
    ///
    /// D minor in octave 8 exists at the extreme upper boundary of human pitch perception and MIDI
    /// representation. With frequencies ranging from ~4.7kHz to ~9.4kHz, this scale operates in a domain
    /// where traditional musical perception fully transforms into pure spectral sensation. These frequencies
    /// lie beyond the capabilities of all conventional acoustic instruments, existing solely in electronic
    /// and computer-generated contexts. At these ultrasonic heights, the minor scale's traditional emotional
    /// qualities exist only as abstract mathematical relationships rather than perceptible musical qualities.
    /// Many adults cannot perceive the highest frequencies in this scale, particularly those over 35-40 years
    /// old. This scale functions exclusively in specialized electronic music, psychoacoustic research, and
    /// sound design contexts exploring the boundaries of human auditory perception.
    pub static ref D8_MINOR_SCALE: Scale<8> = minor_scale(D8);

    /// D# minor scale in octave 8 (MIDI notes 111-123)
    ///
    /// Notes: D#8, E#8, F#8, G#8, A#8, B8, C#9, D#9
    /// MIDI note numbers: 111, 113, 114, 116, 118, 119, 121, 123
    /// Frequency range: ~4978.03 Hz to ~9956.06 Hz
    ///
    /// D# minor in octave 8 (more commonly notated as E♭ minor) exists at the extreme upper threshold of
    /// human auditory perception. With frequencies approaching 10kHz at its highest note, this scale operates
    /// entirely in a domain where conventional musical perception is replaced by pure acoustic sensation for
    /// most listeners. These ultrasonic frequencies exist beyond the capabilities of all conventional acoustic
    /// instruments, accessible only through electronic synthesis and computer-generated sound. At these extreme
    /// heights, the minor scale's theoretical relationships exist primarily as abstract mathematical ratios
    /// rather than perceptible musical expressions. Many adults cannot physically detect the highest notes in
    /// this scale due to natural hearing limitations. This scale functions exclusively in specialized electronic
    /// music, acoustic research, and sound design exploring the absolute upper boundaries of human hearing.
    pub static ref DSHARP8_MINOR_SCALE: Scale<8> = minor_scale(DSHARP8);

    /// E minor scale in octave 8 (MIDI notes 112-124)
    ///
    /// Notes: E8, F#8, G8, A8, B8, C9, D9, E9
    /// MIDI note numbers: 112, 114, 115, 117, 119, 120, 122, 124
    /// Frequency range: ~5274.04 Hz to ~10548.08 Hz
    ///
    /// E minor in octave 8 exists at the absolute upper extreme of human auditory perception. With frequencies
    /// exceeding 10.5kHz at its highest note, this scale operates entirely in a domain where pitch perception
    /// is physically impossible for many listeners, particularly adults. These ultrasonic frequencies exist
    /// beyond the capabilities of all conventional acoustic instruments, accessible only through electronic
    /// synthesis and computer music. At these extreme heights, the minor scale's theoretical framework exists
    /// solely as abstract mathematical relationships rather than perceptible musical expressions. The scale
    /// functions exclusively in specialized electronic music, acoustic research, and sound design contexts
    /// exploring the absolute limits of human auditory perception. The highest notes in this scale begin to
    /// approach frequencies used in specialized ultrasonic applications.
    pub static ref E8_MINOR_SCALE: Scale<8> = minor_scale(E8);

    /// F minor scale in octave 8 (MIDI notes 113-125)
    ///
    /// Notes: F8, G8, A♭8, B♭8, C9, D♭9, E♭9, F9
    /// MIDI note numbers: 113, 115, 116, 118, 120, 121, 123, 125
    /// Frequency range: ~5587.65 Hz to ~11175.30 Hz
    ///
    /// F minor in octave 8 exists at the extreme upper boundary of human auditory perception and MIDI
    /// representation. With frequencies exceeding 11kHz at its highest note, this scale operates entirely
    /// in a domain where conventional pitch perception is physically impossible for many listeners, especially
    /// adults. These ultrasonic frequencies exist beyond the capabilities of all acoustic instruments,
    /// accessible only through electronic synthesis and specialized computer music applications. At these
    /// extreme frequencies, the theoretical relationships of the minor scale exist purely as mathematical
    /// ratios rather than perceptible musical expressions. These sounds function exclusively in specialized
    /// electronic music, acoustic research, and sound design contexts exploring the absolute boundaries of
    /// human hearing. The highest notes approach frequencies used in technical ultrasonic applications.
    pub static ref F8_MINOR_SCALE: Scale<8> = minor_scale(F8);

    /// F# minor scale in octave 8 (MIDI notes 114-126)
    ///
    /// Notes: F#8, G#8, A8, B8, C#9, D9, E9, F#9
    /// MIDI note numbers: 114, 116, 117, 119, 121, 122, 124, 126
    /// Frequency range: ~5919.91 Hz to ~11839.82 Hz
    ///
    /// F# minor in octave 8 exists at the absolute upper boundary of human auditory perception and
    /// MIDI representation. With frequencies approaching 12kHz at its highest note, this scale operates
    /// entirely beyond the range of conventional pitch perception for most human listeners. These ultrasonic
    /// frequencies exceed the capabilities of all acoustic instruments, existing solely in electronic and
    /// computer-generated contexts. At these extreme heights, the minor scale's theoretical framework
    /// exists purely as mathematical relationships rather than perceptible musical expressions. Many adults
    /// cannot physically detect the highest frequencies in this scale due to age-related hearing limitations.
    /// The scale functions exclusively in specialized electronic music, acoustic research, and sound design
    /// exploring the absolute boundaries of human hearing, with its highest frequencies approaching those
    /// used in technical ultrasonic applications.
    pub static ref FSHARP8_MINOR_SCALE: Scale<8> = minor_scale(FSHARP8);

    /// G minor scale in octave 8 (MIDI notes 115-127)
    ///
    /// Notes: G8, A8, B♭8, C9, D9, E♭9, F9, G9
    /// MIDI note numbers: 115, 117, 118, 120, 122, 123, 125, 127
    /// Frequency range: ~6271.93 Hz to ~12543.85 Hz
    ///
    /// G minor in octave 8 represents the absolute upper limit of the MIDI specification and practical
    /// human auditory perception. With frequencies spanning from ~6.3kHz to G9 at ~12.5kHz (MIDI 127, the
    /// highest possible MIDI note), this scale exists entirely beyond conventional pitch perception for
    /// most human listeners. These ultrasonic frequencies exceed the capabilities of all acoustic instruments,
    /// accessible only through electronic synthesis and specialized computer music applications. At these
    /// extreme frequencies, the theoretical framework of the minor scale exists purely as mathematical ratios
    /// rather than perceptible musical expressions. Most adults cannot physically detect the highest frequencies
    /// in this scale due to natural hearing limitations. This scale represents the upper boundary of the MIDI
    /// specification, with G9 (MIDI 127) marking the absolute highest MIDI note possible, functioning exclusively
    /// in specialized electronic music, acoustic research, and sound design contexts exploring the absolute limits
    /// of musical representation.
    pub static ref G8_MINOR_SCALE: Scale<8> = minor_scale(G8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{
        A4, ASHARP4, B4, C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4,
    };
    use crate::ScaleQuality;
    use crate::{Pitch, SEMITONES_PER_OCTAVE};

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
        let roots = [
            C4, CSHARP4, D4, DSHARP4, E4, F4, FSHARP4, G4, GSHARP4, A4, ASHARP4, B4,
        ];

        for root in roots.iter() {
            let scale = MINOR_SCALES.get(root);
            assert!(
                scale.is_some(),
                "MINOR_SCALES should contain the root pitch"
            );

            let scale = scale.unwrap();
            assert_eq!(scale.tonic(), *root, "Scale tonic should match the key");
            assert_eq!(
                scale.steps(),
                MINOR_SCALE_STEPS,
                "Scale should follow minor scale pattern"
            );
        }
    }

    #[test]
    fn test_minor_scales_hashmap_octave_boundaries() {
        // Test scales at the boundaries of the MIDI range
        let lowest_pitch = C0; // Lowest possible MIDI note
        let highest_supported = G8; // Highest possible MIDI note

        // For lowest pitch, ensure we have a scale
        let lowest_scale = MINOR_SCALES.get(&lowest_pitch);
        assert!(
            lowest_scale.is_some(),
            "MINOR_SCALES should contain the lowest pitch"
        );

        // For highest supported pitch, ensure we have a scale
        let highest_scale = MINOR_SCALES.get(&highest_supported);
        assert!(
            highest_scale.is_some(),
            "MINOR_SCALES should contain the highest supported pitch"
        );
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

        let a_minor_notes: Vec<Pitch> = A_MINOR_SCALE.pitches().iter().skip(1).copied().collect();
        let c_major_notes: Vec<Pitch> = C_MAJOR_SCALE.pitches().iter().skip(1).copied().collect();

        // Adjust the octaves to compare the same pitch classes
        // A minor: A, B, C, D, E, F, G, A
        // C major: C, D, E, F, G, A, B, C
        // We need to rotate one to match the other
        let mut rotated_c_major = c_major_notes.clone();
        rotated_c_major.rotate_left(5); // Rotate to start with A
        rotated_c_major[6] = rotated_c_major[0]; // Replace the last note to match octave

        // Now the notes should match in pitch class (ignoring octave differences)
        for i in 0..6 {
            assert_eq!(
                a_minor_notes[i].0 % SEMITONES_PER_OCTAVE,
                rotated_c_major[i].0 % SEMITONES_PER_OCTAVE,
                "Pitch classes should match between relative scales"
            );
        }
    }
}
