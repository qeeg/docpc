/*
    MartyPC
    https://github.com/dbalsom/martypc

    Copyright 2022-2024 Daniel Balsom

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

    --------------------------------------------------------------------------

    devices::ega::tablegen.rs

    Const table generation for various lookups used by the CGA for fast
    character drawing.

*/

use super::*;

/// LUT to extend an 8-bit bitfield into a packed 64-bit value
pub const BIT_EXTEND_TABLE64: [u64; 256] = {
    let mut table = [0u64; 256];
    let mut i = 0;
    while i < 256 {
        let mut bit = 0;
        let mut j = 0u64;
        while bit < 8 {
            let segment = if (i >> bit) & 0x01 != 0 { 0xFFu64 } else { 0x00u64 };
            j |= segment << (bit * 8);
            bit += 1;
        }
        table[i] = j;
        i += 1;
    }
    table
};

/// LUT to extend an 8-bit bitfield into a packed 64-bit value, reversing the bit order
pub const BIT_EXTEND_REVERSE_TABLE64: [u64; 256] = {
    let mut table = [0u64; 256];
    let mut i = 0;
    while i < 256 {
        let mut bit = 0;
        let mut j = 0u64;
        while bit < 8 {
            let segment = if (i >> (7 - bit)) & 0x01 != 0 { 0xFFu64 } else { 0x00u64 };
            j |= segment << (bit * 8);
            bit += 1;
        }
        table[i] = j;
        i += 1;
    }
    table
};

pub const BYTE_EXTEND_TABLE64: [u64; 256] = {
    let mut table: [u64; 256] = [0; 256];
    let mut i: usize = 0;

    while i < 256 {
        let mut bit: usize = 0;
        let mut k: u64 = 0;

        while bit < 8 {
            k |= (i as u64) << ((7 - bit) * 8);
            bit += 1;
        }

        table[i] = k;
        i += 1;
    }
    table
};

pub const BYTE_EXTEND_TABLE: [[u8; 8]; 256] = {
    let mut table: [[u8; 8]; 256] = [[0; 8]; 256];
    let mut i: u32 = 0;

    while i < 256 {
        let mut j: u8 = 0;
        while j < 8 {
            table[i as usize][j as usize] = ((i as u8) >> (7 - j)) & 0x01;
            j += 1;
        }
        i += 1;
    }
    table
};

/// Constant initializer to pack all possible 6-bit values into 64, 64 bit words
/// representing 8 packed pixels each.
pub const EGA_COLORS_U64: [u64; 64] = {
    let mut packed = [0u64; 64];
    let mut i = 0;

    while i < 64 {
        packed[i] = (i as u64) * 0x0101010101010101;
        i += 1;
    }

    packed
};

/// Constant initializer to unpack all possible 8 bit patterns
pub const EGA_8BIT_TABLE: [u64; 256] = {
    let mut table: [u64; 256] = [0; 256];

    let mut glyph: usize = 0;
    let mut glyph_u64: u64;
    let mut bit: u8;
    loop {
        bit = 0;
        glyph_u64 = 0;
        loop {
            let bit_val = glyph & (0x01 << (7 - bit)) != 0;

            glyph_u64 |= (if bit_val { 0xFF } else { 0x00 }) << (bit * 8);

            if bit < 7 {
                bit += 1;
            }
            else {
                break;
            }
        }

        table[glyph] = glyph_u64;

        if glyph < 255 {
            glyph += 1;
        }
        else {
            break;
        }
    }

    table
};

/// Constant initializer to unpack all possible 8 bit patterns
/// in all 16 possible colors into 64 bit values for fast drawing.
pub const EGA_HIRES_GFX_TABLE: [[u64; 256]; 16] = {
    let mut table: [[u64; 256]; 16] = [[0; 256]; 16];
    let mut glyph;
    let mut color: usize = 0;

    loop {
        glyph = 0;
        loop {
            table[color][glyph] = EGA_8BIT_TABLE[glyph] & EGA_COLORS_U64[color];

            if glyph < 255 {
                glyph += 1;
            }
            else {
                break;
            }
        }

        if color < 15 {
            color += 1;
        }
        else {
            break;
        }
    }

    table
};

/// Constant initializer to unpack all possible 8 bit patterns
/// of 4, 2-bit pixels into their corresponding u64 representations
/// by palette. Since value 0 is substituted with the current
/// cc background color, we also generate a mask to use for setting
/// the background color.
/// To use this mask, we perform the following operation:
/// (glyph64, mask64) = table[pal][glyph]
/// draw64 = glyph64 | ((glyph64 & mask64) & cc_altcolor))
pub const CGA_LOWRES_GFX_TABLE: [[(u64, u64); 256]; 6] = {
    let mut table: [[(u64, u64); 256]; 6] = [[(0, 0); 256]; 6];
    let mut glyph;
    let mut palette_i: usize = 0;

    loop {
        glyph = 0;
        loop {
            // Break out 8 bit pattern into 4, 2-bit pixels
            let pix0 = (glyph >> 6) & 0b11;
            let pix1 = (glyph >> 4) & 0b11;
            let pix2 = (glyph >> 2) & 0b11;
            let pix3 = glyph & 0b11;

            // Look up 2-bit pixel indices into current 4-color palette to get
            // a 16-color palette index
            let mut color0: u64 = CGA_PALETTES[palette_i][pix0 as usize] as u64;
            let mut color1: u64 = CGA_PALETTES[palette_i][pix1 as usize] as u64;
            let mut color2: u64 = CGA_PALETTES[palette_i][pix2 as usize] as u64;
            let mut color3: u64 = CGA_PALETTES[palette_i][pix3 as usize] as u64;

            // Double pixels
            color0 |= color0 << 8;
            color1 |= color1 << 8;
            color2 |= color2 << 8;
            color3 |= color3 << 8;

            // Build a mask where color index 0 == FFFF
            let mask0: u64 = if pix0 == 0 { 0xFFFF } else { 0x0000 };
            let mask1: u64 = if pix1 == 0 { 0xFFFF } else { 0x0000 };
            let mask2: u64 = if pix2 == 0 { 0xFFFF } else { 0x0000 };
            let mask3: u64 = if pix3 == 0 { 0xFFFF } else { 0x0000 };

            // Create the glyph tuple
            //let glyph64 = color0 << 48 | color1 << 32 | color2 << 16 | color3;
            //let mask64 = mask0 << 48 | mask1 << 32 | mask2 << 16 | mask3;

            let glyph64 = color3 << 48 | color2 << 32 | color1 << 16 | color0;
            let mask64 = mask3 << 48 | mask2 << 32 | mask1 << 16 | mask0;

            table[palette_i][glyph] = (glyph64, mask64);

            if glyph < 255 {
                glyph += 1;
            }
            else {
                break;
            }
        }

        if palette_i < 5 {
            palette_i += 1;
        }
        else {
            break;
        }
    }

    table
};
