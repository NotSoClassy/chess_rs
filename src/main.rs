extern crate piston_window;
extern crate image;

use piston_window::*;

mod piece;
mod board;
mod sprite;
mod cursor;
mod game_state;
mod move_generator;

use sprite::SpriteHandler;
use game_state::GameState;

const PIECE_WIDTH: f64 = 64.0;
const PIECE_HEIGHT: f64 = 64.0;

fn main() {
  let mut window: PistonWindow =
    WindowSettings::new("Chess", [PIECE_WIDTH * 8.0, PIECE_HEIGHT * 8.0])
    .exit_on_esc(true).build().unwrap();

  let mut texture_ctx = window.create_texture_context();
  let sprite_handler = SpriteHandler::new("assets/pieces.png", &mut texture_ctx);
  let mut state = GameState::new(sprite_handler);

  while let Some(event) = window.next() {
    match &event {
      Event::Input(input, _) => {
        state.input_event(input);
      }

      _ => {}
    }

    state.game_update();

    window.draw_2d(&event, |ctx, graphics, _device| {
      clear([1.0; 4], graphics);
      state.draw_board(ctx, graphics);
      state.draw_selected_piece(ctx, graphics);
    });
  }

  println!("{}", state.board.to_fen());
}