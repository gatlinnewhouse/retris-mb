//! Tetris for the micro:bit v2
//! Gatlin Newhouse 2023
#![no_main]
#![no_std]

mod mylib;
use crate::mylib::{
    beep::{beep, repeat_beep},
    game::GameState,
    pixeldisplay::display_frame,
    pixeldisplay::Raster,
    GameAbstractionLayer,
};
use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::{
    pac::{interrupt, TIMER0, TIMER2},
    Board,
};
use nanorand::{Pcg64, Rng};
use panic_rtt_target as _;

microbit_display!(TIMER0);
microbit_beep!(TIMER2);

/// Main function for the game
#[entry]
fn main() -> ! {
    use rtt_target::{rprintln, rtt_init_print};
    rtt_init_print!();
    // Tick time in milliseconds
    let tick: u16 = 1000;
    // Take ownership of the Board struct
    let board = Board::take().unwrap();
    // Create our input sources
    let mut gal = GameAbstractionLayer::new(board);
    // Initialize the speaker
    init_beep(gal.speaker_timer, gal.speaker_pin.degrade());
    // Initialize the display
    init_display(gal.display_timer, gal.display_pins);
    beep();
    // Setup the random number generator
    let mut rng = Pcg64::new_seed(1337);
    let mut seed = rng.generate();
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
    // Loop and read input data and print to serial console via probe-rs and rtt
    loop {
        gal.delay.delay_ms(100_u32);
        a = gal.buttons.read_a();
        b = gal.buttons.read_b();
        logo = gal.logo.read_logo();
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
        let mut raster = Raster::default();
        game.step(&mut raster, seed);
        display_frame(&raster);
    }
}
