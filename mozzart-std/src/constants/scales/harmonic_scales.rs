use crate::{constants::*, harmonic_scale};
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

/// The harmonic minor scale interval structure.
///
/// This array defines the sequence of intervals that form the harmonic minor scale:
/// - TONE (whole step): Between scale degrees 1-2
/// - SEMITONE (half step): Between scale degrees 2-3
/// - TONE (whole step): Between scale degrees 3-4
/// - TONE (whole step): Between scale degrees 4-5
/// - SEMITONE (half step): Between scale degrees 5-6
/// - MINOR_THIRD (three half steps): Between scale degrees 6-7 - the characteristic interval
/// - SEMITONE (half step): Between scale degrees 7-8 (octave)
///
/// The harmonic minor scale is characterized by its augmented second interval (MINOR_THIRD)
/// between the 6th and 7th scale degrees, which creates its distinctive exotic sound.
/// This interval gives the scale a Middle Eastern or Eastern European flavor, commonly
/// found in Jewish, Arabic, Gypsy, and Spanish Flamenco music.
///
/// Unlike the natural minor (which flattens scale degrees 3, 6, and 7) or the melodic minor
/// (which flattens scale degree 3 ascending but reverts to natural minor descending),
/// the harmonic minor consistently flattens only scale degrees 3 and 6, keeping the 7th
/// scale degree raised to create a stronger resolution to the tonic.
///
/// This raised 7th degree creates a leading tone that provides tension and a stronger
/// pull toward the tonic, allowing for the dominant (V) chord to be major rather than minor,
/// which enables the powerful V-i cadence used extensively in classical music.
pub const HARMONIC_SCALE_STEPS: [Interval; 7] =
    [TONE, SEMITONE, TONE, TONE, SEMITONE, MINOR_THIRD, SEMITONE];

lazy_static! {
    /// A comprehensive mapping of all possible pitches to their corresponding harmonic minor scales.
    ///
    /// This HashMap provides O(1) lookup access to any harmonic minor scale by its root pitch.
    /// It contains all harmonic minor scales from the lowest MIDI pitch C0 (MIDI note 12) to
    /// the highest representable pitch G9 (MIDI note 127).
    ///
    /// Key features:
    /// - Complete coverage of the entire MIDI pitch range
    /// - Each scale is pre-constructed and cached for efficient retrieval
    /// - Scales are built using the harmonic_scale() function with the HARMONIC_SCALE_STEPS intervals
    /// - All scales have a length of 8 notes (root through octave)
    ///
    /// This collection is particularly useful for:
    /// - Rapid scale lookup during performance or improvisation tools
    /// - Harmonic analysis of musical passages
    /// - Scale-based pattern generation and transposition
    /// - Educational applications demonstrating scale relationships
    ///
    /// Note: This uses lazy initialization to avoid the memory cost until first access.
    pub static ref HARMONIC_SCALES: HashMap<Pitch, Scale<8>> = {
        let mut map = HashMap::new();

        for p in C.0..G9.0 {
            let pitch = Pitch::new(p);
            map.insert(pitch, harmonic_scale(pitch));
        }

        map
    };
}

lazy_static! {
    /// C harmonic minor scale (octave-agnostic).
    ///
    /// This is the prototypical harmonic minor scale, with notes: C, D, E♭, F, G, A♭, B, C.
    ///
    /// The C harmonic minor is often considered the reference harmonic minor scale due to its
    /// lack of accidentals in its natural minor form. The raised 7th degree (B) creates the
    /// characteristic augmented second interval (A♭ to B) that gives harmonic minor its distinctive
    /// sound. This scale is prominently featured in classical piano repertoire, particularly in
    /// works by Beethoven, Chopin, and Rachmaninoff.
    pub static ref C_HARMONIC_SCALE: Scale<8> = harmonic_scale(C);

    /// C♯ harmonic minor scale (octave-agnostic).
    ///
    /// Notes: C♯, D♯, E, F♯, G♯, A, B♯, C♯.
    ///
    /// The C♯ harmonic minor scale creates a particularly dramatic sound with its augmented
    /// second between A and B♯. This tonality appears in Romantic and Post-Romantic piano
    /// works, notably in pieces by Rachmaninoff and Scriabin. Due to its challenging key
    /// signature, it is sometimes enharmonically rewritten as D♭ harmonic minor.
    pub static ref CSHARP_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP);

    /// D harmonic minor scale (octave-agnostic).
    ///
    /// Notes: D, E, F, G, A, B♭, C♯, D.
    ///
    /// The D harmonic minor creates a distinctly plaintive mood with its augmented second
    /// interval between B♭ and C♯. This scale is frequently employed in Baroque music, especially
    /// violin repertoire, as well as in Spanish flamenco and Middle Eastern traditions. Bach's
    /// D minor works often utilize the harmonic minor's raised 7th degree for cadential passages.
    pub static ref D_HARMONIC_SCALE: Scale<8> = harmonic_scale(D);

    /// D♯ harmonic minor scale (octave-agnostic).
    ///
    /// Notes: D♯, E♯, F♯, G♯, A♯, B, C♯♯, D♯.
    ///
    /// The D♯ harmonic minor is a rarely used key due to its complex notation with double-sharps.
    /// In practice, it is almost always enharmonically rewritten as E♭ harmonic minor. When it
    /// appears, it's typically in modulatory passages or in theoretical contexts rather than as
    /// a primary key center.
    pub static ref DSHARP_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP);

    /// E harmonic minor scale (octave-agnostic).
    ///
    /// Notes: E, F♯, G, A, B, C, D♯, E.
    ///
    /// The E harmonic minor scale creates a dramatic, intense character with its augmented second
    /// interval between C and D♯. This key is extensively used in classical guitar repertoire,
    /// flamenco music, and metal. In classical music, it often conveys passionate or tragic themes,
    /// as in Chopin's E minor works where the raised 7th adds dramatic tension.
    pub static ref E_HARMONIC_SCALE: Scale<8> = harmonic_scale(E);

    /// F harmonic minor scale (octave-agnostic).
    ///
    /// Notes: F, G, A♭, B♭, C, D♭, E, F.
    ///
    /// The F harmonic minor scale has a distinctly dark and mysterious quality, with its augmented
    /// second interval between D♭ and E creating an exotic sound. This scale appears in Romantic
    /// piano literature, orchestral works, and Eastern European folk music. The tension between
    /// the flattened 6th (D♭) and the raised 7th (E) gives it a particularly unstable, yearning
    /// character.
    pub static ref F_HARMONIC_SCALE: Scale<8> = harmonic_scale(F);

    /// F♯ harmonic minor scale (octave-agnostic).
    ///
    /// Notes: F♯, G♯, A, B, C♯, D, E♯, F♯.
    ///
    /// The F♯ harmonic minor scale has a rich, exotic quality with its augmented second between
    /// D and E♯. This scale appears in Romantic and Modern compositions, particularly those
    /// with a Eastern European or Middle Eastern influence. The key is prominently featured
    /// in Brahms' works and is valued for its ability to generate both tension and melancholy.
    pub static ref FSHARP_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP);

    /// G harmonic minor scale (octave-agnostic).
    ///
    /// Notes: G, A, B♭, C, D, E♭, F♯, G.
    ///
    /// The G harmonic minor scale combines the melancholy of minor with the distinctive tension
    /// created by the augmented second interval between E♭ and F♯. This scale is frequently
    /// found in Baroque music, particularly organ works, as well as in Eastern European folk
    /// traditions. Mozart's G minor symphonies and Bach's G minor works often employ the harmonic
    /// minor's raised leading tone at cadential points.
    pub static ref G_HARMONIC_SCALE: Scale<8> = harmonic_scale(G);

    /// G♯ harmonic minor scale (octave-agnostic).
    ///
    /// Notes: G♯, A♯, B, C♯, D♯, E, F♯♯, G♯.
    ///
    /// The G♯ harmonic minor scale is complex to notate with its double-sharp on F, and it
    /// is typically rewritten enharmonically as A♭ harmonic minor in practice. When it appears,
    /// it's often in modulatory passages or brief tonal diversions rather than as a primary
    /// tonality, primarily due to its challenging notation.
    pub static ref GSHARP_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP);

    /// A harmonic minor scale (octave-agnostic).
    ///
    /// Notes: A, B, C, D, E, F, G♯, A.
    ///
    /// The A harmonic minor scale is perhaps the most frequently used harmonic minor scale,
    /// serving as a prototypical example of the harmonic minor sound. With its distinctive
    /// augmented second interval between F and G♯, it creates the quintessential "exotic"
    /// minor sound. This scale is extensively featured in classical repertoire (notably in
    /// Mozart's famous "Rondo alla Turca"), flamenco guitar, and Eastern European folk music.
    pub static ref A_HARMONIC_SCALE: Scale<8> = harmonic_scale(A);

    /// A♯ harmonic minor scale (octave-agnostic).
    ///
    /// Notes: A♯, B♯, C♯, D♯, E♯, F♯, G♯♯, A♯.
    ///
    /// The A♯ harmonic minor scale is theoretically complex with its multiple sharps and
    /// double-sharp. In practice, it is almost always rewritten enharmonically as B♭ harmonic
    /// minor. This scale would only appear in highly unusual harmonic contexts or in theoretical
    /// discussions rather than practical composition.
    pub static ref ASHARP_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP);

    /// B harmonic minor scale (octave-agnostic).
    ///
    /// Notes: B, C♯, D, E, F♯, G, A♯, B.
    ///
    /// The B harmonic minor scale has a uniquely intense quality with its augmented second
    /// between G and A♯. This scale appears in romantic piano works, violin concertos, and
    /// Spanish-influenced music. The high leading tone creates a particularly strong pull
    /// toward the tonic, making it effective for creating dramatic tension and resolution.
    /// Chopin, Liszt, and later Impressionist composers employed this scale for its rich
    /// expressive potential.
    pub static ref B_HARMONIC_SCALE: Scale<8> = harmonic_scale(B);
}

lazy_static! {
    /// C harmonic minor scale in octave 0.
    ///
    /// Notes: C0, D0, E♭0, F0, G0, A♭0, B0, C1
    /// MIDI note numbers: 12, 14, 15, 17, 19, 20, 23, 24
    /// Frequency range: ~16.35 Hz to ~32.70 Hz
    ///
    /// This extremely low register represents the bottom octave of standard MIDI, beginning with
    /// the lowest C on a full-range piano. At these frequencies, the harmonic minor's distinctive
    /// augmented second interval is virtually imperceptible as a melodic feature, as the entire
    /// scale falls largely below the threshold of clear pitch perception (~20 Hz). These notes
    /// are experienced more as rhythmic pulses than as distinct pitches. This register is used
    /// exclusively for specialized sub-bass applications in electronic music, film scoring for
    /// seismic effects, and experimental compositions exploring the boundaries of perception.
    pub static ref C0_HARMONIC_SCALE: Scale<8> = harmonic_scale(C0);

    /// C♯ harmonic minor scale in octave 0.
    ///
    /// Notes: C♯0, D♯0, E0, F♯0, G♯0, A0, B♯0, C♯1
    /// MIDI note numbers: 13, 15, 16, 18, 20, 21, 24, 25
    /// Frequency range: ~17.32 Hz to ~34.65 Hz
    ///
    /// This C♯ harmonic minor scale occupies a frequency range that spans the threshold of human
    /// pitch perception (~20 Hz). At these extremely low frequencies, the scale's notes function
    /// more as tactile vibrations than as clearly defined pitches. The augmented second interval
    /// characteristic of harmonic minor scales is primarily theoretical at this register. This
    /// scale is exclusively used in specialized electronic music contexts for sub-bass effects and
    /// rumble textures rather than for melodic or harmonic content.
    pub static ref CSHARP0_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP0);

    /// D harmonic minor scale in octave 0.
    ///
    /// Notes: D0, E0, F0, G0, A0, B♭0, C♯1, D1
    /// MIDI note numbers: 14, 16, 17, 19, 21, 22, 25, 26
    /// Frequency range: ~18.35 Hz to ~36.71 Hz
    ///
    /// The D harmonic minor scale in octave 0 spans a frequency range that begins below the
    /// threshold of clear pitch perception and extends into the lowest audible bass range. The
    /// harmonic minor's characteristic raised 7th degree (C♯1) is the first note to cross into
    /// a more clearly audible pitch range. These frequencies are primarily felt as physical
    /// vibrations rather than heard as musical pitches. This scale is encountered in specialized
    /// electronic music production, particularly in genres that emphasize extreme sub-bass
    /// frequencies, such as certain forms of experimental electronic music.
    pub static ref D0_HARMONIC_SCALE: Scale<8> = harmonic_scale(D0);

    /// D♯ harmonic minor scale in octave 0.
    ///
    /// Notes: D♯0, E♯0, F♯0, G♯0, A♯0, B0, C♯♯1, D♯1
    /// MIDI note numbers: 15, 17, 18, 20, 22, 23, 26, 27
    /// Frequency range: ~19.45 Hz to ~38.89 Hz
    ///
    /// This D♯ harmonic minor scale spans a frequency range that begins at the very threshold of
    /// human pitch perception and extends into the low bass region. At these frequencies, the notes
    /// function primarily as sub-bass signals, with the upper notes of the scale beginning to take
    /// on distinct pitch qualities. The theoretical double-sharp (C♯♯1) would be enharmonically
    /// notated as D1 in practice. This scale appears exclusively in electronic music contexts for
    /// creating specialized sub-bass effects and rumble textures.
    pub static ref DSHARP0_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP0);

    /// E harmonic minor scale in octave 0.
    ///
    /// Notes: E0, F♯0, G0, A0, B0, C1, D♯1, E1
    /// MIDI note numbers: 16, 18, 19, 21, 23, 24, 27, 28
    /// Frequency range: ~20.60 Hz to ~41.20 Hz
    ///
    /// The E harmonic minor scale in octave 0 spans frequencies from the threshold of human pitch
    /// perception (~20 Hz) into the low bass range. The scale's lower notes are experienced primarily
    /// as rhythmic vibrations, while the upper notes (particularly the raised 7th degree D♯1) begin
    /// to take on more distinct pitch qualities. The augmented second interval (C1 to D♯1) becomes
    /// marginally perceptible in this register. This scale is used in electronic music contexts
    /// for sub-bass design and is occasionally employed in experimental composition exploring the
    /// threshold between rhythm and pitch.
    pub static ref E0_HARMONIC_SCALE: Scale<8> = harmonic_scale(E0);

    /// F harmonic minor scale in octave 0.
    ///
    /// Notes: F0, G0, A♭0, B♭0, C1, D♭1, E1, F1
    /// MIDI note numbers: 17, 19, 20, 22, 24, 25, 28, 29
    /// Frequency range: ~21.83 Hz to ~43.65 Hz
    ///
    /// This F harmonic minor scale begins just above the threshold of pitch perception and
    /// extends into the low bass register. The extreme depth of these frequencies requires
    /// specialized sound reproduction equipment to be accurately represented. The augmented
    /// second interval (D♭1 to E1) remains difficult to perceive as a melodic feature at these
    /// depths. This scale is primarily used in electronic music production for sub-bass design
    /// and in specialized sound installations exploring the physical properties of extremely
    /// low frequencies.
    pub static ref F0_HARMONIC_SCALE: Scale<8> = harmonic_scale(F0);

    /// F♯ harmonic minor scale in octave 0.
    ///
    /// Notes: F♯0, G♯0, A0, B0, C♯1, D1, E♯1, F♯1
    /// MIDI note numbers: 18, 20, 21, 23, 25, 26, 29, 30
    /// Frequency range: ~23.12 Hz to ~46.25 Hz
    ///
    /// The F♯ harmonic minor scale in octave 0 spans from barely perceptible low frequencies
    /// into the low bass register. The lower pitches function as felt vibrations rather than
    /// clear tones, while the upper notes of the scale (particularly E♯1 and F♯1) begin to
    /// take on more distinct musical qualities. This register is exclusively used in electronic
    /// music contexts where specialized equipment can reproduce such extreme low frequencies.
    /// The characteristic augmented second (D1 to E♯1) remains largely theoretical rather than
    /// perceptible at these depths.
    pub static ref FSHARP0_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP0);

    /// G harmonic minor scale in octave 0.
    ///
    /// Notes: G0, A0, B♭0, C1, D1, E♭1, F♯1, G1
    /// MIDI note numbers: 19, 21, 22, 24, 26, 27, 30, 31
    /// Frequency range: ~24.50 Hz to ~49.00 Hz
    ///
    /// The G harmonic minor scale in octave 0 spans frequencies that transition from sub-bass
    /// vibrations to more clearly defined low pitches. The raised 7th degree (F♯1) is the first
    /// note to exhibit clearer tonal qualities. At this register, the characteristic augmented
    /// second interval (E♭1 to F♯1) begins to become slightly more perceptible as a melodic feature,
    /// though it remains challenging to distinguish. This scale is utilized in electronic music for
    /// foundational sub-bass tones and in experimental compositions that explore the boundary
    /// between felt vibration and heard pitch.
    pub static ref G0_HARMONIC_SCALE: Scale<8> = harmonic_scale(G0);

    /// G♯ harmonic minor scale in octave 0.
    ///
    /// Notes: G♯0, A♯0, B0, C♯1, D♯1, E1, F♯♯1, G♯1
    /// MIDI note numbers: 20, 22, 23, 25, 27, 28, 31, 32
    /// Frequency range: ~25.96 Hz to ~51.91 Hz
    ///
    /// This G♯ harmonic minor scale in octave 0 spans frequencies that transition from the
    /// threshold of pitch perception into the low bass register. The theoretical double-sharp (F♯♯1)
    /// would be enharmonically notated as G1 in practice. This scale's upper notes begin to take
    /// on more distinct pitch qualities, though the lower notes remain primarily felt rather than
    /// heard. This extremely low register is exclusively utilized in electronic music contexts,
    /// particularly for specialized sub-bass design and textural elements that exploit the physical
    /// properties of very low frequencies.
    pub static ref GSHARP0_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP0);

    /// A harmonic minor scale in octave 0.
    ///
    /// Notes: A0, B0, C1, D1, E1, F1, G♯1, A1
    /// MIDI note numbers: 21, 23, 24, 26, 28, 29, 32, 33
    /// Frequency range: ~27.50 Hz to ~55.00 Hz
    ///
    /// The A harmonic minor scale in octave 0 begins at the lowest A on a standard piano (A0)
    /// and spans into the low bass register. This scale represents a transition point where the
    /// lower notes function primarily as felt vibrations, while the upper notes (particularly
    /// the raised 7th degree G♯1) begin to exhibit clearer tonal qualities. The characteristic
    /// augmented second interval (F1 to G♯1) becomes somewhat more perceptible at this register.
    /// This scale is used in electronic music for foundational bass tones and occasionally in
    /// specialized piano compositions that explore the extreme low range of the instrument.
    pub static ref A0_HARMONIC_SCALE: Scale<8> = harmonic_scale(A0);

    /// A♯ harmonic minor scale in octave 0.
    ///
    /// Notes: A♯0, B♯0, C♯1, D♯1, E♯1, F♯1, G♯♯1, A♯1
    /// MIDI note numbers: 22, 24, 25, 27, 29, 30, 33, 34
    /// Frequency range: ~29.14 Hz to ~58.27 Hz
    ///
    /// The A♯ harmonic minor scale in octave 0 spans from extremely low frequencies into the more
    /// clearly defined bass register. The highly theoretical B♯0 would be enharmonically represented
    /// as C1 in practice, and the double-sharp (G♯♯1) as A1. At these frequencies, the scale begins
    /// to take on more distinct musical qualities, especially in its upper register. This scale is
    /// primarily utilized in electronic music contexts for specialized bass design and occasionally
    /// in theoretical explorations of harmony at extreme frequency ranges.
    pub static ref ASHARP0_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP0);

    /// B harmonic minor scale in octave 0.
    ///
    /// Notes: B0, C♯1, D1, E1, F♯1, G1, A♯1, B1
    /// MIDI note numbers: 23, 25, 26, 28, 30, 31, 34, 35
    /// Frequency range: ~30.87 Hz to ~61.74 Hz
    ///
    /// The B harmonic minor scale in octave 0 spans from the lowest B on a standard piano into
    /// the bass register. At these frequencies, the notes begin to exhibit more clearly defined
    /// pitch qualities, particularly in the upper half of the scale. The characteristic augmented
    /// second interval between G1 and A♯1) becomes more perceptible as a melodic feature. This scale marks
    /// a transition point between sub-bass frequencies and the more musically functional low bass
    /// register. It appears in electronic music contexts and occasionally in contemporary classical
    /// compositions that explore extreme registers.
    pub static ref B0_HARMONIC_SCALE: Scale<8> = harmonic_scale(B0);
}

lazy_static! {
    /// C harmonic minor scale in octave 1.
    ///
    /// Notes: C1, D1, E♭1, F1, G1, A♭1, B1, C2
    /// MIDI note numbers: 24, 26, 27, 29, 31, 32, 35, 36
    /// Frequency range: ~32.70 Hz to ~65.41 Hz
    ///
    /// The C harmonic minor scale in octave 1 occupies the deep bass register, spanning from
    /// the lowest C on many pianos into the more clearly audible bass range. At these frequencies,
    /// the notes begin to take on more distinct pitch characteristics, though they remain primarily
    /// in the domain of foundational bass tones. The augmented second interval between A♭1 and B1
    /// becomes more perceptible as a melodic feature. This scale is used in contrabass, pipe organ
    /// pedal passages, bass synthesizers, and the lowest range of the piano, particularly in
    /// dramatic classical works that exploit the visceral depth of this register.
    pub static ref C1_HARMONIC_SCALE: Scale<8> = harmonic_scale(C1);

    /// C♯ harmonic minor scale in octave 1.
    ///
    /// Notes: C♯1, D♯1, E1, F♯1, G♯1, A1, B♯1, C♯2
    /// MIDI note numbers: 25, 27, 28, 30, 32, 33, 36, 37
    /// Frequency range: ~34.65 Hz to ~69.30 Hz
    ///
    /// The C♯ harmonic minor scale in octave 1 spans the deep bass register. At these frequencies,
    /// the notes create strong fundamental bass tones with a distinct pitch quality, though harmonic
    /// complexity is minimized in this range. The augmented second interval between A1 and B♯1
    /// creates a perceptible melodic tension. This scale is used in orchestral bass passages,
    /// pipe organ pedal lines, and bass synthesizer parts in electronic and film music, where it
    /// provides both foundational support and a hint of the harmonic minor's exotic character.
    pub static ref CSHARP1_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP1);

    /// D harmonic minor scale in octave 1.
    ///
    /// Notes: D1, E1, F1, G1, A1, B♭1, C♯2, D2
    /// MIDI note numbers: 26, 28, 29, 31, 33, 34, 37, 38
    /// Frequency range: ~36.71 Hz to ~73.42 Hz
    ///
    /// The D harmonic minor scale in octave 1 spans frequencies that form the foundation of
    /// orchestral and contemporary bass sections. In this register, the notes produce strong
    /// fundamental tones with increasing pitch clarity. The characteristic augmented second interval
    /// between B♭1 and C♯2 becomes clearly perceptible. This scale is employed in contrabass
    /// passages, pipe organ pedal parts, bass guitar's lowest range, and synthesizer bass lines in
    /// genres from classical to electronic music, providing both foundational depth and harmonic
    /// interest through its distinctive raised leading tone.
    pub static ref D1_HARMONIC_SCALE: Scale<8> = harmonic_scale(D1);

    /// D♯ harmonic minor scale in octave 1.
    ///
    /// Notes: D♯1, E♯1, F♯1, G♯1, A♯1, B1, C♯♯2, D♯2
    /// MIDI note numbers: 27, 29, 30, 32, 34, 35, 38, 39
    /// Frequency range: ~38.89 Hz to ~77.78 Hz
    ///
    /// The D♯ harmonic minor scale in octave 1 occupies the deep bass register. At these
    /// frequencies, the notes establish fundamental bass tones with increasing pitch definition.
    /// The theoretical double-sharp (C♯♯2) would be enharmonically notated as D2 in practice.
    /// This scale is used in contrabass, bass synthesizer, and pipe organ pedal parts, particularly
    /// in contemporary classical and film music contexts where its exotic intervals create a
    /// distinctive tension while maintaining foundational support.
    pub static ref DSHARP1_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP1);

    /// E harmonic minor scale in octave 1.
    ///
    /// Notes: E1, F♯1, G1, A1, B1, C2, D♯2, E2
    /// MIDI note numbers: 28, 30, 31, 33, 35, 36, 39, 40
    /// Frequency range: ~41.20 Hz to ~82.41 Hz
    ///
    /// The E harmonic minor scale in octave 1 spans the deep bass register, moving from
    /// fundamental bass tones to clearer pitch centers. The distinctive augmented second
    /// interval between C2 and D♯2 becomes prominently perceptible at these frequencies.
    /// This scale is utilized in contrabass solos, pipe organ pedal passages, bass guitar's
    /// lowest range, and synthesizer bass lines in classical, film, and electronic music.
    /// The raised leading tone (D♯2) creates a strong harmonic pull that's particularly
    /// effective in this register for establishing a minor tonality with dramatic tension.
    pub static ref E1_HARMONIC_SCALE: Scale<8> = harmonic_scale(E1);

    /// F harmonic minor scale in octave 1.
    ///
    /// Notes: F1, G1, A♭1, B♭1, C2, D♭2, E2, F2
    /// MIDI note numbers: 29, 31, 32, 34, 36, 37, 40, 41
    /// Frequency range: ~43.65 Hz to ~87.31 Hz
    ///
    /// The F harmonic minor scale in octave 1 occupies the deep bass register, providing strong
    /// foundational tones with increasingly distinct pitch qualities. The characteristic augmented
    /// second interval between D♭2 and E2 creates a clearly perceptible tension. This scale is
    /// employed in orchestral bass sections, organ pedal parts, and bass synthesizer lines in
    /// classical, film, and electronic music. In this register, the scale's distinctive harmonic
    /// minor character begins to emerge clearly while still providing substantial low-end support.
    pub static ref F1_HARMONIC_SCALE: Scale<8> = harmonic_scale(F1);

    /// F♯ harmonic minor scale in octave 1.
    ///
    /// Notes: F♯1, G♯1, A1, B1, C♯2, D2, E♯2, F♯2
    /// MIDI note numbers: 30, 32, 33, 35, 37, 38, 41, 42
    /// Frequency range: ~46.25 Hz to ~92.50 Hz
    ///
    /// The F♯ harmonic minor scale in octave 1 spans the primary bass register, providing both
    /// harmonic foundation and melodic clarity. At these frequencies, the notes exhibit full
    /// pitch definition with substantial body. The characteristic augmented second interval
    /// between D2 and E♯2 creates a pronounced exotic quality. This scale is utilized in bass
    /// guitar, cello, and bassoon parts, as well as piano left-hand passages. The F♯ minor
    /// tonality has historical significance in classical works by Bach, Haydn, and Beethoven,
    /// where its distinctive harmonic minor character creates a particularly intense emotional
    /// expression.
    pub static ref FSHARP1_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP1);

    /// G harmonic minor scale in octave 1.
    ///
    /// Notes: G1, A1, B♭1, C2, D2, E♭2, F♯2, G2
    /// MIDI note numbers: 31, 33, 34, 36, 38, 39, 42, 43
    /// Frequency range: ~49.00 Hz to ~98.00 Hz
    ///
    /// The G harmonic minor scale in octave 1 spans the deep bass register, with increasingly
    /// clear pitch definition as it ascends. The distinctive augmented second interval between
    /// E♭2 and F♯2 creates a prominent tension that's clearly perceptible. This scale is employed
    /// in contrabass parts, pipe organ pedals, bass guitar, and synthesizer bass lines in
    /// classical, jazz, and film music. At these frequencies, the scale's harmonic minor
    /// characteristics are distinctly audible while still providing powerful foundational support.
    pub static ref G1_HARMONIC_SCALE: Scale<8> = harmonic_scale(G1);

    /// G♯ harmonic minor scale in octave 1.
    ///
    /// Notes: G♯1, A♯1, B1, C♯2, D♯2, E2, F♯♯2, G♯2
    /// MIDI note numbers: 32, 34, 35, 37, 39, 40, 43, 44
    /// Frequency range: ~51.91 Hz to ~103.83 Hz
    ///
    /// The G♯ harmonic minor scale in octave 1 occupies the deep bass register with increasingly
    /// defined pitch qualities. The theoretical double-sharp (F♯♯2) would be enharmonically notated
    /// as G2 in practice. This scale is used in orchestral bass sections, pipe organ pedals, and
    /// bass synthesizer parts in classical, film, and electronic music. At these frequencies, the
    /// exotic character of the harmonic minor becomes quite apparent while maintaining the powerful
    /// presence characteristic of the low bass register.
    pub static ref GSHARP1_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP1);

    /// A harmonic minor scale in octave 1.
    ///
    /// Notes: A1, B1, C2, D2, E2, F2, G♯2, A2
    /// MIDI note numbers: 33, 35, 36, 38, 40, 41, 44, 45
    /// Frequency range: ~55.00 Hz to ~110.00 Hz
    ///
    /// The A harmonic minor scale in octave 1 spans from the deep bass register into the more
    /// clearly defined bass range. At these frequencies, the scale's notes offer both foundational
    /// support and distinct pitch qualities. The characteristic augmented second interval between
    /// F2 and G♯2 creates a prominent exotic flavor that's clearly perceptible. This scale is
    /// employed in orchestral bass sections, low piano passages, organ pedals, and synthesizer
    /// bass lines in classical, film, and popular music, where its distinctive harmonic minor
    /// character is fully realized while maintaining substantial low-end presence.
    pub static ref A1_HARMONIC_SCALE: Scale<8> = harmonic_scale(A1);

    /// A♯ harmonic minor scale in octave 1.
    ///
    /// Notes: A♯1, B♯1, C♯2, D♯2, E♯2, F♯2, G♯♯2, A♯2
    /// MIDI note numbers: 34, 36, 37, 39, 41, 42, 45, 46
    /// Frequency range: ~58.27 Hz to ~116.54 Hz
    ///
    /// The A♯ harmonic minor scale in octave 1 spans the low bass register with clear pitch
    /// definition. The theoretical B♯1 would be enharmonically represented as C2 in practice,
    /// and the double-sharp (G♯♯2) as A2. This scale is used in orchestral bass parts, organ
    /// pedals, and bass synthesizer lines in classical, film, and electronic music. At these
    /// frequencies, the exotic character of the harmonic minor becomes prominently audible
    /// while maintaining the substantial presence of the bass register.
    pub static ref ASHARP1_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP1);

    /// B harmonic minor scale in octave 1.
    ///
    /// Notes: B1, C♯2, D2, E2, F♯2, G2, A♯2, B2
    /// MIDI note numbers: 35, 37, 38, 40, 42, 43, 46, 47
    /// Frequency range: ~61.74 Hz to ~123.47 Hz
    ///
    /// The B harmonic minor scale in octave 1 spans from the deep bass register into the more
    /// clearly defined bass range. At these frequencies, the scale's notes provide both substantial
    /// low-end presence and distinct pitch characteristics. The augmented second interval between
    /// G2 and A♯2 creates a prominent tension that's clearly perceptible. This scale is employed
    /// in orchestral bass sections, organ pedals, piano bass lines, and synthesizer parts in
    /// classical, film, and electronic music contexts. The exotic character of the harmonic minor
    /// is fully realized while maintaining the powerful impact of the low register.
    pub static ref B1_HARMONIC_SCALE: Scale<8> = harmonic_scale(B1);
}

lazy_static! {
    /// C harmonic minor scale in octave 2.
    ///
    /// Notes: C2, D2, E♭2, F2, G2, A♭2, B2, C3
    /// MIDI note numbers: 36, 38, 39, 41, 43, 44, 47, 48
    /// Frequency range: ~65.41 Hz to ~130.81 Hz
    ///
    /// The C harmonic minor scale in octave 2 occupies the functional low bass register,
    /// beginning with the lowest C on many bass instruments. At these frequencies, the
    /// scale's notes have distinct pitch qualities while retaining substantial low-end power.
    /// The augmented second interval between A♭2 and B2 creates a clear and characteristic
    /// tension. This scale is employed extensively in bass guitar, cello, bassoon, and tuba
    /// parts, as well as the lower range of the piano. It provides a rich harmonic foundation
    /// in orchestral, jazz, and soundtrack compositions, with the raised leading tone creating
    /// a strong harmonic pull that defines the minor tonality.
    pub static ref C2_HARMONIC_SCALE: Scale<8> = harmonic_scale(C2);

    /// C♯ harmonic minor scale in octave 2.
    ///
    /// Notes: C♯2, D♯2, E2, F♯2, G♯2, A2, B♯2, C♯3
    /// MIDI note numbers: 37, 39, 40, 42, 44, 45, 48, 49
    /// Frequency range: ~69.30 Hz to ~138.59 Hz
    ///
    /// The C♯ harmonic minor scale in octave 2 spans the low bass register that forms the
    /// foundation of many compositions. At these frequencies, the notes offer clear pitch
    /// definition while maintaining substantial presence and weight. The augmented second
    /// interval between A2 and B♯2 creates a distinctive exotic quality that's fully perceptible.
    /// This scale is utilized in bass guitar, cello, double bass, and tuba parts, as well as
    /// piano left-hand passages. It's effective for creating dramatic bass lines in classical,
    /// film, and jazz contexts, with the exotic harmonic minor character clearly expressed.
    pub static ref CSHARP2_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP2);

    /// D harmonic minor scale in octave 2.
    ///
    /// Notes: D2, E2, F2, G2, A2, B♭2, C♯3, D3
    /// MIDI note numbers: 38, 40, 41, 43, 45, 46, 49, 50
    /// Frequency range: ~73.42 Hz to ~146.83 Hz
    ///
    /// The D harmonic minor scale in octave 2 occupies the bass register that provides the
    /// harmonic foundation for most musical genres. At these frequencies, the notes exhibit
    /// clear pitch definition while retaining substantial low-end presence. The augmented second
    /// interval between B♭2 and C♯3 creates a pronounced exotic quality. This scale is employed
    /// in bass guitar, cello, bassoon, and trombone parts, as well as left-hand piano passages.
    /// It serves as an effective foundation for baroque, classical, and film music compositions,
    /// with the raised leading tone providing strong harmonic direction.
    pub static ref D2_HARMONIC_SCALE: Scale<8> = harmonic_scale(D2);

    /// D♯ harmonic minor scale in octave 2.
    ///
    /// Notes: D♯2, E♯2, F♯2, G♯2, A♯2, B2, C♯♯3, D♯3
    /// MIDI note numbers: 39, 41, 42, 44, 46, 47, 50, 51
    /// Frequency range: ~77.78 Hz to ~155.56 Hz
    ///
    /// The D♯ harmonic minor scale in octave 2 spans the functionally rich bass register. At these
    /// frequencies, the notes provide both clear pitch definition and substantial low-end support.
    /// The theoretical double-sharp (C♯♯3) would be enharmonically written as D3 in practice.
    /// This scale appears in bass guitar, cello, bassoon, and tuba parts, most often rewritten as
    /// E♭ harmonic minor for practical notation. It's effective for creating dramatic bass lines and
    /// harmonic foundations in film, classical, and jazz contexts, with the distinctive augmented
    /// second interval clearly articulating the scale's exotic character.
    pub static ref DSHARP2_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP2);

    /// E harmonic minor scale in octave 2.
    ///
    /// Notes: E2, F♯2, G2, A2, B2, C3, D♯3, E3
    /// MIDI note numbers: 40, 42, 43, 45, 47, 48, 51, 52
    /// Frequency range: ~82.41 Hz to ~164.81 Hz
    ///
    /// The E harmonic minor scale in octave 2 spans the primary bass register, a crucial range
    /// for providing harmonic foundations. At these frequencies, notes have clear pitch identity
    /// while maintaining substantial body and presence. The characteristic augmented second
    /// interval between C3 and D♯3 creates a pronounced exotic quality. This scale is extensively
    /// used in bass guitar (including the lowest string in standard tuning), cello, bassoon, and
    /// trombone parts, as well as left-hand piano passages. It's particularly effective in guitar-based
    /// compositions, flamenco music, and classical works requiring a dramatic minor tonality.
    pub static ref E2_HARMONIC_SCALE: Scale<8> = harmonic_scale(E2);

    /// F harmonic minor scale in octave 2.
    ///
    /// Notes: F2, G2, A♭2, B♭2, C3, D♭3, E3, F3
    /// MIDI note numbers: 41, 43, 44, 46, 48, 49, 52, 53
    /// Frequency range: ~87.31 Hz to ~174.61 Hz
    ///
    /// The F harmonic minor scale in octave 2 spans the functional bass register that provides
    /// harmonic underpinning for most compositions. At these frequencies, the notes exhibit
    /// clear pitch definition with substantial weight and presence. The augmented second interval
    /// between D♭3 and E3 creates a distinctive and fully perceptible tension. This scale is
    /// employed in bass guitar, cello, bassoon, and tuba parts, as well as piano left-hand
    /// passages in classical, film, and jazz contexts. The scale's dark, exotic character is fully
    /// expressed in this register, making it effective for creating dramatic or mysterious moods.
    pub static ref F2_HARMONIC_SCALE: Scale<8> = harmonic_scale(F2);

    /// F♯ harmonic minor scale in octave 2.
    ///
    /// Notes: F♯2, G♯2, A2, B2, C♯3, D3, E♯3, F♯3
    /// MIDI note numbers: 42, 44, 45, 47, 49, 50, 53, 54
    /// Frequency range: ~92.50 Hz to ~185.00 Hz
    ///
    /// The F♯ harmonic minor scale in octave 2 spans the primary bass register, providing both
    /// harmonic foundation and melodic clarity. At these frequencies, the notes exhibit full
    /// pitch definition with substantial body. The characteristic augmented second interval
    /// between D3 and E♯3 creates a pronounced exotic quality. This scale is utilized in bass
    /// guitar, cello, and bassoon parts, as well as piano left-hand passages. The F♯ minor
    /// tonality has historical significance in classical works by Bach, Haydn, and Beethoven,
    /// where its distinctive harmonic minor character creates a particularly intense emotional
    /// expression.
    pub static ref FSHARP2_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP2);

    /// G harmonic minor scale in octave 2.
    ///
    /// Notes: G2, A2, B♭2, C3, D3, E♭3, F♯3, G3
    /// MIDI note numbers: 43, 45, 46, 48, 50, 51, 54, 55
    /// Frequency range: ~98.00 Hz to ~196.00 Hz
    ///
    /// The G harmonic minor scale in octave 2 occupies the standard bass register, a crucial
    /// foundation for most musical compositions. At these frequencies, the notes exhibit full
    /// pitch clarity with substantial presence. The augmented second interval between E♭3 and
    /// F♯3 creates a distinctive exotic character that's prominently perceptible. This scale is
    /// extensively employed in bass guitar, cello, bassoon, and trombone parts, as well as
    /// piano left-hand passages. G minor is a historically significant key in classical music,
    /// with Bach, Mozart, and Tchaikovsky featuring works where the distinctive raised leading
    /// tone creates powerful emotional tension.
    pub static ref G2_HARMONIC_SCALE: Scale<8> = harmonic_scale(G2);

    /// G♯ harmonic minor scale in octave 2.
    ///
    /// Notes: G♯2, A♯2, B2, C♯3, D♯3, E3, F♯♯3, G♯3
    /// MIDI note numbers: 44, 46, 47, 49, 51, 52, 55, 56
    /// Frequency range: ~103.83 Hz to ~207.65 Hz
    ///
    /// The G♯ harmonic minor scale in octave 2 spans the functional bass register that bridges
    /// deep bass and baritone ranges. At these frequencies, the notes exhibit full pitch definition
    /// with substantial body. The theoretical double-sharp (F♯♯3) would be enharmonically notated as
    /// G3 in practice. This scale's upper notes begin to take
    /// on more distinct pitch qualities, though the lower notes remain primarily felt rather than
    /// heard. This extremely low register is exclusively utilized in electronic music contexts,
    /// particularly for specialized sub-bass design and textural elements that exploit the physical
    /// properties of very low frequencies.
    pub static ref GSHARP2_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP2);

    /// A harmonic minor scale in octave 2.
    ///
    /// Notes: A2, B2, C3, D3, E3, F3, G♯3, A3
    /// MIDI note numbers: 45, 47, 48, 50, 52, 53, 56, 57
    /// Frequency range: ~110.00 Hz to ~220.00 Hz
    ///
    /// The A harmonic minor scale in octave 2 spans the standard bass register, reaching into the
    /// baritone range at its upper end. At these frequencies, the notes exhibit full pitch clarity
    /// and substantial presence. The characteristic augmented second interval between F3 and G♯3
    /// creates a pronounced exotic quality. This scale is extensively employed in bass guitar,
    /// cello, bassoon, and trombone parts, as well as piano compositions. A harmonic minor is
    /// perhaps the most widely used harmonic minor scale in classical music, featured prominently
    /// in works by Mozart, Bach, and countless others, with its distinctive exotic character and
    /// dramatic leading tone fully realized in this register.
    pub static ref A2_HARMONIC_SCALE: Scale<8> = harmonic_scale(A2);

    /// A♯ harmonic minor scale in octave 2.
    ///
    /// Notes: A♯2, B♯2, C♯3, D♯3, E♯3, F♯3, G♯♯3, A♯3
    /// MIDI note numbers: 46, 48, 49, 51, 53, 54, 57, 58
    /// Frequency range: ~116.54 Hz to ~233.08 Hz
    ///
    /// The A♯ harmonic minor scale in octave 2 spans the bass register, extending into the
    /// baritone range. The theoretical B♯2 would be enharmonically written as C3 in practice,
    /// and the double-sharp (G♯♯3) as A3. Due to its complex notation, this scale is typically
    /// rewritten as B♭ harmonic minor. At these frequencies, the notes exhibit full pitch clarity
    /// with substantial body. This scale appears in bass guitar, cello, and bassoon parts, as well
    /// as piano compositions in classical, film, and jazz contexts. Its exotic harmonic minor
    /// character is fully expressed, creating a distinctive dramatic or mysterious atmosphere.
    pub static ref ASHARP2_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP2);

    /// B harmonic minor scale in octave 2.
    ///
    /// Notes: B2, C♯3, D3, E3, F♯3, G3, A♯3, B3
    /// MIDI note numbers: 47, 49, 50, 52, 54, 55, 58, 59
    /// Frequency range: ~123.47 Hz to ~246.94 Hz
    ///
    /// The B harmonic minor scale in octave 2 spans from the bass register into the more
    /// clearly defined bass range. At these frequencies, the scale's notes provide both substantial
    /// low-end presence and distinct pitch characteristics. The augmented second interval between
    /// G2 and A♯2 creates a prominent tension that's clearly perceptible. This scale is employed
    /// in orchestral bass sections, organ pedals, piano bass lines, and synthesizer parts in
    /// classical, film, and electronic music contexts. The exotic character of the harmonic minor
    /// is fully realized while maintaining the powerful impact of the low register.
    pub static ref B2_HARMONIC_SCALE: Scale<8> = harmonic_scale(B2);
}

lazy_static! {
    /// C harmonic minor scale in octave 3.
    ///
    /// Notes: C3, D3, E♭3, F3, G3, A♭3, B3, C4
    /// MIDI note numbers: 48, 50, 51, 53, 55, 56, 59, 60
    /// Frequency range: ~130.81 Hz to ~261.63 Hz
    ///
    /// The C harmonic minor scale in octave 3 spans the lower-middle register that forms a
    /// crucial bridge between bass and tenor ranges. This register (with C4 at its upper end)
    /// is central to cello repertoire, baritone voice, tenor trombone, bassoon, and left-hand
    /// piano passages. The augmented second interval between A♭3 and B3 creates a prominent
    /// exotic quality that's fully expressed in this comfortable register. This scale is
    /// extensively used in baritone vocal solos, cello repertoire, and as harmonic material
    /// in orchestral writing. The scale's dark, mysterious character combined with the clear
    /// pitch definition of this register makes it particularly effective for expressive
    /// melodic content.
    pub static ref C3_HARMONIC_SCALE: Scale<8> = harmonic_scale(C3);

    /// C♯ harmonic minor scale in octave 3.
    ///
    /// Notes: C♯3, D♯3, E3, F♯3, G♯3, A3, B♯3, C♯4
    /// MIDI note numbers: 49, 51, 52, 54, 56, 57, 60, 61
    /// Frequency range: ~138.59 Hz to ~277.18 Hz
    ///
    /// The C♯ harmonic minor scale in octave 3 occupies the rich lower-middle register that
    /// serves as a melodic center for many instruments and voice types. At these frequencies,
    /// the notes provide a perfect balance of warmth and clarity. The augmented second interval
    /// between A3 and B♯3 creates a distinctive exotic quality that's fully realized. This scale
    /// is employed in cello, viola, and tenor trombone parts, as well as baritone vocals and
    /// piano compositions. Its dramatic character is particularly effective in Romantic and
    /// contemporary classical works, where the scale's exotic intervals create expressive
    /// tension and resolution.
    pub static ref CSHARP3_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP3);

    /// D harmonic minor scale in octave 3.
    ///
    /// Notes: D3, E3, F3, G3, A3, B♭3, C♯4, D4
    /// MIDI note numbers: 50, 52, 53, 55, 57, 58, 61, 62
    /// Frequency range: ~146.83 Hz to ~293.66 Hz
    ///
    /// The D harmonic minor scale in octave 3 spans the crucial lower-middle register that
    /// bridges baritone and tenor ranges. At these frequencies, the scale exhibits excellent
    /// clarity and warmth, making it ideal for expressive melodic content. The augmented second
    /// interval between B♭3 and C♯4 creates a characteristic tension that's particularly effective
    /// in this register. This scale is extensively utilized in viola, cello, tenor trombone,
    /// and baritone vocal parts, as well as piano compositions. D minor is a historically
    /// significant key in the Baroque era, with Bach's D minor works often employing the
    /// harmonic minor's distinctive raised leading tone for dramatic effect.
    pub static ref D3_HARMONIC_SCALE: Scale<8> = harmonic_scale(D3);

    /// D♯ harmonic minor scale in octave 3.
    ///
    /// Notes: D♯3, E♯3, F♯3, G♯3, A♯3, B3, C♯♯4, D♯4
    /// MIDI note numbers: 51, 53, 54, 56, 58, 59, 62, 63
    /// Frequency range: ~155.56 Hz to ~311.13 Hz
    ///
    /// The D♯ harmonic minor scale in octave 3 occupies the expressive lower-middle register.
    /// At these frequencies, the scale offers an ideal balance of definition and warmth.
    /// The theoretical double-sharp (C♯♯4) would be enharmonically notated as D4 in practice,
    /// and the scale is typically rewritten as E♭ minor for practical reasons. This scale
    /// appears in viola, cello, tenor saxophone, and trombone parts, as well as baritone
    /// vocals and piano compositions. The exotic quality of the harmonic minor is particularly
    /// effective in this register for creating emotional depth and color in classical,
    /// jazz, and film music contexts.
    pub static ref DSHARP3_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP3);

    /// E harmonic minor scale in octave 3.
    ///
    /// Notes: E3, F♯3, G3, A3, B3, C4, D♯4, E4
    /// MIDI note numbers: 52, 54, 55, 57, 59, 60, 63, 64
    /// Frequency range: ~164.81 Hz to ~329.63 Hz
    ///
    /// The E harmonic minor scale in octave 3 spans the pivotal lower-middle register, with its
    /// upper notes reaching into the central tenor range. At these frequencies, the scale
    /// exhibits excellent clarity and expressiveness. The augmented second interval between C4
    /// (Middle C) and D♯4 creates a particularly striking effect in this register. This scale
    /// is extensively used in viola, cello, guitar, and tenor vocal repertoire, as well as
    /// piano compositions. E minor holds special significance in classical guitar literature
    /// and Spanish-influenced music, where its harmonic minor form creates the characteristic
    /// Phrygian dominant sound that defines flamenco and related styles.
    pub static ref E3_HARMONIC_SCALE: Scale<8> = harmonic_scale(E3);

    /// F harmonic minor scale in octave 3.
    ///
    /// Notes: F3, G3, A♭3, B♭3, C4, D♭4, E4, F4
    /// MIDI note numbers: 53, 55, 56, 58, 60, 61, 64, 65
    /// Frequency range: ~174.61 Hz to ~349.23 Hz
    ///
    /// The F harmonic minor scale in octave 3 spans from the lower-middle register into the
    /// central tenor range, encompassing Middle C (C4) within its compass. At these frequencies,
    /// the scale provides excellent balance between warmth and clarity. The augmented second
    /// interval between D♭4 and E4 creates a distinctive exotic quality that's fully expressed.
    /// This scale is employed in viola, cello, tenor vocal, and tenor saxophone parts, as well as
    /// piano compositions. The scale's dark, mysterious character combined with the resonant
    /// qualities of this register makes it particularly effective for expressive melodic material
    /// in classical, film, and jazz contexts.
    pub static ref F3_HARMONIC_SCALE: Scale<8> = harmonic_scale(F3);

    /// F♯ harmonic minor scale in octave 3.
    ///
    /// Notes: F♯3, G♯3, A3, B3, C♯4, D4, E♯4, F♯4
    /// MIDI note numbers: 54, 56, 57, 59, 61, 62, 65, 66
    /// Frequency range: ~185.00 Hz to ~369.99 Hz
    ///
    /// The F♯ harmonic minor scale in octave 3 spans from the lower-middle to the central
    /// tenor register, with its upper notes reaching well into the comfortable singing range
    /// for tenor voices. At these frequencies, the scale offers excellent clarity and expressive
    /// potential. The augmented second interval between D4 and E♯4 creates a prominent exotic
    /// quality that's fully realized. This scale is utilized in viola, cello, tenor vocal,
    /// and clarinet parts, as well as piano compositions. The F♯ minor tonality appears
    /// frequently in Romantic music, where its harmonic minor form creates a particularly
    /// passionate and melancholic character.
    pub static ref FSHARP3_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP3);

    /// G harmonic minor scale in octave 3.
    ///
    /// Notes: G3, A3, B♭3, C4, D4, E♭4, F♯4, G4
    /// MIDI note numbers: 55, 57, 58, 60, 62, 63, 66, 67
    /// Frequency range: ~196.00 Hz to ~392.00 Hz
    ///
    /// The G harmonic minor scale in octave 3 spans from the lower-middle to the central tenor
    /// register, encompassing Middle C (C4) within its range. At these frequencies, the scale
    /// exhibits excellent clarity and expressiveness. The augmented second interval between
    /// E♭4 and F♯4 creates a distinctive exotic quality that's prominently featured. This scale
    /// is extensively employed in viola, violin, tenor vocal, and clarinet parts, as well as
    /// piano compositions. G minor is a historically significant key in Western music, with
    /// Mozart's G minor symphony exemplifying the emotional power of this key. In this register,
    /// the scale provides rich harmonic foundations and expressive melodic material.
    pub static ref G3_HARMONIC_SCALE: Scale<8> = harmonic_scale(G3);

    /// G♯ harmonic minor scale in octave 3.
    ///
    /// Notes: G♯3, A♯3, B3, C♯4, D♯4, E4, F♯♯4, G♯4
    /// MIDI note numbers: 56, 58, 59, 61, 63, 64, 67, 68
    /// Frequency range: ~207.65 Hz to ~415.30 Hz
    ///
    /// The G♯ harmonic minor scale in octave 3 spans from the lower-middle to the central tenor
    /// register. At these frequencies, the scale offers a perfect balance of warmth and clarity.
    /// The theoretical double-sharp (F♯♯4) would be enharmonically written as G4 in practice,
    /// and the scale is typically rewritten as A♭ minor for simplicity. This scale appears in
    /// viola, violin, tenor vocal, and clarinet parts, as well as piano compositions. The exotic
    /// character of the harmonic minor is particularly effective in this register for creating
    /// emotional intensity in classical, jazz, and film music contexts.
    pub static ref GSHARP3_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP3);

    /// A harmonic minor scale in octave 3.
    ///
    /// Notes: A3, B3, C4, D4, E4, F4, G♯4, A4
    /// MIDI note numbers: 57, 59, 60, 62, 64, 65, 68, 69
    /// Frequency range: ~220.00 Hz to ~440.00 Hz
    ///
    /// The A harmonic minor scale in octave 3 spans from the tenor range to the central register,
    /// culminating at concert pitch A4 (440 Hz). At these frequencies, the scale exhibits excellent
    /// clarity and expressiveness. The augmented second interval between F4 and G♯4 creates a
    /// distinctive exotic quality that's prominently featured. This scale is extensively employed
    /// in violin, viola, tenor vocal, and woodwind parts, as well as piano compositions. A minor
    /// is perhaps the most prominently featured minor key in classical music, with countless works
    /// exploiting its characteristic sound. Its harmonic minor form, with the raised G♯, creates
    /// the distinctive "exotic" sound that Mozart employed in his famous "Rondo alla Turca."
    pub static ref A3_HARMONIC_SCALE: Scale<8> = harmonic_scale(A3);

    /// A♯ harmonic minor scale in octave 3.
    ///
    /// Notes: A♯3, B♯3, C♯4, D♯4, E♯4, F♯4, G♯♯4, A♯4
    /// MIDI note numbers: 58, 60, 61, 63, 65, 66, 69, 70
    /// Frequency range: ~233.08 Hz to ~466.16 Hz
    ///
    /// The A♯ harmonic minor scale in octave 3 spans from the lower-middle to the central
    /// register. At these frequencies, the scale offers excellent clarity and expressiveness.
    /// The theoretical B♯3 would be enharmonically written as C4 (Middle C) in practice,
    /// and the double-sharp (G♯♯4) as A4. Due to its complex notation, this scale is typically
    /// rewritten as B♭ minor for practical purposes. This scale appears in violin, viola,
    /// tenor vocal, and woodwind parts, as well as piano compositions. Its exotic harmonic
    /// minor character is particularly effective for creating emotional depth and color in
    /// classical, jazz, and film music contexts.
    pub static ref ASHARP3_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP3);

    /// B harmonic minor scale in octave 3.
    ///
    /// Notes: B3, C♯4, D4, E4, F♯4, G4, A♯4, B4
    /// MIDI note numbers: 59, 61, 62, 64, 66, 67, 70, 71
    /// Frequency range: ~246.94 Hz to ~493.88 Hz
    ///
    /// The B harmonic minor scale in octave 3 spans from the lower-middle register into the
    /// central range, with its upper notes reaching well into the comfortable soprano/violin range.
    /// At these frequencies, the scale exhibits excellent clarity and brilliance. The augmented
    /// second interval between G4 and A♯4 creates a distinctive exotic quality that's prominently
    /// featured. This scale is extensively utilized in violin, viola, tenor/soprano vocal, and
    /// woodwind parts, as well as piano compositions. B minor has historical significance in
    /// classical music, featured in works by Bach, Tchaikovsky, and Chopin. The harmonic minor's
    /// raised A♯ in this register creates an especially brilliant and penetrating leading tone.
    pub static ref B3_HARMONIC_SCALE: Scale<8> = harmonic_scale(B3);
}

lazy_static! {
    /// C harmonic minor scale in octave 4.
    ///
    /// Notes: C4, D4, E♭4, F4, G4, A♭4, B4, C5
    /// MIDI note numbers: 60, 62, 63, 65, 67, 68, 71, 72
    /// Frequency range: ~261.63 Hz to ~523.25 Hz
    ///
    /// The C harmonic minor scale in octave 4 spans the central register that forms the core of
    /// most instrumental and vocal writing. Beginning with Middle C (C4), this scale sits in the
    /// sweet spot of human hearing and musical expression. The augmented second interval between
    /// A♭4 and B4 creates a striking exotic quality that's brilliantly clear in this register.
    /// This scale is employed extensively in soprano vocal parts, violin, flute, oboe, and right-hand
    /// piano passages. C minor represents one of the most emotionally expressive keys in classical
    /// repertoire, with Beethoven's Pathétique Sonata and Fifth Symphony showcasing the power of
    /// this tonality. In this register, the harmonic minor's distinctive tension is perfectly
    /// balanced between clarity and warmth.
    pub static ref C4_HARMONIC_SCALE: Scale<8> = harmonic_scale(C4);

    /// C♯ harmonic minor scale in octave 4.
    ///
    /// Notes: C♯4, D♯4, E4, F♯4, G♯4, A4, B♯4, C♯5
    /// MIDI note numbers: 61, 63, 64, 66, 68, 69, 72, 73
    /// Frequency range: ~277.18 Hz to ~554.37 Hz
    ///
    /// The C♯ harmonic minor scale in octave 4 occupies the central register that's ideal for
    /// melodic writing. At these frequencies, the scale offers a perfect balance of clarity,
    /// presence, and warmth. The augmented second interval between A4 (concert pitch 440 Hz) and
    /// B♯4 creates a distinctive exotic quality that's brilliantly realized. This scale is utilized
    /// in soprano vocal parts, violin, flute, clarinet, and piano passages, where its complex
    /// character can be fully expressed. C♯ minor appears prominently in works by Chopin and
    /// Rachmaninoff, where its harmonic minor form contributes to particularly emotional and
    /// dramatic passages in the piano repertoire.
    pub static ref CSHARP4_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP4);

    /// D harmonic minor scale in octave 4.
    ///
    /// Notes: D4, E4, F4, G4, A4, B♭4, C♯5, D5
    /// MIDI note numbers: 62, 64, 65, 67, 69, 70, 73, 74
    /// Frequency range: ~293.66 Hz to ~587.33 Hz
    ///
    /// The D harmonic minor scale in octave 4 spans the central register of music, including
    /// concert pitch A4 (440 Hz) within its range. At these frequencies, the scale exhibits
    /// perfect clarity and expressiveness. The augmented second interval between B♭4 and C♯5
    /// creates a particularly brilliant tension in this register. This scale is extensively
    /// employed in soprano and mezzo-soprano vocal parts, violin, flute, oboe, and piano
    /// compositions. D minor has historical significance as a key of profound expression,
    /// from Bach's D minor Toccata and Fugue to Mozart's Requiem. In this register, the
    /// harmonic minor's exotic character is balanced with exceptional clarity, making it
    /// ideal for expressive melodic writing.
    pub static ref D4_HARMONIC_SCALE: Scale<8> = harmonic_scale(D4);

    /// D♯ harmonic minor scale in octave 4.
    ///
    /// Notes: D♯4, E♯4, F♯4, G♯4, A♯4, B4, C♯♯5, D♯5
    /// MIDI note numbers: 63, 65, 66, 68, 70, 71, 74, 75
    /// Frequency range: ~311.13 Hz to ~622.25 Hz
    ///
    /// The D♯ harmonic minor scale in octave 4 occupies the central register of music, offering
    /// exceptional clarity and presence. At these frequencies, the scale provides a brilliant
    /// platform for expressive melodic writing. The theoretical double-sharp (C♯♯5) would be
    /// enharmonically notated as D5 in practice, and the scale is typically rewritten as E♭
    /// harmonic minor. This scale appears in soprano vocal parts, violin, flute, clarinet, and
    /// piano compositions. Its exotic harmonic minor character is particularly effective in this
    /// register for creating emotional intensity in classical, jazz, and film music contexts,
    /// with the augmented second interval creating a striking and brilliant effect.
    pub static ref DSHARP4_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP4);

    /// E harmonic minor scale in octave 4.
    ///
    /// Notes: E4, F♯4, G4, A4, B4, C5, D♯5, E5
    /// MIDI note numbers: 64, 66, 67, 69, 71, 72, 75, 76
    /// Frequency range: ~329.63 Hz to ~659.26 Hz
    ///
    /// The E harmonic minor scale in octave 4 spans the brilliant central register of music,
    /// including concert pitch A4 (440 Hz) within its range. At these frequencies, the scale
    /// exhibits exceptional clarity and brightness. The augmented second interval between C5
    /// and D♯5 creates a particularly brilliant and exotic quality. This scale is extensively
    /// used in soprano vocal parts, violin, flute, oboe, and guitar repertoire, as well as
    /// piano compositions. E minor holds special significance in classical guitar literature
    /// and Spanish-influenced music, where its harmonic minor form creates the characteristic
    /// Phrygian dominant sound that defines flamenco. In this register, the scale provides
    /// both brilliance and expressiveness for melodic writing.
    pub static ref E4_HARMONIC_SCALE: Scale<8> = harmonic_scale(E4);

    /// F harmonic minor scale in octave 4.
    ///
    /// Notes: F4, G4, A♭4, B♭4, C5, D♭5, E5, F5
    /// MIDI note numbers: 65, 67, 68, 70, 72, 73, 76, 77
    /// Frequency range: ~349.23 Hz to ~698.46 Hz
    ///
    /// The F harmonic minor scale in octave 4 occupies the bright central register of music.
    /// At these frequencies, the scale offers exceptional clarity and presence. The augmented
    /// second interval between D♭5 and E5 creates a brilliant exotic quality that stands out
    /// clearly. This scale is employed in soprano vocal parts, violin, flute, clarinet, and
    /// piano compositions. F minor appears prominently in works by Chopin and Beethoven, where
    /// its harmonic minor form contributes to particularly expressive passages. In this register,
    /// the scale provides an ideal balance of brilliance and expressiveness, making it suitable
    /// for emotionally intense melodic writing that requires both clarity and intensity.
    pub static ref F4_HARMONIC_SCALE: Scale<8> = harmonic_scale(F4);

    /// F♯ harmonic minor scale in octave 4.
    ///
    /// Notes: F♯4, G♯4, A4, B4, C♯5, D5, E♯5, F♯5
    /// MIDI note numbers: 66, 68, 69, 71, 73, 74, 77, 78
    /// Frequency range: ~369.99 Hz to ~739.99 Hz
    ///
    /// The F♯ harmonic minor scale in octave 4 spans the central register of music, including
    /// concert pitch A4 (440 Hz) within its range. At these frequencies, the scale exhibits
    /// exceptional clarity and brilliance. The augmented second interval between D5 and E♯5
    /// creates a particularly bright and exotic quality. This scale is extensively employed in
    /// soprano vocal parts, violin, flute, piccolo, and piano compositions. The F♯ minor tonality
    /// appears prominently in Romantic and modern repertoire, often used for its capacity to
    /// express deep emotion. In this register, the harmonic minor form creates a particularly
    /// brilliant effect, making it suitable for virtuosic passages that showcase the distinctive
    /// character of this scale.
    pub static ref FSHARP4_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP4);

    /// G harmonic minor scale in octave 4.
    ///
    /// Notes: G4, A4, B♭4, C5, D5, E♭5, F♯5, G5
    /// MIDI note numbers: 67, 69, 70, 72, 74, 75, 78, 79
    /// Frequency range: ~392.00 Hz to ~783.99 Hz
    ///
    /// The G harmonic minor scale in octave 4 spans the bright central register of music,
    /// including concert pitch A4 (440 Hz) as its second degree. At these frequencies, the
    /// scale provides exceptional clarity and brilliance. The augmented second interval between
    /// E♭5 and F♯5 creates a particularly striking exotic quality in this high-middle register.
    /// This scale is extensively employed in soprano vocal parts, violin, flute, piccolo, and
    /// piano compositions. G minor is a historically significant key in classical music, from
    /// Bach's G minor fugues to Mozart's Symphony No. 40. In this register, the harmonic minor's
    /// distinctive character is prominently featured with exceptional clarity, making it ideal
    /// for expressive melodic writing that exploits the tension of the raised leading tone.
    pub static ref G4_HARMONIC_SCALE: Scale<8> = harmonic_scale(G4);

    /// G♯ harmonic minor scale in octave 4.
    ///
    /// Notes: G♯4, A♯4, B4, C♯5, D♯5, E5, F♯♯5, G♯5
    /// MIDI note numbers: 68, 70, 71, 73, 75, 76, 79, 80
    /// Frequency range: ~415.30 Hz to ~830.61 Hz
    ///
    /// The G♯ harmonic minor scale in octave 4 occupies the brilliant central-to-upper register
    /// of music. At these frequencies, the scale offers exceptional clarity with an increasingly
    /// bright timbre. The theoretical double-sharp (F♯♯5) would be enharmonically notated as G5
    /// in practice, and the scale is typically rewritten as A♭ minor for simplicity. This scale
    /// appears in soprano and coloratura soprano vocal parts, violin, flute, and piano compositions.
    /// The exotic character of the harmonic minor is particularly effective in this register for
    /// creating shimmering, brilliant passages in classical, impressionist, and film music contexts,
    /// with the augmented second interval standing out with exceptional clarity.
    pub static ref GSHARP4_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP4);

    /// A harmonic minor scale in octave 4.
    ///
    /// Notes: A4, B4, C5, D5, E5, F5, G♯5, A5
    /// MIDI note numbers: 69, 71, 72, 74, 76, 77, 80, 81
    /// Frequency range: ~440.00 Hz to ~880.00 Hz
    ///
    /// The A harmonic minor scale in octave 4 begins with concert pitch A4 (440 Hz) and spans
    /// into the brilliant upper register. At these frequencies, the scale exhibits exceptional
    /// clarity and brightness. The augmented second interval between F5 and G♯5 creates a
    /// particularly striking exotic quality at this height. This scale is extensively employed
    /// in soprano and coloratura soprano vocal parts, violin, flute, piccolo, and piano
    /// compositions. A minor is perhaps the most frequently used minor key in classical music,
    /// and its harmonic form is the quintessential example of the harmonic minor sound. Mozart's
    /// famous "Turkish Rondo" utilizes this scale to create its distinctive exotic character.
    /// In this register, the scale's distinctive features are presented with brilliant clarity.
    pub static ref A4_HARMONIC_SCALE: Scale<8> = harmonic_scale(A4);

    /// A♯ harmonic minor scale in octave 4.
    ///
    /// Notes: A♯4, B♯4, C♯5, D♯5, E♯5, F♯5, G♯♯5, A♯5
    /// MIDI note numbers: 70, 72, 73, 75, 77, 78, 81, 82
    /// Frequency range: ~466.16 Hz to ~932.33 Hz
    ///
    /// The A♯ harmonic minor scale in octave 4 spans the bright upper-middle register of music.
    /// At these frequencies, the scale offers exceptional brilliance and clarity. The theoretical
    /// B♯4 would be enharmonically written as C5 in practice, and the double-sharp (G♯♯5) as A5.
    /// Due to its complex notation, this scale is typically rewritten as B♭ minor. This scale
    /// appears in coloratura soprano vocal parts, violin, flute, piccolo, and piano compositions.
    /// Its exotic harmonic minor character is particularly effective in this register for creating
    /// brilliant and shimmering passages in classical, impressionist, and film music contexts,
    /// with the augmented second interval creating a particularly striking effect.
    pub static ref ASHARP4_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP4);

    /// B harmonic minor scale in octave 4.
    ///
    /// Notes: B4, C♯5, D5, E5, F♯5, G5, A♯5, B5
    /// MIDI note numbers: 71, 73, 74, 76, 78, 79, 82, 83
    /// Frequency range: ~493.88 Hz to ~987.77 Hz
    ///
    /// The B harmonic minor scale in octave 4 spans from the upper-middle to the high register,
    /// extending nearly to the 1 kHz threshold. At these frequencies, the scale exhibits brilliant
    /// clarity with an increasingly crystalline quality. The augmented second interval between G5
    /// and A♯5 creates a particularly striking exotic quality in this high register. This scale is
    /// employed in coloratura soprano vocal parts, violin, flute, piccolo, and piano compositions.
    /// B minor has been favored by many Romantic composers, including Chopin and Tchaikovsky, for
    /// its capacity to express deep passion. In this register, the harmonic minor's distinctive
    /// raised leading tone creates an exceptionally brilliant effect, providing a strong pull toward
    /// the tonic that's particularly effective for creating tension and release in virtuosic passages.
    pub static ref B4_HARMONIC_SCALE: Scale<8> = harmonic_scale(B4);
}

lazy_static! {
    /// C harmonic minor scale in octave 5.
    ///
    /// Notes: C5, D5, E♭5, F5, G5, A♭5, B5, C6
    /// MIDI note numbers: 72, 74, 75, 77, 79, 80, 83, 84
    /// Frequency range: ~523.25 Hz to ~1046.50 Hz
    ///
    /// The C harmonic minor scale in octave 5 spans the brilliant high register that exceeds the
    /// range of most human voices except for coloratura sopranos. At these frequencies, the scale
    /// exhibits exceptional brilliance and crystalline clarity. The augmented second interval
    /// between A♭5 and B5 creates a particularly striking exotic quality in this high register.
    /// This scale is employed in piccolo, flute, violin harmonics, glockenspiel, and coloratura
    /// soprano passages. In this register, the harmonic minor's distinctive intervals take on an
    /// almost ethereal, shimmering quality. The C5 harmonic minor is featured in virtuosic cadenzas,
    /// orchestral climaxes, and impressionistic compositions where its tension and brilliance can
    /// create moments of intense musical expression.
    pub static ref C5_HARMONIC_SCALE: Scale<8> = harmonic_scale(C5);

    /// C♯ harmonic minor scale in octave 5.
    ///
    /// Notes: C♯5, D♯5, E5, F♯5, G♯5, A5, B♯5, C♯6
    /// MIDI note numbers: 73, 75, 76, 78, 80, 81, 84, 85
    /// Frequency range: ~554.37 Hz to ~1108.73 Hz
    ///
    /// The C♯ harmonic minor scale in octave 5 occupies the brilliant high register that creates
    /// a penetrating, crystalline sound quality. At these frequencies, the scale offers exceptional
    /// definition and brilliance. The augmented second interval between A5 and B♯5 creates a
    /// strikingly ethereal exotic quality. This scale is employed in piccolo, flute, highest
    /// violin positions, celesta, and occasional coloratura soprano passages. The C♯ minor tonality
    /// in this register creates a particularly otherworldly, shimmering character ideal for
    /// creating moments of transcendent beauty in impressionistic compositions, virtuosic solo
    /// passages, and delicate orchestral textures where its distinctive harmonic minor intervals
    /// create a sense of suspended animation.
    pub static ref CSHARP5_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP5);

    /// D harmonic minor scale in octave 5.
    ///
    /// Notes: D5, E5, F5, G5, A5, B♭5, C♯6, D6
    /// MIDI note numbers: 74, 76, 77, 79, 81, 82, 85, 86
    /// Frequency range: ~587.33 Hz to ~1174.66 Hz
    ///
    /// The D harmonic minor scale in octave 5 spans a brilliant high register that creates a
    /// penetrating, glistening sound quality. At these frequencies, the scale exhibits exceptional
    /// brilliance with crystalline definition. The augmented second interval between B♭5 and C♯6
    /// creates a particularly striking exotic quality with an almost otherworldly character. This
    /// scale is employed in piccolo, flute, highest violin positions, glockenspiel, and is generally
    /// beyond the range of most vocalists except for exceptional coloratura sopranos. In this
    /// register, the harmonic minor's distinctive intervals create a sense of ethereal tension
    /// and otherworldly beauty, making it particularly effective in impressionistic compositions,
    /// virtuosic cadenzas, and delicate orchestral textures.
    pub static ref D5_HARMONIC_SCALE: Scale<8> = harmonic_scale(D5);

    /// D♯ harmonic minor scale in octave 5.
    ///
    /// Notes: D♯5, E♯5, F♯5, G♯5, A♯5, B5, C♯♯6, D♯6
    /// MIDI note numbers: 75, 77, 78, 80, 82, 83, 86, 87
    /// Frequency range: ~622.25 Hz to ~1244.51 Hz
    ///
    /// The D♯ harmonic minor scale in octave 5 occupies an exceptionally high register with
    /// brilliant, crystalline sonic properties. At these frequencies, the scale creates a
    /// penetrating, bell-like quality. The theoretical double-sharp (C♯♯6) would be enharmonically
    /// notated as D6 in practice, and the scale is typically rewritten as E♭ minor. This scale
    /// appears in piccolo, flute, highest violin harmonics, glockenspiel, and celesta passages.
    /// In this register, the harmonic minor's distinctive intervals create an almost otherworldly
    /// quality that composers have utilized for creating moments of transcendent beauty in
    /// impressionistic works, virtuosic solo passages, and delicate orchestral textures where
    /// the exotic quality of the harmonic minor becomes almost celestial.
    pub static ref DSHARP5_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP5);

    /// E harmonic minor scale in octave 5.
    ///
    /// Notes: E5, F♯5, G5, A5, B5, C6, D♯6, E6
    /// MIDI note numbers: 76, 78, 79, 81, 83, 84, 87, 88
    /// Frequency range: ~659.26 Hz to ~1318.51 Hz
    ///
    /// The E harmonic minor scale in octave 5 spans an exceptionally high register with brilliant,
    /// shimmering sonic properties. At these frequencies, the scale creates a penetrating,
    /// crystalline quality. The augmented second interval between C6 and D♯6 creates a particularly
    /// striking exotic quality with an almost celestial character. This scale is primarily employed
    /// in piccolo, flute, highest violin positions, glockenspiel, and celesta. In this register,
    /// the harmonic minor's distinctive intervals create an ethereal, otherworldly quality that
    /// composers like Debussy and Ravel utilized for creating moments of transcendent beauty.
    /// The E minor tonality at this extreme height takes on an almost supernatural quality,
    /// particularly effective for ethereal or mystical musical moments.
    pub static ref E5_HARMONIC_SCALE: Scale<8> = harmonic_scale(E5);

    /// F harmonic minor scale in octave 5.
    ///
    /// Notes: F5, G5, A♭5, B♭5, C6, D♭6, E6, F6
    /// MIDI note numbers: 77, 79, 80, 82, 84, 85, 88, 89
    /// Frequency range: ~698.46 Hz to ~1396.91 Hz
    ///
    /// The F harmonic minor scale in octave 5 occupies an exceptionally high register with brilliant,
    /// silvery sonic properties. At these frequencies, the scale creates a penetrating, bell-like
    /// quality. The augmented second interval between D♭6 and E6 creates a particularly striking
    /// exotic quality with an almost ethereal character. This scale is employed almost exclusively
    /// in piccolo, flute, highest violin harmonics, glockenspiel, and celesta. In this register,
    /// the harmonic minor's distinctive intervals create a shimmering, otherworldly quality that
    /// composers have utilized for creating moments of transcendent beauty and tension in
    /// impressionistic compositions, where the dark F minor tonality is transformed into something
    /// delicate and almost supernatural at this extreme height.
    pub static ref F5_HARMONIC_SCALE: Scale<8> = harmonic_scale(F5);

    /// F♯ harmonic minor scale in octave 5.
    ///
    /// Notes: F♯5, G♯5, A5, B5, C♯6, D6, E♯6, F♯6
    /// MIDI note numbers: 78, 80, 81, 83, 85, 86, 89, 90
    /// Frequency range: ~739.99 Hz to ~1479.98 Hz
    ///
    /// The F♯ harmonic minor scale in octave 5 spans an extremely high register with brilliant,
    /// crystalline sonic properties. At these frequencies, the scale creates a piercing, bell-like
    /// quality. The augmented second interval between D6 and E♯6 creates a particularly striking
    /// exotic quality with an almost celestial character. This scale is employed exclusively in
    /// piccolo, flute, highest violin harmonics, glockenspiel, and celesta. In this register, the
    /// harmonic minor's distinctive intervals create a shimmering, almost supernatural quality that
    /// composers like Debussy utilized for creating moments of ethereal beauty. The F♯ minor tonality
    /// at this height becomes transformative, creating a sense of otherworldly tension and release
    /// that transcends its traditional melancholic associations.
    pub static ref FSHARP5_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP5);

    /// G harmonic minor scale in octave 5.
    ///
    /// Notes: G5, A5, B♭5, C6, D6, E♭6, F♯6, G6
    /// MIDI note numbers: 79, 81, 82, 84, 86, 87, 90, 91
    /// Frequency range: ~783.99 Hz to ~1567.98 Hz
    ///
    /// The G harmonic minor scale in octave 5 occupies an extremely high register with brilliant,
    /// glistening sonic properties. At these frequencies, the scale creates a penetrating, almost
    /// glass-like quality. The augmented second interval between E♭6 and F♯6 creates a particularly
    /// striking exotic quality with an almost supernatural character. This scale is employed in
    /// piccolo, highest flute register, extreme violin harmonics, glockenspiel, and celesta. In this
    /// register, the harmonic minor's distinctive intervals create a shimmering, otherworldly quality
    /// that composers have utilized for creating moments of transcendent beauty in impressionistic
    /// works. The traditional dark character of G minor is transformed at this height into something
    /// delicate, ethereal, and almost floating above conventional harmonic expression.
    pub static ref G5_HARMONIC_SCALE: Scale<8> = harmonic_scale(G5);

    /// G♯ harmonic minor scale in octave 5.
    ///
    /// Notes: G♯5, A♯5, B5, C♯6, D♯6, E6, F♯♯6, G♯6
    /// MIDI note numbers: 80, 82, 83, 85, 87, 88, 91, 92
    /// Frequency range: ~830.61 Hz to ~1661.22 Hz
    ///
    /// The G♯ harmonic minor scale in octave 5 spans an extremely high register with brilliant,
    /// crystalline sonic properties. At these frequencies, the scale creates a penetrating, almost
    /// glass-like quality. The theoretical double-sharp (F♯♯6) would be enharmonically notated as
    /// G6 in practice, and the scale is typically rewritten as A♭ minor. This scale is employed
    /// exclusively in piccolo, extreme flute register, violin harmonics, glockenspiel, celesta,
    /// and occasionally in advanced synthesizer programming. In this register, the harmonic minor's
    /// distinctive intervals create an almost otherworldly quality that approaches the threshold of
    /// timbral rather than harmonic expression. The exotic quality becomes transformed into pure
    /// sonic color, especially useful in impressionistic and contemporary compositions.
    pub static ref GSHARP5_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP5);

    /// A harmonic minor scale in octave 5.
    ///
    /// Notes: A5, B5, C6, D6, E6, F6, G♯6, A6
    /// MIDI note numbers: 81, 83, 84, 86, 88, 89, 92, 93
    /// Frequency range: ~880.00 Hz to ~1760.00 Hz
    ///
    /// The A harmonic minor scale in octave 5 occupies an extremely high register with brilliant,
    /// shimmering sonic properties. At these frequencies, the scale creates a penetrating, bell-like
    /// quality approaching the upper limits of conventional musical notation. The augmented second
    /// interval between F6 and G♯6 creates a particularly striking exotic quality with an almost
    /// celestial character. This scale is employed exclusively in piccolo, extreme flute register,
    /// highest violin harmonics, glockenspiel, and celesta. In this register, the harmonic minor's
    /// distinctive intervals create a crystalline, otherworldly quality that composers have utilized
    /// for creating moments of transcendent beauty. The traditional "exotic" character of A harmonic
    /// minor becomes almost purely timbral at this extreme height.
    pub static ref A5_HARMONIC_SCALE: Scale<8> = harmonic_scale(A5);

    /// A♯ harmonic minor scale in octave 5.
    ///
    /// Notes: A♯5, B♯5, C♯6, D♯6, E♯6, F♯6, G♯♯6, A♯6
    /// MIDI note numbers: 82, 84, 85, 87, 89, 90, 93, 94
    /// Frequency range: ~932.33 Hz to ~1864.66 Hz
    ///
    /// The A♯ harmonic minor scale in octave 5 spans an extremely high register with brilliant,
    /// crystalline sonic properties. At these frequencies, the scale creates a penetrating,
    /// almost glass-like quality. The theoretical B♯5 would be enharmonically written as C6 in
    /// practice, and the double-sharp (G♯♯6) as A6. This scale is employed exclusively in piccolo,
    /// extreme flute register, highest violin harmonics, glockenspiel, and celesta. In this register,
    /// the harmonic minor's distinctive intervals are transformed into pure sonic color rather than
    /// traditional harmonic expression. Composers have utilized this extreme register for creating
    /// otherworldly effects in impressionistic works, where the complex harmonic minor relationships
    /// become almost purely timbral phenomena.
    pub static ref ASHARP5_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP5);

    /// B harmonic minor scale in octave 5.
    ///
    /// Notes: B5, C♯6, D6, E6, F♯6, G6, A♯6, B6
    /// MIDI note numbers: 83, 85, 86, 88, 90, 91, 94, 95
    /// Frequency range: ~987.77 Hz to ~1975.53 Hz
    ///
    /// The B harmonic minor scale in octave 5 spans an extraordinarily high register with brilliant,
    /// glass-like sonic properties. At these frequencies, the scale creates a penetrating, almost
    /// ethereal quality approaching the upper limits of conventional musical notation. The augmented
    /// second interval between G6 and A♯6 creates a particularly striking exotic quality with an
    /// almost supernatural character. This scale is employed exclusively in piccolo, extreme flute
    /// register, highest violin harmonics, glockenspiel, and celesta. In this register, the
    /// harmonic minor's distinctive intervals are transformed almost entirely into timbral rather
    /// than harmonic phenomena. Composers have utilized this extreme register for creating
    /// otherworldly, celestial effects where the traditional passionate character of B minor becomes
    /// ethereal and transcendent.
    pub static ref B5_HARMONIC_SCALE: Scale<8> = harmonic_scale(B5);
}

lazy_static! {
    /// C harmonic minor scale in octave 6.
    ///
    /// Notes: C6, D6, E♭6, F6, G6, A♭6, B6, C7
    /// MIDI note numbers: 84, 86, 87, 89, 91, 92, 95, 96
    /// Frequency range: ~1046.50 Hz to ~2093.00 Hz
    ///
    /// The C harmonic minor scale in octave 6 spans an extraordinarily high register that approaches
    /// the upper limits of conventional musical notation and instrumental range. At these frequencies,
    /// the scale creates an extremely brilliant, piercing quality more akin to pure tone color than
    /// traditional harmony. The augmented second interval between A♭6 and B6 becomes almost purely a
    /// timbral effect rather than a melodic feature. This scale is employed exclusively in piccolo
    /// (at its very upper limit), glockenspiel, celesta, crotales, specialized synthesizer sounds, and
    /// occasionally notated as extreme violin harmonics. In this register, the harmonic minor's distinctive
    /// intervals create what composers might describe as "shafts of light" or "crystalline sparkle,"
    /// utilized sparingly for extreme coloristic effects in orchestration and electroacoustic music.
    pub static ref C6_HARMONIC_SCALE: Scale<8> = harmonic_scale(C6);

    /// C♯ harmonic minor scale in octave 6.
    ///
    /// Notes: C♯6, D♯6, E6, F♯6, G♯6, A6, B♯6, C♯7
    /// MIDI note numbers: 85, 87, 88, 90, 92, 93, 96, 97
    /// Frequency range: ~1108.73 Hz to ~2217.46 Hz
    ///
    /// The C♯ harmonic minor scale in octave 6 occupies an extreme high register beyond conventional
    /// melodic writing. At these frequencies, the scale produces an ultra-brilliant, glass-like quality
    /// where the traditional harmonic relationships become almost pure timbral colors. The augmented
    /// second interval between A6 and B♯6 creates a striking spectral effect rather than a melodic
    /// feature. This scale appears exclusively in glockenspiel, celesta, crotales, and specialized
    /// electronic tones, exceeding the range of virtually all standard orchestral instruments. Composers
    /// utilize this extreme register primarily for coloristic effects, bell-like sonorities, and what
    /// Messiaen might have described as "resonance effects," where the pitches function more as
    /// overtone structures than as conventional melodic material.
    pub static ref CSHARP6_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP6);

    /// D harmonic minor scale in octave 6.
    ///
    /// Notes: D6, E6, F6, G6, A6, B♭6, C♯7, D7
    /// MIDI note numbers: 86, 88, 89, 91, 93, 94, 97, 98
    /// Frequency range: ~1174.66 Hz to ~2349.32 Hz
    ///
    /// The D harmonic minor scale in octave 6 spans an extreme high register at the threshold of
    /// human pitch discrimination capabilities. At these frequencies, the scale creates a super-brilliant,
    /// almost needle-like quality where the notes function more as spectral colors than as
    /// traditional harmonic entities. The augmented second interval between B♭6 and C♯7 manifests as
    /// a striking timbral effect rather than a melodic feature. This scale is utilized exclusively
    /// in glockenspiel, crotales, specialized electronic tones, and as theoretical extensions on
    /// instruments like celesta. In contemporary compositions, these extreme registers might be employed
    /// for special effects like "shimmer," "sparkle," or "crystalline textures," evoking what composer
    /// Kaija Saariaho has described as "the boundary between sound and light."
    pub static ref D6_HARMONIC_SCALE: Scale<8> = harmonic_scale(D6);

    /// D♯ harmonic minor scale in octave 6.
    ///
    /// Notes: D♯6, E♯6, F♯6, G♯6, A♯6, B6, C♯♯7, D♯7
    /// MIDI note numbers: 87, 89, 90, 92, 94, 95, 98, 99
    /// Frequency range: ~1244.51 Hz to ~2489.02 Hz
    ///
    /// The D♯ harmonic minor scale in octave 6 reaches an extreme high register where conventional
    /// harmonic perception begins to transform into pure timbral experience. At these frequencies,
    /// the scale creates an ultra-brilliant, glass-like quality. The theoretical double-sharp (C♯♯7)
    /// would be enharmonically notated as D7 in practice. This scale exists almost exclusively in
    /// electronic music, specialized percussion like crotales and glass harmonica, and as theoretical
    /// extensions beyond the practical range of acoustic instruments. In this register, the harmonic
    /// minor's distinctive intervals create what composer Jonathan Harvey described as "upper
    /// partials of timbre" rather than traditional melodic content, used for creating otherworldly
    /// sonic textures in spectral music and advanced electroacoustic compositions.
    pub static ref DSHARP6_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP6);

    /// E harmonic minor scale in octave 6.
    ///
    /// Notes: E6, F♯6, G6, A6, B6, C7, D♯7, E7
    /// MIDI note numbers: 88, 90, 91, 93, 95, 96, 99, 100
    /// Frequency range: ~1318.51 Hz to ~2637.02 Hz
    ///
    /// The E harmonic minor scale in octave 6 occupies an extreme high register at the upper
    /// limits of conventional musical notation. At these frequencies, the scale produces an
    /// ultra-brilliant, glass-like quality where traditional harmonic relationships are perceived
    /// almost entirely as timbral colors. The augmented second interval between C7 and D♯7 exists
    /// primarily as a spectral effect rather than a melodic feature. This scale appears exclusively
    /// in specialized electronic tones, crotales, and as theoretical extensions beyond most acoustic
    /// instruments. In works by composers like György Ligeti and Tristan Murail, such extreme
    /// registers function as pure "sonic light" - creating luminous textures that transcend
    /// conventional melody and harmony to evoke what might be described as "acoustic iridescence."
    pub static ref E6_HARMONIC_SCALE: Scale<8> = harmonic_scale(E6);

    /// F harmonic minor scale in octave 6.
    ///
    /// Notes: F6, G6, A♭6, B♭6, C7, D♭7, E7, F7
    /// MIDI note numbers: 89, 91, 92, 94, 96, 97, 100, 101
    /// Frequency range: ~1396.91 Hz to ~2793.83 Hz
    ///
    /// The F harmonic minor scale in octave 6 spans an extreme high register that approaches the
    /// upper threshold of practical musical application. At these frequencies, the scale creates
    /// an ultra-brilliant, needle-like quality where notes function primarily as pure sonic color.
    /// The augmented second interval between D♭7 and E7 exists almost entirely as a spectral rather
    /// than a melodic phenomenon. This scale exists primarily in electronic music contexts, specialized
    /// microtonal percussion, and advanced synthesis techniques. In this register, the harmonic
    /// minor's distinctive intervals serve what composer Gérard Grisey described as "prismatic"
    /// functions - breaking sound into pure spectral components that transcend traditional melodic
    /// and harmonic frameworks, used primarily for creating ethereal, otherworldly textures in
    /// advanced contemporary compositions.
    pub static ref F6_HARMONIC_SCALE: Scale<8> = harmonic_scale(F6);

    /// F♯ harmonic minor scale in octave 6.
    ///
    /// Notes: F♯6, G♯6, A6, B6, C♯7, D7, E♯7, F♯7
    /// MIDI note numbers: 90, 92, 93, 95, 97, 98, 101, 102
    /// Frequency range: ~1479.98 Hz to ~2959.96 Hz
    ///
    /// The F♯ harmonic minor scale in octave 6 exists in an extreme high register that verges on
    /// the limits of practical musical applications. At these frequencies, the scale produces an
    /// extremely brilliant, crystalline quality where notes function almost exclusively as timbral
    /// phenomena rather than melodic entities. The augmented second interval between D7 and E♯7
    /// manifests as a pure spectral effect. This scale exists primarily in advanced electronic music,
    /// specialized microtonal percussion, and theoretical extensions beyond conventional instruments.
    /// In works by composers like Karlheinz Stockhausen, these ultra-high frequencies serve as pure
    /// "sound-light" - creating glistening sonic textures that function more as acoustic prisms than
    /// conventional melodic material, used for evoking what might be described as "acoustic aurora."
    pub static ref FSHARP6_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP6);

    /// G harmonic minor scale in octave 6.
    ///
    /// Notes: G6, A6, B♭6, C7, D7, E♭7, F♯7, G7
    /// MIDI note numbers: 91, 93, 94, 96, 98, 99, 102, 103
    /// Frequency range: ~1567.98 Hz to ~3135.96 Hz
    ///
    /// The G harmonic minor scale in octave 6 occupies an extraordinarily high register that approaches
    /// the upper limits of practical musical usage. At these frequencies, the scale creates an
    /// ultra-brilliant, almost laser-like quality where notes function primarily as spectral phenomena.
    /// The augmented second interval between E♭7 and F♯7 exists purely as a timbral effect rather than
    /// a melodic feature. This scale appears almost exclusively in advanced electronic music, specialized
    /// microtonal percussion, and theoretical extensions beyond most conventional instruments. In works
    /// by composers exploring spectral techniques, these extreme frequencies function as what composer
    /// Salvatore Sciarrino described as "acoustic phosphorescence" - creating glistening sonic textures
    /// that transcend conventional notions of melody and harmony to evoke pure sonic illumination.
    pub static ref G6_HARMONIC_SCALE: Scale<8> = harmonic_scale(G6);

    /// G♯ harmonic minor scale in octave 6.
    ///
    /// Notes: G♯6, A♯6, B6, C♯7, D♯7, E7, F♯♯7, G♯7
    /// MIDI note numbers: 92, 94, 95, 97, 99, 100, 103, 104
    /// Frequency range: ~1661.22 Hz to ~3322.44 Hz
    ///
    /// The G♯ harmonic minor scale in octave 6 spans an extreme high register that exists at the
    /// very threshold of practical musical application. At these frequencies, the scale produces
    /// an ultra-brilliant, needle-like quality where the theoretical double-sharp (F♯♯7) would be
    /// enharmonically notated as G7, though at such extreme registers, traditional notation becomes
    /// almost purely theoretical. This scale exists primarily in advanced electronic music, specialized
    /// synthesis techniques, and as theoretical extensions beyond acoustic instruments. In contemporary
    /// compositions employing spectral techniques, these extreme registers function as what composer
    /// Toru Takemitsu described as "streams of sound-light" - creating ethereal sonic textures that
    /// transcend conventional musical parameters to function purely as acoustic phenomena.
    pub static ref GSHARP6_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP6);

    /// A harmonic minor scale in octave 6.
    ///
    /// Notes: A6, B6, C7, D7, E7, F7, G♯7, A7
    /// MIDI note numbers: 93, 95, 96, 98, 100, 101, 104, 105
    /// Frequency range: ~1760.00 Hz to ~3520.00 Hz
    ///
    /// The A harmonic minor scale in octave 6 occupies an extraordinarily high register at the extreme
    /// upper edge of practical musical application. At these frequencies, the scale creates an
    /// ultra-brilliant, stratospheric quality where notes function almost exclusively as spectral
    /// phenomena rather than melodic entities. The augmented second interval between F7 and G♯7 exists
    /// purely as a timbral effect. This scale appears primarily in advanced electronic music, spectral
    /// synthesis, and theoretical extensions far beyond conventional acoustic instruments. In works
    /// exploring electroacoustic techniques, these extreme registers serve what composer Brian Ferneyhough
    /// might describe as "sonic luminescence" - creating shimmering textures that transcend conventional
    /// musical parameters to function as pure acoustic light phenomena, evoking sensations of sonic
    /// transparency and crystalline brilliance.
    pub static ref A6_HARMONIC_SCALE: Scale<8> = harmonic_scale(A6);

    /// A♯ harmonic minor scale in octave 6.
    ///
    /// Notes: A♯6, B♯6, C♯7, D♯7, E♯7, F♯7, G♯♯7, A♯7
    /// MIDI note numbers: 94, 96, 97, 99, 101, 102, 105, 106
    /// Frequency range: ~1864.66 Hz to ~3729.31 Hz
    ///
    /// The A♯ harmonic minor scale in octave 6 spans an extreme high register that exists primarily
    /// in theoretical contexts. At these frequencies, the scale creates an ultra-brilliant, almost
    /// purely spectral quality where the theoretical B♯6 would be enharmonically written as C7, and
    /// the double-sharp (G♯♯7) as A7, though at such extreme heights, traditional notation becomes
    /// almost purely conceptual. This scale exists primarily in advanced electronic music, specialized
    /// synthesis techniques, and computer-generated sound. In contemporary experimental compositions,
    /// these extreme registers function as what composer Helmut Lachenmann might describe as "sound
    /// structures beyond pitch" - creating ethereal textures that transcend conventional musical
    /// parameters to function as pure acoustic phenomena, more akin to sonic light than traditional
    /// musical notes.
    pub static ref ASHARP6_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP6);

    /// B harmonic minor scale in octave 6.
    ///
    /// Notes: B6, C♯7, D7, E7, F♯7, G7, A♯7, B7
    /// MIDI note numbers: 95, 97, 98, 100, 102, 103, 106, 107
    /// Frequency range: ~1975.53 Hz to ~3951.07 Hz
    ///
    /// The B harmonic minor scale in octave 6 exists at an extreme high register that reaches the
    /// very threshold of human pitch perception capabilities. At these frequencies, approaching 4kHz,
    /// the scale creates an ultra-brilliant, ethereal quality where notes function almost entirely
    /// as spectral phenomena rather than melodic entities. The augmented second interval between G7
    /// and A♯7 exists purely as a timbral effect beyond conventional harmonic perception. This scale
    /// appears exclusively in advanced electronic music, spectral synthesis, and theoretical extensions
    /// far beyond conventional acoustic instruments. In works exploring electroacoustic boundaries,
    /// these extreme registers function as what composer Kaija Saariaho described as "the fusion of
    /// sound and light" - creating ephemeral textures that transcend conventional musical parameters
    /// to evoke pure sensations of acoustic luminosity and spectral brilliance.
    pub static ref B6_HARMONIC_SCALE: Scale<8> = harmonic_scale(B6);
}

lazy_static! {
    /// C harmonic minor scale in octave 7.
    ///
    /// Notes: C7, D7, E♭7, F7, G7, A♭7, B7, C8
    /// MIDI note numbers: 96, 98, 99, 101, 103, 104, 107, 108
    /// Frequency range: ~2093.00 Hz to ~4186.01 Hz
    ///
    /// The C harmonic minor scale in octave 7 represents an extraordinarily high register at the
    /// extreme limit of conventional musical notation and human pitch perception. At these frequencies,
    /// beyond 2kHz, the scale exists primarily as pure spectral phenomenon rather than melodic material.
    /// The augmented second interval between A♭7 and B7 manifests purely as a timbral effect, with all
    /// harmonic relationships perceived more as sonic color than traditional harmony. This scale exists
    /// almost exclusively in advanced electronic music, spectral synthesis, computer-generated sound art,
    /// and theoretical contexts. In contemporary compositions, these extreme registers serve what
    /// composer Gérard Grisey described as "the dissolution of pitch into pure timbre" - functioning
    /// as sonic light or acoustic prisms that create brilliant, ethereal textures at the threshold
    /// where pitch perception begins to merge with our perception of pure timbre.
    pub static ref C7_HARMONIC_SCALE: Scale<8> = harmonic_scale(C7);

    /// C♯ harmonic minor scale in octave 7.
    ///
    /// Notes: C♯7, D♯7, E7, F♯7, G♯7, A7, B♯7, C♯8
    /// MIDI note numbers: 97, 99, 100, 102, 104, 105, 108, 109
    /// Frequency range: ~2217.46 Hz to ~4434.92 Hz
    ///
    /// The C♯ harmonic minor scale in octave 7 occupies an extreme high register that approaches
    /// the upper threshold of human pitch discrimination. At these frequencies, exceeding 2kHz,
    /// traditional musical concepts of melody and harmony give way almost entirely to spectral and
    /// timbral phenomena. The augmented second interval between A7 and B♯7 functions purely as a
    /// spectral effect. This scale exists exclusively in electronic music, spectral synthesis, and
    /// computer-generated sound art - beyond the practical range of all acoustic instruments. In this
    /// register, the harmonic minor's distinctive intervals function as what composer Jonathan Harvey
    /// called "spectral colorations" - creating textures that transcend conventional musical parameters
    /// to evoke sensations of pure crystalline light, used in advanced contemporary compositions to
    /// create otherworldly sonic atmospheres at the boundary of conventional musical notation.
    pub static ref CSHARP7_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP7);

    /// D harmonic minor scale in octave 7.
    ///
    /// Notes: D7, E7, F7, G7, A7, B♭7, C♯8, D8
    /// MIDI note numbers: 98, 100, 101, 103, 105, 106, 109, 110
    /// Frequency range: ~2349.32 Hz to ~4698.64 Hz
    ///
    /// The D harmonic minor scale in octave 7 spans an extremely high register that exists at the very
    /// boundary of conventional musical notation and pitch perception. At these frequencies, approaching
    /// 5kHz, traditional musical concepts dissolve into pure sonic phenomena. The augmented second
    /// interval between B♭7 and C♯8 functions exclusively as a spectral effect rather than a melodic
    /// feature. This scale exists only in electronic music, computer synthesis, and theoretical contexts,
    /// far beyond the capabilities of acoustic instruments. In works of electronic and spectral music,
    /// these extreme frequencies function as what composer Tristan Murail described as "sonic photons" -
    /// creating ephemeral textures that behave more like particles of acoustic light than traditional
    /// musical notes, employed for ethereal effects where sound and light seemingly converge.
    pub static ref D7_HARMONIC_SCALE: Scale<8> = harmonic_scale(D7);

    /// D♯ harmonic minor scale in octave 7.
    ///
    /// Notes: D♯7, E♯7, F♯7, G♯7, A♯7, B7, C♯♯8, D♯8
    /// MIDI note numbers: 99, 101, 102, 104, 106, 107, 110, 111
    /// Frequency range: ~2489.02 Hz to ~4978.03 Hz
    ///
    /// The D♯ harmonic minor scale in octave 7 reaches an extraordinarily high register beyond
    /// conventional musical applications, where notation becomes almost purely theoretical. At these
    /// frequencies, approaching 5kHz, pitch perception begins to merge with our perception of timbre.
    /// The theoretical double-sharp (C♯♯8) would be enharmonically notated as D8, though at such
    /// extreme heights, traditional notation is primarily conceptual. This scale exists exclusively
    /// in computer music, advanced spectral synthesis, and theoretical contexts. In electronic
    /// compositions, these ultra-high frequencies serve what composer Kaija Saariaho described as
    /// "acoustic luminosity" - creating pure spectral textures that function entirely as sonic color
    /// rather than traditional pitch material, utilized at the furthest boundaries of musical notation
    /// where sound approaches pure light.
    pub static ref DSHARP7_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP7);

    /// E harmonic minor scale in octave 7.
    ///
    /// Notes: E7, F♯7, G7, A7, B7, C8, D♯8, E8
    /// MIDI note numbers: 100, 102, 103, 105, 107, 108, 111, 112
    /// Frequency range: ~2637.02 Hz to ~5274.04 Hz
    ///
    /// The E harmonic minor scale in octave 7 exists at the extreme upper threshold of conventional
    /// musical notation, where pitch perception begins to transition to pure spectral sensation. At
    /// these frequencies, exceeding 5kHz at its upper range, traditional musical relationships
    /// dissolve into pure sonic phenomena. The augmented second interval between C8 and D♯8 functions
    /// exclusively as spectral coloration rather than a melodic feature. This scale exists solely
    /// in electronic music, computer synthesis, and theoretical contexts. In advanced electroacoustic
    /// works, these extreme registers serve what composer Karlheinz Stockhausen described as "aural
    /// light particles" - creating ethereal textures that transcend conventional musical parameters to
    /// function as pure acoustic phenomena, existing at the uppermost boundary where conventional
    /// music notation meets pure sonic abstraction.
    pub static ref E7_HARMONIC_SCALE: Scale<8> = harmonic_scale(E7);

    /// F harmonic minor scale in octave 7.
    ///
    /// Notes: F7, G7, A♭7, B♭7, C8, D♭8, E8, F8
    /// MIDI note numbers: 101, 103, 104, 106, 108, 109, 112, 113
    /// Frequency range: ~2793.83 Hz to ~5587.65 Hz
    ///
    /// The F harmonic minor scale in octave 7 spans an extreme high register that exists at the
    /// absolute boundary of conventional musical notation and human pitch perception. At these
    /// frequencies, above 5.5kHz at its upper range, pitch relationships dissolve entirely into
    /// pure spectral phenomena. The augmented second interval between D♭8 and E8 manifests exclusively
    /// as a timbral effect. This scale exists solely in computer music, spectral synthesis, and
    /// purely theoretical contexts. In works exploring the outer limits of sonic perception, these
    /// extreme registers function as what composer Brian Ferneyhough might describe as "the sonic
    /// equivalent of ultraviolet light" - creating textures that exist at the threshold of human
    /// perception, where sound becomes pure acoustic energy and traditional musical parameters
    /// dissolve into abstract spectral relationships.
    pub static ref F7_HARMONIC_SCALE: Scale<8> = harmonic_scale(F7);

    /// F♯ harmonic minor scale in octave 7.
    ///
    /// Notes: F♯7, G♯7, A7, B7, C♯8, D8, E♯8, F♯8
    /// MIDI note numbers: 102, 104, 105, 107, 109, 110, 113, 114
    /// Frequency range: ~2959.96 Hz to ~5919.91 Hz
    ///
    /// The F♯ harmonic minor scale in octave 7 represents an extreme high register that exists at
    /// the absolute upper threshold of conventional musical notation. At these frequencies, approaching
    /// 6kHz, pitch perception merges completely with our perception of pure timbre. The augmented
    /// second interval between D8 and E♯8 exists purely as a spectral phenomenon rather than a
    /// melodic feature. This scale exists exclusively in electronic music, advanced spectral synthesis,
    /// and theoretical contexts. In contemporary electronic compositions, these ultra-high frequencies
    /// serve what composer Salvatore Sciarrino might describe as "points of acoustic light" - creating
    /// ephemeral textures that transcend conventional musical parameters to function as pure sonic
    /// energy, operating at the furthest boundary of musical notation where traditional concepts of
    /// melody and harmony dissolve entirely into abstract spectral relationships.
    pub static ref FSHARP7_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP7);

    /// G harmonic minor scale in octave 7.
    ///
    /// Notes: G7, A7, B♭7, C8, D8, E♭8, F♯8, G8
    /// MIDI note numbers: 103, 105, 106, 108, 110, 111, 114, 115
    /// Frequency range: ~3135.96 Hz to ~6271.93 Hz
    ///
    /// The G harmonic minor scale in octave 7 exists at the extreme upper boundary of conventional
    /// musical notation and pitch perception. At these frequencies, exceeding 6kHz at its upper range,
    /// traditional musical relationships become entirely abstract spectral phenomena. The augmented
    /// second interval between E♭8 and F♯8 exists purely as a timbral effect. This scale exists
    /// exclusively in advanced electronic music, computer synthesis, and theoretical contexts. In
    /// contemporary experimental compositions, these extreme registers serve what composer Helmut
    /// Lachenmann described as "sound structures beyond the threshold of pitch" - creating ethereal
    /// textures that function as pure sonic energy rather than conventional musical material. This
    /// scale represents an extreme boundary of musical notation where sound approaches the limits of
    /// human pitch perception, existing primarily as abstract spectral phenomena.
    pub static ref G7_HARMONIC_SCALE: Scale<8> = harmonic_scale(G7);

    /// G♯ harmonic minor scale in octave 7.
    ///
    /// Notes: G♯7, A♯7, B7, C♯8, D♯8, E8, F♯♯8, G♯8
    /// MIDI note numbers: 104, 106, 107, 109, 111, 112, 115, 116
    /// Frequency range: ~3322.44 Hz to ~6644.88 Hz
    ///
    /// The G♯ harmonic minor scale in octave 7 spans an extremely high register that exists at
    /// the absolute threshold of human pitch perception and conventional musical notation. At these
    /// frequencies, above 6.5kHz, conventional musical concepts dissolve entirely into abstract
    /// spectral phenomena. The theoretical double-sharp (F♯♯8) would be enharmonically represented
    /// as G8, though at such extreme registers, traditional notation is purely conceptual. This scale
    /// exists exclusively in computer music, advanced spectral synthesis, and theoretical contexts.
    /// In electronic and computer music exploring perceptual boundaries, these extreme registers
    /// function entirely as spectral colors that transcend conventional musical parameters, operating
    /// at frequencies where human pitch perception begins to break down and sound exists primarily
    /// as abstract spectral energy rather than perceived pitch.
    pub static ref GSHARP7_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP7);

    /// A harmonic minor scale in octave 7.
    ///
    /// Notes: A7, B7, C8, D8, E8, F8, G♯8, A8
    /// MIDI note numbers: 105, 107, 108, 110, 112, 113, 116, 117
    /// Frequency range: ~3520.00 Hz to ~7040.00 Hz
    ///
    /// The A harmonic minor scale in octave 7 represents the extreme upper limit of conventional
    /// musical notation, approaching the upper threshold of human pitch perception. At these
    /// frequencies, exceeding 7kHz at its upper range, traditional musical concepts become entirely
    /// abstract as we approach the limits of human hearing. The augmented second interval between
    /// F8 and G♯8 exists purely as a spectral effect, beyond conventional melodic perception. This
    /// scale exists exclusively in computer music, advanced spectral synthesis, and theoretical
    /// contexts. In works exploring the furthest boundaries of sonic perception, these extreme
    /// registers function as what composer Gérard Grisey might describe as "acoustic particles" -
    /// creating ephemeral textures that transcend conventional musical parameters to function as
    /// pure spectral energy, operating at the furthest boundary of human pitch perception.
    pub static ref A7_HARMONIC_SCALE: Scale<8> = harmonic_scale(A7);

    /// A♯ harmonic minor scale in octave 7.
    ///
    /// Notes: A♯7, B♯7, C♯8, D♯8, E♯8, F♯8, G♯♯8, A♯8
    /// MIDI note numbers: 106, 108, 109, 111, 113, 114, 117, 118
    /// Frequency range: ~3729.31 Hz to ~7458.62 Hz
    ///
    /// The A♯ harmonic minor scale in octave 7 exists at the absolute upper boundary of conventional
    /// musical notation, well beyond practical applications. At these frequencies, exceeding 7.4kHz
    /// at its upper range, pitch relationships become entirely abstract spectral phenomena. The
    /// theoretical B♯7 would be enharmonically written as C8, and the double-sharp (G♯♯8) as A8,
    /// though at such extreme frequencies, traditional notation becomes purely conceptual. This scale
    /// exists exclusively in computer music and purely theoretical contexts. In advanced electronic
    /// compositions exploring the outer boundaries of human perception, these extreme registers
    /// function as what composer Iannis Xenakis might describe as "sonic photonic structures" -
    /// creating textures that exist at the absolute threshold of human pitch perception, where sound
    /// manifests primarily as abstract spectral energy rather than as discrete pitches.
    pub static ref ASHARP7_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP7);

    /// B harmonic minor scale in octave 7.
    ///
    /// Notes: B7, C♯8, D8, E8, F♯8, G8, A♯8, B8
    /// MIDI note numbers: 107, 109, 110, 112, 114, 115, 118, 119
    /// Frequency range: ~3951.07 Hz to ~7902.13 Hz
    ///
    /// The B harmonic minor scale in octave 7 spans an extreme high register at the absolute upper
    /// limit of conventional musical notation and human pitch perception. At these frequencies,
    /// approaching 8kHz, we reach the threshold where pitch discrimination becomes increasingly
    /// difficult and sound exists primarily as abstract spectral energy. The augmented second interval
    /// between G8 and A♯8 exists purely as a spectral phenomenon beyond conventional melodic or
    /// harmonic perception. This scale exists exclusively in computer music, advanced spectral
    /// synthesis, and theoretical contexts. In electronic compositions exploring perceptual boundaries,
    /// these extreme registers function as what composer Jonathan Harvey described as "the threshold
    /// where sound approaches light" - creating ethereal textures that exist at the furthest boundary
    /// of human perception, where musical notation reaches its practical limit and sound functions
    /// primarily as pure acoustic energy rather than as conventional musical material.
    pub static ref B7_HARMONIC_SCALE: Scale<8> = harmonic_scale(B7);
}

lazy_static! {
    /// C harmonic minor scale in octave 8.
    ///
    /// Notes: C8, D8, E♭8, F8, G8, A♭8, B8, C9
    /// MIDI note numbers: 108, 110, 111, 113, 115, 116, 119, 120
    /// Frequency range: ~4186.01 Hz to ~8372.02 Hz
    ///
    /// The C harmonic minor scale in octave 8 exists at the absolute upper threshold of human
    /// pitch perception and conventional musical notation. At these ultrasonic frequencies,
    /// approaching 8.4kHz at its upper range, traditional musical concepts become completely
    /// abstract as we approach the limits of human hearing. The augmented second interval between
    /// A♭8 and B8 exists purely as a spectral phenomenon beyond conventional melodic perception.
    /// This scale exists exclusively in specialized computer music, advanced spectral synthesis,
    /// and purely theoretical contexts. At these extreme frequencies, the character of the harmonic
    /// minor scale dissolves entirely into what composer Horacio Vaggione described as "micro-sonic
    /// structures" - creating textures that exist at the absolute boundary of human pitch perception,
    /// where sound functions entirely as pure spectral energy rather than as conventional musical
    /// material. These frequencies approach the realm where pitch perception begins to break down,
    /// and many adults cannot perceive the highest notes of this scale at all.
    pub static ref C8_HARMONIC_SCALE: Scale<8> = harmonic_scale(C8);

    /// C♯ harmonic minor scale in octave 8.
    ///
    /// Notes: C♯8, D♯8, E8, F♯8, G♯8, A8, B♯8, C♯9
    /// MIDI note numbers: 109, 111, 112, 114, 116, 117, 120, 121
    /// Frequency range: ~4434.92 Hz to ~8869.84 Hz
    ///
    /// The C♯ harmonic minor scale in octave 8 occupies an extreme ultrasonic register at the
    /// absolute boundary of human hearing capabilities. At these frequencies, approaching 9kHz
    /// at its upper limit, pitch perception becomes highly uncertain for most listeners, with
    /// traditional musical relationships dissolving entirely into spectral phenomena. The theoretical
    /// B♯8 would extend beyond the standard MIDI range (which ends at 127 or G9). This scale
    /// exists exclusively in advanced computer music, spectral synthesis research, and purely
    /// theoretical contexts. At these frequencies, the harmonic minor's distinctive intervals have
    /// no practical musical meaning and function solely as what composer Curtis Roads termed
    /// "microsound" - creating textures that exist at the furthest boundary of human auditory
    /// perception, where sound becomes pure spectral energy and many listeners cannot perceive
    /// the highest pitches at all, particularly older adults with age-related hearing loss.
    pub static ref CSHARP8_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP8);

    /// D harmonic minor scale in octave 8.
    ///
    /// Notes: D8, E8, F8, G8, A8, B♭8, C♯9, D9
    /// MIDI note numbers: 110, 112, 113, 115, 117, 118, 121, 122
    /// Frequency range: ~4698.64 Hz to ~9397.27 Hz
    ///
    /// The D harmonic minor scale in octave 8 represents an extreme ultrasonic register at the
    /// absolute limit of human hearing capability. At these frequencies, approaching 9.4kHz at
    /// its upper extremity, traditional pitch perception becomes highly uncertain or impossible
    /// for many listeners, particularly adults, with musical relationships transformed entirely
    /// into abstract spectral phenomena. This scale exists purely in theoretical contexts,
    /// advanced computer music, and acoustic research. The augmented second interval between B♭8
    /// and C♯9 has no practical melodic or harmonic meaning, functioning solely as a theoretical
    /// spectral relationship. These ultrahigh frequencies operate in what composer Iannis Xenakis
    /// described as the realm of "sonic particles" - creating textures that exist at the furthest
    /// boundary of human perception, where conventional musical notation reaches its absolute limit
    /// and sound functions exclusively as abstract spectral energy, approaching the threshold where
    /// many listeners perceive nothing at all.
    pub static ref D8_HARMONIC_SCALE: Scale<8> = harmonic_scale(D8);

    /// D♯ harmonic minor scale in octave 8.
    ///
    /// Notes: D♯8, E♯8, F♯8, G♯8, A♯8, B8, C♯♯9, D♯9
    /// MIDI note numbers: 111, 113, 114, 116, 118, 119, 122, 123
    /// Frequency range: ~4978.03 Hz to ~9956.06 Hz
    ///
    /// The D♯ harmonic minor scale in octave 8 exists at the extreme ultrasonic threshold of human
    /// hearing capabilities. At these frequencies, approaching 10kHz at its upper boundary,
    /// traditional pitch perception becomes impossible for many listeners, with musical relationships
    /// dissolving entirely into abstract spectral energy. The theoretical double-sharp (C♯♯9) would
    /// be enharmonically represented as D9, though at such extreme frequencies, traditional notation
    /// becomes purely theoretical. This scale exists exclusively in specialized computer music,
    /// acoustic research contexts, and purely theoretical applications. At these frequencies, the
    /// harmonic minor's distinctive intervals have no practical musical meaning, functioning solely
    /// as what composer Barry Truax termed "acoustic microscopy" - creating textures that exist at
    /// the absolute boundary of human auditory perception, where conventional musical parameters
    /// disappear entirely and many listeners, especially adults, perceive only the faintest whispers
    /// of sound if anything at all.
    pub static ref DSHARP8_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP8);

    /// E harmonic minor scale in octave 8.
    ///
    /// Notes: E8, F♯8, G8, A8, B8, C9, D♯9, E9
    /// MIDI note numbers: 112, 114, 115, 117, 119, 120, 123, 124
    /// Frequency range: ~5274.04 Hz to ~10548.08 Hz
    ///
    /// The E harmonic minor scale in octave 8 represents an ultrasonic register at the furthest
    /// threshold of human hearing. At these frequencies, exceeding 10.5kHz at its upper limit,
    /// pitch perception becomes entirely theoretical for most listeners, with traditional musical
    /// relationships completely dissolved. This scale exists exclusively in acoustic research,
    /// specialized computer music contexts, and purely theoretical applications. The augmented
    /// second interval between C9 and D♯9 exists purely as an abstract spectral relationship with
    /// no practical musical function. At these extreme frequencies, sound operates in what composer
    /// Trevor Wishart described as "the realm beyond pitch" - creating textures that exist at the
    /// absolute boundary of human perception, where only the youngest listeners with perfect hearing
    /// may perceive the highest pitches as the faintest whisper of sound. For most adults, especially
    /// those over 40, the upper notes of this scale are completely inaudible, existing purely in
    /// theoretical mathematical relationships.
    pub static ref E8_HARMONIC_SCALE: Scale<8> = harmonic_scale(E8);

    /// F harmonic minor scale in octave 8.
    ///
    /// Notes: F8, G8, A♭8, B♭8, C9, D♭9, E9, F9
    /// MIDI note numbers: 113, 115, 116, 118, 120, 121, 124, 125
    /// Frequency range: ~5587.65 Hz to ~11175.30 Hz
    ///
    /// The F harmonic minor scale in octave 8 exists at the extreme ultrasonic threshold where
    /// human pitch perception reaches its absolute limit. At these frequencies, exceeding 11kHz
    /// at its upper range, traditional musical concepts become entirely theoretical, with
    /// conventional pitch perception impossible for most human listeners. This scale exists
    /// exclusively in acoustic research, specialized computer music, and purely theoretical
    /// contexts. The augmented second interval between D♭9 and E9 has no practical musical
    /// meaning, functioning solely as an abstract mathematical relationship. At these frequencies,
    /// sound operates in what composer Jean-Claude Risset described as "the region where pitch
    /// becomes pure timbre" - creating textures that exist at the absolute limit of human auditory
    /// perception, where conventional musical notation reaches its ultimate boundary. The highest
    /// notes of this scale are inaudible to most adult listeners, with only those having exceptional
    /// hearing potentially perceiving the faintest traces of sound.
    pub static ref F8_HARMONIC_SCALE: Scale<8> = harmonic_scale(F8);

    /// F♯ harmonic minor scale in octave 8.
    ///
    /// Notes: F♯8, G♯8, A8, B8, C♯9, D9, E♯9, F♯9
    /// MIDI note numbers: 114, 116, 117, 119, 121, 122, 125, 126
    /// Frequency range: ~5919.91 Hz to ~11839.82 Hz
    ///
    /// The F♯ harmonic minor scale in octave 8 represents an ultrasonic register approaching the
    /// uppermost limit of conventional musical notation and human hearing capability. At these
    /// frequencies, nearly 12kHz at its highest point, traditional pitch perception becomes
    /// entirely theoretical, as these frequencies exceed the hearing range of many adults. This
    /// scale exists exclusively in acoustic research, specialized computer music, and purely
    /// theoretical contexts. The augmented second interval between D9 and E♯9 functions solely
    /// as an abstract mathematical relationship with no practical musical meaning. At these extreme
    /// frequencies, sound operates in what composer Agostino Di Scipio termed "the boundary of
    /// sonic perception" - creating textures that exist at the absolute threshold of human auditory
    /// capability. The highest notes of this scale are completely inaudible to most adult listeners,
    /// existing purely as mathematical constructs rather than perceivable musical tones.
    pub static ref FSHARP8_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP8);

    /// G harmonic minor scale in octave 8.
    ///
    /// Notes: G8, A8, B♭8, C9, D9, E♭9, F♯9, G9
    /// MIDI note numbers: 115, 117, 118, 120, 122, 123, 126, 127
    /// Frequency range: ~6271.93 Hz to ~12543.85 Hz
    ///
    /// The G harmonic minor scale in octave 8 exists at the absolute uppermost limit of
    /// conventional musical notation, with G9 (MIDI note 127) representing the highest note in
    /// the standard MIDI specification. At these ultrasonic frequencies, exceeding 12.5kHz at
    /// its highest point, traditional musical concepts become purely theoretical, as these
    /// frequencies approach or exceed the upper threshold of human hearing, particularly for
    /// adults. This scale exists exclusively in acoustic research and purely theoretical contexts.
    /// The augmented second interval between E♭9 and F♯9 has no practical musical meaning,
    /// functioning solely as an abstract mathematical relationship. At these extreme frequencies,
    /// sound exists in what acoustician Hermann Helmholtz first described as the threshold where
    /// "tone disappears entirely" - creating textures that exist at the absolute limit of human
    /// auditory perception, where conventional musical notation reaches its ultimate boundary.
    /// The highest notes of this scale are inaudible to the vast majority of listeners, existing
    /// purely as mathematical constructs rather than as perceivable musical elements.
    pub static ref G8_HARMONIC_SCALE: Scale<8> = harmonic_scale(G8);
}
