#![deny(missing_docs)]

//! A Tetris clone
pub use crate::gameboard::Gameboard;
mod gameboard;
mod gameboard_controller;
mod gameboard_views;

extern crate piston_window;

use crate::gameboard_controller::GameboardController;
use crate::gameboard_views::{GameBoardView, GameboardViewSettings};
use piston_window::grid::Grid;
use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut wnd_settings: WindowSettings = WindowSettings::new("Tetrust", [512; 2])
        .exit_on_esc(true)
        .graphics_api(opengl);
    let mut wnd: PistonWindow = wnd_settings.build().unwrap();

    let mut gb = Gameboard::new();
    let mut gb_controller = GameboardController::new(gb);
    let gb_vw_settings = GameboardViewSettings::new();
    let gb_view = GameBoardView::new(gb_vw_settings);

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut wnd) {
        if let Some(args) = event.render_args() {
            wnd.draw_2d(&event, |c, g, _dev| {
                clear(color::WHITE, g);
                gb_view.draw(&gb_controller, &c, g);
            });
        }
    }
}
