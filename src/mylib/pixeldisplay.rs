//! Pixel LED display grid
//!
//! For use when not using a Adafruit 0.96” 160x80 Color TFT Display
//! Generic MicroBit v2 nonblocking display handler.
#[cfg(feature = "text")]
use crate::mylib::font::{character, wide_char};
use crate::DISPLAY;
#[cfg(feature = "text")]
use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use microbit::display::nonblocking::GreyscaleImage;
#[cfg(feature = "text")]
use microbit::hal::Timer;
#[cfg(feature = "text")]
use microbit::pac::TIMER1;

/// LED array proxy for rendering. Indexed as row and then column.
pub type Raster = [[u8; 5]; 5];

/// Wrapper around a Raster to make it easier to use.
#[cfg(feature = "text")]
pub struct Render {
    raster: Raster,
}

/// Implementations for Render.
#[cfg(feature = "text")]
impl Render {
    /// Create a new Render from a Raster.
    ///
    /// # Arguments
    /// * `raster` - A borrowed Raster type
    pub const fn new_from(raster: &Raster) -> Self {
        Self { raster: *raster }
    }
    /// Get the Raster from a Render.
    ///
    /// # Returns
    /// * A borrowed Raster type of the current Render
    pub fn get(&self) -> &Raster {
        &self.raster
    }
}

/// Macro for declaring the stuff needed for the non-blocking display.
///
/// Argument is which timer to be used, in all caps: for example, `TIMER0`.
#[macro_export]
macro_rules! microbit_display {
    ($timer:ident) => {
        /// Global state of display.
        pub static DISPLAY: cortex_m::interrupt::Mutex<
            RefCell<Option<microbit::display::nonblocking::Display<$timer>>>,
        > = cortex_m::interrupt::Mutex::new(RefCell::new(None));

        /// Display timer handler.
        #[interrupt]
        fn $timer() {
            cortex_m::interrupt::free(|cs| {
                if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                    d.handle_display_event();
                }
            });
        }

        /// Set up the display. This must be called before first use.
        pub fn init_display(timer: $timer, display_pins: microbit::gpio::DisplayPins) {
            use microbit::display::nonblocking::{Display, GreyscaleImage};

            let mut display = Display::new(timer, display_pins);
            let image = GreyscaleImage::blank();
            cortex_m::interrupt::free(|cs| {
                display.show(&image);
                *DISPLAY.borrow(cs).borrow_mut() = Some(display);
                unsafe {
                    microbit::pac::NVIC::unmask(microbit::pac::Interrupt::$timer);
                }
            });
        }
    };
}

/// Display a frame on the micro:bit v2's pixel display
///
/// # Arguments
/// * `raster` - A borrowed Raster type
pub fn display_frame(raster: &Raster) {
    let frame = GreyscaleImage::new(raster);
    cortex_m::interrupt::free(|cs| {
        if let Some(d) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            d.show(&frame);
        }
    });
}

/// Clear the micro:bit v2's pixel display
#[cfg(feature = "text")]
pub fn clear_display() {
    let clear = Raster::default();
    display_frame(&clear);
}

/// Scrolling text on the micro:bit v2's pixel display
///
/// # Arguments
/// * `s` - A string slice
/// * `board_timer` - A mutable reference to a TIMER1 timer
#[cfg(feature = "text")]
pub fn scroll_text(s: &str, board_timer: &mut Timer<TIMER1>) {
    let mut frame = Raster::default();
    display_frame(&frame);
    let mut prior_char = ' ';
    for (_idx, char) in s.char_indices() {
        // if the char is a space then just add x empty columns to the left
        // side of the frame, when x is three for a wide char and two otherwise
        let x = if wide_char(prior_char) { 3 } else { 2 };
        if char == ' ' {
            for _ in 0..x {
                for row in frame.iter_mut() {
                    for k in 1..5 {
                        row[k - 1] = row[k];
                    }
                    row[4] = 0;
                }
                display_frame(&frame);
                board_timer.delay_ms(100u16);
            }
            continue;
        }
        let char_frame = character(char as u8).get();
        // shift char_frame characters left into the right side of frame
        for i in 0..5 {
            if i != 0 {
                // shift all columns left
                for row in frame.iter_mut() {
                    for k in 1..5 {
                        row[k - 1] = row[k];
                    }
                }
            }
            for j in 0..5 {
                frame[j][4] = char_frame[j][i];
            }
            display_frame(&frame);
            board_timer.delay_ms(100u16);
        }
        // shift all the columns left and add a column of 0s to the right side
        // twice if a character is "wide"
        let w = if wide_char(char) { 2 } else { 1 };
        for _ in 0..w {
            for row in frame.iter_mut() {
                for k in 1..5 {
                    row[k - 1] = row[k];
                }
                row[4] = 0;
            }
            display_frame(&frame);
        }
        prior_char = char;
    }
    // Clear the display by scrolling into a blank frame
    for _ in 0..5 {
        for row in frame.iter_mut() {
            for k in 1..5 {
                row[k - 1] = row[k];
            }
            row[4] = 0;
        }
        display_frame(&frame);
        board_timer.delay_ms(100u16);
    }
}
