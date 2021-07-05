//! Gameboard view
use crate::gameboard_controller::GameboardController;
use piston_window::types::Color;
use piston_window::{color, Context, Graphics};
/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of game board along horizontal and vertical edge.
    pub size: (f64, f64),
    /// BG color
    pub bg_color: Color,
    /// Border Color
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge radiues around the whole board
    pub board_edge_radius: f64,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [56.0; 2],
            size: (400.0, 400.0),
            bg_color: color::BLACK,
            border_color: color::CYAN,
            board_edge_color: color::YELLOW,
            board_edge_radius: 3.0,
        }
    }
}
/// Stores visual information about a gameboard
pub struct GameBoardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

impl GameBoardView {
    pub fn new(settings: GameboardViewSettings) -> GameBoardView {
        GameBoardView { settings }
    }
    // Draw gameboard
    pub fn draw<G: Graphics>(&self, controller: &GameboardController, c: &Context, g: &mut G) {
        use piston_window::{Line, Rectangle};

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0],
            settings.position[1],
            settings.size.0,
            settings.size.1,
        ];

        // Draw board background
        Rectangle::new(settings.bg_color).draw(board_rect, &c.draw_state, c.transform, g);

        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );
    }
}
