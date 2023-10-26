//! Tetris for the micro:bit v2
//! Gatlin Newhouse 2023
#![no_main]
#![no_std]

mod mylib;
use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::Board;
use mylib::Inputs;
use panic_rtt_target as _;

/// Main function for the game
#[cfg(not(feature = "demo"))]
#[entry]
fn main() -> ! {
    todo!();
}

/// Main function for the demo of inputs
#[cfg(feature = "demo")]
#[entry]
fn main() -> ! {
    use rtt_target::{rprintln, rtt_init_print};
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut inputs = Inputs::new(board);
    loop {
        inputs.timer.delay_ms(100_u32);
        let data = inputs.accel.read_accel().unwrap();
        rprintln!("x {} y {} z {}", data.0, data.1, data.2);
        if let Some(true) = inputs.buttons.read_a() {
            rprintln!("button a pressed");
        }
        if let Some(true) = inputs.buttons.read_b() {
            rprintln!("button b pressed");
        }
        if let Some(true) = inputs.logo.read_logo() {
            rprintln!("logo pressed");
        }
    }
}
