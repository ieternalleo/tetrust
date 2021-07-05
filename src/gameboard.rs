//! Game board logic.

/// Size of game board;
const SIZE: (usize, usize) = (40, 10);

/// Stores game board information.
pub struct Gameboard {
    /// Stores the content of the cells.
    /// '0' is an empty cell
    pub cells: [[u8; SIZE.1]; SIZE.0],
}

impl Gameboard {
    /// Create a new game board
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE.1]; SIZE.0],
        }
    }
}
