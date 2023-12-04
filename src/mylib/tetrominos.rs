//! Tetrominos shape
//!
//! Names for shapes taken from <https://en.wikipedia.org/wiki/Tetromino>
//!
//! For use when not using a Adafruit 0.96” 160x80 Color TFT Display

use crate::mylib::rand::random_range;
use nanorand::Pcg64;

/// Straight shape piece
const STRAIGHT: [[u8; 2]; 2] = [[1, 0], [1, 0]];

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
/// Weighted against the straight piece (1/10 chance to get straight piece)
///
/// # Arguments
/// * `rng` - A mutable reference to a Pcg64 random number generator.
///
/// # Returns
/// * A random tetromino shape const
pub fn get_random_tetromino(rng: &mut Pcg64) -> [[u8; 2]; 2] {
    let rand = random_range(rng, 0, 10);
    match rand {
        0 => L,
        1 => S,
        2 => T,
        3 => SQUARE,
        4 => STRAIGHT,
        5 => L,
        6 => S,
        7 => T,
        8 => L,
        9 => L,
        _ => T,
    }
}

/// Rotate a tetromino clockwise 90 degrees
///
/// # Arguments
/// * `tetromino` - A tetromino shape value
///
/// # Returns
/// * That tetromino rotated clockwise 90 degrees
pub fn rotate_clockwise(tetromino: [[u8; 2]; 2]) -> [[u8; 2]; 2] {
    let mut rotated = [[0; 2]; 2];
    rotated[0][0] = tetromino[1][0];
    rotated[0][1] = tetromino[0][0];
    rotated[1][0] = tetromino[1][1];
    rotated[1][1] = tetromino[0][1];
    rotated
}
