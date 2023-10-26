//! Touch logo sensor as modified from example branch `touch-logo`
//! https://github.com/pdx-cs-embedded-rust/mb2-touch
//! Thanks Bart!
use microbit::hal::{
    gpio::{p1::P1_04, Floating, Input},
    prelude::InputPin,
};

/// Material needed for button presses
pub struct LogoButton {
    /// Logo button
    pub logo: P1_04<Input<Floating>>,
}

impl LogoButton {
    /// Set up the button
    pub fn new(logo: P1_04<Input<Floating>>) -> Self {
        Self { logo }
    }

    /// Check if the logo button is pressed
    pub fn read_logo(&self) -> Option<bool> {
        if self.logo.is_low().unwrap() {
            Some(true)
        } else {
            None
        }
    }
}
