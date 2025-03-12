use crate::{constants::*, melodic_scale};
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

/// The interval pattern for the ascending melodic minor scale.
///
/// The melodic minor scale is a unique scale that traditionally has different ascending
/// and descending forms, though this implementation represents only the ascending form.
/// It features a raised 6th and 7th degree compared to the natural minor scale, creating
/// a smoother melodic motion toward the tonic when ascending.
///
/// Interval pattern: Whole, Half, Whole, Whole, Whole, Whole, Half (W-H-W-W-W-W-H)
/// Scale degrees: 1, 2, ♭3, 4, 5, 6, 7, 8
///
/// The melodic minor scale is distinctive because:
/// - It has a minor third (flat 3rd) like natural minor, giving it a minor quality
/// - Its upper tetrachord (5-6-7-8) is identical to a major scale, creating a smooth
///   leading tone resolution
/// - This unique combination creates a sound that's often described as "jazzy" or "exotic"
///
/// In jazz theory, this scale (in its ascending form) is also called the "Jazz Minor Scale"
/// and forms the basis for several important modal scales used in jazz improvisation.
pub const MELODIC_SCALE_STEPS: [Interval; 7] = [TONE, SEMITONE, TONE, TONE, TONE, TONE, SEMITONE];

lazy_static! {
    /// A HashMap containing melodic minor scales for all possible pitches in the MIDI range.
    ///
    /// This collection maps each possible root pitch (from C-1 up to G9) to its corresponding
    /// melodic minor scale. The melodic minor scale implemented here is the ascending form,
    /// which is widely used in jazz and contemporary music.
    ///
    /// The HashMap is created at compile time using lazy_static and provides O(1) access to
    /// any melodic minor scale by its root pitch.
    ///
    /// # Examples
    ///
    /// ```
    /// use mozzart_std::{A4, MELODIC_SCALES};
    ///
    /// // Get the A melodic minor scale (A4 melodic minor)
    /// let a_melodic = MELODIC_SCALES.get(&A4).unwrap();
    ///
    /// // Retrieve the pitches of the A melodic minor scale
    /// let pitches = a_melodic.pitches();
    /// // pitches contains: A4, B4, C5, D5, E5, F♯5, G♯5, A5
    ///
    /// // Access melodic minor scale for any pitch in the MIDI range
    /// ```
    ///
    /// This HashMap is useful for algorithmic composition, music theory analysis, and
    /// whenever you need to work with melodic minor scales programmatically across different keys.
    pub static ref MELODIC_SCALES: HashMap<Pitch, Scale<8>> = {
        let mut map = HashMap::new();

        for p in C.0..G9.0 {
            let pitch = Pitch::new(p);
            map.insert(pitch, melodic_scale(pitch));
        }

        map
    };
}

lazy_static! {
    /// C melodic minor scale.
    ///
    /// Notes: C, D, E♭, F, G, A, B, C
    /// Key signature: 3 flats (E♭, B♭, A♭) with A and B natural in ascending form
    ///
    /// This scale is used extensively in classical music for its smooth ascending
    /// melodic motion. In jazz contexts, it's the basis for the Lydian dominant
    /// and the altered scale (7th mode of melodic minor).
    ///
    /// The C melodic minor scale contains the notes that would be played on the white
    /// and black keys of a piano starting from C with E♭ lowered.
    pub static ref C_MELODIC_SCALE: Scale<8> = melodic_scale(C);

    /// C♯ melodic minor scale.
    ///
    /// Notes: C♯, D♯, E, F♯, G♯, A♯, B♯, C♯
    /// Key signature: 4 sharps (F♯, C♯, G♯, D♯) with E natural and B♯ in ascending form
    ///
    /// A challenging scale in classical contexts due to its complex enharmonic
    /// spelling. In jazz and modern contexts, this scale offers rich harmonic
    /// possibilities when working in sharp keys.
    pub static ref CSHARP_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP);

    /// D melodic minor scale.
    ///
    /// Notes: D, E, F, G, A, B, C♯, D
    /// Key signature: 2 flats (B♭, E♭) with B and C♯ natural in ascending form
    ///
    /// The D melodic minor scale is common in baroque and classical repertoire.
    /// It creates a distinctive sound with its minor third (F) and major sixth and
    /// seventh (B and C♯).
    pub static ref D_MELODIC_SCALE: Scale<8> = melodic_scale(D);

    /// D♯ melodic minor scale.
    ///
    /// Notes: D♯, E♯, F♯, G♯, A♯, B♯, C♯♯, D♯
    /// Key signature: 6 sharps (F♯, C♯, G♯, D♯, A♯, E♯) with B♯ and C♯♯ in ascending form
    ///
    /// Typically written as E♭ melodic minor in practical usage due to its complex
    /// spelling with double sharps. It's less commonly used in its enharmonic D♯ form.
    pub static ref DSHARP_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP);

    /// E melodic minor scale.
    ///
    /// Notes: E, F♯, G, A, B, C♯, D♯, E
    /// Key signature: 1 sharp (F♯) with G natural, C♯ and D♯ in ascending form
    ///
    /// An important scale in classical guitar repertoire. The E melodic minor scale
    /// has a distinctive sound that's used in various genres from classical to
    /// contemporary jazz and rock.
    pub static ref E_MELODIC_SCALE: Scale<8> = melodic_scale(E);

    /// F melodic minor scale.
    ///
    /// Notes: F, G, A♭, B♭, C, D, E, F
    /// Key signature: 4 flats (B♭, E♭, A♭, D♭) with D and E natural in ascending form
    ///
    /// This scale creates a distinctive jazz sound when used over minor 7 chords.
    /// The F melodic minor scale's flat third with natural sixth and seventh creates
    /// interesting tension and resolution possibilities.
    pub static ref F_MELODIC_SCALE: Scale<8> = melodic_scale(F);

    /// F♯ melodic minor scale.
    ///
    /// Notes: F♯, G♯, A, B, C♯, D♯, E♯, F♯
    /// Key signature: 3 sharps (F♯, C♯, G♯) with A natural, D♯ and E♯ in ascending form
    ///
    /// Used in works by composers like Chopin and Debussy for its colorful sonority.
    /// The F♯ melodic minor scale is also utilized in modern jazz for its distinctive
    /// altered tensions.
    pub static ref FSHARP_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP);

    /// G melodic minor scale.
    ///
    /// Notes: G, A, B♭, C, D, E, F♯, G
    /// Key signature: 2 flats (B♭, E♭) with E natural and F♯ in ascending form
    ///
    /// Often used in classical violin repertoire. The G melodic minor scale creates
    /// a smooth melodic motion with its natural sixth and seventh when ascending,
    /// making it ideal for melodic passages.
    pub static ref G_MELODIC_SCALE: Scale<8> = melodic_scale(G);

    /// G♯ melodic minor scale.
    ///
    /// Notes: G♯, A♯, B, C♯, D♯, E♯, F♯♯, G♯
    /// Key signature: 5 sharps (F♯, C♯, G♯, D♯, A♯) with B natural, E♯ and F♯♯ in ascending form
    ///
    /// Typically written as A♭ melodic minor in practice. This scale is used in jazz
    /// improvisation, particularly over altered dominant chords, creating a tense,
    /// outside sound.
    pub static ref GSHARP_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP);

    /// A melodic minor scale.
    ///
    /// Notes: A, B, C, D, E, F♯, G♯, A
    /// Key signature: 0 flats/sharps (natural) with C natural, F♯ and G♯ in ascending form
    ///
    /// One of the most frequently used melodic minor scales in classical repertoire.
    /// It's the relative minor of C major, and its distinctive sound is found in works
    /// from Mozart to contemporary film scores.
    pub static ref A_MELODIC_SCALE: Scale<8> = melodic_scale(A);

    /// A♯ melodic minor scale.
    ///
    /// Notes: A♯, B♯, C♯, D♯, E♯, F♯♯, G♯♯, A♯
    /// Key signature: 7 sharps (F♯, C♯, G♯, D♯, A♯, E♯, B♯) with F♯♯ and G♯♯ in ascending form
    ///
    /// More commonly notated as B♭ melodic minor due to its complex spelling with
    /// double sharps. It's frequently used in jazz contexts over minor ii-V-I
    /// progressions in flat keys.
    pub static ref ASHARP_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP);

    /// B melodic minor scale.
    ///
    /// Notes: B, C♯, D, E, F♯, G♯, A♯, B
    /// Key signature: 2 sharps (F♯, C♯) with D natural, G♯ and A♯ in ascending form
    ///
    /// Used extensively in romantic piano repertoire. The B melodic minor scale
    /// has a rich, complex sound that creates tension and emotional depth in
    /// compositions by Chopin, Liszt, and other composers.
    pub static ref B_MELODIC_SCALE: Scale<8> = melodic_scale(B);
}

lazy_static! {
    /// C melodic minor scale in octave 0.
    ///
    /// Notes: C0, D0, E♭0, F0, G0, A0, B0, C1
    /// MIDI note numbers: 12, 14, 15, 17, 19, 21, 23, 24
    /// Frequency range: ~16.35 Hz to ~32.70 Hz
    ///
    /// This extremely low register melodic minor scale is below the range of human hearing at its lowest notes,
    /// but can be felt as vibrations. The upper notes of this scale enter the audible range but remain very deep.
    /// Useful for sub-bass electronic music applications and physical sound design.
    pub static ref C0_MELODIC_SCALE: Scale<8> = melodic_scale(C0);

    /// C♯ melodic minor scale in octave 0.
    ///
    /// Notes: C♯0, D♯0, E0, F♯0, G♯0, A♯0, B♯0, C♯1
    /// MIDI note numbers: 13, 15, 16, 18, 20, 22, 24, 25
    /// Frequency range: ~17.32 Hz to ~34.65 Hz
    ///
    /// A very low register scale useful primarily for electronic music and physical sound design.
    /// The lowest notes are at the threshold of human hearing, while the higher notes provide
    /// deep, foundational tones for sub-bass applications.
    pub static ref CSHARP0_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP0);

    /// D melodic minor scale in octave 0.
    ///
    /// Notes: D0, E0, F0, G0, A0, B0, C♯1, D1
    /// MIDI note numbers: 14, 16, 17, 19, 21, 23, 25, 26
    /// Frequency range: ~18.35 Hz to ~36.71 Hz
    ///
    /// This extremely low scale is at the bottom threshold of human hearing. While not typically
    /// used in traditional acoustic music, it's valuable for electronic music production to create
    /// sub-bass effects and vibrations that are felt more than heard.
    pub static ref D0_MELODIC_SCALE: Scale<8> = melodic_scale(D0);

    /// D♯ melodic minor scale in octave 0.
    ///
    /// Notes: D♯0, E♯0, F♯0, G♯0, A♯0, B♯0, C♯♯1, D♯1
    /// MIDI note numbers: 15, 17, 18, 20, 22, 24, 26, 27
    /// Frequency range: ~19.45 Hz to ~38.89 Hz
    ///
    /// At this extremely low register, the D♯ melodic minor scale produces frequencies that are
    /// more felt as vibrations than heard as distinct pitches. Primarily used in electronic music
    /// for sub-bass sounds and physical impact.
    pub static ref DSHARP0_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP0);

    /// E melodic minor scale in octave 0.
    ///
    /// Notes: E0, F♯0, G0, A0, B0, C♯1, D♯1, E1
    /// MIDI note numbers: 16, 18, 19, 21, 23, 25, 27, 28
    /// Frequency range: ~20.60 Hz to ~41.20 Hz
    ///
    /// This very low register E melodic minor scale spans from barely audible frequencies to
    /// those that can be distinctly heard. The upper notes of this scale begin to have more
    /// defined pitch characteristics while the lowest remain primarily felt as vibrations.
    pub static ref E0_MELODIC_SCALE: Scale<8> = melodic_scale(E0);

    /// F melodic minor scale in octave 0.
    ///
    /// Notes: F0, G0, A♭0, B♭0, C1, D1, E1, F1
    /// MIDI note numbers: 17, 19, 20, 22, 24, 26, 28, 29
    /// Frequency range: ~21.83 Hz to ~43.65 Hz
    ///
    /// In this extremely low register, the F melodic minor scale produces frequencies that begin
    /// to be audible but remain quite deep. The upper notes of this scale start to have more
    /// defined pitch characteristics while still providing a powerful sub-bass foundation.
    pub static ref F0_MELODIC_SCALE: Scale<8> = melodic_scale(F0);

    /// F♯ melodic minor scale in octave 0.
    ///
    /// Notes: F♯0, G♯0, A0, B0, C♯1, D♯1, E♯1, F♯1
    /// MIDI note numbers: 18, 20, 21, 23, 25, 27, 29, 30
    /// Frequency range: ~23.12 Hz to ~46.25 Hz
    ///
    /// This extremely low F♯ melodic minor scale lies at the threshold where pitches begin to be
    /// more clearly audible. While the lowest notes remain very deep, the upper notes begin to
    /// have more distinct tonal characteristics useful for bass-centered music.
    pub static ref FSHARP0_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP0);

    /// G melodic minor scale in octave 0.
    ///
    /// Notes: G0, A0, B♭0, C1, D1, E1, F♯1, G1
    /// MIDI note numbers: 19, 21, 22, 24, 26, 28, 30, 31
    /// Frequency range: ~24.50 Hz to ~49.00 Hz
    ///
    /// In this low register, the G melodic minor scale spans from very deep bass frequencies
    /// to lower bass notes. The upper part of this scale becomes more clearly audible while
    /// the bottom notes remain powerful sub-bass tones useful for modern music production.
    pub static ref G0_MELODIC_SCALE: Scale<8> = melodic_scale(G0);

    /// G♯ melodic minor scale in octave 0.
    ///
    /// Notes: G♯0, A♯0, B0, C♯1, D♯1, E♯1, F♯♯1, G♯1
    /// MIDI note numbers: 20, 22, 23, 25, 27, 29, 31, 32
    /// Frequency range: ~25.96 Hz to ~51.91 Hz
    ///
    /// This low register G♯ melodic minor scale features deep bass tones with the upper notes
    /// becoming more distinctly audible. The pitch relationships become more perceptible in
    /// this range, making it useful for bass parts in electronic and modern music.
    pub static ref GSHARP0_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP0);

    /// A melodic minor scale in octave 0.
    ///
    /// Notes: A0, B0, C1, D1, E1, F♯1, G♯1, A1
    /// MIDI note numbers: 21, 23, 24, 26, 28, 30, 32, 33
    /// Frequency range: ~27.50 Hz to ~55.00 Hz
    ///
    /// This A melodic minor scale in octave 0 spans from the lowest A on a standard piano (A0)
    /// to notes that are firmly in the bass register. At this range, the harmonic relationships
    /// become more easily perceptible, making it useful for bass lines and foundational harmonies.
    pub static ref A0_MELODIC_SCALE: Scale<8> = melodic_scale(A0);

    /// A♯ melodic minor scale in octave 0.
    ///
    /// Notes: A♯0, B♯0, C♯1, D♯1, E♯1, F♯♯1, G♯♯1, A♯1
    /// MIDI note numbers: 22, 24, 25, 27, 29, 31, 33, 34
    /// Frequency range: ~29.14 Hz to ~58.27 Hz
    ///
    /// In this low register, the A♯ melodic minor scale spans deep bass frequencies that are
    /// clearly audible but still very low. The complex harmonic character of the melodic minor
    /// scale becomes more noticeable at this range, useful for rich bass textures.
    pub static ref ASHARP0_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP0);

    /// B melodic minor scale in octave 0.
    ///
    /// Notes: B0, C♯1, D1, E1, F♯1, G♯1, A♯1, B1
    /// MIDI note numbers: 23, 25, 26, 28, 30, 32, 34, 35
    /// Frequency range: ~30.87 Hz to ~61.74 Hz
    ///
    /// The B melodic minor scale in octave 0 is the highest of the octave 0 scales, with its
    /// upper notes firmly in the audible bass range. This scale spans from very deep bass to
    /// standard bass frequencies, making it useful for complex bass lines in various genres.
    pub static ref B0_MELODIC_SCALE: Scale<8> = melodic_scale(B0);
}

lazy_static! {
    /// C melodic minor scale in octave 1.
    ///
    /// Notes: C1, D1, E♭1, F1, G1, A1, B1, C2
    /// MIDI note numbers: 24, 26, 27, 29, 31, 33, 35, 36
    /// Frequency range: ~32.70 Hz to ~65.41 Hz
    ///
    /// This scale lies in the deep bass register, providing foundational tones
    /// commonly used in bass instruments and electronic music. The C1 melodic minor scale
    /// offers rich harmonic possibilities while staying in a register that provides
    /// solid low-end support for musical compositions.
    pub static ref C1_MELODIC_SCALE: Scale<8> = melodic_scale(C1);

    /// C♯ melodic minor scale in octave 1.
    ///
    /// Notes: C♯1, D♯1, E1, F♯1, G♯1, A♯1, B♯1, C♯2
    /// MIDI note numbers: 25, 27, 28, 30, 32, 34, 36, 37
    /// Frequency range: ~34.65 Hz to ~69.30 Hz
    ///
    /// This bass register scale provides deep but clearly audible tones, useful
    /// for creating rich bass lines in various musical contexts. The unique melodic
    /// minor quality becomes more defined in this register, allowing for expressive
    /// bass passages with distinctive character.
    pub static ref CSHARP1_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP1);

    /// D melodic minor scale in octave 1.
    ///
    /// Notes: D1, E1, F1, G1, A1, B1, C♯2, D2
    /// MIDI note numbers: 26, 28, 29, 31, 33, 35, 37, 38
    /// Frequency range: ~36.71 Hz to ~73.42 Hz
    ///
    /// In the bass register, the D melodic minor scale provides a foundation for
    /// many musical styles. This range is commonly used for bass guitar, double bass,
    /// and synthesizer bass lines, where the distinctive melodic minor character
    /// creates interesting harmonic possibilities.
    pub static ref D1_MELODIC_SCALE: Scale<8> = melodic_scale(D1);

    /// D♯ melodic minor scale in octave 1.
    ///
    /// Notes: D♯1, E♯1, F♯1, G♯1, A♯1, B♯1, C♯♯2, D♯2
    /// MIDI note numbers: 27, 29, 30, 32, 34, 36, 38, 39
    /// Frequency range: ~38.89 Hz to ~77.78 Hz
    ///
    /// This bass register D♯ melodic minor scale (often notated as E♭ melodic minor)
    /// provides rich, warm tones in the clearly audible bass range. It's useful for
    /// crafting expressive bass lines with the distinctive tension created by the
    /// melodic minor's raised 6th and 7th degrees.
    pub static ref DSHARP1_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP1);

    /// E melodic minor scale in octave 1.
    ///
    /// Notes: E1, F♯1, G1, A1, B1, C♯2, D♯2, E2
    /// MIDI note numbers: 28, 30, 31, 33, 35, 37, 39, 40
    /// Frequency range: ~41.20 Hz to ~82.41 Hz
    ///
    /// The E melodic minor scale in octave 1 provides rich bass tones that work well
    /// for bass guitar and other bass instruments. This range offers good clarity while
    /// maintaining substantial low-end presence, making it ideal for expressive bass
    /// lines that utilize the melodic minor's unique character.
    pub static ref E1_MELODIC_SCALE: Scale<8> = melodic_scale(E1);

    /// F melodic minor scale in octave 1.
    ///
    /// Notes: F1, G1, A♭1, B♭1, C2, D2, E2, F2
    /// MIDI note numbers: 29, 31, 32, 34, 36, 38, 40, 41
    /// Frequency range: ~43.65 Hz to ~87.31 Hz
    ///
    /// In the bass register, the F melodic minor scale creates a warm foundation
    /// with clearly defined pitch relationships. This range is commonly used in bass
    /// instruments across various genres, where the melodic minor's unique intervallic
    /// structure can create distinctive basslines and harmonic textures.
    pub static ref F1_MELODIC_SCALE: Scale<8> = melodic_scale(F1);

    /// F♯ melodic minor scale in octave 1.
    ///
    /// Notes: F♯1, G♯1, A1, B1, C♯2, D♯2, E♯2, F♯2
    /// MIDI note numbers: 30, 32, 33, 35, 37, 39, 41, 42
    /// Frequency range: ~46.25 Hz to ~92.50 Hz
    ///
    /// The F♯ melodic minor scale in octave 1 offers bass tones with good definition.
    /// This register is useful for creating more complex bass lines that take advantage
    /// of the melodic minor's unique combination of minor quality with leading tone
    /// resolution, particularly effective in jazz and fusion contexts.
    pub static ref FSHARP1_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP1);

    /// G melodic minor scale in octave 1.
    ///
    /// Notes: G1, A1, B♭1, C2, D2, E2, F♯2, G2
    /// MIDI note numbers: 31, 33, 34, 36, 38, 40, 42, 43
    /// Frequency range: ~49.00 Hz to ~98.00 Hz
    ///
    /// This G melodic minor scale spans from deep bass to lower-mid bass frequencies.
    /// This register provides excellent clarity for bass lines while maintaining powerful
    /// low-end presence. The melodic minor quality is clearly articulated in this range,
    /// making it useful for expressive bass parts in various musical contexts.
    pub static ref G1_MELODIC_SCALE: Scale<8> = melodic_scale(G1);

    /// G♯ melodic minor scale in octave 1.
    ///
    /// Notes: G♯1, A♯1, B1, C♯2, D♯2, E♯2, F♯♯2, G♯2
    /// MIDI note numbers: 32, 34, 35, 37, 39, 41, 43, 44
    /// Frequency range: ~51.91 Hz to ~103.83 Hz
    ///
    /// The G♯ melodic minor scale (often notated as A♭ melodic minor) in octave 1
    /// provides bass tones that are clearly defined yet still firmly in the bass register.
    /// This scale is effective for crafting bass lines with melodic interest, particularly
    /// in jazz and contemporary musical styles.
    pub static ref GSHARP1_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP1);

    /// A melodic minor scale in octave 1.
    ///
    /// Notes: A1, B1, C2, D2, E2, F♯2, G♯2, A2
    /// MIDI note numbers: 33, 35, 36, 38, 40, 42, 44, 45
    /// Frequency range: ~55.00 Hz to ~110.00 Hz
    ///
    /// The A melodic minor scale in octave 1 spans from standard bass to lower-mid frequencies.
    /// This register is commonly used for bass guitar's standard range, offering clear pitches
    /// while maintaining solid low-end presence. The melodic minor quality provides interesting
    /// harmonic color for bass lines in various musical genres.
    pub static ref A1_MELODIC_SCALE: Scale<8> = melodic_scale(A1);

    /// A♯ melodic minor scale in octave 1.
    ///
    /// Notes: A♯1, B♯1, C♯2, D♯2, E♯2, F♯♯2, G♯♯2, A♯2
    /// MIDI note numbers: 34, 36, 37, 39, 41, 43, 45, 46
    /// Frequency range: ~58.27 Hz to ~116.54 Hz
    ///
    /// This A♯ melodic minor scale (often notated as B♭ melodic minor) sits in an optimal
    /// bass register that balances low-end power with pitch clarity. The scale spans from
    /// standard bass frequencies to lower-mid range, making it useful for expressive bass
    /// lines that take advantage of the melodic minor's unique character.
    pub static ref ASHARP1_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP1);

    /// B melodic minor scale in octave 1.
    ///
    /// Notes: B1, C♯2, D2, E2, F♯2, G♯2, A♯2, B2
    /// MIDI note numbers: 35, 37, 38, 40, 42, 44, 46, 47
    /// Frequency range: ~61.74 Hz to ~123.47 Hz
    ///
    /// The B melodic minor scale in octave 1 spans from deep bass to mid-bass frequencies.
    /// It's the highest of the octave 1 melodic minor scales, with its upper notes approaching
    /// the mid-range. This scale works well for bass lines that require more melodic flexibility
    /// and harmonic complexity in the upper bass register.
    pub static ref B1_MELODIC_SCALE: Scale<8> = melodic_scale(B1);
}

lazy_static! {
    /// C melodic minor scale in octave 2.
    ///
    /// Notes: C2, D2, E♭2, F2, G2, A2, B2, C3
    /// MIDI note numbers: 36, 38, 39, 41, 43, 45, 47, 48
    /// Frequency range: ~65.41 Hz to ~130.81 Hz
    ///
    /// This C melodic minor scale sits in the lower tenor range, providing a balance
    /// between bass foundation and melodic clarity. Common in cello, bass guitar, and
    /// lower piano parts, this register allows for clear articulation of the melodic minor's
    /// characteristic interval pattern while maintaining substantial warmth and depth.
    pub static ref C2_MELODIC_SCALE: Scale<8> = melodic_scale(C2);

    /// C♯ melodic minor scale in octave 2.
    ///
    /// Notes: C♯2, D♯2, E2, F♯2, G♯2, A♯2, B♯2, C♯3
    /// MIDI note numbers: 37, 39, 40, 42, 44, 46, 48, 49
    /// Frequency range: ~69.30 Hz to ~138.59 Hz
    ///
    /// Located in the lower tenor register, this C♯ melodic minor scale provides rich harmonies
    /// with good clarity. This range is frequently used in orchestral cello parts and lower
    /// register piano passages. The unique melodic minor sonority becomes more defined and
    /// expressive in this range.
    pub static ref CSHARP2_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP2);

    /// D melodic minor scale in octave 2.
    ///
    /// Notes: D2, E2, F2, G2, A2, B2, C♯3, D3
    /// MIDI note numbers: 38, 40, 41, 43, 45, 47, 49, 50
    /// Frequency range: ~73.42 Hz to ~146.83 Hz
    ///
    /// The D melodic minor scale in octave 2 occupies the lower tenor range, offering a good
    /// balance of warmth and clarity. This register is commonly used for cello melodies,
    /// electric bass solos, and lower piano parts. The scale's raised 6th and 7th degrees
    /// create expressive melodic possibilities in this sonorous register.
    pub static ref D2_MELODIC_SCALE: Scale<8> = melodic_scale(D2);

    /// D♯ melodic minor scale in octave 2.
    ///
    /// Notes: D♯2, E♯2, F♯2, G♯2, A♯2, B♯2, C♯♯3, D♯3
    /// MIDI note numbers: 39, 41, 42, 44, 46, 48, 50, 51
    /// Frequency range: ~77.78 Hz to ~155.56 Hz
    ///
    /// This D♯ melodic minor scale (often written as E♭ melodic minor) spans the lower tenor
    /// register. It provides a rich, warm sonority with good definition, making it ideal for
    /// expressive passages in cello and trombone repertoire, as well as jazz bass solos and
    /// lower register keyboard parts.
    pub static ref DSHARP2_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP2);

    /// E melodic minor scale in octave 2.
    ///
    /// Notes: E2, F♯2, G2, A2, B2, C♯3, D♯3, E3
    /// MIDI note numbers: 40, 42, 43, 45, 47, 49, 51, 52
    /// Frequency range: ~82.41 Hz to ~164.81 Hz
    ///
    /// The E melodic minor scale in octave 2 bridges the bass and tenor registers, offering
    /// a warm, resonant sound with good definition. This register works well for guitar bass
    /// strings, cello middle register, and lower trombone passages. The unique melodic minor
    /// sound is clearly articulated in this range.
    pub static ref E2_MELODIC_SCALE: Scale<8> = melodic_scale(E2);

    /// F melodic minor scale in octave 2.
    ///
    /// Notes: F2, G2, A♭2, B♭2, C3, D3, E3, F3
    /// MIDI note numbers: 41, 43, 44, 46, 48, 50, 52, 53
    /// Frequency range: ~87.31 Hz to ~174.61 Hz
    ///
    /// This F melodic minor scale sits in the tenor register, offering rich harmonics and
    /// good melodic clarity. It's commonly used in cello and viola repertoire, lower guitar
    /// positions, and left-hand piano parts. The scale's distinctive character becomes more
    /// pronounced in this register, making it useful for expressive passages.
    pub static ref F2_MELODIC_SCALE: Scale<8> = melodic_scale(F2);

    /// F♯ melodic minor scale in octave 2.
    ///
    /// Notes: F♯2, G♯2, A2, B2, C♯3, D♯3, E♯3, F♯3
    /// MIDI note numbers: 42, 44, 45, 47, 49, 51, 53, 54
    /// Frequency range: ~92.50 Hz to ~185.00 Hz
    ///
    /// The F♯ melodic minor scale in octave 2 spans the tenor register with good resonance
    /// and definition. This range is frequently used for viola and cello melodies, guitar
    /// chord voicings, and expressive jazz passages. The melodic minor's unique interval
    /// pattern creates rich harmonic possibilities in this register.
    pub static ref FSHARP2_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP2);

    /// G melodic minor scale in octave 2.
    ///
    /// Notes: G2, A2, B♭2, C3, D3, E3, F♯3, G3
    /// MIDI note numbers: 43, 45, 46, 48, 50, 52, 54, 55
    /// Frequency range: ~98.00 Hz to ~196.00 Hz
    ///
    /// This G melodic minor scale occupies the tenor register, offering excellent clarity
    /// and resonance. Commonly used in cello and viola melodic passages, guitar open position
    /// playing, and mid-range vocal parts. The melodic minor's distinctive quality is
    /// particularly expressive in this range.
    pub static ref G2_MELODIC_SCALE: Scale<8> = melodic_scale(G2);

    /// G♯ melodic minor scale in octave 2.
    ///
    /// Notes: G♯2, A♯2, B2, C♯3, D♯3, E♯3, F♯♯3, G♯3
    /// MIDI note numbers: 44, 46, 47, 49, 51, 53, 55, 56
    /// Frequency range: ~103.83 Hz to ~207.65 Hz
    ///
    /// The G♯ melodic minor scale (often notated as A♭ melodic minor) spans the tenor range
    /// with excellent clarity. This register is accessible to many instruments including viola,
    /// cello, tenor saxophone, and guitar. The complex melodic minor harmony becomes
    /// particularly expressive in this versatile register.
    pub static ref GSHARP2_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP2);

    /// A melodic minor scale in octave 2.
    ///
    /// Notes: A2, B2, C3, D3, E3, F♯3, G♯3, A3
    /// MIDI note numbers: 45, 47, 48, 50, 52, 54, 56, 57
    /// Frequency range: ~110.00 Hz to ~220.00 Hz
    ///
    /// This A melodic minor scale sits in the tenor register, offering excellent resonance
    /// and clarity. It's a versatile range accessible to viola, cello, tenor voice, and
    /// guitar. The distinctive melodic minor quality is particularly expressive in this
    /// register, making it ideal for solos and expressive melodic passages.
    pub static ref A2_MELODIC_SCALE: Scale<8> = melodic_scale(A2);

    /// A♯ melodic minor scale in octave 2.
    ///
    /// Notes: A♯2, B♯2, C♯3, D♯3, E♯3, F♯♯3, G♯♯3, A♯3
    /// MIDI note numbers: 46, 48, 49, 51, 53, 55, 57, 58
    /// Frequency range: ~116.54 Hz to ~233.08 Hz
    ///
    /// The A♯ melodic minor scale (often notated as B♭ melodic minor) spans the upper
    /// tenor register with excellent clarity and resonance. This versatile range is accessible
    /// to tenors, viola, guitar, and many wind instruments. The scale's complex melodic minor
    /// character is particularly expressive in this register.
    pub static ref ASHARP2_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP2);

    /// B melodic minor scale in octave 2.
    ///
    /// Notes: B2, C♯3, D3, E3, F♯3, G♯3, A♯3, B3
    /// MIDI note numbers: 47, 49, 50, 52, 54, 56, 58, 59
    /// Frequency range: ~123.47 Hz to ~246.94 Hz
    ///
    /// The B melodic minor scale in octave 2 bridges the tenor and alto registers with
    /// excellent resonance and clarity. This versatile range is ideal for viola, tenor
    /// saxophone, and mid-range piano passages. The melodic minor's unique character
    /// shines in this register, offering rich expressive possibilities for melodies.
    pub static ref B2_MELODIC_SCALE: Scale<8> = melodic_scale(B2);
}

lazy_static! {
    /// C melodic minor scale in octave 3.
    ///
    /// Notes: C3, D3, E♭3, F3, G3, A3, B3, C4
    /// MIDI note numbers: 48, 50, 51, 53, 55, 57, 59, 60
    /// Frequency range: ~130.81 Hz to ~261.63 Hz
    ///
    /// This C melodic minor scale in octave 3 sits in the middle register, providing excellent
    /// clarity and articulation. This is the central range for viola, guitar, and clarinet,
    /// offering a warm, expressive sound with good projection. The melodic minor's unique
    /// character is particularly clear in this versatile register.
    pub static ref C3_MELODIC_SCALE: Scale<8> = melodic_scale(C3);

    /// C♯ melodic minor scale in octave 3.
    ///
    /// Notes: C♯3, D♯3, E3, F♯3, G♯3, A♯3, B♯3, C♯4
    /// MIDI note numbers: 49, 51, 52, 54, 56, 58, 60, 61
    /// Frequency range: ~138.59 Hz to ~277.18 Hz
    ///
    /// The C♯ melodic minor scale in octave 3 offers excellent clarity and expressiveness.
    /// This register is commonly used for violin and viola parts, middle-range vocal writing,
    /// and guitar melodies. The melodic minor's distinctive interval pattern creates rich
    /// harmonic possibilities for melodic development.
    pub static ref CSHARP3_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP3);

    /// D melodic minor scale in octave 3.
    ///
    /// Notes: D3, E3, F3, G3, A3, B3, C♯4, D4
    /// MIDI note numbers: 50, 52, 53, 55, 57, 59, 61, 62
    /// Frequency range: ~146.83 Hz to ~293.66 Hz
    ///
    /// This D melodic minor scale in octave 3 spans the central register with excellent
    /// clarity and resonance. This range is particularly effective for viola melodies,
    /// alto vocal parts, clarinet passages, and guitar solos. The melodic minor quality
    /// shines in this versatile, expressive register.
    pub static ref D3_MELODIC_SCALE: Scale<8> = melodic_scale(D3);

    /// D♯ melodic minor scale in octave 3.
    ///
    /// Notes: D♯3, E♯3, F♯3, G♯3, A♯3, B♯3, C♯♯4, D♯4
    /// MIDI note numbers: 51, 53, 54, 56, 58, 60, 62, 63
    /// Frequency range: ~155.56 Hz to ~311.13 Hz
    ///
    /// The D♯ melodic minor scale (often written as E♭ melodic minor) spans the central
    /// register with excellent definition. This range is ideal for alto saxophone, clarinet,
    /// and violin melodies. The melodic minor's unique character offers rich expressive
    /// possibilities in this highly versatile register.
    pub static ref DSHARP3_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP3);

    /// E melodic minor scale in octave 3.
    ///
    /// Notes: E3, F♯3, G3, A3, B3, C♯4, D♯4, E4
    /// MIDI note numbers: 52, 54, 55, 57, 59, 61, 63, 64
    /// Frequency range: ~164.81 Hz to ~329.63 Hz
    ///
    /// This E melodic minor scale occupies a central range accessible to many instruments
    /// including violin, viola, alto voice, and guitar. The distinctive melodic minor
    /// color is particularly clear in this register, creating expressive possibilities
    /// for melodies and improvisations across various genres.
    pub static ref E3_MELODIC_SCALE: Scale<8> = melodic_scale(E3);

    /// F melodic minor scale in octave 3.
    ///
    /// Notes: F3, G3, A♭3, B♭3, C4, D4, E4, F4
    /// MIDI note numbers: 53, 55, 56, 58, 60, 62, 64, 65
    /// Frequency range: ~174.61 Hz to ~349.23 Hz
    ///
    /// The F melodic minor scale in octave 3 spans from the alto to the mid-soprano range,
    /// bridging central and upper-middle registers. This versatile range works excellently
    /// for clarinet, violin, and vocal melodic passages. The scale's unique color is well-defined
    /// and expressive in this register.
    pub static ref F3_MELODIC_SCALE: Scale<8> = melodic_scale(F3);

    /// F♯ melodic minor scale in octave 3.
    ///
    /// Notes: F♯3, G♯3, A3, B3, C♯4, D♯4, E♯4, F♯4
    /// MIDI note numbers: 54, 56, 57, 59, 61, 63, 65, 66
    /// Frequency range: ~185.00 Hz to ~369.99 Hz
    ///
    /// This F♯ melodic minor scale offers excellent clarity and projection in the central-to-upper
    /// register. Commonly used for expressive violin passages, alto saxophone solos, and vocal
    /// melodies. The melodic minor's distinctive tension and resolution is particularly effective
    /// in this range for melodic development.
    pub static ref FSHARP3_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP3);

    /// G melodic minor scale in octave 3.
    ///
    /// Notes: G3, A3, B♭3, C4, D4, E4, F♯4, G4
    /// MIDI note numbers: 55, 57, 58, 60, 62, 64, 66, 67
    /// Frequency range: ~196.00 Hz to ~392.00 Hz
    ///
    /// The G melodic minor scale in octave 3 spans from alto to soprano registers,
    /// making it a versatile range for many instruments and voices. This register is particularly
    /// effective for violin, flute, and clarinet melodies. The upper notes of this scale enter
    /// the range containing Middle C (C4), a central reference point in music.
    pub static ref G3_MELODIC_SCALE: Scale<8> = melodic_scale(G3);

    /// G♯ melodic minor scale in octave 3.
    ///
    /// Notes: G♯3, A♯3, B3, C♯4, D♯4, E♯4, F♯♯4, G♯4
    /// MIDI note numbers: 56, 58, 59, 61, 63, 65, 67, 68
    /// Frequency range: ~207.65 Hz to ~415.30 Hz
    ///
    /// This G♯ melodic minor scale (often notated as A♭ melodic minor) spans the alto
    /// and soprano registers with excellent clarity. The range is ideal for flute, violin,
    /// soprano saxophone, and upper clarinet passages. The complex melodic minor harmony
    /// is particularly vibrant and expressive in this versatile register.
    pub static ref GSHARP3_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP3);

    /// A melodic minor scale in octave 3.
    ///
    /// Notes: A3, B3, C4, D4, E4, F♯4, G♯4, A4
    /// MIDI note numbers: 57, 59, 60, 62, 64, 66, 68, 69
    /// Frequency range: ~220.00 Hz to ~440.00 Hz
    ///
    /// The A melodic minor scale in octave 3 spans from the alto to the soprano register.
    /// This range includes Middle C (C4) and reaches up to A4 (concert pitch A=440Hz),
    /// making it a central and highly versatile range for melody instruments. Ideal for
    /// violin, flute, oboe, and soprano voice.
    pub static ref A3_MELODIC_SCALE: Scale<8> = melodic_scale(A3);

    /// A♯ melodic minor scale in octave 3.
    ///
    /// Notes: A♯3, B♯3, C♯4, D♯4, E♯4, F♯♯4, G♯♯4, A♯4
    /// MIDI note numbers: 58, 60, 61, 63, 65, 67, 69, 70
    /// Frequency range: ~233.08 Hz to ~466.16 Hz
    ///
    /// This A♯ melodic minor scale (often notated as B♭ melodic minor) spans from alto
    /// to soprano registers. Highly versatile for melody instruments like the flute,
    /// violin, and soprano voice. The scale offers excellent projection and expressiveness
    /// in this range, with its distinctive melodic minor character clearly articulated.
    pub static ref ASHARP3_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP3);

    /// B melodic minor scale in octave 3.
    ///
    /// Notes: B3, C♯4, D4, E4, F♯4, G♯4, A♯4, B4
    /// MIDI note numbers: 59, 61, 62, 64, 66, 68, 70, 71
    /// Frequency range: ~246.94 Hz to ~493.88 Hz
    ///
    /// The B melodic minor scale in octave 3 extends from the upper alto into the soprano
    /// register. This range provides excellent projection and clarity, making it ideal for
    /// soprano voice, violin, flute, and oboe. The melodic minor's unique character is
    /// particularly brilliant and expressive in this upper-middle register.
    pub static ref B3_MELODIC_SCALE: Scale<8> = melodic_scale(B3);
}

lazy_static! {
    /// C melodic minor scale in octave 4.
    ///
    /// Notes: C4, D4, E♭4, F4, G4, A4, B4, C5
    /// MIDI note numbers: 60, 62, 63, 65, 67, 69, 71, 72
    /// Frequency range: ~261.63 Hz to ~523.25 Hz
    ///
    /// The C melodic minor scale in octave 4 begins with Middle C (C4), making it
    /// a central and highly versatile register. This range is ideal for soprano and tenor
    /// voices, violin, flute, and right-hand piano melodies. The melodic minor's unique
    /// character is particularly clear and expressive in this register.
    pub static ref C4_MELODIC_SCALE: Scale<8> = melodic_scale(C4);

    /// C♯ melodic minor scale in octave 4.
    ///
    /// Notes: C♯4, D♯4, E4, F♯4, G♯4, A♯4, B♯4, C♯5
    /// MIDI note numbers: 61, 63, 64, 66, 68, 70, 72, 73
    /// Frequency range: ~277.18 Hz to ~554.37 Hz
    ///
    /// This C♯ melodic minor scale occupies the central soprano register with excellent
    /// projection and clarity. Ideal for soprano voice, flute, violin, and upper woodwind
    /// instruments. This range supports expressive melodies with the distinctive melodic
    /// minor quality clearly articulated and resonant.
    pub static ref CSHARP4_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP4);

    /// D melodic minor scale in octave 4.
    ///
    /// Notes: D4, E4, F4, G4, A4, B4, C♯5, D5
    /// MIDI note numbers: 62, 64, 65, 67, 69, 71, 73, 74
    /// Frequency range: ~293.66 Hz to ~587.33 Hz
    ///
    /// The D melodic minor scale in octave 4 spans the central soprano register,
    /// including concert pitch A4 (440 Hz) in its range. This versatile register
    /// is excellent for flute, violin, soprano voice, and upper woodwinds. The melodic
    /// minor's unique character offers rich expressive potential in this brilliant register.
    pub static ref D4_MELODIC_SCALE: Scale<8> = melodic_scale(D4);

    /// D♯ melodic minor scale in octave 4.
    ///
    /// Notes: D♯4, E♯4, F♯4, G♯4, A♯4, B♯4, C♯♯5, D♯5
    /// MIDI note numbers: 63, 65, 66, 68, 70, 72, 74, 75
    /// Frequency range: ~311.13 Hz to ~622.25 Hz
    ///
    /// This D♯ melodic minor scale (often written as E♭ melodic minor) sits in the soprano
    /// register, offering brilliant clarity and projection. Commonly used for soprano voice,
    /// flute, and piccolo, this register showcases the melodic minor scale's distinctive
    /// tension and resolution characteristics with crystalline clarity.
    pub static ref DSHARP4_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP4);

    /// E melodic minor scale in octave 4.
    ///
    /// Notes: E4, F♯4, G4, A4, B4, C♯5, D♯5, E5
    /// MIDI note numbers: 64, 66, 67, 69, 71, 73, 75, 76
    /// Frequency range: ~329.63 Hz to ~659.26 Hz
    ///
    /// The E melodic minor scale in octave 4 occupies the soprano register and includes
    /// concert pitch A4 (440 Hz). This brilliant register is ideal for soprano voice,
    /// flute, and violin. The scale offers excellent projection and clarity, with its
    /// distinctive melodic minor character creating expressive melodic possibilities.
    pub static ref E4_MELODIC_SCALE: Scale<8> = melodic_scale(E4);

    /// F melodic minor scale in octave 4.
    ///
    /// Notes: F4, G4, A♭4, B♭4, C5, D5, E5, F5
    /// MIDI note numbers: 65, 67, 68, 70, 72, 74, 76, 77
    /// Frequency range: ~349.23 Hz to ~698.46 Hz
    ///
    /// This F melodic minor scale spans the soprano register with brilliant clarity and
    /// projection. This range works perfectly for soprano voice, flute, piccolo, and
    /// violin. The distinctive melodic minor quality shines in this register, creating
    /// vibrant, expressive melodic lines with excellent resonance.
    pub static ref F4_MELODIC_SCALE: Scale<8> = melodic_scale(F4);

    /// F♯ melodic minor scale in octave 4.
    ///
    /// Notes: F♯4, G♯4, A4, B4, C♯5, D♯5, E♯5, F♯5
    /// MIDI note numbers: 66, 68, 69, 71, 73, 75, 77, 78
    /// Frequency range: ~369.99 Hz to ~739.99 Hz
    ///
    /// The F♯ melodic minor scale in octave 4 spans the upper soprano register, including
    /// concert pitch A4 (440 Hz). This brilliant range is ideal for soprano voice, piccolo,
    /// and violin. The melodic minor's unique color creates striking melodic possibilities
    /// with excellent projection and clarity in this high register.
    pub static ref FSHARP4_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP4);

    /// G melodic minor scale in octave 4.
    ///
    /// Notes: G4, A4, B♭4, C5, D5, E5, F♯5, G5
    /// MIDI note numbers: 67, 69, 70, 72, 74, 76, 78, 79
    /// Frequency range: ~392.00 Hz to ~784.00 Hz
    ///
    /// This G melodic minor scale occupies the upper soprano register, containing concert
    /// pitch A4 (440 Hz). This brilliant range offers excellent projection for soprano
    /// voice, piccolo, and violin. The melodic minor's distinctive character creates
    /// vibrant, penetrating melodic lines in this high register.
    pub static ref G4_MELODIC_SCALE: Scale<8> = melodic_scale(G4);

    /// G♯ melodic minor scale in octave 4.
    ///
    /// Notes: G♯4, A♯4, B4, C♯5, D♯5, E♯5, F♯♯5, G♯5
    /// MIDI note numbers: 68, 70, 71, 73, 75, 77, 79, 80
    /// Frequency range: ~415.30 Hz to ~830.61 Hz
    ///
    /// The G♯ melodic minor scale (often notated as A♭ melodic minor) spans the high soprano
    /// register. This brilliant range is favored by coloratura sopranos, piccolo, and high
    /// violin passages. The distinctive melodic minor sound creates shimmering, bright
    /// melodic lines with extraordinary projection in this high register.
    pub static ref GSHARP4_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP4);

    /// A melodic minor scale in octave 4.
    ///
    /// Notes: A4, B4, C5, D5, E5, F♯5, G♯5, A5
    /// MIDI note numbers: 69, 71, 72, 74, 76, 78, 80, 81
    /// Frequency range: ~440.00 Hz to ~880.00 Hz
    ///
    /// This A melodic minor scale begins on concert pitch A4 (440 Hz) and spans the high
    /// soprano register. This brilliant range is perfect for coloratura soprano, piccolo,
    /// and high violin passages. The melodic minor quality creates sparkling, radiant
    /// melodic lines with exceptional projection and clarity.
    pub static ref A4_MELODIC_SCALE: Scale<8> = melodic_scale(A4);

    /// A♯ melodic minor scale in octave 4.
    ///
    /// Notes: A♯4, B♯4, C♯5, D♯5, E♯5, F♯♯5, G♯♯5, A♯5
    /// MIDI note numbers: 70, 72, 73, 75, 77, 79, 81, 82
    /// Frequency range: ~466.16 Hz to ~932.33 Hz
    ///
    /// The A♯ melodic minor scale (often notated as B♭ melodic minor) occupies the high
    /// soprano register. This extremely bright range is used for coloratura soprano,
    /// piccolo, and the highest violin passages. The melodic minor's complex character
    /// creates dazzling, brilliant melodic lines in this high register.
    pub static ref ASHARP4_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP4);

    /// B melodic minor scale in octave 4.
    ///
    /// Notes: B4, C♯5, D5, E5, F♯5, G♯5, A♯5, B5
    /// MIDI note numbers: 71, 73, 74, 76, 78, 80, 82, 83
    /// Frequency range: ~493.88 Hz to ~987.77 Hz
    ///
    /// This B melodic minor scale spans the highest soprano register. This exceptionally
    /// brilliant range is utilized by coloratura sopranos, piccolo, and the uppermost
    /// violin passages. The melodic minor's distinctive character creates glistening,
    /// ethereal melodic lines with extraordinary brilliance and projection.
    pub static ref B4_MELODIC_SCALE: Scale<8> = melodic_scale(B4);
}

lazy_static! {
    /// C melodic minor scale in octave 5.
    ///
    /// Notes: C5, D5, E♭5, F5, G5, A5, B5, C6
    /// MIDI note numbers: 72, 74, 75, 77, 79, 81, 83, 84
    /// Frequency range: ~523.25 Hz to ~1046.50 Hz
    ///
    /// This C melodic minor scale spans the high soprano/sopranino register.
    /// This brilliant range is primarily used by coloratura sopranos, piccolo, and
    /// violin harmonics. The distinctive melodic minor quality creates sparkling,
    /// radiant melodic lines with exceptional clarity and brilliance in this high register.
    pub static ref C5_MELODIC_SCALE: Scale<8> = melodic_scale(C5);

    /// C♯ melodic minor scale in octave 5.
    ///
    /// Notes: C♯5, D♯5, E5, F♯5, G♯5, A♯5, B♯5, C♯6
    /// MIDI note numbers: 73, 75, 76, 78, 80, 82, 84, 85
    /// Frequency range: ~554.37 Hz to ~1108.73 Hz
    ///
    /// The C♯ melodic minor scale in octave 5 sits in the extreme high soprano register.
    /// This exceptionally brilliant range is utilized primarily by piccolo, violin harmonics,
    /// and coloratura soprano in their highest range. The melodic minor's complex color
    /// creates shimmering, crystalline melodic lines with extraordinary brightness.
    pub static ref CSHARP5_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP5);

    /// D melodic minor scale in octave 5.
    ///
    /// Notes: D5, E5, F5, G5, A5, B5, C♯6, D6
    /// MIDI note numbers: 74, 76, 77, 79, 81, 83, 85, 86
    /// Frequency range: ~587.33 Hz to ~1174.66 Hz
    ///
    /// This D melodic minor scale in octave 5 occupies the highest soprano/sopranino register.
    /// This extremely bright range is primarily used for piccolo, glockenspiel, and the very
    /// highest notes of coloratura soprano repertoire. The melodic minor's distinctive
    /// tension and resolution creates dazzling, radiant melodic lines at this extreme height.
    pub static ref D5_MELODIC_SCALE: Scale<8> = melodic_scale(D5);

    /// D♯ melodic minor scale in octave 5.
    ///
    /// Notes: D♯5, E♯5, F♯5, G♯5, A♯5, B♯5, C♯♯6, D♯6
    /// MIDI note numbers: 75, 77, 78, 80, 82, 84, 86, 87
    /// Frequency range: ~622.25 Hz to ~1244.51 Hz
    ///
    /// The D♯ melodic minor scale (often notated as E♭ melodic minor) in octave 5 spans
    /// an extremely high register that approaches the upper limits of soprano voices.
    /// This brilliantly penetrating range is primarily used for piccolo, high synthesizer
    /// parts, and the highest extremes of coloratura technique. The melodic minor's unique
    /// color creates scintillating, ethereal melodic lines at this altitude.
    pub static ref DSHARP5_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP5);

    /// E melodic minor scale in octave 5.
    ///
    /// Notes: E5, F♯5, G5, A5, B5, C♯6, D♯6, E6
    /// MIDI note numbers: 76, 78, 79, 81, 83, 85, 87, 88
    /// Frequency range: ~659.26 Hz to ~1318.51 Hz
    ///
    /// This E melodic minor scale sits in the extreme upper soprano register, approaching
    /// the limits of the human vocal range. This exceptionally brilliant register is primarily
    /// used for piccolo, celesta, glockenspiel, and extended techniques on flute and violin.
    /// The melodic minor's distinctive character creates shimmering, transparent textures.
    pub static ref E5_MELODIC_SCALE: Scale<8> = melodic_scale(E5);

    /// F melodic minor scale in octave 5.
    ///
    /// Notes: F5, G5, A♭5, B♭5, C6, D6, E6, F6
    /// MIDI note numbers: 77, 79, 80, 82, 84, 86, 88, 89
    /// Frequency range: ~698.46 Hz to ~1396.91 Hz
    ///
    /// The F melodic minor scale in octave 5 occupies an extremely high register beyond
    /// the range of most singers. This brilliantly piercing register is primarily used
    /// for piccolo, toy piano, celesta, and glockenspiel. The melodic minor quality creates
    /// glistening, ethereal sonorities with exceptional brilliance at this extreme height.
    pub static ref F5_MELODIC_SCALE: Scale<8> = melodic_scale(F5);

    /// F♯ melodic minor scale in octave 5.
    ///
    /// Notes: F♯5, G♯5, A5, B5, C♯6, D♯6, E♯6, F♯6
    /// MIDI note numbers: 78, 80, 81, 83, 85, 87, 89, 90
    /// Frequency range: ~739.99 Hz to ~1479.98 Hz
    ///
    /// This F♯ melodic minor scale spans an extremely high register that exceeds the
    /// range of human voices. This extraordinarily brilliant register is primarily used for
    /// piccolo, celesta, and electronic tones. The melodic minor's complex character
    /// creates sparkling, crystalline textures of extraordinary delicacy and penetration.
    pub static ref FSHARP5_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP5);

    /// G melodic minor scale in octave 5.
    ///
    /// Notes: G5, A5, B♭5, C6, D6, E6, F♯6, G6
    /// MIDI note numbers: 79, 81, 82, 84, 86, 88, 90, 91
    /// Frequency range: ~784.00 Hz to ~1568.00 Hz
    ///
    /// The G melodic minor scale in octave 5 reaches an extremely high register
    /// beyond conventional vocal ranges. This extraordinarily brilliant register is
    /// primarily used for piccolo, glockenspiel, and celesta. The melodic minor character
    /// creates transparent, shimmering textures with exceptional brilliance and penetration.
    pub static ref G5_MELODIC_SCALE: Scale<8> = melodic_scale(G5);

    /// G♯ melodic minor scale in octave 5.
    ///
    /// Notes: G♯5, A♯5, B5, C♯6, D♯6, E♯6, F♯♯6, G♯6
    /// MIDI note numbers: 80, 82, 83, 85, 87, 89, 91, 92
    /// Frequency range: ~830.61 Hz to ~1661.22 Hz
    ///
    /// This G♯ melodic minor scale (often notated as A♭ melodic minor) occupies an extreme
    /// high register used primarily by percussion instruments like glockenspiel and celesta,
    /// as well as piccolo in its highest range. The melodic minor's distinctive color
    /// creates delicate, crystalline textures with extraordinary brilliance at this altitude.
    pub static ref GSHARP5_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP5);

    /// A melodic minor scale in octave 5.
    ///
    /// Notes: A5, B5, C6, D6, E6, F♯6, G♯6, A6
    /// MIDI note numbers: 81, 83, 84, 86, 88, 90, 92, 93
    /// Frequency range: ~880.00 Hz to ~1760.00 Hz
    ///
    /// The A melodic minor scale in octave 5 spans an extremely high register well beyond
    /// the range of human voices. This extraordinarily brilliant register is primarily used
    /// for celesta, glockenspiel, and the highest piccolo notes. The melodic minor's unique
    /// character creates ethereal, transparent textures of exceptional delicacy and brilliance.
    pub static ref A5_MELODIC_SCALE: Scale<8> = melodic_scale(A5);

    /// A♯ melodic minor scale in octave 5.
    ///
    /// Notes: A♯5, B♯5, C♯6, D♯6, E♯6, F♯♯6, G♯♯6, A♯6
    /// MIDI note numbers: 82, 84, 85, 87, 89, 91, 93, 94
    /// Frequency range: ~932.33 Hz to ~1864.66 Hz
    ///
    /// This A♯ melodic minor scale (often notated as B♭ melodic minor) occupies an extremely
    /// high register at the upper limit of most acoustic instruments. Used primarily by
    /// glockenspiel, celesta, crotales, and piccolo at its extreme range. The melodic minor's
    /// distinctive tension creates gossamer, crystalline textures of extraordinary brilliance.
    pub static ref ASHARP5_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP5);

    /// B melodic minor scale in octave 5.
    ///
    /// Notes: B5, C♯6, D6, E6, F♯6, G♯6, A♯6, B6
    /// MIDI note numbers: 83, 85, 86, 88, 90, 92, 94, 95
    /// Frequency range: ~987.77 Hz to ~1975.53 Hz
    ///
    /// The B melodic minor scale in octave 5 represents an extremely high register approaching
    /// the limits of human hearing sensitivity. This exceptionally brilliant register is used by
    /// glockenspiel, celesta, crotales, and electronic tones. The melodic minor's unique
    /// character creates diaphanous, glistening textures of extraordinary delicacy and brilliance.
    pub static ref B5_MELODIC_SCALE: Scale<8> = melodic_scale(B5);
}

lazy_static! {
    /// C melodic minor scale in octave 6.
    ///
    /// Notes: C6, D6, E♭6, F6, G6, A6, B6, C7
    /// MIDI note numbers: 84, 86, 87, 89, 91, 93, 95, 96
    /// Frequency range: ~1046.50 Hz to ~2093.00 Hz
    ///
    /// This C melodic minor scale spans an extremely high register beyond the capability
    /// of most acoustic instruments. This ultra-bright register is used primarily for
    /// glockenspiel, celesta, crotales, and electronic tones. The melodic minor's distinctive
    /// character creates gossamer, hyper-brilliant textures with extraordinary shimmer.
    pub static ref C6_MELODIC_SCALE: Scale<8> = melodic_scale(C6);

    /// C♯ melodic minor scale in octave 6.
    ///
    /// Notes: C♯6, D♯6, E6, F♯6, G♯6, A♯6, B♯6, C♯7
    /// MIDI note numbers: 85, 87, 88, 90, 92, 94, 96, 97
    /// Frequency range: ~1108.73 Hz to ~2217.46 Hz
    ///
    /// The C♯ melodic minor scale in octave 6 occupies an extremely high register that
    /// exceeds the range of nearly all acoustic instruments except percussion. This
    /// extraordinarily brilliant register is used by instruments like crotales, high
    /// celesta, and electronic sounds. The melodic minor's unique color creates a
    /// scintillating, crystalline shimmer at this extreme altitude.
    pub static ref CSHARP6_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP6);

    /// D melodic minor scale in octave 6.
    ///
    /// Notes: D6, E6, F6, G6, A6, B6, C♯7, D7
    /// MIDI note numbers: 86, 88, 89, 91, 93, 95, 97, 98
    /// Frequency range: ~1174.66 Hz to ~2349.32 Hz
    ///
    /// This D melodic minor scale in octave 6 reaches an exceptionally high register at
    /// the extreme limits of acoustic instrumental range. This hyper-brilliant register
    /// is primarily used for percussion instruments like crotales and celesta, as well as
    /// electronic tones. The melodic minor quality creates an ethereal, translucent
    /// shimmer with extraordinary delicacy and penetration.
    pub static ref D6_MELODIC_SCALE: Scale<8> = melodic_scale(D6);

    /// D♯ melodic minor scale in octave 6.
    ///
    /// Notes: D♯6, E♯6, F♯6, G♯6, A♯6, B♯6, C♯♯7, D♯7
    /// MIDI note numbers: 87, 89, 90, 92, 94, 96, 98, 99
    /// Frequency range: ~1244.51 Hz to ~2489.02 Hz
    ///
    /// The D♯ melodic minor scale in octave 6 (often notated as E♭ melodic minor) spans an
    /// extremely high register beyond standard instrumental ranges. This register is used
    /// almost exclusively by percussion instruments like crotales and electronic sounds.
    /// The scale's unique melodic minor quality creates a glassy, crystalline sound with
    /// extraordinary brightness and minimal harmonic complexity.
    pub static ref DSHARP6_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP6);

    /// E melodic minor scale in octave 6.
    ///
    /// Notes: E6, F♯6, G6, A6, B6, C♯7, D♯7, E7
    /// MIDI note numbers: 88, 90, 91, 93, 95, 97, 99, 100
    /// Frequency range: ~1318.51 Hz to ~2637.02 Hz
    ///
    /// This E melodic minor scale occupies an ultrahigh register that approaches the limits
    /// of human pitch perception. This extremely bright register is primarily used by
    /// percussion instruments like crotales and electronic tones. The melodic minor's
    /// distinctive character creates an almost crystalline, diaphanous quality with
    /// extraordinary brilliance and minimal harmonic density.
    pub static ref E6_MELODIC_SCALE: Scale<8> = melodic_scale(E6);

    /// F melodic minor scale in octave 6.
    ///
    /// Notes: F6, G6, A♭6, B♭6, C7, D7, E7, F7
    /// MIDI note numbers: 89, 91, 92, 94, 96, 98, 100, 101
    /// Frequency range: ~1396.91 Hz to ~2793.83 Hz
    ///
    /// The F melodic minor scale in octave 6 reaches an extremely high register that
    /// approaches the threshold of pitch perception. This ultra-brilliant register is
    /// primarily used for crotales, electronic sounds, and digital synthesis. The melodic
    /// minor quality creates a glistening, glass-like sonority with extraordinary brightness
    /// and virtually no harmonic complexity.
    pub static ref F6_MELODIC_SCALE: Scale<8> = melodic_scale(F6);

    /// F♯ melodic minor scale in octave 6.
    ///
    /// Notes: F♯6, G♯6, A6, B6, C♯7, D♯7, E♯7, F♯7
    /// MIDI note numbers: 90, 92, 93, 95, 97, 99, 101, 102
    /// Frequency range: ~1479.98 Hz to ~2959.96 Hz
    ///
    /// This F♯ melodic minor scale spans an extraordinarily high register well beyond
    /// standard acoustic instrument ranges. This ultra-brilliant register is primarily used
    /// in electronic music and certain specialized percussion like crotales. The melodic
    /// minor's complex character is reduced to a crystalline, bell-like quality with
    /// extreme brightness and minimal tonal complexity.
    pub static ref FSHARP6_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP6);

    /// G melodic minor scale in octave 6.
    ///
    /// Notes: G6, A6, B♭6, C7, D7, E7, F♯7, G7
    /// MIDI note numbers: 91, 93, 94, 96, 98, 100, 102, 103
    /// Frequency range: ~1568.00 Hz to ~3136.00 Hz
    ///
    /// The G melodic minor scale in octave 6 occupies an extremely high register approaching
    /// the upper threshold of pitch discrimination for many listeners. This extraordinarily
    /// brilliant register is used primarily for electronic sounds, digital synthesis, and
    /// the highest percussion like crotales. The scale creates a pure, ethereal shimmer with
    /// virtually no harmonic density.
    pub static ref G6_MELODIC_SCALE: Scale<8> = melodic_scale(G6);

    /// G♯ melodic minor scale in octave 6.
    ///
    /// Notes: G♯6, A♯6, B6, C♯7, D♯7, E♯7, F♯♯7, G♯7
    /// MIDI note numbers: 92, 94, 95, 97, 99, 101, 103, 104
    /// Frequency range: ~1661.22 Hz to ~3322.44 Hz
    ///
    /// This G♯ melodic minor scale (often notated as A♭ melodic minor) spans an ultra-high
    /// register nearing the upper limits of pitch perception. This extraordinarily brilliant
    /// register is primarily used for special effects in electronic music and digital synthesis.
    /// The melodic minor's distinctive color is reduced to pure, crystalline tones with
    /// extreme brightness and virtually no perceptible harmonic content.
    pub static ref GSHARP6_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP6);

    /// A melodic minor scale in octave 6.
    ///
    /// Notes: A6, B6, C7, D7, E7, F♯7, G♯7, A7
    /// MIDI note numbers: 93, 95, 96, 98, 100, 102, 104, 105
    /// Frequency range: ~1760.00 Hz to ~3520.00 Hz
    ///
    /// The A melodic minor scale in octave 6 reaches an extremely high register approaching
    /// the limits of pitch perception. This extraordinarily brilliant register is primarily
    /// used for electronic music, digital synthesis, and specialized sound design. The melodic
    /// minor's distinctive character creates a pure, crystalline shimmer that is perceived
    /// more as timbre than as harmony at this extreme altitude.
    pub static ref A6_MELODIC_SCALE: Scale<8> = melodic_scale(A6);

    /// A♯ melodic minor scale in octave 6.
    ///
    /// Notes: A♯6, B♯6, C♯7, D♯7, E♯7, F♯♯7, G♯♯7, A♯7
    /// MIDI note numbers: 94, 96, 97, 99, 101, 103, 105, 106
    /// Frequency range: ~1864.66 Hz to ~3729.31 Hz
    ///
    /// This A♯ melodic minor scale (often notated as B♭ melodic minor) spans an ultra-high
    /// register at the threshold of distinct pitch perception. This extraordinarily brilliant
    /// register is used almost exclusively in electronic music and sound design. The melodic
    /// minor's distinctive quality creates a hyper-brilliant, transparent shimmer that begins
    /// to function more as pure color than as traditional harmony.
    pub static ref ASHARP6_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP6);

    /// B melodic minor scale in octave 6.
    ///
    /// Notes: B6, C♯7, D7, E7, F♯7, G♯7, A♯7, B7
    /// MIDI note numbers: 95, 97, 98, 100, 102, 104, 106, 107
    /// Frequency range: ~1975.53 Hz to ~3951.07 Hz
    ///
    /// The B melodic minor scale in octave 6 occupies the highest register still broadly
    /// recognized as distinct pitches. This extraordinarily brilliant register is used
    /// primarily in electronic music and digital synthesis for special effects. At this
    /// extreme altitude, the melodic minor's distinctive character creates a hyper-bright,
    /// crystalline shimmer that functions as pure timbral color rather than traditional harmony.
    pub static ref B6_MELODIC_SCALE: Scale<8> = melodic_scale(B6);
}

lazy_static! {
    /// C melodic minor scale in octave 7.
    ///
    /// Notes: C7, D7, E♭7, F7, G7, A7, B7, C8
    /// MIDI note numbers: 96, 98, 99, 101, 103, 105, 107, 108
    /// Frequency range: ~2093.00 Hz to ~4186.01 Hz
    ///
    /// This C melodic minor scale exists in an ultra-high register at the threshold of
    /// human pitch perception. This rarefied sonic territory is accessible only to
    /// specialized electronic sounds and a very few percussion instruments like crotales
    /// at their extreme range. The melodic minor sonority at this altitude is perceived
    /// primarily as pure timbre rather than as conventional harmony.
    pub static ref C7_MELODIC_SCALE: Scale<8> = melodic_scale(C7);

    /// C♯ melodic minor scale in octave 7.
    ///
    /// Notes: C♯7, D♯7, E7, F♯7, G♯7, A♯7, B♯7, C♯8
    /// MIDI note numbers: 97, 99, 100, 102, 104, 106, 108, 109
    /// Frequency range: ~2217.46 Hz to ~4434.92 Hz
    ///
    /// The C♯ melodic minor scale in octave 7 reaches an extremely high register beyond
    /// conventional instrumental possibilities. This hyper-brilliant register exists
    /// almost exclusively in the domain of electronic music synthesis and digital sound
    /// design. At these frequencies, the distinctive melodic minor quality transforms into
    /// pure spectral color with virtually no harmonic complexity.
    pub static ref CSHARP7_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP7);

    /// D melodic minor scale in octave 7.
    ///
    /// Notes: D7, E7, F7, G7, A7, B7, C♯8, D8
    /// MIDI note numbers: 98, 100, 101, 103, 105, 107, 109, 110
    /// Frequency range: ~2349.32 Hz to ~4698.64 Hz
    ///
    /// This D melodic minor scale occupies an ultra-high register approaching the upper
    /// limits of human hearing. This extraordinarily bright frequency range is primarily
    /// used in electronic music, sound design, and software synthesis. The melodic minor's
    /// distinctive intervallic pattern is perceived mainly as spectral coloration rather
    /// than as traditional scale relationships at this extreme altitude.
    pub static ref D7_MELODIC_SCALE: Scale<8> = melodic_scale(D7);

    /// D♯ melodic minor scale in octave 7.
    ///
    /// Notes: D♯7, E♯7, F♯7, G♯7, A♯7, B♯7, C♯♯8, D♯8
    /// MIDI note numbers: 99, 101, 102, 104, 106, 108, 110, 111
    /// Frequency range: ~2489.02 Hz to ~4978.03 Hz
    ///
    /// The D♯ melodic minor scale (often notated as E♭ melodic minor) in octave 7 reaches
    /// frequency ranges at the periphery of human pitch perception. This ultra-high register
    /// is primarily utilized in electronic music, advanced synthesis, and specialized sound
    /// design. The melodic minor quality here creates microsonic textures of extreme brilliance
    /// that function primarily as timbral colors.
    pub static ref DSHARP7_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP7);

    /// E melodic minor scale in octave 7.
    ///
    /// Notes: E7, F♯7, G7, A7, B7, C♯8, D♯8, E8
    /// MIDI note numbers: 100, 102, 103, 105, 107, 109, 111, 112
    /// Frequency range: ~2637.02 Hz to ~5274.04 Hz
    ///
    /// This E melodic minor scale spans an extremely high register approaching the limits of
    /// human hearing acuity. This ultra-brilliant register is exclusive to electronic sounds
    /// and digital synthesis, lying beyond conventional instrumental ranges. The melodic minor
    /// quality creates a hyper-bright spectral coloration that functions primarily as pure
    /// acoustic energy rather than as perceived harmony.
    pub static ref E7_MELODIC_SCALE: Scale<8> = melodic_scale(E7);

    /// F melodic minor scale in octave 7.
    ///
    /// Notes: F7, G7, A♭7, B♭7, C8, D8, E8, F8
    /// MIDI note numbers: 101, 103, 104, 106, 108, 110, 112, 113
    /// Frequency range: ~2793.83 Hz to ~5587.65 Hz
    ///
    /// The F melodic minor scale in octave 7 extends into frequency ranges where pitch
    /// discrimination becomes increasingly difficult for human hearing. This ultra-high
    /// register is used exclusively in electronic music, sound design, and advanced synthesis.
    /// The melodic minor quality at these frequencies manifests primarily as spectral texture
    /// rather than as harmonic relationships.
    pub static ref F7_MELODIC_SCALE: Scale<8> = melodic_scale(F7);

    /// F♯ melodic minor scale in octave 7.
    ///
    /// Notes: F♯7, G♯7, A7, B7, C♯8, D♯8, E♯8, F♯8
    /// MIDI note numbers: 102, 104, 105, 107, 109, 111, 113, 114
    /// Frequency range: ~2959.96 Hz to ~5919.91 Hz
    ///
    /// This F♯ melodic minor scale exists in an ultra-high register where conventional
    /// concepts of pitch begin to blur into pure sonic color. This extraordinary frequency
    /// range is accessible only through electronic synthesis and advanced digital sound design.
    /// The melodic minor intervals create spectral textures that are perceived more as timbral
    /// qualities than as distinctly recognizable pitches.
    pub static ref FSHARP7_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP7);

    /// G melodic minor scale in octave 7.
    ///
    /// Notes: G7, A7, B♭7, C8, D8, E8, F♯8, G8
    /// MIDI note numbers: 103, 105, 106, 108, 110, 112, 114, 115
    /// Frequency range: ~3136.00 Hz to ~6271.93 Hz
    ///
    /// The G melodic minor scale in octave 7 occupies an extraordinarily high register at
    /// the upper boundaries of human pitch perception. This extreme register is utilized
    /// exclusively in electronic music, advanced synthesis, and specialized sound design.
    /// At these frequencies, the melodic minor's intervallic structure creates shimmering
    /// spectral textures that function as pure acoustic energy.
    pub static ref G7_MELODIC_SCALE: Scale<8> = melodic_scale(G7);

    /// G♯ melodic minor scale in octave 7.
    ///
    /// Notes: G♯7, A♯7, B7, C♯8, D♯8, E♯8, F♯♯8, G♯8
    /// MIDI note numbers: 104, 106, 107, 109, 111, 113, 115, 116
    /// Frequency range: ~3322.44 Hz to ~6644.88 Hz
    ///
    /// This G♯ melodic minor scale (often notated as A♭ melodic minor) spans an ultra-high
    /// register where pitch identity begins to dissolve into pure spectral energy. This
    /// extreme frequency range is accessible only through advanced digital synthesis.
    /// The melodic minor quality creates microsonic textures that exist at the very threshold
    /// of pitch perception, functioning primarily as timbre and acoustic sensation.
    pub static ref GSHARP7_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP7);

    /// A melodic minor scale in octave 7.
    ///
    /// Notes: A7, B7, C8, D8, E8, F♯8, G♯8, A8
    /// MIDI note numbers: 105, 107, 108, 110, 112, 114, 116, 117
    /// Frequency range: ~3520.00 Hz to ~7040.00 Hz
    ///
    /// The A melodic minor scale in octave 7 extends to the uppermost region of human pitch
    /// perception. This ultra-high register is accessible only through electronic means and
    /// specialized synthesis techniques. The melodic minor intervals at these extreme frequencies
    /// create shimmering spectral textures that are perceived primarily as acoustic sensation
    /// rather than as distinct pitches with harmonic relationships.
    pub static ref A7_MELODIC_SCALE: Scale<8> = melodic_scale(A7);

    /// A♯ melodic minor scale in octave 7.
    ///
    /// Notes: A♯7, B♯7, C♯8, D♯8, E♯8, F♯♯8, G♯♯8, A♯8
    /// MIDI note numbers: 106, 108, 109, 111, 113, 115, 117, 118
    /// Frequency range: ~3729.31 Hz to ~7458.62 Hz
    ///
    /// This A♯ melodic minor scale (often notated as B♭ melodic minor) occupies an ultra-high
    /// register at the extreme upper limit of human hearing acuity. This extraordinary frequency
    /// range is only accessible through advanced digital synthesis and electronic sound design.
    /// The melodic minor quality manifests as pure spectral texture, creating acoustical energy
    /// patterns that transcend conventional notions of harmony and pitch.
    pub static ref ASHARP7_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP7);

    /// B melodic minor scale in octave 7.
    ///
    /// Notes: B7, C♯8, D8, E8, F♯8, G♯8, A♯8, B8
    /// MIDI note numbers: 107, 109, 110, 112, 114, 116, 118, 119
    /// Frequency range: ~3951.07 Hz to ~7902.13 Hz
    ///
    /// The B melodic minor scale in octave 7 reaches the absolute limits of human pitch
    /// perception. This extraordinary frequency range is exclusively the domain of electronic
    /// sound synthesis and digital sound design. At this extreme altitude, the melodic minor's
    /// distinctive character creates ultra-bright spectral textures that are perceived as
    /// pure acoustic energy, with individual pitches barely distinguishable. The scale's highest
    /// notes approach 8 kHz, nearing the upper threshold of human hearing.
    pub static ref B7_MELODIC_SCALE: Scale<8> = melodic_scale(B7);
}

lazy_static! {
    /// C melodic minor scale in octave 8.
    ///
    /// Notes: C8, D8, E♭8, F8, G8, A8, B8, C9
    /// MIDI note numbers: 108, 110, 111, 113, 115, 117, 119, 120
    /// Frequency range: ~4186.01 Hz to ~8372.02 Hz
    ///
    /// This C melodic minor scale exists at the absolute threshold of human pitch perception.
    /// Starting at the uppermost C on a standard piano (C8), these frequencies are accessible
    /// only through digital synthesis and advanced electronic music techniques. At this extreme
    /// altitude, melodic minor intervals are no longer perceived as musical relationships but
    /// as pure spectral components and acoustic energy. These frequencies overlap with critical
    /// bands used in human speech perception.
    pub static ref C8_MELODIC_SCALE: Scale<8> = melodic_scale(C8);

    /// C♯ melodic minor scale in octave 8.
    ///
    /// Notes: C♯8, D♯8, E8, F♯8, G♯8, A♯8, B♯8, C♯9
    /// MIDI note numbers: 109, 111, 112, 114, 116, 118, 120, 121
    /// Frequency range: ~4434.92 Hz to ~8869.84 Hz
    ///
    /// The C♯ melodic minor scale in octave 8 extends beyond conventional pitch perception
    /// into the realm of pure spectral color and acoustic sensation. This extreme register
    /// exists solely in digital synthesis and electronic sound design. The highest notes of
    /// this scale approach 9 kHz, well past the point where most humans can distinguish pitch
    /// relationships, functioning instead as pure psychoacoustic phenomena.
    pub static ref CSHARP8_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP8);

    /// D melodic minor scale in octave 8.
    ///
    /// Notes: D8, E8, F8, G8, A8, B8, C♯9, D9
    /// MIDI note numbers: 110, 112, 113, 115, 117, 119, 121, 122
    /// Frequency range: ~4698.64 Hz to ~9397.28 Hz
    ///
    /// This D melodic minor scale occupies a frequency range at the uppermost limits of
    /// human hearing capability. This ultra-extreme register is exclusively the domain of
    /// digital synthesis and specialized electronic sound design. At these frequencies,
    /// interval relationships become theoretical constructs rather than perceptual realities,
    /// as the human ear perceives these sounds primarily as brilliance, air, or "presence"
    /// regions in the audio spectrum.
    pub static ref D8_MELODIC_SCALE: Scale<8> = melodic_scale(D8);

    /// D♯ melodic minor scale in octave 8.
    ///
    /// Notes: D♯8, E♯8, F♯8, G♯8, A♯8, B♯8, C♯♯9, D♯9
    /// MIDI note numbers: 111, 113, 114, 116, 118, 120, 122, 123
    /// Frequency range: ~4978.03 Hz to ~9956.06 Hz
    ///
    /// The D♯ melodic minor scale in octave 8 spans frequencies that approach the uppermost
    /// limits of human hearing. This ultra-high register exists only in the domain of advanced
    /// digital synthesis and electronic sound design. With its highest note approaching 10 kHz,
    /// this scale extends into frequencies where pitch identity is almost entirely superseded by
    /// pure spectral sensation and psychoacoustic effects.
    pub static ref DSHARP8_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP8);

    /// E melodic minor scale in octave 8.
    ///
    /// Notes: E8, F♯8, G8, A8, B8, C♯9, D♯9, E9
    /// MIDI note numbers: 112, 114, 115, 117, 119, 121, 123, 124
    /// Frequency range: ~5274.04 Hz to ~10548.08 Hz
    ///
    /// This E melodic minor scale occupies a frequency range that extends beyond the
    /// limits of practical musical pitch perception. This hyper-extreme register exists
    /// exclusively in digital synthesis and specialized sound design. With its highest
    /// notes exceeding 10 kHz, this scale ventures into frequencies used primarily
    /// for audio mixing "air bands" and psychoacoustic enhancement rather than for
    /// musical pitch content.
    pub static ref E8_MELODIC_SCALE: Scale<8> = melodic_scale(E8);

    /// F melodic minor scale in octave 8.
    ///
    /// Notes: F8, G8, A♭8, B♭8, C9, D9, E9, F9
    /// MIDI note numbers: 113, 115, 116, 118, 120, 122, 124, 125
    /// Frequency range: ~5587.65 Hz to ~11175.30 Hz
    ///
    /// The F melodic minor scale in octave 8 reaches frequencies that transcend
    /// conventional musical perception entirely. This ultra-high register exists purely
    /// in the realm of advanced digital synthesis and electronic sound design. With
    /// its highest notes exceeding 11 kHz, this scale extends into frequencies where
    /// the human ear perceives only brightness or "shimmer" rather than distinct pitches
    /// or harmonic relationships.
    pub static ref F8_MELODIC_SCALE: Scale<8> = melodic_scale(F8);

    /// F♯ melodic minor scale in octave 8.
    ///
    /// Notes: F♯8, G♯8, A8, B8, C♯9, D♯9, E♯9, F♯9
    /// MIDI note numbers: 114, 116, 117, 119, 121, 123, 125, 126
    /// Frequency range: ~5919.91 Hz to ~11839.82 Hz
    ///
    /// This F♯ melodic minor scale spans frequencies that reach the uppermost extremes of
    /// human auditory perception. This ultra-high register exists exclusively as electronic
    /// synthesis and digital signal processing phenomena. With its highest frequencies
    /// approaching 12 kHz, this scale ventures into spectral regions typically used for
    /// "presence" enhancement in audio mixing rather than for musical pitch content.
    pub static ref FSHARP8_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP8);

    /// G melodic minor scale in octave 8.
    ///
    /// Notes: G8, A8, B♭8, C9, D9, E9, F♯9, G9
    /// MIDI note numbers: 115, 117, 118, 120, 122, 124, 126, 127
    /// Frequency range: ~6271.93 Hz to ~12543.86 Hz
    ///
    /// The G melodic minor scale in octave 8 represents the absolute uppermost limit
    /// of the MIDI pitch range, with its final note G9 corresponding to MIDI note 127.
    /// This ultra-high register exists solely in the realm of digital synthesis and
    /// electronic sound design. With frequencies exceeding 12.5 kHz, this scale ventures
    /// deep into the "brilliance" or "air" range of human hearing, where distinct pitch
    /// perception is replaced by pure acoustic sensation and psychoacoustic effects.
    pub static ref G8_MELODIC_SCALE: Scale<8> = melodic_scale(G8);
}
