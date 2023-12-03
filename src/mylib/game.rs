//! Module for Game state and logic
//!
//! Helps processs frames, inputs, etc.
use nanorand::{Pcg64, Rng};

#[cfg(not(feature = "screen"))]
use crate::mylib::pixeldisplay::{Raster, Render};
#[cfg(not(feature = "screen"))]
use crate::mylib::tetrominos::get_random_tetromino;
#[cfg(not(feature = "screen"))]
use crate::mylib::tetrominos::rotate_clockwise;

/// Location of a piece, indexed by its bottom left corner
#[derive(Clone, Copy)]
struct PieceLocation {
    row: usize,
    col: usize,
}

/// Initial location of a piece, starts at the top middle
const INITIAL_LOC: PieceLocation = PieceLocation { row: 1, col: 2 };

pub struct GameState {
    /// Current piece falling
    falling_piece: [[u8; 2]; 2],
    /// Location of a piece, indexed by its bottom left corner
    fall_loc: PieceLocation,
}

impl GameState {
    /// Create a new GameState
    pub fn new(tick: u16) -> Self {
        Self {
            falling_piece: [[0; 2]; 2],
            fall_loc: INITIAL_LOC,
        }
    }
    /// Place the piece on the screen
    fn place_piece(&mut self, curr_screen: &mut Raster) {
        // Bottom left of current falling piece is placed at fall_loc
        curr_screen[self.fall_loc.row][self.fall_loc.col] = 9 * self.falling_piece[1][0];
        // Top left
        curr_screen[self.fall_loc.row - 1][self.fall_loc.col] = 9 * self.falling_piece[0][0];
        // Bottom right
        curr_screen[self.fall_loc.row][self.fall_loc.col + 1] = 9 * self.falling_piece[1][1];
        // Top right
        curr_screen[self.fall_loc.row - 1][self.fall_loc.col + 1] = 9 * self.falling_piece[0][1];
    }
    /// Move the currently falling piece left or right
    ///
    /// # Arguments
    /// * `curr_screen` - The current screen state
    ///
    /// # Returns
    /// * The new piece location
    fn move_piece(&mut self, curr_screen: &mut Raster) {
        // Move left
        if self.fall_loc.row > 0 {
            self.fall_loc.row -= 1;
            // Update the screen moving the piece left
        }
        // Move right
        if (self.fall_loc.row + 1) < 4 {
            self.fall_loc.row += 1;
            // Update the screen moving the piece right
        }
    }
    /// Rotate the currently falling piece 90 degrees clockwise
    ///
    /// # Returns
    /// * The rotated piece
    fn rotate_piece(&mut self) -> [[u8; 2]; 2] {
        rotate_clockwise(self.falling_piece)
    }
    /// Drop the currently falling piece down one row
    ///
    /// # Returns
    /// * The new piece location
    fn drop_piece(&mut self) -> PieceLocation {
        self.fall_loc.col += 1;
        self.fall_loc
    }
    /// Add the currently falling piece to the solid blocks on the screen
    /// and reset the falling piece to nothing, reset the fall location to initial
    /// location.
    fn add_piece(&mut self) {
        todo!();
    }
    /// Check if the currently falling piece is colliding with the solid blocks
    /// on the bottom of the screen or with the bottom of the screen
    ///
    /// Adds the currently falling piece to the solid blocks if it is colliding
    /// with either
    ///
    /// # Returns
    /// * Collision location as indexes of the Raster
    fn check_collision(&mut self) -> (usize, usize) {
        todo!();
    }
    /// Function to check for full rows, clear them, and drop the rest down a row
    pub fn clear_row(&mut self, raster: &mut Raster) -> Render {
        todo!();
    }
    /// Step the game state forward one frame
    pub fn step(&mut self, raster: &mut Raster, seed: u128) {
        let mut rng = Pcg64::new_seed(seed);
        if self.falling_piece == [[0; 2]; 2] {
            self.falling_piece = get_random_tetromino(&mut rng);
        }
        self.place_piece(raster);
    }
}
