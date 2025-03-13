use mozzart_std::constants::*;
use mozzart_std::*;

fn main() {
    // Create a C major scale
    let c_major_scale = C4.major_scale();
    {
        // Result: [C4, D4, E4, F4, G4, A4, B4, C5]
        let s = NamedSlice::new("C Major".to_string(), c_major_scale.notes());
        println!("{:?}", s);
    }

    let c_major_triad = C4.major_triad_chord();
    {
        // Result: [C4, E4, G4]
        let s = NamedSlice::new("C Major Triad".to_string(), c_major_triad.notes());
        println!("{:?}", s);
    }
}
