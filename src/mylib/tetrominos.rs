//! Tetrominos shape
//!
//! Names for shapes taken from https://en.wikipedia.org/wiki/Tetromino
//!
//! For use when not using a Adafruit 0.96” 160x80 Color TFT Display

use crate::mylib::rand::random_range;
use nanorand::Pcg64;

/// Straight shape piece
const STRAIGHT: [[u8; 2]; 2] = [[0, 1], [0, 1]];

/// Square shape piece
const SQUARE: [[u8; 2]; 2] = [[1, 1], [1, 1]];

/// Botton left corner shape piece, formerly L
const L: [[u8; 2]; 2] = [[1, 0], [1, 1]];

/// Corners piece, formerly S
const S: [[u8; 2]; 2] = [[0, 1], [1, 0]];

/// Top right corner shape piece, formerly T
const T: [[u8; 2]; 2] = [[1, 1], [0, 1]];

/// Get a random tetromino
///
/// # Arguments
/// * `rng` - A mutable reference to a Pcg64 random number generator.
///
/// # Returns
/// * A random tetromino shape const
pub fn get_random_tetromino(rng: &mut Pcg64) -> [[u8; 2]; 2] {
    match random_range(rng, 0, 5) {
        0 => STRAIGHT,
        1 => SQUARE,
        2 => L,
        3 => S,
        4 => T,
        _ => panic!("Invalid random number"),
    }
}
