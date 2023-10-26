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
use mylib::Inputs;
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
    use rtt_target::{rprintln, rtt_init_print};
    use crate::mylib::beep::{beep, repeat_beep};
    rtt_init_print!();
    // Take ownership of the Board struct
    let board = Board::take().unwrap();
    // Create our input sources
    let mut inputs = Inputs::new(board);
    // Initialize the speaker
    init_beep(inputs.speaker_timer, inputs.speaker_pin.degrade());
    beep();
    // Loop and read input data and print to serial console via probe-rs and rtt
    loop {
        inputs.timer0.delay_ms(100_u32);
        let data = inputs.accel.read_accel().unwrap();
        rprintln!("x {} y {} z {}", data.0, data.1, data.2);
        if let Some(true) = inputs.buttons.read_a() {
            rprintln!("button a pressed");
            repeat_beep(1u8, 75u16, &mut inputs.delay)
        }
        if let Some(true) = inputs.buttons.read_b() {
            rprintln!("button b pressed");
            repeat_beep(2u8, 75u16, &mut inputs.delay)
        }
        if let Some(true) = inputs.logo.read_logo() {
            rprintln!("logo pressed");
            repeat_beep(3u8, 75u16, &mut inputs.delay)
        }
    }
}
