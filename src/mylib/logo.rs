//! Use the logo capacitive touch as a control for the game
//!
//! As modified from example branch `touch-logo`
//!
//! Original source:
//! <https://github.com/pdx-cs-embedded-rust/mb2-touch>
//!
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
    ///
    /// # Arguments
    /// * `logo` - The logo button pin as a floating input
    ///
    /// # Returns
    /// * `Self` - The logo button as a struct
    pub fn new(logo: P1_04<Input<Floating>>) -> Self {
        Self { logo }
    }

    /// Check if the logo button is pressed
    ///
    /// # Returns
    /// * `Option<bool>` - Whether the logo button is pressed, Some(true) if pressed, None if not pressed
    pub fn read_logo(&self) -> Option<bool> {
        if self.logo.is_low().unwrap() {
            Some(true)
        } else {
            None
        }
    }
}
