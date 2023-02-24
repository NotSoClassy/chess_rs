use crate::board::{ Board };
use crate::piece::{ Piece, PieceType, Color };

const SQUARES_TO_EDGE: [[isize; 8]; 8 * 8] = compile_squares_to_edge();
const KSQUARES_TO_EDGE: [[isize; 8]; 8 * 8] = compile_knight_moves();

                                      /* N  S  W  E  NW  SE  NE  SW */
const DIRECTION_OFFSETS: [isize; 8] = [ -1, 1, -8, 8, -9, -7, 7, 9 ];
const KNIGHT_OFFSETS: [isize; 8] = [ -6, -15, -17, -10, 6, 15, 17, 10 ];
const WHITE_PAWN_OFFSETS: [isize; 2] = [ 7, -9 ];
const BLACK_PAWN_OFFSETS: [isize; 2] = [ 9, -7 ];
const EN_PASSANT_OFFSETS: [isize; 2] = [ 8, -8 ];

#[derive(Debug, Clone, Copy)]
pub enum SpecialMove {
  Promotion,
  EnPassant(usize),
  None
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
  pub start: usize,
  pub target: usize,
  pub special: SpecialMove
}

pub fn new_move(start: usize, target: usize) -> Move {
  Move { start, target, special: SpecialMove::None }
}

fn generate_moves_in_direction(board: &mut Board, start: usize, directions: [isize; 8], direction_idx: usize, squares_to_edge: [[isize; 8]; 8 * 8], max_n: isize) {
  for n in 0 .. squares_to_edge[start][direction_idx] {
    let target = (start as isize + directions[direction_idx] * (n as isize + 1)) as usize;

    if let Some(tile) = board.board.get(target) {
      match tile.piece {
        Some(piece) => {
          /* friendly piece, cant take */
          if piece.color == board.color_to_move { break; }

          /* enemy piece take, and cant move any further */
          board.moves.push(new_move(start, target));
          break;
        }

        /* no piece there, can move */
        None => board.moves.push(new_move(start, target))
      }
    }

    if max_n != 0 && n + 1 >= max_n { break }
  }
}

fn generate_knight_moves(board: &mut Board, start: usize) {
  for idx in 0 .. 8 {
    generate_moves_in_direction(board, start, KNIGHT_OFFSETS, idx, KSQUARES_TO_EDGE, 1);
  }
}

pub fn generate_sliding_moves(board: &mut Board, start: usize, piece: Piece) {
  let start_dir = if piece.piece == PieceType::Bishop { 4 } else { 0 };
  let end_dir = if piece.piece == PieceType::Rook { 4 } else { 8 };

  for direction in start_dir .. end_dir {
    generate_moves_in_direction(board, start, DIRECTION_OFFSETS, direction, SQUARES_TO_EDGE, 0);
  }
}

fn generate_king_moves(board: &mut Board, start: usize) {
  for direction in 0 .. 8 {
    generate_moves_in_direction(board, start, DIRECTION_OFFSETS, direction, SQUARES_TO_EDGE, 1);
  }
}

fn pawn_promotion(board: &mut Board, start: usize, target: usize, pawn_offset: isize, file: usize, last_file: isize) -> bool {
  if file as isize + pawn_offset != last_file { return false }

  board.moves.push(Move { start, target, special: SpecialMove::Promotion });

  return true;
}

fn generate_pawn_moves(board: &mut Board, start: usize, piece: Piece, file: usize) {
  let color = piece.color;
  let pawn_offset: isize = if color == Color::White { -1 } else { 1 };
  let start_file = if board.color_to_move == Color::White { 6 } else { 1 };
  let last_file = if board.color_to_move == Color::White { 0 } else { 7 };

  let one_forward = (start as isize + pawn_offset) as usize;
  let two_forward = (one_forward as isize + pawn_offset) as usize;

  /* moves */

  if board.board[one_forward].piece.is_none() {
    if !pawn_promotion(board, start, one_forward, pawn_offset, file, last_file) {
      board.moves.push(new_move(start, one_forward));
    }
  }

  if start_file == file && board.board[two_forward].piece.is_none() {
    board.moves.push(new_move(start, two_forward));
  }

  /* takes */

  let direction_offset = if color == Color::White { WHITE_PAWN_OFFSETS } else { BLACK_PAWN_OFFSETS };

  for offset in direction_offset {
    let target = (start as isize + offset) as usize;

    if let Some(tile) = board.board.get(target) {
      if let Some(piece) = tile.piece {
        /* cannot capture friendly piece */
        if piece.color == color { continue; }

        if !pawn_promotion(board, start, target, pawn_offset, file, last_file) {
          board.moves.push(new_move(start, target));
        }
      }
    }
  }

  /* en passant */

  if let Some(last_move) = board.last_move {
    if let Some(piece) = board.board[last_move.target].piece {
      if piece.piece != PieceType::Pawn { return }
    }

    for i in 0 .. 2 {
      let other_pawn = (start as isize + EN_PASSANT_OFFSETS[i]) as usize;
      let target = (start as isize + direction_offset[i]) as usize;

      if other_pawn == last_move.target {
        board.moves.push(Move { target, start, special: SpecialMove::EnPassant(other_pawn) });
      }
    }
  }
}

pub fn generate_piece_moves(board: &mut Board, start: usize, file: usize, piece: Piece) {
  match piece.piece {
    PieceType::Bishop | PieceType::Queen | PieceType::Rook => generate_sliding_moves(board, start, piece),
    PieceType::King => generate_king_moves(board, start),
    PieceType::Knight => generate_knight_moves(board, start),
    PieceType::Pawn => generate_pawn_moves(board, start, piece, file)
  }
}

pub fn generate_all_moves(board: &mut Board) {
  board.moves.clear();

  for file in 0 .. 8 {
    for rank in 0 .. 8 {
      let start = rank * 8 + file;
      let piece = board.board[start];

      if let Some(piece) = piece.piece {
        if piece.color == board.color_to_move {
          generate_piece_moves(board, start, file, piece);
        }
      }
    }
  }
}

const fn compile_squares_to_edge() -> [[isize; 8]; 8 * 8] {
  let mut buf = [[0; 8]; 8 * 8];

  let mut i = 0;
  while i < 64 {
    let row_index = i / 8;
    let col_index = i % 8;

    let mut j = 0;
    while j < 8 {
      let direction = DIRECTION_OFFSETS[j];
      let mut distance = 0;
      let mut cur_index = i as isize + direction;

      while cur_index >= 0 && cur_index < 64 as isize && (cur_index as usize) % 8 != 7 && (cur_index as usize) % 8 != 0 {
        let cur_row_index = cur_index as usize / 8;
        let cur_col_index = cur_index as usize % 8;

        distance += 1;
        cur_index += direction;

        if cur_row_index == row_index || cur_col_index == col_index {
          break;
        }
      }

      buf[i][j] = distance;
      j += 1;
    }

    i += 1;
  }

  return buf
}


const fn compile_knight_moves() -> [[isize; 8]; 8 * 8] {
  let mut buf = [[0; 8]; 8 * 8];
  let mut i = 0;

  while i < 64 {
    let row_index = i / 8;
    let col_index = i % 8;
    let mut j = 0;

    while j < 8 {
      let offset = KNIGHT_OFFSETS[j];
      let cur_index = i as isize + offset;
      let cur_row_index = cur_index as usize / 8;
      let cur_col_index = cur_index as usize % 8;

      if cur_index >= 0 && cur_index < 64 as isize &&
         ((cur_col_index as isize - col_index as isize).abs() == 1 &&
          (cur_row_index as isize - row_index as isize).abs() == 2 ||
          (cur_col_index as isize - col_index as isize).abs() == 2 &&
          (cur_row_index as isize - row_index as isize).abs() == 1) {
        buf[i][j] = 1;
      }

      j += 1;
    }

    i += 1;
  }

  buf
}