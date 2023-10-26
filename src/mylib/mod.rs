//! Module for library functions of the game
//! Wraps inputs for the game based on cargo features enabled
//! Gatlin Newhouse 2023
use microbit::{
    hal::{twim, Timer},
    pac::{twim0::frequency::FREQUENCY_A, TIMER0},
    Board,
};

#[cfg(feature = "buttons")]
mod buttons;
#[cfg(feature = "logo")]
mod logo;
#[cfg(feature = "accelerometer")]
mod accel;

/// Inputs for the game as a struct with feature compilation
pub struct Inputs {
    #[cfg(feature = "buttons")]
    pub buttons: buttons::Buttons,
    #[cfg(feature = "logo")]
    pub logo: logo::LogoButton,
    #[cfg(feature = "accelerometer")]
    pub accel: accel::Accel,
    pub timer: Timer<TIMER0>,
}

impl Inputs {
    /// Create a new instance of the inputs as a struct
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
