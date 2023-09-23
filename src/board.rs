// This needs to be a macro, otherwise the borrow checker goes bonkers
macro_rules! at {
    ($self:expr, $pos:expr) => {
        $self.board[$pos.row as usize][$pos.col as usize]
    };
}

pub mod default;
pub mod generate_moves;
mod is_attacked;
pub mod is_legal;

pub use self::default::*;
pub use self::generate_moves::*;
pub use self::is_legal::*;
pub const BOARD_ROW_COUNT: usize = 8;
pub const BOARD_COL_COUNT: usize = 8;
pub(crate) const BLACK_PIECE_STARTING_ROW: i8 = 0;
pub(crate) const WHITE_PIECE_STARTING_ROW: i8 = BOARD_ROW_COUNT as i8 - 1;
pub(crate) const BLACK_PAWN_STARTING_ROW: i8 = BLACK_PIECE_STARTING_ROW + 1;
pub(crate) const WHITE_PAWN_STARTING_ROW: i8 = WHITE_PIECE_STARTING_ROW - 1;
pub(crate) const KING_STARTING_COL: i8 = 4;
pub(crate) const ROOK_LONG_STARTING_COL: i8 = 0;
pub(crate) const ROOK_SHORT_STARTING_COL: i8 = BOARD_COL_COUNT as i8 - 1;
pub(crate) const WHITE_KING_STARTING_POS: Position = Position {
    row: WHITE_PIECE_STARTING_ROW,
    col: KING_STARTING_COL,
};
pub(crate) const BLACK_KING_STARTING_POS: Position = Position {
    row: BLACK_PIECE_STARTING_ROW,
    col: KING_STARTING_COL,
};
pub(crate) const BLACK_ROOK_LONG_STARTING_POS: Position = Position {
    row: BLACK_PIECE_STARTING_ROW,
    col: ROOK_LONG_STARTING_COL,
};
pub(crate) const BLACK_ROOK_SHORT_STARTING_POS: Position = Position {
    row: BLACK_PIECE_STARTING_ROW,
    col: ROOK_SHORT_STARTING_COL,
};
pub(crate) const WHITE_ROOK_LONG_STARTING_POS: Position = Position {
    row: WHITE_PIECE_STARTING_ROW,
    col: ROOK_LONG_STARTING_COL,
};
pub(crate) const WHITE_ROOK_SHORT_STARTING_POS: Position = Position {
    row: WHITE_PIECE_STARTING_ROW,
    col: ROOK_SHORT_STARTING_COL,
};
pub(crate) const WHITE_EN_PASSANT_FROM_ROW: i8 = 3;
pub(crate) const BLACK_EN_PASSANT_FROM_ROW: i8 = 4;

extern crate num_traits;

use self::num_traits::*;
use crate::piece::{Color, Piece, PieceType};
use std::mem;
use std::ops::*;

// None => empty square, Some(Piece) => square occupied by Piece
pub type SquareType = Option<Piece>;
pub type RefSquareType<'a> = Option<&'a Piece>;
pub type MutSquareType<'a> = Option<&'a mut Piece>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BoardError {
    IllegalMove,
}

#[derive(Debug, Clone)]
pub struct Board {
    // The representation of the board. Value is Some(Piece) if occupied, otherwise None
    // Public in lib.rs for debugging purposes
    pub(super) board: [[SquareType; 8]; 8],

    // Useful for checking if king will be attacked after a specific move
    white_king_pos: Position,
    black_king_pos: Position,

    // If castling is still available for black or white. Doesn't mean that it is a legal move
    short_castle_white: bool,
    long_castle_white: bool,
    short_castle_black: bool,
    long_castle_black: bool,

    // The column that you are able to do en passant to, or -1 if unavailable
    en_passant_col: i8,

    turn: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: i8,
    pub col: i8,
}

impl Position {
    #[inline]
    fn abs(&self) -> Self {
        Self {
            row: self.row.abs(),
            col: self.col.abs(),
        }
    }
}

impl Add for Position {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}

impl Sub for Position {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.col += rhs.col;
        self.row += rhs.row;
    }
}

impl<T: PrimInt + Into<i8> + From<i8>> Div<T> for Position {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let rhs_i8: i8 = rhs.into();
        Self {
            col: self.col / rhs_i8,
            row: self.row / rhs_i8,
        }
    }
}

impl Position {
    pub(crate) fn out_of_bounds(&self) -> bool {
        self.col < 0
            || self.col >= BOARD_COL_COUNT as i8
            || self.row < 0
            || self.row >= BOARD_ROW_COUNT as i8
    }
}

// Pass this struct as an argument when you want to make a move
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    // Set this to Some(PieceType) if promotion is necessary
    pub promotion: Option<PieceType>,
}

impl Board {
    // Generates a new board with the default starting position
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }
    #[inline]
    pub fn get_board(&self) -> [[SquareType; 8]; 8] {
        self.board.clone()
    }

    pub fn whose_turn(&self) -> Color {
        self.turn.clone()
    }

    /*
     * Iterate through a all squares and and call get_legal_moves_piece() on every square that
     * holds a piece of the correct color
     */

    #[inline]
    fn dir(color: &Color) -> i8 {
        match color {
            Color::White => -1,
            Color::Black => 1,
        }
    }

    pub fn is_stalemate(&self) -> bool {
        self.generate_legal_moves().is_empty()
            && !self.is_attacked_by_opponent(&match self.turn {
                Color::White => self.white_king_pos,
                Color::Black => self.black_king_pos,
            })
    }

    pub fn is_checkmate(&self) -> bool {
        self.generate_legal_moves().is_empty()
            && self.is_attacked_by_opponent(&match self.turn {
                Color::White => self.white_king_pos,
                Color::Black => self.black_king_pos,
            })
    }

    pub(crate) fn print_board(&self) {
        for row in 0..BOARD_ROW_COUNT {
            for col in 0..BOARD_COL_COUNT {
                print!(
                    "{}",
                    self.board[row][col]
                        .as_ref()
                        .map_or("·", |piece| match piece.color {
                            Color::White => match piece.piece_type {
                                PieceType::Pawn => "P",
                                PieceType::Rook => "R",
                                PieceType::Knight => "N",
                                PieceType::Bishop => "B",
                                PieceType::Queen => "Q",
                                PieceType::King => "K",
                            },
                            Color::Black => match piece.piece_type {
                                PieceType::Pawn => "p",
                                PieceType::Rook => "r",
                                PieceType::Knight => "n",
                                PieceType::Bishop => "b",
                                PieceType::Queen => "q",
                                PieceType::King => "k",
                            },
                        })
                );
            }
            println!();
        }
        println!();
    }

    // Makes a move without checking if it's legal
    fn unsafe_make_move(&mut self, mv: &Move) -> Result<(), BoardError> {
        self.turn = self.turn.flip();

        // Reset en passant
        self.en_passant_col = -1;

        let piece = match at!(self, mv.from).as_ref() {
            None => return Err(BoardError::IllegalMove),
            Some(piece) => piece,
        };

        match mv.to {
            WHITE_ROOK_SHORT_STARTING_POS => self.short_castle_white = false,
            WHITE_ROOK_LONG_STARTING_POS => self.long_castle_white = false,
            BLACK_ROOK_SHORT_STARTING_POS => self.short_castle_black = false,
            BLACK_ROOK_LONG_STARTING_POS => self.long_castle_black = false,
            _ => {}
        };
        match mv.from {
            WHITE_ROOK_SHORT_STARTING_POS => self.short_castle_white = false,
            WHITE_ROOK_LONG_STARTING_POS => self.long_castle_white = false,
            BLACK_ROOK_SHORT_STARTING_POS => self.short_castle_black = false,
            BLACK_ROOK_LONG_STARTING_POS => self.long_castle_black = false,
            _ => {}
        };

        match piece.piece_type {
            PieceType::Pawn => {
                // Flag en passant as available for the next move if pawn moved 2 steps
                if (mv.to.row - mv.from.row).abs() == 2 {
                    self.en_passant_col = mv.to.col;
                }
                // Promote pawn
                else if (mv.to.row == 0) || (mv.to.row == (BOARD_ROW_COUNT as i8) - 1) {
                    at!(self, &mv.from).as_mut().unwrap().piece_type =
                        *mv.promotion.as_ref().unwrap();
                }
                // If capture using en passant
                if (mv.to.col - mv.from.col).abs() == 1 && at!(self, mv.to).is_none() {
                    // Remove captured piece
                    at!(
                        self,
                        Position {
                            row: mv.from.row,
                            col: mv.to.col,
                        }
                    ) = None;
                }
            }

            PieceType::King => {
                // Update king position
                *match piece.color {
                    Color::White => &mut self.white_king_pos,
                    Color::Black => &mut self.black_king_pos,
                } = mv.to;

                // If king move, mark castling as unavailable
                *match piece.color {
                    Color::White => &mut self.short_castle_white,
                    Color::Black => &mut self.short_castle_black,
                } = false;
                *match piece.color {
                    Color::White => &mut self.long_castle_white,
                    Color::Black => &mut self.long_castle_black,
                } = false;

                // Move the rook if castling
                if (mv.from.col - mv.to.col).abs() >= 2 {
                    let dir = (mv.to.col - mv.from.col) / (mv.to.col - mv.from.col).abs();
                    let mut rook_pos = mv.from.clone();
                    rook_pos.col += dir;

                    at!(self, rook_pos) = mem::replace(
                        &mut at!(
                            self,
                            match dir > 0 {
                                true => Position {
                                    row: mv.from.row,
                                    col: ROOK_SHORT_STARTING_COL,
                                },
                                false => Position {
                                    row: mv.from.row,
                                    col: ROOK_LONG_STARTING_COL,
                                },
                            }
                        ),
                        None,
                    )
                }
            }

            _ => {}
        };

        // Move the piece into the new position, replacing it with None in the process
        at!(self, mv.to) = mem::replace(&mut at!(self, mv.from), None);
        Ok(())
    }

    pub fn make_move(&mut self, mv: &Move) -> Result<(), BoardError> {
        if self.is_legal(&mv) == false {
            return Err(BoardError::IllegalMove);
        }
        self.unsafe_make_move(mv)
    }
}
