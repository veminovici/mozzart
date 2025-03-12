use crate::{constants::*, harmonic_scale};
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub const HARMONIC_SCALE_STEPS: [Interval; 7] =
    [TONE, SEMITONE, TONE, TONE, SEMITONE, MINOR_THIRD, SEMITONE];

lazy_static! {
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
    pub static ref C_HARMONIC_SCALE: Scale<8> = harmonic_scale(C);
    pub static ref CSHARP_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP);
    pub static ref D_HARMONIC_SCALE: Scale<8> = harmonic_scale(D);
    pub static ref DSHARP_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP);
    pub static ref E_HARMONIC_SCALE: Scale<8> = harmonic_scale(E);
    pub static ref F_HARMONIC_SCALE: Scale<8> = harmonic_scale(F);
    pub static ref FSHARP_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP);
    pub static ref G_HARMONIC_SCALE: Scale<8> = harmonic_scale(G);
    pub static ref GSHARP_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP);
    pub static ref A_HARMONIC_SCALE: Scale<8> = harmonic_scale(A);
    pub static ref ASHARP_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP);
    pub static ref B_HARMONIC_SCALE: Scale<8> = harmonic_scale(B);
}

lazy_static! {
    pub static ref C0_HARMONIC_SCALE: Scale<8> = harmonic_scale(C0);
    pub static ref CSHARP0_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP0);
    pub static ref D0_HARMONIC_SCALE: Scale<8> = harmonic_scale(D0);
    pub static ref DSHARP0_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP0);
    pub static ref E0_HARMONIC_SCALE: Scale<8> = harmonic_scale(E0);
    pub static ref F0_HARMONIC_SCALE: Scale<8> = harmonic_scale(F0);
    pub static ref FSHARP0_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP0);
    pub static ref G0_HARMONIC_SCALE: Scale<8> = harmonic_scale(G0);
    pub static ref GSHARP0_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP0);
    pub static ref A0_HARMONIC_SCALE: Scale<8> = harmonic_scale(A0);
    pub static ref ASHARP0_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP0);
    pub static ref B0_HARMONIC_SCALE: Scale<8> = harmonic_scale(B0);
}

lazy_static! {
    pub static ref C1_HARMONIC_SCALE: Scale<8> = harmonic_scale(C1);
    pub static ref CSHARP1_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP1);
    pub static ref D1_HARMONIC_SCALE: Scale<8> = harmonic_scale(D1);
    pub static ref DSHARP1_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP1);
    pub static ref E1_HARMONIC_SCALE: Scale<8> = harmonic_scale(E1);
    pub static ref F1_HARMONIC_SCALE: Scale<8> = harmonic_scale(F1);
    pub static ref FSHARP1_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP1);
    pub static ref G1_HARMONIC_SCALE: Scale<8> = harmonic_scale(G1);
    pub static ref GSHARP1_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP1);
    pub static ref A1_HARMONIC_SCALE: Scale<8> = harmonic_scale(A1);
    pub static ref ASHARP1_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP1);
    pub static ref B1_HARMONIC_SCALE: Scale<8> = harmonic_scale(B1);
}

lazy_static! {
    pub static ref C2_HARMONIC_SCALE: Scale<8> = harmonic_scale(C2);
    pub static ref CSHARP2_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP2);
    pub static ref D2_HARMONIC_SCALE: Scale<8> = harmonic_scale(D2);
    pub static ref DSHARP2_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP2);
    pub static ref E2_HARMONIC_SCALE: Scale<8> = harmonic_scale(E2);
    pub static ref F2_HARMONIC_SCALE: Scale<8> = harmonic_scale(F2);
    pub static ref FSHARP2_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP2);
    pub static ref G2_HARMONIC_SCALE: Scale<8> = harmonic_scale(G2);
    pub static ref GSHARP2_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP2);
    pub static ref A2_HARMONIC_SCALE: Scale<8> = harmonic_scale(A2);
    pub static ref ASHARP2_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP2);
    pub static ref B2_HARMONIC_SCALE: Scale<8> = harmonic_scale(B2);
}

lazy_static! {
    pub static ref C3_HARMONIC_SCALE: Scale<8> = harmonic_scale(C3);
    pub static ref CSHARP3_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP3);
    pub static ref D3_HARMONIC_SCALE: Scale<8> = harmonic_scale(D3);
    pub static ref DSHARP3_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP3);
    pub static ref E3_HARMONIC_SCALE: Scale<8> = harmonic_scale(E3);
    pub static ref F3_HARMONIC_SCALE: Scale<8> = harmonic_scale(F3);
    pub static ref FSHARP3_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP3);
    pub static ref G3_HARMONIC_SCALE: Scale<8> = harmonic_scale(G3);
    pub static ref GSHARP3_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP3);
    pub static ref A3_HARMONIC_SCALE: Scale<8> = harmonic_scale(A3);
    pub static ref ASHARP3_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP3);
    pub static ref B3_HARMONIC_SCALE: Scale<8> = harmonic_scale(B3);
}

lazy_static! {
    pub static ref C4_HARMONIC_SCALE: Scale<8> = harmonic_scale(C4);
    pub static ref CSHARP4_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP4);
    pub static ref D4_HARMONIC_SCALE: Scale<8> = harmonic_scale(D4);
    pub static ref DSHARP4_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP4);
    pub static ref E4_HARMONIC_SCALE: Scale<8> = harmonic_scale(E4);
    pub static ref F4_HARMONIC_SCALE: Scale<8> = harmonic_scale(F4);
    pub static ref FSHARP4_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP4);
    pub static ref G4_HARMONIC_SCALE: Scale<8> = harmonic_scale(G4);
    pub static ref GSHARP4_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP4);
    pub static ref A4_HARMONIC_SCALE: Scale<8> = harmonic_scale(A4);
    pub static ref ASHARP4_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP4);
    pub static ref B4_HARMONIC_SCALE: Scale<8> = harmonic_scale(B4);
}

lazy_static! {
    pub static ref C5_HARMONIC_SCALE: Scale<8> = harmonic_scale(C5);
    pub static ref CSHARP5_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP5);
    pub static ref D5_HARMONIC_SCALE: Scale<8> = harmonic_scale(D5);
    pub static ref DSHARP5_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP5);
    pub static ref E5_HARMONIC_SCALE: Scale<8> = harmonic_scale(E5);
    pub static ref F5_HARMONIC_SCALE: Scale<8> = harmonic_scale(F5);
    pub static ref FSHARP5_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP5);
    pub static ref G5_HARMONIC_SCALE: Scale<8> = harmonic_scale(G5);
    pub static ref GSHARP5_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP5);
    pub static ref A5_HARMONIC_SCALE: Scale<8> = harmonic_scale(A5);
    pub static ref ASHARP5_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP5);
    pub static ref B5_HARMONIC_SCALE: Scale<8> = harmonic_scale(B5);
}

lazy_static! {
    pub static ref C6_HARMONIC_SCALE: Scale<8> = harmonic_scale(C6);
    pub static ref CSHARP6_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP6);
    pub static ref D6_HARMONIC_SCALE: Scale<8> = harmonic_scale(D6);
    pub static ref DSHARP6_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP6);
    pub static ref E6_HARMONIC_SCALE: Scale<8> = harmonic_scale(E6);
    pub static ref F6_HARMONIC_SCALE: Scale<8> = harmonic_scale(F6);
    pub static ref FSHARP6_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP6);
    pub static ref G6_HARMONIC_SCALE: Scale<8> = harmonic_scale(G6);
    pub static ref GSHARP6_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP6);
    pub static ref A6_HARMONIC_SCALE: Scale<8> = harmonic_scale(A6);
    pub static ref ASHARP6_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP6);
    pub static ref B6_HARMONIC_SCALE: Scale<8> = harmonic_scale(B6);
}

lazy_static! {
    pub static ref C7_HARMONIC_SCALE: Scale<8> = harmonic_scale(C7);
    pub static ref CSHARP7_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP7);
    pub static ref D7_HARMONIC_SCALE: Scale<8> = harmonic_scale(D7);
    pub static ref DSHARP7_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP7);
    pub static ref E7_HARMONIC_SCALE: Scale<8> = harmonic_scale(E7);
    pub static ref F7_HARMONIC_SCALE: Scale<8> = harmonic_scale(F7);
    pub static ref FSHARP7_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP7);
    pub static ref G7_HARMONIC_SCALE: Scale<8> = harmonic_scale(G7);
    pub static ref GSHARP7_HARMONIC_SCALE: Scale<8> = harmonic_scale(GSHARP7);
    pub static ref A7_HARMONIC_SCALE: Scale<8> = harmonic_scale(A7);
    pub static ref ASHARP7_HARMONIC_SCALE: Scale<8> = harmonic_scale(ASHARP7);
    pub static ref B7_HARMONIC_SCALE: Scale<8> = harmonic_scale(B7);
}

lazy_static! {
    pub static ref C8_HARMONIC_SCALE: Scale<8> = harmonic_scale(C8);
    pub static ref CSHARP8_HARMONIC_SCALE: Scale<8> = harmonic_scale(CSHARP8);
    pub static ref D8_HARMONIC_SCALE: Scale<8> = harmonic_scale(D8);
    pub static ref DSHARP8_HARMONIC_SCALE: Scale<8> = harmonic_scale(DSHARP8);
    pub static ref E8_HARMONIC_SCALE: Scale<8> = harmonic_scale(E8);
    pub static ref F8_HARMONIC_SCALE: Scale<8> = harmonic_scale(F8);
    pub static ref FSHARP8_HARMONIC_SCALE: Scale<8> = harmonic_scale(FSHARP8);
    pub static ref G8_HARMONIC_SCALE: Scale<8> = harmonic_scale(G8);
}
