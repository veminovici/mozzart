use crate::{constants::*, minor_scale};
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub const MINOR_SCALE_STEPS: [Interval; 7] = [TONE, SEMITONE, TONE, TONE, SEMITONE, TONE, TONE];

lazy_static! {
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
    pub static ref C_MINOR_SCALE: Scale<8> = minor_scale(C);
    pub static ref CSHARP_MINOR_SCALE: Scale<8> = minor_scale(CSHARP);
    pub static ref D_MINOR_SCALE: Scale<8> = minor_scale(D);
    pub static ref DSHARP_MINOR_SCALE: Scale<8> = minor_scale(DSHARP);
    pub static ref E_MINOR_SCALE: Scale<8> = minor_scale(E);
    pub static ref F_MINOR_SCALE: Scale<8> = minor_scale(F);
    pub static ref FSHARP_MINOR_SCALE: Scale<8> = minor_scale(FSHARP);
    pub static ref G_MINOR_SCALE: Scale<8> = minor_scale(G);
    pub static ref GSHARP_MINOR_SCALE: Scale<8> = minor_scale(GSHARP);
    pub static ref A_MINOR_SCALE: Scale<8> = minor_scale(A);
    pub static ref ASHARP_MINOR_SCALE: Scale<8> = minor_scale(ASHARP);
    pub static ref B_MINOR_SCALE: Scale<8> = minor_scale(B);
}

lazy_static! {
    pub static ref C0_MINOR_SCALE: Scale<8> = minor_scale(C0);
    pub static ref CSHARP0_MINOR_SCALE: Scale<8> = minor_scale(CSHARP0);
    pub static ref D0_MINOR_SCALE: Scale<8> = minor_scale(D0);
    pub static ref DSHARP0_MINOR_SCALE: Scale<8> = minor_scale(DSHARP0);
    pub static ref E0_MINOR_SCALE: Scale<8> = minor_scale(E0);
    pub static ref F0_MINOR_SCALE: Scale<8> = minor_scale(F0);
    pub static ref FSHARP0_MINOR_SCALE: Scale<8> = minor_scale(FSHARP0);
    pub static ref G0_MINOR_SCALE: Scale<8> = minor_scale(G0);
    pub static ref GSHARP0_MINOR_SCALE: Scale<8> = minor_scale(GSHARP0);
    pub static ref A0_MINOR_SCALE: Scale<8> = minor_scale(A0);
    pub static ref ASHARP0_MINOR_SCALE: Scale<8> = minor_scale(ASHARP0);
    pub static ref B0_MINOR_SCALE: Scale<8> = minor_scale(B0);
}

lazy_static! {
    pub static ref C1_MINOR_SCALE: Scale<8> = minor_scale(C1);
    pub static ref CSHARP1_MINOR_SCALE: Scale<8> = minor_scale(CSHARP1);
    pub static ref D1_MINOR_SCALE: Scale<8> = minor_scale(D1);
    pub static ref DSHARP1_MINOR_SCALE: Scale<8> = minor_scale(DSHARP1);
    pub static ref E1_MINOR_SCALE: Scale<8> = minor_scale(E1);
    pub static ref F1_MINOR_SCALE: Scale<8> = minor_scale(F1);
    pub static ref FSHARP1_MINOR_SCALE: Scale<8> = minor_scale(FSHARP1);
    pub static ref G1_MINOR_SCALE: Scale<8> = minor_scale(G1);
    pub static ref GSHARP1_MINOR_SCALE: Scale<8> = minor_scale(GSHARP1);
    pub static ref A1_MINOR_SCALE: Scale<8> = minor_scale(A1);
    pub static ref ASHARP1_MINOR_SCALE: Scale<8> = minor_scale(ASHARP1);
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
