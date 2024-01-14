//! Tetris for the micro:bit v2
//! Gatlin Newhouse 2023
#![no_main]
#![no_std]

mod mylib;
#[cfg(feature = "text")]
use crate::mylib::pixeldisplay::scroll_text;
#[cfg(feature = "text")]
use crate::mylib::pixeldisplay::{clear_display, display_frame};
use crate::mylib::{
    beep::repeat_beep, game::GameState, pixeldisplay::Raster, GameAbstractionLayer,
};
use cortex_m_rt::entry;
use microbit::hal::prelude::*;
use microbit::{
    pac::{interrupt, TIMER0, TIMER2},
    Board,
};
use nanorand::{Pcg64, Rng};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

microbit_display!(TIMER0);
microbit_beep!(TIMER2);

/// Main function for the game
#[entry]
fn main() -> ! {
    #[cfg(not(feature = "debug"))]
    play_game();
    #[cfg(feature = "debug")]
    demo_inputs();
}

#[cfg(not(feature = "debug"))]
fn play_game() -> ! {
    // Setup the serial console for panics
    rtt_init_print!();
    // Tick time in milliseconds
    let tick: u16 = 1500;
    // Take ownership of the Board struct
    let board = Board::take().unwrap();
    // Create our input sources
    let mut gal = GameAbstractionLayer::new(board);
    // Initialize the speaker
    init_beep(gal.speaker_timer, gal.speaker_pin.degrade());
    // Initialize the display
    init_display(gal.display_timer, gal.display_pins);
    // Setup the random number generator
    let mut rng = Pcg64::new_seed(1337);
    let mut seed = rng.generate();
    // Set up and run a game.
    let mut game = GameState::new();
    // Set up screen raster
    #[cfg(not(feature = "screen"))]
    let mut raster = Raster::default();
    // Show "TETRIS" on the display
    #[cfg(feature = "text")]
    scroll_text("TETRIS", &mut gal.delay);
    // Clear the display before starting the game
    #[cfg(feature = "text")]
    clear_display();
    // Loop to play game
    loop {
        gal.delay.delay_ms(tick);
        if matches!(gal.buttons.read_a(), Some(true)) {
            game.move_left(&mut raster);
        }
        if matches!(gal.buttons.read_b(), Some(true)) {
            game.move_right(&mut raster);
        }
        if matches!(gal.logo.read_logo(), Some(true)) {
            game.rotate_piece(&mut raster);
        }
        let clr_rows = game.step(&mut raster, seed);
        seed = rng.generate();
        if clr_rows > 0 && clr_rows != 7 {
            // Beep for each row cleared
            repeat_beep(clr_rows, 75u16, &mut gal.delay);
        } else if clr_rows == 7 {
            // Game over
            loop {
                #[cfg(feature = "text")]
                clear_display();
                #[cfg(feature = "text")]
                scroll_text("GAME OVER", &mut gal.delay);
            }
        }
        display_frame(&raster);
    }
}

#[cfg(feature = "debug")]
fn demo_inputs() -> ! {
    // Setup the serial console
    use rtt_target::rprintln;
    // Import beep function
    use crate::mylib::beep::beep;
    rtt_init_print!();
    // Tick time in milliseconds
    let tick: u16 = 1500;
    // Take ownership of the Board struct
    let board = Board::take().unwrap();
    // Create our input sources
    let mut gal = GameAbstractionLayer::new(board);
    // Initialize the speaker
    init_beep(gal.speaker_timer, gal.speaker_pin.degrade());
    // Initialize the display
    init_display(gal.display_timer, gal.display_pins);
    // Beep to indicate start of demo
    beep();
    // Setup the random number generator
    let mut rng = Pcg64::new_seed(0);
    let mut seed = rng.generate();
    // Set up and run a game.
    let mut game = GameState::new();
    // Set up screen raster
    #[cfg(not(feature = "screen"))]
    let mut raster = Raster::default();
    // Loop and read input data and print to serial console via probe-rs and rtt
    loop {
        gal.delay.delay_ms(tick);
        let data = gal.accel.read_accel().unwrap();
        rprintln!("x {} y {} z {}", data.0, data.1, data.2);
        if matches!(gal.buttons.read_a(), Some(true)) {
            rprintln!("button a pressed");
            game.move_left(&mut raster);
            repeat_beep(1u8, 75u16, &mut gal.delay)
        }
        if matches!(gal.buttons.read_b(), Some(true)) {
            rprintln!("button b pressed");
            game.move_right(&mut raster);
            repeat_beep(2u8, 75u16, &mut gal.delay)
        }
        if matches!(gal.logo.read_logo(), Some(true)) {
            rprintln!("logo pressed");
            game.rotate_piece(&mut raster);
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
        let clr_rows = game.step(&mut raster, seed);
        seed = rng.generate();
        if clr_rows > 0 && clr_rows != 7 {
            repeat_beep(clr_rows, 75u16, &mut gal.delay);
        } else if clr_rows == 7 {
            loop {
                #[cfg(feature = "text")]
                clear_display();
                #[cfg(feature = "text")]
                scroll_text("GAME OVER", &mut gal.delay);
            }
        }
        display_frame(&raster);
        rprintln!("row: {} col: {}", game.fall_loc.row, game.fall_loc.col);
    }
}
