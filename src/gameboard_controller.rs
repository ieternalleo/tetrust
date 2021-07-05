//! Gameboard Controller.

use piston_window::generic_event::GenericEvent;

use crate::Gameboard;

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Store the gameboard state.
    pub gameboard: Gameboard,
}

impl GameboardController {
    ///Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController { gameboard }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {}
}
