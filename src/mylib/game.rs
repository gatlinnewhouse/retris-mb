//! Module for Game state and logic
//!
//! Helps processs frames, inputs, etc.
use crate::mylib::pixeldisplay::{Raster, Render};

pub struct GameState {}

impl GameState {
    pub fn new(tick: u16) -> Self {
        Self {}
    }

    pub fn tick(&mut self, raster: &mut Raster) -> Render {
        Render::new_from(raster)
    }
}
