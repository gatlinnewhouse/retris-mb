//! Use the A and B buttons as controls for the game
//!
//! Taken from my `breakout` project for the Embedded Rust class
use microbit::hal::{
    gpio::{p0::P0_14, p0::P0_23, Floating, Input},
    prelude::InputPin,
};

/// Material needed for button presses
pub struct Buttons {
    /// A button
    pub a: P0_14<Input<Floating>>,
    /// B button
    pub b: P0_23<Input<Floating>>,
}

impl Buttons {
    /// Set up the buttons
    ///
    /// # Arguments
    /// * `button_a` - The A button pin as a floating input
    /// * `button_b` - The B button pin as a floating input
    ///
    /// # Returns
    /// * `Self` - The buttons as a struct
    pub fn new(button_a: P0_14<Input<Floating>>, button_b: P0_23<Input<Floating>>) -> Self {
        let a = button_a;
        let b = button_b;
        Self { a, b }
    }

    /// Check if the A button is pressed
    ///
    /// # Returns
    /// * `Option<bool>` - Whether the A button is pressed, Some(true) if pressed, None if not pressed
    pub fn read_a(&self) -> Option<bool> {
        if self.a.is_low().unwrap() {
            Some(true)
        } else {
            None
        }
    }

    /// Check if the B button is pressed
    ///
    /// # Returns
    /// * `Option<bool>` - Whether the B button is pressed, Some(true) if pressed, None if not pressed
    pub fn read_b(&self) -> Option<bool> {
        if self.b.is_low().unwrap() {
            Some(true)
        } else {
            None
        }
    }
}
