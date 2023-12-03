//! Random number generator
//!
//! Custom functions for generating random numbers.
use nanorand::{Pcg64, Rng};

/// Generate a random number between min and max.
///
/// # Arguments
/// * `rng` - A mutable reference to a Pcg64 random number generator.
/// * `min` - The minimum value of the random number.
/// * `max` - The maximum value of the random number.
///
/// # Returns
/// * A random number between min and max.
pub fn random_range(rng: &mut Pcg64, min: u8, max: u8) -> u8 {
    let r: u8 = rng.generate();
    r % (max - min) + min
}
