//! Module for Game state and logic
//!
//! Helps processs frames, inputs, etc.
use nanorand::Pcg64;

#[cfg(not(feature = "screen"))]
use crate::mylib::pixeldisplay::Raster;
#[cfg(not(feature = "screen"))]
use crate::mylib::tetrominos::get_random_tetromino;
#[cfg(not(feature = "screen"))]
use crate::mylib::tetrominos::rotate_clockwise;

/// Location of a piece, indexed by its bottom left corner
#[derive(Clone, Copy)]
pub struct PieceLocation {
    pub row: usize,
    pub col: usize,
}

/// Initial location of a piece, starts at the top middle
const INITIAL_LOC: PieceLocation = PieceLocation { row: 1, col: 2 };

pub struct GameState {
    /// Current piece falling
    #[cfg(not(feature = "screen"))]
    falling_piece: [[u8; 2]; 2],
    /// Location of a piece, indexed by its bottom left corner
    #[cfg(not(feature = "debug"))]
    fall_loc: PieceLocation,
    /// Location of a piece, indexed by its bottom left corner
    #[cfg(feature = "debug")]
    pub fall_loc: PieceLocation,
}

impl GameState {
    /// Create a new GameState
    #[cfg(not(feature = "screen"))]
    pub fn new() -> Self {
        Self {
            falling_piece: [[0; 2]; 2],
            fall_loc: INITIAL_LOC,
        }
    }
    /// Place the piece on the screen
    ///
    /// # Arguments
    /// * `curr_screen` - The current screen state
    #[cfg(not(feature = "screen"))]
    fn place_piece(&mut self, curr_screen: &mut Raster) {
        // Bottom left of current falling piece is placed at fall_loc
        if self.falling_piece[1][0] == 1 && curr_screen[self.fall_loc.row][self.fall_loc.col] == 0 {
            curr_screen[self.fall_loc.row][self.fall_loc.col] = 9 * self.falling_piece[1][0];
        }
        // Top left
        if self.falling_piece[0][0] == 1
            && curr_screen[self.fall_loc.row - 1][self.fall_loc.col] == 0
        {
            curr_screen[self.fall_loc.row - 1][self.fall_loc.col] = 9 * self.falling_piece[0][0];
        }
        // Bottom right
        if self.falling_piece[1][1] == 1
            && curr_screen[self.fall_loc.row][self.fall_loc.col + 1] == 0
        {
            curr_screen[self.fall_loc.row][self.fall_loc.col + 1] = 9 * self.falling_piece[1][1];
        }
        // Top right
        if self.falling_piece[0][1] == 1
            && curr_screen[self.fall_loc.row - 1][self.fall_loc.col + 1] == 0
        {
            curr_screen[self.fall_loc.row - 1][self.fall_loc.col + 1] =
                9 * self.falling_piece[0][1];
        }
    }
    /// Move the currently falling piece left one column
    ///
    /// # Arguments
    /// * `curr_screen` - The current screen state
    #[cfg(not(feature = "screen"))]
    pub fn move_left(&mut self, curr_screen: &mut Raster) {
        if self.fall_loc.col != 0 {
            // Bottom left of current falling piece is placed at fall_loc
            if self.falling_piece[1][0] == 1 {
                curr_screen[self.fall_loc.row][self.fall_loc.col] = 0;
            }
            // Top left
            if self.falling_piece[0][0] == 1 {
                curr_screen[self.fall_loc.row - 1][self.fall_loc.col] = 0;
            }
            // Bottom right
            if self.falling_piece[1][1] == 1 {
                curr_screen[self.fall_loc.row][self.fall_loc.col + 1] = 0;
            }
            // Top right
            if self.falling_piece[0][1] == 1 {
                curr_screen[self.fall_loc.row - 1][self.fall_loc.col + 1] = 0;
            }
            self.fall_loc.col -= 1;
            self.place_piece(curr_screen);
        }
    }
    /// Move the currently falling piece right one column
    ///
    /// # Arguments
    /// * `curr_screen` - The current screen state
    #[cfg(not(feature = "screen"))]
    pub fn move_right(&mut self, curr_screen: &mut Raster) {
        if self.fall_loc.col + 1 != 4 {
            // Bottom left of current falling piece is placed at fall_loc
            if self.falling_piece[1][0] == 1 {
                curr_screen[self.fall_loc.row][self.fall_loc.col] = 0;
            }
            // Top left
            if self.falling_piece[0][0] == 1 {
                curr_screen[self.fall_loc.row - 1][self.fall_loc.col] = 0;
            }
            // Bottom right
            if self.falling_piece[1][1] == 1 {
                curr_screen[self.fall_loc.row][self.fall_loc.col + 1] = 0;
            }
            // Top right
            if self.falling_piece[0][1] == 1 {
                curr_screen[self.fall_loc.row - 1][self.fall_loc.col + 1] = 0;
            }
            self.fall_loc.col += 1;
            self.place_piece(curr_screen);
        }
    }
    /// Rotate the currently falling piece 90 degrees clockwise
    ///
    /// # Arguments
    /// * `curr_screen` - The current screen state
    #[cfg(not(feature = "screen"))]
    pub fn rotate_piece(&mut self, curr_screen: &mut Raster) {
        if self.fall_loc.col != 4 {
            // Bottom left of current falling piece is placed at fall_loc
            curr_screen[self.fall_loc.row][self.fall_loc.col] = 0;
            // Top left
            curr_screen[self.fall_loc.row - 1][self.fall_loc.col] = 0;
            // Bottom right
            curr_screen[self.fall_loc.row][self.fall_loc.col + 1] = 0;
            // Top right
            curr_screen[self.fall_loc.row - 1][self.fall_loc.col + 1] = 0;
            self.falling_piece = rotate_clockwise(self.falling_piece);
            self.place_piece(curr_screen);
        }
    }
    /// Drop the currently falling piece down one row
    ///
    /// # Arguments
    /// * `curr_screen` - The current screen state
    ///
    /// # Returns
    /// * 0 if the game is not over, 7 if the game is over
    #[cfg(not(feature = "screen"))]
    fn drop_piece(&mut self, curr_screen: &mut Raster) -> u8 {
        // Falling piece corners
        let bottom_left = self.falling_piece[1][0];
        let bottom_right = self.falling_piece[1][1];
        let top_left = self.falling_piece[0][0];
        let top_right = self.falling_piece[0][1];
        // Pieces on the board possibly blocking the falling piece
        let top_left_board: u8;
        let top_right_board: u8;
        // If the falling row is 5, then we have a I piece in the top row
        if self.fall_loc.row == 5 {
            top_left_board = curr_screen[self.fall_loc.row - 1][self.fall_loc.col];
            top_right_board = curr_screen[self.fall_loc.row - 1][self.fall_loc.col + 1];
        } else {
            top_left_board = curr_screen[self.fall_loc.row][self.fall_loc.col];
            top_right_board = curr_screen[self.fall_loc.row][self.fall_loc.col + 1];
        }
        // If either bottom corner is 1 and the falling location row is not 4 then
        // move the piece down one row, and increment the falling location row
        if bottom_left == 1 || bottom_right == 1 {
            if self.fall_loc.row == 4 {
                // If this is the final row and there are bottom corners then
                // add it to the solid blocks
                if !self.add_piece(curr_screen) {
                    // If the game is over then return 7
                    return 7;
                }
                return 0;
            } else if self.fall_loc.row != 4 {
                let bottom_left_board = curr_screen[self.fall_loc.row + 1][self.fall_loc.col];
                let bottom_right_board = curr_screen[self.fall_loc.row + 1][self.fall_loc.col + 1];
                if (bottom_left == 1 && bottom_left_board == 5)
                    || (bottom_right == 1 && bottom_right_board == 5)
                    || (top_left == 1 && top_left_board == 5)
                    || (top_right == 1 && top_right_board == 5)
                {
                    // If there is a solid block below the falling piece then
                    // add it to the solid blocks
                    if !self.add_piece(curr_screen) {
                        // If the game is over then return 7
                        return 7;
                    }
                    return 0;
                } else {
                    // Otherwise move it down one row
                    self.fall_loc.row += 1;
                }
            }
            // Set the current falling piece to 0 on the screen
            if bottom_left == 1 {
                curr_screen[self.fall_loc.row - 1][self.fall_loc.col] = 0;
            }
            if bottom_right == 1 {
                curr_screen[self.fall_loc.row - 1][self.fall_loc.col + 1] = 0;
            }
            if top_left == 1 {
                curr_screen[self.fall_loc.row - 2][self.fall_loc.col] = 0;
            }
            if top_right == 1 {
                curr_screen[self.fall_loc.row - 2][self.fall_loc.col + 1] = 0;
            }
        }
        // If both bottom corners are 0 and the falling location row is 4 then
        // move the piece down one row, and increment the falling location row
        if bottom_left == 0 && bottom_right == 0 {
            if self.fall_loc.row == 5 {
                // If this is the final row and there are not bottom corners then
                // add it to the solid blocks
                if !self.add_piece(curr_screen) {
                    // If the game is over then return 7
                    return 7;
                }
                curr_screen[self.fall_loc.row - 1][self.fall_loc.col] = 0;
                curr_screen[self.fall_loc.row - 1][self.fall_loc.col + 1] = 0;
            } else if top_left_board == 5 || top_right_board == 5 {
                // If there is a solid block below the falling piece then
                // add it to the solid blocks
                if !self.add_piece(curr_screen) {
                    // If the game is over then return 7
                    return 7;
                }
                return 0;
            } else {
                // Otherwise move it down one row
                self.fall_loc.row += 1;
                // Set the current falling piece to 0 on the screen
                if top_left == 1 {
                    curr_screen[self.fall_loc.row - 2][self.fall_loc.col] = 0;
                }
                if top_right == 1 {
                    curr_screen[self.fall_loc.row - 2][self.fall_loc.col + 1] = 0;
                }
            }
        }
        // Place the piece on the screen
        self.place_piece(curr_screen);
        // Return 0 if the game is not over
        0
    }
    /// Add the currently falling piece to the solid blocks on the screen
    /// and reset the falling piece to nothing, reset the fall location to initial
    /// location.
    ///
    /// # Arguments
    /// * `curr_screen` - The current screen state
    ///
    /// # Returns
    /// * True if a piece was added, false if the game is over
    #[cfg(not(feature = "screen"))]
    fn add_piece(&mut self, curr_screen: &mut Raster) -> bool {
        // Bottom left of current falling piece is placed at fall_loc
        if self.falling_piece[1][0] == 1 {
            curr_screen[self.fall_loc.row][self.fall_loc.col] = 5 * self.falling_piece[1][0];
        }
        // Top left
        if self.falling_piece[0][0] == 1 {
            curr_screen[self.fall_loc.row - 1][self.fall_loc.col] = 5 * self.falling_piece[0][0];
        }
        // Bottom right
        if self.falling_piece[1][1] == 1 {
            curr_screen[self.fall_loc.row][self.fall_loc.col + 1] = 5 * self.falling_piece[1][1];
        }
        // Top right
        if self.falling_piece[0][1] == 1 {
            curr_screen[self.fall_loc.row - 1][self.fall_loc.col + 1] =
                5 * self.falling_piece[0][1];
        }
        // Check if any solid blocks are in the top row
        if self.check_column(curr_screen) {
            return false;
        }
        // Reset the falling piece and fall location
        self.falling_piece = [[0; 2]; 2];
        self.fall_loc = INITIAL_LOC;
        // Return true if a piece was added
        true
    }
    /// Function to check for full rows, clear them, and drop the rest down a row
    ///
    /// # Arguments
    /// * `raster` - The current screen state
    ///
    /// # Returns
    /// * The number of rows cleared
    #[cfg(not(feature = "screen"))]
    pub fn check_rows(&mut self, raster: &mut Raster) -> u8 {
        let mut count: u8 = 0;
        let mut full_rows: [bool; 5] = [false; 5];
        // Check for full rows, then keep track of them in full_rows
        for row in 0..5 {
            if raster[row][0] == 5
                && raster[row][1] == 5
                && raster[row][2] == 5
                && raster[row][3] == 5
                && raster[row][4] == 5
            {
                full_rows[row] = true;
                count += 1;
            }
        }
        // If no rows are full then return
        if count == 0 {
            return count;
        }
        // Clear full rows
        for row in (0..5).rev() {
            if full_rows[row] {
                // If it's a full row then clear it
                for col in 0..5 {
                    raster[row][col] = 0;
                }
            }
        }
        // Move any non-full rows down
        for row in (0..5).rev() {
            for col in 0..5 {
                if raster[row][col] == 5 {
                    // If it is a solid block then move it down
                    let mut curr_row = row;
                    while curr_row != 4 && raster[curr_row + 1][col] == 0 {
                        raster[curr_row + 1][col] = 5;
                        raster[curr_row][col] = 0;
                        curr_row += 1;
                    }
                }
            }
        }
        count
    }
    /// Check if there are any solid blocks in the top row
    ///
    /// # Arguments
    /// * `raster` - The current screen state
    ///
    /// # Returns
    /// * True if there are blocks in the top row, false otherwise
    #[cfg(not(feature = "screen"))]
    fn check_column(&mut self, raster: &mut Raster) -> bool {
        for col in 0..5 {
            if raster[0][col] == 5 {
                return true;
            }
        }
        false
    }
    /// Step the game state forward one frame
    ///
    /// # Arguments
    /// * `raster` - The current screen state
    /// * `seed` - The seed for the random number generator
    ///
    /// # Returns
    /// * The number of rows cleared or 7 if the game is over
    pub fn step(&mut self, raster: &mut Raster, seed: u128) -> u8 {
        let mut rng = Pcg64::new_seed(seed);
        if self.falling_piece == [[0; 2]; 2] {
            self.falling_piece = get_random_tetromino(&mut rng);
            self.place_piece(raster);
        } else {
            if self.drop_piece(raster) == 7 {
                return 7;
            }
            return self.check_rows(raster);
        }
        0
    }
}
