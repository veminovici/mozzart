use crate::{constants::*, major_scale};
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub const MAJOR_SCALE_STEPS: [Interval; 7] = [TONE, TONE, SEMITONE, TONE, TONE, TONE, SEMITONE];

lazy_static! {
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
    pub static ref C_MAJOR_SCALE: Scale<8> = major_scale(C4);
    pub static ref C_SHARP_MAJOR_SCALE: Scale<8> = major_scale(CSHARP4);
    pub static ref D_MAJOR_SCALE: Scale<8> = major_scale(D4);
    pub static ref D_SHARP_MAJOR_SCALE: Scale<8> = major_scale(DSHARP4);
    pub static ref E_MAJOR_SCALE: Scale<8> = major_scale(E4);
    pub static ref F_MAJOR_SCALE: Scale<8> = major_scale(F4);
    pub static ref F_SHARP_MAJOR_SCALE: Scale<8> = major_scale(FSHARP4);
    pub static ref G_MAJOR_SCALE: Scale<8> = major_scale(G4);
    pub static ref G_SHARP_MAJOR_SCALE: Scale<8> = major_scale(GSHARP4);
    pub static ref A_MAJOR_SCALE: Scale<8> = major_scale(A4);
    pub static ref A_SHARP_MAJOR_SCALE: Scale<8> = major_scale(ASHARP4);
    pub static ref B_MAJOR_SCALE: Scale<8> = major_scale(B4);
}

lazy_static! {
    pub static ref C0_MAJOR_SCALE: Scale<8> = major_scale(C0);
    pub static ref CSHARP0_MAJOR_SCALE: Scale<8> = major_scale(CSHARP0);
    pub static ref D0_MAJOR_SCALE: Scale<8> = major_scale(D0);
    pub static ref DSHARP0_MAJOR_SCALE: Scale<8> = major_scale(DSHARP0);
    pub static ref E0_MAJOR_SCALE: Scale<8> = major_scale(E0);
    pub static ref F0_MAJOR_SCALE: Scale<8> = major_scale(F0);
    pub static ref FSHARP0_MAJOR_SCALE: Scale<8> = major_scale(FSHARP0);
    pub static ref G0_MAJOR_SCALE: Scale<8> = major_scale(G0);
    pub static ref GSHARP0_MAJOR_SCALE: Scale<8> = major_scale(GSHARP0);
    pub static ref A0_MAJOR_SCALE: Scale<8> = major_scale(A0);
    pub static ref ASHARP0_MAJOR_SCALE: Scale<8> = major_scale(ASHARP0);
    pub static ref B0_MAJOR_SCALE: Scale<8> = major_scale(B0);
}

lazy_static! {
    pub static ref C1_MAJOR_SCALE: Scale<8> = major_scale(C1);
    pub static ref CSHARP1_MAJOR_SCALE: Scale<8> = major_scale(CSHARP1);
    pub static ref D1_MAJOR_SCALE: Scale<8> = major_scale(D1);
    pub static ref DSHARP1_MAJOR_SCALE: Scale<8> = major_scale(DSHARP1);
    pub static ref E1_MAJOR_SCALE: Scale<8> = major_scale(E1);
    pub static ref F1_MAJOR_SCALE: Scale<8> = major_scale(F1);
    pub static ref FSHARP1_MAJOR_SCALE: Scale<8> = major_scale(FSHARP1);
    pub static ref G1_MAJOR_SCALE: Scale<8> = major_scale(G1);
    pub static ref GSHARP1_MAJOR_SCALE: Scale<8> = major_scale(GSHARP1);
    pub static ref A1_MAJOR_SCALE: Scale<8> = major_scale(A1);
    pub static ref ASHARP1_MAJOR_SCALE: Scale<8> = major_scale(ASHARP1);
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
