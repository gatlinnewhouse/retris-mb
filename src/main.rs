//! Tetris for the micro:bit v2
//! Gatlin Newhouse 2023
#![no_main]
#![no_std]

mod mylib;
use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::{
    pac::{interrupt, TIMER0, TIMER2},
    Board,
};
use mylib::GameAbstractionLayer;
use panic_rtt_target as _;

microbit_display!(TIMER0);
microbit_beep!(TIMER2);

/// Main function for the game
#[cfg(not(feature = "debug"))]
#[entry]
fn main() -> ! {
    // Import the game state and raster
    use mylib::{game::GameState, pixeldisplay::Raster};
    // Tick time in milliseconds
    let tick: u16 = 100;
    // Take ownership of the Board struct
    let board = Board::take().unwrap();
    // Create our input sources
    let mut gal = GameAbstractionLayer::new(board);
    // Initialize the speaker
    init_beep(gal.speaker_timer, gal.speaker_pin.degrade());
    // Initialize the display
    init_display(gal.display_timer, gal.display_pins);
    // Initialize inputs to local variables as aliases
    #[cfg(feature = "buttons")]
    let mut a = gal.buttons.read_a();
    #[cfg(feature = "buttons")]
    let mut b = gal.buttons.read_b();
    #[cfg(feature = "logo")]
    let mut logo = gal.logo.read_logo();
    #[cfg(feature = "accelerometer")]
    let mut tilt_left = gal.accel.tilt_left();
    #[cfg(feature = "accelerometer")]
    let mut tilt_right = gal.accel.tilt_right();
    // Set up and run a game.
    let mut game = GameState::new(tick);
    loop {
        a = gal.buttons.read_a();
        b = gal.buttons.read_b();
        logo = gal.logo.read_logo();
        let mut raster = Raster::default();
    }
}

/// Main function for the demo of inputs
#[cfg(feature = "debug")]
#[entry]
fn main() -> ! {
    use crate::mylib::beep::{beep, repeat_beep};
    use rtt_target::{rprintln, rtt_init_print};
    rtt_init_print!();
    // Take ownership of the Board struct
    let board = Board::take().unwrap();
    // Create our input sources
    let mut gal = GameAbstractionLayer::new(board);
    // Initialize the speaker
    init_beep(gal.speaker_timer, gal.speaker_pin.degrade());
    beep();
    // Loop and read input data and print to serial console via probe-rs and rtt
    loop {
        gal.delay.delay_ms(100_u32);
        let data = gal.accel.read_accel().unwrap();
        rprintln!("x {} y {} z {}", data.0, data.1, data.2);
        if let Some(true) = gal.buttons.read_a() {
            rprintln!("button a pressed");
            repeat_beep(1u8, 75u16, &mut gal.delay)
        }
        if let Some(true) = gal.buttons.read_b() {
            rprintln!("button b pressed");
            repeat_beep(2u8, 75u16, &mut gal.delay)
        }
        if let Some(true) = gal.logo.read_logo() {
            rprintln!("logo pressed");
            repeat_beep(3u8, 75u16, &mut gal.delay)
        }
        if gal.accel.tilt_left() {
            rprintln!("tilted left");
            repeat_beep(1u8, 75u16, &mut gal.delay);
            gal.delay.delay_ms(100_u32);
        }
        if gal.accel.tilt_right() {
            rprintln!("tilted right");
            repeat_beep(2u8, 75u16, &mut gal.delay);
            gal.delay.delay_ms(100_u32);
        }
    }
}
