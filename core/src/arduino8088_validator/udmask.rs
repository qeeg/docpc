/*
    MartyPC
    https://github.com/dbalsom/martypc

    Copyright 2022-2023 Daniel Balsom

    Permission is hereby granted, free of charge, to any person obtaining a
    copy of this software and associated documentation files (the “Software”),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom the
    Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in
    all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
    DEALINGS IN THE SOFTWARE.
*/
#![allow(dead_code)]

use crate::arduino8088_validator::ArduinoValidator;

pub const VFLAG_CARRY: u16 = 0x001;
pub const VFLAG_PARITY: u16 = 0x004;
pub const VFLAG_AUXILIARY: u16 = 0x010;
pub const VFLAG_ZERO: u16 = 0x040;
pub const VFLAG_SIGN: u16 = 0x080;
pub const VFLAG_TRAP: u16 = 0x100;
pub const VFLAG_INTERRUPT: u16 = 0x200;
pub const VFLAG_DIRECTION: u16 = 0x400;
pub const VFLAG_OVERFLOW: u16 = 0x800;

pub const IGNORE_MASK: u16 = 0xCD5;

pub struct FlagMask {
    pub opcode: i16,
    pub group:  usize,
    pub mask:   u16,
}

#[rustfmt::skip]
pub const FLAG_MASK_LOOKUP: [FlagMask; 256] =  [
    FlagMask { opcode: 0x00, group: 0, mask: 0 },
    FlagMask { opcode: 0x01, group: 0, mask: 0 },
    FlagMask { opcode: 0x02, group: 0, mask: 0 },
    FlagMask { opcode: 0x03, group: 0, mask: 0 },
    FlagMask { opcode: 0x04, group: 0, mask: 0 },
    FlagMask { opcode: 0x05, group: 0, mask: 0 },
    FlagMask { opcode: 0x06, group: 0, mask: 0 },
    FlagMask { opcode: 0x07, group: 0, mask: 0 },
    FlagMask { opcode: 0x08, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x09, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x0A, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x0B, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x0C, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x0D, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x0E, group: 0, mask: 0 },
    FlagMask { opcode: 0x0F, group: 0, mask: 0 },
    FlagMask { opcode: 0x10, group: 0, mask: 0 },
    FlagMask { opcode: 0x11, group: 0, mask: 0 },
    FlagMask { opcode: 0x12, group: 0, mask: 0 },
    FlagMask { opcode: 0x13, group: 0, mask: 0 },
    FlagMask { opcode: 0x14, group: 0, mask: 0 },
    FlagMask { opcode: 0x15, group: 0, mask: 0 },
    FlagMask { opcode: 0x16, group: 0, mask: 0 },
    FlagMask { opcode: 0x17, group: 0, mask: 0 },
    FlagMask { opcode: 0x18, group: 0, mask: 0 },
    FlagMask { opcode: 0x19, group: 0, mask: 0 },
    FlagMask { opcode: 0x1A, group: 0, mask: 0 },
    FlagMask { opcode: 0x1B, group: 0, mask: 0 },
    FlagMask { opcode: 0x1C, group: 0, mask: 0 },
    FlagMask { opcode: 0x1D, group: 0, mask: 0 },
    FlagMask { opcode: 0x1E, group: 0, mask: 0 },
    FlagMask { opcode: 0x1F, group: 0, mask: 0 },
    FlagMask { opcode: 0x20, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x21, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x22, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x23, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x24, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x25, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x26, group: 0, mask: 0 },
    //FlagMask { opcode: 0x27, group: 0, mask: VFLAG_OVERFLOW },
    FlagMask { opcode: 0x27, group: 0, mask: 0 }, // DAA - implemented overflow flag behavior
    FlagMask { opcode: 0x28, group: 0, mask: 0 },
    FlagMask { opcode: 0x29, group: 0, mask: 0 },
    FlagMask { opcode: 0x2A, group: 0, mask: 0 },
    FlagMask { opcode: 0x2B, group: 0, mask: 0 },
    FlagMask { opcode: 0x2C, group: 0, mask: 0 },
    FlagMask { opcode: 0x2D, group: 0, mask: 0 },
    FlagMask { opcode: 0x2E, group: 0, mask: 0 },
    //FlagMask { opcode: 0x2F, group: 0, mask: VFLAG_OVERFLOW },
    FlagMask { opcode: 0x2F, group: 0, mask: 0 }, // DAS - implemented overflow flag behavior
    FlagMask { opcode: 0x30, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x31, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x32, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x33, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x34, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x35, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x36, group: 0, mask: 0 },
    //FlagMask { opcode: 0x37, group: 0, mask: VFLAG_PARITY | VFLAG_ZERO | VFLAG_SIGN | VFLAG_OVERFLOW },
    FlagMask { opcode: 0x37, group: 0, mask: 0 }, // AAA - implemented flag behavior
    FlagMask { opcode: 0x38, group: 0, mask: 0 },
    FlagMask { opcode: 0x39, group: 0, mask: 0 },
    FlagMask { opcode: 0x3A, group: 0, mask: 0 },
    FlagMask { opcode: 0x3B, group: 0, mask: 0 },
    FlagMask { opcode: 0x3C, group: 0, mask: 0 },
    FlagMask { opcode: 0x3D, group: 0, mask: 0 },
    FlagMask { opcode: 0x3E, group: 0, mask: 0 },
    //FlagMask { opcode: 0x3F, group: 0, mask: VFLAG_PARITY | VFLAG_ZERO | VFLAG_SIGN | VFLAG_OVERFLOW },
    FlagMask { opcode: 0x3F, group: 0, mask: 0 }, // AAS - implemented flag behavior
    FlagMask { opcode: 0x40, group: 0, mask: 0 },
    FlagMask { opcode: 0x41, group: 0, mask: 0 },
    FlagMask { opcode: 0x42, group: 0, mask: 0 },
    FlagMask { opcode: 0x43, group: 0, mask: 0 },
    FlagMask { opcode: 0x44, group: 0, mask: 0 },
    FlagMask { opcode: 0x45, group: 0, mask: 0 },
    FlagMask { opcode: 0x46, group: 0, mask: 0 },
    FlagMask { opcode: 0x47, group: 0, mask: 0 },
    FlagMask { opcode: 0x48, group: 0, mask: 0 },
    FlagMask { opcode: 0x49, group: 0, mask: 0 },
    FlagMask { opcode: 0x4A, group: 0, mask: 0 },
    FlagMask { opcode: 0x4B, group: 0, mask: 0 },
    FlagMask { opcode: 0x4C, group: 0, mask: 0 },
    FlagMask { opcode: 0x4D, group: 0, mask: 0 },
    FlagMask { opcode: 0x4E, group: 0, mask: 0 },
    FlagMask { opcode: 0x4F, group: 0, mask: 0 },
    FlagMask { opcode: 0x50, group: 0, mask: 0 },
    FlagMask { opcode: 0x51, group: 0, mask: 0 },
    FlagMask { opcode: 0x52, group: 0, mask: 0 },
    FlagMask { opcode: 0x53, group: 0, mask: 0 },
    FlagMask { opcode: 0x54, group: 0, mask: 0 },
    FlagMask { opcode: 0x55, group: 0, mask: 0 },
    FlagMask { opcode: 0x56, group: 0, mask: 0 },
    FlagMask { opcode: 0x57, group: 0, mask: 0 },
    FlagMask { opcode: 0x58, group: 0, mask: 0 },
    FlagMask { opcode: 0x59, group: 0, mask: 0 },
    FlagMask { opcode: 0x5A, group: 0, mask: 0 },
    FlagMask { opcode: 0x5B, group: 0, mask: 0 },
    FlagMask { opcode: 0x5C, group: 0, mask: 0 },
    FlagMask { opcode: 0x5D, group: 0, mask: 0 },
    FlagMask { opcode: 0x5E, group: 0, mask: 0 },
    FlagMask { opcode: 0x5F, group: 0, mask: 0 },
    FlagMask { opcode: 0x60, group: 0, mask: 0 },
    FlagMask { opcode: 0x61, group: 0, mask: 0 },
    FlagMask { opcode: 0x62, group: 0, mask: 0 },
    FlagMask { opcode: 0x63, group: 0, mask: 0 },
    FlagMask { opcode: 0x64, group: 0, mask: 0 },
    FlagMask { opcode: 0x65, group: 0, mask: 0 },
    FlagMask { opcode: 0x66, group: 0, mask: 0 },
    FlagMask { opcode: 0x67, group: 0, mask: 0 },
    FlagMask { opcode: 0x68, group: 0, mask: 0 },
    FlagMask { opcode: 0x69, group: 0, mask: 0 },
    FlagMask { opcode: 0x6A, group: 0, mask: 0 },
    FlagMask { opcode: 0x6B, group: 0, mask: 0 },
    FlagMask { opcode: 0x6C, group: 0, mask: 0 },
    FlagMask { opcode: 0x6D, group: 0, mask: 0 },
    FlagMask { opcode: 0x6E, group: 0, mask: 0 },
    FlagMask { opcode: 0x6F, group: 0, mask: 0 },
    FlagMask { opcode: 0x70, group: 0, mask: 0 },
    FlagMask { opcode: 0x71, group: 0, mask: 0 },
    FlagMask { opcode: 0x72, group: 0, mask: 0 },
    FlagMask { opcode: 0x73, group: 0, mask: 0 },
    FlagMask { opcode: 0x74, group: 0, mask: 0 },
    FlagMask { opcode: 0x75, group: 0, mask: 0 },
    FlagMask { opcode: 0x76, group: 0, mask: 0 },
    FlagMask { opcode: 0x77, group: 0, mask: 0 },
    FlagMask { opcode: 0x78, group: 0, mask: 0 },
    FlagMask { opcode: 0x79, group: 0, mask: 0 },
    FlagMask { opcode: 0x7A, group: 0, mask: 0 },
    FlagMask { opcode: 0x7B, group: 0, mask: 0 },
    FlagMask { opcode: 0x7C, group: 0, mask: 0 },
    FlagMask { opcode: 0x7D, group: 0, mask: 0 },
    FlagMask { opcode: 0x7E, group: 0, mask: 0 },
    FlagMask { opcode: 0x7F, group: 0, mask: 0 },
    FlagMask { opcode: 0x80, group: 1, mask: 0 },
    FlagMask { opcode: 0x81, group: 1, mask: 0 },
    FlagMask { opcode: 0x82, group: 1, mask: 0 },
    FlagMask { opcode: 0x83, group: 1, mask: 0 },
    FlagMask { opcode: 0x84, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x85, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0x86, group: 0, mask: 0 },
    FlagMask { opcode: 0x87, group: 0, mask: 0 },
    FlagMask { opcode: 0x88, group: 0, mask: 0 },
    FlagMask { opcode: 0x89, group: 0, mask: 0 },
    FlagMask { opcode: 0x8A, group: 0, mask: 0 },
    FlagMask { opcode: 0x8B, group: 0, mask: 0 },
    FlagMask { opcode: 0x8C, group: 0, mask: 0 },
    FlagMask { opcode: 0x8D, group: 0, mask: 0 },
    FlagMask { opcode: 0x8E, group: 0, mask: 0 },
    FlagMask { opcode: 0x8F, group: 0, mask: 0 },
    FlagMask { opcode: 0x90, group: 0, mask: 0 },
    FlagMask { opcode: 0x91, group: 0, mask: 0 },
    FlagMask { opcode: 0x92, group: 0, mask: 0 },
    FlagMask { opcode: 0x93, group: 0, mask: 0 },
    FlagMask { opcode: 0x94, group: 0, mask: 0 },
    FlagMask { opcode: 0x95, group: 0, mask: 0 },
    FlagMask { opcode: 0x96, group: 0, mask: 0 },
    FlagMask { opcode: 0x97, group: 0, mask: 0 },
    FlagMask { opcode: 0x98, group: 0, mask: 0 },
    FlagMask { opcode: 0x99, group: 0, mask: 0 },
    FlagMask { opcode: 0x9A, group: 0, mask: 0 },
    FlagMask { opcode: 0x9B, group: 0, mask: 0 },
    FlagMask { opcode: 0x9C, group: 0, mask: 0 },
    FlagMask { opcode: 0x9D, group: 0, mask: 0 },
    FlagMask { opcode: 0x9E, group: 0, mask: 0 },
    FlagMask { opcode: 0x9F, group: 0, mask: 0 },
    FlagMask { opcode: 0xA0, group: 0, mask: 0 },
    FlagMask { opcode: 0xA1, group: 0, mask: 0 },
    FlagMask { opcode: 0xA2, group: 0, mask: 0 },
    FlagMask { opcode: 0xA3, group: 0, mask: 0 },
    FlagMask { opcode: 0xA4, group: 0, mask: 0 },
    FlagMask { opcode: 0xA5, group: 0, mask: 0 },
    FlagMask { opcode: 0xA6, group: 0, mask: 0 },
    FlagMask { opcode: 0xA7, group: 0, mask: 0 },
    FlagMask { opcode: 0xA8, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0xA9, group: 0, mask: VFLAG_AUXILIARY },
    FlagMask { opcode: 0xAA, group: 0, mask: 0 },
    FlagMask { opcode: 0xAB, group: 0, mask: 0 },
    FlagMask { opcode: 0xAC, group: 0, mask: 0 },
    FlagMask { opcode: 0xAD, group: 0, mask: 0 },
    FlagMask { opcode: 0xAE, group: 0, mask: 0 },
    FlagMask { opcode: 0xAF, group: 0, mask: 0 },
    FlagMask { opcode: 0xB0, group: 0, mask: 0 },
    FlagMask { opcode: 0xB1, group: 0, mask: 0 },
    FlagMask { opcode: 0xB2, group: 0, mask: 0 },
    FlagMask { opcode: 0xB3, group: 0, mask: 0 },
    FlagMask { opcode: 0xB4, group: 0, mask: 0 },
    FlagMask { opcode: 0xB5, group: 0, mask: 0 },
    FlagMask { opcode: 0xB6, group: 0, mask: 0 },
    FlagMask { opcode: 0xB7, group: 0, mask: 0 },
    FlagMask { opcode: 0xB8, group: 0, mask: 0 },
    FlagMask { opcode: 0xB9, group: 0, mask: 0 },
    FlagMask { opcode: 0xBA, group: 0, mask: 0 },
    FlagMask { opcode: 0xBB, group: 0, mask: 0 },
    FlagMask { opcode: 0xBC, group: 0, mask: 0 },
    FlagMask { opcode: 0xBD, group: 0, mask: 0 },
    FlagMask { opcode: 0xBE, group: 0, mask: 0 },
    FlagMask { opcode: 0xBF, group: 0, mask: 0 },
    FlagMask { opcode: 0xC0, group: 0, mask: 0 },
    FlagMask { opcode: 0xC1, group: 0, mask: 0 },
    FlagMask { opcode: 0xC2, group: 0, mask: 0 },
    FlagMask { opcode: 0xC3, group: 0, mask: 0 },
    FlagMask { opcode: 0xC4, group: 0, mask: 0 },
    FlagMask { opcode: 0xC5, group: 0, mask: 0 },
    FlagMask { opcode: 0xC6, group: 0, mask: 0 },
    FlagMask { opcode: 0xC7, group: 0, mask: 0 },
    FlagMask { opcode: 0xC8, group: 0, mask: 0 },
    FlagMask { opcode: 0xC9, group: 0, mask: 0 },
    FlagMask { opcode: 0xCA, group: 0, mask: 0 },
    FlagMask { opcode: 0xCB, group: 0, mask: 0 },
    FlagMask { opcode: 0xCC, group: 0, mask: 0 },
    FlagMask { opcode: 0xCD, group: 0, mask: 0 },
    FlagMask { opcode: 0xCE, group: 0, mask: 0 },
    FlagMask { opcode: 0xCF, group: 0, mask: 0 },
    FlagMask { opcode: 0xD0, group: 2, mask: 0 },
    FlagMask { opcode: 0xD1, group: 2, mask: 0 },
    FlagMask { opcode: 0xD2, group: 3, mask: 0 },
    FlagMask { opcode: 0xD3, group: 3, mask: 0 },
    FlagMask { opcode: 0xD4, group: 0, mask: VFLAG_CARRY | VFLAG_AUXILIARY | VFLAG_OVERFLOW },
    FlagMask { opcode: 0xD5, group: 0, mask: VFLAG_CARRY | VFLAG_AUXILIARY | VFLAG_OVERFLOW },
    FlagMask { opcode: 0xD6, group: 0, mask: 0 },
    FlagMask { opcode: 0xD7, group: 0, mask: 0 },
    FlagMask { opcode: 0xD8, group: 0, mask: 0 },
    FlagMask { opcode: 0xD9, group: 0, mask: 0 },
    FlagMask { opcode: 0xDA, group: 0, mask: 0 },
    FlagMask { opcode: 0xDB, group: 0, mask: 0 },
    FlagMask { opcode: 0xDC, group: 0, mask: 0 },
    FlagMask { opcode: 0xDD, group: 0, mask: 0 },
    FlagMask { opcode: 0xDE, group: 0, mask: 0 },
    FlagMask { opcode: 0xDF, group: 0, mask: 0 },
    FlagMask { opcode: 0xE0, group: 0, mask: 0 },
    FlagMask { opcode: 0xE1, group: 0, mask: 0 },
    FlagMask { opcode: 0xE2, group: 0, mask: 0 },
    FlagMask { opcode: 0xE3, group: 0, mask: 0 },
    FlagMask { opcode: 0xE4, group: 0, mask: 0 },
    FlagMask { opcode: 0xE5, group: 0, mask: 0 },
    FlagMask { opcode: 0xE6, group: 0, mask: 0 },
    FlagMask { opcode: 0xE7, group: 0, mask: 0 },
    FlagMask { opcode: 0xE8, group: 0, mask: 0 },
    FlagMask { opcode: 0xE9, group: 0, mask: 0 },
    FlagMask { opcode: 0xEA, group: 0, mask: 0 },
    FlagMask { opcode: 0xEB, group: 0, mask: 0 },
    FlagMask { opcode: 0xEC, group: 0, mask: 0 },
    FlagMask { opcode: 0xED, group: 0, mask: 0 },
    FlagMask { opcode: 0xEE, group: 0, mask: 0 },
    FlagMask { opcode: 0xEF, group: 0, mask: 0 },
    FlagMask { opcode: 0xF0, group: 0, mask: 0 },
    FlagMask { opcode: 0xF1, group: 0, mask: 0 },
    FlagMask { opcode: 0xF2, group: 0, mask: 0 },
    FlagMask { opcode: 0xF3, group: 0, mask: 0 },
    FlagMask { opcode: 0xF4, group: 0, mask: 0 },
    FlagMask { opcode: 0xF5, group: 0, mask: 0 },
    FlagMask { opcode: 0xF6, group: 4, mask: 0 },
    FlagMask { opcode: 0xF7, group: 4, mask: 0 },
    FlagMask { opcode: 0xF8, group: 0, mask: 0 },
    FlagMask { opcode: 0xF9, group: 0, mask: 0 },
    FlagMask { opcode: 0xFA, group: 0, mask: 0 },
    FlagMask { opcode: 0xFB, group: 0, mask: 0 },
    FlagMask { opcode: 0xFC, group: 0, mask: 0 },
    FlagMask { opcode: 0xFD, group: 0, mask: 0 },
    FlagMask { opcode: 0xFE, group: 5, mask: 0 },
    FlagMask { opcode: 0xFF, group: 5, mask: 0 }
];

#[rustfmt::skip]
pub const FLAG_MASK_GROUP_LOOKUP: [[FlagMask; 8]; 5] = [
    // Group #1 0x80-0x83
    [
        FlagMask { opcode: 0x80, group: 0, mask: 0 },
        FlagMask { opcode: 0x80, group: 0, mask: VFLAG_AUXILIARY },
        FlagMask { opcode: 0x80, group: 0, mask: 0 },
        FlagMask { opcode: 0x80, group: 0, mask: 0 },
        FlagMask { opcode: 0x80, group: 0, mask: VFLAG_AUXILIARY },
        FlagMask { opcode: 0x80, group: 0, mask: 0 },
        FlagMask { opcode: 0x80, group: 0, mask: VFLAG_AUXILIARY },
        FlagMask { opcode: 0x80, group: 0, mask: 0 },
    ],
    // Group #2 0xD0-0xD1
    [
        FlagMask { opcode: 0xD0, group: 0, mask: 0 },
        FlagMask { opcode: 0xD0, group: 0, mask: 0 },
        FlagMask { opcode: 0xD0, group: 0, mask: 0 },
        FlagMask { opcode: 0xD0, group: 0, mask: 0 },
        FlagMask { opcode: 0xD0, group: 0, mask: VFLAG_AUXILIARY },
        FlagMask { opcode: 0xD0, group: 0, mask: VFLAG_AUXILIARY },
        FlagMask { opcode: 0xD0, group: 0, mask: VFLAG_AUXILIARY },
        FlagMask { opcode: 0xD0, group: 0, mask: VFLAG_AUXILIARY },
    ],
    // Group #3 0xD2-0xD3
    [
        FlagMask { opcode: 0xD2, group: 0, mask: VFLAG_OVERFLOW },
        FlagMask { opcode: 0xD2, group: 0, mask: VFLAG_OVERFLOW },
        FlagMask { opcode: 0xD2, group: 0, mask: VFLAG_OVERFLOW },
        FlagMask { opcode: 0xD2, group: 0, mask: VFLAG_OVERFLOW },
        FlagMask { opcode: 0xD2, group: 0, mask: VFLAG_CARRY | VFLAG_AUXILIARY | VFLAG_OVERFLOW },
        FlagMask { opcode: 0xD2, group: 0, mask: VFLAG_CARRY | VFLAG_AUXILIARY | VFLAG_OVERFLOW },
        FlagMask { opcode: 0xD2, group: 0, mask: VFLAG_CARRY | VFLAG_AUXILIARY | VFLAG_OVERFLOW },
        FlagMask { opcode: 0xD2, group: 0, mask: VFLAG_AUXILIARY | VFLAG_OVERFLOW },
    ],
    // Group #4 0xF6-0xF7
    [
        FlagMask { opcode: 0xF6, group: 0, mask: VFLAG_AUXILIARY },
        FlagMask { opcode: 0xF6, group: 0, mask: VFLAG_AUXILIARY },
        FlagMask { opcode: 0xF6, group: 0, mask: 0 },
        FlagMask { opcode: 0xF6, group: 0, mask: 0 },
        FlagMask { opcode: 0xF6, group: 0, mask: VFLAG_PARITY | VFLAG_AUXILIARY | VFLAG_ZERO | VFLAG_SIGN },
        FlagMask { opcode: 0xF6, group: 0, mask: VFLAG_PARITY | VFLAG_AUXILIARY | VFLAG_ZERO | VFLAG_SIGN },
        FlagMask { opcode: 0xF6, group: 0, mask: VFLAG_CARRY | VFLAG_PARITY | VFLAG_AUXILIARY | VFLAG_ZERO | VFLAG_SIGN | VFLAG_OVERFLOW },
        FlagMask { opcode: 0xF6, group: 0, mask: VFLAG_CARRY | VFLAG_PARITY | VFLAG_AUXILIARY | VFLAG_ZERO | VFLAG_SIGN | VFLAG_OVERFLOW },
    ],
    // Group #5 0xFE-0xFF
    [
        FlagMask { opcode: 0xFE, group: 0, mask: 0 },
        FlagMask { opcode: 0xFE, group: 0, mask: 0 },
        FlagMask { opcode: 0xFE, group: 0, mask: 0 },
        FlagMask { opcode: 0xFE, group: 0, mask: 0 },
        FlagMask { opcode: 0xFE, group: 0, mask: 0 },
        FlagMask { opcode: 0xFE, group: 0, mask: 0 },
        FlagMask { opcode: 0xFE, group: 0, mask: 0 },
        FlagMask { opcode: 0xFE, group: 0, mask: 0 },
    ],    
];

impl ArduinoValidator {
    pub fn mask_undefined_flags(opcode: u8, modrm: u8, flags: u16) -> u16 {
        let mut masked_flags = flags & IGNORE_MASK; // Ignore I, T and reserved flags

        let grp = FLAG_MASK_LOOKUP[opcode as usize].group as usize;

        if grp == 0 {
            // Not a group opcode, mask directly.
            masked_flags &= !FLAG_MASK_LOOKUP[opcode as usize].mask;
        }
        else {
            // Is group opcode, look up from group table.
            let grp_op = ((modrm >> 3) & 0x07) as usize;
            masked_flags &= !FLAG_MASK_GROUP_LOOKUP[grp - 1][grp_op].mask;
        }

        masked_flags
    }

    pub fn is_group_opcode(opcode: u8) -> bool {
        FLAG_MASK_LOOKUP[opcode as usize].group != 0
    }
}
