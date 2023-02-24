use std::fmt::Debug;

use crate::cursor::MouseHandler;
use crate::piece::{Piece, Color, PieceType};

use crate::{ PIECE_WIDTH, PIECE_HEIGHT };

use crate::move_generator::*;

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const DEFAULT_BOARD: [Tile; 8 * 8] = [Tile { piece: None, selected: false }; 8 * 8];

#[derive(Clone, Copy)]
pub struct Tile {
  pub piece: Option<Piece>,
  pub selected: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SelectedPiece {
  pub piece: Piece,
  pub pos: [f64; 2],
  pub origin_file: usize,
  pub origin_rank: usize,
  pub origin: usize,
  pub dest: Option<usize>
}

pub struct Board {
  pub board: [Tile; 8 * 8],
  pub current_select: Option<SelectedPiece>,
  pub color_to_move: Color,
  pub moves: Vec<Move>,
  pub last_move: Option<Move>,
  stack: Vec<(usize, Option<Piece>)>,
  old_stack: Vec<(usize, Option<Piece>)>,
  selected: Vec<usize>
}

impl Debug for Tile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.piece {
      Some(piece) => write!(f, "{:?}", piece)?,
      None => write!(f, "#")?
    }

    Ok(())
  }
}

impl Debug for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

    for file in 0 .. 8 {
      for rank in 0 .. 8 {
        write!(f, "{:?}{}{}",
          self.board[rank * 8 + file], rank * 8 + file,
          if rank < 7 { ", " } else { " " }
        )?;
      }

      write!(f, "\n")?;
    }

    Ok(())
  }
}

impl Board {
  pub fn new() -> Self {
    return Board::from_fen(DEFAULT_FEN);
  }

  fn push_move(&mut self, idx: usize, piece: Option<Piece>) {
    self.stack.push((idx, piece))
  }

  fn commit_moves(&mut self) {
    for (idx, piece) in &self.stack {
      self.old_stack.push((*idx, *piece));
      self.board[*idx].piece = *piece;
    }

    self.stack.clear()
  }

  fn undo_commit(&mut self) {
    for (idx, piece) in &self.old_stack {
      self.board[*idx].piece = *piece;
    }

    self.clear_stacks();
  }

  fn clear_stacks(&mut self) {
    self.old_stack.clear();
    self.stack.clear();
  }

  pub fn try_move(&mut self, select: SelectedPiece) {
    self.unhighlight_tiles();

    let mut piece = select.piece;
    //let last_last_move = self.last_move;
    let dest = select.dest.unwrap();
    let origin = select.origin;

    macro_rules! reset {
      () => {
        self.moves.clear();
        self.clear_stacks();
        self.board[origin].piece = Some(piece);
      };
    }

    if piece.color != self.color_to_move {
      reset!();
      return;
    }

    for mov in &self.moves {
      let mov = mov.clone();

      if mov.start == origin && mov.target == dest {

        self.push_move(origin, None);

        match mov.special {
          SpecialMove::EnPassant(target_pawn) => {
            self.push_move(dest, Some(piece));
            self.push_move(target_pawn, None);
          }

          SpecialMove::Promotion => {
            self.push_move(dest, Some(Piece { color: piece.color, piece: PieceType::Queen, moved: true }));
          }

          SpecialMove::None => {
            self.push_move(dest, Some(piece));
          }
        }

        self.last_move = Some(mov);
        self.commit_moves();

        /*generate_all_moves(self);

        for mov in &self.moves {
          let target_piece = self.board[mov.target].piece;
          if let Some(target_piece) = target_piece {
            if target_piece.piece == PieceType::King && target_piece.color == piece.color {
              self.undo_commit();
              self.last_move = last_last_move;
              reset!();
              return
            }
          }
        }*/

        self.highlight_tile(origin);
        self.highlight_tile(dest);

        self.color_to_move.reverse();
        piece.moved = true;

        println!("{:?}", self);

        self.moves.clear();
        return
      }
    }

    reset!();
  }



  pub fn select(&mut self, mouse: &mut MouseHandler) {
    if mouse.started_drag == false { return; }

    match self.current_select {
      Some(_) => {
        if !mouse.drag_completed { return; }

        match (self.get_board_index_from_pos(mouse.end.unwrap()), self.current_select) {
          (Some((idx, _, _)), Some(mut piece)) => {
            piece.dest = Some(idx);
            self.try_move(piece);
          }

          (None, Some(piece)) => {
            self.board[piece.origin].piece = Some(piece.piece);
          }

          _ => {}
        }

        mouse.reset_drag();
        self.current_select = None;
      }

      _ => {
        let ret = self.get_board_index_from_pos(mouse.start.unwrap());

        if ret.is_none() { return; }

        let (pos, rank, file) = ret.unwrap();

        let tile = self.board[pos];

        if tile.piece.is_none() { return };

        generate_piece_moves(self, pos, file, tile.piece.unwrap());

        self.current_select = Some(SelectedPiece {
          piece: tile.piece.unwrap(),
          origin_file: file,
          origin_rank: rank,
          origin: pos,
          pos: mouse.current.unwrap(),
          dest: None
        });

        self.board[pos].piece = None;
      }
    }
  }

  pub fn unhighlight_tiles(&mut self) {
    let mut pos = self.selected.pop();

    while let Some(idx) = pos {
      self.board[idx].selected = false;
      pos = self.selected.pop();
    }
  }

  pub fn highlight_tile(&mut self, idx: usize) {
    self.selected.push(idx);
    self.board[idx].selected = true;
  }

  pub fn get_piece_at(&self, idx: usize) -> Option<Piece> {
    self.board[idx].piece
  }

  pub fn get_tile_at(&self, idx: usize) -> Tile {
    self.board[idx]
  }

  fn is_inside_rect(&self, point: [f64; 2], rect: [f64; 4]) -> bool {
    let (x, y) = (point[0], point[1]);
    let (left, top, width, height) = (rect[0], rect[1], rect[2], rect[3]);
    x >= left && x <= left + width && y >= top && y <= top + height
  }

  pub fn get_board_index_from_pos(&self, position: [f64; 2]) -> Option<(usize, usize, usize)> {
    for file in 0 .. 8 {
      for rank in 0 .. 8 {
        let rect = [
          rank as f64 * PIECE_HEIGHT,
          file as f64 * PIECE_WIDTH,
          PIECE_WIDTH,
          PIECE_HEIGHT
        ];

        if self.is_inside_rect(position, rect) {
          return Some((rank * 8 + file, rank, file));
        }
      }
    }

    return None;
  }

  pub fn from_fen(fen: &str) -> Self {
    let mut board = Board {
      board: DEFAULT_BOARD.clone(),
      current_select: None,
      moves: Vec::new(),
      color_to_move: Color::White,
      last_move: None,
      selected: Vec::new(),
      old_stack: Vec::new(),
      stack: Vec::new()
    };

    let mut file = 0;
    let mut rank = 0;

    for c in fen.chars() {
      if c == '/' {
        rank = 0;
        file += 1;
      } else {
        if c.is_numeric() {
          rank += c.to_digit(10).unwrap();
        } else {
          let piece = Piece::from_fen(c);

          if let Some(piece) = piece {
            board.board[(rank * 8 + file) as usize].piece = Some(piece);
            rank += 1;
          } else {
            break;
          }
        }
      }
    }

    return board;
  }

  pub fn to_fen(&self) -> String {
    let mut fen = String::new();

    let mut skip = 0;

    macro_rules! push_skip {
      () => {
        if skip > 0 {
          fen.push_str(skip.to_string().as_str());
          skip = 0;
        }
      };
    }

    for file in 0 .. 8 {
      for rank in 0 .. 8 {
        let piece = self.get_piece_at(rank * 8 + file);

        match piece {
          Some(piece) => {
            push_skip!();

            fen.push(piece.to_fen());
          }

          None => {
            skip += 1;
          }
        }
      }

      push_skip!();

      if file < 7 { fen.push('/'); }
    }

    return fen;
  }
}