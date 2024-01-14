//! Module for library functions of the game
//!
//! Wraps inputs for the game based on cargo features enabled
//!
//! Acts as an abstraction layer for what we want to do with the board
#[cfg(feature = "accelerometer")]
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};
use microbit::{
    hal::{
        gpio::{p0::P0_00, Disconnected},
        Timer,
    },
    pac::{TIMER0, TIMER1, TIMER2},
    Board,
};

/// Module for the built-in speaker on the micro:bit v2
pub mod beep;
/// Module for the game logic
pub mod game;
mod rand;

#[cfg(feature = "accelerometer")]
mod accel;
#[cfg(feature = "buttons")]
mod buttons;
/// Module for displaying text on the built-in 5x5 pixeldisplay on the micro:bit v2
#[cfg(feature = "text")]
pub mod font;
#[cfg(feature = "logo")]
mod logo;
#[cfg(feature = "text")]
mod pendolino;
/// Module for the built-in 5x5 pixeldisplay on the micro:bit v2
#[cfg(not(feature = "screen"))]
pub mod pixeldisplay;
#[cfg(feature = "screen")]
mod screen;
#[cfg(not(feature = "screen"))]
mod tetrominos;

/// Inputs for the game as a struct with feature compilation
pub struct GameAbstractionLayer {
    /// A and B buttons on the front of the micro:bit v2
    #[cfg(feature = "buttons")]
    pub buttons: buttons::Buttons,
    /// Logo button on the front of the micro:bit v2
    #[cfg(feature = "logo")]
    pub logo: logo::LogoButton,
    /// Accelerometer sensor on the back of the micro:bit v2
    #[cfg(feature = "accelerometer")]
    pub accel: accel::Accel,
    /// Display timer
    pub display_timer: TIMER0,
    /// Display pins
    #[cfg(not(feature = "screen"))]
    pub display_pins: microbit::gpio::DisplayPins,
    /// Board timer1 as delay
    pub delay: Timer<TIMER1>,
    /// Board timer2 as speaker_timer
    pub speaker_timer: TIMER2,
    /// Speaker pin
    pub speaker_pin: P0_00<Disconnected>,
}

impl GameAbstractionLayer {
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
            display_timer: board.TIMER0,
            #[cfg(not(feature = "screen"))]
            display_pins: board.display_pins,
            delay: Timer::new(board.TIMER1),
            speaker_timer: board.TIMER2,
            speaker_pin: board.speaker_pin,
        }
    }
}
