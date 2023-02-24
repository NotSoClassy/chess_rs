use std::fmt::Debug;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Color {
  Black,
  White
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PieceType {
  Pawn,
  King,
  Rook,
  Queen,
  Knight,
  Bishop
}

#[derive(Clone, Copy)]
pub struct Piece {
  pub color: Color,
  pub piece: PieceType,
  pub moved: bool,
}

impl Debug for Piece {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_fen())?;

    Ok(())
  }
}

impl Color {
  pub fn reverse(&mut self) {
    if *self == Color::White {
      *self = Color::Black;
    } else {
      *self = Color::White;
    }
  }
}

impl Into<char> for PieceType {
  fn into(self) -> char {
    return match self {
      PieceType::Pawn => 'p',
      PieceType::King => 'k',
      PieceType::Rook => 'r',
      PieceType::Queen => 'q',
      PieceType::Knight => 'n',
      PieceType::Bishop => 'b'
    }
  }
}

impl Piece {

  pub fn sprite_sheet_pos(self) -> usize {
    let x = match self.piece {
      PieceType::Pawn => 6,
      PieceType::King => 1,
      PieceType::Rook => 5,
      PieceType::Queen => 2,
      PieceType::Bishop => 3,
      PieceType::Knight => 4
    };

    return if self.color == Color::White { x - 1 } else { x + 5 }; // 5 is number of pieces from 0
  }

  pub fn from_fen(fen: char) -> Option<Self> {
    let mut piece = Piece {
      color: Color::Black,
      piece: PieceType::Pawn,
      moved: false
    };

    if fen.is_uppercase() {
      piece.color = Color::White
    }

    let tt = match fen.to_ascii_lowercase() {
      'p' => Some(PieceType::Pawn),
      'k' => Some(PieceType::King),
      'r' => Some(PieceType::Rook),
      'q' => Some(PieceType::Queen),
      'n' => Some(PieceType::Knight),
      'b' => Some(PieceType::Bishop),

      _ => None
    };

    if let Some(piece_type) = tt {
      piece.piece = piece_type;
      return Some(piece);
    } else {
      return None;
    }
  }

  pub fn to_fen(self) -> char {
    let c = match self.piece {
      PieceType::Pawn => 'p',
      PieceType::King => 'k',
      PieceType::Rook => 'r',
      PieceType::Queen => 'q',
      PieceType::Knight => 'n',
      PieceType::Bishop => 'b'
    };

    return if self.color == Color::White {
      c.to_ascii_uppercase()
    } else {
      c
    }
  }
}