//! Use the A and B buttons as controls for the game
//! Taken from my `breakout` project for Embedded Rust
//! Gatlin Newhouse 2023
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
    pub fn new(button_a: P0_14<Input<Floating>>, button_b: P0_23<Input<Floating>>) -> Self {
        let a = button_a;
        let b = button_b;
        Self { a, b }
    }

    /// Check if the A button is pressed
    pub fn read_a(&self) -> Option<bool> {
        if self.a.is_low().unwrap() {
            Some(true)
        } else {
            None
        }
    }

    /// Check if the B button is pressed
    pub fn read_b(&self) -> Option<bool> {
        if self.b.is_low().unwrap() {
            Some(true)
        } else {
            None
        }
    }
}
