//! Musical pitch constants using MIDI note numbers
//!
//! This module provides constants for all musical pitches from C0 to G9,
//! covering the entire MIDI note range (0-127). Each pitch is represented
//! by its MIDI note number, with middle C (C4) being 60.
//!
//! # Frequency Ranges
//! - Octave 0: 16.35 Hz to 30.87 Hz
//! - Octave 1: 32.70 Hz to 61.74 Hz
//! - Octave 2: 65.41 Hz to 123.47 Hz
//! - Octave 3: 130.81 Hz to 246.94 Hz
//! - Octave 4: 261.63 Hz (middle C) to 493.88 Hz
//! - Octave 5: 523.25 Hz to 987.77 Hz
//! - Octave 6: 1046.50 Hz to 1975.53 Hz
//! - Octave 7: 2093.00 Hz to 3951.07 Hz
//! - Octave 8: 4186.01 Hz to 7902.13 Hz
//! - Octave 9: 8372.02 Hz to 15804.27 Hz
//!
//! # Key Reference Points
//! - A4 (concert pitch) = 440 Hz (MIDI note 69)
//! - Middle C (C4) = 261.63 Hz (MIDI note 60)

use crate::Pitch;

/// Pitch class constants (octave-independent)
///
/// These represent the twelve chromatic pitches in their base form,
/// without octave information. Each pitch class maps to a MIDI note
/// number from 0 to 11.
pub const C: Pitch = Pitch::new(0);
pub const CSHARP: Pitch = Pitch::new(1);
pub const DFLAT: Pitch = CSHARP;
pub const D: Pitch = Pitch::new(2);
pub const DSHARP: Pitch = Pitch::new(3);
pub const EFLAT: Pitch = DSHARP;
pub const E: Pitch = Pitch::new(4);
pub const F: Pitch = Pitch::new(5);
pub const FSHARP: Pitch = Pitch::new(6);
pub const GFLAT: Pitch = FSHARP;
pub const G: Pitch = Pitch::new(7);
pub const GSHARP: Pitch = Pitch::new(8);
pub const AFLAT: Pitch = GSHARP;
pub const A: Pitch = Pitch::new(9);
pub const ASHARP: Pitch = Pitch::new(10);
pub const BFLAT: Pitch = ASHARP;
pub const B: Pitch = Pitch::new(11);

/// Pitches in octave 0 (16.35 Hz to 30.87 Hz)
pub const C0: Pitch = Pitch::new(12);
pub const CSHARP0: Pitch = Pitch::new(13);
pub const DFLAT0: Pitch = CSHARP0;
pub const D0: Pitch = Pitch::new(14);
pub const DSHARP0: Pitch = Pitch::new(15);
pub const EFLAT0: Pitch = DSHARP0;
pub const E0: Pitch = Pitch::new(16);
pub const F0: Pitch = Pitch::new(17);
pub const FSHARP0: Pitch = Pitch::new(18);
pub const GFLAT0: Pitch = FSHARP0;
pub const G0: Pitch = Pitch::new(19);
pub const GSHARP0: Pitch = Pitch::new(20);
pub const AFLAT0: Pitch = GSHARP0;
pub const A0: Pitch = Pitch::new(21);
pub const ASHARP0: Pitch = Pitch::new(22);
pub const BFLAT0: Pitch = ASHARP0;
pub const B0: Pitch = Pitch::new(23);

/// Pitches in octave 1 (32.70 Hz to 61.74 Hz)
pub const C1: Pitch = Pitch::new(24);
pub const CSHARP1: Pitch = Pitch::new(25);
pub const DFLAT1: Pitch = CSHARP1;
pub const D1: Pitch = Pitch::new(26);
pub const DSHARP1: Pitch = Pitch::new(27);
pub const EFLAT1: Pitch = DSHARP1;
pub const E1: Pitch = Pitch::new(28);
pub const F1: Pitch = Pitch::new(29);
pub const FSHARP1: Pitch = Pitch::new(30);
pub const GFLAT1: Pitch = FSHARP1;
pub const G1: Pitch = Pitch::new(31);
pub const GSHARP1: Pitch = Pitch::new(32);
pub const AFLAT1: Pitch = GSHARP1;
pub const A1: Pitch = Pitch::new(33);
pub const ASHARP1: Pitch = Pitch::new(34);
pub const BFLAT1: Pitch = ASHARP1;
pub const B1: Pitch = Pitch::new(35);

/// Pitches in octave 2 (65.41 Hz to 123.47 Hz)
pub const C2: Pitch = Pitch::new(36);
pub const CSHARP2: Pitch = Pitch::new(37);
pub const DFLAT2: Pitch = CSHARP2;
pub const D2: Pitch = Pitch::new(38);
pub const DSHARP2: Pitch = Pitch::new(39);
pub const EFLAT2: Pitch = DSHARP2;
pub const E2: Pitch = Pitch::new(40);
pub const F2: Pitch = Pitch::new(41);
pub const FSHARP2: Pitch = Pitch::new(42);
pub const GFLAT2: Pitch = FSHARP2;
pub const G2: Pitch = Pitch::new(43);
pub const GSHARP2: Pitch = Pitch::new(44);
pub const AFLAT2: Pitch = GSHARP2;
pub const A2: Pitch = Pitch::new(45);
pub const ASHARP2: Pitch = Pitch::new(46);
pub const BFLAT2: Pitch = ASHARP2;
pub const B2: Pitch = Pitch::new(47);

/// Pitches in octave 3 (130.81 Hz to 246.94 Hz)
pub const C3: Pitch = Pitch::new(48);
pub const CSHARP3: Pitch = Pitch::new(49);
pub const DFLAT3: Pitch = CSHARP3;
pub const D3: Pitch = Pitch::new(50);
pub const DSHARP3: Pitch = Pitch::new(51);
pub const EFLAT3: Pitch = DSHARP3;
pub const E3: Pitch = Pitch::new(52);
pub const F3: Pitch = Pitch::new(53);
pub const FSHARP3: Pitch = Pitch::new(54);
pub const GFLAT3: Pitch = FSHARP3;
pub const G3: Pitch = Pitch::new(55);
pub const GSHARP3: Pitch = Pitch::new(56);
pub const AFLAT3: Pitch = GSHARP3;
pub const A3: Pitch = Pitch::new(57);
pub const ASHARP3: Pitch = Pitch::new(58);
pub const BFLAT3: Pitch = ASHARP3;
pub const B3: Pitch = Pitch::new(59);

/// Pitches in octave 4 (261.63 Hz to 493.88 Hz)
/// This octave contains middle C (C4 = 261.63 Hz)
pub const C4: Pitch = Pitch::new(60);
pub const CSHARP4: Pitch = Pitch::new(61);
pub const DFLAT4: Pitch = CSHARP4;
pub const D4: Pitch = Pitch::new(62);
pub const DSHARP4: Pitch = Pitch::new(63);
pub const EFLAT4: Pitch = DSHARP4;
pub const E4: Pitch = Pitch::new(64);
pub const F4: Pitch = Pitch::new(65);
pub const FSHARP4: Pitch = Pitch::new(66);
pub const GFLAT4: Pitch = FSHARP4;
pub const G4: Pitch = Pitch::new(67);
pub const GSHARP4: Pitch = Pitch::new(68);
pub const AFLAT4: Pitch = GSHARP4;
pub const A4: Pitch = Pitch::new(69);
pub const ASHARP4: Pitch = Pitch::new(70);
pub const BFLAT4: Pitch = ASHARP4;
pub const B4: Pitch = Pitch::new(71);

/// Pitches in octave 5 (523.25 Hz to 987.77 Hz)
pub const C5: Pitch = Pitch::new(72);
pub const CSHARP5: Pitch = Pitch::new(73);
pub const DFLAT5: Pitch = CSHARP5;
pub const D5: Pitch = Pitch::new(74);
pub const DSHARP5: Pitch = Pitch::new(75);
pub const EFLAT5: Pitch = DSHARP5;
pub const E5: Pitch = Pitch::new(76);
pub const F5: Pitch = Pitch::new(77);
pub const FSHARP5: Pitch = Pitch::new(78);
pub const GFLAT5: Pitch = FSHARP5;
pub const G5: Pitch = Pitch::new(79);
pub const GSHARP5: Pitch = Pitch::new(80);
pub const AFLAT5: Pitch = GSHARP5;
pub const A5: Pitch = Pitch::new(81);
pub const ASHARP5: Pitch = Pitch::new(82);
pub const BFLAT5: Pitch = ASHARP5;
pub const B5: Pitch = Pitch::new(83);

/// Pitches in octave 6 (1046.50 Hz to 1975.53 Hz)
pub const C6: Pitch = Pitch::new(84);
pub const CSHARP6: Pitch = Pitch::new(85);
pub const DFLAT6: Pitch = CSHARP6;
pub const D6: Pitch = Pitch::new(86);
pub const DSHARP6: Pitch = Pitch::new(87);
pub const EFLAT6: Pitch = DSHARP6;
pub const E6: Pitch = Pitch::new(88);
pub const F6: Pitch = Pitch::new(89);
pub const FSHARP6: Pitch = Pitch::new(90);
pub const GFLAT6: Pitch = FSHARP6;
pub const G6: Pitch = Pitch::new(91);
pub const GSHARP6: Pitch = Pitch::new(92);
pub const AFLAT6: Pitch = GSHARP6;
pub const A6: Pitch = Pitch::new(93);
pub const ASHARP6: Pitch = Pitch::new(94);
pub const BFLAT6: Pitch = ASHARP6;
pub const B6: Pitch = Pitch::new(95);

/// Pitches in octave 7 (2093.00 Hz to 3951.07 Hz)
pub const C7: Pitch = Pitch::new(96);
pub const CSHARP7: Pitch = Pitch::new(97);
pub const DFLAT7: Pitch = CSHARP7;
pub const D7: Pitch = Pitch::new(98);
pub const DSHARP7: Pitch = Pitch::new(99);
pub const EFLAT7: Pitch = DSHARP7;
pub const E7: Pitch = Pitch::new(100);
pub const F7: Pitch = Pitch::new(101);
pub const FSHARP7: Pitch = Pitch::new(102);
pub const GFLAT7: Pitch = FSHARP7;
pub const G7: Pitch = Pitch::new(103);
pub const GSHARP7: Pitch = Pitch::new(104);
pub const AFLAT7: Pitch = GSHARP7;
pub const A7: Pitch = Pitch::new(105);
pub const ASHARP7: Pitch = Pitch::new(106);
pub const BFLAT7: Pitch = ASHARP7;
pub const B7: Pitch = Pitch::new(107);

/// Pitches in octave 8 (4186.01 Hz to 7902.13 Hz)
pub const C8: Pitch = Pitch::new(108);
pub const CSHARP8: Pitch = Pitch::new(109);
pub const DFLAT8: Pitch = CSHARP8;
pub const D8: Pitch = Pitch::new(110);
pub const DSHARP8: Pitch = Pitch::new(111);
pub const EFLAT8: Pitch = DSHARP8;
pub const E8: Pitch = Pitch::new(112);
pub const F8: Pitch = Pitch::new(113);
pub const FSHARP8: Pitch = Pitch::new(114);
pub const GFLAT8: Pitch = FSHARP8;
pub const G8: Pitch = Pitch::new(115);
pub const GSHARP8: Pitch = Pitch::new(116);
pub const AFLAT8: Pitch = GSHARP8;
pub const A8: Pitch = Pitch::new(117);
pub const ASHARP8: Pitch = Pitch::new(118);
pub const BFLAT8: Pitch = ASHARP8;
pub const B8: Pitch = Pitch::new(119);

/// Pitches in octave 9 (8372.02 Hz to 15804.27 Hz)
pub const C9: Pitch = Pitch::new(120);
pub const CSHARP9: Pitch = Pitch::new(121);
pub const DFLAT9: Pitch = CSHARP9;
pub const D9: Pitch = Pitch::new(122);
pub const DSHARP9: Pitch = Pitch::new(123);
pub const EFLAT9: Pitch = DSHARP9;
pub const E9: Pitch = Pitch::new(124);
pub const F9: Pitch = Pitch::new(125);
pub const FSHARP9: Pitch = Pitch::new(126);
pub const GFLAT9: Pitch = FSHARP9;
pub const G9: Pitch = Pitch::new(127);
