use crate::{constants::*, melodic_scale};
use crate::{Interval, Pitch, Scale};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub const MELODIC_SCALE_STEPS: [Interval; 7] = [TONE, SEMITONE, TONE, TONE, TONE, TONE, SEMITONE];

lazy_static! {
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
    pub static ref C_MELODIC_SCALE: Scale<8> = melodic_scale(C);
    pub static ref CSHARP_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP);
    pub static ref D_MELODIC_SCALE: Scale<8> = melodic_scale(D);
    pub static ref DSHARP_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP);
    pub static ref E_MELODIC_SCALE: Scale<8> = melodic_scale(E);
    pub static ref F_MELODIC_SCALE: Scale<8> = melodic_scale(F);
    pub static ref FSHARP_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP);
    pub static ref G_MELODIC_SCALE: Scale<8> = melodic_scale(G);
    pub static ref GSHARP_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP);
    pub static ref A_MELODIC_SCALE: Scale<8> = melodic_scale(A);
    pub static ref ASHARP_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP);
    pub static ref B_MELODIC_SCALE: Scale<8> = melodic_scale(B);
}

lazy_static! {
    pub static ref C0_MELODIC_SCALE: Scale<8> = melodic_scale(C0);
    pub static ref CSHARP0_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP0);
    pub static ref D0_MELODIC_SCALE: Scale<8> = melodic_scale(D0);
    pub static ref DSHARP0_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP0);
    pub static ref E0_MELODIC_SCALE: Scale<8> = melodic_scale(E0);
    pub static ref F0_MELODIC_SCALE: Scale<8> = melodic_scale(F0);
    pub static ref FSHARP0_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP0);
    pub static ref G0_MELODIC_SCALE: Scale<8> = melodic_scale(G0);
    pub static ref GSHARP0_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP0);
    pub static ref A0_MELODIC_SCALE: Scale<8> = melodic_scale(A0);
    pub static ref ASHARP0_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP0);
    pub static ref B0_MELODIC_SCALE: Scale<8> = melodic_scale(B0);
}

lazy_static! {
    pub static ref C1_MELODIC_SCALE: Scale<8> = melodic_scale(C1);
    pub static ref CSHARP1_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP1);
    pub static ref D1_MELODIC_SCALE: Scale<8> = melodic_scale(D1);
    pub static ref DSHARP1_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP1);
    pub static ref E1_MELODIC_SCALE: Scale<8> = melodic_scale(E1);
    pub static ref F1_MELODIC_SCALE: Scale<8> = melodic_scale(F1);
    pub static ref FSHARP1_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP1);
    pub static ref G1_MELODIC_SCALE: Scale<8> = melodic_scale(G1);
    pub static ref GSHARP1_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP1);
    pub static ref A1_MELODIC_SCALE: Scale<8> = melodic_scale(A1);
    pub static ref ASHARP1_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP1);
    pub static ref B1_MELODIC_SCALE: Scale<8> = melodic_scale(B1);
}

lazy_static! {
    pub static ref C2_MELODIC_SCALE: Scale<8> = melodic_scale(C2);
    pub static ref CSHARP2_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP2);
    pub static ref D2_MELODIC_SCALE: Scale<8> = melodic_scale(D2);
    pub static ref DSHARP2_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP2);
    pub static ref E2_MELODIC_SCALE: Scale<8> = melodic_scale(E2);
    pub static ref F2_MELODIC_SCALE: Scale<8> = melodic_scale(F2);
    pub static ref FSHARP2_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP2);
    pub static ref G2_MELODIC_SCALE: Scale<8> = melodic_scale(G2);
    pub static ref GSHARP2_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP2);
    pub static ref A2_MELODIC_SCALE: Scale<8> = melodic_scale(A2);
    pub static ref ASHARP2_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP2);
    pub static ref B2_MELODIC_SCALE: Scale<8> = melodic_scale(B2);
}

lazy_static! {
    pub static ref C3_MELODIC_SCALE: Scale<8> = melodic_scale(C3);
    pub static ref CSHARP3_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP3);
    pub static ref D3_MELODIC_SCALE: Scale<8> = melodic_scale(D3);
    pub static ref DSHARP3_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP3);
    pub static ref E3_MELODIC_SCALE: Scale<8> = melodic_scale(E3);
    pub static ref F3_MELODIC_SCALE: Scale<8> = melodic_scale(F3);
    pub static ref FSHARP3_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP3);
    pub static ref G3_MELODIC_SCALE: Scale<8> = melodic_scale(G3);
    pub static ref GSHARP3_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP3);
    pub static ref A3_MELODIC_SCALE: Scale<8> = melodic_scale(A3);
    pub static ref ASHARP3_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP3);
    pub static ref B3_MELODIC_SCALE: Scale<8> = melodic_scale(B3);
}

lazy_static! {
    pub static ref C4_MELODIC_SCALE: Scale<8> = melodic_scale(C4);
    pub static ref CSHARP4_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP4);
    pub static ref D4_MELODIC_SCALE: Scale<8> = melodic_scale(D4);
    pub static ref DSHARP4_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP4);
    pub static ref E4_MELODIC_SCALE: Scale<8> = melodic_scale(E4);
    pub static ref F4_MELODIC_SCALE: Scale<8> = melodic_scale(F4);
    pub static ref FSHARP4_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP4);
    pub static ref G4_MELODIC_SCALE: Scale<8> = melodic_scale(G4);
    pub static ref GSHARP4_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP4);
    pub static ref A4_MELODIC_SCALE: Scale<8> = melodic_scale(A4);
    pub static ref ASHARP4_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP4);
    pub static ref B4_MELODIC_SCALE: Scale<8> = melodic_scale(B4);
}

lazy_static! {
    pub static ref C5_MELODIC_SCALE: Scale<8> = melodic_scale(C5);
    pub static ref CSHARP5_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP5);
    pub static ref D5_MELODIC_SCALE: Scale<8> = melodic_scale(D5);
    pub static ref DSHARP5_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP5);
    pub static ref E5_MELODIC_SCALE: Scale<8> = melodic_scale(E5);
    pub static ref F5_MELODIC_SCALE: Scale<8> = melodic_scale(F5);
    pub static ref FSHARP5_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP5);
    pub static ref G5_MELODIC_SCALE: Scale<8> = melodic_scale(G5);
    pub static ref GSHARP5_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP5);
    pub static ref A5_MELODIC_SCALE: Scale<8> = melodic_scale(A5);
    pub static ref ASHARP5_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP5);
    pub static ref B5_MELODIC_SCALE: Scale<8> = melodic_scale(B5);
}

lazy_static! {
    pub static ref C6_MELODIC_SCALE: Scale<8> = melodic_scale(C6);
    pub static ref CSHARP6_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP6);
    pub static ref D6_MELODIC_SCALE: Scale<8> = melodic_scale(D6);
    pub static ref DSHARP6_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP6);
    pub static ref E6_MELODIC_SCALE: Scale<8> = melodic_scale(E6);
    pub static ref F6_MELODIC_SCALE: Scale<8> = melodic_scale(F6);
    pub static ref FSHARP6_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP6);
    pub static ref G6_MELODIC_SCALE: Scale<8> = melodic_scale(G6);
    pub static ref GSHARP6_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP6);
    pub static ref A6_MELODIC_SCALE: Scale<8> = melodic_scale(A6);
    pub static ref ASHARP6_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP6);
    pub static ref B6_MELODIC_SCALE: Scale<8> = melodic_scale(B6);
}

lazy_static! {
    pub static ref C7_MELODIC_SCALE: Scale<8> = melodic_scale(C7);
    pub static ref CSHARP7_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP7);
    pub static ref D7_MELODIC_SCALE: Scale<8> = melodic_scale(D7);
    pub static ref DSHARP7_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP7);
    pub static ref E7_MELODIC_SCALE: Scale<8> = melodic_scale(E7);
    pub static ref F7_MELODIC_SCALE: Scale<8> = melodic_scale(F7);
    pub static ref FSHARP7_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP7);
    pub static ref G7_MELODIC_SCALE: Scale<8> = melodic_scale(G7);
    pub static ref GSHARP7_MELODIC_SCALE: Scale<8> = melodic_scale(GSHARP7);
    pub static ref A7_MELODIC_SCALE: Scale<8> = melodic_scale(A7);
    pub static ref ASHARP7_MELODIC_SCALE: Scale<8> = melodic_scale(ASHARP7);
    pub static ref B7_MELODIC_SCALE: Scale<8> = melodic_scale(B7);
}

lazy_static! {
    pub static ref C8_MELODIC_SCALE: Scale<8> = melodic_scale(C8);
    pub static ref CSHARP8_MELODIC_SCALE: Scale<8> = melodic_scale(CSHARP8);
    pub static ref D8_MELODIC_SCALE: Scale<8> = melodic_scale(D8);
    pub static ref DSHARP8_MELODIC_SCALE: Scale<8> = melodic_scale(DSHARP8);
    pub static ref E8_MELODIC_SCALE: Scale<8> = melodic_scale(E8);
    pub static ref F8_MELODIC_SCALE: Scale<8> = melodic_scale(F8);
    pub static ref FSHARP8_MELODIC_SCALE: Scale<8> = melodic_scale(FSHARP8);
    pub static ref G8_MELODIC_SCALE: Scale<8> = melodic_scale(G8);
}
