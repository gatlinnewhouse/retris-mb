//! Tetris for the micro:bit v2
//! Gatlin Newhouse 2023
#![no_main]
#![no_std]

mod mylib;
use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::{
    pac::{interrupt, TIMER2},
    Board,
};
use mylib::GameAbstractionLayer;
use panic_rtt_target as _;

microbit_beep!(TIMER2);

/// Main function for the game
#[cfg(not(feature = "debug"))]
#[entry]
fn main() -> ! {
    todo!();
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
        gal.timer0.delay_ms(100_u32);
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
            gal.timer0.delay_ms(100_u32);
        }
        if gal.accel.tilt_right() {
            rprintln!("tilted right");
            repeat_beep(2u8, 75u16, &mut gal.delay);
            gal.timer0.delay_ms(100_u32);
        }
    }
}
