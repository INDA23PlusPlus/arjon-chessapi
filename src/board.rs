// This needs to be a macro, otherwise the borrow checker goes bonkers
macro_rules! at {
    ($self:expr, $pos:expr) => {
        $self.board[$pos.row as usize][$pos.col as usize]
    };
}

mod legal_move;
mod default;

pub use self::default::*;
pub use self::legal_move::*;
pub const BOARD_ROW_COUNT: usize = 8;
pub const BOARD_COL_COUNT: usize = 8;
pub(crate) const WHITE_KING_STARTING_POS: Position = Position {col: 4, row: 7};
pub(crate) const BLACK_KING_STARTING_POS: Position = Position {col: 4, row: 0};
pub(crate) const WHITE_PAWN_STARTING_ROW: i8 = 6;
pub(crate) const BLACK_PAWN_STARTING_ROW: i8 = 1;
pub(crate) const WHITE_EN_PASSANT_FROM_ROW: i8 = 3;
pub(crate) const BLACK_EN_PASSANT_FROM_ROW: i8 = 4;

extern crate num_traits;

use self::num_traits::*;
use std::mem;
use std::ops::*;
use crate::piece::{Color, Piece, PieceType};

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
    board: [[SquareType; 8]; 8],

    // Useful for checking if king will be attacked after a specific move
    white_king_pos: Position,
    black_king_pos: Position,

    // If castling is still available for black or white. Doesn't mean that it is a legal move
    short_castle_white: bool,
    long_castle_white:  bool,
    short_castle_black: bool,
    long_castle_black:  bool,

    // The column that you are able to do en passant to, or -1 if unavailable
    en_passant_col: i8,

    turn: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    row: i8,
    col: i8,
}

impl Position {
    fn abs(&self) -> Self {
        Self {row: self.row.abs(), col: self.col.abs() }
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {col: self.col + rhs.col, row: self.row + rhs.row }
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { row: self.row - rhs.row, col: self.col - rhs.col }
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
        Self { col: self.col / rhs_i8, row: self.row / rhs_i8 }
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
#[derive(Debug, Clone)]
pub struct Move {
    from: Position,
    to: Position,
    // Set this to Some(PieceType) if promotion is necessary
    promotion: Option<PieceType>,
}

impl Board {
    // Generates a new board with the default starting position
    pub fn new() -> Board {
        Default::default()
    }

    fn is_own_king_attacked_after_move(&self, mv: &Move) -> bool {
        todo!();
    }

    fn get_legal_moves_piece(&self, pos: &Position) -> Vec<Move> {
        todo!();
    }

    /*
     * Iterate through a all squares and and call get_legal_moves_piece() on every square that
     * holds a piece of the correct color
     */
    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for row in 0..BOARD_ROW_COUNT {
            for col in 0..BOARD_COL_COUNT {
                if self.board[row][col] == None { continue; }
                if self.board[row][col].unwrap().color != self.turn { continue; }
                moves.append(
                    &mut self.get_legal_moves_piece(
                        &Position { row: row as i8, col: col as i8 }
                    )
                );
            }
        }
        moves
    }

    pub fn make_move(&mut self, mv: Move) -> Result<(), BoardError> {
        if self.is_legal(&mv) == false {
            return Err(BoardError::IllegalMove);
        }
        // Basically flipping the value
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };

        // Reset en passant
        self.en_passant_col = -1;


        let piece = at!(self, mv.from).as_ref().unwrap();

        match piece.piece_type {
            PieceType::Pawn => {
                // Flag en passant as available for the next move if pawn moved 2 steps
                if (mv.to.row - mv.from.row).abs() == 2 {
                    self.en_passant_col = mv.to.col;
                }
                // Promote pawn
                else if (mv.to.row == 0) || (mv.to.row == (BOARD_ROW_COUNT as i8) - 1) {
                    at!(self, &mv.from).as_mut().unwrap().piece_type = *mv.promotion.as_ref().unwrap();
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
            }

            _ => {}
        };

        // Move the piece into the new position, replacing it with None in the process
        at!(self, mv.to) = mem::replace(&mut at!(self, mv.from), None);


        Ok(())
    }
}