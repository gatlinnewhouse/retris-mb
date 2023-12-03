/*
The MIT License (MIT)

Copyright (c) 2021 Matthew Woodcraft.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//! A 5×5 ascii font.
//!
//! This is a copy of the 'pendolino' font from the [micro:bit runtime][dal].
//!
//! [dal]: https://lancaster-university.github.io/microbit-docs/

use crate::mylib::pendolino;
use crate::mylib::pixeldisplay::Render;

/// Index of the first character in the standard font
pub const PRINTABLE_START: usize = 32;

/// Number of characters in the standard font
pub const PRINTABLE_COUNT: usize = 95;

const UNKNOWN: Render = Render::new_from(&[
    [1, 1, 1, 1, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 1],
]);

/// Returns an image representing the requested ascii character.
///
/// If the requested character isn't printable, returns a 'hollow square' image.
///
/// # Example
///
/// `font::character(b'x')`
pub fn character(index: u8) -> &'static Render {
    let index = index as usize;
    if !(PRINTABLE_START..PRINTABLE_START + PRINTABLE_COUNT).contains(&index) {
        return &UNKNOWN;
    }
    &pendolino::PENDOLINO3[index - PRINTABLE_START]
}

/// Returns a Render representing the requested ascii character.
pub const fn font_entry(data: [u8; 5]) -> Render {
    // Note the data in the pendolino font uses the opposite column numbering
    // system to BitImage.
    const fn row_bits(byte: u8) -> [u8; 5] {
        [
            (((byte & 1 << 4) != 0) as u8) * 9,
            (((byte & 1 << 3) != 0) as u8) * 9,
            (((byte & 1 << 2) != 0) as u8) * 9,
            (((byte & 1 << 1) != 0) as u8) * 9,
            (((byte & 1 << 0) != 0) as u8) * 9,
        ]
    }
    Render::new_from(&[
        row_bits(data[0]),
        row_bits(data[1]),
        row_bits(data[2]),
        row_bits(data[3]),
        row_bits(data[4]),
    ])
}

/// Function that returns boolean if char is wide or not (wide = 5 columns)
pub fn wide_char(c: char) -> bool {
    let wide_chars = ['G', 'g', 'a', 'M', 'm', 'V', 'v', 'N', 'T'];
    for wc in wide_chars.iter() {
        if *wc == c {
            return true;
        }
    }
    false
}
