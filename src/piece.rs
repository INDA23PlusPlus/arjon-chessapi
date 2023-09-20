#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[inline]
    pub(crate) fn flip(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub(crate) piece_type: PieceType,
    pub(crate) color: Color,
}
