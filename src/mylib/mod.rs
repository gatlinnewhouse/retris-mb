//! Module for library functions of the game
//!
//! Wraps inputs for the game based on cargo features enabled
//!
//! Acts as an abstraction layer for what we want to do with the board
use microbit::{
    hal::{twim, Timer},
    pac::{twim0::frequency::FREQUENCY_A, TIMER0},
    Board,
};

mod beep;

#[cfg(feature = "accelerometer")]
mod accel;
#[cfg(feature = "buttons")]
mod buttons;
#[cfg(feature = "logo")]
mod logo;
#[cfg(not(feature = "screen"))]
mod pixeldisplay;
#[cfg(feature = "screen")]
mod screen;
#[cfg(not(feature = "screen"))]
mod tetrominos;

/// Inputs for the game as a struct with feature compilation
pub struct Inputs {
    /// A and B buttons on the front of the micro:bit v2
    #[cfg(feature = "buttons")]
    pub buttons: buttons::Buttons,
    /// Logo button on the front of the micro:bit v2
    #[cfg(feature = "logo")]
    pub logo: logo::LogoButton,
    /// Accelerometer sensor on the back of the micro:bit v2
    #[cfg(feature = "accelerometer")]
    pub accel: accel::Accel,
    /// Board timer for delays
    pub timer: Timer<TIMER0>,
}

impl Inputs {
    /// Create a new instance of the inputs as a struct
    ///
    /// # Arguments
    /// * `board` - The micro:bit v2 board struct after taking ownership
    ///
    /// # Returns
    /// * `Self` - The feature enabled inputs as a struct
    pub fn new(board: Board) -> Self {
        Self {
            #[cfg(feature = "buttons")]
            buttons: buttons::Buttons::new(board.buttons.button_a, board.buttons.button_b),
            #[cfg(feature = "logo")]
            logo: logo::LogoButton::new(board.pins.p1_04.into_floating_input()),
            #[cfg(feature = "accelerometer")]
            accel: {
                let i2c =
                    { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
                accel::Accel::new(i2c)
            },
            timer: Timer::new(board.TIMER0),
        }
    }
}
