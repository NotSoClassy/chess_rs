use crate::sprite::SpriteHandler;
use crate::board::Board;
use crate::cursor::MouseHandler;

use crate::{ PIECE_HEIGHT, PIECE_WIDTH };

use piston_window::*;

macro_rules! rgb_to_color {
    ($r:expr, $g:expr, $b:expr) => {
      [$r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0, 1.0]
    };
}

const DARK_SQUARE: [f32; 4] = rgb_to_color!(161, 111, 92);
const LIGHT_SQUARE: [f32; 4] = rgb_to_color!(236, 211, 186);
const SELECTED_SQUARE: [f32; 4] = rgb_to_color!(93, 142, 162);
const LEGAL_DARK_SQUARE: [f32; 4] = rgb_to_color!(176, 39, 49);
const LEGAL_LIGHT_SQUARE: [f32; 4] = rgb_to_color!(222, 62, 77);

pub struct GameState<'a> {
  pub sprite_handler: SpriteHandler<'a>,
  pub board: Board,
  pub mouse: MouseHandler,
  paused: bool
}

impl <'a>GameState<'a> {
  pub fn new(sprite_handler: SpriteHandler<'a>) -> Self {
    let mut state = GameState {
      sprite_handler,
      mouse: MouseHandler::new(),
      board: Board::new(),
      paused: false
    };

    state.sprite_handler.load();

    return state;
  }

  pub fn input_event(&mut self, input: &Input) {
    match input {
      Input::Focus(focus) => {
        self.paused = !*focus;
      }

      _ => self.mouse.handle_input(input)
    }
  }

  pub fn game_update(&mut self) {
    if self.paused { return; }

    self.board.select(&mut self.mouse);
  }

  fn get_color(&self, rank: i32, file: i32) -> [f32; 4] {
    let tile = self.board.get_tile_at((rank * 8 + file) as usize);

    if tile.selected == true {
      return SELECTED_SQUARE;
    }

    let is_light_square = (rank + file) % 2 == 0;

    return if is_light_square { LIGHT_SQUARE } else { DARK_SQUARE };
  }

  pub fn draw_board(&self, ctx: Context, graphics: &mut G2d) {
    for file in 0 .. 8 {
      for rank in 0 .. 8 {
        let color = self.get_color(rank, file);
        let (x, y) = (rank as f64 * PIECE_HEIGHT, file as f64 * PIECE_WIDTH);

        rectangle(
          color,
          [x, y, PIECE_WIDTH, PIECE_HEIGHT],
          ctx.transform,
          graphics
        );

        if let Some(piece) = self.board.get_piece_at((rank * 8 + file) as usize) {
          let idx = piece.sprite_sheet_pos();
          let sprite_image = self.sprite_handler.sprites.get(idx).unwrap();
          let transform = ctx.transform.trans(x, y);

          image(sprite_image, transform, graphics);
        }
      }
    }
  }

  pub fn draw_selected_piece(&self, ctx: Context, graphics: &mut G2d) {
    match (self.board.current_select, self.mouse.current) {
      (Some(selected_piece), Some(current)) => {
        self.draw_legal_moves(ctx, graphics);

        let sprite_image = self.sprite_handler.sprites.get(selected_piece.piece.sprite_sheet_pos()).unwrap();
        let transform = ctx.transform.trans(current[0], current[1]);

        image(sprite_image, transform, graphics);
      }

      _ => {}
    }
  }

  pub fn draw_legal_moves(&self, ctx: Context, graphics: &mut G2d) {
    match self.board.current_select {
      Some(_) => {
        for mov in &self.board.moves {
          // cant think of a better way rn
          for file in 0 .. 8 {
            for rank in 0 .. 8 {
              let target = rank * 8 + file;

              if target == mov.target {
                let color = if (rank + file) % 2 == 0 { LEGAL_LIGHT_SQUARE } else { LEGAL_DARK_SQUARE };

                let (x, y) = (rank as f64 * PIECE_HEIGHT, file as f64 * PIECE_WIDTH);

                rectangle(
                  color,
                  [x, y, PIECE_WIDTH, PIECE_HEIGHT],
                  ctx.transform,
                  graphics
                );

                break;
              }
            }
          }
        }

      }

      _ => {}
    }
  }
}